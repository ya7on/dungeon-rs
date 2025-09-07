#![warn(clippy::all)]
#![warn(clippy::pedantic)]
// #![warn(clippy::nursery)] TODO: update code with this linting rule
#![warn(missing_docs)]
// #![warn(clippy::cargo)] Update Cargo.toml
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![forbid(unsafe_code)]

//! This module contains the engine for the dungeon game.

use std::sync::{Arc, Mutex};

use protocol::{PlayerAction, State, StepResult};
use transport::{LocalState, Transport, TransportError};

pub use transport::LocalTransport;

/// This type represents the engine for the dungeon game.
pub type LocalEngine = Engine<LocalTransport>;

/// This struct represents the engine for the dungeon game.
pub struct Engine<T: Transport> {
    transport: T,
}

impl Engine<LocalTransport> {
    /// Creates a new instance of the corelib.
    #[must_use]
    pub fn new_state() -> LocalState {
        LocalTransport::new_state()
    }

    /// Creates a new instance of the engine.
    #[must_use]
    pub fn new_local_game(state: Arc<Mutex<LocalState>>) -> Self {
        Self { transport: LocalTransport::new(state) }
    }
}

impl<T: Transport> Engine<T> {
    /// Applies a player action to the game state.
    pub async fn apply_step(
        &mut self,
        action: PlayerAction,
    ) -> Result<StepResult, TransportError> {
        self.transport.apply_step(action).await
    }

    /// Returns the current state of the game.
    pub fn state(&self) -> State {
        self.transport.state()
    }
}
