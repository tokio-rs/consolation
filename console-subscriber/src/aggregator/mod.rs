use crate::{record::Recorder, WatchRequest};

use super::{Command, Event, WakeOp, Watch};
use console_api as proto;
use tokio::sync::{mpsc, Notify};

use futures::FutureExt;
use std::{
    collections::hash_map::Entry,
    convert::TryInto,
    sync::{
        atomic::{AtomicBool, Ordering::*},
        Arc,
    },
    time::{Duration, SystemTime},
};
use tracing_core::{span, Metadata};

use hdrhistogram::{
    serialization::{Serializer, V2SerializeError, V2Serializer},
    Histogram,
};

pub type TaskId = u64;

mod shrink;
mod task_data;
use self::shrink::{ShrinkMap, ShrinkVec};
use self::task_data::TaskData;

pub(crate) struct Aggregator {
    /// Channel of incoming events emitted by `TaskLayer`s.
    events: mpsc::Receiver<Event>,

    /// New incoming RPCs.
    rpcs: mpsc::Receiver<Command>,

    /// The interval at which new data updates are pushed to clients.
    publish_interval: Duration,

    /// How long to keep task data after a task has completed.
    retention: Duration,

    /// Triggers a flush when the event buffer is approaching capacity.
    flush_capacity: Arc<Flush>,

    /// Currently active RPCs streaming task events.
    watchers: ShrinkVec<Watch<proto::tasks::TaskUpdate>>,

    /// Currently active RPCs streaming task details events, by task ID.
    details_watchers: ShrinkMap<TaskId, Vec<Watch<proto::tasks::TaskDetails>>>,

    /// *All* metadata for task spans and user-defined spans that we care about.
    ///
    /// This is sent to new clients as part of the initial state.
    all_metadata: ShrinkVec<proto::register_metadata::NewMetadata>,

    /// *New* metadata that was registered since the last state update.
    ///
    /// This is emptied on every state update.
    new_metadata: Vec<proto::register_metadata::NewMetadata>,

    /// Map of task IDs to task static data.
    tasks: TaskData<Task>,

    /// Map of task IDs to task stats.
    stats: TaskData<Stats>,

    task_ids: TaskIds,

    /// A sink to record all events to a file.
    recorder: Option<Recorder>,

    /// The time "state" of the aggregator, such as paused or live.
    temporality: Temporality,
}

#[derive(Debug)]
pub(crate) struct Flush {
    pub(crate) should_flush: Notify,
    pub(crate) triggered: AtomicBool,
}

#[derive(Debug)]
enum Temporality {
    Live,
    Paused,
}

struct Stats {
    // task stats
    polls: u64,
    current_polls: u64,
    created_at: Option<SystemTime>,
    first_poll: Option<SystemTime>,
    last_poll_started: Option<SystemTime>,
    last_poll_ended: Option<SystemTime>,
    busy_time: Duration,
    closed_at: Option<SystemTime>,

    // waker stats
    wakes: u64,
    waker_clones: u64,
    waker_drops: u64,
    last_wake: Option<SystemTime>,

    poll_times_histogram: Histogram<u64>,
}

struct Task {
    metadata: &'static Metadata<'static>,
    fields: Vec<proto::Field>,
}

#[derive(Debug, Default)]
struct TaskIds {
    /// A counter for the pretty task IDs.
    next: TaskId,

    /// A table that contains the span ID to pretty task ID mappings.
    id_mappings: ShrinkMap<span::Id, TaskId>,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            polls: 0,
            current_polls: 0,
            created_at: None,
            first_poll: None,
            last_poll_started: None,
            last_poll_ended: None,
            busy_time: Default::default(),
            closed_at: None,
            wakes: 0,
            waker_clones: 0,
            waker_drops: 0,
            last_wake: None,
            // significant figures should be in the [0-5] range and memory usage
            // grows exponentially with higher a sigfig
            poll_times_histogram: Histogram::<u64>::new(2).unwrap(),
        }
    }
}

impl Aggregator {
    pub(crate) fn new(
        events: mpsc::Receiver<Event>,
        rpcs: mpsc::Receiver<Command>,
        builder: &crate::Builder,
    ) -> Self {
        Self {
            flush_capacity: Arc::new(Flush {
                should_flush: Notify::new(),
                triggered: AtomicBool::new(false),
            }),
            rpcs,
            publish_interval: builder.publish_interval,
            retention: builder.retention,
            events,
            watchers: Default::default(),
            details_watchers: Default::default(),
            all_metadata: Default::default(),
            new_metadata: Default::default(),
            tasks: TaskData::new(),
            stats: TaskData::new(),
            task_ids: TaskIds::default(),
            recorder: builder
                .recording_path
                .as_ref()
                .map(|path| Recorder::new(path).expect("creating recorder")),
            temporality: Temporality::Live,
        }
    }

