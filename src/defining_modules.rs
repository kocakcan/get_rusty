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
*  - Paths to code in modules: Once a module is part of your crate, you can refer to code in that
*  module from anywhere else in that same crate, as long as the privacy rules allow, using the path
*  to the code. For example, an Asparagus type in the garden vegetables module would be found at
*  crate::garden::vegetables::Asparagus.
*  - Private vs. public: Code within a module is private from its parent modules by default. To
*  make a module public, declare it with pub mod instead of mod. To make items within a public
*  module public as well, use pub before their declarations.
*  - The use keyword: Within a scope, the use keyword creates shortcuts to items to reduce
*  repetition of long paths, In any scope that you can refer to
*  crate::garden::vegetables::Asparagus, you can create a shortcut with use
*  crate::garden::vegetables::Asparagus; and from then on you only need to write Asparagus to make
*  use of that type in the scope.
*
*  Grouping Related Code in Modules
*
*  Modules let us organize code within a crate for readability and easy reuse. Modules also allow
*  us to control the privacy of items because code within a module is private by default. Private
*  items are internal implementation details not available for outside use. We can choose to make
*  modules and the items within them public, which exposes them to allow external code to use and
*  depend on them.
*
*  In the restaurant industry, some parts of a restaurant are referred to as front of house and
*  others as back of house. Front of house is where customers are; this encompasses where the hosts
*  seat customers, servers take oreders and payment, and bartenders make drinks. Back of house is
*  where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do
*  administrative work.
*
*  To structure our crate in this way, we can organize functions into nested modules. Create a new
*  library named restaurant by running cargo new restaurant --lib. Then enter the code below to
*  define some modules and function signatures; this code is the front of house section:
*
*  We define a module with the mod keyword followed by the name of the module (in this case,
*  front_of_house). The body of the module then goes inside curly brackets. Inside modules, we can
*  place other modules, as in this case with the modules hosting and serving. Modules can also hold
*  definitions for other items, such as structs, enums, constants, traits, functions.
*
*  By using modules, we can group related definitions together and name why they're related.
*  Programmers using this code can navigate the code based on the groups rather than having to read
*  through all the definitions, making it easier to find the definitions relevant to them.
*  Programmers adding new functionality to this code would know where to place the code to keep the
*  program organized.
*
*  Earlier, we mentioned that src/main.rs and src/lib.rs are called crate roots. The reason for
*  their name is that the contents of either of these two files form a module named crate at the
*  root of the crate's module structure, known as the module tree.
*
*  crate
*    └── front_of_house
*       ├── hosting
*       │   ├── add_to_waitlist
*       │   └── seat_at_table
*       └── serving
*           ├── take_order
*           ├── serve_order
*           └── take_payment
*
* This tree shows how some of the modules nest inside other modules; for example hosting nests
* inside front_of_house. The tree also shows that some modules are siblings, meaning they're
* defined in the same module; hosting and serving are siblings defined within front_of_house. If
* module A is contained inside module B, we say that module A is the child of module B and that
* module B is the parent of module A. Notice that the entire module tree is rooted under the
* implicit module named crate.
*
* The module tree might remind you of the filesystem's directory tree on your computer; this is a
* very apt comparison! Just like directories in a filesystem, you use modules to organize your
* code. And just like files in a directory, we need a way to find our modules.
 */
