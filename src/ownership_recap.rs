/*
 * Ownership Recap
 *
 * Ownership versus Garbage Collection
 *
 * To put ownership into context, we should talk about garbage collection. Most programming
 * languages use a garbage collector to manager memory, such as in Python, JavaScript, Java, and
 * Go. A garbage collector works at runtime adjacent to a running program (a tracing collector, at
 * least). The collector scans through memory to find data that's no longer used -- that is, the
 * running program can no longer reach that data from a function-local variable. Then the collector
 * deallocates the unused memory for later use.
 *
 * The key benefit of a garbage collector is that it avoids undefined behaviour (such as using
 * freed memory), as can happen in C or C++. Garbage collection also avoids the need for a complex
 * type system to check for undefined behaviour, like in Rust. However, there are a few drawbacks
 * to garbage collection. One obvious drawback is performance, as garbage collection incurs either
 * frequent small overheads (for reference-counting, like in Python and Swift) or infrequent large
 * overheads (for tracing, like in all other GC'd languages).
 *
 * But another less obvious drawback is that garbage collection can be unpredictable. To illustrate
 * the point, say we are implementing a Document type that represents a mutable list of words.
 *
 * Consider two key questions about document.py:
 *
 * 1. When is the words array deallocated? This program has created three pointers to the same
 *    array. The variables words, d, and d2 all contain a pointer to the words array allocated on
 *    the heap. Therefore Python will only deallocate the words array when all three variables are
 *    out of scope. More generally, it's often difficult to predict where data will be
 *    garbage-collected just by reading the source code.
 * 2. What are the contents of the document d? Because d2 contains a pointer to the same words
 *    array as d, then d2.add_word("world") also mutates the document d. Therefore; In this
 *    example, the words in d are ["Hello", "world"]. This happens because d.get_words() returns a
 *    mutable reference to the words array in d. Pervasive, implicit mutable references can easily
 *    lead to unpredictable bugs when data structures can leak their internals. Here, it is
 *    probably not intended behaviour that a change to d2 can change d.
 *
 * This problem is not unique to Python -- you can encounter similar behaviour in C#, Java,
 * JavaScript, and so on. In fact, most programming languages actually have a concept of pointers.
 * It's just a question of how the language exposes pointers to the programmer. Garbage collection
 * makes it difficult to see which variable points to which data. For example, it wasn't obvious
 * that d.get_words() produced a pointer to data with d.
 *
 * By contrast, Rust's ownership model puts pointers front-and-center. We can see that by
 * translating the Document type into a Rust data structure.
 *
 * This Rust API differs from the Python API in a few key ways:
 *
 * - The function new_document consumes the ownership of the input vector words. That means the
 * Document owns the word vector. The word vector will be predictably deallocated when its owning
 * Document goes out of scope.
 * - The function add_word requires a mutable reference &mut Document to be able to mutate a
 * document. It also consumes ownership of the input word, meaning no one else can mutate the
 * individual words of the document.
 * - The function get_words returns an explicit immutable reference to string within the document.
 * The only way to create a new document from this word is to deep-copy its contents.
 *
 * Rust makes working with memory and pointers concepts explicit, which is fucking awesome. This
 * has the dual benefit of (1) improving runtime performance by avoiding garbage collection, and
 * (2) improving predictability by preventing accidental "leaks" of data.
 *
 * The Concept of Ownership
 *
 * Ownership at Runtime
 *
 * - Rust allocates local variables in stack frames, which are allocated when a function is called
 * and deallocated when the call ends.
 * - Local variables can hold either data (like numbers, booleans, tuples, etc.) or pointers.
 * - Pointers can be created either through boxes (pointers owning data on the heap) or references
 * (non-owning pointers)
 *
 *  fn main() {
*       let mut a_num = 0;
*       inner(&mut a_num);
*  }
*
*  fn inner(x: &mut i32) {
*       let another_num = 1;
*       let a_stack_ref = &another_num;
*
*       let a_box = Box::new(2);
*       let a_box_stack_ref = &a_box;
*       let a_box_heap_ref = &*a_box;
*
*       *x += 5;
*  }
* - Why does a_box_stack_ref point to the stack, while a_box_heap_ref point to the heap?
* + a_box_stack_ref is a reference and it is pointing to the a_box, which is a local variable that
* points to the value 2 on the heap. On the other hand; a_box_heap_ref points to &*a_box. As & and
* * have the same precedence they associate from right to left, meaning we should dereference the
* a_box first which would give us 2 and then it would be a reference to 2 which is on the heap.
* - Why is the value 2 no longer on the heap at L2? 2 is owned by the a_box variable and when
* inner() function call ends, the local variables are deallocated. So, when inner returns to the
* main we 2 (on the heap) and all the local variables to that function are deallocated.
* + Why does a_num have the value 5 at L2? a_num is passed in to inner as a mutable reference and
* in the function body it's modified. Since x is only a reference it doesn't own the actual value,
* so when the inner() function ends the reference gets deallocated but the ownership would be
* transferred to a_num which is the original owner.
*
* Slices are a special kind of reference that refer to a contiguous sequence of data in memory.
*
*   fn main() {
*       let s = String::from("abcdefg");
*       let s_slice = &s[2..5];
*   }
*
* Ownership at Compile-time
*
* Rust tracks R (read), W (write), and O (own) permissions on each variable. Rust requires that a
* variable has appropriate permissions to perform a given operation. As a basic example, if a
* variable is not declared as let mut, then it is missing the W permission and cannot be mutated:
*
*   fn main() {
*       let n = 0;
*
*       -> n        | RO
*
*       n += 1;     | += requires RW and n doesn't have W
*   }
* A variable's permissions can be changed if it is moved or borrowed. A move of a variable with a
* non-copyable type (like Box<T> or String) requires the RO permissions, and the move eliminates
* all permissions on the variable. That rule prevents the use of moved variables:
*
*   fn main() {
*       let s = String::from("Hello world");
*
*       -> s        | RO
*
*       consume_a_string(s);
*
*       -> s        | No permissions as it was moved by consume_a_string()
*
*       println!("{s}");    -> println! requires R which s doesn't have
*   }
*
*   fn consume_a_string(_s: String) {
*       // om nom nom
*   }
* Borrowing a variable (creating a reference to it) temporarily removes some of the variable's
* permissions. An immutable borrow creates an immutable reference, and also disables the borrowed
* data from being mutated or moved. For example, printing an immutable reference is ok:
*
*   let mut s = String::from("Hello");
*
*   -> s        | RWO
*
*   let s_ref = &s;
*
*   -> s        | R
*   -> s_ref    | RO
*   -> *s_ref   | R
*
*   println!("{s_ref}");
*
*   -> s        | RWO
*   -> s_ref    | No permissions (goes out of scope)
*   -> *s_ref   | No permissions (goes out of scope)
*
*   println!("{s}");    -> again println! requires R which s has
*
*   -> s        | No permissions (goes out of scope)
* But mutating an immutable reference is not ok:
*
*   let mut s = String::from("Hello");
*
*   -> s        | RWO
*
*   let s_ref = &s;     -> referencing/borrowing requires R
*
*   -> s_ref    | RO
*   -> *s_ref   | R
*   -> s        | R
*
*   s_ref.push_str(" world");   -> push_str() requires RW
*
*   -> s        | No permissions
*   -> s_ref    | RO
*   -> *s_ref   | R
*
*   println!("{s_ref}");
*
*   -> s_ref    | No permissions (goes out of scope)
*   -> *s_ref   | No permissions (goes out of scope)
* But mutating an immutable reference is not ok:
*
*   let mut s = String::from("Hello");
*
*   -> s        | RWO
*
*   let s_ref = &s;             -> referencing/borrowing requires R
*
*   -> s        | R
*   -> s_ref    | RO
*   -> *s_ref   | R
*
*   s_ref.push_str(" world");   -> push_str() requires RW
*                               -> s_ref doesn't have W
*   println!("{s}");            -> this is ok because s would regain
*                               -> all its permissions
* And mutating the immutable borrowed data is not ok:
*
*   let mut s = String::from("Hello");
*
*   -> s        | RWO
*
*   let s_ref = &s;             -> referencing/borrowing requires R
*
*   -> s        | R
*   -> s_ref    | RO
*   -> *s_ref   | R
*
*   s.push_str(" world");       -> push_str() requires RW
*                               -> a s doesn't have it since s_ref is still in scope
*   println!("{s_ref}");        -> if it wasn't for this line, the previous line would be ok
* And moving data out of the reference is not ok:
*
*   let mut s = String::from("Hello");
*
*   -> s        | RWO
*
*   let s_ref = &s;
*
*   -> s        | R
*   -> s_ref    | RO
*   -> *s_ref   | R
*
*   let s2 = *s_ref;            -> this is moving not borrowing
*                               -> so it requires O as well along with R
*   println!("{s}");
* A mutable borrow creates a mutable reference, which disables the borrowed data from being read,
* written, or moved. For example, mutating a mutable reference is ok:
*
*   let mut s = String::from("Hello");
*
*   -> s        | RWO
*
*   let s_ref = &mut s;
*
*   -> s        | No permissions (loses all permissions due to mutable reference)
*   -> s_ref    | RWO
*   -> *s_ref   | RW
*
*   println!("{s}");            -> requires R which s doesn't have
*   s_ref.push_str(" world")    -> ok since s_ref still has W permission
*
*   -> s_ref    | No permissions (goes out of scope)
*   -> *s_ref   | No permissions (goes out of scope)
* Connecting Ownership between Compile-time and Runtime
*
* Rust's permissions are designed to prevent undefined behaviour. For example, one kind of
* undefined behaviour is a use-after-free where freed memory is read or written. Immutable borrows
* remove the W permission to avoid use-after-free, like in this case:
*
*   let mut v = vec![1, 2, 3];
*   let n = &v[0];
*   v.push(4);
*   println!("{n}");
* Here, n borrows the v, which in turn strips v of its WO permissions. On the last line we are
* printing the value of n so we know that n will be in the scope until that line. We can't perform
* push() operation as at that place v wouldn't have its W permission. Had this been compiled, we
* would've read the freed memory location as n would be pointing to an invalid memory location
* after push() has been performed.
*
* Another kind of undefined behaviour is a double-free where memory is freed twice. Dereferences of
* references to non-copyable data do not have the O permission to avoid double-frees, like in this
* case:
*
*   let v = vec![1, 2, 3];
*   let v_ref: &Vec<i32> = &v;
*   let v2 = *v_ref;
*   drop(v2);
*   drop(v);
* When v2 goes out scope, its heap data will be deallocated, meaning v would be pointing to an
* invalid memory location and then v is also dropped, causing freed memory location to be freed
* once again.
*
* The Rest of Ownership
*
* structs, enums, and traits will have specific interactions with ownership.
 */
