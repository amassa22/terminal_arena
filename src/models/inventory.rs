use crate::models::items::weapon;
use crate::models::utils::print_line;

use super::items::armor::ArmorType;
use super::items::hand_item::{HandItem, HandItemType};
use super::items::shield;
use super::items::weapon::Weapon;
use super::items::{armor::Armor, shield::Shield};
use rand::seq::index;
use serde::{Deserialize, Serialize};
use prettytable::{cell, row, Cell, Row, Table};

#[derive(Serialize, Deserialize, Debug)]
pub struct Inventory {
    pub weapons: Vec<Weapon>,
    pub shields: Vec<Shield>,
    pub helmets: Vec<Armor>
}

impl Inventory {
    pub fn new() -> Self {
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

        let helmet = Armor::new("Basic Helmet".to_string(), ArmorType::Helmet, 5, 10, 100, 5);
        let weapon2 = Weapon::new(
            "Advanced Rusty Sword".to_string(),
            HandItemType::Single,
            1,
            3,
            1,
            5,
        );
        let mut weapons  = Vec::new();
        let mut shields = Vec::new();
        let mut helmets = Vec::new();
        helmets.push(helmet);
        shields.push(shield);
        shields.push(shield2);
        weapons.push(weapon2);
        //

        Inventory {
            weapons,
            shields,
            helmets,
        }
    }

    pub fn add_weapon(&mut self, w: Weapon) {
        self.weapons.push(w);
    }

    pub fn remove_weapon(&mut self, index: usize) {
        self.weapons.remove(index);
    }

    pub fn add_helmet(&mut self, item: Armor) {
        self.helmets.push(item);
    }

    pub fn print_all_weapons(&self) {
        let mut weapons_table = Table::new();

        weapons_table.set_titles(row!["Name", "Type", "Damage", "Required Strength"]);

        for weapon in &self.weapons {
            weapons_table.add_row(row![
                &weapon.name,
                &weapon.item_type,
                format!("{}-{}", weapon.min_damage, weapon.max_damage),
                weapon.req_strength.to_string()
            ]);
        }
        println!("Inventory");
        print_line();
        println!("Weapons");
        weapons_table.printstd();
    }

    pub fn print_all_helmets(&self) {
        let mut armors_table = Table::new();
        armors_table.set_titles(row![
            "Name",
            "Type",
            "Defense",
            "Weight",
            "Required Strength",
            "Price"
        ]);

        for item in &self.helmets {
            armors_table.add_row(row![
                item.name,
                item.armor_type,
                item.defense,
                item.weight,
                item.req_strength,
                item.price
            ]);
        }

        // Print the tables
        println!("Inventory");
        print_line();
        println!("Armors");
        armors_table.printstd();
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
        println!("Inventory");
        print_line();
        println!("Shields");
        shields_table.printstd();
    }

}
