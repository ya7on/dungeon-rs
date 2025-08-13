use crate::{
    GameState,
    events::GameEvent,
    items::{ItemId, SlotId},
};

/// Moves the player in the specified direction.
pub(crate) fn player_equip_item(
    state: &mut GameState,
    item_id: ItemId,
    slot: SlotId,
) -> Vec<GameEvent> {
    if state.hotbar.contains(slot) {
        return vec![];
    }

    let Some(stack) = state.inventory.take(item_id) else {
        return vec![];
    };

    state.hotbar.equip(stack, slot);

    vec![GameEvent::PlayerEquippedItem { item_id, slot }]
}
