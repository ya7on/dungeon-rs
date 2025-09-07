use serde::{Deserialize, Serialize};

/// Represents a position in the game.
#[derive(Serialize, Deserialize)]
pub struct Position {
    /// The x-coordinate of the position.
    pub x: i32,
    /// The y-coordinate of the position.
    pub y: i32,
}
