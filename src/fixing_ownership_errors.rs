/*
* Fixing Ownership Errors
*
* Rust will always rejects an unsafe program. But sometimes, Rust will also reject a safe program.
*
* Fixing an Unsafe Program: Returning a Reference to the Stack
*
*   fn return_a_string() -> &String {
*       let s = String::from("Hello world");
*       &s
*   }
* Here, the issue is with the lifetime of the referred data. If you want to pass around a reference
* to a string, you have to make sure that the underlying string lives long enough.
*
* Depending on the situation, here are four ways you can extend the lifetime of the string. One is
* to move ownership of the string out of the function, changing &String to String:
*
*   fn return_a_string() -> String {
*       let s = String::from("Hello world");
*       s
*   }
* Another possibility is to return a string literal, which lives forever (indicated by 'static).
* This solution applies if we never intend to change the string, and then a heap allocation is
* unnecessary:
*
*   fn return_a_string() -> &'static str {
*       "Hello world"
*   }
* Another possibility is to defere borrow-checking to runtime by using garbage collection. For
* example, you can use a reference-counted pointer:
*
*   use std::rc::Rc;
*   fn return_a_string() -> Rc<String> {
*       let s = Rc::new(String::from("Hello world"));
*       Rc::clone(&s)
*   }
* In short, Rc::clone only clones a pointer to s and not the data itself. At runtime, the Rc checks
* when the last Rc pointing to data has been dropped, and then deallocates the data.
*
* Yet another possibility is to have the caller provide a "slot" to put the string using a mutable
* reference:
*
*   fn return_a_string(output: &mut String) {
*       output.replace_range(.., "Hello world");
*   }
* With this strategy, the caller is responsible for creating space for the string. This style can
* be verbose, but it can also be more memory-efficient if the caller needs to carefully control
* when allocations occur.
*
* Which strategy is most appropriate will depend on your application. But the key idea is to
* recognize the root issue underlying the surface-level ownership error. How long should my string
* live? Who should be in charge of deallocating it? Once you have a clear answer to those
* questions, then it's a matter of changing your API to match.
*
* Fixing an Unsafe Program: Not Enough Permissions
*
* Another common issue is trying to mutate read-only data, or tyring to drop data behind a
* reference. For example, let's say we tried to write a function stringify_name_with_title. This
* function is supposed to create a person's full name from a vector of name parts, including an
* extra title.
*
*   fn stringify_name_with_title(name: &Vec<String>) -> String {
*       -> name     | RO
*       -> *name    | R
*
*       name.push(String::from("Esq."));    -> name requires RW permissions
*       let full = name.join(" ");
*       full
*   }
* This program is rejected by the borrow checker because name is an immutable reference, but
* name.push(..) requires the W permission. This program is unsafe because push could invalidate
* other references to name outside of stringify_name_with_title, like this:
*
*   fn main() {
*       let name = vec![String::from("Ferris")];
*       let first = &name[0];               -> L1
*       stringify_name_with_title(&name);   -> L2
*       println!("{}", first);              -> L3
*   }
* In this example, a reference first to name[0] is created before calling
* stringify_name_with_title. The function name.push(..) reallocates the contents of name, which
* invalidates first, causing the println to read deallocated memory.
*
* So how do we fix this API? One straightforward solution is to change the type of name form
* &Vec<String> to &mut Vec<String>:
*
*   fn stringify_name_with_title(name: &mut Vec<String>) -> String {
*       name.push(String::from("Esq."));
*       let full = name.join(" ");
*       full
*   }
* But this is not a good solution! Functions should not mutate their inputs if the caller would not
* expect it. A person calling stringify_name_with_title probably does not expect their vector to be
* modified by this function. Another function like add_title_to_name might be expected to mutate
* its input, but not our function.
*
* Another option is to take ownership of the name, by changing &Vec<String> to Vec<String>:
*
*   fn stringify_name_with_title(mut name: Vec<String>) -> String {
*       name.push(String::from("Esq."));
*       let full = name.join(" ");
*       full
*   }
* But this is also not a good solution! It is very rare for Rust functions to take ownership of
* heap-owning data structures like Vec and String. This version of stringify_name_with_title would
* make the input name unusable, which is very annoying to a caller.
*
* So the choice &Vec is actually a good one, which we do not want to change. Instead, we can
* change the body of the function. There are many possible fixes which vary in how much memory they
* use. One possibility is to clone the input name:
*
*   fn stringify_name_with_title(name: &Vec<String>) -> String {
*       let mut name_clone = name.clone();
*       name_clone.push(String::from("Esq."));
*       let full = name_clone.join(" ");
*       full
*   }
* By cloning name, we are allowed to mutate the local copy of the vector. However, this clone
* copies every string in the input. We can avoid unnecessary copies by adding the suffix later:
*
*   fn stringify_name_with_title(name: &Vec<String>) -> String {
*       let mut full = name.join(" ");
*       full.push_str(" Esq.");
*       full
*   }
* This solution works because slice::join already copies the data in name into the string full.
*
* In general, writing Rust functions is a careful balance of asking for the right level of
* permissions. For this example, it's most idiomatic to only expect the read permission on name.
*/

// Missing lifetime specifier
// fn return_a_string() -> &String {
//     &String::from("Hello world")
// }

// Returns a string itself, thereby moving the ownership out of the function
// fn return_a_string() -> String {
//     String::from("Hello world")
// }

// Returning a string literal, which lives indefinitely but it cannot be changed
// Use it if heap allocation is unnecessary
// fn return_a_string() -> &'static str {
//     "Hello world"
// }

// Return the cloned pointer
// use std::rc::Rc;
// fn return_a_string() -> Rc<String> {
//     let s = Rc::new(String::from("Hello world"));
//     Rc::clone(&s)
// }

// Have the caller provide a slot to put the string using a mutable reference
// fn return_a_string(output: &mut String) {
//     output.replace_range(.., "Hello world");
// }

fn stringify_name_with_title(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

fn main() {
    let mut names: Vec<String> = vec![String::from("Ferris"), String::from("Jr.")];
    let stringified_name = stringify_name_with_title(&mut names);
    println!("My name is {stringified_name}");
}