    pub(crate) fn flush(&self) -> &Arc<Flush> {
        &self.flush_capacity
    }

    pub(crate) async fn run(mut self) {
        let mut publish = tokio::time::interval(self.publish_interval);
        loop {
            let should_send = tokio::select! {
                // if the flush interval elapses, flush data to the client
                _ = publish.tick() => {
                    match self.temporality {
                        Temporality::Live => true,
                        Temporality::Paused => false,
                    }
                }

                // triggered when the event buffer is approaching capacity
                _ = self.flush_capacity.should_flush.notified() => {
                    self.flush_capacity.triggered.store(false, Release);
                    tracing::debug!("approaching capacity; draining buffer");
                    false
                }

                // a new command from a client
                cmd = self.rpcs.recv() => {
                    match cmd {
                        Some(Command::WatchTasks(subscription)) => {
                            self.add_task_subscription(subscription);
                        },
                        Some(Command::WatchTaskDetail(watch_request)) => {
                            self.add_task_detail_subscription(watch_request);
                        },
                        Some(Command::Pause) => {
                            self.temporality = Temporality::Paused;
                        }
                        Some(Command::Resume) => {
                            self.temporality = Temporality::Live;
                        }
                        None => {
                            tracing::debug!("rpc channel closed, terminating");
                            return;
                        }
                    };

                    false
                }

            };

            // drain and aggregate buffered events.
            //
            // Note: we *don't* want to actually await the call to `recv` --- we
            // don't want the aggregator task to be woken on every event,
            // because it will then be woken when its own `poll` calls are
            // exited. that would result in a busy-loop. instead, we only want
            // to be woken when the flush interval has elapsed, or when the
            // channel is almost full.
            while let Some(event) = self.events.recv().now_or_never() {
                match event {
                    Some(event) => {
                        // always be recording...
                        if let Some(ref recorder) = self.recorder {
                            recorder.record(&event);
                        }
                        self.update_state(event)
                    }
                    // The channel closed, no more events will be emitted...time
                    // to stop aggregating.
                    None => {
                        tracing::debug!("event channel closed; terminating");
                        return;
                    }
                };
            }

            // flush data to clients, if there are any currently subscribed
            // watchers and we should send a new update.
            if !self.watchers.is_empty() && should_send {
                self.publish();
            }

            // drop any tasks that have completed *and* whose final data has already
            // been sent off.
            self.drop_closed_tasks();
        }
    }

    /// Add the task subscription to the watchers after sending the first update
    fn add_task_subscription(&mut self, subscription: Watch<proto::tasks::TaskUpdate>) {
        tracing::debug!("new tasks subscription");
        let new_tasks = self
            .tasks
            .all()
            .map(|(&id, task)| task.to_proto(id))
            .collect();
        let now = SystemTime::now();
        let stats_update = self
            .stats
            .all()
            .map(|(&id, stats)| (id, stats.to_proto()))
            .collect();
        // Send the initial state --- if this fails, the subscription is already dead
        if subscription.update(&proto::tasks::TaskUpdate {
            new_metadata: Some(proto::RegisterMetadata {
                metadata: (*self.all_metadata).clone(),
            }),
            new_tasks,
            stats_update,
            now: Some(now.into()),
        }) {
            self.watchers.push(subscription)
        }
    }

    /// Add the task details subscription to the watchers after sending the first update,
    /// if the task is found.
    fn add_task_detail_subscription(
        &mut self,
        watch_request: WatchRequest<proto::tasks::TaskDetails>,
    ) {
        let WatchRequest {
            id,
            stream_sender,
            buffer,
        } = watch_request;
        tracing::debug!(id = ?id, "new task details subscription");
        if let Some(stats) = self.stats.get(&id) {
            let (tx, rx) = mpsc::channel(buffer);
            let subscription = Watch(tx);
            let now = SystemTime::now();
            // Send back the stream receiver.
            // Then send the initial state --- if this fails, the subscription is already dead.
            if stream_sender.send(rx).is_ok()
                && subscription.update(&proto::tasks::TaskDetails {
                    task_id: Some(id.into()),
                    now: Some(now.into()),
                    poll_times_histogram: serialize_histogram(&stats.poll_times_histogram).ok(),
                })
            {
                self.details_watchers
                    .entry(id)
                    .or_insert_with(Vec::new)
                    .push(subscription);
            }
        }
        // If the task is not found, drop `stream_sender` which will result in a not found error
    }

