use super::hand_item::HandItemType;
use super::item::Item;
use prettytable::{cell, row, Row};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Shield {
    pub name: String,
    pub item_type: HandItemType,
    pub block_damage: u8,
    pub req_strength: u8,
    pub price: i32,
    pub weight: i32,
}

impl Shield {
    pub fn new(
        name: String,
        item_type: HandItemType,
        block_damage: u8,
        req_strength: u8,
        price: i32,
        weight: i32,
    ) -> Shield {
        Shield {
            name,
            item_type,
            block_damage,
            req_strength,
            price,
            weight,
        }
    }

    pub fn to_row(&self) -> Row {
        row![
            &self.name,
            &self.item_type,
            &self.block_damage,
            &self.weight,
            &self.req_strength,
            &self.price
        ]
    }
}
