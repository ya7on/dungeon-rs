use crate::{
    actions::{PlayerAction, try_attack, try_move},
    actor::Actor,
    ai::simple_ai,
    dungeon::DungeonMap,
    rng::MyRng,
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
    /// The random number generator.
    pub(crate) rng: MyRng,
}

impl GameState {
    /// Creates a new game state with the given player and entities.
    pub(crate) fn new(
        player: Actor,
        entities: Vec<Actor>,
        map: DungeonMap,
        rng: MyRng,
    ) -> Self {
        GameState { tick_id: 0, player, entities, dungeon: map, rng }
    }

    /// Applies the given player action to the game state.
    pub fn apply_player_action(&mut self, action: PlayerAction) {
        match action {
            PlayerAction::Skip => {},
            PlayerAction::Move(direction) => {
                try_move(self, &direction);
            },
            PlayerAction::Attack(direction) => {
                try_attack(self, &direction);
            },
        }

        simple_ai(self);

        self.tick_id += 1;
    }

    /// Returns a reference to the player.
    #[must_use]
    pub fn player(&self) -> &Actor {
        &self.player
    }

    /// Returns a reference to the entities.
    #[must_use]
    pub fn entities(&self) -> &[Actor] {
        &self.entities
    }

    /// Returns a reference to the dungeon.
    #[must_use]
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
            DungeonMap::simple(10, 10),
            MyRng::new(),
        );

        gs.apply_player_action(PlayerAction::Skip);

        assert_eq!(gs.tick_id, 1);
    }
}
