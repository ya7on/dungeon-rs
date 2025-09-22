use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use corelib::GameState;
use protocol::{PlayerAction, State, StepResult};

/// Local implementation of `GameState`.
pub type LocalState = GameState;

use crate::{
    Transport, TransportError, TransportResult,
    adapter::corelib::{FromCorelib, ToCorelib},
};

/// Local implementation of Transport.
pub struct LocalTransport {
    state: Arc<Mutex<LocalState>>,
}

impl LocalTransport {
    /// Build a new instance of local transport.
    #[must_use]
    pub fn new(state: Arc<Mutex<LocalState>>) -> Self {
        Self { state }
    }

    /// Create a new game state.
    #[must_use]
    pub fn new_state() -> corelib::GameState {
        corelib::new_game(&corelib::WorldSettings {
            seed: [0; 32],
            map_width: 1024,
            map_height: 1024,
            floor_tiles: 100,
            enemies: 10,
        })
    }
}

#[async_trait]
impl Transport for LocalTransport {
    async fn apply_step(
        &mut self,
        action: PlayerAction,
    ) -> TransportResult<StepResult> {
        let mut guard =
            self.state.lock().map_err(|_| TransportError::LockError)?;
        let result = guard.apply_player_action(&action.to_corelib());
        Ok(StepResult {
            events: result
                .events
                .into_iter()
                .map(protocol::GameEvent::from_corelib)
                .collect(),
            diff: protocol::StateDiff::from_corelib(result.diff),
        })
    }

    /// Get the current game state.
    fn state(&self) -> State {
        let Ok(guard) = self.state.lock() else {
            // Mutex was poisoned, which indicates a panic in another thread
            panic!("Failed to acquire lock on game state: mutex was poisoned")
        };
        State {
            player: protocol::Position::from_corelib(guard.player().position()),
        }
    }
}
