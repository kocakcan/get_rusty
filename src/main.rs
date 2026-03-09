mod constants;
mod control_flow;
mod converter;
mod converter_boxed;
mod data_types;
mod document;
mod enum_practice;
mod enums;
mod example_program;
mod fibonacci;
mod fixing_ownership_errors;
mod functions;
mod if_let;
mod match_construct;
mod meme;
mod method_syntax;
mod methods_and_ownership;
mod ownership;
mod ownership_inventory;
mod ownership_recap;
mod packages_and_crates;
mod practice;
mod question;
mod quiz;
mod references_and_borrowing;
mod shadowing;
mod structs;
mod the_slice_type;
mod variables_and_mutability;

use colored::*;

enum Card {
    Spell {
        name: String,
        mana_cost: u8,
        spell_school: SpellSchool,
        class: Class,
        text: String,
    },
    Minion {
        name: String,
        mana_cost: u8,
        minion_type: MinionType,
        class: Class,
        text: String,
    },
    Location {
        name: String,
        mana_cost: u8,
        class: Class,
        text: String,
    },
    Weapon {
        name: String,
        mana_cost: u8,
        class: Class,
        text: String,
    },
    Hero {
        name: String,
        mana_cost: u8,
        class: Class,
        text: String,
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
            text,
        } => {
            println!(
                "Name: {} | Mana: {} | Spell school: {:?} | Class: {:?} | Text: {}",
                name, mana_cost, spell_school, class, text
            );
        }
        Card::Minion {
            name,
            mana_cost,
            minion_type,
            class,
            text,
        } => {
            println!(
                "Name: {} | Mana: {} | Minion type: {:?} | Class: {:?} | Text: {}",
                name, mana_cost, minion_type, class, text
            );
        }
        Card::Location {
            name,
            mana_cost,
            class,
            text,
        }
        | Card::Weapon {
            name,
            mana_cost,
            class,
            text,
        }
        | Card::Hero {
            name,
            mana_cost,
            class,
            text,
        } => {
            println!(
                "Name: {} | Mana: {} | Class: {:?} | Text: {}",
                name, mana_cost, class, text
            );
        }
    }
}

fn main() {
    printc(Card::Minion {
        name: String::from("Edwin van Cleef"),
        mana_cost: 3,
        minion_type: MinionType::Pirate,
        class: Class::Rogue,
        text: String::from("Combo: Gain +2/+2 for each other card you've played this turn."),
    });
    printc(Card::Hero {
        name: String::from("Deathwing, Worldbreaker"),
        mana_cost: 10,
        class: Class::Neutral,
        text: String::from("Battlecry: Choose 1 Cataclysm to unleash! Herald twice to upgrade."),
    });
    printc(Card::Weapon {
        name: String::from("Kingsbane"),
        mana_cost: 1,
        class: Class::Rogue,
        text: String::from("Always keeps enhancements. Deathrattle: Shuffle this into your deck."),
    });
    printc(Card::Location {
        name: String::from("Amirdrassil"),
        mana_cost: 5,
        class: Class::Druid,
        text: String::from(
            "Summon a 1-Cost minion. Gain 1 Armor. Draw 1 card. Refresh 1 Mana Crystal. (Improves each use!)",
        ),
    });
    printc(Card::Spell {
        name: String::from("Preparation"),
        mana_cost: 0,
        spell_school: SpellSchool::Shadow,
        class: Class::Rogue,
        text: String::from("The next spell you cast this turn costs (2) less."),
    });

    let bold_text = format!(
        "{} {}",
        "Combo:".bold(),
        "Gain +2/+2 for each other card you've played this turn."
    );
    println!("{}", bold_text);
}
