/*
* Bringing Paths into Scope with the use Keyword
*
* Having to write out the paths to call functions can feel inconvenient and repetitive. Before,
* whether we chose the absolute or relative path to the add_to_waitlist function, every time we
* wanted to call add_to_waitlist we had specify front_of_house and hosting too. Fortunately,
* there's a way to simplify this process: we can create a shortcut to a path with the use keyword
* once, and then use the shorter name everywhere else in the scope.
*
* Below, we bring the crate::front_of_house::hosting module into the scope of the eat_at_restaurant
* function so we only have to specify hosting::add_to_waitlist to call the add_to_waitlist function
* in eat_at_restaurant.
*
* Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. By
* adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that
* scope, just as though the hosting module had been defined in the crate root. Paths brought into
* scope with use also check privacy, like any other paths.
*
* Note that use only creates the shortcut for the particular scope in which the use occurs. Below
* moves the eat_at_restaurant function into a new child module named customer, which is then a
* different scope than the use statement, so the function body won't compile.
*
* The compiler error shows that the shortcut no longer applies within the customer module:
*
*   failed to resolve: use of undeclared crate or module `hosting`
* Notice there's also a warning that the use is no longer used in its scope! To fix this problem,
* move the use within the customer module too, or reference the shortcut in the parent module with
* super::hosting within the child customer module.
*
* Creating Idiomatic use Paths
*/
