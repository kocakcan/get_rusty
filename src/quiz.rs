// fn incr(n: &mut i32) {
//     *n += 1;
// }
//
// fn main() {
//     let mut n = 1;
//     incr(&mut n);
//     println!("{n}")
// }

// It is illegal to use a mutable reference to a value (s3) while an immutable reference is live
// (s2).

// fn main() {
//     let mut s = String::from("Hello");
//     let s2 = &s;
//     let s3 = &mut s;
//     s3.push_str(" world");
//     println!("{s2}");
// }

/* v.push(n) can cause v to reallocate its internal contents, invalidating any references to the
* elements of v on the heap. Therefore calling give_and_take(&v, 4) will cause previously-created
* element references to point to invalid memory. The program that does println!("{}"< n) will cause
* undefined behaviour by reading the invalid memory.
*/

// fn give_and_take(v: &Vec<i32>, n: i32) -> i32 {
//     v.push(n);
//     v.remove(0)
// }
//
// fn main() {
//     let v = vec![1, 2, 3];
//     let n = &v[0];
//     give_and_take(&v, 4);
//     println!("{}", n);
// }

/// We only know that this type implements Display trait and we also know that it's not a String
/// which has the push_str method. Therefore we cannot call s2.push_str(..). If the return type of
/// displayable was -> T, then this program would compile.
// use std::fmt::Display;
// fn displayable<T: Display>(t: T) -> impl Display { t }
//
// fn main() {
//     let s = String::from("hello");
//     let mut s2 = displayable(s);
//     s2.push_str(" world");
//     println!("{s2}");
// }
