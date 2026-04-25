pub trait FlavorText {
    fn text(&self) -> &str;
}

pub enum Card<'a> {
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
pub enum Class {
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
pub enum MinionType {
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
pub enum SpellSchool {
    Arcane,
    Fel,
    Fire,
    Frost,
    Holy,
    Nature,
    Shadow,
    General,
}

impl<'a> FlavorText for Card<'a> {
    fn text(&self) -> &str {
        match self {
            Card::Spell { text, .. }
            | Card::Minion { text, .. }
            | Card::Location { text, .. }
            | Card::Hero { text, .. }
            | Card::Weapon { text, .. } => text,
        }
    }
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

pub fn printd(deck: &[Card]) {
    for card in deck {
        printc(card);
    }
}
