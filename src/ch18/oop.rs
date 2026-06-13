/// Characteristics of Object-Oriented Languages
///
/// Object-oriented programs are made up of objects. An object packages both data and the procedures
/// that operate on that data. The procedures are typically called methods or operations.
///
/// Using this definition, Rust is object oriented. Structs and enums have data, and impl blocks
/// provide methods on structs and enums. Even though structs and enums with methods aren't called
/// objects, they provide the same functionality.
///
/// Encapsulation That Hides Implementation Details
///
/// Another aspect commonly associated with OOP is the idea of encapsulation, which means that the
/// implementation details of an object aren't accessible to code using that object. Therefore, the
/// only way to interact with an object is through its public API; code using the object shouldn't
/// be able to reach into object's internals and change data or behaviour directly. This enables the
/// programmer to change and refactor an object's internals without needing to change the code that
/// uses the object.
///
/// We can use the pub keyword to decide which modules, types, functions, and methods in our code
/// should be public, and by default everything is private.
///
///     pub struct AveragedCollection {
///         list: Vec<i32>,
///         average: f64,
///     }
///     Listing 18-1: An AveragedCollection struct that maintains a list of integers and the average
///     of the items in the collection
/// The struct is marked pub so that other code can use it, but the fields within the struct
/// remain private. This is important in this case because we want to ensure that whenever a value
/// is added or removed from the list, the average is also updated. We do this by implementing add,
/// remove, and average methods on the struct, as shown in Listing 18-2:
///
///     impl AveragedCollection {
///         pub fn add(&mut self, value: i32) {
///             self.list.push(value);
///             self.update_average();
///         }
///
///         pub fn remove(&mut self) -> Option<i32> {
///             let result = self.list.pop();
///             match result {
///                 Some(value) => {
///                     self.update_average();
///                     Some(value)
///                 }
///                 None => None,
///             }
///         }
///
///         pub fn average(&self) -> f64 {
///             self.average
///         }
///
///         fn update_average(&mut self) {
///             let total: i32 = self.list.iter().sum();
///             self.average = total as f64 / self.list.len() as f64;
///         }
///     }
///     Listing 18-2: Implementations of the public methods add, remove, and average on
///     AveragedCollection
/// The public methods add, remove, and average are the only ways to access or modify data in an
/// instance of AveragedCollection. When an item is added to list using the add method or removed
/// using the remove method, the implementations of each call the private update_average method that
/// handles updating the average field as well.
///
/// We leave the list and average fields private so that there is no way for external code to add or
/// remove items to or from to list field directly; Otherwise, the average field might become out of
/// sync when the list changes. The average method returns the value in the average field, allowing
/// external code to read the average but not modify it.
///
/// Becase we've encapsulated the implementation details of the struct AveragedCollection, we can
/// easily change aspects, such as the data structure, in the future. For instance, we could use a
/// HashSet<i32> instead of a Vec<i32> for the list field. As long as the signatures of the add,
/// remove, and average public methods stayed the same, code using AveragedCollection wouldn't need
/// to change. If we made list public instead, this wouldn't necessarily be the case: HashSet<i32>
/// and Vec<i32> have different methods for adding and removing items, so the external code would
/// likely have to change if it were modifying list directly.
///
/// If encapsulation is a required aspect for a language to be considered object oriented, then Rust
/// meets the requirement. The option to use pub or not for different parts of code enables
/// encapsulation of implementation details.
///
/// Defining a Trait for Common Behaviour
///
/// A trait object points to both and instance of a type implementing our specified trait and a
/// table used to look up trait methods on that type at runtime. We create a trait object by
/// specifying some sort of pointer, such as a reference or a Box<T> smart pointer, then the dyn
/// keyword, and then specifying the relevant trait. We can use trait objects in place of a generic
/// or concrete type. Wherever we use a trait object, Rust's type system will ensure at compile time
/// that any value used in that context will implement the trait object's trait. Consequently, we
/// don't need to know all the possible types at compile time.
use std::fmt;

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

impl fmt::Display for AveragedCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List: {:?}\nAverage: {}", self.list, self.average)
    }
}

fn main() {
    let mut coll = AveragedCollection {
        list: (0..10).collect::<Vec<_>>(),
        average: 5.5,
    };
    coll.add(50);
    println!("{}", coll);
    coll.remove();
    println!("{}", coll);
}
