/*
* What is Ownership?
*
* Ownership is a discipline for ensuring the safety of Rust programs. To understand ownership, we
* first need to understand what makes a Rust program safe (or unsafe).
*
* Safety is the Absence of Undefined Behaviour
*/

// fn read(y: bool) {
//     if y {
//         println!("y is true!");
//     }
// }
//
// fn main() {
//     let x = true;
//     read(x);
// }

/*
* We can make this program unsafe to execute by moving the call to read before the definition of x:
*
*   fn read(y: bool) {
*       if y {
*           println!("y is true!");
*       }
*   }
*
*   fn main() {
*       read(x);    // oh no! x isn't defined!
*       let x = true;
*   }
* This second program is unsafe because read(x) expects x to have a value of type bool, but x
* doesn't have a value yet.
*
* When a program like this is executed by an interpreter, then reading x before it's defined would
* raise an exception such as Python's NameError or JavaScript's ReferenceError. But exceptions come
* at a cost. Each time an interpreted programs reads a variable, then the interpreter must check
* whether that variable is defined.
*
* Rust's goal is to compile programs into efficient binaries that require as few runtime checks as
* possible. Therefore Rust doesn't check at runtime whether a variable is defined before being
* used. Instead, Rust checks at compile-time. If you try to compile the unsafe program, you'll get
* an error.
*
* You probably have the intuition that it's good for Rust to ensure that variables are defined
* before they are used. But why? To justify the rule, we have to ask: what would happen if Rust
* allowed a rejected program to compile?
*
* On a computer with a processor using an x86 architecture, Rust generates the following assembly
* code for the main function in the safe program.
*
*   main:
*       ; ...
*       mov     edi, 1
*       call    read
*       ; ...
* This assembly code will:
* - Move the number 1, representing true, into a "register" (a kind of assembly variable) called
* edi.
* - Call the read function, which expects its first argument y to be in the edi register.
* If the unsafe function was allowed to compile, its assembly might look like this:
*
*   main:
*       ; ...
*       call    read
*       mov     edi, 1  ; mov is after call
*       ; ...
* This program is unsafe because read will expect edi to be a boolean, which is either the number 0
* or 1. But edi could be anything: 2, 100, 0x12345678. When read wants to use its argument y for
* any purpose, it will immediately cause UB.
*
* Rust doesn't specify what happens if you try to run if y { .. } when y isn't true or false. That
* behaviour, or what happens after executing the instruction, is undefined. Something will happen,
* for example:
*
* - The code executes without crashing, and no one notices a problem.
* - The code immediately crashes due to a segmentation fault or another kind of operating system
* error.
* - The code executes without crashing, until a malicious actor creates the right input to delete
* your production database, overwrite your backups, and steal your lunch money.
*
* A foundational goal of Rust is to ensure that your programs never have undefined behaviour. That
* is the meaning of safety. Undefined behaviour is especially dangerous for low-level programs with
* direct access to memory.
*
* A secondary goal of Rust is to prevent undefined behaviour at compile-time instead of run-time.
* This goal has two motivations:
*
* 1. Catching bugs at compile-time means avoiding bugs in production, improving the reliability of
*    your software.
* 2. Catching bugs at compile-time means fewer runtime checks for those bugs, improving the
*    performance of your software.
* Rust cannot prevent all bugs. If an application exposes a public and unauthenticated
* /delete-production-database endpoint, then a malicious actor doesn't need a suspicious
* if-statement to delete the database. But Rust's protections are still likely to make programs
* safer versus using a language with fewer protections.
*
* Ownership as a Discipline for Memory Safety
*
* Since safety is the absence of undefined behaviour, and since ownership is about safety, then we
* need to understand ownership in terms of the undefined behaviours it prevents.
*
* Memory is the space where data is stored during the execution of a program. There are many ways
* to think about memory:
*
* - You can think of memory at a low level like "memory is an array of bytes" or "memory is the
* pointers I get back from the malloc".
* The high-level model is too abstract to explain how Rust works. You will need to understand the
* concept of a pointer, for instance. The low-level model is too concrete to explain how Rust
* works. Rust does not allow you to interpret memory as an array of bytes, for instance.
*
* Rust provides a particular way to think about memory. Ownership is a discipline for safely using
* memory within that way of thinking.
*
* Variables Live in the Stack
*
*   fn main() {
*       let n = 5;              -> L1
*       let y = plus_one(n);    -> L3
*       println!("The value of y is: {y}");
*   }
*
*   fn plus_one(x: i32) -> i32 {
*       x + 1                   -> L2
*   }
* Variables live in frames. A frame is a mapping from variables to values within a single scope,
* such as a function. For example:
*
* - The frame for main at location L1 holds n = 5;
* - The frame for plus_one at L2 holds x = 5;
* - The frame for main at location L3 holds n = 5, y = 6;
* Frames are organized into a stack of currently-called-functions. For example, at L2 the frame for
* main sits above the frame for the called function plus_one. After a function returns, Rust
* deallocates the function's frame. (Deallocation is also called freeing or dropping and we use
* those terms interchangeably.) This sequence of frames is called a stack because the most recent
* frame added is always the next frame freed.
*
* When an expression reads a variable, the variable's value is copied from its slot in the stack
* frame. For example, if we run this program:
*
*   let a = 5;      -> L1
*   let mut b = a;  -> L2
*   b += 1;         -> L3
* The value of a is copied into b, and a is left unchanged, even after changing b.
*
* Boxes Live in the Heap
*
* However, copying data can take up a lot of memory. For example, here's a slightly different
* program. This program copies an array with 1 million elements:
*
*   let a = [0; 1_000_000];
*   let b = a;
* Copying a into b causes the main frame to contain 2 million elements.
*
* To transfer access to data without copying it, Rust uses pointers. A pointer is a value that
* describes a location in memory. The value that a pointer points-to is called its pointee. One
* common way to make a pointer is to allocate memory in the heap. The heap is a separate region of
* memory where data can live indefinitely. Heap data is not tied to a specific stack frame. Rust
* provides a construct called Box for putting data on the heap. For example, we can wrap the
* million-element array in Box::bew like this:
*
*   let a = Box::new([0; 1_000_000]);
*   let b = a;
* Now, there is only ever a single array at a time. At L1, the value of a is a pointer to the array
* inside the heap. The statement let b = a copies the pointer from a into b, but the pointed-to
* data is not copied.
*
* Rust Does Not Permit Manual Memory Management
*
* Memory management is the process of allocating memory and deallocating memory. In other words,
* it's the process of finding unused memory and later returning that memory when it is no longer
* used. Stack frames are automatically managed by Rust. When a function is called, Rust allocates a
* stack frame for the called function. When the call ends, Rust deallocates the stack frame.
*
* As we saw above, heap data is allocated when calling Box::new(..). But when is heap data
* deallocated? Imagine that Rust had a free() function that frees a heap allocation. Imagine that
* Rust let a programmer call free whenever they wanted. This kind of "manual" memory management
* easily leads to bugs. For example, we could read a pointer to freed memory.
*
*   let b = Box::new([0; 100]);
*   free(b);
*   assert!(b[0] == 0);
* Here, we allocate an array on the heap. Then we call free(b), which deallocates the heap memory
* of b. Therefore the value b is a pointer to invalid memory. No undefined behaviour has happened
* yet! The program is still safe at L2. It's not necessarily a problem to have an invalid pointer.
*
* The undefined behaviour happens when we try to use the pointer by reading b[0]. That would
* attempt to access invalid memory, which could cause the program to crash. Or worse, it could not
* crash and return arbitrary data. Therefore, this program is unsafe.
*
* Rust does not allow programs to manually deallocate memory. That policy avoids the kinds of
* undefined behaviours shown above.
*
* A Box's Owner Manages Deallocation
*
* Instead, Rust automatically frees a box's heap memory. If a variable is bound to a box, when Rust
* deallocates the variable's frame, then Rust deallocates the box's heap memory.
*
*   fn main() {
*       let a_num = 4;              -> L1
*       make_and_drop();            -> L3
*   }
*
*   fn make_and_drop() {
*       let a_box = Box::new(5);    -> L2
*   }
* At L1, before calling make_and_drop, the state of memory is just the stack frame for main. Then
* at L2, while calling make_and_drop, a_box points to 5 on the heap. Once make_and_drop is
* finished, Rust deallocates its stack frame. make_and_drop contains the variable a_box, so Rust
* also deallocates the heap data in a_box. Therefore the heap is empty at L3.
*
* The box's heap memory has been successfully managed. But what if we abused this system? What
* happens when we bind two variables to a box?
*
*   let a = Box::new([0; 1_000_000]);
*   let b = a;
* The boxed array has now been bound to both a and b. By our "almost correct" principle, Rust would
* try to free the box's heap memory twice on behalf of both variables. That's undefined behaviour
* too!
*
* To avoid this situation, we finally arrive at ownership. When a is bound to Box::new([0;
 * 1_000_000]), we say that a owns the box. The statement let b = a moves ownership of the box
* from a to b. Given these concepts, Rust's policy for freeing boxes is more accurately described
* as: If a variable owns a box, when Rust deallocates the variable's frame, then Rust deallocates
* the box's heap memory.
*
* In the example above, b owns the boxed array. Therefore when the scope ends, Rust deallocates the
* box only once on behalf of b, not a.
*
* Collections Use Boxes
*
* Boxes are used by Rust data structures like Vec, String, and HashMap to hold a variable number of
* elements. For example, here's a program that creates, moves, and mutates a string:
*
*   fn main() {
*       let first = String::from("Ferris"); -> L1
*       let full = add_suffix(first);       -> L4
*       println!("{full}");
*   }
*
*   fn add_suffix(mut name: String) -> String {
*       name.push_str(" Jr.");  -> L2 & L3
*       name
*   }
* 1. At L1, the string "Ferris" has been allocated on the heap. It is owned by first.
* 2. At L2, the function add_suffix(first) has been called. This moves ownership of the string from
*    first to name. The string data is not copied, but the pointer to the data is copied.
* 3. At L3, the function name.push_str(" Jr.") resizes the string's heap allocation. This does
*    three things. First, it creats a new larger allocation. Second, it writes "Ferris Jr." into
*    the new allocation. Third, it frees the original heap memory. first now points to the
*    deallocated memory.
* 4. At L4, the frame for add_suffix is gone. This function returned name, transferring ownership
*    of the string to full.
*
* Variables Cannot Be Used After Being Moved
*
* The string program helps illustrate a key safety principle for ownership. Imagine that first were
* used in main after calling add_suffix. We can simulate such a program and see the undefined
* behaviour that results:
*
*   fn main() {
*       let first = String::from("Ferris");
*       let full = add_suffix(first);
*       println!("{full}, originally {first}"); -> L1 // first is now used here!
*   }
*
*   fn add_suffix(mut name: String) {
*       name.push_str(" Jr.");
*       name
*   }
* first points to deallocated memory after calling add_suffix. Reading first in println! would
* therefore be a violation of memory safety (undefined behaviour). Remember: it's not a problem
* that first points to deallocated memory. It's a probilem that we tried to use first after it
* became invalid.
*
* Thankfully, Rust will refuse to compile this program, giving the following error: value borrowed
* here after move or some shit like that.
*
* Let's walk through the steps of this error. Rust says that first is moved when we called
* add_suffix(first) on line 3. The error clarifies that first is moved beucase it has type String,
* which does not implement Copy. Finally, the error says that we use first after being moved (it's
* borrowed).
*
* So if you move a variable, Rust will stop you from using that variable later. More generally, the
* compiler will enforce this principle: If a variable x moves ownership of heap data to another
* variable y, then x cannot be used after the move.
*
* Moving ownership of heap data avoid undefined behavior from reading deallocated memory.
*
* Cloning Avoids Moves
*
* One way to avoid moving data is to clone it using the .clone() method. For example, we can fix
* the safety issue in the previous program with a clone:
*
*   fn main() {
*       let first = String::from("Ferris");
*       let first_clone = first.clone();        -> L1
*       let full = add_suffix(first_clone);     -> L2
*       println!("{full}, originally {first}");
*   }
*
*   fn add_suffix(mut name: String) -> String {
*       name.push_str(" Jr.");
*       name
*   }
* At L1, first_clone did not "shallow" copy the pointer in first, but instead "deep" copied the
* string data into a new heap allocation. Therefore at L2, while first_clone has been moved and
* invalidated by add_suffix, the original first variable is unchanged. It is safe to continue using
* first.
*
* Summary
*
* Ownership is primarily a discipline of heap management:
* - All heap data must be owned by exactly one variable.
* - Rust deallocates heap data once its owner goes out of scope.
* - Ownership can be transferred by moves, which happen on assignments and function calls.
* - Heap data can only be accessed through its current owner, not a previous owner.
*/
fn main() {
    let first = String::from("Can");
    let full = add_suffix(first);
    println!("My full name is {full}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Kocak");
    name
}
