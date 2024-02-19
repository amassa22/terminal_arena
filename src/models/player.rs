use serde::{Serialize, Deserialize};
use super::hand_item::HandItem;
use prettytable::{Table, Row, Cell, format, row, cell};
use super::inventory::Inventory;
use super::armor::Armor;
use super::weapon::Weapon;
use super::item_type::ItemType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub fame: u8, 
    tiredness_level: u8, // max 255
    pub health: i32,
    pub max_health: i32,
    pub strength: u8,
    pub defense: u8,
    pub agility: u8,
    pub left_hand: HandItem,
    pub right_hand: Option<HandItem>,
    pub helmet: Option<Armor>,
    pub breastplate: Option<Armor>,
    pub legs: Option<Armor>,
    pub money: i32,
    pub inventory: Inventory
}

impl Player {
    pub fn new (name: String) -> Player {
        let weapon = Weapon::new("Basic Rusty Sword".to_string(), ItemType::SingleHand, 1, 3, 1);
        let inventory = Inventory::new();


        Player {name, left_hand: HandItem::Weapon(weapon), fame: 0, tiredness_level: 0, health: 100, strength: 5, agility: 5, defense: 1, money: 0, max_health:100, inventory, helmet: None, right_hand: None, breastplate: None, legs: None }
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
    }

    pub fn heal (&mut self, amount: i32) {
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

        table.set_titles(Row::new(vec![
            Cell::new("Attribute"),
            Cell::new("Value"),
        ]));
        table.add_row(row!["Money", format!("💰 {}", self.money)]);
        table.add_row(row!["Fame", format!("🏆 {}", self.fame)]);
        table.add_row(row!["Tiredness Level", format!("⚡ {}", self.tiredness_level)]);
        table.add_row(row!["Health", format!("❤️ {}/{}", self.health, self.max_health)]);
        table.add_row(row!["Strength", format!("💪 {}", self.strength)]);
        table.add_row(row!["Defense", format!("🛡️ {}", self.defense)]);
        table.add_row(row!["Agility", format!("🏃 {}", self.agility)]);
        
        println!("👤 Player Information: {}", self.name);
        table.printstd();

    }

    pub fn player_inventory(&self) {
        let mut weapons_table = Table::new();
        let mut shield_table = Table::new();
        
        weapons_table.set_titles(row!["Name", "Type", "Damage", "Required Strength"]);
        shield_table.set_titles(row!["Name", "Block Damage", "Required Strength"]);
        
        for item in &self.inventory.hand_items {
            match item {
                HandItem::Weapon(weapon) => {
                    weapons_table.add_row(row![&weapon.name, &weapon.item_type, format!("{}-{}", weapon.min_damage, weapon.max_damage), weapon.req_strength.to_string()]);
                },
                HandItem::Shield(shield) => {
                    shield_table.add_row(row![&shield.name, shield.block_damage.to_string(), shield.req_strength.to_string()]);
                },
                HandItem::BareHand => {
                    // Explicitly do nothing here, which should not cause a type inconsistency
                },
            }
        }

        let mut armors_table = Table::new();
        armors_table.set_titles(row!["Name", "Type", "Defense", "Weight", "Required Strength", "Price"]);

        for item in &self.inventory.armors {
            armors_table.add_row(row![item.name, item.armor_type, item.defense, item.weight, item.req_strength, item.price]);
        }

        // Print the tables
        println!("Inventory");
        print_line();
        println!("Weapons");
        weapons_table.printstd();
        println!("Shields");
        shield_table.printstd();
        println!("Armors");
        armors_table.printstd();
    }

}


fn print_line() {
    println!("{}", "═".repeat(30)); // Adjust the number of repetitions to fit your layout
}
