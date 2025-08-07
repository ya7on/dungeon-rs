#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

//! This module contains the core game logic and data structures.

mod actions;
mod actor;
mod actor_kind;
mod actor_stats;
mod ai;
mod array2d;
mod direction;
mod dungeon;
mod events;
mod game_state;
mod position;
mod rng;

pub use actions::PlayerAction;
pub use actor::Actor;
use actor_kind::ActorKind;
pub use actor_stats::Stats;
pub use array2d::Array2D;
pub use direction::Direction;
pub use dungeon::{DungeonMap, Tile};
pub use game_state::GameState;
pub use position::Position;
use rng::MyRng;

/// Settings for the world generation.
pub struct WorldSettings {
    /// Seed for the random number generator.
    pub seed: [u8; 32],
    /// Width of the map.
    pub map_width: usize,
    /// Height of the map.
    pub map_height: usize,
    /// Number of floor tiles.
    pub floor_tiles: usize,
    /// Number of enemies.
    pub enemies: usize,
}

/// Creates a new game instance.
#[must_use]
pub fn new_game(settings: &WorldSettings) -> GameState {
    let mut rng = MyRng::from_seed(settings.seed);

    let map = dungeon::DungeonMap::generate(
        settings.map_width,
        settings.map_height,
        &mut rng,
        settings.floor_tiles,
    );

    let mut entities = Vec::with_capacity(settings.enemies);
    while entities.len() < entities.capacity() {
        let half_width =
            i32::try_from(settings.map_width).unwrap_or(i32::MAX) / 2;
        let half_height =
            i32::try_from(settings.map_height).unwrap_or(i32::MAX) / 2;
        let x = rng.range(-half_width..=half_width);
        let y = rng.range(-half_height..=half_height);

        if map.is_walkable(Position { x, y }) {
            entities.push(actor::Actor::create(
                position::Position { x, y },
                ActorKind::Enemy,
            ));
        }
    }

    GameState::new(
        actor::Actor::create_player(position::Position { x: 0, y: 0 }),
        entities,
        map,
        rng,
    )
}
