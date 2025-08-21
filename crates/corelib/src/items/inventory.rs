use super::{ItemId, item_stack::ItemStack};

const DEFAULT_INVENTORY_SIZE: usize = 36;

/// Represents a player's inventory.
#[derive(Debug)]
pub struct Inventory {
    slots: Vec<Option<ItemStack>>,
}

impl Inventory {
    /// Creates an empty inventory.
    pub(crate) fn empty() -> Self {
        // TODO: Fixme
        let mut slots = Vec::with_capacity(DEFAULT_INVENTORY_SIZE);
        slots.push(Some(ItemStack { item_id: 0, count: 1 }));
        slots.push(Some(ItemStack { item_id: 1, count: 1 }));
        slots.push(Some(ItemStack { item_id: 2, count: 1 }));

        Self { slots }
    }

    pub(crate) fn add(&mut self, item: ItemStack) {
        for slot in &mut self.slots {
            if slot.is_none() {
                *slot = Some(item);
                return;
            }
        }
    }

    pub(crate) fn take(&mut self, item_id: ItemId) -> Option<ItemStack> {
        for item in &mut self.slots {
            if let Some(stack) = item {
                if stack.item_id == item_id {
                    return item.take();
                }
            }
        }
        None
    }

    pub fn iter(&self) -> impl Iterator<Item = &Option<ItemStack>> {
        self.slots.iter()
    }
}

impl IntoIterator for Inventory {
    type Item = Option<ItemStack>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.slots.into_iter()
    }
}
