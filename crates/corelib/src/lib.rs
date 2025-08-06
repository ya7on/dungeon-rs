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

/// Creates a new game instance.
#[must_use]
pub fn new_game() -> GameState {
    const DEFAULT_SEED: [u8; 32] = [0; 32];
    const DEFAULT_MAP_WIDTH: usize = 101;
    const DEFAULT_MAP_HEIGHT: usize = 101;

    let mut rng = MyRng::from_seed(DEFAULT_SEED);

    let map = dungeon::DungeonMap::generate(
        DEFAULT_MAP_WIDTH,
        DEFAULT_MAP_HEIGHT,
        &mut rng,
    );

    let mut entities = Vec::with_capacity(10);
    while entities.len() < entities.capacity() {
        let half_width =
            i32::try_from(DEFAULT_MAP_WIDTH).unwrap_or(i32::MAX) / 2;
        let half_height =
            i32::try_from(DEFAULT_MAP_HEIGHT).unwrap_or(i32::MAX) / 2;
        let x = rng.range(-half_width..=half_width);
        let y = rng.range(-half_height..=half_height);

        if map.get_tile(Position { x, y }) == &Tile::Floor {
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
