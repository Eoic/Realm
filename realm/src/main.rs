use crate::character::{Character, CharacterClass};

mod character;
mod inventory;

fn main() {
    let mut character = Character::new("Eoic".to_string(), CharacterClass::Warrior);
    character.record_exp(490);
    println!("{}", character);
}
