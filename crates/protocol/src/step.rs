use std::collections::VecDeque;

use crate::GameEvent;

/// Represents the result of a step operation.
pub struct StepResult {
    /// A queue of game events that occurred during the step.
    pub events: VecDeque<GameEvent>,
}
