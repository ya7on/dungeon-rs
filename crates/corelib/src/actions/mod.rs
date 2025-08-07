mod player_attack;
mod player_move;

pub(crate) use player_attack::player_attack;
pub(crate) use player_move::player_move;

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
