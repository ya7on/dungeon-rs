pub(crate) type ItemId = usize; // TODO: Maybe use u32 for optimization?
pub(crate) type SlotId = usize; // TODO: Maybe use u8 for optimization?

const MAX_HOTBAR_SIZE: usize = 10;

mod hotbar;
mod inventory;
mod item;
mod item_kind;
mod item_stack;

pub(crate) use hotbar::Hotbar;
pub(crate) use inventory::Inventory;
pub(crate) use item::ItemDef;
pub(crate) use item_kind::ItemKind;
