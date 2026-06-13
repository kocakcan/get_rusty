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
///
/// In a struct or enum, the data in the struct fields and the behaviour in impl blocks are
/// separaterd, whereas in other Languages, the data and behaviour combined into one concept is
/// often labeled an object. Trait objects differ from objects in other languages in that we can't
/// add data to a trait object. Trait objects aren't as generally as objects in other languages:
/// Their specific purpose is to allow abstraction across common behaviour.
///
///     pub trait Draw {
///         fn draw(&self);
///     }
///     Listing 18-3: Definition of the Draw trait
/// Listing 18-4 defines a struct named Screen that holds a vector named components. This vector is
/// of type Box<dyn Draw>, which is a trait object; it's a stand-in for any type inside a Box that
/// implements the Draw trait.
///
///     pub struct Screen {
///         pub components: Vec<Box<dyn Draw>>,
///     }
///     Listing 18-4: Definition of the Screen struct with a component field holding a vector of
///     trait objects that implement the Draw trait
///
///     impl Screen {
///         pub fn run(&self) {
///             for component in self.components.iter() {
///                 component.draw();
///             }
///         }
///     }
///     Listing 18-5: A run method on Screen that calls the draw method on each component
/// This works differently from defining a struct that uses a generic type parameter with trait
/// bounds. A generic type parameter can be substituted with only one concrete type at a time,
/// whereas trait objects allow for multiple concrete types to fill in for the trait object at
/// runtime. For example, we could have defined the Screen struct using a generic type and a trait
/// bound, as in Listing 18-6:
///
///     pub struct Screen<T: Draw> {
///         pub components: Vec<T>,
///     }
///
///     impl<T> Screen<T>
///     where
///         T: Draw,
///     {
///         pub fn run(&self) {
///             for component in self.components.iter() {
///                 component.draw();
///             }
///         }
///     }
///     Listing 18-6: An alternate implementation of the Screen struct and its run method using
///     generics and trait bound
/// This restricts us to a Screen instance that has a list of components all of
/// type Button or all of type TextField. If you'll only ever have homogenous collections, using
/// generics and trait bounds is preferable because the definitions will be monomorphized at compile
/// time to use the concrete types.
///
/// On the other hand, with the method using trait objects, one Screen instance can hold a Vec<T>
/// that contains a Box<Button> as well as a Box<TextField>.
///
/// Implementing the Trait
///
///     pub struct Button {
///         pub width: u32,
///         pub height: u32,
///         pub label: String,
///     }
///
///     impl Draw for Button {
///         fn draw(&self) {
///             // code to actually draw a button
///         }
///     }
///     Listing 18-7: A Button struct that implements the Draw trait
/// The width, height, and label fields on Button will differ from the fields on other components;
/// For example, a TextField type might have those same fields plus a placeholder field. Each of the
/// types we want to draw on the screen will implement the Draw trait but will use different code in
/// the draw method to define how to draw that particular type, as Button has here. The Button type,
/// for instance, might have an additional impl block containing methods related to what happpens
/// when a user click the button.
///
///     use gui::Draw;
///
///     struct SelectBox {
///         width: u32,
///         height: u32,
///         options: Vec<String>
///     }
///
///     impl Draw for SelectBox {
///         fn draw(&self) {
///             // code to actually draw a select box
///         }
///     }
///     Listing 18-8: Another crate using gui and implementing trait on a SelectBox struct
///
/// Using the Trait
///
/// To the Screen instance, they can add a SelectBox and a Button by putting each in a Box<T> to
/// become a trait object. They can then call the run method on the Screen instance, which will call
/// draw on each of the components.
///
///     use gui::{Button, Screen};
///
///     fn main() {
///         let screen = Screen {
///             components: vec![
///                 Box::new(SelectBox {
///                     width: 75,
///                     height: 10,
///                     options: vec![
///                         String::from("Yes"),
///                         String::from("Maybe"),
///                         String::from("No"),
///                     ],
///                 }),
///                 Box::new(Button {
///                     width: 50,
///                     height: 10,
///                     label: String::from("OK"),
///                 }),
///             ],
///         };
///
///         screen.run();
///     }
///     Listing 18-9: Using trait objects to store values of different types that implement the same
///     trait
/// The advantage of using trait objects and Rust's type system to write code similar to code using
/// duck typing is that we never have to check whether a value implements a particular method at
/// runtime or worry about getting errors if a value doesn't implement a method but we call it
/// anyway. Rust won't compiler our code if the values don't implement the traits that the trait
/// objects need.
///
/// Trait Objects and Type Inference
///
/// One downside to using trait objects is how they interact with type inference. For example,
/// consider type inference for Vec<T>. When T is not a trait object, Rust just needs to know the
/// type of a single element in the vector to infer T. So an empty vector causes a type inference
/// error:
///
///     let v = vec![],
///     // error: type annotations needed for `Vec<T>`
/// But adding an element enables Rust to infer the type of the vector.
/// Type inference is trickier for trait objects. For example, say we tried to factor the components
/// array in Listing 18-9 into a separable variable, like this:
///
///     fn main() {
///         let components = vec![
///             Box::new(SelectBox { /* .. */ }),
///             Box::new(Button { /* .. */ }),
///         ];
///         let screen = Screen { components };
///         screen.run();
///     }
///     Listing 18-11: Factoring the components array causes a type error
/// The compiler understands that the components vector must have the type Vec<Box<dyn Draw>>
/// because that's specified in the Screen struct definition. But in Listing 18-11, the compiler
/// loses that information at point where components is defined. To fix the issue, you have to give
/// a hint to the type inference algorithm. That can either be via an explicit cast on any elements
/// of the vector, like this:
///
///     let components = vec![
///         Box::new(SelectBox { /* .. */ }) as Box<dyn Draw>,
///         Box::new(SelectBox { /* .. */ }),
///     ]
/// Or it can be via a type annotation on the let-binding, like this:
///
///     let components: Vec<Box<dyn Draw>> = vec![
///         Box::new(SelectBox { /* .. */ }),
///         Box::new(SelectBox { /* .. */ }),
///     ]
/// Performing Dynamic Dispatch
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
