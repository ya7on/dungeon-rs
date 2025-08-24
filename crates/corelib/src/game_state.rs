use std::collections::VecDeque;

use crate::{
    Stats,
    actions::{
        PlayerAction, player_attack, player_equip_item, player_move,
        player_unequip_item,
    },
    actors::Actor,
    ai::simple_ai,
    catalog::{EffectsCatalog, ItemsCatalog},
    dungeon::DungeonMap,
    effects::EffectKind,
    events::GameEvent,
    items::{Hotbar, Inventory, ItemKind},
    rng::MyRng,
    walk_map::WalkMap,
};

/// Represents the result of a game step.
pub struct StepResult {
    /// The events that occurred during the step.
    pub events: VecDeque<GameEvent>,
}

/// Represents the state of the game.
#[derive(Debug)]
pub struct GameState {
    /// The ID of the current tick.
    pub(crate) tick_id: u64,
    /// The player
    pub(crate) player: Actor,
    /// The player's inventory.
    pub(crate) inventory: Inventory,
    /// The player's hotbar.
    pub(crate) hotbar: Hotbar,
    /// Other entities in the game.
    pub(crate) entities: Vec<Actor>,
    /// The dungeon map.
    pub(crate) dungeon: DungeonMap,
    /// The random number generator.
    pub(crate) rng: MyRng,
    /// Global items catalog.
    pub(crate) items_catalog: ItemsCatalog,
    /// Global effects catalog.
    pub(crate) effects_catalog: EffectsCatalog,
}

impl GameState {
    /// Creates a new game state with the given player and entities.
    pub(crate) fn new(
        player: Actor,
        entities: Vec<Actor>,
        map: DungeonMap,
        rng: MyRng,
    ) -> Self {
        GameState {
            tick_id: 0,
            player,
            entities,
            dungeon: map,
            rng,
            hotbar: Hotbar::empty(),
            inventory: Inventory::empty(),
            items_catalog: ItemsCatalog::new(),
            effects_catalog: EffectsCatalog::new(),
        }
    }

    /// Applies the given player action to the game state.
    pub fn apply_player_action(&mut self, action: &PlayerAction) -> StepResult {
        let mut walk_map = self.recalculate_walk_map();
        // TODO: add "dirty" flag, recalculate only after player action
        self.player.stats = self.calculate_hotbar_stats();

        let mut events = VecDeque::new();

        events.extend(Self::tick_effects(
            &self.effects_catalog,
            &mut self.player,
        ));

        match action {
            PlayerAction::Skip => {
                events.push_back(GameEvent::PlayerSkippedMove);
            },
            PlayerAction::Move(direction) => {
                events.extend(player_move(self, *direction, &mut walk_map));
            },
            PlayerAction::Attack(direction) => {
                events.extend(player_attack(self, *direction));
            },
            PlayerAction::EquipItem { item_id, slot } => {
                events.extend(player_equip_item(self, *item_id, *slot));
            },
            PlayerAction::UnequipItem { slot } => {
                events.extend(player_unequip_item(self, *slot));
            },
        }

        events.extend(simple_ai(self, &mut walk_map));

        self.tick_id += 1;

        StepResult { events }
    }

    /// Recalculates the walk map based on the current dungeon and entities.
    #[must_use]
    fn recalculate_walk_map(&self) -> WalkMap {
        let mut walk_map = self
            .dungeon
            .iter()
            .filter_map(|(position, tile)| {
                (tile.is_walkable()).then_some(position)
            })
            .collect::<WalkMap>();

        for entity in &self.entities {
            walk_map.occupy(entity.position);
        }
        walk_map.occupy(self.player.position);

        walk_map
    }

    /// Calculates the stats for the player by iterating over the hotbar items and calculating their stats.
    #[must_use]
    fn calculate_hotbar_stats(&self) -> Stats {
        let mut stats = self.player.kind.default_stats();
        stats.hp = self.player.stats.hp;

        for stack in self.hotbar.iter().flatten() {
            let Some(item) = self.items_catalog.get(stack.item_id) else {
                continue;
            };
            match item.kind {
                ItemKind::Weapon { min_damage, max_damage } => {
                    stats.min_damage += min_damage;
                    stats.max_damage += max_damage;
                },
                ItemKind::Armor { defense } => {
                    stats.defense += defense;
                },
            }
        }

        stats
    }

