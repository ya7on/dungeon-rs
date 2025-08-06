use corelib::{Direction, PlayerAction, WorldSettings, new_game};
use ratatui::crossterm::event::{self, Event, KeyCode};
use sha2::Digest;

use crate::tui::TuiApplication;

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
                    game.apply_player_action(PlayerAction::Skip);
                },
                KeyCode::Char('w') => {
                    game.apply_player_action(PlayerAction::Move(
                        Direction::North,
                    ));
                },
                KeyCode::Char('s') => {
                    game.apply_player_action(PlayerAction::Move(
                        Direction::South,
                    ));
                },
                KeyCode::Char('a') => {
                    game.apply_player_action(PlayerAction::Move(
                        Direction::West,
                    ));
                },
                KeyCode::Char('d') => {
                    game.apply_player_action(PlayerAction::Move(
                        Direction::East,
                    ));
                },
                KeyCode::Char('W') => {
                    game.apply_player_action(PlayerAction::Attack(
                        Direction::North,
                    ));
                },
                KeyCode::Char('S') => {
                    game.apply_player_action(PlayerAction::Attack(
                        Direction::South,
                    ));
                },
                KeyCode::Char('A') => {
                    game.apply_player_action(PlayerAction::Attack(
                        Direction::West,
                    ));
                },
                KeyCode::Char('D') => {
                    game.apply_player_action(PlayerAction::Attack(
                        Direction::East,
                    ));
                },
                _ => {},
            }
        }
    }
}
