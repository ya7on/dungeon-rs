use std::{collections::VecDeque, io::Stdout};

use corelib::{GameEvent, GameState};
use ratatui::{
    Terminal,
    layout::{Constraint, Layout},
    prelude::CrosstermBackend,
    widgets::{Block, Borders, Paragraph, TableState},
};
use widgets::{
    event_log::EventLogWidget, hotbar::HotbarWidget,
    inventory::InventoryWidget, viewport::ViewportWidget,
};

pub mod widgets;

#[derive(PartialEq)]
pub enum Mode {
    Inventory,
    Game,
}

#[derive(PartialEq)]
pub enum InventoryFocus {
    Inventory,
    Hotbar,
}

pub struct TuiApplication {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    event_log: VecDeque<GameEvent>,
    mode: Mode,
    inventory_focus: InventoryFocus,
    hotbar_state: TableState,
    inventory_state: TableState,
}

impl TuiApplication {
    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn inventory_focus(&self) -> &InventoryFocus {
        &self.inventory_focus
    }

    pub fn inventory_state(&self) -> &TableState {
        &self.inventory_state
    }

    pub fn hotbar_state(&self) -> &TableState {
        &self.hotbar_state
    }

    pub fn toggle_inventory(&mut self) {
        match self.mode {
            Mode::Inventory => self.mode = Mode::Game,
            Mode::Game => self.mode = Mode::Inventory,
        }
    }

    pub fn select_next(&mut self) {
        match self.inventory_focus {
            InventoryFocus::Inventory => self.inventory_state.select_next(),
            InventoryFocus::Hotbar => self.hotbar_state.select_next(),
        }
    }

    pub fn select_previous(&mut self) {
        match self.inventory_focus {
            InventoryFocus::Inventory => self.inventory_state.select_previous(),
            InventoryFocus::Hotbar => self.hotbar_state.select_previous(),
        }
    }

    pub fn toggle_inventory_focus(&mut self) {
        match self.inventory_focus {
            InventoryFocus::Inventory => {
                self.inventory_state.select(None);
                self.hotbar_state.select(Some(0));
                self.inventory_focus = InventoryFocus::Hotbar
            },
            InventoryFocus::Hotbar => {
                self.hotbar_state.select(None);
                self.inventory_state.select(Some(0));
                self.inventory_focus = InventoryFocus::Inventory
            },
        }
    }

    pub fn set_event_log(&mut self, events: VecDeque<GameEvent>) {
        self.event_log = events;
    }
}

impl Default for TuiApplication {
    fn default() -> Self {
        Self {
            terminal: ratatui::init(),
            mode: Mode::Game,
            inventory_focus: InventoryFocus::Inventory,
            hotbar_state: TableState::default(),
            inventory_state: TableState::default(),
            event_log: VecDeque::with_capacity(100),
        }
    }
}

impl TuiApplication {
    pub fn draw_frame(&mut self, state: &GameState) {
        let viewport = ViewportWidget::new(state);
        // let stats = StatsWidget::from(state);
        let inventory = InventoryWidget::new(state);
        let hotbar = HotbarWidget::new(state);
        let event_log = EventLogWidget::new(&self.event_log);

        self.terminal
            .draw(|frame| {
                let vertical = Layout::vertical([
                    Constraint::Length(3),
                    Constraint::Min(0),
                ]);
                let [title_area, main_area] = vertical.areas(frame.area());
                let horizontal = Layout::horizontal([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]);
                let [left_area, right_area] = horizontal.areas(main_area);

                let title = Paragraph::new("dungeon-rs")
                    .block(Block::default().borders(Borders::ALL));

                frame.render_widget(title, title_area);
                if self.mode == Mode::Inventory {
                    frame.render_stateful_widget(
                        inventory,
                        left_area,
                        &mut self.inventory_state,
                    );
                } else {
                    frame.render_widget(viewport, left_area);
                }
                if self.mode == Mode::Inventory {
                    frame.render_stateful_widget(
                        hotbar,
                        right_area,
                        &mut self.hotbar_state,
                    );
                } else {
                    frame.render_widget(event_log, right_area);
                }
            })
            .unwrap(); // TODO: Implement error handling
    }
}
