use crate::position::Position;

use super::{tile::Tile, tiles::Tiles};

#[derive(Debug)]
pub struct DungeonMap {
    tiles: Tiles,
}

impl DungeonMap {
    /// Generates a new dungeon map with the given width and height.
    ///
    /// Temporary generates a simple dungeon map with floor tiles.
    pub(crate) fn generate(width: usize, height: usize, _seed: u64) -> Self {
        let mut tiles = Tiles::empty(width, height);
        // TODO: Implement proper dungeon generation algorithm.
        for x in 0..width {
            for y in 0..height {
                tiles.set_tile(
                    Position {
                        x: x as i32,
                        y: y as i32,
                    },
                    Tile::Floor,
                );
            }
        }
        Self { tiles }
    }

    /// Returns a reference to the tile at the given position.
    pub fn get_tile(&self, position: &Position) -> &Tile {
        self.tiles.get_tile(position)
    }
}
