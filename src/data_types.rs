/*
* Data Types
*
* Every value in Rust is of a certain data type, which tells Rust what kind of data is being
* specified so it knows how to work with that data.
*
* Keep in mind that Rust is a statically typed language, which means that it must know the types of
* all variables at compile-time. The compiler can usually infer what type we want to use based on
* the value and how we use it. In cases when many types are possible, such as when a String is
* converted to a numeric types using parse(), we must add a type annotation, like this:
*
*   let guess: u32 = "42".parse().expect("Not a number!");
* If we don't add the : u32 type annotation shown in the preceding code, Rust will display the
* a error, which means the compiler needs more information from us to know which type we want to
* use.
*
* Scalar Types
*
* A scalar type represents a single value. Rust has four primary scalar types: integer,
* floating-point numbers, Booleans, and characters.
*
* Integer Types
*
* An integer is a number without a fractional component. Each variant can be either signed or
* unsigned and has an explicit size. Signed and unsigned refer to whether it's possible for the
* number to be negative--in other words, whether the numbers needs to have a sign with it (signed)
* or whether it will only ever be positive and can therefore be represented without a sign
* (unsigned). It's like writing numbers on paper: when the sign matters, a number is shown with a
* plus sign or a minus sign; However, when it's safe to assume the number is positive, it's shown
* with no sign. Signed numbers are stored using two's complement representation.
*
* Each signed variant can store numbers from -(2^n-1) to 2^n-1 - 1 inclusive, where n is the number
* of bits that variant uses. So an i8 can store numbers from -(2^7) to 2^7 - 1, which equals -128
* to 127. Unsigned variants can store numbers from 0 to 2^n-1, so a u8 can soter numbers from 0 to
* 2^8-1, which equals 0 to 255.
*
* Additionally, the isize and usize types depend on the architecture of the computer your program
* is running on: 64 bits if you're on a 64-bit architecture and 32 bits if you're on a 32-bit
* architecture.
*
* Note that number literals that can be multiple numeric types allow a type suffix, such as 57u8,
* to designate the type. Number literals can also use _ as a visual separator to make the number
* easier to read, such as 1_000, which will have the same value as if you had specified 1000.
*
* Integer types default to i32.
*
* Integer Overflow
*
* Let's say you have a variable of type u8 that can hold values between 0 and 255. If you try to
* change the variable to a value outside that range, such as 256, integer overflow will occur,
* which can result in one of two behaviours. When you're compiling in debug mode, Rust includes
* check for integer overflow that cause your program to panic at runtime if this behaviour occurs.
* Rust uses the term panicking when a program exits wiwth an error.
*
* When you're compiling in release mode with the --release flag, Rust does not include checks for
* integer overflow that cause panics. Instead, if overflow occurs, Rust performs two's complement
* wrapping. In short, values greater than the maximum value the type can hold "wrap around" to the
* minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value
* 257 becomes 1, and so on. The program won't panic, but the variable will have a value that
* probably isn't what you were expecting it to have. Relying on integer overflow's wrapping
* behaviour is considered an error.
*
* To avoid integer overflow:
* - Wrap in all modes with the wrapping_* methods, such as wrapping_add.
* - Return the None value if there is overflow with the checked_* methods.
* - Return the value and a Boolean indicating whether there was oveflow with the overflowing_*
* methods.
* - Saturate at the value's minimum or maximum values with the saturating_* methods.
*
* Floating-Point Types
*
* Rust also has two primitive types for floating-point numbers, which are numbers with decimal
* points. Rust's floating-point types are f32 and f64, which are 32 bits and 64 bits in size,
* respectively. The default type is f64 because on moder CPUs, it's roughly the same speed as f32
* but is capable of more precision. All floating-point types are signed.
*
* The Boolean Type
*
* As in most other programming languages, a Boolean type in Rust has two possible values: true and
* false. Booleans are one byte in size. The Boolean type in Rust is specified using bool.
*
* The Character Type
*
* Rust's char type is the language's most primitive alphabetic type. Note that we specify char
* literals with single quotes, as opposed to string literals, which use double quotes. Rust's char
* type is four bytes in size and represents a Unicode scalar value, which means it can represent a
* lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and
* zero-width spaces are all valid char values in Rust.
*
* Compound Types
*
* Compound types can group multiple values into one type. Rust has two primitive compound types:
* tuples and arrays.
*
* The Tuple Type
*
* A tuple is a general way of grouping together a number of values with a variety of types into one
* compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
*
* We create a tuple by writing a comma-separated list of values inside parantheses. Each position
* in the tuple has a type, and the types of the different values in the tuple don't have to be the
* same. Pattern matching can be used to destructure a tuple value. We can also access a tuple
* element directly by using a period . followed by the index of the value we want to access.
*
* The tuple without any values has a special name, unit. This value and its corresponding type are
* both written () and represent an empty value or an empty return type. Expressions implicitly
* return the unit value if they don't return any other value.
*
* Additionally, we can modify individual elements of a mutable type.
*
* The Array Type
*
* Another way to have a collection of multiple values is with the array. Unlike a tuple, every
* element of an array must have the same type. Unlike arrays in some other languages, arrays in
* Rust have a fixed length.
*
* We write the values in an array as a comma-separated list inside square brackets.
*
* Arrays are useful when you want your data allocated on the stack, the same as the other types we
* have seen so far, rather than the heap or when you want to ensure you always have a fixed number
* of elements. An array isn't as flexible as the vector type, though. A vector is a similar
* collection type provided by the standard library that is allowed to grow or shrink in size
* because its contents live on the heap.
*
* However, arrays are more useful when you know the number of elements will not need to change. For
* example, if you were using the names of the month in a program, you would probably use an array
* rather than a vector because you know it will always contain 12 elements.
*
* You write an array's type using square brackets with the type of each element, a semicolon, and
* then the number of elements in the array, like so:
*
*   let a: [i32; 5] = [1, 2, 3, 4, 5];
* Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array
* contains five elements.
*
* You can also initialize an array to contain the same value for each element by specifying the
* initial value, followed by a semicolon, and then the length of the array in square brackets, as
* shown here:
*
*   let a = [3; 5];
* The array named a will contain 5 elements that will all be set to value 3 initially. This is the
* same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.
*
* Accessing Array Elements
*
* An array is a single chunk of memory of a known, fixed size that can be allocated on the stack.
* You can access elements of an array using indexing.
*/

fn main() {
    let age = 28;
    let my_pi = 3.14;
    let is_engineer = true;
    let best_emoji = '🌝';

    // addition
    let sum = 5 + 10;
    // subtraction
    let diff = 19 - (-14);
    // multiplication
    let product = sum * diff;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    // remainder
    let remainder = 43 % 5;
    // Tuples can store values of different types
    let scalars = (age, my_pi, is_engineer, best_emoji);
    let list = [1, 2, 3, 4, 5];
    let zeros = [0; 10];
    println!("The best emoji is {}", scalars.3);
}
