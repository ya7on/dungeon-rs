use corelib::GameState;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Row, StatefulWidget, Table, TableState},
};

pub struct HotbarWidget<'a> {
    state: &'a GameState,
}

impl<'a> HotbarWidget<'a> {
    pub fn new(state: &'a GameState) -> Self {
        Self { state }
    }
}

impl StatefulWidget for HotbarWidget<'_> {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut rows = vec![];

        for (index, item) in self.state.hotbar().iter().enumerate() {
            if let Some(item) = item {
                let item_name = self
                    .state
                    .items_catalog()
                    .get(item.id())
                    .map(|item| item.title())
                    .unwrap_or("Unknown item")
                    .to_string();
                rows.push(Row::new(vec![
                    format!("Slot {}", index + 1),
                    item_name,
                ]));
            } else {
                rows.push(Row::new(vec![
                    format!("Slot {}", index),
                    "Empty".to_string(),
                ]));
            }
        }

        let widths = [Constraint::Percentage(20), Constraint::Percentage(90)];
        let table = Table::new(rows, widths)
            .column_spacing(1)
            .header(
                Row::new(vec!["Slot", "Item"])
                    .style(Style::new().bold())
                    .bottom_margin(1),
            )
            .block(Block::default().title("Hotbar").borders(Borders::ALL))
            .row_highlight_style(Style::new().reversed())
            .highlight_symbol(">>");

        StatefulWidget::render(table, area, buf, state);
    }
}
