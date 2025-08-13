/// Represents the kind of an item.
#[derive(Debug)]
pub(crate) enum ItemKind {
    /// Represents a weapon item.
    Weapon { min_damage: u32, max_damage: u32 },
    /// Represents a shield item.
    Armor { defense: u32 },
}
