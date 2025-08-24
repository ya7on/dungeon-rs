use corelib::GameState;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    widgets::{
        Block, Borders, Paragraph, Row, StatefulWidget, Table, TableState,
        Widget,
    },
};

pub struct InventoryWidget<'a> {
    state: &'a GameState,
}

impl<'a> InventoryWidget<'a> {
    pub fn new(state: &'a GameState) -> Self {
        Self { state }
    }
}

impl StatefulWidget for InventoryWidget<'_> {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut rows = vec![];
        for item in self.state.inventory().iter().flatten() {
            let item_name = self
                .state
                .items_catalog()
                .get(item.id())
                .map(|item| item.title())
                .unwrap_or("Unknown item")
                .to_string();
            let item_description = self
                .state
                .items_catalog()
                .get(item.id())
                .map(|item| item.description())
                .unwrap_or("No description available")
                .to_string();
            rows.push(Row::new(vec![
                format!("{}", item.id()),
                item_name,
                item_description,
            ]));
        }

        if rows.is_empty() {
            let paragraph = Paragraph::new("Inventory is empty").block(
                Block::default().title("Inventory").borders(Borders::ALL),
            );
            paragraph.render(area, buf);
        }

        let widths = [
            Constraint::Percentage(10),
            Constraint::Percentage(45),
            Constraint::Percentage(45),
        ];
        let table = Table::new(rows, widths)
            .column_spacing(1)
            .header(
                Row::new(vec!["ID", "Item", "Description"])
                    .style(Style::new().bold())
                    .bottom_margin(1),
            )
            .block(Block::default().title("Inventory").borders(Borders::ALL))
            .row_highlight_style(Style::new().reversed())
            .highlight_symbol(">>");

        StatefulWidget::render(table, area, buf, state);
    }
}
