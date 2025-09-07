use serde::{Deserialize, Serialize};

use crate::Position;

/// Represents the state of a game.
#[derive(Serialize, Deserialize)]
pub struct State {
    /// The position of the player.
    pub player: Position,
}