    /// Ticks the effects of an entity.
    fn tick_effects(
        effects_catalog: &EffectsCatalog,
        entity: &mut Actor,
    ) -> VecDeque<GameEvent> {
        let mut events = VecDeque::new();
        entity.effects.retain_mut(|effect| {
            if effect.remaining_turns == 0 {
                events.push_back(GameEvent::EffectExpired {
                    entity_id: entity.id,
                    effect_id: effect.effect_id,
                });
                return false;
            }
            effect.remaining_turns = effect.remaining_turns.saturating_sub(1);

            let Some(effect_def) = effects_catalog.get(effect.effect_id) else {
                // Unknown effect, remove it
                events.push_back(GameEvent::EffectExpired {
                    entity_id: entity.id,
                    effect_id: effect.effect_id,
                });
                return false;
            };
            match effect_def.kind {
                EffectKind::Heal { hp_per_turn } => {
                    entity.stats.hp += hp_per_turn;
                },
            }
            events.push_back(GameEvent::EffectTick {
                entity_id: entity.id,
                effect_id: effect.effect_id,
            });

            true
        });
        events
    }

    /// Returns a reference to the player.
    #[must_use]
    pub fn player(&self) -> &Actor {
        &self.player
    }

    /// Returns a reference to the entities.
    #[must_use]
    pub fn entities(&self) -> &[Actor] {
        &self.entities
    }

    /// Returns a reference to the dungeon.
    #[must_use]
    pub fn dungeon(&self) -> &DungeonMap {
        &self.dungeon
    }

    /// Returns a reference to the items catalog.
    #[must_use]
    pub fn items_catalog(&self) -> &ItemsCatalog {
        &self.items_catalog
    }

    /// Returns a reference to the hotbar.
    #[must_use]
    pub fn hotbar(&self) -> &Hotbar {
        &self.hotbar
    }

    /// Returns a reference to the inventory.
    #[must_use]
    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }
}

#[cfg(test)]
mod tests {

    use crate::{position::Position, ActorKind, Direction};

    use super::*;

    #[test]
    fn test_tick_increment() {
        let mut gs = GameState::new(
            Actor::create_player(Position::new(0, 0)),
            vec![],
            DungeonMap::simple(10, 10),
            MyRng::new(),
        );

        gs.apply_player_action(&PlayerAction::Skip);

        assert_eq!(gs.tick_id, 1);
    }

    #[test]
    fn player_event_before_npc_event() {
        let mut gs = GameState::new(
            Actor::create_player(Position::new(0, 0)),
            vec![Actor::create(Position::new(3, 0), ActorKind::Enemy)],
            DungeonMap::simple(10, 10),
            MyRng::new(),
        );
        let result = gs.apply_player_action(&PlayerAction::Move(Direction::East));
        let events: Vec<_> = result.events.into_iter().collect();
        assert!(matches!(events[0], GameEvent::PlayerMoved { .. }));
        assert!(matches!(events[1], GameEvent::EntityMoved { .. }));
    }

    #[test]
    fn determinism_same_seed_and_actions() {
        let seed = [4; 32];
        let mut gs1 = GameState::new(
            Actor::create_player(Position::new(0, 0)),
            vec![Actor::create(Position::new(1, 0), ActorKind::Enemy)],
            DungeonMap::simple(5, 5),
            MyRng::from_seed(seed),
        );
        let mut gs2 = GameState::new(
            Actor::create_player(Position::new(0, 0)),
            vec![Actor::create(Position::new(1, 0), ActorKind::Enemy)],
            DungeonMap::simple(5, 5),
            MyRng::from_seed(seed),
        );
        for _ in 0..2 {
            let r1 = gs1.apply_player_action(&PlayerAction::Skip);
            let r2 = gs2.apply_player_action(&PlayerAction::Skip);
            let e1: Vec<_> = r1.events.into_iter().collect();
            let e2: Vec<_> = r2.events.into_iter().collect();
            assert_eq!(e1.len(), e2.len());
            for (a, b) in e1.iter().zip(e2.iter()) {
                assert_eq!(std::mem::discriminant(a), std::mem::discriminant(b));
                match (a, b) {
                    (
                        GameEvent::EntityAttacked { target: t1, damage: d1, .. },
                        GameEvent::EntityAttacked { target: t2, damage: d2, .. },
                    ) => {
                        assert_eq!(t1, t2);
                        assert_eq!(d1, d2);
                    },
                    _ => {},
                }
            }
            assert_eq!(gs1.player.stats.hp, gs2.player.stats.hp);
        }
    }
}
