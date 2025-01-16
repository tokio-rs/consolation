#![doc = include_str!("../README.md")]
use std::{
    cell::RefCell,
    fmt,
    net::{IpAddr, Ipv4Addr},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use console_api as proto;
use proto::{instrument::instrument_server::InstrumentServer, resources::resource};
use serde::Serialize;
use thread_local::ThreadLocal;
#[cfg(unix)]
use tokio::net::UnixListener;
use tokio::sync::{mpsc, oneshot};
#[cfg(unix)]
use tokio_stream::wrappers::UnixListenerStream;
use tracing_core::{
    span::{self, Id},
    subscriber::{self, Subscriber},
    Metadata,
};
use tracing_subscriber::{
    layer::Context,
    registry::{Extensions, LookupSpan},
    Layer,
};

mod aggregator;
mod attribute;
mod builder;
mod callsites;
mod record;
mod stack;
mod stats;
pub(crate) mod sync;
mod visitors;

pub use aggregator::Aggregator;
pub use builder::{init, spawn};
pub use builder::{Builder, ServerAddr};
use callsites::Callsites;
use record::Recorder;
use stack::SpanStack;
use visitors::{AsyncOpVisitor, ResourceVisitor, ResourceVisitorResult, TaskVisitor, WakerVisitor};

use crate::visitors::{PollOpVisitor, StateUpdateVisitor};

/// A [`ConsoleLayer`] is a [`tracing_subscriber::Layer`] that records [`tracing`]
/// spans and events emitted by the async runtime.
///
/// Runtimes emit [`tracing`] spans and events that represent specific operations
/// that occur in asynchronous Rust programs, such as spawning tasks and waker
/// operations. The `ConsoleLayer` collects and aggregates these events, and the
/// resulting diagnostic data is exported to clients by the corresponding gRPC
/// [`Server`] instance.
///
/// [`tracing`]: https://docs.rs/tracing
pub struct ConsoleLayer {
    current_spans: ThreadLocal<RefCell<SpanStack>>,
    tx: mpsc::Sender<Event>,
    shared: Arc<Shared>,
    /// When the channel capacity goes under this number, a flush in the aggregator
    /// will be triggered.
    flush_under_capacity: usize,

    /// Set of callsites for spans representing spawned tasks.
    ///
    /// For task spans, each runtime these will have like, 1-5 callsites in it, max, so
    /// 8 should be plenty. If several runtimes are in use, we may have to spill
    /// over into the backup hashmap, but it's unlikely.
    spawn_callsites: Callsites<8>,

    /// Set of callsites for events representing waker operations.
    ///
    /// 16 is probably a reasonable number of waker ops; it's a bit generous if
    /// there's only one async runtime library in use, but if there are multiple,
    /// they might all have their own sets of waker ops.
    waker_callsites: Callsites<16>,

    /// Set of callsites for spans representing resources
    ///
    /// TODO: Take some time to determine more reasonable numbers
    resource_callsites: Callsites<32>,

    /// Set of callsites for spans representing async operations on resources
    ///
    /// TODO: Take some time to determine more reasonable numbers
    async_op_callsites: Callsites<32>,

    /// Set of callsites for spans representing async op poll operations
    ///
    /// TODO: Take some time to determine more reasonable numbers
    async_op_poll_callsites: Callsites<32>,

    /// Set of callsites for events representing poll operation invocations on resources
    ///
    /// TODO: Take some time to determine more reasonable numbers
    poll_op_callsites: Callsites<32>,

    /// Set of callsites for events representing state attribute state updates on resources
    ///
    /// TODO: Take some time to determine more reasonable numbers
    resource_state_update_callsites: Callsites<32>,

    /// Set of callsites for events representing state attribute state updates on async resource ops
    ///
    /// TODO: Take some time to determine more reasonable numbers
    async_op_state_update_callsites: Callsites<32>,

    /// A sink to record all events to a file.
    recorder: Option<Recorder>,

    /// Used to anchor monotonic timestamps to a base `SystemTime`, to produce a
    /// timestamp that can be sent over the wire or recorded to JSON.
    base_time: stats::TimeAnchor,

    /// Maximum value for the poll time histogram.
    ///
    /// By default, this is one second.
    max_poll_duration_nanos: u64,

    /// Maximum value for the scheduled time histogram.
    ///
    /// By default, this is one second.
    max_scheduled_duration_nanos: u64,
}

/// A gRPC [`Server`] that implements the [`tokio-console` wire format][wire].
///
/// Client applications, such as the [`tokio-console` CLI][cli] connect to the gRPC
/// server, and stream data about the runtime's history (such as a list of the
/// currently active tasks, or statistics summarizing polling times). A [`Server`] also
/// interprets commands from a client application, such a request to focus in on
/// a specific task, and translates that into a stream of details specific to
/// that task.
///
/// [wire]: https://docs.rs/console-api
/// [cli]: https://crates.io/crates/tokio-console
pub struct Server {
    subscribe: mpsc::Sender<Command>,
    addr: ServerAddr,
    aggregator: Option<Aggregator>,
    client_buffer: usize,
}

pub(crate) trait ToProto {
    type Output;
    fn to_proto(&self, base_time: &stats::TimeAnchor) -> Self::Output;
}

/// State shared between the `ConsoleLayer` and the `Aggregator` task.
#[derive(Debug, Default)]
struct Shared {
    /// Used to notify the aggregator task when the event buffer should be
    /// flushed.
    flush: aggregator::Flush,

    /// A counter of how many task events were dropped because the event buffer
    /// was at capacity.
    dropped_tasks: AtomicUsize,

    /// A counter of how many async op events were dropped because the event buffer
    /// was at capacity.
    dropped_async_ops: AtomicUsize,

    /// A counter of how many resource events were dropped because the event buffer
    /// was at capacity.
    dropped_resources: AtomicUsize,
}

struct Watch<T>(mpsc::Sender<Result<T, tonic::Status>>);

enum Command {
    Instrument(Watch<proto::instrument::Update>),
    WatchTaskDetail(WatchRequest<proto::tasks::TaskDetails>),
    WatchState(Watch<proto::instrument::State>),
    Pause,
    Resume,
}

struct WatchRequest<T> {
    id: Id,
    stream_sender: oneshot::Sender<mpsc::Receiver<Result<T, tonic::Status>>>,
    buffer: usize,
}

#[derive(Debug)]
enum Event {
    Metadata(&'static Metadata<'static>),
    Spawn {
        id: span::Id,
        metadata: &'static Metadata<'static>,
        stats: Arc<stats::TaskStats>,
        fields: Vec<proto::Field>,
        location: Option<proto::Location>,
    },
    Resource {
        id: span::Id,
        parent_id: Option<span::Id>,
        metadata: &'static Metadata<'static>,
        concrete_type: String,
        kind: resource::Kind,
        location: Option<proto::Location>,
        is_internal: bool,
        stats: Arc<stats::ResourceStats>,
    },
    PollOp {
        metadata: &'static Metadata<'static>,
        resource_id: span::Id,
        op_name: String,
        async_op_id: span::Id,
        task_id: span::Id,
        is_ready: bool,
    },
    AsyncResourceOp {
        id: span::Id,
        parent_id: Option<span::Id>,
        resource_id: span::Id,
        metadata: &'static Metadata<'static>,
        source: String,

        stats: Arc<stats::AsyncOpStats>,
    },
}

#[derive(Clone, Debug, Copy, Serialize)]
enum WakeOp {
    Wake { self_wake: bool },
    WakeByRef { self_wake: bool },
    Clone,
    Drop,
}

impl ConsoleLayer {
    /// Returns a `ConsoleLayer` built with the default settings.
    ///
    /// Note: these defaults do *not* include values provided via the
    /// environment variables specified in [`Builder::with_default_env`].
    ///
    /// See also [`Builder::build`].
    pub fn new() -> (Self, Server) {
        Self::builder().build()
    }

    /// Returns a [`Builder`] for configuring a `ConsoleLayer`.
    ///
    /// Note that the returned builder does *not* include values provided via
    /// the environment variables specified in [`Builder::with_default_env`].
    /// To extract those, you can call that method on the returned builder.
    pub fn builder() -> Builder {
        Builder::default()
    }

    fn build(config: Builder) -> (Self, Server) {
        // The `cfg` value *appears* to be a constant to clippy, but it changes
        // depending on the build-time configuration...
        #![allow(clippy::assertions_on_constants)]
        assert!(
            cfg!(any(tokio_unstable, console_without_tokio_unstable)),
            "task tracing requires Tokio to be built with RUSTFLAGS=\"--cfg tokio_unstable\"!"
        );

        let base_time = stats::TimeAnchor::new();
        tracing::debug!(
            config.event_buffer_capacity,
            config.client_buffer_capacity,
            ?config.publish_interval,
            ?config.retention,
            ?config.server_addr,
            ?config.recording_path,
            ?config.filter_env_var,
            ?config.poll_duration_max,
            ?config.scheduled_duration_max,
            ?base_time,
            "configured console subscriber"
        );

        let (tx, events) = mpsc::channel(config.event_buffer_capacity);
        let (subscribe, rpcs) = mpsc::channel(256);
        let shared = Arc::new(Shared::default());
        let aggregator = Aggregator::new(events, rpcs, &config, shared.clone(), base_time.clone());
        // Conservatively, start to trigger a flush when half the channel is full.
        // This tries to reduce the chance of losing events to a full channel.
        let flush_under_capacity = config.event_buffer_capacity / 2;
        let recorder = config
            .recording_path
            .as_ref()
            .map(|path| Recorder::new(path).expect("creating recorder"));
        let server = Server {
            aggregator: Some(aggregator),
            addr: config.server_addr,
            subscribe,
            client_buffer: config.client_buffer_capacity,
        };
        let layer = Self {
            current_spans: ThreadLocal::new(),
            tx,
            shared,
            flush_under_capacity,
            spawn_callsites: Callsites::default(),
            waker_callsites: Callsites::default(),
            resource_callsites: Callsites::default(),
            async_op_callsites: Callsites::default(),
            async_op_poll_callsites: Callsites::default(),
            poll_op_callsites: Callsites::default(),
            resource_state_update_callsites: Callsites::default(),
            async_op_state_update_callsites: Callsites::default(),
            recorder,
            base_time,
            max_poll_duration_nanos: config.poll_duration_max.as_nanos() as u64,
            max_scheduled_duration_nanos: config.scheduled_duration_max.as_nanos() as u64,
        };
        (layer, server)
    }
}

impl ConsoleLayer {
    /// Default maximum capacity for the channel of events sent from a
    /// [`ConsoleLayer`] to a [`Server`].
    ///
    /// When this capacity is exhausted, additional events will be dropped.
    /// Decreasing this value will reduce memory usage, but may result in
    /// events being dropped more frequently.
    ///
    /// See also [`Builder::event_buffer_capacity`].
    pub const DEFAULT_EVENT_BUFFER_CAPACITY: usize = 1024 * 100;
    /// Default maximum capacity for th echannel of events sent from a
    /// [`Server`] to each subscribed client.
    ///
    /// When this capacity is exhausted, the client is assumed to be inactive,
    /// and may be disconnected.
    ///
    /// See also [`Builder::client_buffer_capacity`].
    pub const DEFAULT_CLIENT_BUFFER_CAPACITY: usize = 1024 * 4;

    /// Default frequency for publishing events to clients.
    ///
    /// Note that methods like [`init`][`crate::init`] and [`spawn`][`crate::spawn`] will take the value
    /// from the `TOKIO_CONSOLE_PUBLISH_INTERVAL` [environment variable] before falling
    /// back on this default.
    ///
    /// See also [`Builder::publish_interval`].
    ///
    /// [environment variable]: `Builder::with_default_env`
    pub const DEFAULT_PUBLISH_INTERVAL: Duration = Duration::from_secs(1);

    /// By default, completed spans are retained for six seconds, which
    /// matches tokio-console's `retain_for` display period.
    ///
    /// Note that methods like [`init`][`crate::init`] and
    /// [`spawn`][`crate::spawn`] will take the value from the
    /// `TOKIO_CONSOLE_RETENTION` [environment variable] before falling back on
    /// this default.
    ///
    /// See also [`Builder::retention`].
    ///
    /// [environment variable]: `Builder::with_default_env`
    pub const DEFAULT_RETENTION: Duration = Duration::from_secs(6);

    /// The default maximum value for task poll duration histograms.
    ///
    /// Any poll duration exceeding this will be clamped to this value. By
    /// default, the maximum poll duration is one second.
    ///
    /// See also [`Builder::poll_duration_histogram_max`].
    pub const DEFAULT_POLL_DURATION_MAX: Duration = Duration::from_secs(1);

    /// The default maximum value for the task scheduled duration histogram.
    ///
    /// Any scheduled duration (the time from a task being woken until it is next
    /// polled) exceeding this will be clamped to this value. By default, the
    /// maximum scheduled duration is one second.
    ///
    /// See also [`Builder::scheduled_duration_histogram_max`].
    pub const DEFAULT_SCHEDULED_DURATION_MAX: Duration = Duration::from_secs(1);

    fn is_spawn(&self, meta: &'static Metadata<'static>) -> bool {
        self.spawn_callsites.contains(meta)
    }

    fn is_resource(&self, meta: &'static Metadata<'static>) -> bool {
        self.resource_callsites.contains(meta)
    }

    fn is_async_op(&self, meta: &'static Metadata<'static>) -> bool {
        self.async_op_callsites.contains(meta)
    }

    fn is_id_spawned<S>(&self, id: &span::Id, cx: &Context<'_, S>) -> bool
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        cx.span(id)
            .map(|span| self.is_spawn(span.metadata()))
            .unwrap_or(false)
    }

    fn is_id_resource<S>(&self, id: &span::Id, cx: &Context<'_, S>) -> bool
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        cx.span(id)
            .map(|span| self.is_resource(span.metadata()))
            .unwrap_or(false)
    }

    fn is_id_async_op<S>(&self, id: &span::Id, cx: &Context<'_, S>) -> bool
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        cx.span(id)
            .map(|span| self.is_async_op(span.metadata()))
            .unwrap_or(false)
    }

    fn first_entered<P>(&self, stack: &SpanStack, p: P) -> Option<span::Id>
    where
        P: Fn(&span::Id) -> bool,
    {
        stack
            .stack()
            .iter()
            .rev()
            .find(|id| p(id.id()))
            .map(|id| id.id())
            .cloned()
    }

    fn send_metadata(&self, dropped: &AtomicUsize, event: Event) -> bool {
        self.send_stats(dropped, move || (event, ())).is_some()
    }

    fn send_stats<S>(
        &self,
        dropped: &AtomicUsize,
        mk_event: impl FnOnce() -> (Event, S),
    ) -> Option<S> {
        use mpsc::error::TrySendError;

        // Return whether or not we actually sent the event.
        let sent = match self.tx.try_reserve() {
            Ok(permit) => {
                let (event, stats) = mk_event();
                permit.send(event);
                Some(stats)
            }
            Err(TrySendError::Closed(_)) => {
                // we should warn here eventually, but nop for now because we
                // can't trigger tracing events...
                None
            }
            Err(TrySendError::Full(_)) => {
                // this shouldn't happen, since we trigger a flush when
                // approaching the high water line...but if the executor wait
                // time is very high, maybe the aggregator task hasn't been
                // polled yet. so... eek?!
                dropped.fetch_add(1, Ordering::Release);
                None
            }
        };

        let capacity = self.tx.capacity();
        if capacity <= self.flush_under_capacity {
            self.shared.flush.trigger();
        }

        sent
    }

    fn record(&self, event: impl FnOnce() -> record::Event) {
        if let Some(ref recorder) = self.recorder {
            recorder.record(event());
        }
    }

    fn state_update<S>(
        &self,
        id: &Id,
        event: &tracing::Event<'_>,
        ctx: &Context<'_, S>,
        get_stats: impl for<'a> Fn(&'a Extensions) -> Option<&'a stats::ResourceStats>,
    ) where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let meta_id = event.metadata().into();
        let mut state_update_visitor = StateUpdateVisitor::new(meta_id);
        event.record(&mut state_update_visitor);

        let update = match state_update_visitor.result() {
            Some(update) => update,
            None => return,
        };

        let span = match ctx.span(id) {
            Some(span) => span,
            // XXX(eliza): no span exists for a resource ID, we should maybe
            // record an error here...
            None => return,
        };

        let exts = span.extensions();
        let stats = match get_stats(&exts) {
            Some(stats) => stats,
            // XXX(eliza): a resource span was not a resource??? this is a bug
            None => return,
        };

        stats.update_attribute(id, &update);

        if let Some(parent) = stats.parent_id.as_ref().and_then(|parent| ctx.span(parent)) {
            let exts = parent.extensions();
            if let Some(stats) = get_stats(&exts) {
                if stats.inherit_child_attributes {
                    stats.update_attribute(id, &update);
                }
            }
        }
    }
}

