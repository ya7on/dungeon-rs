use corelib::{GameState, Position, Tile};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{Block, Borders, Widget},
};

#[derive(Debug, Clone, Copy)]
enum Symbol {
    Empty,
    Floor,
    Player,
    Enemy,
}

pub struct ViewportWidget {
    field: [[(Symbol, Color); 9]; 9],
}

impl From<&GameState> for ViewportWidget {
    fn from(state: &GameState) -> Self {
        let mut field = [[(Symbol::Empty, Color::Black); 9]; 9];

        let player = state.player();

        for row in -4..5 {
            for col in -4..5 {
                let tile = state
                    .dungeon()
                    .get_tile(&(player.position().clone() + Position::new(row, col)));

                let x_index = (col + 4) as usize;
                let y_index = (row + 4) as usize;
                match tile {
                    Tile::Floor => field[x_index][y_index] = (Symbol::Floor, Color::Black),
                    Tile::Empty => {
                        field[x_index][y_index] = (Symbol::Empty, Color::Black);
                    }
                }
            }
        }

        for entity in state.entities() {
            let position = entity.position();
            let relative_position = position.clone() - player.position().clone();

            if relative_position.x <= 4
                && relative_position.y <= 4
                && relative_position.x >= -4
                && relative_position.y >= -4
            {
                field[(relative_position.y + 4) as usize][(relative_position.x + 4) as usize] =
                    (Symbol::Enemy, Color::Red);
            }
        }

        field[4][4] = (Symbol::Player, Color::Green);

        Self { field }
    }
}

impl Widget for ViewportWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().title("Game View").borders(Borders::ALL);
        let inner = block.inner(area);
        block.render(area, buf);

        // TODO
        for row in 0..9 {
            for col in 0..9 {
                let symbol = self.field[row][col].0;
                let color = self.field[row][col].1;
                let position = (col as u16 + inner.x, row as u16 + inner.y);
                if let Some(cell) = buf.cell_mut(position) {
                    let symbol = match symbol {
                        Symbol::Floor => "_",
                        Symbol::Empty => ".",
                        Symbol::Enemy => "E",
                        Symbol::Player => "@",
                    };
                    cell.set_symbol(symbol).set_style(color);
                }
            }
        }
    }
}
