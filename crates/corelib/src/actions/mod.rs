mod player_attack;
mod player_equip_item;
mod player_move;
mod player_unequip_item;

pub(crate) use player_attack::player_attack;
pub(crate) use player_equip_item::player_equip_item;
pub(crate) use player_move::player_move;
pub(crate) use player_unequip_item::player_unequip_item;

use crate::{
    direction::Direction,
    items::{ItemId, SlotId},
};

/// Represents an action that a player can take.
#[derive(Debug)]
pub enum PlayerAction {
    /// Skip the current turn.
    Skip,
    /// Move in the specified direction.
    Move(Direction),
    /// Attack in the specified direction.
    Attack(Direction),
    /// Equip an item in the specified slot.
    EquipItem {
        /// The ID of the item to equip.
        item_id: ItemId,
        /// The slot to equip the item in.
        slot: SlotId,
    },
    /// Unequip an item from the specified slot.
    UnequipItem {
        /// The slot to unequip the item from.
        slot: SlotId,
    },
}
