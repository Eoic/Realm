use crate::Item;
use crate::items::database::get_item_template;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

struct Stats {
    attack: u32,
    defence: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            attack: 0,
            defence: 0,
        }
    }
}

pub struct Equipment {
    stats: Stats,
    slots: HashMap<String, Option<Rc<Item>>>,
}

impl Equipment {
    pub fn new() -> Self {
        let mut slots: HashMap<String, Option<Rc<Item>>> = HashMap::new();
        slots.insert("left hand".to_string(), None);
        slots.insert("right hand".to_string(), None);

        Self {
            stats: Stats::default(),
            slots,
        }
    }

    pub fn equip(
        &mut self,
        item: Rc<Item>,
        target_slot: &str,
    ) -> Result<Option<Rc<Item>>, EquipError> {
        match self.slots.get_mut(target_slot) {
            Some(slot) => {
                println!("Equipped {:?}", item.id);
                Ok(slot.replace(item))
            }
            None => Err(EquipError::InvalidSlot),
        }
    }

    pub fn unequip(&mut self, target_slot: &str) -> Option<Rc<Item>> {
        self.slots.get_mut(target_slot)?.take()
    }

    pub fn display(&self) {
        println!("EQUIPMENT");

        self.slots.iter().for_each(|entry| {
            println!("{:?}", entry);
        });
    }
}

pub enum EquipError {
    InvalidSlot,
}
