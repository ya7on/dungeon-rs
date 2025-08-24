use crate::{
    GameState, actors::ActorKind, direction::Direction, events::GameEvent,
    mechanics::try_attack,
};

/// Attacks the enemy in the specified direction.
pub(crate) fn player_attack(
    state: &mut GameState,
    direction: Direction,
) -> Vec<GameEvent> {
    let player_position = state.player.position();
    let target_position = player_position + direction.to_offset_position();

    let Some(target) = state
        .entities
        .iter_mut()
        .find(|a| a.position == target_position && a.kind != ActorKind::Player)
    else {
        return vec![GameEvent::PlayerAttackMissed];
    };

    let damage = try_attack(&mut state.player, target, &mut state.rng);

    vec![GameEvent::PlayerAttacked { target: target.id(), damage }]
}

#[cfg(test)]
mod tests {

    use crate::{
        actors::Actor, dungeon::DungeonMap, position::Position, rng::MyRng,
    };

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
            DungeonMap::simple(10, 10),
            MyRng::from_seed([0; 32]),
        );
        player_attack(&mut gs, Direction::North);
        assert_eq!(gs.entities[0].stats.hp, 20);
        assert_eq!(gs.entities[0].position, Position::new(1, 0));
        assert_eq!(gs.entities[1].stats.hp, 20);
        assert_eq!(gs.entities[1].position, Position::new(-1, 0));
        assert_eq!(gs.entities[2].stats.hp, 20);
        assert_eq!(gs.entities[2].position, Position::new(0, 1));
        assert!(gs.entities[3].stats.hp < 20);
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
            DungeonMap::simple(10, 10),
            MyRng::from_seed([0; 32]),
        );
        player_attack(&mut gs, Direction::North);
        player_attack(&mut gs, Direction::North);
        player_attack(&mut gs, Direction::North);
        player_attack(&mut gs, Direction::North);
        player_attack(&mut gs, Direction::North);
        assert_eq!(gs.entities[0].stats.hp, 20);
        assert_eq!(gs.entities[0].position, Position::new(1, 0));
        assert_eq!(gs.entities[1].stats.hp, 20);
        assert_eq!(gs.entities[1].position, Position::new(-1, 0));
        assert_eq!(gs.entities[2].stats.hp, 20);
        assert_eq!(gs.entities[2].position, Position::new(0, 1));
        assert_eq!(gs.entities[3].stats.hp, 0);
        assert_eq!(gs.entities[3].position, Position::new(0, -1));
        assert!(!gs.entities[3].is_alive());
    }
}
