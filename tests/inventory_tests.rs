use realm::{Inventory, Item, ItemId, ItemState};

#[test]
fn test_inventory_workflow() {
    let mut inventory = Inventory::new(10);

    for i in 1..=5 {
        let item = Item::new(ItemId::new(), i, ItemState::None);
        assert!(inventory.add(item));
    }

    assert_eq!(inventory.len(), 5);
    assert!(!inventory.is_full());
}

#[test]
fn test_inventory_capacity_limit() {
    let capacity = 3;
    let mut inventory = Inventory::new(capacity);

    for _ in 0..capacity {
        let item = Item::new(ItemId::new(), 1, ItemState::None);
        assert!(inventory.add(item));
    }

    assert!(inventory.is_full());

    let extra_item = Item::new(ItemId::new(), 1, ItemState::None);
    assert!(!inventory.add(extra_item));
}

#[test]
fn test_inventory_drop_and_add() {
    let mut inventory = Inventory::new(2);
    let item1 = Item::new(ItemId::new(), 1, ItemState::None);
    let item1_id = item1.id;
    let item2 = Item::new(ItemId::new(), 2, ItemState::None);

    inventory.add(item1);
    inventory.add(item2);
    assert!(inventory.is_full());
    assert!(inventory.drop(item1_id));
    assert!(!inventory.is_full());

    let item3 = Item::new(ItemId::new(), 3, ItemState::None);
    assert!(inventory.add(item3));
}

#[test]
fn test_potion_item_state() {
    let potion = Item::new(ItemId::new(), 1, ItemState::Potion { fill: 100 });

    match potion.state {
        ItemState::Potion { fill } => assert_eq!(fill, 100),
        _ => panic!("Expected Potion state"),
    }
}

#[test]
fn test_currency_item_state() {
    let gold = Item::new(ItemId::new(), 1, ItemState::Currency { count: 500 });

    match gold.state {
        ItemState::Currency { count } => assert_eq!(count, 500),
        _ => panic!("Expected Currency state"),
    }
}
