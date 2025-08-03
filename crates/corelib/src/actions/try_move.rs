use crate::{GameState, direction::Direction};

/// Moves the player in the specified direction.
pub(crate) fn try_move(state: &mut GameState, direction: Direction) {
    // TODO: Implement movement logic with collision detection and boundary checks
    state.player.position = state.player.position + direction.to_offset_position();
}

#[cfg(test)]
mod tests {
    use crate::{
        GameState, actions::PlayerAction, actor::Actor, direction::Direction, position::Position,
    };

    #[test]
    fn test_player_movement() {
        // North
        {
            let mut gs = GameState::new(Actor::create_player(Position::new(0, 0)), vec![]);
            gs.apply_player_action(PlayerAction::Move(Direction::North));
            assert_eq!(gs.player.position, Position::new(0, -1));
        }

        // South
        {
            let mut gs = GameState::new(Actor::create_player(Position::new(0, 0)), vec![]);
            gs.apply_player_action(PlayerAction::Move(Direction::South));
            assert_eq!(gs.player.position, Position::new(0, 1));
        }

        // East
        {
            let mut gs = GameState::new(Actor::create_player(Position::new(0, 0)), vec![]);
            gs.apply_player_action(PlayerAction::Move(Direction::East));
            assert_eq!(gs.player.position, Position::new(1, 0));
        }

        // West
        {
            let mut gs = GameState::new(Actor::create_player(Position::new(0, 0)), vec![]);
            gs.apply_player_action(PlayerAction::Move(Direction::West));
            assert_eq!(gs.player.position, Position::new(-1, 0));
        }
    }
}
