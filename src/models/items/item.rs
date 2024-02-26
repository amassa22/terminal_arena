use super::{armor::Armor, shield::Shield, weapon::Weapon};


pub enum Item {
    Weapon(Weapon),
    Shield(Shield),
    Armor(Armor),
}