use std::io::Stdout;

use corelib::GameState;
use ratatui::{
    Terminal,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    prelude::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
};
use widgets::{stats::StatsWidget, viewport::ViewportWidget};

pub mod widgets;

pub struct TuiApplication {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Default for TuiApplication {
    fn default() -> Self {
        Self {
            terminal: ratatui::init(),
        }
    }
}

impl TuiApplication {
    pub fn draw_frame(&mut self, state: &GameState) {
        let viewport = ViewportWidget::from(state);
        let stats = StatsWidget::from(state);

        self.terminal
            .draw(|frame| {
                let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]);
                let [title_area, main_area] = vertical.areas(frame.area());
                let horizontal =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
                let [left_area, right_area] = horizontal.areas(main_area);

                let title =
                    Paragraph::new("dungeon-rs").block(Block::default().borders(Borders::ALL));

                frame.render_widget(title, title_area);
                frame.render_widget(viewport, left_area);
                frame.render_widget(stats, right_area);
            })
            .unwrap(); // TODO: Implement error handling
    }
}