impl<S> Layer<S> for ConsoleLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn register_callsite(&self, meta: &'static Metadata<'static>) -> subscriber::Interest {
        if !meta.is_span() && !meta.is_event() {
            return subscriber::Interest::never();
        }

        let dropped = match (meta.name(), meta.target()) {
            ("runtime.spawn", _) | ("task", "tokio::task") => {
                self.spawn_callsites.insert(meta);
                &self.shared.dropped_tasks
            }
            (_, "runtime::waker") | (_, "tokio::task::waker") => {
                self.waker_callsites.insert(meta);
                &self.shared.dropped_tasks
            }
            (ResourceVisitor::RES_SPAN_NAME, _) => {
                self.resource_callsites.insert(meta);
                &self.shared.dropped_resources
            }
            (AsyncOpVisitor::ASYNC_OP_SPAN_NAME, _) => {
                self.async_op_callsites.insert(meta);
                &self.shared.dropped_async_ops
            }
            ("runtime.resource.async_op.poll", _) => {
                self.async_op_poll_callsites.insert(meta);
                &self.shared.dropped_async_ops
            }
            (_, PollOpVisitor::POLL_OP_EVENT_TARGET) => {
                self.poll_op_callsites.insert(meta);
                &self.shared.dropped_async_ops
            }
            (_, StateUpdateVisitor::RE_STATE_UPDATE_EVENT_TARGET) => {
                self.resource_state_update_callsites.insert(meta);
                &self.shared.dropped_resources
            }
            (_, StateUpdateVisitor::AO_STATE_UPDATE_EVENT_TARGET) => {
                self.async_op_state_update_callsites.insert(meta);
                &self.shared.dropped_async_ops
            }
            (_, _) => &self.shared.dropped_tasks,
        };

