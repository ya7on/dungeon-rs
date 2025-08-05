/// Represents the kind of actor.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ActorKind {
    /// Represents a player character.
    Player,
    /// Represents an enemy character.
    Enemy,
}
