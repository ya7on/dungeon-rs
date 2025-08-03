mod try_attack;
mod try_move;

pub(crate) use try_attack::try_attack;
pub(crate) use try_move::try_move;

use crate::direction::Direction;

/// Represents an action that a player can take.
#[derive(Debug)]
pub enum PlayerAction {
    /// Skip the current turn.
    Skip,
    /// Move in the specified direction.
    Move(Direction),
    /// Attack in the specified direction.
    Attack(Direction),
}
