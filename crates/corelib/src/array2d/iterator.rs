use crate::Position;

use super::Array2D;

pub struct Array2DIterator<'a, T>
where
    T: Default + Clone,
{
    inner: &'a Array2D<T>,
    x: i32,
    y: i32,
}

impl<'a, T> Array2DIterator<'a, T>
where
    T: Default + Clone,
{
    pub fn new(map: &'a Array2D<T>) -> Self {
        let top_left = map.top_left();
        Self {
            inner: map,
            x: top_left.x,
            y: top_left.y,
        }
    }
}

impl<'a, T> Iterator for Array2DIterator<'a, T>
where
    T: Default + Clone,
{
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y > self.inner.half_height() {
            return None;
        }

        let position = Position {
            x: self.x,
            y: self.y,
        };
        let item = self.inner.get(&position);

        self.x += 1;

        if self.x > self.inner.half_width() {
            self.x = self.inner.top_left().x;
            self.y += 1;
        }

        if let Some(item) = item {
            Some((position, item))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let map = Array2D::<usize>::empty(3, 3);
        let mut iter = Array2DIterator::new(&map);

        assert_eq!(iter.next(), Some((Position { x: -1, y: -1 }, &0)));
        assert_eq!(iter.next(), Some((Position { x: 0, y: -1 }, &0)));
        assert_eq!(iter.next(), Some((Position { x: 1, y: -1 }, &0)));

        assert_eq!(iter.next(), Some((Position { x: -1, y: 0 }, &0)));
        assert_eq!(iter.next(), Some((Position { x: 0, y: 0 }, &0)));
        assert_eq!(iter.next(), Some((Position { x: 1, y: 0 }, &0)));

        assert_eq!(iter.next(), Some((Position { x: -1, y: 1 }, &0)));
        assert_eq!(iter.next(), Some((Position { x: 0, y: 1 }, &0)));
        assert_eq!(iter.next(), Some((Position { x: 1, y: 1 }, &0)));

        assert_eq!(iter.next(), None);
    }
}
