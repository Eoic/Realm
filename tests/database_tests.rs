use realm::{ItemsDatabase, asset_path};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_load_database_from_toml() {
    let toml_content = r#"
[[items]]
id = 100
name = "Test Item"
alias = "test item"
description = "A test item for integration testing."
weight = 10
stackable = true

[items.kind]
type = "Weapon"
damage = 25
"#;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");

    temp_file
        .write_all(toml_content.as_bytes())
        .expect("Failed to write temp file");

    let db = ItemsDatabase::load_from_file(temp_file.path()).expect("Failed to load database");
    assert_eq!(db.len(), 1);

    let template = db.template_by_id(100).expect("Template not found");
    assert_eq!(template.name, "Test Item");
    assert_eq!(template.weight, 10);
}

#[test]
fn test_load_multiple_items_from_toml() {
    let toml_content = r#"
[[items]]
id = 1
name = "Sword"
alias = "sword"
description = "A sword."
weight = 5
stackable = false

[items.kind]
type = "Weapon"
damage = 15

[[items]]
id = 2
name = "Shield"
alias = "shield"
description = "A shield."
weight = 8
stackable = false

[items.kind]
type = "Armor"
defence = 10

[[items]]
id = 3
name = "Mana Potion"
alias = "mana potion"
description = "Restores mana."
weight = 1
stackable = true

[items.kind]
type = "Potion"
capacity = 50

[[items]]
id = 4
name = "Coins"
alias = "coins"
description = "Currency."
weight = 0
stackable = true

[items.kind]
type = "Currency"
"#;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");

    temp_file
        .write_all(toml_content.as_bytes())
        .expect("Failed to write temp file");

    let db = ItemsDatabase::load_from_file(temp_file.path()).expect("Failed to load database");

    assert_eq!(db.len(), 4);
    assert!(db.template_by_id(1).is_some());
    assert!(db.template_by_id(2).is_some());
    assert!(db.template_by_id(3).is_some());
    assert!(db.template_by_id(4).is_some());
}

#[test]
fn test_assets_path_exists() {
    let assets = asset_path("config/items.toml");
    assert!(assets.exists(), "Assets file should exist at {:?}", assets);
}
