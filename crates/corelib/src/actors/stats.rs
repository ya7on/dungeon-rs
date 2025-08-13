use super::ActorKind;

/// Represents the stats of an actor.
#[derive(Debug)]
pub struct Stats {
    /// The current health of the actor.
    pub(crate) hp: u32,
    /// Minimum attack power of the actor.
    pub(crate) min_damage: u32,
    /// Maximum attack power of the actor.
    pub(crate) max_damage: u32,
    /// Defense power of the actor.
    pub(crate) defense: u32,
}

impl Stats {
    /// Creates a new `Stats` instance with the given values.
    pub(crate) fn new(
        hp: u32,
        min_damage: u32,
        max_damage: u32,
        defense: u32,
    ) -> Self {
        Stats { hp, min_damage, max_damage, defense }
    }

    /// Returns the current health of the actor.
    #[must_use]
    pub fn hp(&self) -> u32 {
        self.hp
    }

    /// Returns the minimum attack power of the actor
    #[must_use]
    pub fn min_damage(&self) -> u32 {
        self.min_damage
    }

    /// Returns the maximum attack power of the actor
    #[must_use]
    pub fn max_damage(&self) -> u32 {
        self.max_damage
    }

    /// Returns the defense power of the actor.
    #[must_use]
    pub fn defense(&self) -> u32 {
        self.defense
    }
}

impl ActorKind {
    /// Default stats table for each actor kind.
    pub(crate) fn default_stats(&self) -> Stats {
        match self {
            ActorKind::Player => Stats::new(30, 5, 10, 2),
            ActorKind::Enemy => Stats::new(20, 3, 5, 1),
        }
    }
}