    /// Publish the current state to all active watchers.
    ///
    /// This drops any watchers which have closed the RPC, or whose update
    /// channel has filled up.
    fn publish(&mut self) {
        let new_metadata = if !self.new_metadata.is_empty() {
            Some(proto::RegisterMetadata {
                metadata: std::mem::take(&mut self.new_metadata),
            })
        } else {
            None
        };
        let new_tasks = self
            .tasks
            .since_last_update()
            .map(|(&id, task)| task.to_proto(id))
            .collect();
        let now = SystemTime::now();
        let stats_update = self
            .stats
            .since_last_update()
            .map(|(&id, stats)| (id, stats.to_proto()))
            .collect();

        let update = proto::tasks::TaskUpdate {
            new_metadata,
            new_tasks,
            stats_update,
            now: Some(now.into()),
        };
        self.watchers
            .retain_and_shrink(|watch: &Watch<proto::tasks::TaskUpdate>| watch.update(&update));

        let stats = &self.stats;
        // Assuming there are much fewer task details subscribers than there are
        // stats updates, iterate over `details_watchers` and compact the map.
        self.details_watchers.retain_and_shrink(|&id, watchers| {
            if let Some(task_stats) = stats.get(&id) {
                let details = proto::tasks::TaskDetails {
                    task_id: Some(id.into()),
                    now: Some(now.into()),
                    poll_times_histogram: serialize_histogram(&task_stats.poll_times_histogram)
                        .ok(),
                };
                watchers.retain(|watch| watch.update(&details));
                !watchers.is_empty()
            } else {
                false
            }
        });
    }

    /// Update the current state with data from a single event.
    fn update_state(&mut self, event: Event) {
        // do state update
        match event {
            Event::Metadata(meta) => {
                self.all_metadata.push(meta.into());
                self.new_metadata.push(meta.into());
            }
            Event::Spawn {
                id,
                metadata,
                at,
                fields,
            } => {
                let task_id = self.task_ids.id_for(id);
                self.tasks.insert(
                    task_id,
                    Task {
                        metadata,
                        fields,
                        // TODO: parents
                    },
                );
                self.stats.insert(
                    task_id,
                    Stats {
                        polls: 0,
                        created_at: Some(at),
                        ..Default::default()
                    },
                );
            }
            Event::Enter { id, at } => {
                let task_id = self.task_ids.id_for(id);
                let mut stats = self.stats.update_or_default(task_id);
                if stats.current_polls == 0 {
                    stats.last_poll_started = Some(at);
                    if stats.first_poll == None {
                        stats.first_poll = Some(at);
                    }
                    stats.polls += 1;
                }
                stats.current_polls += 1;
            }

            Event::Exit { id, at } => {
                let task_id = self.task_ids.id_for(id);
                let mut stats = self.stats.update_or_default(task_id);
                stats.current_polls -= 1;
                if stats.current_polls == 0 {
                    if let Some(last_poll_started) = stats.last_poll_started {
                        let elapsed = at.duration_since(last_poll_started).unwrap();
                        stats.last_poll_ended = Some(at);
                        stats.busy_time += elapsed;
                        stats
                            .poll_times_histogram
                            .record(elapsed.as_nanos().try_into().unwrap_or(u64::MAX))
                            .unwrap();
                    }
                }
            }

            Event::Close { id, at } => {
                let task_id = self.task_ids.id_for(id);
                self.stats.update_or_default(task_id).closed_at = Some(at);
            }

            Event::Waker { id, op, at } => {
                let task_id = self.task_ids.id_for(id);
                // It's possible for wakers to exist long after a task has
                // finished. We don't want those cases to create a "new"
                // task that isn't closed, just to insert some waker stats.
                //
                // It may be useful to eventually be able to report about
                // "wasted" waker ops, but we'll leave that for another time.
                if let Some(mut stats) = self.stats.update(&task_id) {
                    match op {
                        WakeOp::Wake | WakeOp::WakeByRef => {
                            stats.wakes += 1;
                            stats.last_wake = Some(at);

                            // Note: `Waker::wake` does *not* call the `drop`
                            // implementation, so waking by value doesn't
                            // trigger a drop event. so, count this as a `drop`
                            // to ensure the task's number of wakers can be
                            // calculated as `clones` - `drops`.
                            //
                            // see
                            // https://github.com/rust-lang/rust/blob/673d0db5e393e9c64897005b470bfeb6d5aec61b/library/core/src/task/wake.rs#L211-L212
                            if let WakeOp::Wake = op {
                                stats.waker_drops += 1;
                            }
                        }
                        WakeOp::Clone => {
                            stats.waker_clones += 1;
                        }
                        WakeOp::Drop => {
                            stats.waker_drops += 1;
                        }
                    }
                }
            }
        }
    }

