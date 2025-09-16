use crate::{
    GameState,
    events::GameEvent,
    items::{ItemId, SlotId},
    step_result::StepContext,
};

/// Moves the player in the specified direction.
pub(crate) fn player_equip_item(
    state: &mut GameState,
    step_context: &mut StepContext,
    item_id: ItemId,
    slot: SlotId,
) {
    if state.hotbar.contains(slot) {
        return;
    }

    let Some(stack) = state.inventory.take(item_id) else {
        return;
    };

    state.hotbar.equip(stack, slot);

    step_context.add_event(GameEvent::PlayerEquippedItem { item_id, slot });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        actors::Actor, dungeon::DungeonMap, position::Position, rng::MyRng,
    };

    fn setup_state() -> GameState {
        GameState::new(
            Actor::create_player(Position::new(0, 0)),
            vec![],
            DungeonMap::simple(5, 5),
            MyRng::new(),
        )
    }

    #[test]
    fn equipping_item_moves_it_from_inventory() {
        let mut gs = setup_state();
        let events = player_equip_item(&mut gs, &mut StepContext::new(), 0, 0);
        assert!(matches!(
            events.as_slice(),
            [GameEvent::PlayerEquippedItem { item_id: 0, slot: 0 }]
        ));
        assert!(gs.hotbar.contains(0));
        assert!(
            gs.inventory
                .iter()
                .all(|slot| slot.as_ref().map_or(true, |s| s.id() != 0))
        );
    }

    #[test]
    fn cannot_equip_if_slot_taken_or_item_missing() {
        let mut gs = setup_state();
        // First equip item 0 into slot 0
        player_equip_item(&mut gs, 0, 0);
        // Try to equip another item into same slot
        let events = player_equip_item(&mut gs, 1, 0);
        assert!(events.is_empty());
        // Try to equip non-existent item
        let events = player_equip_item(&mut gs, 999, 1);
        assert!(events.is_empty());
    }
}
