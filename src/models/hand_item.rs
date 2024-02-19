use serde::{Serialize, Deserialize};
use super::weapon::Weapon;
use super::shield::Shield;

#[derive(Serialize, Deserialize, Debug)]
pub enum HandItem {
    Weapon(Weapon),
    Shield(Shield),
    BareHand
}