fn main() {
    let x = Box::new(0);
    let y = Box::new(&x);

    // How many dereferences would you need to access 0 through y?
    // x points to 0, which is on the heap.
    // y points to a reference that is on the heap pointing to x
    // If we dereference y once, we would access y
    // If we dereference y we would access x
    // Finally if we derefence x access the 0.

    // Both work as Rust implicitly inserts references and dereferences
    println!("x: {}", ***y);
    println!("x: {}", y);
}
