use corelib::GameState;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct StatsWidget {
    hp: u32,
    min_damage: u32,
    max_damage: u32,
    defense: u32,
}

impl From<&GameState> for StatsWidget {
    fn from(game_state: &GameState) -> Self {
        Self {
            hp: game_state.player().stats().hp(),
            min_damage: game_state.player().stats().min_damage(),
            max_damage: game_state.player().stats().max_damage(),
            defense: game_state.player().stats().defense(),
        }
    }
}

impl Widget for StatsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let hp = self.hp;
        let min_damage = self.min_damage;
        let max_damage = self.max_damage;
        let defense = self.defense;

        let paragraph = Paragraph::new(format!(
            "HP: {hp}\nAttack: {min_damage}-{max_damage}\nDefense: {defense}"
        ))
        .block(Block::default().title("Stats").borders(Borders::ALL));
        paragraph.render(area, buf);
    }
}
