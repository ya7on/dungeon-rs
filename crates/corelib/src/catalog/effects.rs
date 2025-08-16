use crate::effects::{EffectDef, EffectId, EffectKind};

#[derive(Debug)]
pub struct EffectsCatalog {
    effects: Vec<EffectDef>,
}

impl EffectsCatalog {
    pub(crate) fn new() -> Self {
        EffectsCatalog {
            effects: vec![EffectDef {
                name: "Healing".to_string(),
                description: "Restores health.".to_string(),
                kind: EffectKind::Heal { hp_per_turn: 1 },
            }],
        }
    }

    /// Get an item by its ID
    #[must_use]
    pub fn get(&self, id: EffectId) -> Option<&EffectDef> {
        self.effects.get(id)
    }
}
