pub(crate) type EffectId = usize; // TODO: Maybe use u32 for optimization?

mod effect;
mod kind;

pub(crate) use effect::EffectDef;
pub(crate) use kind::EffectKind;
