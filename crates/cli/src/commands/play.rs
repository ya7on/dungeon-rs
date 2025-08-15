use corelib::{Direction, PlayerAction, WorldSettings, new_game};
use ratatui::crossterm::event::{self, Event, KeyCode};
use sha2::Digest;

use crate::tui::{InventoryFocus, Mode, TuiApplication};

fn seed_from_u64(seed: u64) -> [u8; 32] {
    let mut hasher = sha2::Sha256::new();
    hasher.update(seed.to_be_bytes());
    hasher.finalize().into()
}

/// Run TUI with game
pub(crate) fn play(
    seed: u64,
    map_width: usize,
    map_height: usize,
    floor_tiles: usize,
    enemies: usize,
) {
    let mut game = new_game(&WorldSettings {
        seed: seed_from_u64(seed),
        map_width,
        map_height,
        floor_tiles,
        enemies,
    });
    let mut tui = TuiApplication::default();

    loop {
        tui.draw_frame(&game);

        // TODO: Handle user input
        if let Event::Key(key) = event::read().expect("failed to read event") {
            match key.code {
                KeyCode::Char('q') => {
                    ratatui::restore();
                    break;
                },
                KeyCode::Char(' ') => {
                    if tui.mode() == &Mode::Game {
                        tui.set_event_log(
                            game.apply_player_action(&PlayerAction::Skip),
                        );
                    } else {
                        match tui.inventory_focus() {
                            InventoryFocus::Hotbar => {
                                let state = tui.hotbar_state();
                                let Some(selected) = state.selected() else {
                                    continue;
                                };
                                tui.set_event_log(game.apply_player_action(
                                    &PlayerAction::UnequipItem {
                                        slot: selected,
                                    },
                                ));
                            },
                            InventoryFocus::Inventory => {
                                let state = tui.inventory_state();
                                let Some(selected) = state.selected() else {
                                    continue;
                                };
                                let inventory_items = game
                                    .inventory()
                                    .iter()
                                    .flatten()
                                    .collect::<Vec<_>>();
                                let item = inventory_items[selected];
                                let slot = game.hotbar().empty_slot().unwrap();
                                tui.set_event_log(game.apply_player_action(
                                    &PlayerAction::EquipItem {
                                        slot,
                                        item_id: item.id(),
                                    },
                                ));
                            },
                        }
                    }
                },
                KeyCode::Char('i') => {
                    tui.toggle_inventory();
                },
                KeyCode::Char('w') => {
                    if tui.mode() == &Mode::Game {
                        tui.set_event_log(game.apply_player_action(
                            &PlayerAction::Move(Direction::North),
                        ));
                    } else {
                        tui.select_previous();
                    }
                },
                KeyCode::Char('s') => {
                    if tui.mode() == &Mode::Game {
                        tui.set_event_log(game.apply_player_action(
                            &PlayerAction::Move(Direction::South),
                        ));
                    } else {
                        tui.select_next();
                    }
                },
                KeyCode::Char('a') => {
                    if tui.mode() == &Mode::Game {
                        tui.set_event_log(game.apply_player_action(
                            &PlayerAction::Move(Direction::West),
                        ));
                    } else {
                        tui.toggle_inventory_focus();
                    }
                },
                KeyCode::Char('d') => {
                    if tui.mode() == &Mode::Game {
                        tui.set_event_log(game.apply_player_action(
                            &PlayerAction::Move(Direction::East),
                        ));
                    } else {
                        tui.toggle_inventory_focus();
                    }
                },
                KeyCode::Char('W') => {
                    tui.set_event_log(game.apply_player_action(
                        &PlayerAction::Attack(Direction::North),
                    ));
                },
                KeyCode::Char('S') => {
                    tui.set_event_log(game.apply_player_action(
                        &PlayerAction::Attack(Direction::South),
                    ));
                },
                KeyCode::Char('A') => {
                    tui.set_event_log(game.apply_player_action(
                        &PlayerAction::Attack(Direction::West),
                    ));
                },
                KeyCode::Char('D') => {
                    tui.set_event_log(game.apply_player_action(
                        &PlayerAction::Attack(Direction::East),
                    ));
                },
                _ => {},
            }
        }
    }
}
