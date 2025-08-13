use super::item_kind::ItemKind;

/// Item definition
#[derive(Debug)]
pub(crate) struct ItemDef {
    pub(crate) name: String,
    pub(crate) kind: ItemKind,
    pub(crate) stackable: bool,
}
