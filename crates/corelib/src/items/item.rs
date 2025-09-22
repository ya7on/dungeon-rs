use super::item_kind::ItemKind;

/// Item definition
#[derive(Debug)]
pub struct ItemDef {
    pub(crate) name: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) kind: ItemKind,
    #[allow(dead_code)]
    pub(crate) stackable: bool,
}

impl ItemDef {
    /// Get the name of the item
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the title of the item
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get the description of the item
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}
