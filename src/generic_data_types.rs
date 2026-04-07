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
///
/// In Struct Definitions
///
/// We can also define structs to use a generic type parameter in one or more fields using the <>
/// syntax. Listing 10-6 defines a Point<T> struct to hold x and y coordinate values of any type.
///
///     struct Point<T> {
///         x: T,
///         y: T,
///     }
///
///     fn main() {
///         let integer = Point { x: 5, y: 10 };
///         let float = Point { x: 1.0, y: 4.0 };
///     }
///     Listing 10-6: A Point<T> struct that holds x and y values of type T
/// The syntax for using generics in struct definitions is similar to that used in function
/// definitions. First we declare the name of the type parameter inside angle brackets just after
/// the name of the struct. Then we use the generic type in the struct definition where we would
/// otherwise specify concrete data types.
///
/// Note that because we've used only one generic type to define Point<T>, this definition says
/// that the Point<T> struct is generic over some type T, and the fields x and y are both that same
/// type, whatever type may be. If we create an instance of a Point<T> that has values of different
/// types, as in Listing 10-7, our code won't compile.
///
///     struct Point<T> {
///         x: T,
///         y: T,
///     }
///
///     fn main() {
///         let wont_work = Point { x: 5, y: 4. 0 };
///     }
///     Listing 10-7: The fields x and y must be the same type because both have the same generic
///     data type T.
/// In this example, when we assign the integer value 5 to x, we let the compiler know that the
/// generic type T will be an integer for this instance of Point<T>. Then we specify 4.0 for y
/// which we've defined to have the same type as x, we'll get a type mismatch error like this:
///
///     mismatched type
/// To define a Point struct where x and y are both generics but could have different types, we can
/// use multiple generic type parameters. For example, Listing 10-8, we change the definition of
/// Point to be generic over types T and U where x is of type T and y is of type U.
///
///     struct Point<T, U> {
///         x: T,
///         y: U,
///     }
///
///     fn main() {
///         let both_integer = Point { x: 5, y: 10};
///         let both_float = Point { x: 1.0, y: 4,0};
///         let integer_and_float = Point { x: 5, y: 4.0 };
///     }
///     Listing 10-8: A Point<T, U> generic over two types so that x and y can be values of
///     different types
///
/// In Enum Definitions
///
/// As we did with structs, we can define enums to hold generic data types in their variants. Let's
/// take another look at the Option<T> enum that the standard library provides.
///
///     enum Option<T> {
///         Some(T),
///         None,
///     }
/// This definition should now make more sense to you. As you can see, the Option<T> enum is
/// generic over type T and has two variants: Some, which holds one value of type T, and a None
/// varian that doesn't hold any value. By using the Option<T> enum, we can express the abstract
/// concept of an optional value, and because Option<T> is generic, we can use this abstraction no
/// matter what the type of the optional value is.
///
/// Enums can use multiple generic types as well. The definition of the Result enum is one example:
///
///     enum Result<T, E> {
///         Ok(T),
///         Err(E),
///     }
/// The Result enum is generic over two types, T and E, and has two variants: Ok, which holds a
/// value of type T, and Err, which holds a value of type E. This definition makes it convenient to
/// use the Result enum anywhere we have an operation that might succeed (return a value of some
/// type T) or fail (return an error of some type E).
///
/// When you recognize situations in your code with multiple struct or enum definitions that differ
/// only in the types of the values they hold, you can avoid duplication by using generic types instead.

struct Point<T, U> {
    x: T,
    y: U,
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {result}");
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {result}");
    let both_integer = Point { x: 19, y: 31 };
    let both_float = Point { x: 3.14, y: 2.71 };
    let integer_and_float = Point { x: 7, y: 420.69 };
}
