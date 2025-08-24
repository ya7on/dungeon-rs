use async_trait::async_trait;
use corelib::GameState;
use protocol::{PlayerAction, StepResult};

use crate::{
    Transport, TransportResult,
    adapter::corelib::{FromCorelib, ToCorelib},
};

/// Local implementation of Transport.
pub struct LocalTransport<'a> {
    state: &'a mut GameState,
}

impl<'a> LocalTransport<'a> {
    /// Build a new instance of local transport.
    pub fn new(state: &'a mut GameState) -> Self {
        Self { state }
    }
}

#[async_trait]
impl Transport for LocalTransport<'_> {
    async fn apply_step(
        &mut self,
        action: PlayerAction,
    ) -> TransportResult<StepResult> {
        let result = self.state.apply_player_action(&action.to_corelib());
        Ok(StepResult {
            events: result
                .events
                .into_iter()
                .map(protocol::GameEvent::from_corelib)
                .collect(),
        })
    }
}
