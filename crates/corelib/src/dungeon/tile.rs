/// Represents a tile in the dungeon.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum Tile {
    #[default]
    /// No tile.
    Empty = 0,
    /// A tile that can be walked on.
    Floor = 1,
}

impl Tile {
    /// Gets the tile at the given coordinates.
    pub(crate) fn is_walkable(&self) -> bool {
        matches!(self, Tile::Floor)
    }
}