        self.send_metadata(dropped, Event::Metadata(meta));
        subscriber::Interest::always()
    }

    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let metadata = attrs.metadata();
        if self.is_spawn(metadata) {
            let at = Instant::now();
            let mut task_visitor = TaskVisitor::new(metadata.into());
            attrs.record(&mut task_visitor);
            let (fields, location) = task_visitor.result();
            self.record(|| record::Event::Spawn {
                id: id.into_u64(),
                at: self.base_time.to_system_time(at),
                fields: record::SerializeFields(fields.clone()),
            });
            if let Some(stats) = self.send_stats(&self.shared.dropped_tasks, move || {
                let stats = Arc::new(stats::TaskStats::new(
                    self.max_poll_duration_nanos,
                    self.max_scheduled_duration_nanos,
                    at,
                ));
                let event = Event::Spawn {
                    id: id.clone(),
                    stats: stats.clone(),
                    metadata,
                    fields,
                    location,
                };
                (event, stats)
            }) {
                ctx.span(id).expect("if `on_new_span` was called, the span must exist; this is a `tracing` bug!").extensions_mut().insert(stats);
            }
            return;
        }

        if self.is_resource(metadata) {
            let at = Instant::now();
            let mut resource_visitor = ResourceVisitor::default();
            attrs.record(&mut resource_visitor);
            if let Some(result) = resource_visitor.result() {
                let ResourceVisitorResult {
                    concrete_type,
                    kind,
                    location,
                    is_internal,
                    inherit_child_attrs,
                } = result;
                let parent_id = self.current_spans.get().and_then(|stack| {
                    self.first_entered(&stack.borrow(), |id| self.is_id_resource(id, &ctx))
                });
                if let Some(stats) = self.send_stats(&self.shared.dropped_resources, move || {
                    let stats = Arc::new(stats::ResourceStats::new(
                        at,
                        inherit_child_attrs,
                        parent_id.clone(),
                    ));
                    let event = Event::Resource {
                        id: id.clone(),
                        parent_id,
                        metadata,
                        concrete_type,
                        kind,
                        location,
                        is_internal,
                        stats: stats.clone(),
                    };
                    (event, stats)
                }) {
                    ctx.span(id).expect("if `on_new_span` was called, the span must exist; this is a `tracing` bug!").extensions_mut().insert(stats);
                }
            }
            return;
        }

        if self.is_async_op(metadata) {
            let at = Instant::now();
            let mut async_op_visitor = AsyncOpVisitor::default();
            attrs.record(&mut async_op_visitor);
            if let Some((source, inherit_child_attrs)) = async_op_visitor.result() {
                let resource_id = self.current_spans.get().and_then(|stack| {
                    self.first_entered(&stack.borrow(), |id| self.is_id_resource(id, &ctx))
                });

                let parent_id = self.current_spans.get().and_then(|stack| {
                    self.first_entered(&stack.borrow(), |id| self.is_id_async_op(id, &ctx))
                });

                if let Some(resource_id) = resource_id {
                    if let Some(stats) =
                        self.send_stats(&self.shared.dropped_async_ops, move || {
                            let stats = Arc::new(stats::AsyncOpStats::new(
                                at,
                                inherit_child_attrs,
                                parent_id.clone(),
                            ));
                            let event = Event::AsyncResourceOp {
                                id: id.clone(),
                                parent_id,
                                resource_id,
                                metadata,
                                source,
                                stats: stats.clone(),
                            };
                            (event, stats)
                        })
                    {
                        ctx.span(id).expect("if `on_new_span` was called, the span must exist; this is a `tracing` bug!").extensions_mut().insert(stats);
                    }
                }
            }
        }
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>) {
        let metadata = event.metadata();
        if self.waker_callsites.contains(metadata) {
            let at = Instant::now();
            let mut visitor = WakerVisitor::default();
            event.record(&mut visitor);
            // XXX (eliza): ew...
            if let Some((id, mut op)) = visitor.result() {
                if let Some(span) = ctx.span(&id) {
                    let exts = span.extensions();
                    if let Some(stats) = exts.get::<Arc<stats::TaskStats>>() {
                        if op.is_wake() {
                            // Are we currently inside the task's span? If so, the task
                            // has woken itself.

                            let self_wake = self
                                .current_spans
                                .get()
                                .map(|spans| spans.borrow().iter().any(|span| span == &id))
                                .unwrap_or(false);
                            op = op.self_wake(self_wake);
                        }

                        stats.record_wake_op(op, at);
                        self.record(|| record::Event::Waker {
                            id: id.into_u64(),
                            at: self.base_time.to_system_time(at),
                            op,
                        });
                    }
                }
            }
            return;
        }

        if self.poll_op_callsites.contains(metadata) {
            let resource_id = self.current_spans.get().and_then(|stack| {
                self.first_entered(&stack.borrow(), |id| self.is_id_resource(id, &ctx))
            });
            // poll op event should have a resource span parent
            if let Some(resource_id) = resource_id {
                let mut poll_op_visitor = PollOpVisitor::default();
                event.record(&mut poll_op_visitor);
                if let Some((op_name, is_ready)) = poll_op_visitor.result() {
                    let task_and_async_op_ids = self.current_spans.get().and_then(|stack| {
                        let stack = stack.borrow();
                        let task_id =
                            self.first_entered(&stack, |id| self.is_id_spawned(id, &ctx))?;
                        let async_op_id =
                            self.first_entered(&stack, |id| self.is_id_async_op(id, &ctx))?;
                        Some((task_id, async_op_id))
                    });
                    // poll op event should be emitted in the context of an async op and task spans
                    if let Some((task_id, async_op_id)) = task_and_async_op_ids {
                        if let Some(span) = ctx.span(&async_op_id) {
                            let exts = span.extensions();
                            if let Some(stats) = exts.get::<Arc<stats::AsyncOpStats>>() {
                                stats.set_task_id(&task_id);
                            }
                        }

                        self.send_stats(&self.shared.dropped_async_ops, || {
                            let event = Event::PollOp {
                                metadata,
                                op_name,
                                resource_id,
                                async_op_id,
                                task_id,
                                is_ready,
                            };
                            (event, ())
                        });

                        // TODO: JSON recorder doesn't care about poll ops.
                    }
                }
            }
            return;
        }

        if self.resource_state_update_callsites.contains(metadata) {
            // state update event should have a resource span parent
            let resource_id = self.current_spans.get().and_then(|stack| {
                self.first_entered(&stack.borrow(), |id| self.is_id_resource(id, &ctx))
            });
            if let Some(id) = resource_id {
                self.state_update(&id, event, &ctx, |exts| {
                    exts.get::<Arc<stats::ResourceStats>>()
                        .map(<Arc<stats::ResourceStats> as std::ops::Deref>::deref)
                });
            }

            return;
        }

        if self.async_op_state_update_callsites.contains(metadata) {
            let async_op_id = self.current_spans.get().and_then(|stack| {
                self.first_entered(&stack.borrow(), |id| self.is_id_async_op(id, &ctx))
            });
            if let Some(id) = async_op_id {
                self.state_update(&id, event, &ctx, |exts| {
                    let async_op = exts.get::<Arc<stats::AsyncOpStats>>()?;
                    Some(&async_op.stats)
                });
            }
        }
    }

    fn on_enter(&self, id: &span::Id, cx: Context<'_, S>) {
        if let Some(span) = cx.span(id) {
            let now = Instant::now();
            let exts = span.extensions();
            // if the span we are entering is a task or async op, record the
            // poll stats.
            if let Some(stats) = exts.get::<Arc<stats::TaskStats>>() {
                stats.start_poll(now);
            } else if let Some(stats) = exts.get::<Arc<stats::AsyncOpStats>>() {
                stats.start_poll(now);
            } else if exts.get::<Arc<stats::ResourceStats>>().is_some() {
                // otherwise, is the span a resource? in that case, we also want
                // to enter it, although we don't care about recording poll
                // stats.
            } else {
                return;
            };

            self.current_spans
                .get_or_default()
                .borrow_mut()
                .push(id.clone());

            self.record(|| record::Event::Enter {
                id: id.into_u64(),
                at: self.base_time.to_system_time(now),
            });
        }
    }

    fn on_exit(&self, id: &span::Id, cx: Context<'_, S>) {
        if let Some(span) = cx.span(id) {
            let exts = span.extensions();
            let now = Instant::now();
            // if the span we are entering is a task or async op, record the
            // poll stats.
            if let Some(stats) = exts.get::<Arc<stats::TaskStats>>() {
                stats.end_poll(now);
            } else if let Some(stats) = exts.get::<Arc<stats::AsyncOpStats>>() {
                stats.end_poll(now);
            } else if exts.get::<Arc<stats::ResourceStats>>().is_some() {
                // otherwise, is the span a resource? in that case, we also want
                // to enter it, although we don't care about recording poll
                // stats.
            } else {
                return;
            };

            self.current_spans.get_or_default().borrow_mut().pop(id);

            self.record(|| record::Event::Exit {
                id: id.into_u64(),
                at: self.base_time.to_system_time(now),
            });
        }
    }

    fn on_close(&self, id: span::Id, cx: Context<'_, S>) {
        if let Some(span) = cx.span(&id) {
            let now = Instant::now();
            let exts = span.extensions();
            if let Some(stats) = exts.get::<Arc<stats::TaskStats>>() {
                stats.drop_task(now);
            } else if let Some(stats) = exts.get::<Arc<stats::AsyncOpStats>>() {
                stats.drop_async_op(now);
            } else if let Some(stats) = exts.get::<Arc<stats::ResourceStats>>() {
                stats.drop_resource(now);
            }
            self.record(|| record::Event::Close {
                id: id.into_u64(),
                at: self.base_time.to_system_time(now),
            });
        }
    }
}

