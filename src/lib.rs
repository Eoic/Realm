pub mod character;
pub mod items;

use std::path::PathBuf;

pub use character::{Character, CharacterClass};
pub use items::database::{
    ItemKindTemplate, ItemTemplate, ItemTemplateId, ItemsDatabase, create_database, get_database,
};
pub use items::inventory::{Inventory, Item, ItemId, ItemState};

pub fn assets_dir() -> PathBuf {
    if cfg!(debug_assertions) {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")
    } else {
        std::env::current_exe()
            .expect("Failed to get executable path")
            .parent()
            .expect("Failed to get executable directory")
            .join("assets")
    }
}

pub fn asset_path(relative_path: &str) -> PathBuf {
    assets_dir().join(relative_path)
}
