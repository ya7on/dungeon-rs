use crate::position::Position;

/// Heap allocated 2D array.
/// Center is always at (0, 0).
/// No reallocation. No size changes.
#[derive(Debug)]
pub struct Array2D<T>
where
    T: Clone + Default,
{
    width: usize,
    height: usize,
    inner: Box<[T]>,
}

impl<T> Array2D<T>
where
    T: Clone + Default,
{
    /// Creates a new empty Tiles instance.
    pub(crate) fn empty(width: usize, height: usize) -> Self {
        let inner = vec![T::default(); width * height].into_boxed_slice();
        Self { width, height, inner }
    }

    /// Returns the half width of `Array2D`.
    #[must_use]
    pub fn half_width(&self) -> i32 {
        i32::try_from(self.width / 2).unwrap_or(i32::MAX)
    }

    /// Returns the half height of `Array2D`.
    #[must_use]
    pub fn half_height(&self) -> i32 {
        i32::try_from(self.height / 2).unwrap_or(i32::MAX)
    }

    /// Returns the index of the tile at the given coordinates.
    #[must_use]
    fn position_to_index(&self, position: Position) -> Option<usize> {
        if !self.in_bounds(position) {
            return None;
        }

        let x = position.x() + self.half_width();
        let y = position.y() + self.half_height();

        let x = usize::try_from(x).unwrap_or_default();
        let y = usize::try_from(y).unwrap_or_default();

        Some(x + y * self.width)
    }

    /// Checks if the given position is within the bounds of the `Array2D`.
    #[must_use]
    pub(crate) fn in_bounds(&self, position: Position) -> bool {
        let x = position.x() + self.half_width();
        let y = position.y() + self.half_height();

        let width = i32::try_from(self.width).unwrap_or(i32::MAX);
        let height = i32::try_from(self.height).unwrap_or(i32::MAX);

        x >= 0 && y >= 0 && x < width && y < height
    }

    /// Gets the tile at the given coordinates.
    #[must_use]
    pub(crate) fn get(&self, position: Position) -> Option<&T> {
        let index = self.position_to_index(position)?;

        self.inner.get(index)
    }

    /// Sets the tile at the given coordinates.
    pub(crate) fn set(&mut self, position: Position, tile: T) {
        let Some(index) = self.position_to_index(position) else {
            return;
        };

        self.inner[index] = tile;
    }

    /// Returns the width of the `Array2D`.
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the `Array2D`.
    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the top-left position of the `Array2D`.
    #[must_use]
    pub fn top_left(&self) -> Position {
        Position::new(-self.half_width(), -self.half_height())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tile() {
        let array = Array2D {
            width: 3,
            height: 3,
            inner: vec![
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (0, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ]
            .into_boxed_slice(),
        };

        let position = Position::new(0, 0);
        let tile = array.get(position);
        assert_eq!(tile, Some(&(0, 0)));
        let position = Position::new(1, 1);
        let tile = array.get(position);
        assert_eq!(tile, Some(&(1, 1)));
        let position = Position::new(-1, -1);
        let tile = array.get(position);
        assert_eq!(tile, Some(&(-1, -1)));

        let position = Position::new(2, 1);
        let tile = array.get(position);
        assert_eq!(tile, None);
    }

    #[test]
    fn test_set_tile() {
        let mut array = Array2D::<usize>::empty(3, 3);

        let position = Position::new(0, 0);
        array.set(position, 100_500);
        let tile = array.get(position);
        assert_eq!(tile, Some(&100_500));
    }

    #[test]
    fn test_top_left() {
        {
            let array = Array2D::<usize>::empty(3, 5);
            let position = array.top_left();
            assert_eq!(position, Position::new(-1, -2));
        }

        {
            let array = Array2D::<usize>::empty(1, 9);
            let position = array.top_left();
            assert_eq!(position, Position::new(0, -4));
        }
    }
}
