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
        let mut step_context = StepContext::default();
        player_equip_item(&mut gs, &mut step_context, 0, 0);
        let result = step_context.build();
        assert!(matches!(
            result.events.as_slices().0,
            [GameEvent::PlayerEquippedItem { item_id: 0, slot: 0 }]
        ));
        assert!(gs.hotbar.contains(0));
        assert!(
            gs.inventory
                .iter()
                .all(|slot| slot.as_ref().is_none_or(|s| s.id() != 0))
        );
    }

    #[test]
    fn cannot_equip_if_slot_taken_or_item_missing() {
        let mut gs = setup_state();
        // First equip item 0 into slot 0
        let mut step_context = StepContext::default();
        player_equip_item(&mut gs, &mut step_context, 0, 0);

        // Try to equip another item into same slot
        let mut step_context2 = StepContext::default();
        player_equip_item(&mut gs, &mut step_context2, 1, 0);
        let result2 = step_context2.build();
        assert!(result2.events.is_empty());

        // Try to equip non-existent item
        let mut step_context3 = StepContext::default();
        player_equip_item(&mut gs, &mut step_context3, 999, 1);
        let result3 = step_context3.build();
        assert!(result3.events.is_empty());
    }
}
