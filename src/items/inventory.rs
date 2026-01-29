use crate::items::database::{ItemTemplateId, get_database};
use std::hash::Hash;
use uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ItemId(uuid::Uuid);

impl ItemId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[derive(Debug, Clone)]
struct Weapon {
    damage: u32,
}

#[derive(Debug, Clone)]
struct Armor {
    defence: u32,
}

#[derive(Debug, Clone)]
struct Potion {
    capacity: u32,
    fill_amount: u32,
}

#[derive(Debug, Clone)]
struct Currency {
    count: u64,
}

enum ItemKind {
    Weapon(Weapon),
    Armor(Armor),
    Potion(Potion),
    Currency(Currency),
}

#[derive(Debug, Clone)]
pub enum ItemState {
    None,
    Potion { fill: u32 },
    Currency { count: u64 },
}

#[derive(Debug, Clone)]
pub struct Item {
    pub id: ItemId,
    pub template_id: ItemTemplateId,
    pub state: ItemState,
}

impl Eq for Item {}

impl Item {
    pub fn new(id: ItemId, template_id: ItemTemplateId, state: ItemState) -> Self {
        Self {
            id,
            template_id,
            state,
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Inventory {
    capacity: usize,
    items: Vec<Item>,
}

impl Inventory {
    pub fn new(capacity: usize) -> Inventory {
        Inventory {
            capacity,
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, item: Item) -> bool {
        if self.items.len() >= self.capacity {
            return false;
        }

        self.items.push(item);
        true
    }

    fn count_stack(&self, item: &Item) -> u64 {
        self.items
            .iter()
            .map(|item| item.template_id)
            .filter(|template_id| *template_id == item.template_id)
            .sum::<u64>()
    }

    pub fn drop(&mut self, item_id: ItemId) -> bool {
        let needle = self
            .items
            .iter()
            .position(|target: &Item| target.id == item_id);

        if let Some(index) = needle {
            self.items.remove(index);
            return true;
        }

        false
    }

    pub fn display(&self) {
        println!("|-{:-<80}-|", "");

        if self.items.len() == 0 {
            println!("| {:<80} |", "Inventory is empty.");
            println!("|-{:-<80}-|", "");
            return;
        }

        for (index, item) in self.items.iter().enumerate() {
            println!(
                "| {:<80} |",
                format!(
                    "{}. {} ({})",
                    index + 1,
                    get_database().template(&item).name,
                    self.count_stack(&item),
                )
            )
        }

        println!("|-{:-<80}-|", "");
    }

    pub fn weight(&self) {
        self.items
            .iter()
            .map(|item| get_database().template(&item).weight)
            .sum::<u64>();
    }
}
