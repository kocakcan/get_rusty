/// Generic Data Types
///
/// We use generics to create definitions for items like function signatures or structs, which we
/// can then use with many different concrete data types.
///
/// In Function Definitions
///
/// When defining a function that uses generics, we place the generics in the signature of the
/// function where we would usually specify the data types of the parameters and return value.
/// Doing so makes our code more flexible and provides more functionality to callers of our
/// function while preventing code duplication.
///
/// Continuing with our largest function, Listing 10-4 shows two functions that both find the
/// largest value in a slice. We'll then combine these into a single function that uses generics.
///
///     fn largest_i32(list: &[i32]) -> &i32 {
///         let mut largest = &list[0];
///         for item in list {
///             if item > largest {
///                 largest = item;
///             }
///         }
///         largest
///     }
///
///     fn largest_char(list: &[char]) -> &char {
///         let mut largest = &list[0];
///         for item in list {
///             if item > largest {
///                 largest = item;
///             }
///         }
///         largest
///     }
///
///     fn main() {
///         let number_list = vec![34, 50, 25, 100, 65];
///         let result = largest_i32(&number_list);
///         println!("The largest number is {result}");
///         let char_list = vec!['y', 'm', 'a', 'q'];
///         let result = largest_char(&char_list);
///         println!("The largest char is {result}");
///     }
///     Listing 10-4: Two functions that differ only in their names in their signatures
/// The largest_i32 function is the one we extracted in Listing 10-3 that finds the largest i32 in
///  a slice. The largest_char function finds the largest char in a slice. The function bodies have
///  the same code, so let's eliminate the duplication by introducing a generic type parameter in a
///  single function.
///
///  To paremeterize the types in a new single function, we need to name the type parameter, just
///  as we do for the value parameters to a function. You can use any identifier as a type
///  parameter name. But we'll use T because, by convention, type parameter names in Rust are
///  short, often just one letter, and Rust's type-naming convention is CamelCase. Short for type,
///  T is the default choice of most Rust programmers.
///
///  When we use a parameter in the body of the function, we have to declare the parameter name in
///  the signature so the compiler knows what that name means. Similary, when we use a type
///  parameter name in a function signature, we have to declare the parameter name before we use
///  it. To define the generic largest function, we place type name declarations inside angle
///  brackets, <>, between the name of the function and the parameter list, like this:
///
///     fn largest<T>(list: &[T]) -> &T {
///  We read this definition as: the function largest is generic over some type T. This function
///  has one parameter named list, which is a slice of values of type T. The largest function will
///  return a reference to a value of the same type T.
///
///  Listing 10-5 shows the combined largest function definition using the generic data type in its
///  signature. The listing also shows how we can call the function with either a slice of i32
///  values or char values. Note that this code won't compile yet.
///
///     fn largest<T>(list: &[T]) -> &T {
///         let mut largest = &list[0];
///         for item in list {
///             if item > largest {
///                 largest = item;
///             }
///         }
///         largest
///     }
///
///     fn main() {
///         let number_list = vec![34, 50, 25, 100, 65];
///         let result = largest(&number_list);
///         println!("The largest number is {result}");
///         let char_list = vec!['y', 'm', 'a', 'q'];
///         let result = largest(&char_list);
///         println!("The largest char is {result}");
///     }
///     Listing 10-5: The largest function using generic type parameters; this doesn't compile yet.
/// If we compile this code right now, we'll get this error:
///
///     binary operation `>` cannot be applied to type `&T`
/// The issue above is that when largest takes a slice &[T] as input, the function cannot assume
/// anything about the type T. It could be i32, it could be String, it could be File. However,
/// largest requires that T is something you can compare with > (i.e. that T implements PartialOrd,
/// a trait). Some types like i32 and String are comparable, but other types like File are not comparable.
///
/// In a language like C++ with templates, the compiler would not complain about the implementation
/// of largest, but instead it would complain about trying to call largest on e.g. a file sclie
/// &[File]. Rust instead requires you to state the expected capabilities of generic types up
/// front. If T needs to be comparable then largest must say so. Therefore the compiler error says
/// largest will not compile until T is restricted.
///
/// Additionally, unlike languages like Java where all objects have a set of core methods like
/// Object.toString(), there are no core methods in Rust. Without restrictions, a generic type T
/// has no capabilities: it cannot be printed, cloned, or mutated (although it can be dropped).

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");
}