impl fmt::Debug for ConsoleLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConsoleLayer")
            // mpsc::Sender debug impl is not very useful
            .field("tx", &format_args!("<...>"))
            .field("tx.capacity", &self.tx.capacity())
            .field("shared", &self.shared)
            .field("spawn_callsites", &self.spawn_callsites)
            .field("waker_callsites", &self.waker_callsites)
            .finish()
    }
}

impl Server {
    // XXX(eliza): why is `SocketAddr::new` not `const`???
    /// A [`Server`] by default binds socket address 127.0.0.1 to service remote
    /// procedure calls.
    ///
    /// Note that methods like [`init`][`crate::init`] and
    /// [`spawn`][`crate::spawn`] will parse the socket address from the
    /// `TOKIO_CONSOLE_BIND` [environment variable] before falling back on
    /// constructing a socket address from this default.
    ///
    /// See also [`Builder::server_addr`].
    ///
    /// [environment variable]: `Builder::with_default_env`
    pub const DEFAULT_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    /// A [`Server`] by default binds port 6669 to service remote procedure
    /// calls.
    ///
    /// Note that methods like [`init`][`crate::init`] and
    /// [`spawn`][`crate::spawn`] will parse the socket address from the
    /// `TOKIO_CONSOLE_BIND` [environment variable] before falling back on
    /// constructing a socket address from this default.
    ///
    /// See also [`Builder::server_addr`].
    ///
    /// [environment variable]: `Builder::with_default_env`
    pub const DEFAULT_PORT: u16 = 6669;

