use crate::character::{Character, CharacterClass};

mod character;
mod inventory;

fn main() {
    let mut warrior = Character::new("Eoic".to_string(), CharacterClass::Warrior);
    let mage = Character::new("Eoic".to_string(), CharacterClass::Mage);
    warrior.record_exp(490);
    warrior.change_health(10);
    warrior.change_mana(1000);
    println!("{}", warrior);
    println!("{}", mage);
}
