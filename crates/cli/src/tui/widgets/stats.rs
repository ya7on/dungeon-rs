use corelib::GameState;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct StatsWidget {
    hp: u32,
    attack: u32,
    defense: u32,
}

impl From<&GameState> for StatsWidget {
    fn from(game_state: &GameState) -> Self {
        Self {
            hp: game_state.player().stats().hp(),
            attack: game_state.player().stats().attack(),
            defense: game_state.player().stats().defense(),
        }
    }
}

impl Widget for StatsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let hp = self.hp;
        let attack = self.attack;
        let defense = self.defense;

        let paragraph = Paragraph::new(format!(
            "HP: {hp}\nAttack: {attack}\nDefense: {defense}"
        ))
        .block(Block::default().title("Stats").borders(Borders::ALL));
        paragraph.render(area, buf);
    }
}
