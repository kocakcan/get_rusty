/*
* Bringing Paths into Scope with the use Keyword
*
* Having to write out the paths to call functions can feel inconvenient and repetitive. Before,
* whether we chose the absolute or relative path to the add_to_waitlist function, every time we
* wanted to call add_to_waitlist we had specify front_of_house and hosting too. Fortunately,
* there's a way to simplify this process: we can create a shortcut to a path with the use keyword
* once, and then use the shorter name everywhere else in the scope.
*
* Below, we bring the crate::front_of_house::hosting module into the scope of the eat_at_restaurant
* function so we only have to specify hosting::add_to_waitlist to call the add_to_waitlist function
* in eat_at_restaurant.
*
* Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. By
* adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that
* scope, just as though the hosting module had been defined in the crate root. Paths brought into
* scope with use also check privacy, like any other paths.
*
* Note that use only creates the shortcut for the particular scope in which the use occurs. Below
* moves the eat_at_restaurant function into a new child module named customer, which is then a
* different scope than the use statement, so the function body won't compile.
*
* The compiler error shows that the shortcut no longer applies within the customer module:
*
*   failed to resolve: use of undeclared crate or module `hosting`
* Notice there's also a warning that the use is no longer used in its scope! To fix this problem,
* move the use within the customer module too, or reference the shortcut in the parent module with
* super::hosting within the child customer module.
*
* Creating Idiomatic use Paths
*
* Above, you might have wondered why we specified use crate::front_of_house::hosting and then
* called hosting::add_to_waitlist in eat_at_restaurant, rather than specifying the use path all the
* way out to the add_to_waitlist function to achieve the same result as below.
*
* Although they both accomplish the same task, above is the idiomatic way to bring a function into
* scope with use. Bringing the function's parent into scope with use means we have to specify the
* parent module when calling the function. Specifying the parent module when calling the function
* makes it clear that the function isn't locally defined while still minimizing repetition of the
* full path. The code above is unclear as to where add_to_waitlist is defined.
*
* On the other hand, when bringing in structs, enums, and other items with use, it's idiomatic to
* specify the full path. Below shows the idiomatic way to bring the standard library's HashMap
* struct into the scope of a binary crate.
*
*   use std::collections::HashMap;
*
*   fn main() {
*       let mut map = HashMap::new();
*       map.insert(1, 2);
*   }
* There's no strong reason behind this idiom: it's just the convention that has emerged, and folks
* have gotten used to reading and writing Rust code this way.
*
* The exception to this idiom is if we're bringing two items with the same name into scope with use
* statements, because Rust doesn't allow that. Below shows how to bring tow Result types into scope
* that have the same name but different parent modules, and how to refer them.
*
*   use std::fmt;
*   use std::io;
*
*   fn function1() -> fmt::Result {
*       // --snip--
*   }
*
*   fn function2() -> io::Result<()> {
*       // --snip--
*   }
* As you can see, using the parent modules distinguishes the two Result types. If instead we
* specified use std::fmt::Result and use std::io::Result, we'd have two Result types in the same
* scope, and Rust wouldn't know which one we meant when we used Result.
*
* Providing New Names with the as Keyword
*
* There's another solution to the problem of bringing two types of the same name into the same
* scope with use: after the path, we can specify as and a new local name, or alias, for the type.
* Below shows another way to write the code above by renaming one of the two Result types using as.
*
*   use std::fmt::Result;
*   use std::io::Result as IoResult;
*
*   fn function1() -> Result {
*       // --snip--
*   }
*
*   fn function2() -> IoResult<()> {
*       // --snip--
*   }
* In the second use statement, we chose the new name IoResult for the std::io::Result type, which
* won't conflict with the Result from std::fmt that we've also brought into scope. They are both
* considered idiomatic, so the choice is up to you!
*
* Re-exporting Names with pub use
*
* When we bring a name into scope with the use keyword, the name is private to the scope into which
* we imported it. To enable code outside that scope to refer to that names as if it had been
* defined in that scope, we can combine pub and use. This technique is called re-exporting because
* we're bringing an item into scope but also making that item available for others to bring into
* their scope.
*
* Below shows the code above in the root module changed to pub use.
*
*   mod front_of_house {
*       pub mod hosting {
*           pub fn add_to_waitlist() {}
*       }
*   }
*
*   pub use crate::front_of_house::hosting;
*
*   pub fn eat_at_restaurant() {
*       hosting::add_to_waitlist();
*   }
* Before this change, external code would have to call the add_to_waitlist function by using the
* path restaurant::front_of_house::hosting::add_to_waitlist(), which also would have required the
* front_of_house module to be marked as pub. Now that this pub use has re-exported the hosting
* module from the root module, external code can use the path
* restaurant::hosting::add_to_waitlist() instead.
*
* Re-exporting is useful when the internal structure of your code is different from how programmers
* calling your code would think about the domain. For example, in this restaurant metaphor, the
* people running the restaurant think about "front of house" and "back of house". But customers
* visiting a restaurant probably won't think about the parts of the restaurant in those terms. With
* pub use, we can write our code with one structure but expose a different structure. Doing so
* makes our library well organized for programmers working on the library and programmers calling
* the library.
*
* Using External Packages
*
* To bring definitions into the scope of our package, we added a use line starting with the name of
* the crate, rand, and listed the items we wanted to bring into scope.
*
*   use rand::Rng;
*
*   fn main() {
*       let secret_number = rand::thread_rng().gen_range(1..=100);
*   }
* Note that the standard std library is also a crate that's external to our package. Because the
* standard library is shipped with the Rust language, we don't need to change Cargo.toml to include
* std. But we do need to refer to it with use to bring items from there into our package's scope.
* For example, with HashMap we would use this line:
*
*   use std::collections::HashMap;
* This is an absolute path starting with std, the name of the standard library crate.
*
* Using Nested Paths to Clean Up Large use Lists
*
* If we're using multiple items defined in the same crate or same module, listing each item on its
* own line can take up a lot of vertical space in our files. For example, these two use statements
* we had in the guessing game.
*
*   // --snip--
*   use std::cmp::Ordering;
*   use std::io;
*   // --snip--
* Instead, we can use nested paths to bring the same items into scope in one line. We do this by
* specifying the common part of the path, followed by the two colons, and then curly brackets
* around a list of the parts of the paths that differ, as shown below.
*
*   // --snip--
*   use std::{cmp::Ordering, io};
*   // --snip--
* In bigger programs, bringing many items into scope from the same crate or module using nested
* paths can reduce the number of separete use statements needed by a lot!
*
* We can use a nested path at any level in a path, which is useful when combining two use
* statements that share a subpath. For example, below shows two use statements: one that brings
* std::io into scope and one that brings std::io::Write into scope.
*
*   use std::io;
*   use std::io::Write;
* The common part of these two paths is std::io, and that's the complete first path. To merge these
* two paths into one use statement, we can use self in the nested path, as shown below.
*
*   use std::io::{Self, Write};
* This line brings std::io and std::io::Write into scope.
*
* The Glob Operator
*
* If we want to bring all public items defined in a path into scope, we can specify that path
* followed by the * glob operator:
*
*   use std::collections::*;
* This use statement brings all items defined in std::collections into the current scope. Be
* careful when using the glob operator! Glob can make it harder to tell what names are in scope and
* where a name used in your program was defined. Additionally, if the dependency changes its
* definitions, what you've imported changes as well, which may lead to compiler errors when you
* upgrade the dependency if the dependency adds a definition with the same name as a definition of
* yours in the same scope, for example.
*
* The glob operator is often used when testing to bring everything under test into the tests module.
*/
