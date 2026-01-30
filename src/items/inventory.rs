use crate::items::database::{ItemTemplateId, get_database, get_item_template};
use std::error::Error;
use std::rc::Rc;
use std::{collections::HashMap, hash::Hash};
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
    Equipment { wear: u8 },
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
    items: Vec<Rc<Item>>,
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

        self.items.push(Rc::new(item));
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
            .position(|target: &Rc<Item>| target.id == item_id);

        if let Some(index) = needle {
            self.items.remove(index);
            return true;
        }

        false
    }

    pub fn find(&self, name: &str) -> Result<Rc<Item>, InventoryError> {
        let target = self
            .items
            .iter()
            .find(|&item| get_item_template(&item).unwrap().name == name);

        match target {
            Some(target) => Ok(Rc::clone(&target)),
            None => Err(InventoryError::ItemNotFound),
        }
    }

    pub fn display(&self) {
        println!("|-{:-<80}-|", "");

        if self.items.len() == 0 {
            println!("| {:<80} |", "Inventory is empty.");
            println!("|-{:-<80}-|", "");
            return;
        }

        let mut stacked: HashMap<ItemTemplateId, u64> = HashMap::new();

        for item in self.items.iter() {
            if get_database().template(&item).unwrap().stackable {
                stacked
                    .entry(item.template_id)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        let sortable: Vec<(&Rc<Item>, Option<u64>)> = self
            .items
            .iter()
            .flat_map(|item| {
                if stacked.contains_key(&item.template_id) {
                    vec![(item, stacked.remove(&item.template_id))]
                } else {
                    if !get_database().template(item).unwrap().stackable {
                        vec![(item, None)]
                    } else {
                        vec![]
                    }
                }
            })
            .collect();

        for (index, (item, count)) in sortable.iter().enumerate() {
            let name = get_database()
                .template(item)
                .map(|template| template.name.as_str())
                .unwrap_or("Unknown");

            let line = match count {
                Some(value) => format!("{} ({})", name, value),
                None => format!("{}", name),
            };

            println!("| {:<80} |", format!("{}. {}", index + 1, line))
        }

        println!("|-{:-<80}-|", "");
    }

    pub fn weight(&self) -> u64 {
        self.items
            .iter()
            .filter_map(|item| get_database().template(item).map(|t| t.weight))
            .sum()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_full(&self) -> bool {
        self.items.len() >= self.capacity
    }
}

pub enum InventoryError {
    ItemNotFound,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::items::database::ItemTemplateId;

    fn create_test_item(template_id: ItemTemplateId) -> Item {
        Item::new(ItemId::new(), template_id, ItemState::None)
    }

    #[test]
    fn test_inventory_new() {
        let inv = Inventory::new(10);
        assert_eq!(inv.capacity(), 10);
        assert!(inv.is_empty());
        assert_eq!(inv.len(), 0);
    }

    #[test]
    fn test_inventory_add() {
        let mut inv = Inventory::new(5);
        let item = create_test_item(1);
        assert!(inv.add(item));
        assert_eq!(inv.len(), 1);
        assert!(!inv.is_empty());
    }

    #[test]
    fn test_inventory_full() {
        let mut inv = Inventory::new(2);
        assert!(inv.add(create_test_item(1)));
        assert!(inv.add(create_test_item(2)));
        assert!(inv.is_full());
        assert!(!inv.add(create_test_item(3)));
        assert_eq!(inv.len(), 2);
    }

    #[test]
    fn test_inventory_drop() {
        let mut inv = Inventory::new(5);
        let item = create_test_item(1);
        let item_id = item.id;
        inv.add(item);
        assert_eq!(inv.len(), 1);
        assert!(inv.drop(item_id));
        assert!(inv.is_empty());
    }

    #[test]
    fn test_inventory_drop_nonexistent() {
        let mut inv = Inventory::new(5);
        let fake_id = ItemId::new();
        assert!(!inv.drop(fake_id));
    }

    #[test]
    fn test_item_equality() {
        let id = ItemId::new();
        let item1 = Item::new(id, 1, ItemState::None);
        let item2 = Item::new(id, 1, ItemState::None);
        let item3 = Item::new(ItemId::new(), 1, ItemState::None);

        assert_eq!(item1, item2);
        assert_ne!(item1, item3);
    }

    #[test]
    fn test_item_id_unique() {
        let id1 = ItemId::new();
        let id2 = ItemId::new();
        assert_ne!(id1, id2);
    }
}
