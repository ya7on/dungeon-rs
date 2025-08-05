use crate::{GameState, actor_kind::ActorKind, direction::Direction};

/// Attacks the enemy in the specified direction.
pub(crate) fn try_attack(state: &mut GameState, dir: &Direction) {
    let (target_pos, player_attack) = {
        let attack_pos = dir.to_offset_position();
        let player = state.player();
        (player.position + attack_pos, player.stats.attack)
    };

    if let Some((idx, target)) =
        state.entities.iter_mut().enumerate().find(|(_, a)| {
            a.position == target_pos && a.kind != ActorKind::Player
        })
    {
        let damage = (player_attack - target.stats.defense).max(1);
        target.stats.hp = target.stats.hp.saturating_sub(damage);

        if target.stats.hp == 0 {
            // TODO: Implement enemy death logic
            state.remove_entity_by_index(idx);
        }
    } else {
        // TODO: Implement attack miss logic
    }
}

#[cfg(test)]
mod tests {

    use crate::{actor::Actor, dungeon::DungeonMap, position::Position};

    use super::*;

    #[test]
    fn test_hp_decrease() {
        let mut gs = GameState::new(
            Actor::create_player(Position::new(0, 0)),
            vec![
                Actor::create(Position { x: 1, y: 0 }, ActorKind::Enemy),
                Actor::create(Position { x: -1, y: 0 }, ActorKind::Enemy),
                Actor::create(Position { x: 0, y: 1 }, ActorKind::Enemy),
                Actor::create(Position { x: 0, y: -1 }, ActorKind::Enemy),
            ],
            DungeonMap::generate(10, 10, 0),
            0,
        );
        try_attack(&mut gs, &Direction::North);
        assert_eq!(gs.entities[0].stats.hp, 20);
        assert_eq!(gs.entities[0].position, Position::new(1, 0));
        assert_eq!(gs.entities[1].stats.hp, 20);
        assert_eq!(gs.entities[1].position, Position::new(-1, 0));
        assert_eq!(gs.entities[2].stats.hp, 20);
        assert_eq!(gs.entities[2].position, Position::new(0, 1));
        assert_eq!(gs.entities[3].stats.hp, 16);
        assert_eq!(gs.entities[3].position, Position::new(0, -1));
    }

    #[test]
    fn test_deleting_dead_enemy() {
        let mut gs = GameState::new(
            Actor::create_player(Position::new(0, 0)),
            vec![
                Actor::create(Position { x: 1, y: 0 }, ActorKind::Enemy),
                Actor::create(Position { x: -1, y: 0 }, ActorKind::Enemy),
                Actor::create(Position { x: 0, y: 1 }, ActorKind::Enemy),
                Actor::create(Position { x: 0, y: -1 }, ActorKind::Enemy),
            ],
            DungeonMap::generate(10, 10, 0),
            0,
        );
        try_attack(&mut gs, &Direction::North);
        try_attack(&mut gs, &Direction::North);
        try_attack(&mut gs, &Direction::North);
        try_attack(&mut gs, &Direction::North);
        try_attack(&mut gs, &Direction::North);
        assert_eq!(gs.entities.len(), 3);
        assert_eq!(gs.entities[0].stats.hp, 20);
        assert_eq!(gs.entities[0].position, Position::new(1, 0));
        assert_eq!(gs.entities[1].stats.hp, 20);
        assert_eq!(gs.entities[1].position, Position::new(-1, 0));
        assert_eq!(gs.entities[2].stats.hp, 20);
        assert_eq!(gs.entities[2].position, Position::new(0, 1));
    }
}
