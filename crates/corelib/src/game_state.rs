use std::collections::VecDeque;

use crate::{
    actions::{PlayerAction, player_attack, player_move},
    actor::Actor,
    ai::simple_ai,
    dungeon::DungeonMap,
    events::GameEvent,
    rng::MyRng,
    walk_map::WalkMap,
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
    pub fn apply_player_action(
        &mut self,
        action: &PlayerAction,
    ) -> VecDeque<GameEvent> {
        let mut walk_map = self
            .dungeon
            .iter()
            .filter_map(|(position, tile)| {
                (tile.is_walkable()).then_some(position)
            })
            .collect::<WalkMap>();
        for entity in &self.entities {
            walk_map.occupy(entity.position);
        }
        walk_map.occupy(self.player.position);

        let mut events = VecDeque::new();

        match action {
            PlayerAction::Skip => {
                events.push_back(GameEvent::PlayerSkippedMove);
            },
            PlayerAction::Move(direction) => {
                events.extend(player_move(self, *direction, &mut walk_map));
            },
            PlayerAction::Attack(direction) => {
                events.extend(player_attack(self, *direction));
            },
        }

        events.extend(simple_ai(self, &mut walk_map));

        self.tick_id += 1;

        events
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

        gs.apply_player_action(&PlayerAction::Skip);

        assert_eq!(gs.tick_id, 1);
    }
}
