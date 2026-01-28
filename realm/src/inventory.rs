use std::{hash::Hash, ops::Index};
use uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct ItemId(uuid::Uuid);

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

struct Item {
    id: ItemId,
    name: String,
    description: String,
    weight: u64,
    kind: ItemKind,
}

impl Eq for Item {}

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

struct Inventory {
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

    pub fn add_item(&mut self, item: Item) -> bool {
        if self.items.len() >= self.capacity {
            return false;
        }

        self.items.push(item);
        true
    }

    pub fn drop_item(&mut self, item_id: ItemId) -> bool {
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

    pub fn display_items(&self) {
        for (index, item) in self.items.iter().enumerate() {
            println!("{}. {}", index + 1, item.name)
        }
    }

    pub fn total_weight(&self) {
        self.items.iter().map(|item| item.weight).sum::<u64>();
    }
}
