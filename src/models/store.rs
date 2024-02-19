use super::items::armor::Armor;
use super::items::hand_item::HandItem;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Store {
    pub hand_items: Vec<HandItem>,
    pub armor: Vec<Armor>,
}
