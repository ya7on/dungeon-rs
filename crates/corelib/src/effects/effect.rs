use super::kind::EffectKind;

/// Represents a definition of an effect.
#[derive(Debug)]
pub(crate) struct EffectDef {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) kind: EffectKind,
}
