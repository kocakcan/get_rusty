enum Card {
    Spell {
        mana_cost: u8,
        spell_school: u8,
        class: String,
    },
    Minion {
        mana_cost: u8,
        minion_type: String,
        class: String,
    },
    Location {
        mana_cost: u8,
        class: String,
    },
    Weapon {
        mana_cost: u8,
        class: String,
    },
    Hero {
        mana_cost: u8,
        class: String,
    },
}

fn printc(card: Card) {
    match card {
        Card::Spell {
            mana_cost: mana_cost,
            spell_school: spell_school,
            class: class,
        } => {
            println!(
                "Mana: {} | Spell school: {} | Class: {}",
                mana_cost, spell_school, class
            );
        }
        Card::Minion {
            mana_cost: mana_cost,
            minion_type: minion_type,
            class: class,
        } => {
            println!(
                "Mana: {} | Minion type: {} | Class: {}",
                mana_cost, minion_type, class
            );
        }
        Card::Location {
            mana_cost: mana_cost,
            class: class,
        }
        | Card::Weapon {
            mana_cost: mana_cost,
            class: class,
        }
        | Card::Hero {
            mana_cost: mana_cost,
            class: class,
        } => {
            println!("Mana: {} | Class: {}", mana_cost, class);
        }
    }
}

fn main() {
    printc(Card::Minion {
        mana_cost: 3,
        minion_type: String::from("Pirate"),
        class: String::from("Rogue"),
    });
}
