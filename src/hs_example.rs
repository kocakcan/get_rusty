enum Card {
    Spell {
        name: String,
        mana_cost: u8,
        spell_school: SpellSchool,
        class: Class,
    },
    Minion {
        name: String,
        mana_cost: u8,
        minion_type: MinionType,
        class: Class,
    },
    Location {
        name: String,
        mana_cost: u8,
        class: Class,
    },
    Weapon {
        name: String,
        mana_cost: u8,
        class: Class,
    },
    Hero {
        name: String,
        mana_cost: u8,
        class: Class,
    },
}

#[derive(Debug)]
enum Class {
    DeathKnight,
    DemonHunter,
    Druid,
    Hunter,
    Mage,
    Paladin,
    Priest,
    Rogue,
    Shaman,
    Warlock,
    Warrior,
    Neutral,
}

#[derive(Debug)]
enum MinionType {
    Beast,
    Demon,
    Draenei,
    Dragon,
    Elemental,
    Mech,
    Murloc,
    Naga,
    Pirate,
    Quilboar,
    Totem,
    Undead,
}

#[derive(Debug)]
enum SpellSchool {
    Arcane,
    Fel,
    Fire,
    Frost,
    Holy,
    Nature,
    Shadow,
    General,
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
                "Name: {} | Mana: {} | Spell school: {:?} | Class: {:?}",
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
                "Name: {} | Mana: {} | Minion type: {:?} | Class: {:?}",
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
            println!("Name: {} | Mana: {} | Class: {:?}", name, mana_cost, class);
        }
    }
}

fn main() {
    printc(Card::Minion {
        name: String::from("Edwin van Cleef"),
        mana_cost: 3,
        minion_type: MinionType::Pirate,
        class: Class::Rogue,
    });

    printc(Card::Hero {
        name: String::from("Deathwing, Worldbreaker"),
        mana_cost: 10,
        class: Class::Neutral,
    });
    printc(Card::Weapon {
        name: String::from("Kingsbane"),
        mana_cost: 1,
        class: Class::Rogue,
    });
    printc(Card::Location {
        name: String::from("Amirdrassil"),
        mana_cost: 5,
        class: Class::Druid,
    });
}
