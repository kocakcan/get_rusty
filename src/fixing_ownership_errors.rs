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
