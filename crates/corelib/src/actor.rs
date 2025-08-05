use std::sync::atomic::{AtomicU32, Ordering};

use crate::{actor_kind::ActorKind, actor_stats::Stats, position::Position};

/// Atomic counter for generating unique entity IDs.
static ENTITY_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Represents the unique identifier of an entity.
/// Uniqueness is guaranteed by the atomic counter.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct EntityId(u32);

impl From<u32> for EntityId {
    fn from(id: u32) -> Self {
        EntityId(id)
    }
}

impl From<EntityId> for u32 {
    fn from(id: EntityId) -> Self {
        id.0
    }
}

impl EntityId {
    /// Creates a new unique entity ID.
    pub(crate) fn next_entity_id() -> Self {
        EntityId(ENTITY_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

/// Represents an actor in the game. e.g. Player, Enemy.
#[derive(Debug)]
pub struct Actor {
    /// The unique identifier of the actor.
    pub(crate) _id: EntityId,
    /// The position of the actor.
    pub(crate) position: Position,
    /// The kind of actor.
    pub(crate) kind: ActorKind,
    /// The stats of the actor.
    pub(crate) stats: Stats,
}

impl Actor {
    /// Creates a new actor with the given position and kind.
    pub(crate) fn create(position: Position, kind: ActorKind) -> Self {
        Actor {
            // TODO
            _id: EntityId::next_entity_id(),
            position,
            stats: kind.default_stats(),
            kind,
        }
    }

    /// Creates a new player actor with the given position.
    pub(crate) fn create_player(position: Position) -> Self {
        Actor::create(position, ActorKind::Player)
    }

    /// Returns the position of the actor.
    pub fn position(&self) -> &Position {
        &self.position
    }

    /// Returns the stats of the actor.
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
}
