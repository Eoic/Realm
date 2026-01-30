use std::{collections::HashMap, path::Path, sync::OnceLock};

use serde::Deserialize;

use crate::items::inventory::{Item, ItemId, ItemState};

pub type ItemTemplateId = u64;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum ItemKindTemplate {
    Weapon { damage: u32 },
    Armor { defence: u32 },
    Potion { capacity: u32 },
    Currency,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemTemplate {
    pub id: ItemTemplateId,
    pub name: String,
    pub alias: String,
    pub description: String,
    pub weight: u64,
    pub stackable: bool,
    pub kind: ItemKindTemplate,
}

#[derive(Debug, Deserialize)]
struct ItemsConfig {
    items: Vec<ItemTemplate>,
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

    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: ItemsConfig = toml::from_str(&content)?;
        Ok(Self::new(config.items))
    }

    pub fn spawn_by_id(&self, template_id: ItemTemplateId, state: ItemState) -> Item {
        Item {
            id: ItemId::new(),
            template_id,
            state,
        }
    }

    pub fn spawn_by_alias(&self, template_alias: &str, state: ItemState) -> Option<Item> {
        self.templates
            .iter()
            .find(|(_, template)| template.alias == template_alias)
            .map(|(template_id, _)| self.spawn_by_id(*template_id, state))
    }

    pub fn template(&self, item: &Item) -> Option<&ItemTemplate> {
        self.templates.get(&item.template_id)
    }

    pub fn template_by_id(&self, id: ItemTemplateId) -> Option<&ItemTemplate> {
        self.templates.get(&id)
    }

    pub fn name(&self, item: &Item) -> Option<&str> {
        self.template(item).map(|t| t.name.as_str())
    }

    pub fn description(&self, item: &Item) -> Option<&str> {
        self.template(item).map(|t| t.description.as_str())
    }

    pub fn weight(&self, item: &Item) -> Option<u64> {
        self.template(item).map(|t| t.weight)
    }

    pub fn len(&self) -> usize {
        self.templates.len()
    }

    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
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

pub fn get_item_template(item: &Item) -> Option<&ItemTemplate> {
    get_database().template(item)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_templates() -> Vec<ItemTemplate> {
        vec![
            ItemTemplate {
                id: 1,
                name: "Test Sword".into(),
                description: "A test weapon.".into(),
                alias: "test sword".into(),
                weight: 5,
                kind: ItemKindTemplate::Weapon { damage: 10 },
                stackable: false,
            },
            ItemTemplate {
                id: 2,
                name: "Test Potion".into(),
                description: "A test potion.".into(),
                alias: "test potion".into(),
                weight: 1,
                kind: ItemKindTemplate::Potion { capacity: 50 },
                stackable: true,
            },
        ]
    }

    #[test]
    fn test_database_new() {
        let db = ItemsDatabase::new(create_templates());
        assert_eq!(db.len(), 2);
        assert!(!db.is_empty());
    }

    #[test]
    fn test_spawn_by_id() {
        let db = ItemsDatabase::new(create_templates());
        let item = db.spawn_by_id(1, ItemState::None);
        assert_eq!(item.template_id, 1);
    }

    #[test]
    fn test_spawn_by_alias() {
        let db = ItemsDatabase::new(create_templates());
        let item = db.spawn_by_alias("test potion", ItemState::Potion { fill: 50 });
        assert!(item.is_some());
        assert_eq!(item.unwrap().template_id, 2);
    }

    #[test]
    fn test_spawn_by_alias_not_found() {
        let db = ItemsDatabase::new(create_templates());
        let item = db.spawn_by_alias("nonexistent", ItemState::None);
        assert!(item.is_none());
    }

    #[test]
    fn test_template_lookup() {
        let db = ItemsDatabase::new(create_templates());
        let item = db.spawn_by_id(1, ItemState::None);
        let template = db.template(&item);
        assert!(template.is_some());
        assert_eq!(template.unwrap().name, "Test Sword");
    }

    #[test]
    fn test_name_and_description() {
        let db = ItemsDatabase::new(create_templates());
        let item = db.spawn_by_id(2, ItemState::None);
        assert_eq!(db.name(&item), Some("Test Potion"));
        assert_eq!(db.description(&item), Some("A test potion."));
    }
}
