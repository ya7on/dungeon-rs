use crate::items::{ItemDef, ItemId, ItemKind};

#[derive(Debug)]
pub struct ItemsCatalog {
    items: Vec<ItemDef>,
}

impl ItemsCatalog {
    pub(crate) fn new() -> Self {
        ItemsCatalog {
            items: vec![
                /* --- Weapon --- */
                ItemDef {
                    kind: ItemKind::Weapon { min_damage: 10, max_damage: 20 },
                    name: "sword_1".to_string(),
                    title: "Sword".to_string(),
                    description: "A basic sword.".to_string(),
                    stackable: false,
                },
                ItemDef {
                    kind: ItemKind::Weapon { min_damage: 15, max_damage: 25 },
                    name: "sword_2".to_string(),
                    title: "Greatsword".to_string(),
                    description: "A powerful sword.".to_string(),
                    stackable: false,
                },
                /* --- Armor --- */
                ItemDef {
                    kind: ItemKind::Armor { defense: 5 },
                    name: "armor_1".to_string(),
                    title: "Leather Armor".to_string(),
                    description: "A simple leather armor.".to_string(),
                    stackable: false,
                },
            ],
        }
    }

    /// Get an item by its ID
    #[must_use]
    pub fn get(&self, id: ItemId) -> Option<&ItemDef> {
        self.items.get(id)
    }
}
