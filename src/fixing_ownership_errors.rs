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
*
* Fixing an Unsafe Program: Aliasing and Mutating a Data Structure
*
* Another unsafe operation is using a reference to heap data that gets deallocated by another
* alias. For example, here's a function that gets a reference to the largest string in a vector,
* and then uses it while mutating the vector:
*
*   fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
*       -> *dst         |   RW
*       -> dst          |   RO
*       -> *src         |   R
*       -> (*src)[_]    |   R
*
*       let largest: &String =
*           dst.iter().max_by_key(|s| s.len()).unwrap();    -> iter() and len() requires R
*       -> *dst         |   R
*       -> largest      |   RO
*       -> *largest     |   R
*
*       for s in src {
*       -> src          |   No permissions
*       -> *src         |   No permissions
*       -> (*src)[_]    |   No permissions
*       -> s            |   RO
*       -> *s           |   R
*
*           if s.len() > largest.len() {
*               dst.push(s.clone());    -> push() requires RW
*                                       -> clone() requires R
*           }
*       }
*   }
* This program is rejected by the borrow checker because let target = .. removes the W permission
* on dst. However, dst.push(..) requires the W permission. Again, we should ask: why is this
* program unsafe? Because dst.push(..) could deallocate the contents of dst, invalidating the
* reference largest.
*
* To fix this program, the key insight is that we need to shorten the lifetime of largest to not
* overlap with dst.push(..). One possibility is to clone largest:
*
*   fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
*       let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
*       for s in src {
*           dst.push(s.clone());
*       }
*   }
* However, this may cause a performance hit for allocating and copying the string data.
*
* Another possibility is to perform all the length comparisons first, and then mutate dst
* afterwards:
*
*   fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
*       let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
*       let to_add: Vec<String> =
*           src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
*       dst.extend(to_add);
*   }
* However, this also causes a performance hit for allocating the vector to_add.
*
* A final possibility is to copy out the length of largest, since we don't actually need the
* contents of largest, just its length. This solution is arguably the most idiomatic and the most
* performant:
*
*   fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
*       let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
*       for s in src {
*           if s.len() > largest_len {
*               dst.push(s.clone());
*           }
*       }
*   }
* These solutions all share in common the key idea: shortening the lifetime of borrows on dst to
* not overlap with a mutation to dst.
*
* Fixing an Unsafe Program: Copying vs. Moving Out of a Collection
*
* A common confusion for Rust learners happens when copying data out of a collection, like a
* vector:
*
*   let v: Vec<i32> = vec![0, 1, 2];
*
*   -> v | RO
*
*   let n_ref: &i32 = &v[0];
*
*   -> n_ref    | RO
*   -> *n_ref   | R
*   -> v        | No permissions
*
*   let n: i32 = *n_ref;    -> this operation requires R permission
*
*   -> n_ref    | No permissions
*   -> *n_ref   | No permissions
* The dereference operation *n_ref expects just the R permission, which the path *n_ref has. But
* what happens if we change the type of elements in the vector from i32 to String? Then it turns
* out we no longer have the necessary permissions:
*
*   let v: Vec<String> =
*       vec![String::from("Hello world")];
*
*   -> v        | RO
*
*   let s_ref: &String = &v[0];             -> requires R permission (referencing)
*
*   -> s_ref    | RO
*   -> *s_ref   | R
*   -> v        | No permissions
*
*   let s: String = *s_ref;                 -> requires both R and O permissions 😱
*   drop(s);                                -> These drops are normally implicit
*   drop(v);
* The first program will compile, but the second program will not compile. Rust gives the following
* error message: cannot move out of '*s_ref' which is behind a shared reference.
*
* The issue is that the vector v owns the string "Hello world". When we dereference s_ref, that
* tries to take ownership of the string from the vector. But references are non-owning pointers --
* we can't take ownership through a reference. Therefore Rust complains that we "cannot move out of
* [...] s shared reference".
*
* But why is this unsafe? What happens here is a double-free. After executing let s = *s_ref, both
* v and s think they own "Hello world". After s is dropped, "Hello world" is deallocated. Then v is
* dropped, and undefined behaviour happens when the string is freed a second time.
*
* Note: after executing s = *s_ref, we don't even have to use v or s to cause undefined behaviour
* through the double-free. As soon as we move the string out from s_ref, undefined behaviour will
* happen once the elements are dropped.
*
* However, this undefined behaviour does not happen when the vector contains i32 elements. This
* difference is that copying a String copies a pointer to heap data. Copying an i32 does not. In
* technical terms, Rust says that the type i32 implements the Copy trait, while String does not
* implement Copy.
*
* In sum, if a value does not own heap data, then it can be copied without a move. For example:
* - An i32 does not own heap data, so it can be copied without a move.
* - A String does own heap data, so it can not be copied without a move.
* - An &String does not own heap data, so it can be copied without a move.
*
* Note: One exception to this rule is mutable references. For example, &mut i32 is not a copyable
* type. So if you do something like:
*
*   let mut n = 0;
*   let a = &mut n;
*   let b = a;
* Then a cannot be used after being assigned to b. That prevents two mutable references to the same
* data from being used at the same time.
*
* So if we have a vector of non-Copy types like String, then how do we safely get access to an
* element of the vector? Here's a few different ways to safely do so. First, you can avoid taking
* ownership of the string and just use an immutable reference:
*
*   let v: Vec<String> = vec![String::from("Hello world")];
*   let s_ref: &String = &v[0];
*   println!("{s_ref}");
* Second, you can clone the data if you want to get ownership of the string while leaving the
* vector alone:
*
*   let v: Vec<String> = vec![String::from("Hello world")];
*   let mut s: String = v[0].clone();
*   s.push('!');
*   println!("{s}");
* Finally, you can use a method like Vec::remove to move the string out of the vector:
*
*   let mut v: Vec<String> = vec![String::from("Hello world")];
*   let mut s: String = v.remove(0);
*   s.push('!');
*   println!("{s}");
*   assert!(v.len() == 0);
*
* Fixing a Safe Program: Mutating Different Tuple Fields
*
* The above examples are where a program is unsafe. Rust may also reject safe programs. One common
* issue is that Rust tries to track permissions at a fine-grained level. However, Rust may conflate
* two different places as the same place.
*
* This program shows how you can borrow one field of a tuple, and write to a different field of the
* same tuple:
*
*   let mut name = {
*       String::from("Ferris"),
*       String::from("Rustacean")
*   };
*
*   -> name     | RWO
*   -> name.0   | RWO
*   -> name.1   | RWO
*
*   let first = &name.0;        -> & requires R permission
*
*   -> name     | R
*   -> name.0   | R
*   -> first    | RO
*   -> *first   | R
*
*   name.1.push_str(", Esq.");  -> push_str requires RW
*   println!("{first} {}", name.1);
*   -> name     | No permissions
*   -> name.0   | No permissions
*   -> name.1   | No permissions
*   -> first    | No permissions
*   -> *first   | No permissions
* The statement let first = &name.0 borrows name.0. This borrow removes WO permissions from name.0.
* It also removes WO permissions from name. (For example, one could not pass name to a function
* that takes as input value of type (String, String). But name.1 still retains the W permission, so
* doing name.1.push_str(...) is a valid operation.
*
* However, Rust can lose track of exactly which places are borrowed. For example, let's say we
* refactor the expression &name.0 into a function get_first. Notice how after calling
* get_first(&name), Rust now removes the W permission on name.1:
*
*   fn get_first(name: &(String, String)) -> &String {
*       &name.0
*   }
*
*   fn main() {
*       let mut name = {
*           String::from("Ferris"),
*           String::from("Rustacean")
*       };
*
*       let first = get_first(&name);
*       name.1.push_str(", Esq.");
*       println!("{first} {}", name.1);
*   }
* Now we can't do name.1.push_str(..)! Rust will return this error: cannot borrow 'name.1' as
* mutable because it is also borrowed as immutable.
*
* That's strange since the program was safe before we edited it. That edit we made doesn't
* meaningfully change the runtime behaviour. So why does it matter that we put &name.0 into a
* function?
*
* The problem is that Rust doesn't look at the implementation of get_first when deciding what
* get_first(&name) should borrow. Rust only looks at the type signature, which just says "some
* String in the input gets borrowed". Rust conservatively decides then both name.0 and name.1 get
* borrowed, and eliminates write and own permissions on both.
*
* Remember, the key idea is that the program above is safe. It has no undefined behaviour!
*
* Fixing a Safe Program: Mutating Different Array Elements
*
* A similar kind of problem arises when we borrow elements of an array. For example, observe what
* places are borrowed when we take a mutable reference to an array:
*
*   let mut a = [0, 1, 2, 3];
*
*   -> a    | RWO
*   -> a[_] | RW
*
*   let x = &mut a[1];
*
*   -> a[_] | No permissions
*   -> a    | No permissions
*   -> x    | RO
*   -> *x   | RW
*
*   *x += 1;    -> requires RW
*
*   -> a    | RW
*   -> x    | No permissions
*   -> *x   | No permissions
*
*   println!("{a:?}");
*
*   -> a    | No permissions
*   -? a[_] | No permissions
* Rust's borrow checker does not contain different places for a[0], a[1], and so on. It uses a
* single place a[_] that represents all indexes of a. Rust does this because it cannot always
* determine the value of an index.
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

// fn stringify_name_with_title(name: &mut Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

// fn main() {
//     let mut names: Vec<String> = vec![String::from("Ferris"), String::from("Jr.")];
//     let stringified_name = stringify_name_with_title(&mut names);
//     println!("My name is {stringified_name}");
// }

/* largest removes W permission from dst and then we're trying to perform push() on dst
 * which dst doesn't have the permission for it. */
// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }
