/// Functional Language Features: Iterators and Closures
///
/// Rust's design has taken inspiration from many existing languages and techniques, and one
/// significant influence is functional programming. Programming in a functional style often
/// includes using functions as values by passing them in arguments, returning them from other
/// functions, assigning them to variables for later execution, and so forth.
///
/// Closures: Anonymous Functions That Capture Their Environment
///
/// Rust's closures are anonymous functions you can save in a variable or pass as arguments to other
/// functions. You can create the closure in one place and then call the closure elsewhere to
/// evaluate it in a different context. Unlike functions, closures can capture values from the scope
/// in which they're defined.
///
/// Capturing the Environment with Closures
///
/// Here's the scenario: Every so often, our t-shirt company gives away an exclusive,
/// limited-edition shirt to someone on our mailing list as a promotion. People on the mailing list
/// can optionally add their favourite color to their profile. If the person chosen for a free shirt
/// has their favourite color set, they get that color shirt. If the person hasn't specified a
/// favourite color, they get whatever color the company has the most of.
///
/// The store defined in main has two blue shirts and one red shirt remaining to distribute for
/// this limited-edition promotion. We call the giveaway method for a user with a preference for a
/// red shirt and a user without any preference.
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("
        The user with preference {:?} gets {:?}",
        user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("
        The user with preference {:?} gets {:?}",
        user_pref2, giveaway2);

}
