// Treating Smart Pointers Like Regular References
//
// Implementing the Deref trait allows you to customize the behaviour of the dereference operator *
// (not to be confused with the multiplication or glob operator). By implementing Deref in such a
// way that a smart pointer can be treated like a regular reference, you can write code that
// operates on references and use that code with smart pointers too.
//
// Following the Reference to the Value
//
// A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a
// value stored somewhere else. In Listing 15-6, we create a reference to an i32 value and then use
// the dereference operator to follow the reference to the value.
//
//  fn main() {
//      let x = 5;
//      let y = &x;
//
//      assert_eq!(5, x);
//      assert_eq!(5, *x);
//  }
//  Listing 15-6: Using the dereference operator to follow a reference to an i32 value
// The variable x holds an i32 value 5. We set y equal to a reference to x. We can assert that x is
// equal to 5. However, if we want to make an assertion about the value in y, we have to use *y to
// follow the reference to the value it's pointing to (hence, dereference) so that the compiler can
// compare the actual value. Once we dereference y, we have access to the integer value y is
// pointing to that we can compare with 5.
//
// If we tried to write assert_eq!(5, y); Instead, we would get this compilation error:
//
//  error: can't compare `{integer}` with `&{integer}`
// Comparing a number and a reference to a number isn't allowed because they're different types. We
// must use the dereference operator to follow the reference to the value it's pointing to.
//
// Using Box<T> Like a Reference
//
// We can rewrite the code in Listing 15-6 to use a Box<T> instead of a reference; the dereference
// operator used on the Box<T> in Listing 15-7 functions in the same way as the dereference
// operator used on the reference in Listing 15-6.
//
//      fn main() {
//          let x = 5;
//          let y = Box::new(x);
//
//          assert_eq!(5, x);
//          assert_eq!(5, *y);
//      }
//      Listing 15-7: Using the dereference operator on a Box<i32>
// The main difference between Listing 15-7 and Listing 15-6 is that here we set y to be an
// instance of a box pointing to a copied value of x rather than a reference pointing to the value
// of x. In the last assertion, we can use the dereference operator to follow the box's pointer in
// the same way that we did when y was a reference.
//
// Defining Our Own Smart Pointer
//
// Let's build a wrapper type similar to the Box<T> type provided by the standard library to
// experience how smart pointer types behave differently from references by default.
//
//  Note: There's one big difference between the MyBox<T> type we're aboutt to build and the real
//  Box<T>: Our version will not store its data on the heap. We are focusing this example on Deref,
//  so where the data is actually stored is less important than the pointer-like behaviour.
// The Box<T> type is ultimately defined as a tuple struct with one element, so Listing 15-8
// defines a MyBox<T> type in the same way.
//
//      struct MyBox<T>(T);
//
//      impl<T> MyBox<T> {
//          fn new(x: T) -> MyBox<T> {
//              MyBox(x)
//          }
//      }
//      Listing 15-8: Defining a MyBox<T> type
// We define a struct named MyBox and declare a generic parameter T because we want our type to
// hold values of any type. The MyBox type is a tuple struct with one element of type T. The
// MyBox::new function takes one parameter of type T and returns a MyBox instance that holds the
// value passed in.
//
//      fn main() {
//          let x = 5;
//          let y = MyBox::new(x);
//
//          assert_eq!(5, x);
//          assert_eq!(5, *y);
//      }
//      Listing 15-9: Attempting to use MyBox<T> in the same way we used references and Box<T>
//
//      error: type `MyBox<{integer}>` cannot be dereferenced
// Our MyBox<T> type can't be dereferenced because we haven't implemented trait ability on our
// type. To enable dereferencing with the * operator, we implement the Deref trait.
//
// Implementing the Deref Trait
//
// The Deref trait, provided by the standard library, requires us to implement one method named
// deref that borrows self and returns a reference to the inner data. Listing 15-10 contains an
// implementation of Deref to add to the definition of MyBox<T>.
//
//      use std::ops::Deref;
//
//      impl<T> Deref for MyBox<T> {
//          type Target = T;
//
//          fn deref(&self) -> &Self::Target {
//              &self.0
//          }
//      }
//      Listing 15-10: Implementing Deref on MyBox<T>
// The type Target = T; syntax defines an associated type for the Deref trait to use. Associated
// types are a slightly different way of declaring a generic parameter.
//
// We fill in the body of the deref method with &self.0 so that deref returns a reference to the
// value we want to access with the * operator.
//
// Without the Deref trait, the compiler can only dereference & references. The deref method gives
// the compiler the ability to take a value of any type that implements Deref and call the deref
// method to get a reference that it knows how to dereference.
//
// When we entered *y in Listing 15-9, behind the scenes Rust actually ran this code:
//
//      *(y.deref())
// Rust substitutes the * operator with a call to the deref method and then a plain dereference so
// that we don't have to think about whether or not we need to call the deref method. This Rust
// feature lets us write code that functions identically whether we have a regular reference or a
// type that implements Deref.
//
// The reason the deref method returns a reference to a value, and that the plain dereference
// outside the parantheses in *(y.deref()) is still necessary, has to do with ownership system. If
// the deref method returned the value directly instead of a reference to the value, the value would
// be moved out of self. We don't want to take ownership of the inner value inside MyBox<T> in this
// case or in most cases where we use the dereference operator.
//
// Note that the * operator is replaced with a call to the deref method and then a call to the *
// operator just once, each time we use a * in our code. Because the substitution of the * operator
// does not recurse infinitely, we end up with data type of i32, which matches the 5 in the
// assert_eq! in Listing 15-9.
//
// Using Deref Coercion in Functions and Methods
//
// Deref coercion converts a reference to a type that implements the Deref trait into a reference to
// another type. For example, deref coercion can convert &String to &str because String implements
// the Deref trait such that it returns &str. Deref coercion is a convenience Rust performs on
// arguments to functions and methods, and it works only on types that implement the Deref trait. It
// happens automatically when we pass a reference to a particular type's value as an argument to a
// function or method definition. A sequence of calls to the deref method converts the type we
// provided into the type the parameter needs.
//
// Deref coercion was added to Rust so that programmers writing function and method calls don't need
// to add as many explicit references and dereferences with & and *. The deref coercion feature also
// lets us write code that can work for either references or smart pointers.
//
// To see deref coercion in action, let's use the MyBox<T> type we defined in Listing 15-8 as well
// as the implementation of Deref that we added in Listing 15-10. Listing 15-11 shows the definition
// of a function that has a string slice parameter.
//
//      fn hello(name: &str) {
//          println!("Hello, {name}");
//      }
//      Listing 15-11: A hello function that has a parameter name of type &str
// We can call the hello function with a string slice as an argument, such as hello("Rust");, for
// example. Deref coercion makes it possible to call hello with a reference to a value of type
// MyBox<String>, as shown in Listing 15-12.
//
//      fn main() {
//          let m = MyBox::new(String::from("Rust"));
//          hello(&m);
//      }
//      Listing 15-12: Calling hello with a reference to a MyBox<String> value, which works because
//      of deref coercion
// Here, we're calling the hello function with the argument &m, which is a reference to a
// MyBox<String> value. Because we implemented the Deref trait on MyBox<T> in Listing 15-10, Rust
// can turn &MyBox<String> into &String by calling deref. The standard library provides an
// implementation of Deref on String that returns a string slice, and this is in the API
// documentation for Deref. Rust calls deref again to turn the &String into &str, which matches the
// hello function's definition.
//
// If Rust didn't implement deref coercion, we would have to write the code in Listing 15-13 instead
// of the code in Listing 15-12 to call hello with a value of type &MyBox<String>.
//
//      fn main() {
//          let m = MyBox::new(String::from("Rust"));
//          hello(&(*m)[..]);
//      }
//      Listing 15-13: The code we would have to write if Rust didn't have deref coercion
// The (*m) dereferences the MyBox<String> into a String. Then, the & and [..] take a string slice
// of the String that is equal to the whole string to match the signature of hello. This code
// without deref coercions is harder to read, write, and understand with all of these symbols
// involved. Deref coercion allows Rust to handle these conversions for us automatically.
//
// When the Deref trait is defined for the types involved, Rust will analyze the types and use
// Deref::deref as many times as necessary to get a reference to match the parameter's type. The
// number of times that Deref::deref needs to inserted is resolved at compile time, so there is no
// runtime penalty for taking advantage of deref coercion!
//
// Handling Deref Coercion with Mutable References
//
// Similar to how you use the Deref trait to override the * operator on immutable references, you
// can use the DerefMut trait to override the * operator on mutable references.
//
// Rust deref coercion when it finds types and trait implementations in three cases:
//
//  1. From &T to &U when T: Deref<Target=U>
//  2. From &mut T to &mut U when T: DerefMut<Target=U>
//  3. From &mut T to &U when T: Deref<Target=U>
// The first two cases are the same except that the second implements mutability. The first case
// states that if you have a &T, and T implements Deref to some type U, you can get a &U
// transparently. The second case states that the same deref coercion happens for mutable references.
//
// The third case is trickier: Rust will also coerce a mutable reference to an immutable one. But
// the reverse is not possible: Immutable references will never coerce to mutable references.
// Because of the borrowing rules, if you have a mutable reference, that mutable reference must be
// the only reference to that data (otherwise, the program wouldn't compile). Converting one mutable
// reference to one immutable reference will never break the borrowing rules. Converting an
// immutable reference to a mutable reference would require that initial immutable reference is the
// only immutable reference to that data, but the borrowing rules don't guarantee that. Therefore,
// Rust can't make the assumption that converting an immutable reference to a mutable reference is possible.
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 5;
    let y = MyBox::new(5);
    let z = MyBox::new(String::from("Can"));

    assert_eq!(5, x);
    // assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));
    hello(&z);
}
