use super::armor::Armor;
use super::hand_item::HandItem;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Store {
    pub hand_items: Vec<HandItem>,
    pub armor: Vec<Armor>,
}
