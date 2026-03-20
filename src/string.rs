/*
* What is a String?
*
* Rust has only one string type in the core language, which is the string slice str that is usually
* seen in its borrowed form &str. String slices are references to some UTF-8 encoded string data
* stored elsewehere. String literals, for example, are stored in the program's binary and therefore
* string slices.
*
* The String type, which is provided by Rust's standard library rather than coded into the core
* language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to
* "strings" in Rust, they might be referring to either the String or the string slice &str types,
* not just one of those types. Both String and string slices are UTF-8 encoded.
*
* Creating a New String
*
* Many of the same operations available with Vec<T> are available with String as well because
* String is actually implemented as a wrapper around a vector of bytes wiith some extra
* guarantees,, restrictions, and capabilities. An example of a function that works the same way
* with Vec<T> and String is the new function to create an instance, shown below.
*
*   let mut s = String::new();
* This line creates a new, empty string called s, into which we can then load data. Often, we'll
* have some initial data with which we want to start the string. For that, we use the to_string
* method, which is available on any type that implements the Display trait, as string literals do.
*
*   let data = "initial contents";
*   let s = data.to_string();
*   let s = "initial contents".to_string(); -> also works on a literal directly
* This code creates a string containing initial contents.
* We can also use the function String::from to create a String from a string literal. The code
* below is equivalent to the code above.
*
*   let s = String::from("initial contents");
* Because strings are used for so many things, we can use many different generic APIs for strings,
* providings us with a lot of options. Some of them can seem redundant, but they all have their
* place! In this case, String::from and to_string do the same thing, so which one you choose is a
* matter of style and readability.
*
* Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them, as
* shown below:
*
*   let hello = String::from("السلام عليكم");
*   let hello = String::from("Dobrý den");
*   let hello = String::from("Hello");
*   let hello = String::from("שלום");
*   let hello = String::from("नमस्ते");
*   let hello = String::from("こんにちは");
*   let hello = String::from("안녕하세요");
*   let hello = String::from("你好");
*   let hello = String::from("Olá");
*   let hello = String::from("Здравствуйте");
*   let hello = String::from("Hola");
* All of these are valid String values.
*
* Updating a String
*
* A String can grow in size and its contents can change, just like the contents of a Vec<T>, if you
* push more data into it. In addition, you can conveniently use the + operator or the format! macro
* to concatenate String values.
*
* Appending to a String with push_str and push
*
* We can grow a String by using the push_str method to append a string slice, as shown below.
*
*   let mut s = String::from("foo");
*   s.push_str("bar");
* After these two lines, s will contain foobar. The push_str method takes a string slice because we
* don't necessarily want to take ownership of the parameter. For example, in the code below, we
* want to be able to use s2 after appending its contents to s1.
*
*   let mut s1 = String::from("foo");
*   let s2 = "bar";
*   s1.push_str(s2);
*   println!("s2 is {s2}");
* If the push_str method took ownership of s2, we wouldn't be able to print its value on the last
* line. However, this code works as we'd expect!
*
* The push method takes a single character as a parameter and adds it to the String. The code below
* add the letter / to a String using the push method.
*
*   let mut s = String::from("lo");
*   s.push('l');
* As a result, s will contain lol.
*/
fn greetings(name: &str) {
    println!("Greetings {name}!");
}

fn main() {
    let mut name = String::from("Can");
    name.push_str(" Kocak");
    greetings(&name);
}
