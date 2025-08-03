use crate::{
    Actor, PlayerAction,
    actions::{try_attack, try_move},
};

/// Represents the state of the game.
#[derive(Debug)]
pub struct GameState {
    /// The ID of the current tick.
    pub(crate) tick_id: u64,
    /// The player
    pub(crate) player: Actor,
    /// Other entities in the game.
    pub(crate) entities: Vec<Actor>,
}

impl GameState {
    /// Creates a new game state with the given player and entities.
    pub fn new(player: Actor, entities: Vec<Actor>) -> Self {
        GameState {
            tick_id: 0,
            player,
            entities,
        }
    }

    /// Applies the given player action to the game state.
    pub fn apply_player_action(&mut self, action: PlayerAction) {
        match action {
            PlayerAction::Skip => {}
            PlayerAction::Move(direction) => {
                try_move(self, direction);
            }
            PlayerAction::Attack(direction) => {
                try_attack(self, direction);
            }
        }

        self.tick_id += 1;
    }

    /// Returns a reference to the player.
    pub fn player(&self) -> &Actor {
        &self.player
    }

    /// Removes an entity from the game state by its vector index.
    pub fn remove_entity_by_index(&mut self, index: usize) {
        self.entities.swap_remove(index);
    }
}

#[cfg(test)]
mod tests {
    use crate::Position;

    use super::*;

    #[test]
    fn test_tick_increment() {
        let mut gs = GameState::new(Actor::create_player(Position::new(0, 0)), vec![]);

        gs.apply_player_action(PlayerAction::Skip);

        assert_eq!(gs.tick_id, 1);
    }
}