    /// Starts the gRPC service with the default gRPC settings.
    ///
    /// To configure gRPC server settings before starting the server, use
    /// [`serve_with`] instead. This method is equivalent to calling [`serve_with`]
    /// and providing the default gRPC server settings:
    ///
    /// ```rust
    /// # async fn docs() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let (_, server) = console_subscriber::ConsoleLayer::new();
    /// server.serve_with(tonic::transport::Server::default()).await
    /// # }
    /// ```
    /// [`serve_with`]: Server::serve_with
    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.serve_with(tonic::transport::Server::default()).await
    }

    /// Starts the gRPC service with the given [`tonic`] gRPC transport server
    /// `builder`.
    ///
    /// The `builder` parameter may be used to configure gRPC-specific settings
    /// prior to starting the server.
    ///
    /// This spawns both the server task and the event aggregation worker
    /// task on the current async runtime.
    ///
    /// [`tonic`]: https://docs.rs/tonic/
    pub async fn serve_with(
        self,
        mut builder: tonic::transport::Server,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let addr = self.addr.clone();
        let ServerParts {
            instrument_server,
            aggregator,
        } = self.into_parts();
        let aggregate = spawn_named(aggregator.run(), "console::aggregate");
        let router = builder.add_service(instrument_server);
        let res = match addr {
            ServerAddr::Tcp(addr) => {
                let serve = router.serve(addr);
                spawn_named(serve, "console::serve").await
            }
            #[cfg(unix)]
            ServerAddr::Unix(path) => {
                let incoming = UnixListener::bind(path)?;
                let serve = router.serve_with_incoming(UnixListenerStream::new(incoming));
                spawn_named(serve, "console::serve").await
            }
        };
        aggregate.abort();
        res?.map_err(Into::into)
    }

    /// Starts the gRPC service with the default gRPC settings and gRPC-Web
    /// support.
    ///
    /// # Examples
    ///
    /// To serve the instrument server with gRPC-Web support with the default
    /// settings:
    ///
    /// ```rust
    /// # async fn docs() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let (_, server) = console_subscriber::ConsoleLayer::new();
    /// server.serve_with_grpc_web(tonic::transport::Server::default()).await
    /// # }
    /// ```
    ///
    /// To serve the instrument server with gRPC-Web support and a custom CORS configuration, use the
    /// following code:
    ///
    /// ```rust
    /// # use std::{thread, time::Duration};
    /// #
    /// use console_subscriber::{ConsoleLayer, ServerParts};
    /// use tonic_web::GrpcWebLayer;
    /// use tower_http::cors::{CorsLayer, AllowOrigin};
    /// use http::header::HeaderName;
    /// # use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    /// # const DEFAULT_MAX_AGE: Duration = Duration::from_secs(24 * 60 * 60);
    /// # const DEFAULT_EXPOSED_HEADERS: [&str; 3] =
    /// #    ["grpc-status", "grpc-message", "grpc-status-details-bin"];
    /// # const DEFAULT_ALLOW_HEADERS: [&str; 5] = [
    /// #    "x-grpc-web",
    /// #    "content-type",
    /// #    "x-user-agent",
    /// #    "grpc-timeout",
    /// #    "user-agent",
    /// # ];
    ///
    /// let (console_layer, server) = ConsoleLayer::builder().with_default_env().build();
    /// # thread::Builder::new()
    /// #    .name("subscriber".into())
    /// #    .spawn(move || {
    /// // Customize the CORS configuration.
    /// let cors = CorsLayer::new()
    ///     .allow_origin(AllowOrigin::mirror_request())
    ///     .allow_credentials(true)
    ///     .max_age(DEFAULT_MAX_AGE)
    ///     .expose_headers(
    ///         DEFAULT_EXPOSED_HEADERS
    ///             .iter()
    ///             .cloned()
    ///             .map(HeaderName::from_static)
    ///             .collect::<Vec<HeaderName>>(),
    ///     )
    ///     .allow_headers(
    ///         DEFAULT_ALLOW_HEADERS
    ///             .iter()
    ///             .cloned()
    ///             .map(HeaderName::from_static)
    ///             .collect::<Vec<HeaderName>>(),
    ///     );
    /// #       let runtime = tokio::runtime::Builder::new_current_thread()
    /// #           .enable_all()
    /// #           .build()
    /// #           .expect("console subscriber runtime initialization failed");
    /// #       runtime.block_on(async move {
    ///
    /// let ServerParts {
    ///     instrument_server,
    ///     aggregator,
    ///     ..
    /// } = server.into_parts();
    /// tokio::spawn(aggregator.run());
    ///
    /// // Serve the instrument server with gRPC-Web support and the CORS configuration.
    /// let router = tonic::transport::Server::builder()
    ///     .accept_http1(true)
    ///     .layer(cors)
    ///     .layer(GrpcWebLayer::new())
    ///     .add_service(instrument_server);
    /// let serve = router.serve(std::net::SocketAddr::new(
    ///     std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
    ///     // 6669 is a restricted port on Chrome, so we cannot use it. We use a different port instead.
    ///     9999,
    /// ));
    ///
    /// // Finally, spawn the server.
    /// serve.await.expect("console subscriber server failed");
    /// #       });
    /// #   })
    /// #   .expect("console subscriber could not spawn thread");
    /// # tracing_subscriber::registry().with(console_layer).init();
    /// ```
    ///
    /// For a comprehensive understanding and complete code example,
    /// please refer to the `grpc-web` example in the examples directory.
    ///
    /// [`Router::serve`]: fn@tonic::transport::server::Router::serve
    #[cfg(feature = "grpc-web")]
    pub async fn serve_with_grpc_web(
        self,
        builder: tonic::transport::Server,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let addr = self.addr.clone();
        let ServerParts {
            instrument_server,
            aggregator,
        } = self.into_parts();
        let router = builder
            .accept_http1(true)
            .add_service(tonic_web::enable(instrument_server));
        let aggregate = spawn_named(aggregator.run(), "console::aggregate");
        let res = match addr {
            ServerAddr::Tcp(addr) => {
                let serve = router.serve(addr);
                spawn_named(serve, "console::serve").await
            }
            #[cfg(unix)]
            ServerAddr::Unix(path) => {
                let incoming = UnixListener::bind(path)?;
                let serve = router.serve_with_incoming(UnixListenerStream::new(incoming));
                spawn_named(serve, "console::serve").await
            }
        };
        aggregate.abort();
        res?.map_err(Into::into)
    }

    /// Returns the parts needed to spawn a gRPC server and the aggregator that
    /// supplies it.
    ///
    /// Note that a server spawned in this way will disregard any value set by
    /// [`Builder::server_addr`], as the user becomes responsible for defining
    /// the address when calling [`Router::serve`].
    ///
    /// Additionally, the user of this API must ensure that the [`Aggregator`]
    /// is running for as long as the gRPC server is. If the server stops
    /// running, the aggregator task can be aborted.
    ///
    /// # Examples
    ///
    /// The parts can be used to serve the instrument server together with
    /// other endpoints from the same gRPC server.
    ///
    /// ```
    /// use console_subscriber::{ConsoleLayer, ServerParts};
    ///
    /// # let runtime = tokio::runtime::Builder::new_current_thread()
    /// #     .enable_all()
    /// #     .build()
    /// #     .unwrap();
    /// # runtime.block_on(async {
    /// let (console_layer, server) = ConsoleLayer::builder().build();
    /// let ServerParts {
    ///     instrument_server,
    ///     aggregator,
    ///     ..
    /// } = server.into_parts();
    ///
    /// let aggregator_handle = tokio::spawn(aggregator.run());
    /// let router = tonic::transport::Server::builder()
    ///     //.add_service(some_other_service)
    ///     .add_service(instrument_server);
    /// let serve = router.serve(std::net::SocketAddr::new(
    ///     std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
    ///     6669,
    /// ));
    ///
    /// // Finally, spawn the server.
    /// tokio::spawn(serve);
    /// # // Avoid a warning that `console_layer` and `aggregator_handle` are unused.
    /// # drop(console_layer);
    /// # let mut aggregator_handle = aggregator_handle;
    /// # aggregator_handle.abort();
    /// # });
    /// ```
    ///
    /// [`Router::serve`]: fn@tonic::transport::server::Router::serve
    pub fn into_parts(mut self) -> ServerParts {
        let aggregator = self
            .aggregator
            .take()
            .expect("cannot start server multiple times");

        let instrument_server = proto::instrument::instrument_server::InstrumentServer::new(self);

        ServerParts {
            instrument_server,
            aggregator,
        }
    }
}

