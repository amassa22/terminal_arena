use super::hand_item::HandItem;
use super::armor::Armor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Store {
    pub hand_items: Vec<HandItem>,
    pub armor: Vec<Armor>,
}