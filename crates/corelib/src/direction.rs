use crate::position::Position;

/// Represents a direction in the game world.
#[derive(Debug)]
#[repr(u8)]
pub enum Direction {
    /// North direction.
    North,
    /// South direction.
    South,
    /// East direction.
    East,
    /// West direction.
    West,
}

impl Direction {
    /// Returns the offset of the direction.
    pub(crate) fn to_offset_position(&self) -> Position {
        match self {
            Direction::North => Position::new(0, -1),
            Direction::South => Position::new(0, 1),
            Direction::East => Position::new(1, 0),
            Direction::West => Position::new(-1, 0),
        }
    }
}
