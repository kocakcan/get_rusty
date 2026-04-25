<<<<<<< HEAD
pub trait FlavorText {
    fn text(&self) -> &str;
}

#[derive(Debug)]
enum Card<'a> {
    Spell {
        name: &'a str,
        mana_cost: u8,
        spell_school: SpellSchool,
        class: Class,
        text: &'a str,
    },
    Minion {
        name: &'a str,
        mana_cost: u8,
        minion_type: MinionType,
        class: Class,
        text: &'a str,
    },
    Location {
        name: &'a str,
        mana_cost: u8,
        class: Class,
        text: &'a str,
    },
    Weapon {
        name: &'a str,
        mana_cost: u8,
        class: Class,
        text: &'a str,
    },
    Hero {
        name: &'a str,
        mana_cost: u8,
        class: Class,
        text: &'a str,
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

fn printc(card: &Card) {
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

fn printd(deck: &[Card]) {
    for card in deck {
        printc(card);
    }
}
=======
use get_rusty::{Card, Class, MinionType, SpellSchool};
>>>>>>> fb56868 (separated the concerns)

impl<'a> FlavorText for Card<'a> {
    fn text(&self) -> &str {
        match self {
            Card::Spell { text, .. } |
            Card::Minion { text, .. } |
            Card::Location { text, .. } |
            Card::Hero { text, .. } |
            Card::Weapon { text, .. } => text
        }
    }
}

fn main() {
    let custom_deck: Vec<Card> = vec![
        Card::Minion {
            name: "Edwin van Cleef",
            mana_cost: 3,
            minion_type: MinionType::Pirate,
            class: Class::Rogue,
            text: "Combo: Gain +2/+2 for each other card you've played this turn.",
        },
        Card::Hero {
            name: "Deathwing, Worldbreaker",
            mana_cost: 10,
            class: Class::Neutral,
            text: "Battlecry: Choose 1 Cataclysm to unleash! Herald twice to upgrade.",
        },
        Card::Weapon {
            name: "Kingsbane",
            mana_cost: 1,
            class: Class::Rogue,
            text: "Always keeps enhancements. Deathrattle: Shuffle this into your deck.",
        },
        Card::Location {
            name: "Amirdrassil",
            mana_cost: 5,
            class: Class::Druid,
            text: "Summon a 1-Cost minion. Gain 1 Armor. Draw 1 card. Refresh 1 Mana Crystal. (Improves each use!)",
        },
        Card::Spell {
            name: "Preparation",
            mana_cost: 0,
            spell_school: SpellSchool::Shadow,
            class: Class::Rogue,
            text: "The next spell you cast this turn costs (2) less.",
        },
    ];

<<<<<<< HEAD
    for card in &custom_deck {
        println!("{}", card.text());
    }
=======
    get_rusty::printd(&custom_deck);
    println!("There are {} cards in the deck", custom_deck.len());
>>>>>>> fb56868 (separated the concerns)
}
