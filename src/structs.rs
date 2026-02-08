/*
* Defining and Instantiating Structs
*
* Structs are similar to tuples in that both hold multiple related values. Like tuples, the pieces
* of a struct can be different types. Unlike with tuples, in a struct you'll name each piece of
* data so it's clear what the values mean. Adding these names means that structs are more flexible
* than tuples: you don't have to rely on the order of the data to specify or access the values of
* an instance.
*
* To define a struct, we enter keyword struct and name the entire struct. A struct's name should
* describe the significance of the pieces of data being grouped together. Then, inside curly
* brackets, we define the names and types of the pieces of data, which we call fields. For example,
* the following struct stores information about a user account.
*
*   struct User {
*       active: bool,
*       username: String,
*       email: String,
*       sign_in_count: u64,
*   }
* To use a struct after we've defined it, we create an instance of that struct by specifying
* concrete values for each of the fields. We create an instance by stating the name of the struct
* and then add curly brackets containing key: value pairs, where the keys are the names of the
* fields and the values are the data we want to store in those fields. We don't have to specify the
* fields in the same order in which we declared them in the struct. In other words, the struct
* definition is like a general template for the type, and instances fill in that template with
* particular data to create values of the type.
*
*   fn main() {
*       let use1 = User {
*           email: String::from("someone@example.com"),
*           username: String::from("someusername123"),
*           active: true,
*           sign_in_count: 1,
*       };  -> L1
*   }
* To get a specific value from a struct, we use dot notation. For example, to access this user's
* email address, we use user1.email. If the instance is mutable, we can change a value by using the
* dot notation and assigning into a particular field.
*
*   fn main() {
*       let mut use1 = User {
*           email: String::from("someone@example.com"),
*           username: String::from("someusername123"),
*           active: true,
*           sign_in_count: 1,
*       };  -> L1
*       user1.email = String::from("anotheremail@example.com"); -> L2
*   }
* Note that the entire instance must be mutable; Rust doesn't allow us to mark only certain fields
* as mutable. As with any expression, we can construct a new instance of the struct as the last
* expression in the function body to implicitly return that new instance.
*
*  fn build_user(email: String, username: String) -> User {
*   User {
*       active: true,
*       username: username,
*       email: email,
*       sign_in_count: 1,
*   }
*  }
* It makes sense to name the function parameters with the same name as the struct fields, but
* having to repeat the email and username field names and variables is a bit tedious. If the struct
* had more fields, repeating each name would get even more annoying.
*
* Using the Field Init Shorthand
*
* Because the parameter names and the struct field names are exactly the same above we can use the
* field init shorthand syntax to rewrite build_user so it behaves exactly the same but doesn't have
* the repetition of username and email as shown below:
*
*   fn build_user(email: String, username: String) -> User {
*       User {
*           active: true,
*           username,
*           email,
*           sign_in_count: 1,
*       }
*   }
* Here, we're creating a new instance of the User struct, which has a field named email. We want to
* set the email field's value to the value in the email parameter of the build_user function.
* Because the email field and the email parameter have the same name, we only need to write email
* rather than email: email.
*
* Creating Instances from Other Instances with Struct Update Syntax
*
* It's often useful to create a new instance of a struct that includes most of the values from
* another instance of the same type, but changes some. You can do this using struct update syntax.
*
*   fn main() {
*       // --snip--
*
*       let user2 = User {
*           active: user1.active,
*           username: user1.username,
*           email: String::from("another@example.com"),
*           sign_in_count: user1.sign_in_count,
*       };  -> L1
*   }
* Using struct update syntax, we can achieve the same effect with less code, as shown below. The
* syntax .. specifies that the remaining fields not explicitly set should have the same values as
* the fields in the given instance.
*
*   fn main() {
*       // --snip--
*
*       let user2 = User {
*           email: String::from("another@example.com"),
*           ..user1
*       }
*   }
* The code above also creates an instance in user2 that has a different value for email but has the
* same values for the username, active, and sign_in_count fields from user1. The ..user1 must come
* last to specify that any remaining fields should get their values from the corresponding fields
* in user1, but we can choose to specify values for as many fields as we want in any order,
* regardless of the order of the fields in the struct's definition.
*
* Note that the struct update syntax uses = like an assignment; this is because it moves the data.
* In this example, after creating user2, user1 is partially invalidated because the String in the
* username field of user1 was moved into user2. If we had given user2 new String values for both
* email and username, and thus only user the active and sign_in_count values from user1, then user1
* would still be fully valid after creating user2. The types of active and sign_in_count are types
* that implement the Copy trait, so they can be copied.
*
* Using Tuple Structs Without Named Fields to Create Different Types
*
* Rust also supports structs that look similar to tuples, called tuple structs. Tuple structs have
* the added meaning the struct name provides but don't have names associated with their fields;
* rather, they just have the types of the fields. Tuple structs are useful when you want to give
* the whole tuple a name and make the tuple a different type from other tuples, and when naming
* each field as in a regular struct would be verbose or redundant.
*
* To define a tuple struct, start with the struct keyword and the struct name followed by the types
* in the tuple. For example, here we define and use two tuple structs named Color and Point:
*
*   struct Color(i32, i32, i32);
*   struct Point(i32, i32, i32);
*
*   fn main() {
*       let black = Color(0, 0, 0);
*       let origin = Point(0, 0, 0);
*   }
* Note that the black and origin values are different types because they're instanes of different
* tuple structs. Each struct you define is its own type, even though the fields within the struct
* might have the same types. For example, a function that takes a parameter of type Color cannot
* take a Point as an argument, even though both types are made up of three i32 values. Otherwise,
* tuple struct instances are similar to tuples in that you can destructure them into their
* individual pieces, and you can use . followed by the index to access an individual value. Unlike
* tuples, tuple structs require you to name the type of the struct when you destructure them. For
* example, we would write let Point(x, y, z) = origin; to destructure the values in the origin
* point into variables name x, y, and z.
*/
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        sign_in_count: 1,
        email: String::from("someone@example.com"),
    };

    let user2 = User {
        active: false,
        sign_in_count: 2,
        ..user1
    };

    let user3 = build_user(
        String::from("clwy"),
        String::from("komiksivasli@hotmail.com"),
    );

    let users = vec![user2, user3];

    for user in users {
        println!(
            "{} has logged into the application {} times so far",
            user.username, user.sign_in_count
        );
    }

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(x, y, z) = origin;
    println!("({}, {}, {})", x, y, z);
}
