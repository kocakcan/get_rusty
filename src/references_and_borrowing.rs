/*
 * References and Borrowing
 *
 * Ownership, boxes, and moves provide a foundation for safely programming with the heap. However,
 * move-only APIs can be inconvenient to use. For example, say you want to read some strings twice:
 *
 *  fn main() {
 *      let m1 = String::from("Hello");
 *      let m2 = String::from("world");
 *      greet(m1, m2)                       -> L2
 *      let s = format!("{} {}", m1, m2)    -> L3 (Error: m1 and m2 are moved)
 *  }
 *
 *  fn greet(g1: String, g2: String) {
 *      println!("{} {}!", g1, g2);         -> L1
 *  }
 * In this example, calling greet moves the data from m1 and m2 into the parameters of greet. Both
 * strings are dropped at the end of greet, and therefore cannot be used within main. If we try to
 * read them like in the operation format!(...), then that would be undefined behaviour. The Rust
 * compiler therefore rejects this program with the error saying "borrow of moved value: m1".
 *
 * This move behaviour is extremely inconvenient. Programs often need to use a string more than
 * once. An alternative greet could return ownership of the strings like this:
 *
 *  fn main() {
 *      let m1 = String::from("Hello");
 *      let m2 = String::from("world");                 -> L1
 *      let (m1_again, m2_again) = greet(m1, m2);
 *      let s = format!("{} {}", m1_again, m2_again);   -> L2
 *  }
 *
 *  fn greet(g1: String, g2: String) -> (String, String) {
 *      println!("{} {}!", g1, g2);
 *      (g1, g2)
 *  }
 * However, this style of program is quite verbose. Rust provides a concise style of reading and
 * writing without moves through references.
 *
 * References Are Non-Owning Pointers
 *
 * A reference is a kind of pointer. Here's an example of a reference that rewrites our greet
 * program in a more convenient manner:
 *
 *  fn main() {
 *      let m1 = String::from("Hello");
 *      let m2 = String::from("world");     -> L1
 *      greet(&m1, &m2);                    -> L3 (note the ampersands)
 *      let s = format!("{} {}", m1, m2);
 *  }
 *
 *  fn greet(g1: &String, g2: &String) {    -> note the ampersands
 *      println!("{} {}!", g1, g2); -> L2
 *  }
 * The expression &m1 uses the ampersand operator to create a reference to (or "borrow") m1. The
 * type of the greet parameter g1 is changed to &String, meaning "a reference to a String".
 *
 * Observe at L2 that there are two steps from g1 to the String "Hello". g1 is a reference that
 * points to m1 one the stack, and m1 is a String containing a box that points to "Hello" on the
 * heap.
 *
 * While m1 owns the heap data "Hello", g1 does not own either m1 or "Hello". Therefore after greet
 * ends and the program reaches L3, no heap data has been deallocated. Only the stack frame for
 * greet disappears. This fact is consistent with Box Deallocation Principle. Because g1 did not
 * own "Hello", Rust did not allocate "Hello" on behalf of g1.
 *
 * References are non-owning pointers, because they do not own the data they point to.
 *
 * Dereferencing a Pointer Accesses Its Data
 *
 * The previous examples using boxes and strings have not shown how Rust "follows" a pointer to its
 * data. For example, the println! macro has mysteriously worked for both owned strings of type
* String, and for string references of type &String. The underlying mechanism is the dereference
* operator, written with an asterisk (*). For example, here's a program that uses dereference in a
* few different ways:
*
*   let mut x: Box<i32> = Box::new(1);
*   let a: i32 = *x;                // *x reads the heap value, so a = 1
*   *x += 1;                        // *x on the left-side modifies the heap value,
*                                   // so x points to the value 2
*   let r1: &Box<i32> = &x;         // r1 points to x on the stack
*   let b: i32 = **r1;              // two dereferences get us to the heap value
*
*   let r2: &i32 = &*x;             // r2 points to the heap value directly
*   let c: i32 = *r2;   -> L1       // so only one dereference is needed to read it
* Observe that difference between r1 pointing to x on the stack, and r2 pointing to the heap value
* 2.
*
* You probably won't see the dereference operator very often when you read Rust code. Rust
* implicitly inserts dereferences and references in certain cases, such as calling a method with
* the dot operator. For example, this program shows two equivalent ways of calling the i32::abs
* (absolute value) and str::len (string length) functions:
*
*   let x: Box<i32> = Box::new(-1);
*   let x_abs1 = i32::abs(*x)       // explicit dereference
*   let x_abs2 = x.abs();           // implicit dereference
*   assert_eq!(x_abs1, x_abs2);
*
*   let r: &Box<i32> = &x;
*   let r_abs1 = i32::abs(**r);     // explicit dereference (twice)
*   let r_abs2 = r.abs();           // implicit dereference (twice)
*   assert_eq!(r_abs1, r_abs2);
*
*   let s = String::from("Hello");
*   let s_len1 = str::len(&s);      // explicit reference
*   let s_len2 = s.len();           // implicit reference
*   assert_eq!(s_len1, s_len2);
* This example shows implicit conversions in three ways:
* 1. The i32::abs function expects an input of type i32. To call abs with a Box<i32>, you can
*    explicitly dereference the box like i32::abs(*x). You can also implicitly dereference the box
*    using method-call syntax like x.abs(). The dot syntax is syntactic sugar for function-call
*    syntax.
* 2. This implicit conversion works for multiple layers of pointers. For example, calling abs on a
*    reference to a box r: &Box<i32> will insert two dereferences.
* 3. This conversion also works the opposite direction. The function str::len expects a reference
*    &str. If you call len on an owned String, then Rust will insert a single borrowing operator.
 */

fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x);
    let x_abs2 = x.abs();
    assert_eq!(x_abs1, x_abs2);
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}!", *g1, *g2);
}
