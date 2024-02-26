use super::inventory::Inventory;
use super::items::hand_item::Equipment;
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
    pub energy: u8, // max 255
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
            energy: 100,
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
        table.add_row(row!["Energy", format!("⚡ {}", self.energy)]);
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
}
