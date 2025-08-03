use crate::Direction;

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
