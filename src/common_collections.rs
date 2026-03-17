/*
 * Common Collections
 *
 * Rust's standard library includes a number of very useful data structures called collections.
 * Most other data types represent one specific value, but collections can contain multiple values.
 * Unlike the built-in array and tuple types, the data that these collections point to is stored on
 * the heap, which means the amount of data does not need to be known at compile time and can grow
 * or shrink as the program runs. Each kind of collection has different capabilities and costs, and
 * choosing an appropriate one for your current situation is a skill you'll develop over time.
 *
 * Storing Lists of Values with Vectors
 *
 * Vec<T> is also known as vector. Vectors allow you to store more than one value in a single data
 * structure that pulls all the values next to each other in memory. Vectors can only stores values
 * of the same type. They are useful when you have a list of items, such as the lines of text in a
 * file or the prices of items in a shopping cart.
 *
 * Creating a New Vector
 *
 * To create a new empty vector, we call the Vec::new function, as shown below:
 *
 *  let v: Vec<i32> = Vec::new();
 * Note that we added a type annotation here. Because we aren't inserting any values into this
 * vector, Rust doesn't know what kind of elements we intend to store. This is an important point.
 * Vectors are implemented using generics and Vec<T> type provided by the standard library can hold
 * any type. When we create a vector to hold a specific type, we can specify the type within angle
 * brackets.
 *
 * More often, you'll create Vec<T> with initial values and Rust will infer the type of value you
 * want to store, so you rarely need to do this type annotation. Rust conveniently provides the
 * vec! macro, which will create a new vector that holds the values you give it. The code below
 * creates a new Vec<i32> that holds the values 1, 2, and 3. The integer type is i32 because that's
 * the default integer type.
 *
 *  let v = vec![1, 2, 3];
 * Because we've given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the
 * type annotation isn't necessary.
 *
 * Updating a Vector
 */
