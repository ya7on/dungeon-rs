#[derive(Debug, Clone, Default)]
#[repr(u8)]
pub enum Tile {
    #[default]
    Empty = 0,
    Floor = 1,
}

impl Tile {
    /// Gets the tile at the given coordinates.
    pub(crate) fn is_walkable(&self) -> bool {
        matches!(self, Tile::Floor)
    }
}
