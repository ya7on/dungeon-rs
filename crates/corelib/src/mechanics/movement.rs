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
