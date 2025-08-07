use std::collections::HashSet;

use crate::Position;

/// `WalkMap` represents a set of positions where the entities can walk.
pub(crate) struct WalkMap {
    inner: HashSet<Position>,
}

impl WalkMap {
    /// Check if a position is walkable.
    pub(crate) fn is_walkable(&self, position: Position) -> bool {
        self.inner.contains(&position)
    }

    /// Move a position from one location to another.
    pub(crate) fn relocate(&mut self, from: Position, to: Position) {
        self.inner.insert(from);
        self.inner.remove(&to);
    }

    /// Mark position as occupied so it cannot be walked on.
    pub(crate) fn occupy(&mut self, position: Position) {
        self.inner.remove(&position);
    }
}

impl FromIterator<Position> for WalkMap {
    fn from_iter<T: IntoIterator<Item = Position>>(iter: T) -> Self {
        Self { inner: iter.into_iter().collect() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_occupy() {
        let mut walk_map =
            WalkMap::from_iter(vec![Position::new(0, 0), Position::new(1, 1)]);
        walk_map.occupy(Position::new(0, 0));
        assert!(!walk_map.is_walkable(Position::new(0, 0)));
        assert!(walk_map.is_walkable(Position::new(1, 1)));
    }

    #[test]
    fn test_relocate() {
        let mut walk_map =
            WalkMap::from_iter(vec![Position::new(0, 0), Position::new(1, 1)]);
        walk_map.relocate(Position::new(2, 2), Position::new(0, 0));
        assert!(!walk_map.is_walkable(Position::new(0, 0)));
        assert!(walk_map.is_walkable(Position::new(1, 1)));
        assert!(walk_map.is_walkable(Position::new(2, 2)));
    }
}
