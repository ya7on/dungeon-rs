use crate::GameState;

/// Simple AI implementation for entities.
/// 1. Moves towards the player.
/// 2. Attacks the player if within range.
pub(crate) fn simple_ai(state: &mut GameState) {
    let occupied_positions: Vec<_> = state
        .entities
        .iter()
        .map(|e| e.position)
        .chain(std::iter::once(state.player.position))
        .collect();

    for entity in &mut state.entities {
        let relative = entity.position - state.player.position;
        let dist = relative.x().abs() + relative.y().abs();

        if dist == 1 {
            let damage = (entity.stats.attack - state.player.stats.defense).max(1);
            state.player.stats.hp = state.player.stats.hp.saturating_sub(damage);
        } else {
            if relative.x > 0 {
                let mut target = entity.position;
                target.x -= 1;
                if !occupied_positions.contains(&target) {
                    entity.position = target;
                    continue;
                }
            }

            if relative.x < 0 {
                let mut target = entity.position;
                target.x += 1;
                if !occupied_positions.contains(&target) {
                    entity.position = target;
                    continue;
                }
            }

            if relative.y > 0 {
                let mut target = entity.position;
                target.y -= 1;
                if !occupied_positions.contains(&target) {
                    entity.position = target;
                    continue;
                }
            }

            if relative.y < 0 {
                let mut target = entity.position;
                target.y += 1;
                if !occupied_positions.contains(&target) {
                    entity.position = target;
                    continue;
                }
            }
        }
    }
}
