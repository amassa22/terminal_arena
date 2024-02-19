use std::fmt;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum ItemType {
    BareHand,
    SingleHand,
    DoubleHand,
}



impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ItemType::BareHand => write!(f, "Bare-Handed"),
            ItemType::SingleHand => write!(f, "Single-Handed"),
            ItemType::DoubleHand => write!(f, "Double-Handed"),
        }
    }
}