use crate::{Direction, Position, actors::EntityId};

/// All events in the game.
pub enum GameEvent {
    /* --- Player events --- */
    /// Player skipped move event
    PlayerSkippedMove,
    /// Player died event
    PlayerDied,
    /// Player moved
    PlayerMoved { from: Position, to: Position },
    /// Player bumped into something
    PlayerBumped { position: Position, direction: Direction },
    /// Player attacked
    PlayerAttacked { target: EntityId, damage: u32 },
    /// Player attack missed
    PlayerAttackMissed,
    /* --- Entity events --- */
    // /// Entity created
    // EntityCreated { id: EntityId, position: Position },
    // /// Entity died event
    // EntityDied { id: EntityId },
    /// Entity moved
    EntityMoved { id: EntityId, from: Position, to: Position },
    // /// Entity attacked
    // EntityAttacked { id: EntityId, target: Position },
}
