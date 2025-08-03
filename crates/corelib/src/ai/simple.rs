use crate::GameState;

/// Simple AI implementation for entities.
/// 1. Moves towards the player.
/// 2. Attacks the player if within range.
pub(crate) fn simple_ai(state: &mut GameState) {
    for entity in state.entities.iter_mut() {
        let relative = entity.position - state.player.position;
        let dist = relative.x().abs() + relative.y().abs();

        if dist == 1 {
            let damage = (entity.stats.attack - state.player.stats.defense).max(1);
            state.player.stats.hp = state.player.stats.hp.saturating_sub(damage);
        } else {
            if relative.x > 0 {
                entity.position.x -= 1;
            } else if relative.x < 0 {
                entity.position.x += 1;
            } else if relative.y > 0 {
                entity.position.y -= 1;
            } else if relative.y < 0 {
                entity.position.y += 1;
            }
        }
    }
}
