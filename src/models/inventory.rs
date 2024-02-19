use super::hand_item::HandItem;
use super::armor::Armor;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Inventory {
    pub hand_items: Vec<HandItem>,
    pub armors: Vec<Armor>
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            hand_items: Vec::new(),
            armors: Vec::new(),
        }
    }
    pub fn add_hand_item(&mut self, item: HandItem) {
        self.hand_items.push(item);
    }

    pub fn add_armor(&mut self, item: Armor) {
        self.armors.push(item);
    }
}