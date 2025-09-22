use serde::{Deserialize, Serialize};

use crate::{EntityId, Position};

/// Container for entity diffs.
#[derive(Serialize, Deserialize)]
pub struct EntityDiff<D> {
    /// Entity ID.
    pub entity_id: EntityId,
    /// Old entity data.
    pub old: D,
    /// New entity data.
    pub new: D,
}

/// A diff of the game state before and after the step.
#[derive(Serialize, Deserialize)]
pub struct StateDiff {
    /// The positions that changed during the step.
    pub positions: Vec<EntityDiff<Position>>,
}
