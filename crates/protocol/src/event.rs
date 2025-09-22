use serde::{Deserialize, Serialize};

use crate::{Direction, Position};

/// Represents an event that occurs in the game.
#[derive(Serialize, Deserialize)]
pub enum GameEvent {
    /* --- Player events --- */
    /// Player skipped move event
    PlayerSkippedMove,
    /// Player died event
    PlayerDied,
    /// Player moved
    PlayerMoved {
        /// Player's previous position
        from: Position,
        /// Player's new position
        to: Position,
    },
    /// Player bumped into something
    PlayerBumped {
        /// Player's position
        position: Position,
        /// Player's direction
        direction: Direction,
    },
    /// Player attacked
    PlayerAttacked {
        /// Player's target entity ID
        target: u32,
        /// Player's damage
        damage: u32,
    },
    /// Player attack missed
    PlayerAttackMissed,
    /// Player equipped item
    PlayerEquippedItem {
        /// Item ID
        item_id: usize,
        /// Slot ID
        slot: usize,
    },
    /// Player unequipped item
    PlayerUnequippedItem {
        /// Slot ID
        slot: usize,
    },
    /* --- Entity events --- */
    /// Entity moved
    EntityMoved {
        /// Entity ID
        id: u32,
        /// Entity's previous position
        from: Position,
        /// Entity's new position
        to: Position,
    },
    /// Entity attacked
    EntityAttacked {
        /// Entity ID
        id: u32,
        /// Entity's target position
        target: Position,
        /// Entity's damage
        damage: u32,
    },
    /// Entity effect ticked
    EffectTick {
        /// Entity ID
        entity_id: u32,
        /// Effect ID
        effect_id: usize,
    },
    /// Effect expired for entity
    EffectExpired {
        /// Entity ID
        entity_id: u32,
        /// Effect ID
        effect_id: usize,
    },
}
