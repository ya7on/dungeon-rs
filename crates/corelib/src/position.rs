use std::ops::Add;

/// Represents a position in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Position {
    /// The x-coordinate of the position.
    pub(crate) x: i32,
    /// The y-coordinate of the position.
    pub(crate) y: i32,
}

impl Position {
    /// Creates a new position with the given coordinates.
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    /// Returns the x-coordinate of the position.
    pub(crate) fn x(&self) -> i32 {
        self.x
    }

    /// Returns the y-coordinate of the position.
    pub(crate) fn y(&self) -> i32 {
        self.y
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
