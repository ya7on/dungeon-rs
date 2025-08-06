use crate::{
    Array2D, array2d::Array2DIterator, position::Position, rng::MyRng,
};

use super::tile::Tile;

/// Represents a dungeon map.
#[derive(Debug)]
pub struct DungeonMap {
    tiles: Array2D<Tile>,
}

impl DungeonMap {
    /// Generates a new dungeon map with the given width and height.
    ///
    /// Temporary generates a simple dungeon map with floor tiles.
    pub(crate) fn generate(
        width: usize,
        height: usize,
        rng: &mut MyRng,
    ) -> Self {
        const MAX_FLOOR_TILES: usize = 1000;
        let mut tiles = Array2D::empty(width, height);
        let mut current_position = Position { x: 0, y: 0 };
        let mut total_tiles = 0;
        while total_tiles < MAX_FLOOR_TILES {
            if tiles.get(current_position) == Some(&Tile::Empty) {
                tiles.set(current_position, Tile::Floor);
                total_tiles += 1;
            }

            let walk_to_x = rng.range(0..=1) != 0;
            if walk_to_x {
                current_position += Position { x: rng.range(-1..=1), y: 0 };
            } else {
                current_position += Position { x: 0, y: rng.range(-1..=1) };
            }

            current_position.x = current_position.x.min(tiles.half_width());
            current_position.x = current_position.x.max(-tiles.half_width());
            current_position.y = current_position.y.min(tiles.half_height());
            current_position.y = current_position.y.max(-tiles.half_height());
        }
        Self { tiles }
    }

    /// Generates a simple dungeon map with floor tiles.
    ///
    /// TODO: Remove this function
    pub(crate) fn simple(width: usize, height: usize) -> Self {
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
    #[must_use]
    pub fn width(&self) -> usize {
        self.tiles.width()
    }

    /// Returns the height of the dungeon map.
    #[must_use]
    pub fn height(&self) -> usize {
        self.tiles.height()
    }

    /// Returns a reference to the tile at the given position.
    #[must_use]
    pub fn get_tile(&self, position: Position) -> &Tile {
        self.tiles.get(position).unwrap_or(&Tile::Empty)
    }

    /// Returns an iterator over the tiles in the dungeon map.
    pub fn iter(&self) -> impl Iterator<Item = (Position, &Tile)> {
        <&Self as IntoIterator>::into_iter(self)
    }
}

impl<'a> IntoIterator for &'a DungeonMap {
    type Item = (Position, &'a Tile);
    type IntoIter = Array2DIterator<'a, Tile>;

    fn into_iter(self) -> Self::IntoIter {
        Array2DIterator::new(&self.tiles)
    }
}
