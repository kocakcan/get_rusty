/// Validating References with Lifetimes
///
/// Lifetimes are another kind of generic that we've already been using. Rather than ensuring that
/// a type has the behaviour we want, lifetimes ensure that references are valid as long as we need
/// them to be.
///
/// Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
/// Most of the time, lifetimes are implicit and inferred, just like most of the time, types are
/// inferred. We are only required to annotate types when multiple types are possible. In a similar
/// way, we have to annotate lifetimes when the lifetimes of references could be related in a few
/// different ways. Rust requires us to annotate the relationships using generic lifetime
/// parameters to ensure the actual references used at runtime will definitely be valid.
///
/// Preventing Dangling References with Lifetimes
///
/// The main aim of lifetimes is to prevent dangling references, which cause a program to reference
/// data other than the data it's intended to reference. Consider the unsafe program in Listing
/// 10-16, which has an outer scope and an inner scope.
///
///     fn main() {
///         let r;
///
///         {
///             let x = 5;
///             r  = &x;
///         }
///
///         println!("r: {}", r);
///     }
///     Listing 10-16: An attempt to use a reference whose value has gone out of scope
/// Note: The examples in Listings 10-16, 10-17, and 10-23 declare variables without giving them an
/// initial value, so the variable name exists in the outer scope. At first glance, this might
/// appear to be in conflict with Rust's having no null values. However, if we try to use a
/// variable before giving it a value, we'll get a compile-time error, which shows that Rust indeed
/// does not allow null values.
///
/// The outer scope declares a variable named r with no initial value, and the inner scope declares
/// a variable named x with the initial value of 5. Inside the inner scope, we attempt to set the
/// value of r as a reference to x. Then the inner scope ends, and we attempt to print the value in
/// r. This code won't compile because the value that r is referring to has gone out of scope
/// before we try to use it. Here is the error message:
///
///     error: `x` does not live long enough
/// The error message says that the variable x "does not live long enough." The reason is that x
/// will be out of scope when the inner scope ends on line 7. But r is still valid for the outer
/// scope; because its scope is larger, we say that it "lives longer." If Rust allowed this code to
/// work, r would be referencing memory that was deallocated when x went out of scope, and anything
/// tried to do with r wouldn't work correctly. So how does Rust determine that this code is
/// invalid? It uses a borrow checker.
///
/// The Borrow Checker Ensures Data Outlives Its References
///
/// The Rust compiler has a borrow checker that compares scopes to determine whether all borrows
/// are valid. Listing 10-17 shows the same code as Listing 10-16 but with annotations showing the
/// lifetimes of the variables.
///    fn main() {
///       let r;                // ---------+-- 'a
///                             //          |
///       {                     //          |
///           let x = 5;        // -+-- 'b  |
///           r = &x;           //  |       |
///       }                     // -+       |
///                             //          |
///       println!("r: {r}");   //          |
///   }                         // ---------+
///   Listing 10-17: Annotations of the lifetimes of r and x, named 'a and 'b, respectively.
/// Here, we've annotated the lifetime of r with 'a and the lifetime of x with 'b. As you can see,
/// the inner 'b is much smaller than the outer 'a lifetime block. At compile time, Rust compares
/// the size of the two lifetimes and sees that r has a lifetime of 'a: the subject of the
/// reference doesn't live as long as the reference.
///
/// Listing 10-18 fixes the code so it doesn't have a dangling reference and it compiles without
/// any errors.
///
/// fn main() {
///    let x = 5;             // ----------+-- 'b
///                           //           |
///    let r = &x;            // --+-- 'a  |
///                           //   |       |
///    println!("r: {r}");    //   |       |
///                           // --+       |
/// }                         // ----------+
/// Listing 10-18: A valid reference because the data has a longer lifetime than the reference
///
/// Here, x has the lifetime 'b, which in this case is larger than 'a. This means r can reference x
/// because Rust knows that the reference in r will always be valid while x is valid.
///
/// Generic Lifetimes in Functions
///
/// We'll write a function that returns the longer of two string slices. This function will take
/// two string slices and return a single string slice. After we've implemented the longest
/// function, the code in Listing 10-19 should print The longest string is abcd.
///
///     fn main() {
///         let string1 = String::from("abcd");
///         let string2 = "xyz";
///
///         let result = longest(string.as_str(), string2);
///         println!("The longest string is {result}");
///     }
///     Listing 10-19: A main function that calls the longest function to find the longer of two
///     string slices
/// Note that we want the function to take string slices, which are references, rather than
/// strings, because we don't want the longest function to take ownership of its parameters.
///
/// If we try to implement the longest function as shown in Listing 10-20, it won't compile.
///
///     fn longest(x: &str, y: &str) -> &str {
///         if x.len() > y.len() { x } else { y }
///     }
///     Listing 10-20: An implementation of the longest function that returns the longer of two
///     string slices but does not yet compile
///
///     error[E0106]: missing lifetime specifier
/// The help text reveals that the return type needs a generic lifetime parameter on it because
/// Rust can't tell whether the reference being returned refers to x or y. Actually, we don't know
/// either, because the if block in the body of this function returns a reference to x and the else
/// block returns a reference to y!
///
/// When we're defining this function, we don't know the, concrete values that will be passed into
/// this function, so we don't know whether the if case or the else case will execute. We also
/// don't know the concrete lifetimes of the references that will be passed in, so we can't look at
/// the scopes as we did in Listings 10-177 and 10-18 to determine whether the reference we return
/// will always be valid. The borrow checker can't determine this either, because it doesn't know
/// the lifetimes of x and y relate to the lifetime of the return value. To fix this error, we'll
/// add generic lifetime parameters that define the relationship between the references so the
/// borrow checker can perform its analysis.
///
/// Lifetime Annotation Syntax
///
/// Lifetime annotations don't change how long any of the references live. Rather, they describe
/// the relationships of the lifetimes of multiple references to each other without affecting the
/// lifetimes. Just as functions can accept any type when the signature specifies a generic type
/// parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter. 
///
/// Lifetime annotations have a slightly unusual syntax: The names of lifetime parameters must
/// start with an apostrophe (') and are usually all lowercase and very short, like generic types.
/// Most people use the name 'a for the first lifetime annotation. We place lifetime parameter
/// annotations after the & of a reference, using a space to separate the annotation from the
/// reference's type.
///
/// Here are some examples--a reference to an i32 without a lifetime parameter, a reference to an
/// i32 that has a lifetime parameter named 'a, and a mutable reference to an i32 that also has the
/// lifetime 'a:
///
///     &i32        // a reference
///     &'a i32     // a reference with an explicit lifetime
///     &'a mut i32 // a mutable reference with an explicit lifetime
/// One lifetime annotation by itself doesn't have much meaning, because the annotations are meant
/// to tell Rust how generic lifetime parameters of multiple references relate to each other. Let's
/// examine how the lifetime annotations relate to each other in the context of the longest function.
///
/// In Function Signatures
///
///
