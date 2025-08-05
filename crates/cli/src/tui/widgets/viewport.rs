use corelib::{GameState, Position, Tile};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{Block, Borders, Widget},
};

pub struct ViewportWidget<'a> {
    state: &'a GameState,
}

impl<'a> ViewportWidget<'a> {
    pub fn new(state: &'a GameState) -> Self {
        Self { state }
    }
}

impl Widget for ViewportWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().title("Game View").borders(Borders::ALL);
        let inner = block.inner(area);
        block.render(area, buf);

        let dungeon = self.state.dungeon();
        let player = self.state.player();
        let player_pos = player.position();

        let width = inner.width;
        let height = inner.height;

        let player_x = player_pos.x;
        let player_y = player_pos.y;

        for x in 0..width {
            for y in 0..height {
                let dx = x as i32 - (width / 2) as i32;
                let dy = y as i32 - (height / 2) as i32;

                let pos = Position::new(player_x + dx, player_y + dy);
                let tile = dungeon.get_tile(&pos);

                let (mut symbol, mut color) = match tile {
                    Tile::Floor => ("_", Color::White),
                    Tile::Empty => (".", Color::Black),
                };

                // TODO: Fix it
                if self.state.entities().iter().any(|e| e.position() == &pos) {
                    symbol = "E";
                    color = Color::Red;
                };
                if &pos == player_pos {
                    symbol = "@";
                    color = Color::Yellow;
                }

                let screen_x = inner.x + x;
                let screen_y = inner.y + y;

                if let Some(cell) = buf.cell_mut((screen_x, screen_y)) {
                    cell.set_symbol(symbol).set_style(color);
                }
            }
        }
    }
}
