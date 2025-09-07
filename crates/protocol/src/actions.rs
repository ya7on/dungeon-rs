use serde::{Deserialize, Serialize};

use crate::Direction;

/// Represents a player's action in the game.
#[derive(Serialize, Deserialize)]
pub enum PlayerAction {
    /// Move the player in a specific direction.
    Move(Direction),
}
