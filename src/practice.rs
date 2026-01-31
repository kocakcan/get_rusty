// fn greet(g1: String, g2: String) -> (String, String) {
//     println!("{} {}!", g1, g2);
//     (g1, g2)
// }
//
// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     let (m1_again, m2_again) = greet(m1, m2);
//     let _s = format!("{} {}!", m1_again, m2_again);
// }

// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     greet(&m1, &m2);
//     let _s = format!("{} {}!", m1, m2);
// }
//
// fn greet(g1: &String, g2: &String) {
//     println!("{} {}!", *g1, *g2);
// }

// fn main() {
//     let name = String::from("Seyfi");
//     // This is read-only
//     let prefix = " Jr.";
//     let name = concat(name, prefix);
//
//     println!("My name is {}", name);
// }
//
// fn concat(mut text: String, prefix: &str) -> String {
//     text.push_str(prefix);
//     text
// }

/* clone() copies every string in the input */
// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     let mut name_clone = name.clone();
//     name_clone.push(String::from("Esq."));
//     name_clone.join(" ")
// }

/* slice::join already copies the data in name into the string full
 * which avoids unnecessary copies */
// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     let mut full = name.join(" ");
//     full.push_str(" Esq.");
//     full
// }
//
// fn main() {
//     let name = vec![String::from("Can")];
//     let first = &name[0];
//     stringify_name_with_title(&name);
//     println!("{}", first);
// }

/* let s2 = *s_ref; won't compile as s_ref doesn't own the string (it's only a reference). If Rust
* compiled this code, when the main ends, both s and s2 would be dropped and the string would've
* been dropped twice (double-free) which is an undefined behaviour. */
// fn main() {
//     let s = String::from("Can");
//     let s_ref = &s;
//     let s2 = *s_ref;
//     println!("{s2}");
// }

/* You have a mutable borrow (n) to part of v, and you're trying to read from another part of v
* (v[i - 1]) at the same time. Rust can't prove that v[i] and v[i - 1] are different elements and
* you cannot have reader and writer at the same time */
// fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
//     let n = &mut v[i];
//     *n = v[i - 1];
// }

// fn main() {
//     let mut v = vec![1, 2, 3];
//     copy_to_prev(&mut v, 1);
// }

/* name is an immutable reference and has only R permission on the String. We cannot dereference
* name and access the String it points to. */
// fn award_phd(name: &String) {
//     let mut name = *name;
//     name.push_str(", Ph.D.");
// }
