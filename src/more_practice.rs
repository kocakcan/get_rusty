struct Character {
    name: String,
    hp: u32,
    starting_gold: u32,
}

fn status_owned(c: Character) -> String {
    match c {
        Character { hp: 0, .. } => "dead".into(),
        Character { hp, name, .. } if hp < 20 => format!("{} is critical", name),
        Character { name, hp, .. } => format!("{} has {} hp", name, hp),
    }
}

fn status_ref(c: &Character) -> String {
    match c {
        Character { hp: 0, .. } => "dead".into(),
        Character { hp, name, .. } if *hp < 20 => format!("{} is critical", name),
        Character { name, hp, .. } => format!("{} has {} hp", name, hp),
    }
}

fn hp_band(hp: u32) -> &'static str {
    match hp {
        0 => "dead",
        1..=20 => "critical",
        21..=50 => "wounded",
        _ => "healthy",
    }
}

fn find_card<'a>(deck: &'a [&str], name: &str) -> Option<&'a str> {
    deck.iter().find(|&&c| c == name).copied()
}

fn option_demo() {
    let deck = ["Strike", "Defend", "Bash"];

    match find_card(&deck, "Bash") {
        Some(card) => println!("match: found {}", card),
        None => println!("match: not found"),
    }

    // if let
    if let Some(card) = find_card(&deck, "Strike") {
        println!("if let: drew {}", card);
    }

    // unwrap_or - default value if None
    let fallback = find_card(&deck, "Anger").unwrap_or("Nothing");
    println!("unwrap_or: {}", fallback);

    // map - transform Some(x) -> Some(f(x)), leaves None alone
    let shouted = find_card(&deck, "Bash").map(|c| c.to_uppercase());
    println!("map: {:?}", shouted);

    // and_then - chain Option-returning calls (flat map)
    let first_char_of_bash = find_card(&deck, "Bash").and_then(|c| c.chars().next());
    println!("and_then: {:?}", first_char_of_bash);

    // ? operator - early-return None if missing (function must return Option)
    fn first_letter(deck: &[&str], name: &str) -> Option<char> {
        let card = find_card(deck, name)?;
        card.chars().next()
    }
    println!("?: {:?}", first_letter(&deck, "Defend"));
    println!("?: {:?}", first_letter(&deck, "Anger")); // None, short-circuited
}

#[derive(Debug)]
enum GameError {
    NotEnoughGold(u32),
    CardNotFound(String),
}

fn buy_card(gold: u32, price: u32, card_name: &str) -> Result<u32, GameError> {
    if card_name.is_empty() {
        return Err(GameError::CardNotFound(card_name.to_string()));
    }
    if gold < price {
        return Err(GameError::NotEnoughGold(price - gold));
    }
    Ok(gold - price)
}

fn result_demo() {
    match buy_card(100, 50, "Bash") {
        Ok(remaining) => println!("match: bought, {} gold left", remaining),
        Err(GameError::NotEnoughGold(short_by)) => println!("match: need {} more gold", short_by),
        Err(GameError::CardNotFound(name)) => println!("match: no card {}", name),
    }

    // unwrap_or_else - recover with a computed fallback
    let gold = buy_card(30, 50, "Bash").unwrap_or_else(|_| 30);
    println!("unwrap_or_else: gold = {}", gold);

    // map - transform Ok(x), leaves Err alone
    let doubled = buy_card(100, 50, "Bash").map(|g| g * 2);
    println!("map: {:?}", doubled);

    // map_err - transforms the error type/value, leaves Ok alone
    let with_msg = buy_card(10, 50, "Bash").map_err(|e| format!("purchase failed: {:?}", e));
    println!("map_err: {:?}", with_msg);

    fn buy_two(gold: u32) -> Result<u32, GameError> {
        let gold = buy_card(gold, 50, "Bash")?;
        let gold = buy_card(gold, 30, "Defend")?;
        Ok(gold)
    }
    println!("?: {:?}", buy_two(100)); // Ok(20)
    println!("?: {:?}", buy_two(40)); // Err(NotEnoughGold(10))
}

fn main() {
    println!("{}", hp_band(15));
    println!(
        "{}",
        status_ref(&Character {
            name: "Ironclad".into(),
            hp: 80,
            starting_gold: 99
        })
    );
    println!(
        "{}",
        status_owned(Character {
            name: "Ironclad".into(),
            hp: 80,
            starting_gold: 99
        })
    );
    println!("#---Option<T>---#");
    option_demo();
    println!("#---Result<T, E>---#");
    result_demo();
}
