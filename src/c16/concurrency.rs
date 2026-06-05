/// Fearless Concurrency
///
/// Handling concurrent programming safely and efficiently is another of Rust's
/// major goals. Concurrent programming, in which different parts of a program
/// execute independently, and parallel programming, in which different parts o
/// f a prgoram execute at the same time, are becoming increasingly important a
/// s more computers take advantage of their multiple processors. Historically,
/// programming in these contexts has been difficult and error-prone.
///
/// Using Threads to Run Code Simultaneously
///
/// In most current operating systems, an executed program's code is run in a p
/// rocess, and the operating system will manage multiple processes at once. Wi
/// thin a program, you can also have independent parts that run simultaneously
/// . The features that run these independent parts are called threads. For exa
/// mple, a web server could have multiple threads so that it can respond to mo
/// re than one request at the same time.
///
/// Splitting the computation in your program into multiple threads to run mult
/// iple tasks at the same time can improve performance, but it also adds compl
/// exity. Because threads can run simultaneously, there's no inherent guarante
/// e about the order in which parts of your code on different threads will run
/// . This can lead to problems, such as:
///
///     - Race conditions, in which threads are accessing data or resources in
///     an inconsistent order
///     - Deadlocks, in which two threads are waiting for each other, preventin
///     g both threads from continuing
///     - Bugs that only happen in certain situations and are hard to reproduce
///     and fix reliably
///
/// The Rust standard library uses 1:1 model of thread implementation, whereby
/// a program uses one operating system thread per one language thread. There a
/// re crates that implement other models of threading that make different trad
/// e-offs to the 1:1 model.
///
/// Creating a New Thread with spawn
///
/// To create a new thread, we call the thread::spawn function and pass it a cl
/// osure containing the code we want to run in the new thread. The example in
/// Listing 16-1 prints some text from a main thread and other text from a new
/// thread.
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi numner {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
