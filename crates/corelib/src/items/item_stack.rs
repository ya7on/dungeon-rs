use super::ItemId;

/// Represents a stack of items in the inventory.
#[derive(Debug)]
pub struct ItemStack {
    pub(crate) item_id: ItemId,
    pub(crate) count: u32,
}

impl ItemStack {
    pub fn id(&self) -> ItemId {
        self.item_id
    }
}
