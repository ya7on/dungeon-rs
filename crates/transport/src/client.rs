use async_trait::async_trait;
use protocol::{PlayerAction, State, StepResult};

use crate::TransportResult;

/// A trait for transporting data.
#[async_trait]
pub trait Transport {
    /// Apply a game step.
    async fn apply_step(
        &mut self,
        action: PlayerAction,
    ) -> TransportResult<StepResult>;

    /// Get the current game state.
    fn state(&self) -> State;
}
