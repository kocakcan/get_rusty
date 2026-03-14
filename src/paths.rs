/*
* Paths for Referring to an Item in the Module Tree
*
* To show Rust where to find an item in a module tree, we use a path in the same way we use a path
* when navigating a filesystem. To call a function, we need to know its path.
*
* A path can take two forms:
* - An absolute path is the full path starting from a crate root; for code from an external crate,
* the absolute path begins with the crate name, and for code from the current crate, it starts with
* the literal crate.
* - A relative path starts from the current module and uses self, super, or an identifier in the
* current module.
*
* Both absolute and relative paths are followed by one or more identifiers separated by double
* colons (::).
*
* Say we want to call the add_to_waitlist function. This is the same as asking: what's the path of
* the add_to_waitlist function?
*
* We'll show two ways to call the add_to_waitlist function from a new function, eat_at_restaurant,
* defined in the crate root. These paths are correct, but there's another problem remaining that
* will prevent this example from compiling as is.
*
* The eat_at_restaurant function is part of our library crate's public API, so we mark it with the
* pub keyword.
*
* The first time we call the add_to_waitlist function in eat_at_restaurant, we use an absolute
* path. The add_to_waitlist function is defined in the same crate as eat_at_restaurant, which means
* we can use the crate keyword to start an absolute path. We then include each of the successive
* modules until we make our way to add_to_waitlist. You can imagine a filesystem with the same
* structure: we'd specify the path /front_of_house/hosting/add_to_waitlist to run the
* add_to_waitlist program; using the crate name to start from the crate root is like using / to
* start from the filesystem root in your shell.
*/
