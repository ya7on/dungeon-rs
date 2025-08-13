use crate::items::{ItemDef, ItemId, ItemKind};

#[derive(Debug)]
pub(crate) struct ItemsCatalog {
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
                    stackable: false,
                },
                ItemDef {
                    kind: ItemKind::Weapon { min_damage: 15, max_damage: 25 },
                    name: "sword_2".to_string(),
                    stackable: false,
                },
                /* --- Armor --- */
                ItemDef {
                    kind: ItemKind::Armor { defense: 5 },
                    name: "armor_1".to_string(),
                    stackable: false,
                },
            ],
        }
    }

    pub(crate) fn get(&self, id: ItemId) -> Option<&ItemDef> {
        self.items.get(id)
    }
}
