use crate::{GameState, events::GameEvent, items::SlotId};

/// Moves the player in the specified direction.
pub(crate) fn player_unequip_item(
    state: &mut GameState,
    slot: SlotId,
) -> Vec<GameEvent> {
    if !state.hotbar.contains(slot) {
        return vec![];
    }

    let Some(stack) = state.hotbar.take(slot) else {
        return vec![];
    };

    state.inventory.add(stack);

    vec![GameEvent::PlayerUnequippedItem { slot }]
}
