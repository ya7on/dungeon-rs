use crate::{
    GameState, direction::Direction, events::GameEvent, mechanics::try_move,
    walk_map::WalkMap,
};

/// Moves the player in the specified direction.
pub(crate) fn player_move(
    state: &mut GameState,
    direction: Direction,
    walk_map: &mut WalkMap,
) -> Vec<GameEvent> {
    if let Some((old_position, new_position)) =
        try_move(&mut state.player, direction, walk_map)
    {
        vec![GameEvent::PlayerMoved { from: old_position, to: new_position }]
    } else {
        vec![GameEvent::PlayerBumped {
            position: state.player.position,
            direction,
        }]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        GameState, actions::PlayerAction, actor::Actor, direction::Direction,
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