/// Server Parts
///
/// This struct contains the parts returned by [`Server::into_parts`]. It may contain
/// further parts in the future, an as such is marked as `non_exhaustive`.
///
/// The `InstrumentServer<Server>` can be used to construct a router which
/// can be added to a [`tonic`] gRPC server.
///
/// The `aggregator` is a future which should be running as long as the server is.
/// Generally, this future should be spawned onto an appropriate runtime and then
/// aborted if the server gets shut down.
///
/// See the [`Server::into_parts`] documentation for usage.
#[non_exhaustive]
pub struct ServerParts {
    /// The instrument server.
    ///
    /// See the documentation for [`InstrumentServer`] for details.
    pub instrument_server: InstrumentServer<Server>,

    /// The aggregator.
    ///
    /// Responsible for collecting and preparing traces for the instrument server
    /// to send its clients.
    ///
    /// The aggregator should be [`run`] when the instrument server is started.
    /// If the server stops running for any reason, the aggregator task can be
    /// aborted.
    ///
    /// [`run`]: fn@crate::Aggregator::run
    pub aggregator: Aggregator,
}

#[tonic::async_trait]
impl proto::instrument::instrument_server::Instrument for Server {
    type WatchUpdatesStream =
        tokio_stream::wrappers::ReceiverStream<Result<proto::instrument::Update, tonic::Status>>;
    type WatchTaskDetailsStream =
        tokio_stream::wrappers::ReceiverStream<Result<proto::tasks::TaskDetails, tonic::Status>>;
    type WatchStateStream =
        tokio_stream::wrappers::ReceiverStream<Result<proto::instrument::State, tonic::Status>>;
    async fn watch_updates(
        &self,
        req: tonic::Request<proto::instrument::InstrumentRequest>,
    ) -> Result<tonic::Response<Self::WatchUpdatesStream>, tonic::Status> {
        match req.remote_addr() {
            Some(addr) => tracing::debug!(client.addr = %addr, "starting a new watch"),
            None => tracing::debug!(client.addr = %"<unknown>", "starting a new watch"),
        }
        let permit = self.subscribe.reserve().await.map_err(|_| {
            tonic::Status::internal("cannot start new watch, aggregation task is not running")
        })?;
        let (tx, rx) = mpsc::channel(self.client_buffer);
        permit.send(Command::Instrument(Watch(tx)));
        tracing::debug!("watch started");
        let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(tonic::Response::new(stream))
    }

