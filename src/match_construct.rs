/***
 * The match Control Flow Construct
 *
 * Rust has an extremely powerful control flow construct called match that allows you to compare a
 * value against a series of patterns then execute code based on which pattern matches. Patterns
 * can be made up of literal values, variable names, wildcards, and many other things. The power of
 * match comes from the expressiveness of the patterns and the fact that the compiler confirms that
 * all possible cases are handled.
 *
 * Think of a match expression as being like a coin-sorting machine: coins slide down a track with
 * variously sized holes along it, and each coin falls through the first hole it encounters that it
 * fits into. In the same way, values go through each pattern in a match, and at the first pattern
 * the value "fits," the value falls into the associated code block to be used during execution.
 *
 *  enum Coin {
 *      Penny,
 *      Nickel,
 *      Dime,
 *      Quarter,
 *  }
 *
 *  fn value_in_cents(coin: Coin) -> u8 {
 *      match coin {
 *          Coin::Penny => 1,
 *          Coin::Nickel => 5,
 *          Coin::Dime => 10,
 *          Coin::Quarter => 25,
 *      }
 *  }
 * Let's break down the match in the value_in_cents function. First we list the match keyword
 * followed by an expression, which in this case is the value coin. This seems very similar to a
 * conditional expression used with if, but there's a big difference: with if, the condition needs
 * to evaluate to a Boolean value, but here it can be any type. The type of coin in this example is
 * the Coin enum that we defined on the first line.
 *
 * Next are the match arms. An arm has two parts: a pattern and some code. The first arm here has a
 * pattern that is the value Coin::Penny and then the => operator that separates the pattern and
 * the code to run. The code in this case is just the value 1. Each arm is separated from the next
 * with a comma.
 *
 * When the match expression executes, it compares the resultant value against the pattern of each
 * arm, in order. If a pattern matches the value, the code associated with that pattern is
 * executed. If that pattern doesn't match the value, execution continues to the next arm, much as
 * a coin in a coin-sorting machine. We can have as many arms as we need.
 *
 * The code associated with each arms is an expression, and the resultant value of the expression
 * in the matchin arms is the value that gets returned for the entire match expression.
 *
 * We don't typically use curly brackets if the match arm code is short, as it is above where each
 * arm just returns a value. If you want to run multiple lines of code in a match arm, you must use
 * curly brackets, and the comma following the arm is then optional. For example, the following
 * code prints "Lucky penny" every time the method is called with a Coin::Penny, but still returns
 * the last value of the block, 1:
 *
 *  fn value_in_cents(coin: Coin) -> u8 {
 *      match coin {
 *          Coin::Penny => {
 *              println!("Lucky penny!");
 *              1
 *          }
 *          Coin::Nickel => 5,
 *          Coin::Dime => 10,
 *          Coin::Quarter => 25,
 *      }
 *  }
 *
 * Pattern That Binds to Values
 *
 * Another useful feature of match arms is that they can bind to the parts of the values that match
 * the pattern. This is how we can extract values out of enum variants.
 *
 *  #[derive(Debug)]
 *  enum UsState {
 *      Alabama,
 *      Alaska,
 *      --snip--
 *  }
 *
 *  enum Coin {
 *      Penny,
 *      Nickel,
 *      Dime,
 *      Quarter(UsState),
 *  }
 * Let's imagine that a friend is trying to collect all 50 state quarters. While we sort our loose
 * change by coin type, we'll also call out the name of the state associated with each quarter so
 * that if it's one our friend doesn't have, they can add it to their collection.
 *
 * In the match expression for this code, we add a variable called state to the pattern that
 * matches of the variant Coin::Quarter. When a Coin::Quarter matches, the state variable will bind
 * to the value of that quarter's state. Then we can use state in he code for that arm, like so:
 *
 *  fn value_in_cents(coin: Coin) -> u8 {
 *      match coin {
 *          Coin::Penny => 1,
 *          Coin::Nickel => 5,
 *          Coin::Dime => 10,
 *          Coin::Quarter(state) => {
 *              println!("State quarter from {state:?}");
 *              25
 *          }
 *      }
 *  }
 * If we were to call value_in_cents(Coin::Quarter(UsState::Alaska)), coin would be
* Coin::Quarter(UsState::Alaska). When we compare that value with each of the match arms, none of
* them match until we reach Coin::Quarter(state). At that point, the binding for state will be the
* value UsState::Alaska. We can then use that binding in the println! expression, thus getting the
* inner state value out of the Coin enum variant for Quarter.
 */
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
}
