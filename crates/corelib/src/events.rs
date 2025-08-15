use crate::{
    Direction, Position,
    actors::EntityId,
    items::{ItemId, SlotId},
};

/// All events in the game.
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
        target: EntityId,
        /// Player's damage
        damage: u32,
    },
    /// Player attack missed
    PlayerAttackMissed,
    /// Player equipped item
    PlayerEquippedItem {
        /// Item ID
        item_id: ItemId,
        /// Slot ID
        slot: SlotId,
    },
    /// Player unequipped item
    PlayerUnequippedItem {
        /// Slot ID
        slot: SlotId,
    },
    /* --- Entity events --- */
    // /// Entity created
    // EntityCreated { id: EntityId, position: Position },
    // /// Entity died event
    // EntityDied { id: EntityId },
    /// Entity moved
    EntityMoved {
        /// Entity ID
        id: EntityId,
        /// Entity's previous position
        from: Position,
        /// Entity's new position
        to: Position,
    },
    /// Entity attacked
    EntityAttacked {
        /// Entity ID
        id: EntityId,
        /// Entity's target position
        target: Position,
        /// Entity's damage
        damage: u32,
    },
}
