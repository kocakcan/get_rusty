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
*
* Rust Avoids Simultaneous Aliasing and Mutation
*
* Pointers are a powerful and dangerous feature because they enable aliasing. Aliasing is accessing
* the same data through different variables. On its own, aliasing is harmless. But combined with
* mutation, we have a recipe for disaster. One variable can "pull the rug out" from another
* variable in many ways, for example:
*
* - By deallocating the aliased data, leaving the other variable to point to deallocated memory.
* - By mutating the aliased data, invalidating runtime properties expected by the other variable.
* - By concurrently mutating the aliased data, causing a data race with nondeterministic behaviour
* for the other variable.
*
* Unlike arrays which have a fixed length, vectors have a variable length by storing their elements
* in the heap. For example, Vec::push adds an element to the end of a vector, like this:
*
*   let mut v: Vec<i32> = vec![1, 2, 3];
*   let num: &i32 = &v[2];                  -> L1
*   v.push(4);
*   println!("Third element is {}", *num);   -> L3
* The macro vec! creates a vector with the elements between the brackets. The vector v has type
* Vec<i32>. The syntax <i32> means the elements of the vector have type i32.
*
* One important implementation detail is that v allocates a heap array of certain capacity.
*
* Notice that the vector has a length (len) of 3 and a capacity (cap) of 3. The vector is at
* capacity. So when we do a push, the vector has to create a new allocation with larger capacity,
* copy all the elements over, and deallocate the original heap array.
*
* To tie this back to memory safety, let's bring references into the mix. Say we created a
* reference to a vector's heap data. Then that reference can be invalidated by a push.
*
* Initially, v points to an array with 3 elements on the heap. Then num is created as a reference
* to the third element, as seen at L1. However, the operation v.push(4) resizes v. The resize will
* deallocate the previous array and allocate a new, bigger array. In the process, num is left
* pointing to invalid memory. Therefore at L3, *num reads invalid memory, causing undefined
* behaviour.
*
* In more abstract terms, the issue is that the vector v is both aliased (by the reference num) and
* mutated (by the operation v.push(4)). So to avoid these kinds of issues, Rust follows a basic
* principle:
*
*   Pointer Safety Principle: data should never be aliased and mutated at the same time.
* Data can be aliased. Data can be mutated. But data cannot be both aliased and mutated. For
* example, Rust enforces this principle for boxes (owned pointers) by disallowing aliasing.
* Assigning a box from one variable to another will move ownership, invalidating the previous
* variable. Owned data can only be accessed through the owner - no aliases.
*
* However, because references are non-owning pointers, they need different rules than boxes to
* ensure the Pointer Safety Principle. By design, references are meant to temporarily create
* aliases.
*
* References Change Permissions on Places
*
* The core idea behind the borrow checker is that variables have three kinds of permissions on
* their data:
*
* - Read (R): data can be copied to another location.
* - Write (W): data can be mutated.
* - Own (O): data can be moved or dropped.
* These permissions don't exist at runtime, only within the compiler. They describe how the
* compiler "thinks" about your program before the program is executed.
*
* By default, a variable has read/own permissions (RO) on its data. If a variable is annotated with
* let mut, then it also has the write permission (W). The key idea is that references can
* temporarily remove these permissions.
*
*   let mut v: Vec<i32> = vec![1, 2, 3];
*   let num: &i32 = &v[2];
*   println!("Third element is {}", *num);
*   v.push(4);
* 1. After let mut v = (...), the variable v has been initialized. It gains +R+W+O permissions (the
*    plus sign indicates gain).
* 2. After let  num = &v[2], the data in v has been borrowed by num.
*   Three things happen:
*   - The borrow removes WO permissions from v. v cannot be written or owned, but it can still be
*   read.
*   - The variable num has gained RO permissions. num is not writable because it was not marked let
*   mut.
*   - The place *num has gained R permission.
* 3. After println!(...), then num is no longer in use, so v is no longer borrowed.
*   Therefore:
*   - v regains its WO permissions
*   - num and *num have lost all of their permissions
* 4. After v.push(4), then v is no longer is use, and it loses all of its permissions.
*
* Accessing data through a reference is not the same as manipulating the reference itself. For
* example, say we declared a reference to a number with let mut:
*
*   let x = 0;  -> RO permissions
*   let mut x_ref = &x; -> x        | R (not O permission as it was borrowed)
*                       -> x_ref    | RWO permissions
*                       -> *x_ref   | R permission
* Notice that x_ref has the W permission, while *x_ref does not. That means we can assign a
* different reference to the x_ref variable (e.g., x_ref = &y), but we cannot mutate the data it
* points to (e.g., *x_ref += 1).
*
* More generally, permissions are defined on places and not just variables. A place is anything you
* can put on the left-hand side of an assignment. Places include:
*
*   - Variables, like a.
*   - Dereferences of places, like *a.
*   - Array of accesses of places, like a[0].
*   - Fields of places, like a.0 for tuples or a.field for structs.
*   - Any combination of the above, like *((*a)[0].1).
* Second, why do places lose permissions when the become unused? Because some permissions are
* mutually exclusive. If you write num = &v[2], then v cannot be mutated or dropped while num is in
* use. But that doesn't mean it's invalid to use num again. For example, if we add another println!
* to the above program, then num simply loses its permissions one line later:
*
*   let mut v: Vec<i32> = vec![1, 2, 3];                -> v    | RWO
*   let num: &i32 = &v[2];                              -> v    | R
*                                                       -> num  | RO
*                                                       -> *num | R
*   println!("Third element is {}", *num);
*   println!("Again, the third element is {}", *num);   -> v    | RWO
*                                                       -> num  | No permissions
*                                                       -> *num | No permissions
*   v.push(4);                                          -> v    | No permissions
* It's only a problem if you attempt to use num again after mutating v.
*
* The Borrow Checker Finds Permission Violations
*
* Recall the Pointer Safety Principle: data should not be aliased and mutated. The goal of these
* permissions is to ensure that data cannot be mutated if it is aliased. Creating a reference to
* data ("borrowing" it)  causes that data to be temporarily read-only until the reference is no
* longer in use.
*
* Rust uses these permissions in its borrow checker. The borrow checker looks for potentially
* unsafe operations involving references. Let's return to the unsafe program we saw earlier, where
* push invalidates a reference.
*
*   let mut v: Vec<i32> = vec![1, 2, 3];    -> v    | RWO
*   let num: &i32 = &v[2];                  -> v    | R
*                                           -> num  | RO
*                                           -> *num | R
*
*   v.push(4);                              -> v should be readable and writable
*                                           -> but it's only readable
*   println!("Third element is {}", *num);
* Any time a place is used, Rust expects that place to have certain permissions depending on the
* operation. For example, the borrow &v[2] requires that v is readable. Therefore the R permission
* is shown between the operation & and the place v. The letter is filled-in because v has the read
* permission at that line.
*
* By contrast, the mutating operation v.push(4) requires that v is readable and writable. Both R
* and W are shown. However, v does not have write permissions (it is borrowed by num). So the
* letter W is hollow, indicating that the write permission is expected but v does not have it.
*
* The error message explains that v cannot be mutated while the reference num is in use. That's the
* surface-level reason -- the underlying issue is that num could be invalidated by push. Rust
* catches that potential violation of memory safety.
*
* Mutable References Provide Unique and Non-Owning Access to Data
*
* The references we have seen so far are read-only immutable references (also called shared
* references). Immutable references permit aliasing but disallow mutation. However, it is also
* useful to temporarily provide mutable access to data without moving it.
*
* The mechanism for this is mutable references (also called unique references). Here's a simple
* example of a mutable reference with teh accompanying permissions changes:
*
*   let mut v: Vec<i32> = vec![1, 2, 3];    -> v    | RWO
*   let num: &mut i32 = &mut v[2];          -> v    | No permissions
*                                           -> num  | RO
*                                           -> *num | RW
*   *num += 1;                              -> requires read and write
*   println!("Third element is {}", *num);  -> v    | RWO
*                                           -> num  | No permissions
*                                           -> *num | No permissions
*   println!("Vector is now {:?}", v);      -> v    | No permissions
* A mutable reference is created with the &mut operator. The type of num is written as &mut i32.
* Compared to immutable references, you can see two important differences in the permissions:
*   1. When num was an immutable reference, v still had the R permission. Now that num is a mutable
*      reference, v has lost all permissions while num is in use.
*   2. When num was an immutable reference, the place *num only had the R permission. Now that num
*      is a mutable reference, *num has also gained the W permission.
*
* The first observation is what makes mutable references safe. Mutable references allow mutation
* but prevent aliasing. The borrowed place v becomes temporarily unusable, so effectively not an
* alias.
*
* The second observation is what makes mutable references useful. v[2] can be mutated through *num.
* For example, *num += 1 mutates v[2]. Note that *num has the W permission, but num does not. num
* refers to the mutable reference itself, e.g., num cannot be assigned to a different mutable
* reference.
*
* Mutable references can also be temporarily "downgraded" to read-only references. For example:
*
*   let mut v: Vec<i32> = vec![1, 2, 3];
*   let num: &mut i32 = &mut v[2];          -> *num     | RW
*
*   let num2: &i32 = &*num;                 -> *num     | R
*                                           -> *num2    | R
*                                           -> num      | R
*                                           -> num2     | RO
*   println!("{} {}", *num, *num2);
* In this program, the borrow &*num removes the W permission from *num but no the R permission, so
* println!(...) can read both *num and *num2.
*
* Permissions Are Returned At The End of a Reference's Lifetime
*
* We said above that a reference changes permissions while it is "in use". The phrase "in use" is
* describing a reference's lifetime, or the range of code spanning from its birth (where the
* reference is created) to its death (the last time(s) the reference is used).
*
* For example, in this program, the lifetime of y starts with let y = &x, and ends with let z = *y;
*
*   let mut x = 1;  -> x    | RWO
*   let y = &x;     -> x    | R
*                   -> y    | RO
*                   -> *y   | R
*   let z = *y;     -> x    | RWO
*                   -> y    | No permissions
*                   -> *y   | No permissions
*                   -> z    | RO
*   x += z;         -> x    | No permissions
*                   -> z    | No permissions
* The W permission on x is returned to x after the lifetime of y has ended, like we have seen
* before.
*
* In the previous examples, a lifetime has been a contiguous region of code. However, once we
* introduce control flow, this is not necessarily the case. For example, here is a function that
* capitalizes the first character in a vector of ASCII characters:
*
*   fn ascii_capitalize(v: &mut Vec<char>) {
*       let c = &v[0];
*
*       if c.is_ascii_lowercase() {
*           let up = c.to_ascii_uppercase();
*           v[0] = up;
*       } else {
*           println!("Already capitalized: {:?}", v);
*       }
*   }
* The variable c has a different lifetime in each brand of the if-statement. In the then-block; c
* is used in the expression c.to_ascii_uppercase(). Therefore *v does not regain the W permission
* untile after that line.
*
* However, in the else-block, c is not used. *v immediately regains the W permission on entry to
* the else-block.
 */

fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x);
    let x_abs2 = x.abs();
    // let x_abs3 = (*x).abs();
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);
    let r_abs2 = r.abs();
    // let r_abs3 = (*r).abs();
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    // let s_len1 = str::len(&s);
    let s_len2 = s.len();
    let s_len3 = (*s).len();
    assert_eq!(s_len2, s_len3);
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}!", *g1, *g2);
}
