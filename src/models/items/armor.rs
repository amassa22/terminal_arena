use serde::{Deserialize, Serialize};
use std::fmt;
use prettytable::{cell, row, Row};
use super::item::Item;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum ArmorType {
    Helmet,
    // Shoulder,
    // Gloves,
    BreastPlate,
    Legs,
    // Boots,
}

impl fmt::Display for ArmorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArmorType::Helmet => write!(f, "Helmet"),
            ArmorType::BreastPlate => write!(f, "BreastPlate"),
            ArmorType::Legs => write!(f, "Legs"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Armor {
    pub name: String,
    pub armor_type: ArmorType,
    pub defense: u8,
    pub req_strength: u8,
    pub price: u32,
    pub weight: u32,
}

impl fmt::Display for Armor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Armor {
    pub fn new(
        name: String,
        armor_type: ArmorType,
        defense: u8,
        req_strength: u8,
        price: u32,
        weight: u32,
    ) -> Armor {
        Armor {
            name,
            armor_type,
            defense,
            req_strength,
            price,
            weight,
        }
    }

    pub fn to_row(&self) -> Row {
        row![
            &self.name,
            &self.armor_type,
            &self.defense,
            &self.weight,
            &self.req_strength,
            &self.price
        ]
    }
}