use crate::{
    actions::{PlayerAction, try_attack, try_move},
    actor::Actor,
    dungeon::DungeonMap,
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
    /// The dungeon map.
    pub(crate) dungeon: DungeonMap,
    /// The seed for random number generation.
    pub(crate) seed: u64,
}

impl GameState {
    /// Creates a new game state with the given player and entities.
    pub(crate) fn new(player: Actor, entities: Vec<Actor>, map: DungeonMap, seed: u64) -> Self {
        GameState {
            tick_id: 0,
            player,
            entities,
            dungeon: map,
            seed,
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

    pub fn dungeon(&self) -> &DungeonMap {
        &self.dungeon
    }

    /// Removes an entity from the game state by its vector index.
    pub(crate) fn remove_entity_by_index(&mut self, index: usize) {
        self.entities.swap_remove(index);
    }
}

#[cfg(test)]
mod tests {

    use crate::position::Position;

    use super::*;

    #[test]
    fn test_tick_increment() {
        let mut gs = GameState::new(
            Actor::create_player(Position::new(0, 0)),
            vec![],
            DungeonMap::generate(10, 10, 0),
            42,
        );

        gs.apply_player_action(PlayerAction::Skip);

        assert_eq!(gs.tick_id, 1);
    }
}
