use super::items::armor::Armor;
use super::items::hand_item::HandItem;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Enemy {
    name: String,
    fame: u8,
    tiredness_level: u8, // max 255
    health: i32,
    max_health: i32,
    strength: u8,
    defense: u8,
    agility: u8,
    left_hand: HandItem,
    right_hand: Option<HandItem>,
    helmet: Option<Armor>,
    breastplate: Option<Armor>,
    legs: Option<Armor>,
    money: i32,
}
