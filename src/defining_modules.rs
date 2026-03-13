/*
 * Defining Modules to Control Scope and Privacy
 *
 * paths allow you to name items. The use keyword brings a path into scope and the pub keyword make
 * items public.
 *
 * Modules Cheat Sheet
 *
 * - Start from the crate root: When compiling a crate, the compiler first looks in the crate root
 * file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to
 * compile.
 * - Declaring modules: In the crate root file, you can declare new modules; say you declare a
 * "garden" module with mod garden. The compiler will look for the module's code in these places:
 *      - Inline, within curly brackets that replace the semicolon following mod garden
 *      - In the file src/garden.rs
 *      - In the file src/garden/mod.rs
 * - Declaring submodules: In any file other than the crate root, you can declare submodules. For
 * example, you might declare mod vegetables; in src/garden.rs. The compiler will look for the
 * submodule's code within the directory named for the parent module in these places:
 *      - Inline, directly following mod vegetables, within curly brackets instead of semicolon
 *      - In the file src/garden/vegetables.rs
 *      - In the file src/garden/vegetables/mod.rs
 */
