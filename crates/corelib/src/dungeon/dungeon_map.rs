use crate::{Array2D, array2d::Array2DIterator, position::Position};

use super::tile::Tile;

#[derive(Debug)]
pub struct DungeonMap {
    tiles: Array2D<Tile>,
}

impl DungeonMap {
    /// Generates a new dungeon map with the given width and height.
    ///
    /// Temporary generates a simple dungeon map with floor tiles.
    pub(crate) fn generate(width: usize, height: usize, _seed: u64) -> Self {
        let mut tiles = Array2D::empty(width, height);
        // TODO: Implement proper dungeon generation algorithm.
        for x in -10..10 {
            for y in -10..10 {
                tiles.set(Position { x, y }, Tile::Floor);
            }
        }
        for x in 8..20 {
            for y in 8..20 {
                tiles.set(Position { x, y }, Tile::Floor);
            }
        }
        Self { tiles }
    }

    /// Returns the width of the dungeon map.
    pub fn width(&self) -> usize {
        self.tiles.width()
    }

    /// Returns the height of the dungeon map.
    pub fn height(&self) -> usize {
        self.tiles.height()
    }

    /// Returns a reference to the tile at the given position.
    pub fn get_tile(&self, position: &Position) -> &Tile {
        self.tiles.get(position).unwrap_or(&Tile::Empty)
    }

    pub fn iter(&self) -> Array2DIterator<Tile> {
        Array2DIterator::new(&self.tiles)
    }
}
