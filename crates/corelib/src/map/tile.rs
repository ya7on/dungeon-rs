#[derive(Debug, Clone)]
#[repr(u8)]
pub(crate) enum Tile {
    Empty = 0,
    Floor = 1,
}

impl Tile {
    /// Gets the tile at the given coordinates.
    pub(crate) fn is_walkable(&self) -> bool {
        match self {
            Tile::Floor => true,
            _ => false,
        }
    }
}
