use std::sync::atomic::{AtomicU32, Ordering};

use crate::{catalog::EffectInstance, position::Position};

use super::{ActorKind, stats::Stats};

/// Atomic counter for generating unique entity IDs.
static ENTITY_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Represents the unique identifier of an entity.
/// Uniqueness is guaranteed by the atomic counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(u32);

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
    /// Returns the inner value of the entity ID.
    #[must_use]
    pub fn into_inner(self) -> u32 {
        self.0
    }

    /// Creates a new unique entity ID.
    pub fn next_entity_id() -> Self {
        EntityId(ENTITY_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

/// Represents an actor in the game. e.g. Player, Enemy.
#[derive(Debug)]
pub struct Actor {
    /// The unique identifier of the actor.
    pub(crate) id: EntityId,
    /// The position of the actor.
    pub(crate) position: Position,
    /// The kind of actor.
    pub(crate) kind: ActorKind,
    /// The stats of the actor.
    pub(crate) stats: Stats,
    /// The effects currently affecting the actor.
    pub(crate) effects: Vec<EffectInstance>,
}

impl Actor {
    /// Creates a new actor with the given position and kind.
    pub(crate) fn create(position: Position, kind: ActorKind) -> Self {
        Actor {
            id: EntityId::next_entity_id(),
            position,
            stats: kind.default_stats(),
            kind,
            effects: Vec::new(),
        }
    }

    /// Creates a new player actor with the given position.
    pub(crate) fn create_player(position: Position) -> Self {
        Actor::create(position, ActorKind::Player)
    }

    /// Returns the unique identifier of the actor.
    #[must_use]
    pub fn id(&self) -> EntityId {
        self.id
    }

    /// Returns the position of the actor.
    #[must_use]
    pub fn position(&self) -> Position {
        self.position
    }

    /// Returns the stats of the actor.
    #[must_use]
    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    /// Returns the effects currently affecting the actor.
    #[allow(dead_code)]
    #[must_use]
    pub(crate) fn effects(&self) -> &Vec<EffectInstance> {
        &self.effects
    }

    /// Returns true if the actor is alive.
    #[must_use]
    pub(crate) fn is_alive(&self) -> bool {
        self.stats.hp > 0
    }
}
