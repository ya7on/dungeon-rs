use super::tile::{Tile, Tiles};

#[derive(Debug)]
pub(crate) struct DungeonMap {
    tiles: Tiles,
}

// TODO: maybe temporary?
impl Default for DungeonMap {
    fn default() -> Self {
        const DEFAULT_WIDTH: usize = 10;
        const DEFAULT_HEIGHT: usize = 10;

        Self {
            tiles: vec![Tile::Empty; DEFAULT_WIDTH * DEFAULT_HEIGHT].into_boxed_slice(),
        }
    }
}

impl DungeonMap {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![Tile::Empty; width * height].into_boxed_slice(),
        }
    }
}
