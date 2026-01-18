/*
* By default, variables are immutable. However, you still have the option to make your variables
* mutable.
*
* When a variable is immutable, once a value is bound to a name, you can't change that value.
*
* The Rust compiler guarantees that when you state that a value won't change, it really won't
* change, so you don't have to keep track of it yourself. Your code is easier to reason through.
*
* But mutability can be very useful, and can make code more convenient to write. Although variables
* are immutable by default, you can make them mutable by adding mut in front of the variable name.
* Adding mut also conveys intent to future readers of the code by indicating that other parts of
* the code will be changing this variable's value.
*/
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6;  /* error: variables are constant by default, use mut */
    // println!("The value of x is: {x}");
}
