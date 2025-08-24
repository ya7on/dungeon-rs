use super::{MAX_HOTBAR_SIZE, SlotId, item_stack::ItemStack};

/// Represents a hotbar in the game.
#[derive(Debug)]
pub struct Hotbar {
    items: Box<[Option<ItemStack>; MAX_HOTBAR_SIZE]>,
}

impl Hotbar {
    pub(crate) fn empty() -> Self {
        Self { items: Box::new(std::array::from_fn(|_| None)) }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Option<ItemStack>> {
        self.items.iter()
    }

    pub fn empty_slot(&self) -> Option<SlotId> {
        self.items.iter().position(Option::is_none).map(|pos| pos as SlotId)
    }

    pub(crate) fn take(&mut self, slot: SlotId) -> Option<ItemStack> {
        if slot < MAX_HOTBAR_SIZE { self.items[slot].take() } else { None }
    }

    pub(crate) fn contains(&self, slot: SlotId) -> bool {
        if slot < MAX_HOTBAR_SIZE { self.items[slot].is_some() } else { false }
    }

    pub(crate) fn equip(&mut self, stack: ItemStack, slot: SlotId) {
        if slot < MAX_HOTBAR_SIZE {
            self.items[slot] = Some(stack);
        }
    }
}

impl IntoIterator for Hotbar {
    type Item = Option<ItemStack>;
    type IntoIter = std::array::IntoIter<Self::Item, MAX_HOTBAR_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
