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
 * The expression &m1
 */

fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}!", *g1, *g2);
}
