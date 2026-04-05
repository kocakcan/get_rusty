/// Error Handling
///
/// Errors are a fact of life in software, so Rust has a number of features for handling situations
/// in which something goes wrong. In many cases, Rust requires you to acknowledge the
/// possibility of an error and take some action before your code will compile.
///
/// Rust groups errors into two major categories: recoverable and unrecoverable errors. For a
/// recoverable error, such as a file not found error, we most likely just want to report the
/// problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs,
/// such as trying to access a location beyond the end of an array, and so we want to immediately
/// stop the program.
///
/// Most languages don't distinguish between these kinds of errors and handle both in the same way,
/// using mechanisms such as exceptions. Rust doesn't have exceptions. Instead, it has the type
/// Result<T, E> for recoverable errors and the panic! macro that stops execution when the program
/// encounters an unrecoverable error. This chapter covers calling panic! first and then talks about
/// returning Result<T, E> values. Additionally, we'll explore considerations when deciding whether
/// to try to recover from an error or to stop execution.
///
/// Unrecoverable Errors with panic!
///
/// Sometimes bad things happen in your code, and there's nothing you can do about it. In these
/// cases, Rust has the panic! macro. There are two ways to cause panic in practice: by taking an
/// action that causes our code to panic (such as accessing an array past the end) or by explicitly
/// calling the panic! macro. In both cases, we cause a panic in our program. By default, these
/// panics will print a failure message, unwind, clean up the stack, and quit. Via an environment
/// variable, you can also have Rust display the call stack when a panic occurs to make it easier to
/// track down the source of the panic.
///
/// Unwinding the Stack or Aborting in Response to a Panic
///
/// By default, when a panic occurs, the program starts unwinding, which means Rust walks back up
/// the stack and cleans up the data from each function it encounters. However, walking back and
/// cleaning up is a lot of work. Rust therefore allows you to choose the alternative of immediately
/// aborting, which ends the program without cleaning up.
///
/// Memory that the program was using will then need to be cleaned up by the operating system. If in
/// your project you need to make the resultant binary as small as possible, you can switch from
/// unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile]
/// sections in your Cargo.toml file. For example, if you want to abort on panic in release mode,
/// add this:
///
///  [profile.release]
///  panic = 'abort'
///
/// Let's try calling panic! in a simple program:
///
///  fn main() {
///      panic!("crash and burn");
///  }
///
/// When you run the program, you'll see something like this:
///
///  thread 'main' panicked at src/main.rs:2:5:
///  crash and burn
///
/// The call to panic! causes the error message contained in the last two lines. The first line
/// shows our panic message and the place in our source code where the panic occurred:
/// src/main.rs:2:5 indicates that it's the second line, fifth character of our src/main.rs file.
///
/// In this case, the line indicated is part of our code, and if we go to that line, we see the
/// panic! macro call. In other cases, the panic! call might be in code that our code calls, and the
/// filename and line number reported by the error message will be someone else's code where the
/// panic! macro is called, not the line of our code that eventually led to the panic! call.
///
/// We can use the backtrace of the functions the panic! call name from to figure out the part of
/// our code that is causing the problem. To understand how to use a panic! backtrace, let's look at
/// another example and see what's it's like when a panic! call comes from a library because of a
/// bug in our code instead of from our code calling the macro directly. Below has some code that
/// attempts to access an index in a vector beyond the range of valid indexes.
///
///   fn main() {
///      let v = vec![1, 2, 3];
///      v[99];
///   }
/// Here, we're attempting to access the 100th element of our vector (which is at index 99 because
/// indexing starts at zero), but the vector has only three elements. In this situation, Rust
/// will panic. Using [] is supposed to return an element, but if you pass an invalid index, there's
/// no element that Rust could return here that would be correct.
///
/// In C, attempting to read beyond the end of a data structure is undefined behaviour. You might
/// get whatever is at the location in memory that would correspond to that element in the data
/// structure, even though the memory doesn't belong to that structure. This is called a buffer
/// overhead and can lead to security vulnerabilities if an attacker is able to manipulate the index
/// in such a way as to read data they shouldn't be allowed to that is stored after the data
/// structure.
///
/// To protect your program from this sort of vulnerability, if you try to read an element at an
/// index that doesn't exist, Rust will stop execution and refuse to continue.
///
///  thread 'main' panicked at src/main.rs:4:6:
///  index out of bounds: the len is 3 but the index is 99
///
/// This error points at line 4 of our main.rs where we attempt to access index 99 of the vector in
/// v.
///
/// The note: line tells us that we can set the RUST_BACKTRACE environment variable to get a
/// backtrace of exactly what happened to cause the error. A backtrace is a list of all the
/// functions that have been called to get to this point. Backtraces in Rust work as they do in
/// other languages: the key to reading the backtrace is to start from the top and read until you
/// see files you wrote. That's the spot where the problem originated. The lines above that spot are
/// code that your code has called; the lines below are code that called your code. These
/// before-and-after lines might include core Rust code, standard library code, or crates that
/// you're using. Let's try getting a backtrace by setting the RUST_BACKTRACE environment variable
/// to any value except 0.
///
/// That's a lot of output! The exact output you see might be different depending on your operating
/// system and Rust version. In order to get backtraces with this information, debug symbols must be
/// enabled. Debug symbols are enabled by default when using cargo build or cargo run without the
/// --release flag, as we have here.
///
/// In the output above, line 6 of the backtrace points to the line in our project that's causing
/// the problem: line 4 of src/main.rs. If we don't want our program to panic, we should start our
/// investigation at the location pointed to by the first line mentioning a file we wrote. Above,
/// where we deliberately wrote code that would panic, the way to fix the panic is to not request an
/// element beyond the range of the vector indexes. When your code panics in the future, you'll need
/// to figure out what action the code is taking with what values to cause the panic and what the
/// code should do instead.
///
/// Recoverable Errors with Result
///
/// Most errors aren't serious enough to require the program to stop entirely. Sometimes when a
/// function fails it's for a reason that you can easily interpret and respond to. For example, if you
/// try to open a file and that operation fails because the file doesn't exist, you might want to
/// create the file instead of terminating the process.
///
/// Result enum is defined as having two variants, Ok and Err, as follows:
///
///   enum Result<T, E> {
///       Ok(T),
///       Err(E),
///   }
/// The T and E are generic type parameters. T represents the type of the value that will be returned
/// in a success case within the Ok variant, and E represents the type of the error that will be
/// returned in a failure case within the Err variant. Because Result has these generic type
/// parameters, we can use the Result type and the functions defined on it in many different
/// situations where the success value and error value we want to return may differ.
///
///   use std::fs::File;
///
///   fn main() {
///       let greeting_file_result = File::open("hello.txt");
///   }
/// The return type of File::open is a Result<T, E>. The generic parameter T has been filled in by the
/// implementation of File::open with the type of the success value, std::fs::File, which is a file
/// handle. The type of E used in the error value is std::io::Error. This return type means the calls
/// to File::open might succeed and return a file handle that we can read from or write to. The
/// function call also might fail: for example the file might not exist, or we might not have
/// permission to access the file. The File::open function needs to have a way to tell us whether it
/// succeeded or failed and at the same time give us either the file handle or error information. This
/// information is exactly what the Result enum conveys.
///
/// In the case where File::open succeeds, the value in the variable greeting_file_result will be an
/// instance of Ok that contains a file handle. In the case where it fails, the value in
/// greeting_file_result will be an instance of Err that contains more information the kind of error
/// that occurred.
///
/// We need to add the code above to take different actions depending on the value File::open returns.
/// Below shows one way to handle the Result using a basic tool, the match expression.
///
///   use std::fs::File;
///
///   fn main() {
///       let greeting_file_result = File::open("hello.txt");
///
///       let greeting_file = match greeting_file_result {
///           Ok(file) => file,
///           Err(error) => panic!("Problem opening the file: {error:?}"),
///       };
///   }
/// Note that, the Option enum, the Result enum and its variants have been brought into scope by the
/// prelude, so we don't need to specify Result:: before the Ok and Err variants in the match arms.
///
/// When the result is Ok, this code will return the inner file value out of the Ok variant, and we
/// then assign that file handle value to the variable greeting_file. After the match, we can use the
/// file handle for reading or writing.
///
/// The other arm of the match handles the case where we get and Err value from File::open. In this
/// example, we've chosen to call the panic! macro. If there's no file named hello.txt in our current
/// directory and we run this code, we'll see the following output from the panic! macro:
///
///   thread 'main' panicked at src/main.rs:8:23:
///   Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
/// As usual, this output tells us exactly what has gone wrong.
///
/// Matching on Different Errors
///
/// The code above will panic! no matter why File::open failed. However, we want to take different
/// actions for different failure reasons. If File::open failed because the file doesn't exist, we
/// want to create the file and return the handle to the new file. If File::open failed for any
/// other reason--for example, because we didn't have permission to open the file--we still want the
/// code to panic! in the same way it did above. For this, we add an inner match expression
/// shown below
///
///     use std::fs::File;
///     use std::io::ErrorKind;
///
///     fn main() {
///         let greeting_file_result = File::open("hello.txt");
///
///         let greeting_file = match greeting_file_result {
///             Ok(file) => file,
///             Err(error) => match error.kind() {
///                 ErrorKind::NotFound => match File::create("hello.txt") {
///                     Ok(fc) => fc,
///                     Err(e) => panic!("Problem creating the file: {e:?}"),
///                 },
///                 _ => {
///                     panic!("Problem opening the file: {error:?}");
///                 }
///             },
///         },
///     };
///
/// The type of the value that File::open returns inside the Err variant is io::Error, which is a
/// struct provided by the standard library. This struct has a method kind that we can call to get
/// an io::ErrorKind value. The enum io::ErrorKind is provided by the standard library and has
/// variants representing the different kinds of errors that might result from an io operation. The
/// variant we want to use is ErrorKind::NotFound, which indicates the file we're trying to open
/// doesn't exist yet. So we match on greeting_file_result, but we also have an inner match on
/// error.kind().
///
/// The condition we want to check in the inner match is whether the value returned by error.kind()
/// is the NotFound variant of the ErrorKind enum. If it is, we try to create the file with
/// File::create. However, because File::create could also fail, we need a second arm in the inner
/// match expression. When the file can't be created, a different error message is printed. The
/// second arm of the outer match stays the same, so the program panics on any error besides the
/// missing file error.
///
/// Alternatives to Using match with Result<T, E>
///
/// That's a lot of match! The match expression is very useful but also very much a primitive.
/// Closures are used with many of the methods defined on Result<T, E>. These methods can be more
/// concise than using match when handling Result<T, E> values in your code.
///
/// For example, here's another way to write the same logic as shown below, this time using closures
/// and the unwrap_or_else method:
///
///     use std::fs::File;
///     use std::io::ErrorKind;
///
///     fn main() {
///         let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
///             if error.kind() == ErrorKind::NotFound {
///                 File::create("hello.txt").unwrap_or_else(|error| {
///                     panic!("Problem creating the file: {error:?}");
///                 })
///             } else {
///                 panic!("Problem opening the file: {error:?}");
///             }
///         });
///     }
/// Although this code has the same behaviour as before, it doesn't contain any match expressions
/// and is cleaner to read.
///
/// File::open returns a Result and not an Option because Result can represent why an operation
/// failed, and file opening can fail for many reasons.
///
/// Shortcuts for Panic on Error: unwrap and except
///
/// Using match works well enough, but it can be a bit verbose and doesn't always communicate intent
/// well. The Result<T, E> type has many helper methods defined on it to do various, more specific
/// tasks. The unwrap method is a shortcut method implemented just like the match expression we
/// wrote above. If the Result value is the Ok variant, unwrap will return the value inside the Ok.
/// If the Result is the Err variant, unwrap will call the panic! macro for us.
///
///     use std::fs::File;
///
///     fn main() {
///         let greeting_file = File::open("hello.txt").unwrap();
///     }
/// If we run this code without a hello.txt file, we'll see an error message from the panic! call
/// that the unwrap method makes:
///
///     thread 'main' panicked at src/main.rs:4:49
///     called `Result::unwrap()` or an `Err` value: Os { code: 2, kind: NotFound, message: "No such
///     file or directory" }
/// Similarly, the expect method lets us also choose the panic! error message. Using expect instead
/// of unwrap and providing good error messages can convey your intent and make tracking down the
/// source of a panic easier. The syntax of expect looks like this:
///
///     use std::fs::File;
///
///     fn main() {
///         let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this
///         project.");
///     }
/// We use expect in the same way as unwrap: to return the file handle or call the panic! macro. The
/// error message used by expect in its call to panic! will be the parameter that we pass to expect,
/// rather than the default panic! message that unwrap uses. Here's what it looks like:
///
///     thread 'main' panicked at src/main.rs:5:10:
///     hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No suh
///     file or directory" }
/// In production-quality code, most Rustaceans choose expect rather than unwrap and give more
/// context about why the operation is expected to always succeed. That way, if your assumptions are
/// ever proven wrong, you have more information to use in debugging.
///
/// Propagating Errors
///
/// When a function's implementation calls something that might fail, instead of handling the error
/// within the function itself, you can return the error to the calling code so that it can decide
/// what to do. This is known as propagating the error and gives more control to the calling code,
/// where there might be more information or logic that dictates how the error should be handled
/// than what you have available in the context of your code.
///
/// For example, below shows a function that reads a username from a file. If the file doesn't exist
/// or can't be read, this function will return those errors to the code that called the function.
///
///     use std::fs::File;
///     use std::io::{self, Read}
///
///     fn read_username_from_file() -> Result<String, io::Error> {
///         let username_file_result = File::open("hello.txt");
///
///         let mut username_file = match username_file_result {
///             Ok(file) => file,
///             Err(e) => return Err(e),
///         }
///         let mut username = String::new();
///         match username_file.read_to_string(&mut username) {
///             Ok(_) => Ok(username),
///             Err(e) => Err(e),
///         }
///     }
/// This function can be written in a much shorter way. Let's look a the return type of the function
/// first: Result<String, io::Error>. This means the function is returning a value of the type
/// Result<T, E>, where the generic parameter T has been filled in with the concrete type String and
/// the generic type E has been filled in with the concrete type io::Error.
///
/// If this function succeeds without any problems, the code that calls this function will receive
/// an Ok value that holds a String--the username that this function read from the file. If this
/// function encounters any problems, the calling code will receive an Err value that holds and
/// instance of io::Error that contains more information about what the problems were. We chose
/// io::Error as the return type of this function because that happens to be the type of the error
/// value returned from both of the operations we're calling in this function's body that might
/// fail: the File::open function and the read_to_string method.
///
/// The body of the function starts by calling the File::open function. Then we handle the Result
/// value with a match similar to the match above. If File::open succeeds, the file handle in the
/// pattern variable file becomes the value in the mutable variable username_file and the function
/// continues. In the Err case, instead of calling panic!, we use the return keyword to return early
/// out of the function entirely and pass the error value from File::open, now in the pattern
/// variable e, back to the calling code as this function's error value.
///
/// So, if we have a file handle in username_file, the function then creates a new String in
/// variable username and calls the read_to_string method on the file handle in username_file to
/// read the contents of the file into username. The read_to_string method also returns a Result
/// because it might fail, even though File::open succeeded, So we need another match to handle that
/// Result: if read_to_string succeeds, then our function has succeeded, and we return the username
/// from the file that's now in username wrapped in an Ok. If read_to_string fails, we return the
/// error value in the same way that we returned the error value in the match that handled the
/// return value of File::open. However, we don't need to explicitly say return, because this is the
/// last expression in the function.
///
/// The code that calls this code will then handle getting an Ok value that contains a username or
/// an Err value that contains an io::Error. It's up to the calling code to decide what to do with
/// those values. If the calling code gets an Err value, it could call panic! and crash the prgoram,
/// user a default username, or look up the username from somewhere other than a file, for example.
/// We don't have enough information on what the calling code is actually trying to do, so we
/// propragate all the success or error information upward for it to handle appropriately.
///
/// This pattern of propagating errors is so common in Rust that Rust provides the question mark
/// operator ? to make this easier.
///
/// A Shortcut for Propagating Errors: The ? Operator
///
/// Below shows an implementation of read_username_from_file that has the same functionality as
/// above, but this implementation uses the ? operator.
///
///     use std::fs::File;
///     use std::io::{self, Read};
///
///     fn read_username_from_file() -> Result<String, io::Error> {
///         let mut username_file = File::open("hello.txt")?;
///         let mut username = String::new();
///         username_file.read_to_string(&mut username)?;
///         Ok(username)
///     }
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];
    // let greeting_file_result = File::open("hello.txt");
    //
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening file: {error:?}"),
    // };
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}
