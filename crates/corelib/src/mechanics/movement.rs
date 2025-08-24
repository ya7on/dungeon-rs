use crate::{Actor, Direction, Position, walk_map::WalkMap};

/// Try to move an entity in a given direction.
pub(crate) fn try_move(
    entity: &mut Actor,
    direction: Direction,
    walk_map: &mut WalkMap,
) -> Option<(Position, Position)> {
    let old_position = entity.position;
    let new_position = entity.position + direction.to_offset_position();

    if walk_map.is_walkable(new_position) {
        entity.position = new_position;
        walk_map.relocate(old_position, new_position);
        Some((old_position, new_position))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{actors::ActorKind, dungeon::DungeonMap};

    fn setup_walk_map() -> (WalkMap, Actor) {
        let map = DungeonMap::simple(5, 5);
        let mut walk_map = map
            .iter()
            .filter_map(|(pos, tile)| tile.is_walkable().then_some(pos))
            .collect::<WalkMap>();
        let actor = Actor::create(Position::new(0, 0), ActorKind::Enemy);
        walk_map.occupy(actor.position);
        (walk_map, actor)
    }

    #[test]
    fn moves_to_walkable_tile() {
        let (mut walk_map, mut actor) = setup_walk_map();
        let result = try_move(&mut actor, Direction::East, &mut walk_map);
        assert_eq!(
            result,
            Some((Position::new(0, 0), Position::new(1, 0)))
        );
        assert_eq!(actor.position, Position::new(1, 0));
        assert!(!walk_map.is_walkable(Position::new(1, 0)));
        assert!(walk_map.is_walkable(Position::new(0, 0)));
    }

    #[test]
    fn blocked_move_returns_none() {
        let (mut walk_map, mut actor) = setup_walk_map();
        // East is walkable, occupy it to block
        walk_map.occupy(Position::new(1, 0));
        let result = try_move(&mut actor, Direction::East, &mut walk_map);
        assert!(result.is_none());
        assert_eq!(actor.position, Position::new(0, 0));
    }
}
