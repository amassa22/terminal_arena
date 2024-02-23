use super::items::armor::ArmorType;
use super::items::hand_item::{HandItem, HandItemType};
use super::items::weapon::Weapon;
use super::items::{armor::Armor, shield::Shield};
use rand::seq::index;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Inventory {
    pub hand_items: Vec<HandItem>,
    pub armors: Vec<Armor>,
}

impl Inventory {
    pub fn new() -> Self {
        //TODO remove after testing
        let shield = Shield::new(
            "Basic Shield".to_string(),
            HandItemType::Single,
            15,
            10,
            15,
            25,
        );
        let shield2 = Shield::new(
            "Advanced Shield".to_string(),
            HandItemType::Single,
            15,
            50,
            15,
            25,
        );

        let armor = Armor::new("Basic Helmet".to_string(), ArmorType::Helmet, 5, 10, 100, 5);
        let weapon2 = Weapon::new(
            "Advanced Rusty Sword".to_string(),
            HandItemType::Single,
            1,
            3,
            1,
            5,
        );
        let mut hand_items = Vec::new();
        let mut armors = Vec::new();

        armors.push(armor);
        hand_items.push(HandItem::Shield(shield));
        hand_items.push(HandItem::Shield(shield2));
        hand_items.push(HandItem::Weapon(weapon2));
        //

        Inventory {
            hand_items: hand_items,
            armors: armors,
        }
    }

    pub fn add_hand_item(&mut self, item: HandItem) {
        self.hand_items.push(item);
    }

    pub fn remove_hand_item(&mut self, index: usize) {
        self.hand_items.remove(index);
    }

    pub fn add_armor(&mut self, item: Armor) {
        self.armors.push(item);
    }
}
