use crate::{GameState, direction::Direction};

/// Moves the player in the specified direction.
pub(crate) fn try_move(state: &mut GameState, direction: &Direction) {
    let new_position = state.player.position + direction.to_offset_position();

    let tile = state.dungeon.get_tile(new_position);
    if !tile.is_walkable() {
        // TODO: Implement collision detection and boundary checks
        return;
    }

    if state.entities.iter().any(|entity| entity.position == new_position) {
        // TODO: Implement collision detection and boundary checks
        return;
    }

    // TODO: Implement movement logic with collision detection and boundary checks
    state.player.position = new_position;
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
            gs.apply_player_action(PlayerAction::Move(Direction::North));
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
            gs.apply_player_action(PlayerAction::Move(Direction::South));
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
            gs.apply_player_action(PlayerAction::Move(Direction::East));
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
            gs.apply_player_action(PlayerAction::Move(Direction::West));
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
        gs.apply_player_action(PlayerAction::Move(Direction::North));
        assert_eq!(gs.player.position, Position::new(0, -5));

        // Allowed to move to walkable
        gs.apply_player_action(PlayerAction::Move(Direction::East));
        assert_eq!(gs.player.position, Position::new(1, -5));
    }
}
