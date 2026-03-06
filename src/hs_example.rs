enum Card {
    Spell {
        name: String,
        mana_cost: u8,
        spell_school: u8,
        class: String,
    },
    Minion {
        name: String,
        mana_cost: u8,
        minion_type: String,
        class: String,
    },
    Location {
        name: String,
        mana_cost: u8,
        class: String,
    },
    Weapon {
        name: String,
        mana_cost: u8,
        class: String,
    },
    Hero {
        name: String,
        mana_cost: u8,
        class: String,
    },
}

fn printc(card: Card) {
    match card {
        Card::Spell {
            name,
            mana_cost,
            spell_school,
            class,
        } => {
            println!(
                "Name: {} | Mana: {} | Spell school: {} | Class: {}",
                name, mana_cost, spell_school, class
            );
        }
        Card::Minion {
            name,
            mana_cost,
            minion_type,
            class,
        } => {
            println!(
                "Name: {} | Mana: {} | Minion type: {} | Class: {}",
                name, mana_cost, minion_type, class
            );
        }
        Card::Location {
            name,
            mana_cost,
            class,
        }
        | Card::Weapon {
            name,
            mana_cost,
            class,
        }
        | Card::Hero {
            name,
            mana_cost,
            class,
        } => {
            println!("Name: {} | Mana: {} | Class: {}", name, mana_cost, class);
        }
    }
}

// TODO: Spell school, minion type, and class should be enum
fn main() {
    printc(Card::Minion {
        name: String::from("Edwin van Cleef"),
        mana_cost: 3,
        minion_type: String::from("Pirate"),
        class: String::from("Rogue"),
    });

    printc(Card::Hero {
        name: String::from("Deathwing, Worldbreaker"),
        mana_cost: 10,
        class: String::from("Neutral"),
    });
    printc(Card::Weapon {
        name: String::from("Kingsbane"),
        mana_cost: 1,
        class: String::from("Rogue"),
    });
    printc(Card::Location {
        name: String::from("Amirdrassil"),
        mana_cost: 5,
        class: String::from("Druid"),
    });
}
