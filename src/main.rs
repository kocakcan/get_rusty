use get_rusty::FlavorText;
use get_rusty::{Card, Class, MinionType, SpellSchool};

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

    for card in &custom_deck {
        println!("{}", card.text());
    }
}
