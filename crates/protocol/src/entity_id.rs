use serde::{Deserialize, Serialize};

/// Entity ID.
#[derive(Serialize, Deserialize)]
pub struct EntityId(pub u32);
