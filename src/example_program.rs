/*
 * An Example Program Using Structs
 *
 * To understand when we might want to use structs, let's write a program that calculates the area
 * of a rectangle. We'll start by using single variables, and then refactor the program until we're
 * using structs instead.
 *
 * This code succeeds in figuring out the area of the rectangle by calling the area function with
 * each dimension, but we can do more to make this code clear and readable.
 *
 * The issue with this code is evident in the signature of area:
 *
 *  fn area(width: u32, height: u32) -> u32 {
 * The area function is supposed to calculate the area of one rectangle, but the function we wrote
 * has two parameters, and it's not clear anywhere in our program that the parameters are related.
 * It would be more readable and more manageable to group width and height together.
 *
 * Refactoring with Tuples
 *
 * In one way, this program is better. Tuples let us add a bit of structure, and we're now passing
 * just one argument. But in another way, this version is less clear: tuples don't name their
 * elements, so we have to index into the parts of the tuple, making our calculation less obvious.
 *
 * Mixing up the width and height wouldn't matter for the area calculation, but if we want to draw
 * the rectangle on the screen, it would matter! We would have to keep in mind that width is the
 * tuple index 0 and height is the tuple index 1. This would be even harder for someone else to
 * figure out and keep in mind if they were to use our code. Because we haven't conveyed the
 * meaning of our data in our code, it's not easier to introduce errors.
 *
 * Refactoring with Structs: Adding More Meaning
 *
 * Here, we've defined a struct and named it Rectangle. Inside the curly brackets, we defined the
 * fieds as width and height, both of which have type u32. Then, in main, we created a particular
 * instance of Rectangle that has a width of 30 and a height of 50.
 *
 * Our area function is now defined with one parameter, which we've named rectangle, whose type is
 * an immutable borrow of a struct Rectangle instance. As mentioned earlier, we want to borrow the
 * struct rather than take ownership of it. This way, main retains its ownership and can continue
 * using rect1, which is the reason we use the & in the function signature and where we call the
 * function.
 *
 * The area function accesses the width and height fields of the Rectangle instance (note that
 * accessing fields of a borrowed struct instance does not move the field values, which is why you
 * often see borrows of structs). Our function signature for area says exactly what we mean:
 * calculate the area of Rectangle, using its width and height fields. This conveys that the width
 * and height are related to each other, and it gives descriptive names to the values rather than
 * using the tuple index values of 0 and 1. This is a win for clarity.
 *
 * Adding Useful Functionality with Derived Traits
 *
 * It'd be useful to be able to print an instance of Rectangle while we're debugging our program
 * and see the values for all its fields.
 *
 *  println!("rect1 is {rect1}");
 * When we compile the code, we get an error with this core message:
 * `Rectangle` doesn't implement `std::fmt::Display`
 * The println! macro can do many kinds of formatting, and by default, the curly brackets tell
 * println! to use formatting known as Display: output intended for direct end user consumption.
 * The primitive types we've seen so far implement Display by default because there's only one way
 * you'd want to show a 1 or any other primitive type to a user. But with structs, the way println!
 * should format the output is less clear because there are more display possibilities: Do you want
 * commas or not? Do you want to print the curly brackets? Should all the fields be shown? Due to
 * this ambiguity, Rust doesn't try to guess what we want, and structs don't have a provided
 * implementation of Display to use with pritnln! and the {} placeholder.
 *
 *  println!("rect1 is {rect1:?}");
 * Putting the specifier :? inside the curly brackets tells println! we want to use an output
 * format called Debug. The Debug trait enables us to print our struct in a way that is useful for
 * developers so we can see its value while we're debugging our code.
 *
 * Rust does include the functionality to print out debugging information, but we have to
* explicitly opt in to make that functionality available for our struct. To do that, we add the
* outer attribute #[derive(Debug)] just before the struct definition.
*
* It's not the prettiest output, but it shows the values of all fields for this instance, which
* would definitely help during debugging. When we have larger structs, it's useful to have output
* that's a bit easier to read; in those cases, we can use {:#?} instead of {:?} in the println!
* string.
*
* Another way to print out a value using the Debug format is to use the dbg! macro, which takes
* ownership of an expression (as opposed to println!, which takes a reference), prints the file and
* line number of where that dbg! macro call occurs in your code along with the resultant value of
* that expression, and returns ownership of the value.
*
* Note: Calling the dbg! macro prints to the standard error console stream (stderr), as opposed to
* println!, which prints to the standard output console stream (stdout).
*
* We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the
* expression's value, the width field will ge the same value as if we didn't have the dbg! call
* there. We don't want dbg! to take ownership of rect1, so we use a reference to rect1 in the next
* call.
 */

// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }
//
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

/* Refactoring with Tuples */
// fn main() {
//     let rect1 = (30, 50);
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
//
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

/* Refactoring with Structs */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
//     println!("The rect1 is {rect1:?}");
// }

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
