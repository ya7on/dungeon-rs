use crate::position::Position;

use super::tile::Tile;

/// Heap allocated array of Tile.
#[derive(Debug)]
pub(crate) struct Tiles {
    width: usize,
    height: usize,
    inner: Box<[Tile]>,
}

impl Tiles {
    /// Creates a new empty Tiles instance.
    pub(crate) fn empty(width: usize, height: usize) -> Self {
        let inner = vec![Tile::Empty; width * height].into_boxed_slice();
        Self {
            width,
            height,
            inner,
        }
    }

    /// Returns the half width of the dungeon map.
    fn half_width(&self) -> i32 {
        (self.width / 2) as i32
    }

    /// Returns the half height of the dungeon map.
    fn half_height(&self) -> i32 {
        (self.height / 2) as i32
    }

    /// Returns the index of the tile at the given coordinates.
    fn position_to_index(&self, position: Position) -> Option<usize> {
        if !self.in_bounds(position) {
            return None;
        }

        let x = position.x() + self.half_width();
        let y = position.y() + self.half_height();

        let x = x as usize;
        let y = y as usize;

        Some(x + y * self.width)
    }

    /// Checks if the given position is within the bounds of the dungeon map.
    pub(crate) fn in_bounds(&self, position: Position) -> bool {
        let x = position.x() + self.half_width();
        let y = position.y() + self.half_height();

        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
    }

    /// Gets the tile at the given coordinates.
    pub(crate) fn get_tile(&self, position: Position) -> &Tile {
        let Some(index) = self.position_to_index(position) else {
            return &Tile::Empty;
        };

        self.inner.get(index).unwrap_or(&Tile::Empty)
    }

    /// Sets the tile at the given coordinates.
    pub(crate) fn set_tile(&mut self, position: Position, tile: Tile) {
        let Some(index) = self.position_to_index(position) else {
            return;
        };

        self.inner[index] = tile;
    }
}
