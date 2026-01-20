/*
* What is Ownership?
*
* Ownership is a discipline for ensuring the safety of Rust programs. To understand ownership, we
* first need to understand what makes a Rust program safe (or unsafe).
*
* Safety is the Absence of Undefined Behaviour
*/

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    let x = true;
    read(x);
}

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
*/
