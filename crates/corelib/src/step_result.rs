use std::collections::VecDeque;

use crate::{
    GameEvent,
    diff::{DiffBuilder, StateDiff},
};

/// Represents the result of a game step.
pub struct StepResult {
    /// The events that occurred during the step.
    pub events: VecDeque<GameEvent>,
    /// The state diff that occurred during the step.
    pub diff: StateDiff,
}

/// Represents a builder for creating `StepResult` instances.
#[derive(Default)]
pub(crate) struct StepContext {
    events: VecDeque<GameEvent>,
    diff: DiffBuilder,
}

impl StepContext {
    pub(crate) fn add_event(&mut self, event: GameEvent) {
        self.events.push_back(event);
    }

    pub(crate) fn build(self) -> StepResult {
        StepResult { events: self.events, diff: self.diff.build() }
    }
}
