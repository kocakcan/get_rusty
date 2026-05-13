/// RefCell<T> and the Interior Mutability Pattern
///
/// Interior mutability is a design pattern in Rust that allows you to mutate data even when there
/// are immutable references to that data; normally, this action is disallowed by the borrowing
/// rules. To mutate data, the pattern uses unsafe code inside a data structure to bend Rust's
/// usual rules that govern mutation and borrowing. Unsafe code indicates to the compiler that
/// we're checking the rules manually instead of relying on the compiler to check them for us.
///
/// We can use types that use the interior mutability pattern only when we can ensure that the
/// borrowing rules will be followed at runtime, even though the compiler can't guarantee that. The
/// unsafe code involved is then wrapped in a safe API, and the outer type is still immutable.
///
/// Enforcing Borrowing Rules at Runtime
///
/// Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds. So, what
/// makes RefCell<T> different from a type like Box<T>?
///
///     - At any given time, you can have either one mutable reference or any number of immutable
///     references (but not both).
///     - References must always be valid.
/// With references and Box<T>, the borrowing rules' invariants are enforced at compile time. With
/// RefCell<T>, these invariants are enforced at runtime. With references, if you break these
/// rules, you'll get a compiler error. With RefCell<T>, if you break these rules, your program
/// will panic and exit.
///
/// The advantages of checking the borrowing rules at compile time are that errors will be caught
/// sooner in the development process, and there is no impact on runtime performance because all
/// the analysis is computed beforehand. For those reasons, checking the borrowing rules at compile
/// time is the best choice in the majority of cases, which is why this is Rust's default.
///
/// The advantage of checking the borrowing rules at runtime instead is that certain memory-safe
/// scenarios are then allowed, where they would've been disallowed by the compile-time checks.
/// Static analysis, like the Rust compiler, is inherently conservative. Some properties of code
/// are impossible to detect by analyzing the code.
///
/// Because some analysis is impossible, if the Rust compiler can't be sure the code complies with
/// the ownership rules, it might reject a correct program; in this way, it's conservative. If Rust
/// accepted an incorrect program, users wouldn't be able to trust the guarantees Rust makes.
/// However, if Rust rejects a correct program, the programmer will be inconvenienced, but nothing
/// catastrophic can occur. The RefCell<T> type is useful when you're sure your code follows the
/// borrowing rules but the compiler is unable to understand and guarantee that.
///
/// Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a
/// compile-time error if you try using it in a multithreaded context.
///
/// Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
///
///     - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
///     - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only
///     immutable borrows checked at compile time; RefCell<t> allows immutable or mutable borrows
///     checked at runtime.
///     - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value
///     inside the RefCell<T> even when the RefCell<T> is immutable.
/// Mutating the value inside an immutable value is the interior mutability pattern.
///
/// Using Interior Mutability
///
/// A consequence of the borrowing rules is that when you have an immutable value, you can't borrow
/// it mutably. For example, this code won't compile:
///
///     fn main() {
///         let x = 5;
///         let y = &mut x;
///     }
/// If you tried to compile this code, you'd get the following error:
///
///     error: cannot borrow `x` as mutable, as it is not declared as mutable
/// However, there are situations in which it would be useful for a value to mutate itself in its
/// methods but appear immutable to other code. Code outside the value's methods would not be able
/// to mutate the value. Using RefCell<T> is one way to get the ability to have interior
/// mutability, but RefCell<T> doesn't get around the borrowing rules completely: The borrow
/// checker in the compiler allows this interior mutability, and the borrowing rules are checked at
/// runtime instead. If you violate the rules, you'll get a panic! instead of a compiler error.
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, values: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
