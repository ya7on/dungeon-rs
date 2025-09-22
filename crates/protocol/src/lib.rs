#![warn(clippy::all)]
#![warn(clippy::pedantic)]
// #![warn(clippy::nursery)] TODO: update code with this linting rule
#![warn(missing_docs)]
// #![warn(clippy::cargo)] Update Cargo.toml
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![forbid(unsafe_code)]

//! This crate provides the protocol for the dungeon game.
//! It defines the data structures and messages used to communicate between the frontend and backend.

mod actions;
mod diff;
mod directions;
mod entity_id;
mod event;
mod position;
mod state;
mod step;

pub use actions::PlayerAction;
pub use diff::{EntityDiff, StateDiff};
pub use directions::Direction;
pub use entity_id::EntityId;
pub use event::GameEvent;
pub use position::Position;
pub use state::State;
pub use step::StepResult;
