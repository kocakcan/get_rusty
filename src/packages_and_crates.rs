/*
 * Packages: A Cargo feature that lets you build, test, and share crates
 * Crates: A tree of modules that produces a library or executable
 * Modules and use: Let you control the organization, scope, and privacy of paths
 * Paths: A way of naming an item, such as a struct, function, or module
 *
 * Packages and Crates
 *
 * A crate is the smallest amount of code that the Rust compiler considers at a time. Even if you
 * run rustc rather than cargo and pass a single source code file, the compiler considers that file
 * to be a crate. Crates can contain modules, and the modules may be defined in other files that
 * get compiled with the crate.
 *
 * A crate can come in one of two forms: a binary crate or a library crate. Binary crates are
 * programs you can compile to an executable that you can run, such as a command-line program or a
 * server. Each must have a function called main that defines what happens when the executable
 * runs.
 *
 * Library crates don't have a main function, and they don't compile to an executable. Instead,
 * they define functionality intended to be shared with multiple projects. For example, the rand
 * crate provides functionality that generates random numbers. Most of the time when we mention
 * "crate" we mean library crate. Crate can be used interchangeably with the general programming
 * concept of "library".
 *
 * The crate root is a source file that the Rust compiler starts from and makes up the root module
 * of your crate.
 *
 * A package is a bundle of one or more crates that provides a set of functionality. A package
 * contains a Cargo.toml file that describes how to build those crates. Cargo is actually a package
 * that contains the binary crate for the command line tool you've been using to build your code.
 * The Cargo package also contains a library crate that binary crate depends on. Other projects can
 * depend on the Cargo library crate to use the same logic the Cargo command line tool uses.
 *
 * A package can contain as many binary crates as you like, but at most only one library crate. A
 * package must contain at least one crate, whether that's a library of binary crate.
 */
