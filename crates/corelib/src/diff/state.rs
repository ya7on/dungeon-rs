use crate::{Position, Stats};

use super::utils::EntityDiff;

/// Represents the changes in state between two steps.
pub struct StateDiff {
    /// The positions that changed during the step.
    pub positions: Vec<EntityDiff<Position>>,
    /// The stats that changed during the step.
    pub stats: Vec<EntityDiff<Stats>>,
}
