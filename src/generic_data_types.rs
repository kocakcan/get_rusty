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
///
/// In Method Definitions
///
/// We can implement methods on structs and enums and use generic types in their definitions too.
/// Listing 10-9 shows the Point<T> struct we defined in Listing 10-6 with a method name x
/// implemented on it.
///
///     struct Point<T> {
///         x: T,
///         y: T,
///     }
///
///     impl<T> Point<T> {
///         fn x(&self) -> &T {
///             &self.x
///         }
///     }
///
///     fn main() {
///         let p = Point { x: 5, y: 10};
///         println!("p.x = {}", p.x());
///     }
///     Listing 10-9: Implementing a method named x on the Point<T> struct that will return a
///     reference to the x field of type T.
/// Here, we've defined a method named x on Point<T> that returns a reference to the data in the
/// field x.
///
/// Note that we have to declare T just after impl so we can use T to specify that we're
/// implementing methods on the type Point<T>. By declaring T as a generic type after impl, Rust
/// can identify that the type in the angle brackets in Point is a generic type rather than a
/// concrete type. We could have chosen a different name for this generic parameter than the
/// generic parameter declared in the struct definition, but using the same name is conventional.
/// If you write a method within an impl that declares a generic type, that method will be defined
/// on any instance of the type no matter what concrete type ends up substituing for the generic
/// type.
///
/// We can also specify constraints on generic types when defining methods on the type. We could,
/// for example, implement methods only on Point<f32> instances rather than on Point<T> instances
/// with any generic type. In Listing 10-10 we use the concrete type f32, meaning we don't declare
/// any types after impl.
///
///     impl Point<f32> {
///         fn distance_from_origin(&self) -> f32 {
///             (self.x.powi(2) + self.y.powi(2)).sqrt()
///         }
///     }
///     Listing 10-10: An impl block that only applies to a struct with a particular concrete type
///     for the generic type parameter T
/// This code means the type Point<f32> will have a distance_from_origin method; other instances of
/// Point<T> where T is not of type f32 will not have this method defined. The method measures how
/// far our point is from the point at coordinates (0.0, 0.0) and uses mathematical operations that
/// are available only for floating-point types.
///
/// You cannot simultaneously implement specific and generic methods of the same name this way. For
/// example, if you implemented a general distance_from_origin for all types T and a specific
/// distance_from_origin for f32, then the compiler will reject your program: Rust does not know
/// which implementation to use when you call Point<f32>::distance_from_origin. More generally,
/// Rust does not have inheritance-like mechanisms for specializing methods as you might find in an
/// object-oriented language, with one expection (default trait methods).
///
/// Generic type parameters in a struct definition aren't always the same as those you use in the
/// same struct's methods signatures. Listing 10-11 uses the generic types X1 and Y1 for the Point
/// sturct and X2 and Y2 for the mixup method signature to make the example clearer. The method
/// creates a new Point instance with the x value from the self Point (of type X1) and the y value
/// from the passed-in Point (of type Y2).
///
///     struct Point<X1, Y1> {
///         x: X1,
///         y: Y1,
///     }
///
///     impl<X1, Y1> Point<X1, Y1> {
///         fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
///             Point {
///                 x: self.x,
///                 y: other.y,
///             }
///         }
///     }
///
///     fn main() {
///         let p1 = Point { x: 5, y: 10.4 };
///         let p2 = Point { x: "Hello", y: 'c' };
///         let p3 = p1.mixup(p2);
///         println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
///     }
///     Listing 10-11: A method that uses generic types different from its struct's definition
/// In main, we've defined a Point that has an i32 for x (with value 5) and an f64 for y (with
/// value 10.4). The p2 variable is a Point struct that has a string slice for x (with value
/// "Hello") and a char for y (with value c). Calling mixup on p1 with the argument p2 gives us p3,
/// which will have an i32 for x because x came from p1. The p3 variable will have a char for y
/// because y came from p2. The println! macro call will print p3.x = 5, p3.y = c.
///
/// The purpose of this example is to demonstrate a situation in which some generic parameters are
/// declared with impl and some are declared with the method definition. Here, the generic
/// parameters X1 and Y1 are declared after impl because they go with the struct definition. The
/// generic parameters X2 and Y2 are declared after fn mixup because they're only relevant to the method.
///
/// Performance of Code Using Generics
///

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
