use super::hand_item::HandItemType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Weapon {
    pub name: String,
    pub item_type: HandItemType,
    pub min_damage: u8,
    pub max_damage: u8,
    pub req_strength: u8,
}

impl Weapon {
    pub fn new(
        name: String,
        item_type: HandItemType,
        min_damage: u8,
        max_damage: u8,
        req_strength: u8,
    ) -> Weapon {
        Weapon {
            name,
            item_type,
            min_damage,
            max_damage,
            req_strength,
        }
    }
}
