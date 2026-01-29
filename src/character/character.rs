use std::fmt::{Display, Formatter};

use crate::items::inventory::Inventory;

const A: u32 = 20;
const B: u32 = 70;
const C: u32 = 100;

pub enum CharacterClass {
    Warrior,
    Mage,
}

pub struct Character {
    name: String,
    health: u32,
    max_health: u32,
    mana: u32,
    max_mana: u32,
    level: u32,
    experience: u32,
    pub inventory: Inventory,
    class: CharacterClass,
}

impl Character {
    pub fn new(name: String, class: CharacterClass) -> Self {
        match class {
            CharacterClass::Warrior => Self::warrior(name),
            CharacterClass::Mage => Self::mage(name),
        }
    }

    fn warrior(name: String) -> Self {
        Character {
            name,
            health: 120,
            max_health: 120,
            class: CharacterClass::Warrior,
            ..Default::default()
        }
    }

    fn mage(name: String) -> Self {
        Character {
            name,
            mana: 120,
            max_mana: 120,
            class: CharacterClass::Mage,
            ..Default::default()
        }
    }

    fn required_exp(self: &Self) -> u64 {
        (A * self.level * self.level + B * self.level + C) as u64
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn change_health(&mut self, amount: u32) {
        if amount > self.health {
            self.health = 0;
            return;
        }

        self.health -= amount;
    }

    pub fn change_mana(&mut self, amount: u32) {
        if amount >= self.mana {
            self.mana = 0;
            return;
        }

        self.mana -= amount;
    }

    pub fn record_exp(&mut self, amount: u32) {
        self.experience += amount;
        let a = A as i128;
        let b = B as i128;
        let c = C as i128;
        let xp = self.experience as i128;

        if amount <= 0 {
            return;
        }

        let d_i128 = b * b + 4 * a * (xp - c);
        let d = d_i128 as f64;
        let root = d.sqrt();
        let a2 = (2 * a) as f64;
        let approx = ((-(b as f64)) + root) / a2;

        self.level = approx.floor() as u32 + 1;
    }
}

impl Display for Character {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "|-{:-<80}-|\n", "")?;

        write!(
            formatter,
            "| {:^80} |\n",
            format!("{} the {}", self.name, self.class)
        )?;

        write!(formatter, "|-{:-<80}-|\n", "")?;

        write!(
            formatter,
            "| {:<12}{:<68} |\n",
            "Level:",
            format!("{}", self.level)
        )?;

        write!(
            formatter,
            "| {:<12}{:<68} |\n",
            "Experience",
            format!("{} / {}", self.experience, self.required_exp())
        )?;

        write!(
            formatter,
            "| {:<12}{:<68} |\n",
            "Health",
            format!("{} / {}", self.health, self.max_health)
        )?;

        write!(
            formatter,
            "| {:<12}{:<68} |\n",
            "Mana",
            format!("{} / {}", self.mana, self.max_mana)
        )?;

        self.inventory.display();

        write!(formatter, "|-{:-<80}-|\n", "")
    }
}

impl Default for Character {
    fn default() -> Self {
        Character {
            name: "Default".to_string(),
            level: 1,
            experience: 0,
            health: 100,
            max_health: 100,
            mana: 100,
            max_mana: 100,
            inventory: Inventory::new(20),
            class: CharacterClass::Warrior,
        }
    }
}

impl Display for CharacterClass {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Warrior => write!(formatter, "Warrior"),
            Self::Mage => write!(formatter, "Mage"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warrior_creation() {
        let warrior = Character::new("TestWarrior".to_string(), CharacterClass::Warrior);
        assert_eq!(warrior.health, 120);
        assert_eq!(warrior.max_health, 120);
        assert_eq!(warrior.level, 1);
    }

    #[test]
    fn test_mage_creation() {
        let mage = Character::new("TestMage".to_string(), CharacterClass::Mage);
        assert_eq!(mage.mana, 120);
        assert_eq!(mage.max_mana, 120);
        assert_eq!(mage.level, 1);
    }

    #[test]
    fn test_change_health() {
        let mut character = Character::new("Test".to_string(), CharacterClass::Warrior);
        let initial = character.health;
        character.change_health(10);
        assert_eq!(character.health, initial - 10);
    }

    #[test]
    fn test_change_health_overflow() {
        let mut character = Character::new("Test".to_string(), CharacterClass::Warrior);
        character.change_health(1000); // More than max health
        assert_eq!(character.health, 0);
    }

    #[test]
    fn test_change_mana() {
        let mut character = Character::new("Test".to_string(), CharacterClass::Mage);
        let initial = character.mana;
        character.change_mana(10);
        assert_eq!(character.mana, initial - 10);
    }

    #[test]
    fn test_change_mana_overflow() {
        let mut character = Character::new("Test".to_string(), CharacterClass::Mage);
        character.change_mana(1000); // More than max mana
        assert_eq!(character.mana, 0);
    }

    #[test]
    fn test_record_exp_level_up() {
        let mut character = Character::new("Test".to_string(), CharacterClass::Warrior);
        assert_eq!(character.level, 1);

        // Level 2 requires: 20*1*1 + 70*1 + 100 = 190 XP
        character.record_exp(190);
        assert_eq!(character.level, 2);
    }

    #[test]
    fn test_record_exp_multiple_levels() {
        let mut character = Character::new("Test".to_string(), CharacterClass::Warrior);

        // Give enough XP to reach level 5
        character.record_exp(2000);
        assert!(character.level >= 5);
    }

    #[test]
    fn test_default_character() {
        let character = Character::default();
        assert_eq!(character.name, "Default");
        assert_eq!(character.level, 1);
        assert_eq!(character.health, 100);
        assert_eq!(character.mana, 100);
    }

    #[test]
    fn test_character_class_display() {
        assert_eq!(format!("{}", CharacterClass::Warrior), "Warrior");
        assert_eq!(format!("{}", CharacterClass::Mage), "Mage");
    }
}
