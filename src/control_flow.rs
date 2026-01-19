/*
* Control Flow
*
* The ability to run some code depending on whether a condition is true and to run some code
* repeatedly while a condition is true are basic building blocks in most programming languages. The
* most common constructs that let you control the flow of execution of Rust code are if expressions
* and loops.
*
* if expressions
*
* An if expression allows you to branch your code depending on conditions. You provide a condition
* and then state, "If this condition is met, run this block of code. If the condition is not met,
* do not run this block of code."
*
*   fn main() {
*       let number = 3;
*
*       if number < 5 {
*           println!("condition was true");
*       } else {
*           println!("condition was false");
*       }
*   }
* All if expressions start wit the keyword if, followed by a condition. In this case, the condition
* checks whether or not the variable number has a value less than 5. We place the block of code to
* execute if the condition is true immediately after the condition inside curly brackets. Blocks of
* code associated with the conditions in if expressions are sometimes called arms, just like arms
* in match expressions.
*
* Optionally, we can also include an else expression, which we chose to do here, to give the
* program an alternative block of code to execute should the condition evaluate to false. If you
* don't provide an else expression and the condition is false, the program will skip if block and
* move on to the next bit of code.
*
* It's also worth noting that the condition in this code must be a bool. If the condition isn't a
* bool, we'll get an error. Rust will not automatically try to convert non-Boolean types to a
* Boolean. You must be explicit and always provide if with a Boolean as its condition. If we want
* the if code block to run only when a number is not equal to 0, for example, we can change the if
* expression to the following:
*
*   fn main() {
*       let number = 3;
*
*       if number != 0 {
*           println!("number was something other than zero");
*       }
*   }
*
* Handling Multiple Conditions with else if
*
* You can use multiple conditions by combining if and else in an else if expression. For example:
*
*   fn main() {
*       let number = 6;
*
*       if number % 4 == 0 {
*           println!("number is divisible by 4");
*       } else if number % 3 == 0 {
*           println!("number is divisible by 3");
*       } else if number % 2 == 0 {
*           println!("number is divisible by 2");
*       } else {
*           println!("number is not divisible by 4, 3, or 2.");
*       }
*   }
* When this program executes, it checks each if expression in turn and executes the first body for
* which the condition evaluates to true. Note that even though 6 is divisible by 2, we don't see
* the output number is divisible by 2, nor do we see the number is not divisible by 4, 3, or 2 text
* from the else block. That's because Rust only executes the block for the first true condition,
* and once it finds one, it doesn't even check the rest.
*
* Using too many else if expressions can clutter your code, so if you have more than one, you might
* want to refactor your code using match constructs.
*
* Using if in a let Statement
*
* Because if is an expression, we can use it on the right side of a let statement to assign the
* outcome to a variable
*
*   fn main() {
*       let condition = true;
*       let number = if condition { 5 } else { 6 };
*
*       println!("The value of number is: {number}");
*   }
* The number variable will be bound to a value based on the outcome of the if expression. Remember
* that blocks of code evaluate to the last expression in them, and numbers by themselves are also
* expressions. In this case, the value of the whole if expression depends on which block of code
* executes. This means the values that have the potential to be results from each arm of the if
* must be the same type. If the types are mismatched, as in the following example, we'll get an
* error:
*
*   fn main() {
*       let condition = true;
*       let number = if condition { 5 } else { "six" };
*
*       println!("The value of number is: {number}");
*   }
* The if an else arms have value types that are incompatible. The expression in the if block
* evaluates to an integer, and the expression in the else block evaluates to a string. This won't
* work because variables must have a single type, and Rust needs to know at compile time what type
* the number variable is, definitively. Knowing the type of number lets the compiler verify the
* type is valid everywhere we use number. Rust wouldn't be able to do that if the type of number
* was only determinde at runtime; the compiler would be more complex and would make fewer
* guarantees about the code if it had to keep track of multiple hypothetical types for any
* variable.
*
* Repetition with Loops
*
* It's often useful to execute a block of code more than once. For this task, Rust provides several
* loops, which will run through the code inside the loop body to the end and then start immediately
* back at the beginning.
*
* Rust has three kinds of loops: loop, while, and for.
*
* Repeating Code with loop
*
* The loop keyword tells Rust to execute a block of code over and over again forever or until you
* explicitly tell it to stop.
*
*   fn main() {
*       loop {
*           println!("again!");
*       }
*   }
* When we run this program, we'll se again! printed over and over continuously until we stop the
* program manually. Most terminals support the keyboard shortcut ctrl-c to interrupt a program that
* is stuck in a continual loop.
*
* Fortunately, Rust also provides a way to break out of a loop using code. You can place the break
* keyword within the loop to tell the program when to stop executing the loop. continue keyword
* tells the program to skip over any remaining code in this iteration of the loop and go to the
* next iteration.
*
* Returning Values from Loops
*
* One of the uses of a loop is to retry an operation you know might fail, such as checking whether
* a thread has completed its job. You might also need to pass the result of that operation out of
* the loop to the rest of your code. To do this, you can add the value you want returned after the
* break expression you use to stop the loop; that value will be returned out of the loop so you can
* use it, as shown here:
*
*   fn main() {
*       let mut counter = 0;
*
*       let result = loop {
*           counter += 1;
*           if counter == 10 {
*               break counter * 2;
*           }
*       };
*       println!("The result is {result}");
*   }
* Before the loop, we declare a variable named counter and initialize it to 0. Then we declare a
* variable named result to hold the value returned from the loop. On every iteration of the loop,
* we add 1 to the counter variable, and then check whether the counter is equal to 10. When it is,
* we use the break keyword with the value counter * 2. After the loop, we use a semicolon to end
* the statement that assigns the value to result. Finally, we print the value in result, which in
* this case is 20.
*
* You can also return from inside a loop. While break only exits the current loop, return always
* exists the current function.
*
* Note: the semicolon after break counter * 2 is technically optional. break is very similar to
* return, in that both can optionally take an expression as an argument, botch cause a change in
* control flow. Code after a break or return is never executed, so the Rust compiler treats a break
* expression and a return expression as having the value unit, or ().
*
* Loop Labels to Disambiguate Between Multiple Loops
*
* If you have loops within loops, break and continue apply to the innermost loop at that point. You
* can optionally specify a loop label on a loop that you can then use with break or continue to
* specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels
* must begin with a single quote.
*
*   fn main() {
*       let mut count = 0;
*       'counting_up: loop {
*           println!("count = {count}");
*           let mut remaining = 10;
*
*           loop {
*               println!("remaining = {remaining}");
*               if remaining == 9 {
*                   break;
*               }
*               if count == 2 {
*                   break 'counting_up;
*               }
*               remaining -= 1;
*           }
*           count += 1;
*       }
*       println!("End count = {count}");
*   }
* The outer loop has the label 'counting_up, and it will count up from 0 to 2. The inner loop
* without a label counts down from 10 to 9. The first break that doesn't specify a label will exit
* the inner loop only. The break 'counting_up; statenent will exit the outer loop.
*
* Conditional Loops with while
*
* A program will often need to evaluate within a loop. While the condition is true, the loop runs.
* When the condition ceases to be true, the program calls break, stopping the loop. It's possible
* to implement behavior like this using a combination of loop, if, else, and break; you could try
* that now in a program, if you'd like. However, this pattern is so common that Rust has a built-in
* langauge construct for it, called a while loop.
*
*   fn main() {
*       let mut number = 3;
*
*       while number != 0 {
*           println!("{number}");
*           number -= 1;
*       }
*       println!("LIFTOFF!!!");
*   }
* This construct eliminates a lot of nesting that would be necessary if you used loop, if, else,
* and break, and it's cleaner. While a condition evaluates to true, the code runs; otherwise, it
* exits the loop.
*
* Looping Through a Collection with for
*
* You can also use the while construct to loop over the elements of a collection, such as an array.
*
*   fn main() {
*       let a = [10, 20, 30, 40, 50];
*       let mut index = 0;
*
*       while index < 5 {
*           println!("the value is: {}", a[index]);
*           index += 1;
*       }
*   }
* Here, the code counts up through the elements in the array. It starts at index 0, and then loops
* until it reaches the final index in the array (that is, when index < 5 is no longer true).
*
* However, this approach is error prone; we could cause the program to panic if the index value or
* test condition is incorrect. For example, if you changed the definition of the a array to have
* four elements but forgot to update the condition to while index < 4, the code would panic. It's
* also slow, because the compiler adds runtime code to perform the conditional check of whether the
* index is within the bounds of the array on every iteration through the loop.
*
* As a more concise alternative, you can use a for loop and execute some code for each item in a
* collection.
*
*   fn main() {
*       let a = [10, 20, 30, 40, 50];
*
*       for element in a {
*           println!("the value is: {element}");
*       }
*   }
* Machine code generated from for loops can be more efficient as well, because the index doesn't
* need to be compared to the length of the array at every iteration.
*
* Using the for loop, you wouldn't need to remember to change any other code if you changed the
* number of values in the array.
*
* The safety and conciseness of for loops make them the most commonly used loop construct in Rust.
* Even in situations in which you want to run some code a certain number of times, as in the
* countdown example that used a while loop, most Rustaceans would use a for loop. The way to do
* that would be to use a Range, provided by the standard library, which generates all numbers in
* sequence starting from one number and ending before another number.
*
*   fn main() {
*       for number in (1..4).rev() {
*           println!("{number}");
*       }
*       println!("LIFTOFF!!!");
*   }
*/

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 19 };
    // won't work due to mismatched types
    // let number if condition { 5 } else { "19" };

    println!("The value of number is: {number}");
    // loop {
    //     println!("MAXIMUM POWER!!!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // semicolon is optional
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
