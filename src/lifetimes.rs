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
/// To use lifetime annotations in function signatures, we need to declare the generic lifetime
/// parameters inside angle brackets between the function name and the parameter list, just as we
/// did with generic type parameters.
///
/// We want the signature to express the following constraint: The returned reference will be valid
/// as long as both of the parameters are valid. This is the relationship between lifetimes of the
/// parameters and the return value. We'll name the lifetime 'a and then add it to each reference,
/// as shown in Listing 10-21.
///
///     fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
///         if x.len() > y.len() { x } else { y }
///     }
///     Listing 10-21: The longest function definition specifying that all the references in the
///     signature must have the same lifetime 'a.
/// This code should compile and produce tthe result we want when we use it with the main function
/// in Listing 10-19.
///
/// The function signature now tells Rust that for some lifetime 'a, the function takes two
/// parameters, both of which are string slices that live at least as long as lifetime 'a. The
/// function signature also tells Rust that the string slice returned from the function will live
/// at least as long as lifetime 'a. In practice, it means that the lifetime of the reference
/// returned by the longest function is the same as the smaller of the lifetimes of the values
/// referred to by the function arguments. These relationships are what we want Rust to use when
/// analyzing the code.
///
/// Remember, when we specify the lifetime parameters in this function signature, we're not
/// changing the lifetimes of any values passed in or returned. Rather, we're specifying that the
/// borrow checker should reject any values that don't adhere to these constraints. Note that the
/// longest function doesn't need to know exactly how long x and y will live, only that some scope
/// can be substituted for 'a that will satisfy this signature.
///
/// When annotating lifetimes in functions, the annotations go in the function signature, not in
/// the function body. The lifetime annotations become part of the contract of the function, much
/// like the types in the signature. Having function signatures contain the lifetime contract means
/// the analysis the Rust compiler does can be simpler. If there's a problem with the way a
/// function is annotated or the way it is called, the compiler errors can point to the part of our
/// code and the constraints more precisely. If, instead, the Rust compiler made more inferences
/// about what we intended the relationships of the lifetimes to be, the compiler might only be
/// able to point to a use of our code many steps away from the cause of the problem.
///
/// When we pass concrete references to longest, the concrete lifetime that is substituted for 'a
/// is the part of the scope of x that overlaps with the scope of y. In other words, the generic
/// lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x
/// and y. Because we've annotated the returned reference with the same lifetime parameter 'a, the
/// returned reference will also be valid for the length of the smaller of the lifetimes of x and y.
///
/// Let's look at how the lifetime annotations restrict the longest function by passing in
/// references that have different concrete lifetimes. Listing 10-22 is a straightforward example.
///
///     fn main() {
///         let string1 = String::from("long string is long");
///
///         {
///             let string2 = String::from("xyz");
///             let result = longest(string1.as_str(), string2.as_str());
///             println!("The longest string is {result}");
///         }
///     }
///     Listing 10-22: Using the longest function with references to String values that have
///     different concrete lifetimes
/// In this example, string1 is valid until the end of the outer scope, string2 is valid until the
/// end of the inner scope, and result references something that is valid until the end of the
/// inner scope. Run this code and you'll see that the borrow checker approves; it will compile and
/// print The longest string is long string is long.
///
/// Next, let's try an example that shows that the lifetime of the reference in result must be the
/// smaller lifetime of the two arguments. We'll move the declaration of the result variable
/// outside the inner scope but leave assignment of the value to the result variable inside the
/// scope with string2. Then we'll move the println! that uses the result to outside the inner
/// scope, after the inner scope has ended. The code in Listing 10-23 won't compile.
///
///     fn main() {
///         let string1 = String::from("long string is long");
///         let result;
///         {
///             let string2 = String::from("xyz");
///             result = longest(string1.as_str(), string2.as_str());
///         }
///         println!("The longest string is {result}");
///     }
///     Listing 10-23: Attempting to use result after string2 has gone out of scope
/// When we try to compile this code, we get this error:
///
///     `string2` does not live long enough
/// The error shows that for result to be valid for the println! statement, string2 would need to
/// be valid until the end of the outer scope. Rust knows this because we annotated the lifetimes
/// of the function parameters and return values using the same lifetime parameter 'a.
///
/// As humans, we can look at this code and see that string1 is longer than string2, and therefore,
/// result will contain a reference to string1. Because string1 has not gone out of scope yet, a
/// reference to string1 will still be valid for the println! statement. However, the compiler
/// can't see that the reference is valid in this case. We've told Rust that the lifetime of the
/// reference returned by the longest function is the same as the smaller of the lifetimes of the
/// references passed in. Therefore, the borrow checker disallows the code in Listing 10-23 as
/// possibly having an invalid reference.
///
/// Thinking in Terms of Lifetimes
///
/// The way in which you need to specify lifetime parameters depends on what your function is
/// doing. For example, if we changed the implementation of the longest function to always return
/// the first parameter rather than the longest string slice, we wouldn't need to specify a
/// lifetime on the y parameter. The following code will compile:
///
///     fn longest<'a>(x: &'a str, y: &str) -> &'a str {
///         x
///     }
/// We've specified a lifetime parameter 'a for the parameter x and the return type, but not for
/// the parameter y, because the lifetime of y does not have any relationship with the lifetime of
/// x or the return value.
///
/// When returning a reference froma function, the lifetime parameter for the return type needs to
/// match the lifetime parameter for one of the parameters. If the reference returned does not
/// refer to one of the parameters, it must refer to a value created within this function. However,
/// this would be a dangling reference because the value will go out of scope at the end of the
/// function. Consider this attempted implementation of the longest function that won't compile:
///
///     fn longest<'a>(x: &str, y: &str) -> &'a str {
///         let result = String::from("really long string");
///         result.as_str()
///     }
/// Here, even though we've specified a lifetime parameter 'a for the return type, this
/// implementation will fail to compile because the return value lifetime is not related to the
/// lifetime of the parameters at all. Here is the error message we get:
///
///     error[E0515]: cannot return value referencing local variable `result`
/// The problem is that result goes out of scope and gets cleaned up at the end of the longest
/// function. We've also trying to return a reference to result from the function. There is no way
/// we can specify lifetime parameters that would change the dangling reference, and Rust won't let
/// us create a dangling reference. In this case, the best fix would be to return an owned data
/// type rather than a reference so the calling function is then responsible for cleaning up the value.
///
/// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return
/// values of functions. Once they're connected, Rust has enough information to allow memory-safe
/// operations and disallow operations that would create dangling pointers or otherwise violate
/// memory safety.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

enum AttackType {
    Light,
    Normal,
    Heavy,
    ChargedHeavy
}

fn get_damage(at: &AttackType) -> u32 {
    match at {
        AttackType::Light => 10,
        AttackType::Normal => 25,
        AttackType::Heavy => 50,
        AttackType::ChargedHeavy => 100,
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    let first_attack = AttackType::ChargedHeavy;
    println!("The longest string is {result}");
    println!("I will do {} damage to you", get_damage(&first_attack));
}
