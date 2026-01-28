use std::cmp::{max, min};
use std::fmt::{Display, Formatter};

use crate::items::inventory::{self, Inventory};

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
