use super::armor::Armor;
use super::weapon::Weapon;
use super::{armor::ArmorType, shield::Shield};
use prettytable::{cell, row, Table};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum HandItem {
    Weapon(Weapon),
    Shield(Shield),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum HandItemType {
    Single,
    Double,
}

impl fmt::Display for HandItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HandItemType::Single => write!(f, "Single-Handed"),
            HandItemType::Double => write!(f, "Double-Handed"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Hand {
    Empty,
    Busy, // TODO: Assuming TwoHanded weapon occupies both hands find a better name
    Single(HandItem),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ArmorSet {
    pub helmet: Option<Armor>,
    pub breastplate: Option<Armor>,
    pub boots: Option<Armor>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Equipment {
    pub left_hand: Hand,
    pub right_hand: Hand,
    pub armor: ArmorSet,
}

impl Equipment {
    pub fn new() -> Self {
        let weapon = Weapon::new(
            "Basic Rusty Sword".to_string(),
            HandItemType::Single,
            1,
            3,
            1,
            5,
        );
        Equipment {
            right_hand: Hand::Single(HandItem::Weapon(weapon)),
            left_hand: Hand::Empty,
            armor: ArmorSet {
                helmet: None,
                breastplate: None,
                boots: None,
            },
        }
    }

    pub fn equip_weapon(&mut self, weapon: Weapon) {
        match weapon.item_type {
            HandItemType::Single => self.right_hand = Hand::Single(HandItem::Weapon(weapon)),
            HandItemType::Double => {
                self.right_hand = Hand::Single(HandItem::Weapon(weapon));
                self.left_hand = Hand::Busy; // Two-handed weapons occupy both hands
            }
        }
    }

    pub fn equip_shield(&mut self, shield: Shield) {
        match &self.left_hand {
            Hand::Empty => self.left_hand = Hand::Single(HandItem::Shield(shield)),
            Hand::Single(HandItem::Shield(_)) => {
                self.left_hand = Hand::Single(HandItem::Shield(shield))
            }
            Hand::Single(HandItem::Weapon(_)) => {
                self.left_hand = Hand::Single(HandItem::Shield(shield))
            }
            Hand::Busy => {
                self.left_hand = Hand::Single(HandItem::Shield(shield));
                self.right_hand = Hand::Empty;
            }
        }
    }

    pub fn equip_armor(&mut self, armor: Armor) {
        match armor.armor_type {
            ArmorType::Helmet => self.armor.helmet = Some(armor),
            ArmorType::BreastPlate => self.armor.breastplate = Some(armor),
            ArmorType::Legs => self.armor.boots = Some(armor),
        }
    }

    pub fn to_pretty_table(&self) {
        let mut table = Table::new();
        table.add_row(row!["Slot", "Item", "Details"]);

        // Handle hands equipment
        match &self.left_hand {
            Hand::Busy => table.add_row(row!["Left Hand", "Busy", ""]),
            Hand::Empty => table.add_row(row!["Left Hand", "Empty", ""]),
            Hand::Single(item) => match item {
                HandItem::Weapon(weapon) => table.add_row(row![
                    "Left Hand",
                    "Weapon",
                    format!(
                        "{}: {}-{} dmg, req str: {}",
                        weapon.name, weapon.min_damage, weapon.max_damage, weapon.req_strength
                    )
                ]),
                HandItem::Shield(shield) => table.add_row(row![
                    "Left Hand",
                    "Shield",
                    format!(
                        "{}: {} def, req str: {}",
                        shield.name, shield.block_damage, shield.req_strength
                    )
                ]),
            },
        };

        match &self.right_hand {
            Hand::Busy => table.add_row(row!["Right Hand", "Busy", ""]),
            Hand::Empty => table.add_row(row!["Right Hand", "Empty", ""]),
            Hand::Single(item) => match item {
                HandItem::Weapon(weapon) => table.add_row(row![
                    "Right Hand",
                    "Weapon",
                    format!(
                        "{}: {}-{} dmg, req str: {}",
                        weapon.name, weapon.min_damage, weapon.max_damage, weapon.req_strength
                    )
                ]),
                HandItem::Shield(shield) => table.add_row(row![
                    "Right Hand",
                    "Shield",
                    format!(
                        "{}: {} def, req str: {}",
                        shield.name, shield.block_damage, shield.req_strength
                    )
                ]),
            },
        };

        // Handle armor equipment
        if let Some(helmet) = &self.armor.helmet {
            table.add_row(row![
                "Helmet",
                helmet.name.clone(),
                format!("{} def, req str: {}", helmet.defense, helmet.req_strength)
            ]);
        }

        if let Some(breastplate) = &self.armor.breastplate {
            table.add_row(row![
                "Breastplate",
                breastplate.name.clone(),
                format!(
                    "{} def, req str: {}",
                    breastplate.defense, breastplate.req_strength
                )
            ]);
        }

        if let Some(boots) = &self.armor.boots {
            table.add_row(row![
                "Boots",
                boots.name.clone(),
                format!("{} def, req str: {}", boots.defense, boots.req_strength)
            ]);
        }

        // Print the table to stdout
        table.printstd();
    }
}
