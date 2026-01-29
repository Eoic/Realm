mod character;
mod items;

use crate::items::{
    database::{ItemKindTemplate, ItemTemplate, ItemsDatabase, create_database, get_database},
    inventory::ItemState,
};

use crate::character::{Character, CharacterClass};

fn main() {
    let mut warrior = Character::new("Eoic".to_string(), CharacterClass::Warrior);
    let mut mage = Character::new("Eoic".to_string(), CharacterClass::Mage);
    warrior.record_exp(490);
    warrior.change_health(10);
    warrior.change_mana(1000);
    println!("{}", warrior);

    create_database(ItemsDatabase::new(vec![
        ItemTemplate {
            id: 1,
            name: "Rusty Sword".into(),
            description: "Barely sharp.".into(),
            alias: "rusty sword".into(),
            weight: 5,
            kind: ItemKindTemplate::Weapon { damage: 12 },
            stackable: true,
        },
        ItemTemplate {
            id: 2,
            name: "Health Potion".into(),
            description: "Restores HP.".into(),
            alias: "health potion".into(),
            weight: 1,
            kind: ItemKindTemplate::Potion { capacity: 100 },
            stackable: true,
        },
        ItemTemplate {
            id: 3,
            name: "Gold".into(),
            description: "Coins.".into(),
            alias: "gold".into(),
            weight: 0,
            kind: ItemKindTemplate::Currency,
            stackable: true,
        },
    ]));

    let item = get_database().spawn_by_id(1, ItemState::None);
    mage.inventory.add(item.clone());
    mage.inventory.add(item.clone());
    mage.inventory.add(item.clone());

    println!("{}", mage);
}
