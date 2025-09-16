use std::collections::HashMap;

use crate::{EntityId, Position, Stats};

use super::{
    diff::StateDiff,
    utils::{EntityDiff, SingleDiff},
};

type DiffMap<D> = HashMap<EntityId, SingleDiff<D>>;

/// Builder for entity diffs.
#[derive(Default)]
pub struct DiffBuilder {
    positions: Vec<EntityDiff<Position>>,
    stats: DiffMap<Stats>,
}

impl DiffBuilder {
    /// Adds a position diff to the builder.
    pub(crate) fn position(
        &mut self,
        entity_id: EntityId,
        old: Position,
        new: Position,
    ) {
        self.positions.push(EntityDiff { entity_id, old, new });
    }

    pub(crate) fn stat(&mut self, entity_id: EntityId, old: Stats, new: Stats) {
        self.stats.entry(entity_id).or_insert(SingleDiff { old, new });
    }

    /// Builds the entity diffs.
    pub(crate) fn build(self) -> StateDiff {
        StateDiff {
            positions: self.positions,
            stats: self
                .stats
                .into_iter()
                .map(|(entity_id, diff)| EntityDiff {
                    entity_id,
                    old: diff.old,
                    new: diff.new,
                })
                .collect(),
        }
    }
}
