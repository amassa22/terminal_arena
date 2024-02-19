use super::item_type::ItemType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Shield {
    pub name: String,
    pub item_type: ItemType,
    pub block_damage: u8,
    pub req_strength: u8,
}

impl Shield {
    pub fn new(name: String, item_type: ItemType, block_damage: u8, req_strength: u8) -> Shield {
        Shield {
            name,
            item_type,
            block_damage,
            req_strength,
        }
    }
}
