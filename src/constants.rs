/*
* Constants
*
* Like immutable variables, constants are values that are bound to a name, and are not allowed to
* change, but there are a few differences between constants and variables.
*
* First, you aren't allowed to use mut with contants. Constants aren't just immutable by
* default--they're always immutable. You declare constants using the const keyword instead of the
* let keyword, and the type of the value must be annotated.
*
* Constants can be declared in any scope, including the global scope, which makes them useful for
* values that many parts of code need to know about.
*
* The last difference is that constants may be set only to a constant expression, not the result of
* a value that could only be computed at runtime.
*
* Rust's naming convention for constants is to use all uppercase with underscores between words.
* The compiler is able to evaluate a limited set of operations at compile time.
*
* Constants are valid for the entire time a program runs, within the scope in which they were
* declared. This property makes constants useful for values in your application domain that
* multiple parts of the program might need to know about.
*
* Naming hardcoded values used throughout your program as constants is useful in conveying the
* meaning of that value to future maintainers of the code. It also helps to have only one place in
* your code you would need to change if the hardcoded value needed to be updated in the future.
*/
const NAME: &str = "Can";
const BEST_BOSS: &str = "Knight Artorias";

fn main() {
    println!("My name is {NAME} and my favourite boss in Dark Souls is {BEST_BOSS}");
}
