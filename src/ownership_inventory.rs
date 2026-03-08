/// Makes a string to separate lines of text,
/// returning a default if the provided string is blank
// fn make_separator(user_str: &str) -> &str {
//     if user_str == "" {
//         let default = "=".repeat(10);
//         &default
//     } else {
//         user_str
//     }
// }

/* Above function won't compile as we cannot return reference to local variable default
* If we pass an empty string to this function and assuming it would compile we would end up with a
* dangling pointer */

fn make_separator(user_str: &str) -> String {
    if user_str == "" {
        let default = "=".repeat(10);
        default
    } else {
        user_str.to_string()
    }
}

/* Context: There is no way to return a pointer to a stack-allocated variable. The simple solution
* is therefore to change the return type to String and copy the input user_str into an owned
* string. However, requiring user_str to be a String would reduce the flexibility of the API, e.g.,
* a caller could not call make_separator on a substring of a bigger string. It would also require
* callers to heap-allocate strings, e.g., they could not use a string literal like
* make_separator("Galatasaray"). */

/// Gets the string out of an option if it exists,
/// returning a default otherwise
// fn get_or_default(arg: &Option<String>) -> String {
//     if arg.is_none() {
//         return String::new();
//     }
//     let s = arg.unwrap();
//     s.clone()
// }

/* Context: The function Option::unwrap expects self, meaning it expects ownership of arg. However
* arg is an immutable reference to an option, so it cannot provide ownership of the option.
* Therefore the compiler complains that we cannot move out of arg via unwrap. */

/* Calling the get_or_default() with following causes undefined behaviour:
*
* let opt = Some(String::from("Galatasaray"));
* get_or_default(&opt);
*
* let opt = Some(String::from("Galatasaray"));
* get_or_default(&opt);
* println!("{:?}", opt);
*
* let opt = Some(String::from("Galatasaray"));
* let s = get_or_default(&opt);
* println!("{}", s);
*
* Context: All of these programs violate memory safety due to a double-free. If arg.unwrap() were
* permitted, then ownership of the string String::from("Galatasaray") would be assumed by s. After
* get_or_default returns, then the string would be freed. However, opt also believes it owns the
* string, so the string would be freed a second time on behalf of opt.
*/

fn get_or_default(arg: &Option<String>) -> String {
    match arg {
        None => String::new(),
        Some(s) => s.clone(),
    }
}

/* Context: The combination of is_none and unwrap here is a Rust anti-pattern, since a match
* continues the two functionalities and automatically deals with pushing the reference &Option into
* the interior to produce &String. Therefore, the match soltuion is the most idiomatic, and passes
* the compiler without changing the intended type signature of the function.
*
* The solution of changing &Option is not desirable because it requires the caller to provide
* ownership of their option, which is a far more restrictive API */