    fn drop_closed_tasks(&mut self) {
        let tasks = &mut self.tasks;
        let stats = &mut self.stats;
        let task_ids = &mut self.task_ids;
        let has_watchers = !self.watchers.is_empty();
        let now = SystemTime::now();
        let retention = self.retention;

        // drop stats for closed tasks if they have been updated
        tracing::trace!(
            ?self.retention,
            self.has_watchers = has_watchers,
            "dropping closed tasks..."
        );

        let mut dropped_stats = false;
        stats.retain_and_shrink(|id, stats, dirty| {
            if let Some(closed) = stats.closed_at {
                let closed_for = now.duration_since(closed).unwrap_or_default();
                let should_drop =
                    // if there are any clients watching, retain all dirty tasks regardless of age
                    (dirty && has_watchers)
                    || closed_for > retention;
                tracing::trace!(
                    stats.id = ?id,
                    stats.closed_at = ?closed,
                    stats.closed_for = ?closed_for,
                    stats.dirty = dirty,
                    should_drop,
                );
                dropped_stats = should_drop;
                return !should_drop;
            }

            true
        });

        // If we dropped any stats, drop task static data and IDs as
        if dropped_stats {
            // drop closed tasks which no longer have stats.
            tasks.retain_and_shrink(|id, _, _| stats.contains(id));

            task_ids.retain_only(&*tasks);
        }
    }
}

// ==== impl Flush ===

impl Flush {
    pub(crate) fn trigger(&self) {
        if self
            .triggered
            .compare_exchange(false, true, AcqRel, Acquire)
            .is_ok()
        {
            self.should_flush.notify_one();
        } else {
            // someone else already did it, that's fine...
        }
    }
}

impl<T: Clone> Watch<T> {
    fn update(&self, update: &T) -> bool {
        if let Ok(reserve) = self.0.try_reserve() {
            reserve.send(Ok(update.clone()));
            true
        } else {
            false
        }
    }
}

impl Stats {
    fn total_time(&self) -> Option<Duration> {
        self.closed_at.and_then(|end| {
            self.created_at
                .and_then(|start| end.duration_since(start).ok())
        })
    }

    fn to_proto(&self) -> proto::tasks::Stats {
        proto::tasks::Stats {
            polls: self.polls,
            created_at: self.created_at.map(Into::into),
            first_poll: self.first_poll.map(Into::into),
            last_poll_started: self.last_poll_started.map(Into::into),
            last_poll_ended: self.last_poll_ended.map(Into::into),
            busy_time: Some(self.busy_time.into()),
            total_time: self.total_time().map(Into::into),
            wakes: self.wakes,
            waker_clones: self.waker_clones,
            waker_drops: self.waker_drops,
            last_wake: self.last_wake.map(Into::into),
        }
    }
}

impl Task {
    fn to_proto(&self, id: u64) -> proto::tasks::Task {
        proto::tasks::Task {
            id: Some(id.into()),
            // TODO: more kinds of tasks...
            kind: proto::tasks::task::Kind::Spawn as i32,
            metadata: Some(self.metadata.into()),
            parents: Vec::new(), // TODO: implement parents nicely
            fields: self.fields.clone(),
        }
    }
}

// === impl TaskIds ===

impl TaskIds {
    fn id_for(&mut self, span_id: span::Id) -> TaskId {
        match self.id_mappings.entry(span_id) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let task_id = self.next;
                entry.insert(task_id);
                self.next = self.next.wrapping_add(1);
                task_id
            }
        }
    }

    #[inline]
    fn retain_only<T>(&mut self, tasks: &TaskData<T>) {
        self.id_mappings
            .retain(|_, task_id| tasks.contains(task_id));
    }
}

fn serialize_histogram(histogram: &Histogram<u64>) -> Result<Vec<u8>, V2SerializeError> {
    let mut serializer = V2Serializer::new();
    let mut buf = Vec::new();
    serializer.serialize(histogram, &mut buf)?;
    Ok(buf)
}
