use super::hand_item::HandItemType;
use serde::{Deserialize, Serialize};
use prettytable::{cell, row};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Weapon {
    pub name: String,
    pub item_type: HandItemType,
    pub min_damage: u8,
    pub max_damage: u8,
    pub req_strength: u8,
    pub price: i32,
}

impl Weapon {
    pub fn new(
        name: String,
        item_type: HandItemType,
        min_damage: u8,
        max_damage: u8,
        req_strength: u8,
        price: i32,
    ) -> Weapon {
        Weapon {
            name,
            item_type,
            min_damage,
            max_damage,
            req_strength,
            price,
        }
    }

    pub fn to_row(&self) -> prettytable::Row {
        row![
            &self.name,
            &self.item_type,
            format!("{}-{}", self.min_damage, self.max_damage),
            self.req_strength.to_string()
        ]
    }
}

