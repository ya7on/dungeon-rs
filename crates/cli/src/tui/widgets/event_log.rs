use std::collections::VecDeque;

use corelib::GameEvent;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct EventLogWidget<'a> {
    log: &'a VecDeque<GameEvent>,
}

impl<'a> EventLogWidget<'a> {
    pub fn new(log: &'a VecDeque<GameEvent>) -> Self {
        Self { log }
    }
}

impl Widget for EventLogWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut log = String::new();

        for event in self.log {
            match event {
                GameEvent::PlayerSkippedMove => {
                    log.push_str("Player skipped their turn.\n")
                },
                GameEvent::PlayerDied => log.push_str("Player died.\n"),
                GameEvent::PlayerMoved { from: _, to: _ } => {
                    log.push_str("Player moved.\n")
                },
                GameEvent::PlayerBumped { position: _, direction: _ } => {
                    log.push_str("Player bumped into something.\n")
                },
                GameEvent::PlayerAttacked { damage, target: _ } => log
                    .push_str(&format!(
                        "Player attacked with {damage} damage.\n"
                    )),
                GameEvent::PlayerAttackMissed => {
                    log.push_str("Player attack missed.\n")
                },
                GameEvent::PlayerEquippedItem { item_id: _, slot } => log
                    .push_str(&format!(
                        "Player equipped an item to {slot} slot.\n"
                    )),
                GameEvent::PlayerUnequippedItem { slot } => log.push_str(
                    &format!("Player unequipped an item from {slot} slot.\n"),
                ),
                GameEvent::EntityMoved { from: _, id, to: _ } => {
                    log.push_str(&format!("Entity({id:?}) moved.\n",))
                },
                GameEvent::EntityAttacked { damage, id, target: _ } => log
                    .push_str(&format!(
                        "Entity({id:?}) attacked with {damage} damage.\n",
                    )),
                GameEvent::EffectTick { entity_id, effect_id: _ } => log
                    .push_str(&format!(
                        "Entity({entity_id:?}) effect ticked.\n",
                    )),
                GameEvent::EffectExpired { entity_id, effect_id: _ } => log
                    .push_str(&format!(
                        "Entity({entity_id:?}) effect expired.\n",
                    )),
            }
        }

        let paragraph = Paragraph::new(log)
            .block(Block::default().title("Logs").borders(Borders::ALL));
        paragraph.render(area, buf);
    }
}
