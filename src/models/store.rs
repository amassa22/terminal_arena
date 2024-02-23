use crate::models::utils::print_line;

use super::items::hand_item::{HandItem, HandItemType};
use super::items::{armor::Armor, weapon::Weapon};
use prettytable::{cell, row, Cell, Row, Table};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    pub hand_items: Vec<HandItem>,
    pub armor: Vec<Armor>,
}

impl Store {
    pub fn new() -> Self {
        let weapons: Vec<HandItem> = vec![
            HandItem::Weapon(Weapon::new(
                "Rusty Dagger".to_string(),
                HandItemType::Single,
                2,
                5,
                5,
                5,
            )),
            HandItem::Weapon(Weapon::new(
                "Wooden Club".to_string(),
                HandItemType::Single,
                4,
                8,
                10,
                200,
            )),
            HandItem::Weapon(Weapon::new(
                "Short Sword".to_string(),
                HandItemType::Single,
                6,
                10,
                15,
                350,
            )),
            HandItem::Weapon(Weapon::new(
                "Battle Axe".to_string(),
                HandItemType::Single,
                8,
                15,
                20,
                500,
            )),
            HandItem::Weapon(Weapon::new(
                "War Hammer".to_string(),
                HandItemType::Double,
                10,
                18,
                25,
                700,
            )),
            HandItem::Weapon(Weapon::new(
                "Longsword".to_string(),
                HandItemType::Single,
                12,
                20,
                30,
                900,
            )),
            HandItem::Weapon(Weapon::new(
                "Flail".to_string(),
                HandItemType::Single,
                14,
                22,
                35,
                1100,
            )),
            HandItem::Weapon(Weapon::new(
                "Greatsword".to_string(),
                HandItemType::Double,
                16,
                25,
                40,
                1350,
            )),
            HandItem::Weapon(Weapon::new(
                "Halberd".to_string(),
                HandItemType::Double,
                18,
                28,
                45,
                1600,
            )),
            HandItem::Weapon(Weapon::new(
                "Mystic Staff".to_string(),
                HandItemType::Single,
                20,
                30,
                50,
                2000,
            )),
        ];

        Store {
            hand_items: weapons,
            armor: Vec::new(),
        }
    }

    pub fn add_hand_item(&mut self, item: HandItem) {
        self.hand_items.push(item);
    }

    pub fn remove_hand_item(&mut self, index: usize) {
        self.hand_items.remove(index);
    }

    pub fn print_all_weapons(&self) {
        let mut weapons_table = Table::new();

        weapons_table.set_titles(row!["Name", "Type", "Damage", "Required Strength"]);

        for item in &self.hand_items {
            match item {
                HandItem::Weapon(weapon) => {
                    weapons_table.add_row(row![
                        &weapon.name,
                        &weapon.item_type,
                        format!("{}-{}", weapon.min_damage, weapon.max_damage),
                        weapon.req_strength.to_string()
                    ]);
                }
                HandItem::Shield(_) => {}
            }
        }
        // Print the tables
        println!("Store");
        print_line();
        println!("Weapons");
        weapons_table.printstd();
    }

    pub fn print_all_shields(&self) {
        let mut shields_table = Table::new();

        shields_table.set_titles(row![
            "Name",
            "Type",
            "Block Damage",
            "Weight",
            "Required Strength",
            "Price"
        ]);

        for item in &self.hand_items {
            match item {
                HandItem::Shield(shield) => {
                    shields_table.add_row(row![
                        &shield.name,
                        &shield.item_type,
                        shield.block_damage,
                        shield.weight,
                        shield.req_strength,
                        shield.price
                    ]);
                }
                _ => {}
            }
        }
        // Print the tables
        println!("Store");
        print_line();
        println!("Shields");
        shields_table.printstd();
    }
}
