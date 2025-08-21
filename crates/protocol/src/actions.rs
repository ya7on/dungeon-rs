use crate::Direction;

/// Represents a player's action in the game.
pub enum PlayerAction {
    /// Move the player in a specific direction.
    Move(Direction),
}
