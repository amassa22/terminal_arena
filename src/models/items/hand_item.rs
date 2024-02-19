use super::shield::Shield;
use super::weapon::Weapon;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum HandItem {
    Weapon(Weapon),
    Shield(Shield),
    BareHand,
}
