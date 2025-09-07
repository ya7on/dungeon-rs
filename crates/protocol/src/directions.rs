use serde::{Deserialize, Serialize};

/// Represents a direction in the game.
#[derive(Serialize, Deserialize)]
pub enum Direction {
    /// Represents the direction North.
    North,
    /// Represents the direction South.
    South,
    /// Represents the direction East.
    East,
    /// Represents the direction West.
    West,
}
