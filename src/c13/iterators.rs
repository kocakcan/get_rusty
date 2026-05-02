/// Processing a Series of Items with Iterators
///
/// The iterator pattern allows you to perform some task on sequence of items in turn. An iterator
/// is reponsible for the logic of iterating over each item and determining when the sequence has
/// finished. When you use iterators, you don't have to reimplement that logic yourself.
///
/// In Rust, iterators are lazy, meaning they have no effect until you all methods that consume the
/// iterator to use it up. For example, the code in Listing 13-10 creates an iterator over the items
/// in the vector v1 by calling the iter method on Vec<T>. This code by itself doesn't do anything
/// useful.
///
///     let v1 = vec![1, 2, 3];
///     let v1_iter = v1.iter();
///     Listing 13-10: Creating an iterator
