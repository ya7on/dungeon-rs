use crate::effects::EffectId;

/// Represents an instance of an effect in the game.
#[derive(Debug)]
pub(crate) struct EffectInstance {
    pub(crate) effect_id: EffectId,
    pub(crate) remaining_turns: u8,
}
