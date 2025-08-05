use corelib::{Direction, PlayerAction, new_game};
use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::tui::TuiApplication;

/// Run TUI with game
pub(crate) fn play() {
    let mut game = new_game();
    let mut tui = TuiApplication::default();

    loop {
        tui.draw_frame(&game);

        // TODO: Handle user input
        if let Event::Key(key) = event::read().expect("failed to read event") {
            match key.code {
                KeyCode::Char('q') => {
                    ratatui::restore();
                    break;
                }
                KeyCode::Char(' ') => {
                    game.apply_player_action(PlayerAction::Skip);
                }
                KeyCode::Char('w') => {
                    game.apply_player_action(PlayerAction::Move(Direction::North));
                }
                KeyCode::Char('s') => {
                    game.apply_player_action(PlayerAction::Move(Direction::South));
                }
                KeyCode::Char('a') => {
                    game.apply_player_action(PlayerAction::Move(Direction::West));
                }
                KeyCode::Char('d') => {
                    game.apply_player_action(PlayerAction::Move(Direction::East));
                }
                KeyCode::Char('W') => {
                    game.apply_player_action(PlayerAction::Attack(Direction::North));
                }
                KeyCode::Char('S') => {
                    game.apply_player_action(PlayerAction::Attack(Direction::South));
                }
                KeyCode::Char('A') => {
                    game.apply_player_action(PlayerAction::Attack(Direction::West));
                }
                KeyCode::Char('D') => {
                    game.apply_player_action(PlayerAction::Attack(Direction::East));
                }
                _ => {}
            }
        }
    }
}
