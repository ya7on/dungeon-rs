use crate::{
    GameState, events::GameEvent, items::SlotId, step_result::StepContext,
};

/// Moves the player in the specified direction.
pub(crate) fn player_unequip_item(
    state: &mut GameState,
    step_context: &mut StepContext,
    slot: SlotId,
) {
    if !state.hotbar.contains(slot) {
        return;
    }

    let Some(stack) = state.hotbar.take(slot) else {
        return;
    };

    state.inventory.add(stack);

    step_context.add_event(GameEvent::PlayerUnequippedItem { slot });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        actions::player_equip_item, actors::Actor, dungeon::DungeonMap,
        position::Position, rng::MyRng,
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
    fn unequip_moves_item_back_to_inventory() {
        let mut gs = setup_state();
        let mut step_context1 = StepContext::default();
        player_equip_item(&mut gs, &mut step_context1, 0, 0);
        let mut step_context2 = StepContext::default();
        player_unequip_item(&mut gs, &mut step_context2, 0);
        let result = step_context2.build();
        assert!(matches!(
            result.events.as_slices().0,
            [GameEvent::PlayerUnequippedItem { slot: 0 }]
        ));
        assert!(!gs.hotbar.contains(0));
        assert!(
            gs.inventory
                .iter()
                .any(|slot| slot.as_ref().map_or(false, |s| s.id() == 0))
        );
    }

    #[test]
    fn cannot_unequip_empty_slot() {
        let mut gs = setup_state();
        let mut step_context = StepContext::default();
        player_unequip_item(&mut gs, &mut step_context, 0);
        let result = step_context.build();
        assert!(result.events.is_empty());
    }
}