    async fn watch_task_details(
        &self,
        req: tonic::Request<proto::instrument::TaskDetailsRequest>,
    ) -> Result<tonic::Response<Self::WatchTaskDetailsStream>, tonic::Status> {
        let task_id = req
            .into_inner()
            .id
            .ok_or_else(|| tonic::Status::invalid_argument("missing task_id"))?
            .id;

        // `tracing` reserves span ID 0 for niche optimization for `Option<Id>`.
        let id = std::num::NonZeroU64::new(task_id)
            .map(Id::from_non_zero_u64)
            .ok_or_else(|| tonic::Status::invalid_argument("task_id cannot be 0"))?;

        let permit = self.subscribe.reserve().await.map_err(|_| {
            tonic::Status::internal("cannot start new watch, aggregation task is not running")
        })?;

        // Check with the aggregator task to request a stream if the task exists.
        let (stream_sender, stream_recv) = oneshot::channel();
        permit.send(Command::WatchTaskDetail(WatchRequest {
            id,
            stream_sender,
            buffer: self.client_buffer,
        }));
        // If the aggregator drops the sender, the task doesn't exist.
        let rx = stream_recv.await.map_err(|_| {
            tracing::warn!(id = ?task_id, "requested task not found");
            tonic::Status::not_found("task not found")
        })?;

        tracing::debug!(id = ?task_id, "task details watch started");
        let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(tonic::Response::new(stream))
    }

