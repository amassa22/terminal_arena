use crate::models::items::shield;
use crate::models::utils::print_line;

use super::items::hand_item::{HandItem, HandItemType};
use super::items::shield::Shield;
use super::items::{armor::Armor, weapon::Weapon};
use prettytable::{cell, row, Table};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    pub weapons: Vec<Weapon>,
    pub shields: Vec<Shield>,
    pub helmets: Vec<Armor>,
}

impl Store {
    pub fn new() -> Self {
        let weapons: Vec<Weapon> = vec![
            Weapon::new("Rusty Dagger".to_string(), HandItemType::Single, 2, 5, 5, 5),
            Weapon::new(
                "Wooden Club".to_string(),
                HandItemType::Single,
                4,
                8,
                10,
                200,
            ),
            Weapon::new(
                "Short Sword".to_string(),
                HandItemType::Single,
                6,
                10,
                15,
                350,
            ),
            Weapon::new(
                "Battle Axe".to_string(),
                HandItemType::Single,
                8,
                15,
                20,
                500,
            ),
            Weapon::new(
                "War Hammer".to_string(),
                HandItemType::Double,
                10,
                18,
                25,
                700,
            ),
            Weapon::new(
                "Longsword".to_string(),
                HandItemType::Single,
                12,
                20,
                30,
                900,
            ),
            Weapon::new("Flail".to_string(), HandItemType::Single, 14, 22, 35, 1100),
            Weapon::new(
                "Greatsword".to_string(),
                HandItemType::Double,
                16,
                25,
                40,
                1350,
            ),
            Weapon::new(
                "Halberd".to_string(),
                HandItemType::Double,
                18,
                28,
                45,
                1600,
            ),
            Weapon::new(
                "Mystic Staff".to_string(),
                HandItemType::Single,
                20,
                30,
                50,
                2000,
            ),
        ];

        Store {
            weapons,
            shields: Vec::new(),
            helmets: Vec::new(),
        }
    }

    pub fn add_weapon(&mut self, item: Weapon) {
        self.weapons.push(item);
    }

    pub fn remove_weapon(&mut self, index: usize) {
        self.weapons.remove(index);
    }

    pub fn print_all_weapons(&self) {
        let mut weapons_table = Table::new();

        weapons_table.set_titles(row!["Name", "Type", "Damage", "Required Strength", "Price"]);

        for weapon in &self.weapons {
            weapons_table.add_row(row![
                &weapon.name,
                &weapon.item_type,
                format!("{}-{}", weapon.min_damage, weapon.max_damage),
                weapon.req_strength.to_string(),
                weapon.price
            ]);
        }

        // Print the tablesß
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

        for shield in &self.shields {
            shields_table.add_row(row![
                &shield.name,
                &shield.item_type,
                shield.block_damage,
                shield.weight,
                shield.req_strength,
                shield.price
            ]);
        }
        // Print the tables
        println!("Store");
        print_line();
        println!("Shields");
        shields_table.printstd();
    }
}
