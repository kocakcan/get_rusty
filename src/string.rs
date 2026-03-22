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
*
* Concatenation with the + Operator or the format! Macro
*
* Often, you'll want to combine two existing strings. One way to do so is to use the + operator, as
* shown below.
*
*   let s1 = String::from("Hello, ");
*   let s2 = String::from("world!");
*   let s3 = s1 + &s2;   -> note s1 has been moved here and can no longer be used
* The string s3 will contain Hello, world!. The reason s1 is no longer valid after the addition,
* and the reason we used a reference to s2, has to do with the signature of the method that's
* called when we use the + operator. The + operator uses the add method, whose signature looks
* something like this:
*
*   fn add(self, s: &str) -> String {
* In the standard library, you'll see add defined using generics and assoicated types. Here, we've
* substituted in concrete types, which is what happens when we call this method with String values.
* This signature gives us the clues we need in order to understand the tricky bits of the + operator.
*
* First, s2 has an &, meaning that we're adding a reference of the second string to the first
* string. This is because of the s parameter in the add function: we can only add a &str to a
* String; we can't add to String values together. But wait--the type of &s2 is String, not &str, as
* specified in the second parameter to add. So why does this code compile?
*
* The reason we're able to use &s2 in the call to add is that the compiler can coerce the &String
* argument into &str. When we call the add method, Rust uses a deref coercion, which here turns &s2
* into &s2[..]. Because add does not take ownership of the s parameter, s2 will still be a valid
* String after the operation.
*
* Second, we can see in the signature that add takes ownership of self, because self does not have
* an &. This means s1 above will be moved into the add call and will no longer be valid after that.
* So, although let s3 = s1 + &s2; looks like it will copy both strings and create a new one, this
* statment instead does the following:
*
*   1. add takes ownership of s1,
*   2. it appends a copy of the contents of s2 to s1,
*   3. and then it returns back ownership of s1.
* If s1 has enough capacity for s2, then no memory allocations occur. However, if s1 does not have
* enough capacity for s2, then s1 will internally make a larger memory allocation to fit both strings.
*
* If we need to concatenate multiple strings, the behaviour of the + operator gets unwieldy:
*
*   let s1 = String::from("tic");
*   let s2 = String::from("tac");
*   let s3 = String::from("toe");
*
*   let s = s1 + "-" + &s2 + "-" + &s3;
* At this point, s will be tic-tac-toe. With all of the + and " characters, it's difficult to see
* what's going on. For combining strings in more complicated ways, we can instead use the format! macro:
*
*   let s1 = String::from("tic");
*   let s2 = String::from("tac");
*   let s3 = String::from("toe");
*
*   let s = format!("{s1}-{s2}-{s3}");
* This code also sets s to tic-tac-toe. The format! macro works like println!, but instead of
* printing the output to the screen, it returns a String with the contents. The version of the code
* using format! is much easier to read, and the code generated by the format! macro uses references
* so that this call doesn't take ownership of any of its parameters.
*
* Indexing into Strings
*
* In many other programming languages, accessing individual characters in a string by referencing
* them by index is a valid and common operation. However, if you try to access parts of a String
* using indexing syntax in Rust, you'll get an error. Consider the invalid code below.
*
*   let s1 = String::from("hi");
*   let h = s1[0];
* This code will result in the following error:
*
*   the type `str` cannot be indexed by `{integer}`
*   note: you can use `.chars().nth()` or `.bytes().nth()`
* The error and the note tell the story: Rust strings don't support indexing.
*
* Internal Representation
*
* A String is a wrapper over a Vec<u8>.
*
*   let hello = String::from("Hola");
* In this case, len will be 4, which means the vector storing the string "Hola" is 4 bytes long.
* Each of these letters takes one byte when encoded in UTF-8. The following line, however, may
* surprise you (note that this string begins with the Capital Cyrillic letter Ze, not the number
* 3):
*
*    let hello = String::from("Здравствуйте");
* If you were asked how long the string is, you might say 12. In fact, Rust's answer is 24: that's
* the number of bytes it takes to encode this string in UTF-8, because each Unicode scalar value in
* that string takes 2 bytes of storage. Therefore, an index into the string's bytes will not always
* correlate to a valid Unicode scalar value. To demonstrate this, consider this invalid Rust code:
*
*   let hello = "Здравствуйте";
*   let answer = &hello[0];
* You already know that answer will not be 3, the first letter. When encoded in UTF-8, the first
* byte of 3 is 208 and the second is 151, so it would seem that answer should in fact be 208, but
* 208 is not a valid character on its own. Returning 208 is likely not what a user would want if
* they asked for the first letter of this string; however, that's the only data that Rust has
* at byte index 0. Users generally don't want the byte value returned, even if the string contains
* only Latin letters: if &"hi"[0] were valid code that returned the byte value, it would return
* 104, not h.
*
* The answer, then, is that to avoid returning an unexpected value and causing bugs that might not
* be discovered immediately, Rust doesn't compile this code at all and prevents misunderstandings
* early in the development process.
*
* Bytes and Scalar Values and Grapheme Clusters! Oh My!
*
* Another point about UTF-8 is that there are actually three relevant ways to look at strings from
* Rust's perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we
* would call letters).
*
* If we look at the Hindi word “नमस्ते” written in Devanagari script, it is stored as a vector of u8
* values that looks like this:
*
*   [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
*
* That's 18 bytes and is how computers ultimately store this data. If we look at them as Unicode
* scalar values, which are Rust's char type is, those bytes look like this:
*
*   ['न', 'म', 'स', '्', 'त', 'े']
*
* There are six char values here, but the fourth and sixth are not letters: they're diacritics that
* don't make sense on their own. Finally, if we look at them as grapheme clusters, we'd get what a
* person would call the four letters that make up the Hindi word:
*
*   ["न", "म", "स्", "ते"]
* Rust provides different ways of interpreting the raw string data that computers store so that
* each program can choose the interpretation it needs, no matter what human language the data is in.
*
* A final reason Rust doesn't allow us to index into a String to get a character is that indexing
* operations are expected to always take constant time O(1). But it's possible to guarantee that
* performance with a String, because Rust would have to walk through the contents from the
* beginning to the index to determine how many valid characters there were.
*
* Slicing Strings
*
* Indexing into a string is often a bad idea because it's not clear what the return type of the
* string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string
* slice. If you really need to use indices to create string slices, therefore, Rust asks you to be
* more specific.
*
* Rather than idexing using [] with a single number, you can use [] with a range to create a string
* slice containing particular bytes:
*
*   let hello = "Здравствуйте";
*   let s = &hello[0];
* Here, s will be a &str that contains the first four bytes of the string. Earlier, we mentioned
* that each of these characters was two bytes, which means s will be Зд.
*
* If we were try to slice only part of a character's ytes with something like &hello[0..1], Rust
* would panic at runtime in the same way as if an invalid index were accessed in a vector:
*
*   byte index 1 is not a char boundary; it is inside '3' (bytes 0..2) of `Здравствуйте`
* You should use caution when creating string slices with ranges, because doing so can crash your program.
*
* Methods for Iterating Over Strings
*
* The best way to operate on pieces of strings is to be explicit about whether you want characters
* or bytes. For individual Unicode scalar values, use the chars method. Calling chars on "Зд"
* separates out and returns two values of type char, and you can iterate over the result to access
* each element:
*
*   for c in "Зд".chars() {
*       println!("{c}");
*   }
* This code will print the following:
*
*   3
*   д
* Alternatively, the bytes method returns each raw byte, which might be appropriate for your
* domain:
*
*   for b in "3д".bytes() {
*       println!("{b}");
*   }
* This code will print the four bytes that make up this string:
*
*   208
*   151
*   208
*   180
* But be sure to remember that valud Unicode scalar values may be made up of more than one byte.
*
* Getting grapheme clusters from strings, as with Devanagari script, is complex, so this
* functionality is not provided by the standard library.
*
* Strings Are Not So Simple
*
* To summarize, strings are complicated. Different programming languages make different choices
* about how to present this complexity to the programmer. Rust has chosen to make the correct
* handling of String data the default behaviour for all Rust programs, which means programmers have
* to put more thought into handling UTF-8 data up front. This trade-off exposes more of the
* complexity of strings than is apparent in other programming languages, but it prevents you from
* having to handle errors involving non-ASCII characters later in your development life cycle.
*/
fn greetings(name: &str) {
    println!("Greetings {name}!");
}

fn main() {
    let mut name = String::from("Can");
    name.push_str(" Kocak");
    greetings(&name);

    let first_name = String::from("Iroquois");
    let surname = String::from(" Pliskin");
    let full_name = first_name + &surname;
    // println!("First name of the Solid Snake is {first_name}");   // doesn't work as + takes the
    // ownership of first_name
    println!("Solid Snake is also known as {} in MGS2.", full_name);

    let s1 = String::from("Solid");
    let s2 = String::from("Snake");
    println!(
        "{full_name} is an alias for {} in MGS2.",
        format!("{} {}", s1, s2)
    );

    let s3 = String::from("Грозный Град");
    for c in s3.chars() {
        println!("{c}");
    }

    for b in s3.bytes() {
        println!("{b}");
    }

    println!("{}", &s3[..4])

    /* Rust does not allow string indexing because A UTF-8 string can be interpreted as a sequence
     * of bytes, characters, or grapheme cluster. None of these is necessarily the "default" way of
     * interpreting a string, so a default indexing does not make sense. */

    /* The difference between the types of a string slice &str and a byte slice &[u8] is that &str
    * points to bytes that can always be interpreted as UTF-8, whereas &[u8] can be any byte sequence.
    * &str is a promise that the byte sequence it points to will always be valid UTF-8. Therefore, a
    * programmer who wants to e.g., print out an &str never needs to check if it is valid, or worry
    * about accidentally interpreting an invalid string. */
}