    async fn watch_state(
        &self,
        _req: tonic::Request<proto::instrument::StateRequest>,
    ) -> Result<tonic::Response<Self::WatchStateStream>, tonic::Status> {
        let (stream_sender, stream_recv) = mpsc::channel(self.client_buffer);
        self.subscribe
            .send(Command::WatchState(Watch(stream_sender)))
            .await
            .map_err(|_| {
                tonic::Status::internal("cannot get state, aggregation task is not running")
            })?;
        let stream = tokio_stream::wrappers::ReceiverStream::new(stream_recv);
        Ok(tonic::Response::new(stream))
    }

    async fn pause(
        &self,
        _req: tonic::Request<proto::instrument::PauseRequest>,
    ) -> Result<tonic::Response<proto::instrument::PauseResponse>, tonic::Status> {
        self.subscribe.send(Command::Pause).await.map_err(|_| {
            tonic::Status::internal("cannot pause, aggregation task is not running")
        })?;
        Ok(tonic::Response::new(proto::instrument::PauseResponse {}))
    }

    async fn resume(
        &self,
        _req: tonic::Request<proto::instrument::ResumeRequest>,
    ) -> Result<tonic::Response<proto::instrument::ResumeResponse>, tonic::Status> {
        self.subscribe.send(Command::Resume).await.map_err(|_| {
            tonic::Status::internal("cannot resume, aggregation task is not running")
        })?;
        Ok(tonic::Response::new(proto::instrument::ResumeResponse {}))
    }
}

impl WakeOp {
    /// Returns `true` if `self` is a `Wake` or `WakeByRef` event.
    fn is_wake(self) -> bool {
        matches!(self, Self::Wake { .. } | Self::WakeByRef { .. })
    }

    fn self_wake(self, self_wake: bool) -> Self {
        match self {
            Self::Wake { .. } => Self::Wake { self_wake },
            Self::WakeByRef { .. } => Self::WakeByRef { self_wake },
            x => x,
        }
    }
}

#[track_caller]
pub(crate) fn spawn_named<T>(
    task: impl std::future::Future<Output = T> + Send + 'static,
    _name: &str,
) -> tokio::task::JoinHandle<T>
where
    T: Send + 'static,
{
    #[cfg(tokio_unstable)]
    return tokio::task::Builder::new().name(_name).spawn(task).unwrap();

    #[cfg(not(tokio_unstable))]
    tokio::spawn(task)
}
