/*
 * Error Handling
 *
 * Errors are a fact of life in software, so Rust has a number of features for handling situations
 * in which something goes wrong. In many cases, Rust requires you to acknowledge the
 * possibility of an error and take some action before your code will compile.
 *
 * Rust groups errors into two major categories: recoverable and unrecoverable errors. For a
 * recoverable error, such as a file not found error, we most likely just want to report the
 * problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs,
 * such as trying to access a location beyond the end of an array, and so we want to immediately
 * stop the program.
 *
 * Most languages don't distinguish between these kinds of errors and handle both in the same way,
 * using mechanisms such as exceptions. Rust doesn't have exceptions. Instead, it has the type
 * Result<T, E> for recoverable errors and the panic! macro that stops execution when the program
 * encounters an unrecoverable error. This chapter covers calling panic! first and then talks about
 * returning Result<T, E> values. Additionally, we'll explore considerations when deciding whether
 * to try to recover from an error or to stop execution.
 *
 * Unrecoverable Errors with panic!
 *
 * Sometimes bad things happen in your code, and there's nothing you can do about it. In these
 * cases, Rust has the panic! macro. There are two ways to cause panic in practice: by taking an
 * action that causes our code to panic (such as accessing an array past the end) or by explicitly
 * calling the panic! macro. In both cases, we cause a panic in our program. By default, these
 * panics will print a failure message, unwind, clean up the stack, and quit. Via an environment
 * variable, you can also have Rust display the call stack when a panic occurs to make it easier to
 * track down the source of the panic.
 *
 * Unwinding the Stack or Aborting in Response to a Panic
 *
 * By default, when a panic occurs, the program starts unwinding, which means Rust walks back up
 * the stack and cleans up the data from each function it encounters. However, walking back and
 * cleaning up is a lot of work. Rust therefore allows you to choose the alternative of immediately
 * aborting, which ends the program without cleaning up.
 *
 * Memory that the program was using will then need to be cleaned up by the operating system. If in
 * your project you need to make the resultant binary as small as possible, you can switch from
 * unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile]
 * sections in your Cargo.toml file. For example, if you want to abort on panic in release mode,
 * add this:
 *
 *  [profile.release]
 *  panic = 'abort'
 *
 * Let's try calling panic! in a simple program:
 *
 *  fn main() {
 *      panic!("crash and burn");
 *  }
 *
 * When you run the program, you'll see something like this:
 *
 *  thread 'main' panicked at src/main.rs:2:5:
 *  crash and burn
 *
 * The call to panic! causes the error message contained in the last two lines. The first line
 * shows our panic message and the place in our source code where the panic occurred:
 * src/main.rs:2:5 indicates that it's the second line, fifth character of our src/main.rs file.
 *
 * In this case, the line indicated is part of our code, and if we go to that line, we see the
 * panic! macro call. In other cases, the panic! call might be in code that our code calls, and the
 * filename and line number reported by the error message will be someone else's code where the
 * panic! macro is called, not the line of our code that eventually led to the panic! call.
 *
 * We can use the backtrace of the functions the panic! call name from to figure out the part of
 * our code that is causing the problem. To understand how to use a panic! backtrace, let's look at
 * another example and see what's it's like when a panic! call comes from a library because of a
 * bug in our code instead of from our code calling the macro directly. Below has some code that
 * attempts to access an index in a vector beyond the range of valid indexes.
 *
 *   fn main() {
 *      let v = vec![1, 2, 3];
 *      v[99];
 *   }
 * Here, we're attempting to access the 100th element of our vector (which is at index 99 because
 * indexing starts at zero), but the vector has only three elements. In this situation, Rust
 * will panic. Using [] is supposed to return an element, but if you pass an invalid index, there's
 * no element that Rust could return here that would be correct.
 *
 * In C, attempting to read beyond the end of a data structure is undefined behaviour. You might
 * get whatever is at the location in memory that would correspond to that element in the data
 * structure, even though the memory doesn't belong to that structure. This is called a buffer
 * overhead and can lead to security vulnerabilities if an attacker is able to manipulate the index
 * in such a way as to read data they shouldn't be allowed to that is stored after the data
 * structure.
 *
 * To protect your program from this sort of vulnerability, if you try to read an element at an
 * index that doesn't exist, Rust will stop execution and refuse to continue.
 *
 *  thread 'main' panicked at src/main.rs:4:6:
 *  index out of bounds: the len is 3 but the index is 99
 *
 * This error points at line 4 of our main.rs where we attempt to access index 99 of the vector in
 * v.
 *
 * The note: line tells us that we can set the RUST_BACKTRACE environment variable to get a
 * backtrace of exactly what happened to cause the error. A backtrace is a list of all the
 * functions that have been called to get to this point. Backtraces in Rust work as they do in
 * other languages: the key to reading the backtrace is to start from the top and read until you
 * see files you wrote. That's the spot where the problem originated. The lines above that spot are
 * code that your code has called; the lines below are code that called your code. These
 * before-and-after lines might include core Rust code, standard library code, or crates that
 * you're using. Let's try getting a backtrace by setting the RUST_BACKTRACE environment variable
 * to any value except 0.
 *
 * That's a lot of output! The exact output you see might be different depending on your operating
 * system and Rust version. In order to get backtraces with this information, debug symbols must be
 * enabled. Debug symbols are enabled by default when using cargo build or cargo run without the
 * --release flag, as we have here.
 *
 * In the output above, line 6 of the backtrace points to the line in our project that's causing
 * the problem: line 4 of src/main.rs. If we don't want our program to panic, we should start our
 * investigation at the location pointed to by the first line mentioning a file we wrote. Above,
 * where we deliberately wrote code that would panic, the way to fix the panic is to not request an
 * element beyond the range of the vector indexes. When your code panics in the future, you'll need
 * to figure out what action the code is taking with what values to cause the panic and what the
 * code should do instead.
 *
 * Recoverable Errors with Result
 */
fn main() {
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    v[99];
}
