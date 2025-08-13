use super::ItemId;

/// Represents a stack of items in the inventory.
#[derive(Debug)]
pub(crate) struct ItemStack {
    pub(crate) item_id: ItemId,
    pub(crate) count: u32,
}
