use super::inventory::Inventory;
use super::items::hand_item::Equipment;
use super::items::hand_item::HandItem;
use super::utils::print_line;
use prettytable::{cell, row, Cell, Row, Table};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Fame {
    Novice,
    Apprentice,
    Veteran,
    Champion,
    Hero,
    Legend,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub fame: i32,
    pub fame_level: Fame,
    tiredness_level: u8, // max 255
    pub health: i32,
    pub max_health: i32,
    pub strength: u8,
    pub defense: u8,
    pub agility: u8,
    pub money: i32,
    pub inventory: Inventory,
    pub victories: i32,
    pub injured: bool,
    pub equipment: Equipment,
}

impl Player {
    pub fn new(name: String) -> Player {
        let inventory = Inventory::new();
        let equipment = Equipment::new();
        Player {
            name,
            fame: 0,
            fame_level: Fame::Novice,
            tiredness_level: 0,
            health: 100,
            strength: 5,
            agility: 5,
            defense: 1,
            money: 0,
            max_health: 100,
            inventory,
            victories: 0,
            injured: false,
            equipment,
        }
    }

    pub fn take_damage(&mut self, amount: i32) {
        // TODO: check for overflow for negative health
        self.health -= amount;
    }

    pub fn heal(&mut self, amount: i32) {
        self.health += amount;
        if self.health >= self.max_health {
            self.health = self.max_health;
        }
    }

    pub fn health_bar(&self) -> String {
        let total_blocks = 10;
        let health_percentage = self.health as f32 / self.max_health as f32;
        let filled_blocks = (total_blocks as f32 * health_percentage).round() as i32;
        let mut bar = String::new();
        for _ in 0..filled_blocks {
            bar.push('█');
        }
        for _ in filled_blocks..total_blocks {
            bar.push('░');
        }
        format!("Health: [{}] {}/{}", bar, self.health, self.max_health)
    }

    pub fn player_info(&self) {
        print_line();

        let mut table = Table::new();

        table.set_titles(Row::new(vec![Cell::new("Attribute"), Cell::new("Value")]));
        table.add_row(row!["Money", format!("💰 {}", self.money)]);
        table.add_row(row!["Fame", format!("🏆 {:?}", self.fame_level)]); // TODO: implement display for Fame
        table.add_row(row![
            "Tiredness Level",
            format!("⚡ {}", self.tiredness_level)
        ]);
        table.add_row(row!["Victories", format!("⚔️  {}", self.victories)]);
        table.add_row(row![
            "Health",
            format!("❤️ {}/{}", self.health, self.max_health)
        ]);
        table.add_row(row!["Strength", format!("💪 {}", self.strength)]);
        table.add_row(row!["Defense", format!("🛡️ {}", self.defense)]);
        table.add_row(row!["Agility", format!("🏃 {}", self.agility)]);

        println!("👤 Player Information: {}", self.name);
        table.printstd();
    }

    pub fn player_inventory_weapon(&self) {
        let mut weapons_table = Table::new();

        weapons_table.set_titles(row!["Name", "Type", "Damage", "Required Strength"]);

        for item in &self.inventory.hand_items {
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
        println!("Inventory");
        print_line();
        println!("Weapons");
        weapons_table.printstd();
    }

    pub fn player_inventory_shields(&self) {
        let mut shields_table = Table::new();

        shields_table.set_titles(row![
            "Name",
            "Type",
            "Block Damage",
            "Weight",
            "Required Strength",
            "Price"
        ]);

        for item in &self.inventory.hand_items {
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
        println!("Inventory");
        print_line();
        println!("Shields");
        shields_table.printstd();
    }

    pub fn player_inventory_armor(&self) {
        let mut armors_table = Table::new();
        armors_table.set_titles(row![
            "Name",
            "Type",
            "Defense",
            "Weight",
            "Required Strength",
            "Price"
        ]);

        for item in &self.inventory.armors {
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
}
