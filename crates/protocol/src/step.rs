use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::{GameEvent, StateDiff};

/// Represents the result of a step operation.
#[derive(Serialize, Deserialize)]
pub struct StepResult {
    /// A queue of game events that occurred during the step.
    pub events: VecDeque<GameEvent>,
    /// A diff of the game state before and after the step.
    pub diff: StateDiff,
}
