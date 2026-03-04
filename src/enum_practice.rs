enum Character {
    Ironclad { hp: u8, gold: u8, relic: Relic },
    Silent { hp: u8, gold: u8, relic: Relic },
    Defect { hp: u8, gold: u8, relic: Relic },
    Watcher { hp: u8, gold: u8, relic: Relic },
}

#[derive(Debug)]
enum Relic {
    BurningBlood,
    RingoftheSnake,
    CrackedCore,
    PureWater,
}

fn printc(character: Character) {
    match character {
        Character::Ironclad { hp, gold, relic }
        | Character::Silent { hp, gold, relic }
        | Character::Defect { hp, gold, relic }
        | Character::Watcher { hp, gold, relic } => {
            println!("HP: {} | Gold: {} | Relic: {:?}", hp, gold, relic);
        }
    }
}

fn main() {
    printc(Character::Ironclad {
        hp: 80,
        gold: 99,
        relic: Relic::BurningBlood,
    });
}
