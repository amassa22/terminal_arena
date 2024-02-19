use std::fmt;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
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


#[derive(Serialize, Deserialize, Debug)]
pub struct Armor {
    pub name: String,
    pub armor_type: ArmorType,
    pub defense: u8,
    pub req_strength: u8,
    pub price: u32,
    pub weight: u32
}


impl Armor {
    pub fn new(name: String, armor_type: ArmorType, defense:u8, req_strength: u8, price: u32, weight:u32) -> Armor {
        Armor {name, armor_type, defense, req_strength, price, weight}
    }
}