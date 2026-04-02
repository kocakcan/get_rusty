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
 */
fn main() {
    panic!("crash and burn");
}
