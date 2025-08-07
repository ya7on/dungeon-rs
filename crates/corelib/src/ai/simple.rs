use std::collections::VecDeque;

use crate::{
    Direction, GameState, events::GameEvent, mechanics::try_move,
    walk_map::WalkMap,
};

/// Simple AI implementation for entities.
/// 1. Moves towards the player.
/// 2. Attacks the player if within range.
pub(crate) fn simple_ai(
    state: &mut GameState,
    walk_map: &mut WalkMap,
) -> VecDeque<GameEvent> {
    let mut events = VecDeque::new();

    for entity in &mut state.entities {
        if !entity.is_alive() {
            continue;
        }

        let relative = entity.position - state.player.position;
        let dist = relative.x().abs() + relative.y().abs();

        if dist == 1 {
            let damage =
                (entity.stats.attack - state.player.stats.defense).max(1);
            state.player.stats.hp =
                state.player.stats.hp.saturating_sub(damage);
            continue;
        }

        for direction in [
            (relative.x > 0, Direction::West),
            (relative.x < 0, Direction::East),
            (relative.y > 0, Direction::North),
            (relative.y < 0, Direction::South),
        ] {
            let (condition, direction) = direction;
            if condition {
                if let Some((from, to)) = try_move(entity, direction, walk_map)
                {
                    events.push_back(GameEvent::EntityMoved {
                        id: entity.id(),
                        from,
                        to,
                    });
                    break;
                }
            }
        }
    }

    events
}
