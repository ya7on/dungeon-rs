use crate::ActorKind;

/// Represents the stats of an actor.
#[derive(Debug)]
pub struct Stats {
    /// The current health of the actor.
    pub(crate) hp: u32,
    /// Attack power of the actor.
    pub(crate) attack: u32,
    /// Defense power of the actor.
    pub(crate) defense: u32,
}

impl Stats {
    /// Creates a new `Stats` instance with the given values.
    pub fn new(hp: u32, attack: u32, defense: u32) -> Self {
        Stats {
            hp,
            attack,
            defense,
        }
    }
}

impl ActorKind {
    /// Default stats table for each actor kind.
    pub fn default_stats(&self) -> Stats {
        match self {
            ActorKind::Player => Stats::new(30, 5, 2),
            ActorKind::Enemy => Stats::new(20, 3, 1),
            ActorKind::Npc => Stats::new(15, 2, 1),
        }
    }
}
