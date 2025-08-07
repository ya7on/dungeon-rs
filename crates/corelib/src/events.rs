use crate::{Position, actor::EntityId};

/// All events in the game.
pub(crate) enum Events {
    /* --- Player events --- */
    /// Player skipped move event
    PlayerSkippedMove,
    /// Player died event
    PlayerDied,
    /// Player moved
    PlayerMoved { from: Position, to: Position },
    /// Player attacked
    PlayerAttacked { target: Position },

    /* --- Entity events --- */
    /// Entity created
    EntityCreated { id: EntityId, position: Position },
    /// Entity died event
    EntityDied { id: EntityId },
    /// Entity moved
    EntityMoved { id: EntityId, from: Position, to: Position },
    /// Entity attacked
    EntityAttacked { id: EntityId, target: Position },
}
