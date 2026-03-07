/*
* Concise Control Flow with if let and let else
*
* The if let syntax lets you combine if and let into a less verbose way to handle values that match
* one pattern while ignoring the rest. Consider the program below that matches onn an Option<u8>
* value in the config_max variable but only wants to execute code if the value is the Some Variant.
*
*   let config_max = Some(3u8);
*   match config_max {
*       Some(max) => println!("The maximum is configured to be {max}");
*       _ => (),
*   }
* If the value is Some, we print out the value in the Some variant by binding the value to be the
* variable max in the pattern. We don't want to do anything with the None value. To satisfy the
* match expression, we have to add - => () after processing just one variant, which is annoying
* boilerplate code to add.
*
* Instead, we could write this in a shorter way using if let. The following code behaves the same
* as before:
*
*   let config_max = Some(3u8);
*   if let Some(max) = config_max {
*       println!("The maximum is configured to be {max}");
*   }
* The syntax if let takes a pattern and an expression separated by an equal sign. It works the same
* way as a match, where the expression is given to the match and the pattern is its first arm.
* In this case, the pattern is Some(max), and the max binds to the value inside the Some. We can
* then use max in the body of the if block in the same way we used max in the corresponding match
* arm. The code in the if let block only runs if the value matches the pattern.
*
* Using if let means less typing, less indentation, and less boilerplate code. However, you lose
* the exhaustive checking match enforces that ensures you aren't forgetting to handle any cases.
* Choosing between match and if let depends on what you're doing in your particular situation and
* whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.
*
* We can include an else with an if let. The block of code that goes with the else is the same as
* the block of code that would go with the _ case in the match expression that is equivalent to the
* if let and else. Recall the Coin enum definition before, where the Quarter variant also held a
* UsState value. If we wanted to count all non-quarter coins we see while also announcing the state
* of the quarters, we could do that with a match expression, like this:
*
*   let mut count = 0;
*   match Coin {
*       Coin::Quarter(state) => println!("State quarter from {state:?}"),
*       _ => count += 1,
*   }
* Or we could use an if let and else expression, like this:
*
*   let mut count = 0;
*   if let Coin::Quarter(state) = coin {
*       println!("State quarter from {state:?}");
*   } else {
*       count += 1;
*   }
*
* Staying on the "Happy Path" with let...else
*
* The common pattern is to perform some computation when a value is present and return a default
* value otherwise. Continuing on with our example of coins with a UsState value, if we wanted to
* say something funny depending on how old the state on the quarter was, we might introduce a
* method on UsState to check the age of a state, like so:
*
*   impl UsState {
*       fn existed_in(&self, year: u16) -> bool {
*           match self {
*               UsState::Alabama => year >= 1819,
*               UsState::Alaska => year >= 1959,
*               -- snip --
*           }
*       }
*   }
* Then we might use if let to match on the type of coin, introducing a state variable within the
* body of the condition, as shown below:
*
*   fn describe_state_quarter(coin: Coin) -> Option<String> {
*       if let Coin::Quarter(state) = coin {
*           if state.existed_in(1900) {
*               Some(format!("{state:?} is pretty old, for America!"))
*           } else {
*               Some(format!({state:?} is relatively new.))
*           }
*       } else {
*           None
*       }
*   }
* That gets the job done, but it has pushed the work into the body of the if let statement, and if
* the work to be done is more complicated, it might be hard to follow exactly how the top-level
* branches relate. We could also take advantage of the fact that expressions produce a value either
* to produce the state from the if let or to return early, as shown below:
*
*   fn describe_state_quarter(coin: Coin) -> Option<String> {
*       let state = if let Coin::Quarter(state) = coin {
*           state
*       } else {
*           return None;
*       }
*
*       if state.existed_in(1900) {
*           Some(format!("{state:?} is pretty old, for America!"))
*       } else {
*           Some(format!("{state:?} is relatively new."))
*       }
*   }
* This is a bit annoying to follow in its own way, though! One branch of the if let produces a
* value, and the other one returns from the function entirely.
*
* To make this common pattern nicer to express, Rust has let...else. The let...else syntax takes a
* pattern on the left side and an expression on the right, very similar to if let, but it does not
* have an if branch, only an else branch. If the pattern matches, it will bind the value from the
* pattern in the outer scope. If the pattern does not match, the program will flow into the else
* arm, which must return from the function.
*
*   fn describe_state_quarter(coin: Coin) -> Option<String> {
*       let Coin::Quarter(state) = coin else {
*           return None;
*       }
*
*       if state.existed_in(1900) {
*           Some(format!("{state:?} is pretty old, for America!"))
*       } else {
*           Some(format!("{state:?} is relatively new."))
*       }
*   }
* Notice that it says "on the happy path" in the main body of the function this way, without having
* significantly different control flow for two branches the way the if let did.
*
* If you have a situation in which your program has logic that is too verbose to express using a
* match, remember that if let and let...else are in your Rust toolbox as well.
*/
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("You haven't provided a valid configuration value");
    }
}
