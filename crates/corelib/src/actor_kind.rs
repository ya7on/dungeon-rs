/// Represents the kind of actor.
#[derive(Debug, PartialEq, Eq)]
pub enum ActorKind {
    /// Represents a player character.
    Player,
    /// Represents an enemy character.
    Enemy,
    /// Represents a non-player character.
    Npc,
}
