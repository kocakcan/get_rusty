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
*
* Matching with Option<T>
*
* We can also handle Option<T> using match, as we did with the Coin enum! Instead of comparing
* coins, we'll compare variants of Option<T>, but the way the match expression works remains the
* same.
*
* Let's say we want to write a function that takes an Option<i32> and, if there's a value inside,
* adds 1 to that value. If there isn't a value inside, the function should return the None value
* and not attempt to perform any operations.
*
* This function is very easy to write, thanks to match:
*
*   fn plus_one(x: Option<i32>) -> Option<i32> {
*       match x {
*           None => None,
*           Some(i) => Some(i + 1),
*       }
*   }
*
*   let five = Some(5);
*   let six = plus_one(five);
*   let none = plus_one(None);
* Let's examine the first execution of plus_one in more detail. When we call plus_one(five), the
* variable x in the body of plus_one will have the value Some(5). We then compare that against each
* match arm:
*
*   None => None,
* The Some(5) value doesn't match the pattern None, so we continue to the next arm:
*
*   Some(i) => Some(i + 1),
* Does Some(5) match Some(i)? It does! We have the same variant. The i binds to the value in Some,
* so i takes the value 5. The code in the match arm is then executed, so we add 1 to the value of i
* and create a new Some value with our total 6 inside.
*
* Now let's consider the second call of plus_one above, where x is None. We enter the match and
* compare to the first arm:
*
*   None => None,
* It matches! There's no value to add to, so the program stops and returns the None value on the
* right side of =>. Because the first arm matched, no other arms are compared.
*
* Combining match and enums is useful in many situations. You'll see this pattern a lot in Rust
* code: match against an enum, bind a variable to the data inside, and then execute code based on
* it. It's a bit tricky at first, but once you get used to it, you'll wish you had it in all
* languages. It's consistently a user favourite.
*
* Matches are Exhaustive
*
* The arms' patterns must cover all possibilities. Consider this version of our plus_one function,
* which has a bug and won't compile:
*
*   fn plus_one(x: Option<i32>) -> Option<i32> {
*       match x {
*           Some(i) => Some(i + 1),
*       }
*   }
* We didn't handle the None case, so this code will cause a bug.
*
*   non-exhaustive patterns: `None` not covered
* Rust knows that we didn't cover every possible case, and even knows which pattern we forgot!
* Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to
* be valid. Especially in the case of Option<T>, when Rust prevents us from forgetting to
* explicitly handle the None case, it protects us from assuming that we have a value when we might
* have null.
*
* Catch-All Patterns and the _ Placeholder
*
* Using enums, we can also take special actions for a few particular values, but for all other
* values take on default action. Imagine we're implementing a game where, if you roll a 3 on a dice
* roll, your player doesn't move, but instead gets a new fancy hat. If you roll a 7, your player
* loses a fancy hat. For all other values, your player moves that number of spaces on the game
* board.
*
*   let dice_roll = 9;
*   match dice_roll {
*       3 => add_fancy_hat(),
*       7 => remove_fancy_hat(),
*       other = move_player(other),
*   }
*
*   fn add_fancy_hat() {}
*   fn remove_fancy_hat() {}
*   fn move_player(num_spaces: u8) {}
* For the first two arms, the patterns are the literal values 3 and 7. For the last arm that covers
* every other possible value, the pattern is the variable we've chosen to name other. The code that
* runs for the other arm uses the variable by passing it to the move_player function.
*
* This code compiles, even though we haven't listed all the possible values a u8 can have, because
* the last pattern will match all values not specifically listed. This catch-all pattern meets the
* requirement that match must be exhaustive. Note that we have to put the catch-all arm last
* because the patterns are evaluated in order. If we put the catch-all arm earlier, the other arms
* would never run, so Rust will warn us if we add arms after a catch-all!
*
* Rust also has a pattern we can use when we want a catch-all but don't want to use the value in
* the catch-all pattern: _ is a special pattern that matches any value and does not bind to that
* value. This tells Rust we aren't going to use the value, so Rust won't warn us about an unused
* variable.
*
* Let's change the rules of the game: now, if you roll anything other than 3 or a 7, you must roll
* again. We no longer need to use the catch-all value, so we can change our code to use _ instead
* of the variable named other:
*
*   let dice_roll = 9;
*   match dice_roll {
*       3 => add_fancy_hat(),
*       7 => remove_fancy_hat(),
*       _ => reroll(),
*   }
*
*   fn add_fancy_hat() {}
*   fn remove_fancy_hat() {}
*   fn reroll() {}
* This example also meets the exhaustiveness requirement because we're explicitly ignoring all
* other values in the last arm; we haven't forgotten anything.
*
* Finally, we'll change the rules of the game one more time so that nothing else happens on your
* turn if you roll anything other than a 3 or a 7. We can express that by using the unit value (the
* empty tuple type) as the code that goes with the _ arm:
*
*   let dice_roll = 9;
*   match dice_roll {
*       3 => add_fancy_hat(),
*       7 => remove_fancy_hat(),
*       _ => (),
*   }
*
*   fn add_fancy_hat() {}
*   fn remove_fancy_hat() {}
* Here, we're telling Rust explicitly that we aren't going to use any other value that doesn't
* match a pattern in an earlier arm, and we don't want to run any code in this case.
*
* How Matches Interact with Ownership
*
* If an enum contains non-copyable data like a String, then you should be careful with whether a
* match will move or borrow that data. For example, this program using an Option<String> will
* compile:
*
*   let opt: Option<String> =
*       Some(String::from("Hello world"));
*
*   -> opt          | RO
*   -> opt@Some.0   | RO
*
*   match opt {
*       Some(_) => println!("Some!"),
*       None => println!("None!")
*   };
*
*   println!("{:?}", opt);
*
*   -> opt          | No permissions (goes out of scope)
*   -> opt@Some.0   | No permissions (goes out of scope)
* But if we replace the placeholder in Some(_) with a variable name, like Some(s), then the program
* will NOT compile:
*
*   let opt: Option<String> =
*       Some(String::from("Hello world"));
*
*   -> opt          | RO
*   -> opt@Some.0   | RO
*
*   match opt {
*       -> _ became s
*       Some(s) => println!("Some: {}", s);
*
*       -> opt          | No permissions (was moved)
*       -> opt@Some.0   | No permissions (was moved)
*
*       None => println!("None!")
*   };
*
*   println!("{:?}", opt);
* opt is a plain enum its type is Option<String> and not a reference like &Option<String>.
* Therefore a match on opt will move non-ignored fields like s. Notice how opt loses read and own
* permissions sooner in the second program compared to the first. After the match expression, the
* data within opt has been moved, so it's illegal to read opt in the println.
*
* If we want to peek into opt without moving its contents, the idiomatic solution is to match on a
* reference:
*
*   let opt: Option<String> =
*       Some(String::from("Hello world"));
*
*   -> opt          | RO
*   -> opt@Some.0   | RO
*
*   match &opt {
*       -> opt          | R (loses O as it's borrowed)
*       -> opt@Some.0   | R (lose O as it's borrowed)
*
*       Some(s) => println!("Some: {}", s);
*
*       -> opt          | RO (regains O as the borrow is no longer in use)
*       -> opt@Some.0   | RO (regains O as the borrow is no longer in use)
*
*       None => println!("None!")
*
*       -> opt          | RO
*       -> opt@Some.0   | RO
*   };
*
*   println!("{:?}", opt);
*
*   -> opt          | No permissions (goes out of scope)
*   -> opt@Some.0   | No permissions (goes out of scope)
* Rust will "push down" the reference from the outer enum, &Option<String>, to the innter field,
* &String. Therefore s has tyep &String, and opt can be used after the match.
*
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

fn add_fancy_hat() {
    println!("Received a fancy hat");
}

fn remove_fancy_hat() {
    println!("You lost your fancy hat");
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
    let dice_roll = 7;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    let opt: Option<String> = Some(String::from("Hello world"));

    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    println!("{:?}", opt);
}
