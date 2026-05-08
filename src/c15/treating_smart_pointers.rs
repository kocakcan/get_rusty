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
//
