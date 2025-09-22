use crate::{
    Direction, GameState,
    events::GameEvent,
    mechanics::{try_attack, try_move},
    step_result::StepContext,
    walk_map::WalkMap,
};

/// Simple AI implementation for entities.
/// 1. Moves towards the player.
/// 2. Attacks the player if within range.
pub(crate) fn simple_ai(
    state: &mut GameState,
    step_context: &mut StepContext,
    walk_map: &mut WalkMap,
) {
    for entity in &mut state.entities {
        if !entity.is_alive() {
            continue;
        }

        let relative = entity.position - state.player.position;
        let dist = relative.x().abs() + relative.y().abs();

        if dist == 1 {
            let damage = try_attack(entity, &mut state.player, &mut state.rng);
            step_context.add_event(GameEvent::EntityAttacked {
                id: entity.id(),
                target: state.player.position,
                damage,
            });
            continue;
        }

        for direction in [
            (relative.x > 0, Direction::West),
            (relative.x < 0, Direction::East),
            (relative.y > 0, Direction::North),
            (relative.y < 0, Direction::South),
        ] {
            let (condition, direction) = direction;
            if condition
                && let Some((from, to)) = try_move(entity, direction, walk_map)
            {
                step_context.add_event(GameEvent::EntityMoved {
                    id: entity.id(),
                    from,
                    to,
                });
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        actions::PlayerAction,
        actors::{Actor, ActorKind},
        dungeon::DungeonMap,
        position::Position,
        rng::MyRng,
    };

    fn setup_state(player_pos: Position, enemies: Vec<Position>) -> GameState {
        GameState::new(
            Actor::create_player(player_pos),
            enemies
                .into_iter()
                .map(|p| Actor::create(p, ActorKind::Enemy))
                .collect(),
            DungeonMap::simple(10, 10),
            MyRng::new(),
        )
    }

    #[test]
    fn enemy_moves_towards_player() {
        let mut gs =
            setup_state(Position::new(0, 0), vec![Position::new(2, 0)]);
        let result = gs.apply_player_action(&PlayerAction::Skip);
        let events: Vec<_> = result.events.into_iter().collect();
        match events[1] {
            GameEvent::EntityMoved { from, to, .. } => {
                assert_eq!(from, Position::new(2, 0));
                assert_eq!(to, Position::new(1, 0));
            },
            _ => panic!("expected entity movement"),
        }
    }

    #[test]
    fn enemy_attacks_when_adjacent() {
        let mut gs =
            setup_state(Position::new(0, 0), vec![Position::new(1, 0)]);
        let hp_before = gs.player.stats.hp;
        let result = gs.apply_player_action(&PlayerAction::Skip);
        assert!(result
            .events
            .iter()
            .any(|e| matches!(e, GameEvent::EntityAttacked { target, .. } if *target == Position::new(0,0))));
        assert!(gs.player.stats.hp < hp_before);
    }

    #[test]
    fn enemies_act_in_entity_id_order() {
        let mut gs = setup_state(
            Position::new(0, 0),
            vec![Position::new(2, 0), Position::new(0, 2)],
        );
        let result = gs.apply_player_action(&PlayerAction::Skip);
        let events: Vec<_> = result.events.into_iter().collect();
        match events[1] {
            GameEvent::EntityMoved { id, .. } => {
                assert_eq!(id, gs.entities[0].id());
            },
            _ => panic!("expected first entity event"),
        }
        match events[2] {
            GameEvent::EntityMoved { id, .. } => {
                assert_eq!(id, gs.entities[1].id());
            },
            _ => panic!("expected second entity event"),
        }
    }

    #[test]
    fn enemy_blocked_by_other_entity() {
        let mut gs = setup_state(
            Position::new(0, 0),
            vec![Position::new(2, 0), Position::new(1, 0)],
        );
        let before = gs.entities[0].position;
        let result = gs.apply_player_action(&PlayerAction::Skip);
        let events: Vec<_> = result.events.into_iter().collect();
        assert!(
            events
                .iter()
                .any(|e| matches!(e, GameEvent::EntityAttacked { .. }))
        );
        assert_eq!(gs.entities[0].position, before);
    }
}
