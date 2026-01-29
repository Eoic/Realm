use realm::{
    Character, CharacterClass, ItemState, ItemsDatabase, asset_path, create_database, get_database,
};

use macroquad::prelude::*;

// fn main() {
//     let items_path = asset_path("config/items.toml");

//     let database = ItemsDatabase::load_from_file(&items_path)
//         .expect("Failed to load items database from assets.");

//     create_database(database);

//     let mut warrior = Character::new("Eoic".to_string(), CharacterClass::Warrior);
//     let mut mage = Character::new("Eoic".to_string(), CharacterClass::Mage);

//     warrior.record_exp(490);
//     warrior.change_health(10);
//     warrior.change_mana(1000);
//     println!("{}", warrior);

//     let item = get_database().spawn_by_alias("rusty sword", ItemState::Equipment { wear: 0 });
//     let potion = get_database().spawn_by_alias("health potion", ItemState::Potion { fill: 100 });

//     if let Some(item) = item {
//         mage.inventory.add(item.clone());
//         mage.inventory.add(item.clone());
//         mage.inventory.add(item.clone());
//     } else {
//         println!("Could not acquire item.");
//     }

//     if let Some(potion) = potion {
//         mage.inventory.add(potion.clone());
//         mage.inventory.add(potion.clone());
//     }

//     println!("{}", mage);
// }

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(RED);
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);
        next_frame().await
    }
}
