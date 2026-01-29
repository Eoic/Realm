use realm::{Character, CharacterClass};

#[test]
fn test_warrior_progression() {
    let mut warrior = Character::new("TestWarrior".to_string(), CharacterClass::Warrior);

    warrior.change_health(50);
    warrior.record_exp(500);
    assert!(warrior.level() > 1);
}

#[test]
fn test_mage_progression() {
    let mut mage = Character::new("TestMage".to_string(), CharacterClass::Mage);

    mage.change_mana(30);
    mage.record_exp(500);
    assert!(mage.level() > 1);
}
