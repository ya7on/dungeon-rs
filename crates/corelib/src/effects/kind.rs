/// Represents the kind of an effect.
#[derive(Debug)]
pub(crate) enum EffectKind {
    /// Represents a healing effect.
    Heal {
        /// The amount of health to restore every turn.
        hp_per_turn: u32,
    },
}
