use std::collections::HashMap;

use crate::{GameState, Position};

/// Simple AI implementation for entities.
/// 1. Moves towards the player.
/// 2. Attacks the player if within range.
pub(crate) fn simple_ai(
    state: &mut GameState,
    walkable_map: &mut HashMap<Position, bool>,
) {
    for entity in &mut state.entities {
        let relative = entity.position - state.player.position;
        let dist = relative.x().abs() + relative.y().abs();

        if dist == 1 {
            let damage =
                (entity.stats.attack - state.player.stats.defense).max(1);
            state.player.stats.hp =
                state.player.stats.hp.saturating_sub(damage);
            continue;
        }

        if relative.x > 0 {
            let mut target = entity.position;
            target.x -= 1;
            if *walkable_map.get(&target).unwrap_or(&false) {
                walkable_map.insert(entity.position, true);
                entity.position = target;
                walkable_map.insert(entity.position, false);
                continue;
            }
        }

        if relative.x < 0 {
            let mut target = entity.position;
            target.x += 1;
            if *walkable_map.get(&target).unwrap_or(&false) {
                walkable_map.insert(entity.position, true);
                entity.position = target;
                walkable_map.insert(entity.position, false);
                continue;
            }
        }

        if relative.y > 0 {
            let mut target = entity.position;
            target.y -= 1;
            if *walkable_map.get(&target).unwrap_or(&false) {
                walkable_map.insert(entity.position, true);
                entity.position = target;
                walkable_map.insert(entity.position, false);
                continue;
            }
        }

        if relative.y < 0 {
            let mut target = entity.position;
            target.y += 1;
            if *walkable_map.get(&target).unwrap_or(&false) {
                walkable_map.insert(entity.position, true);
                entity.position = target;
                walkable_map.insert(entity.position, false);
            }
        }
    }
}
