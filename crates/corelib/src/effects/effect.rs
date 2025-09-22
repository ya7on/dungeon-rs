use super::kind::EffectKind;

/// Represents a definition of an effect.
#[derive(Debug)]
pub(crate) struct EffectDef {
    #[allow(dead_code)]
    pub(crate) name: String,
    #[allow(dead_code)]
    pub(crate) description: String,
    pub(crate) kind: EffectKind,
}
