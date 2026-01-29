use std::{collections::HashMap, sync::OnceLock};

use crate::items::inventory::{Item, ItemId, ItemState};

pub type ItemTemplateId = u64;

#[derive(Debug, Clone)]
pub enum ItemKindTemplate {
    Weapon { damage: u32 },
    Armor { defence: u32 },
    Potion { capacity: u32 },
    Currency,
}

#[derive(Debug, Clone)]
pub struct ItemTemplate {
    pub id: ItemTemplateId,
    pub name: String,
    pub alias: String,
    pub description: String,
    pub weight: u64,
    pub stackable: bool,
    pub kind: ItemKindTemplate,
}

#[derive(Debug)]
pub struct ItemsDatabase {
    templates: HashMap<ItemTemplateId, ItemTemplate>,
}

impl ItemsDatabase {
    pub fn new(templates: Vec<ItemTemplate>) -> Self {
        Self {
            templates: templates.into_iter().map(|t| (t.id, t)).collect(),
        }
    }

    pub fn spawn_by_id(&self, template_id: ItemTemplateId, state: ItemState) -> Item {
        Item {
            id: ItemId::new(),
            template_id,
            state,
        }
    }

    pub fn spawn_by_alias(&self, template_alias: String, state: ItemState) -> Item {
        if let Some((template_id, _)) = self
            .templates
            .iter()
            .find(|(_, template)| *template.alias == template_alias)
        {
            self.spawn_by_id(*template_id, state)
        } else {
            panic!("Could not find a valid item template.");
        }
    }

    pub fn template(&self, item: &Item) -> &ItemTemplate {
        &self.templates[&item.template_id]
    }

    pub fn name(&self, item: &Item) -> &str {
        &self.template(&item).name
    }

    pub fn description(&self, item: &Item) -> &str {
        &self.template(&item).description
    }

    pub fn weight(&self, item: &Item) -> u64 {
        self.template(&item).weight
    }
}

static DATABASE: OnceLock<ItemsDatabase> = OnceLock::new();

pub fn create_database(database: ItemsDatabase) {
    DATABASE
        .set(database)
        .expect("Items database is already initialized.");
}

pub fn get_database() -> &'static ItemsDatabase {
    DATABASE.get().expect("Database is not initialized yet.")
}
