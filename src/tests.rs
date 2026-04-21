/// How to Write Tests
///
/// Tests are Rust functions that verify the non-test code is functioning in the expected manner.
/// The bodies of test functions typically perform these three actions:
///
///     - Set up any needed data or state.
///     - Run the code you want to test.
///     - Assert that the results are what you expect.
/// Rust provides features such as test attribute, a few macros, and the should_panic attribute for
/// writing tests that take these actions.
///
/// The Anatomy of a Test Function
///
/// At its simplest, a test in Rust is a function that's annotated with the test attribute.
/// Attributes are metadata about pieces of Rust code; one example is the derive attribute we used
/// with structs. To change a function into a test function, add #[test] on the line before fn.
/// When you run your tests with the cargo test command, Rust builds a test runner binary that runs
/// the annotated functions and reports on whether each test function passes or fails.
///
/// Whenever we make a new library project with Cargo, a test module with a test function in it is
/// automatically generated for us. This module gives you a template for writing your tests so you
/// don't have to look up the exact structure and syntax every time you start a new project. You can
/// add as many additional test functions and as many test modules as you want!
///
/// Checking Results with the assert! Macro
///
/// The assert! macro, provided by the standard library, is useful when you want to ensure that
/// some condition in a test evaluates to true. We give the assert! macro an argument that
/// evaluates to a Boolean. If the value is true, nothing happens and the test passes. If the value
/// is false, the assert! macro calls panic! to cause the test to fail. Using the assert! macro
/// helps us check that our code is functioning in the way we intend.
///
/// Testing Equality with assert_eq! and assert_ne! Macros
