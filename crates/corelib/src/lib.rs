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

pub use actions::PlayerAction;
pub use actor::Actor;
pub use actor_stats::Stats;
pub use array2d::Array2D;
pub use direction::Direction;
pub use dungeon::{DungeonMap, Tile};
pub use game_state::GameState;
pub use position::Position;

/// Creates a new game instance.
pub fn new_game() -> GameState {
    const DEFAULT_SEED: u64 = 0; // TODO: Implement proper seeding
    const DEFAULT_MAP_WIDTH: usize = 100;
    const DEFAULT_MAP_HEIGHT: usize = 100;

    let map = dungeon::DungeonMap::generate(DEFAULT_MAP_WIDTH, DEFAULT_MAP_HEIGHT, DEFAULT_SEED);

    GameState::new(
        actor::Actor::create_player(position::Position { x: 0, y: 0 }),
        vec![
            Actor::create(Position { x: 0, y: -4 }, actor_kind::ActorKind::Enemy),
            Actor::create(Position { x: 0, y: 9 }, actor_kind::ActorKind::Enemy),
            Actor::create(Position { x: -1, y: 1 }, actor_kind::ActorKind::Enemy),
            Actor::create(Position { x: 9, y: -1 }, actor_kind::ActorKind::Enemy),
        ],
        map,
        DEFAULT_SEED,
    )
}
