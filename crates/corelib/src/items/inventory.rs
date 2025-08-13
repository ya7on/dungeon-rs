use super::{ItemId, item_stack::ItemStack};

const DEFAULT_INVENTORY_SIZE: usize = 36;

/// Represents a player's inventory.
#[derive(Debug)]
pub(crate) struct Inventory {
    slots: Vec<Option<ItemStack>>,
}

impl Inventory {
    /// Creates an empty inventory.
    pub(crate) fn empty() -> Self {
        Self { slots: Vec::with_capacity(DEFAULT_INVENTORY_SIZE) }
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
        self.slots
            .iter_mut()
            .find_map(|slot| slot.take().filter(|item| item.item_id == item_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
