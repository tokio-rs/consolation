use std::fmt;

use tokio::sync::broadcast::{
    self,
    error::{RecvError, TryRecvError},
};

/// A step in the running of the test
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(super) enum TestStep {
    /// The overall test has begun
    Start,
    /// The instrument server has been started
    ServerStarted,
    /// The client has connected to the instrument server
    ClientConnected,
    /// The future being driven has completed
    TestFinished,
    /// The client has finished recording updates
    UpdatesRecorded,
}

impl fmt::Display for TestStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self as &dyn fmt::Debug).fmt(f)
    }
}

/// The state of the test.
///
/// This struct is used by various parts of the test framework to wait until
/// a specific test step has been reached and advance the test state to a new
/// step.
pub(super) struct TestState {
    receiver: broadcast::Receiver<TestStep>,
    sender: broadcast::Sender<TestStep>,
    step: TestStep,
}

impl TestState {
    pub(super) fn new() -> Self {
        let (sender, receiver) = broadcast::channel(1);
        Self {
            receiver,
            sender,
            step: TestStep::Start,
        }
    }

    /// Block asynchronously until the desired step has been reached.
    ///
    /// # Panics
    ///
    /// This function will panic if the underlying channel gets closed.
    pub(super) async fn wait_for_step(&mut self, desired_step: TestStep) {
        {
            let _guard = tracing::info_span!("wait_for_step").entered();
            self.update_step();
        }
        tracing::info!(
            target: "console_test::support::state",
            "wait_for_step: {current} -> {desired_step}",
            current = self.step,
        );

        loop {
            if self.step >= desired_step {
                break;
            }

            match self.receiver.recv().await {
                Ok(step) => self.step = step,
                Err(RecvError::Lagged(_)) => {
                    // we don't mind being lagged, we'll just get the latest state
                }
                Err(RecvError::Closed) => {
                    panic!("failed to receive current step, waiting for step: {desired_step}, did the test abort?");
                }
            }
        }
    }

    /// Check whether the desired step has been reached without blocking.
    pub(super) fn try_wait_for_step(&mut self, desired_step: TestStep) -> bool {
        {
            let _guard = tracing::info_span!("try_wait_for_step").entered();
            self.update_step();
        }
        tracing::info!(
            target: "console_test::support::state",
            "try_wait_for_step: {current} -> {desired_step}",
            current = self.step,
        );

        self.step == desired_step
    }

    /// Advance to the next step.
    ///
    /// The test must be at the step prior to the next step before starting.
    /// Being in a different step is likely to indicate a logic error in the
    /// test framework.
    ///
    /// # Panics
    ///
    /// This method will panic if the test state is not at the step prior to
    /// `next_step` or if the underlying channel is closed.
    #[track_caller]
    pub(super) fn advance_to_step(&mut self, next_step: TestStep) {
        {
            let _guard = tracing::info_span!("advance_to_step").entered();
            self.update_step();
        }
        tracing::info!(
            target: "console_test::support::state",
            "advance_to_step: {current} -> {next_step}",
            current = self.step,
        );

        if self.step >= next_step {
            panic!(
                "cannot advance to previous or current step! current step: {current}, next step: {next_step}",
                current = self.step);
        }

        match (&self.step, &next_step) {
            (TestStep::Start, TestStep::ServerStarted) |
            (TestStep::ServerStarted, TestStep::ClientConnected) |
            (TestStep::ClientConnected, TestStep::TestFinished) |
            (TestStep::TestFinished, TestStep::UpdatesRecorded) => {},
            (_, _) => panic!(
                "cannot advance more than one step! current step: {current}, next step: {next_step}",
                current = self.step),
        }

        self.sender
            .send(next_step)
            .expect("failed to send the next test step, did the test abort?");
    }

    fn update_step(&mut self) {
        loop {
            match self.receiver.try_recv() {
                Ok(step) => {
                    tracing::info!(
                        target: "console_test::support::state",
                        "update_step: {previous} -> {current}.",
                        previous = self.step,
                        current = step,
                    );
                    self.step = step;
                }
                Err(TryRecvError::Lagged(count)) => {
                    tracing::info!(
                        target: "console_test::support::state",
                        "update_step: lagged by {count}! This is actually a big problem.",
                        count= count,
                    );
                }
                Err(TryRecvError::Closed) => {
                    panic!("failed to update current step, did the test abort?")
                }
                Err(TryRecvError::Empty) => break,
            }
        }
    }
}

impl Clone for TestState {
    fn clone(&self) -> Self {
        Self {
            receiver: self.receiver.resubscribe(),
            sender: self.sender.clone(),
            step: self.step.clone(),
        }
    }
}