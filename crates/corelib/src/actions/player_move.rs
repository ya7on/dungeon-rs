use crate::{
    GameState, direction::Direction, events::GameEvent, mechanics::try_move,
    step_result::StepContext, walk_map::WalkMap,
};

/// Moves the player in the specified direction.
pub(crate) fn player_move(
    state: &mut GameState,
    step_context: &mut StepContext,
    direction: Direction,
    walk_map: &mut WalkMap,
) {
    if let Some((old_position, new_position)) =
        try_move(&mut state.player, direction, walk_map)
    {
        step_context.add_event(GameEvent::PlayerMoved {
            from: old_position,
            to: new_position,
        });
    } else {
        step_context.add_event(GameEvent::PlayerBumped {
            position: state.player.position,
            direction,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        GameState, actions::PlayerAction, actors::Actor, direction::Direction,
        dungeon::DungeonMap, position::Position, rng::MyRng,
    };

    #[test]
    fn test_player_movement() {
        // North
        {
            let mut gs = GameState::new(
                Actor::create_player(Position::new(1, 1)),
                vec![],
                DungeonMap::simple(10, 10),
                MyRng::new(),
            );
            gs.apply_player_action(&PlayerAction::Move(Direction::North));
            assert_eq!(gs.player.position, Position::new(1, 0));
        }

        // South
        {
            let mut gs = GameState::new(
                Actor::create_player(Position::new(1, 1)),
                vec![],
                DungeonMap::simple(10, 10),
                MyRng::new(),
            );
            gs.apply_player_action(&PlayerAction::Move(Direction::South));
            assert_eq!(gs.player.position, Position::new(1, 2));
        }

        // East
        {
            let mut gs = GameState::new(
                Actor::create_player(Position::new(1, 1)),
                vec![],
                DungeonMap::simple(10, 10),
                MyRng::new(),
            );
            gs.apply_player_action(&PlayerAction::Move(Direction::East));
            assert_eq!(gs.player.position, Position::new(2, 1));
        }

        // West
        {
            let mut gs = GameState::new(
                Actor::create_player(Position::new(1, 1)),
                vec![],
                DungeonMap::simple(10, 10),
                MyRng::new(),
            );
            gs.apply_player_action(&PlayerAction::Move(Direction::West));
            assert_eq!(gs.player.position, Position::new(0, 1));
        }
    }

    #[test]
    fn test_move_only_to_walkable() {
        let mut gs = GameState::new(
            Actor::create_player(Position::new(0, -5)),
            vec![],
            DungeonMap::simple(10, 10),
            MyRng::new(),
        );

        // Not allowed to move
        gs.apply_player_action(&PlayerAction::Move(Direction::North));
        assert_eq!(gs.player.position, Position::new(0, -5));

        // Allowed to move to walkable
        gs.apply_player_action(&PlayerAction::Move(Direction::East));
        assert_eq!(gs.player.position, Position::new(1, -5));
    }
}
