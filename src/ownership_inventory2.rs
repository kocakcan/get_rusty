// fn remove_zeros(v: &mut Vec<i32>) {
//     for (i, t) in v.iter().enumerate().rev() {
//         if *t == 0 {
//             v.remove(i);
//             v.shrink_to_fit();
//         }
//     }
// }

/* To violate memory safety, remove_zeros must be called with a vector that contains a zero after
* the first element. The call to v.shrink_to_fit() will deallocate memory owned by the vector (due
* to resizing), which will invalidate the iterator v.iter() which contains a pointer to the old
* data. Note that reading the vector after calling remove_zeros is not essential to the safety
* violation, since the issue is internal to remove_zeros.
*
* let mut v = vec![5, 5, 0];
* remove_zeros(&mut v);
* println!("{:?}", v);
*
* let mut v = vec![1, 2, 0, 3];
* remove_zeros(&mut v);
*/

fn remove_zeros(v: &mut Vec<i32>) {
    for i in (0..v.len()).rev() {
        if v[i] == 0 {
            v.remove(i);
            v.shrink_to_fit();
        }
    }
}

/* Any strategy that requires allocating a new vector, either via Vec::clone or Vec::new, requires
* unnecessary additional allocation. Therefore the simplest strategy that works is to only iterate
* over the indices 0 .. v.len() which does not borrow v. We do this in reverse order to avoid
* removing missing indexes.
*
* The most idiomatic strategy is actually to use a built-in function Vec::retain.
*/

/// Reverses the elements of a vector in-place
// fn reverse(v: &mut Vec<String>) {
//     let n = v.len();
//     for i in 0..n / 2 {
//         std::mem::swap(&mut v[i], &mut v[n - i - 1]);
//     }
// }

/* The compiler does not consider the specific value of the indexes used to access an array, so
* &mut v[i] and &mut v[n - i - 1] are assumed to possibly refer to the same element. Therefore we
* get an error where v cannot be mutably borrowed twice.
*/

/* This function cannot cause a memory safety violation because i != n - i - 1 for all i, so the
* two mutable references always refer to different elements. Note that the let x = &v[0] program
* would not compile, as Rust's borrow checker would not allow reverse to be called while x is live.
*/

fn reverse(v: &mut Vec<String>) {
    let n = v.len();
    for i in 0..n / 2 {
        let p1 = &mut v[i] as *mut String;
        let p2 = &mut v[n - i - 1] as *mut String;
        unsafe {
            std::ptr::swap_nonoverlapping(p1, p2, 1);
        }
    }
}

/* In a situation where the borrow checker rejects an operation that is actually safe and has no
* workaround, then unsafe code is sometimes acceptable if it's critical to avoid allocations. In
* this specific case, you should actually use Vec::swap.
*/

fn main() {
    let mut v = vec![1, 2, 3, 0, 0, 5];
    println!("{v:?}");
    remove_zeros(&mut v);
    println!("{v:?}");

    let mut names = vec![
        String::from("Can"),
        String::from("Seyfi"),
        String::from("Dilan"),
        String::from("Leyli"),
        String::from("Medet"),
    ];
    println!("{names:?}");
    reverse(&mut names);
    println!("{names:?}");
}
