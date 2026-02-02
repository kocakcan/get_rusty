/*
* The Slice Type
*
* Slices let you reference a contiguous sequence of elements in a collection rather than the whole
* collection. A slice is a kind of reference, so it is a non-owning pointer.
*
* Write a function that takes a string of words separaterd by spaces and returns the first word it
* finds in that string. If the function doesn't find a space in the string, the whole string must
* be one word, so the entire string should be returned. Without slices, we might write the
* signature of the function like this:
*
*   fn first_word(s: &String) -> ?
* The first_word function has a &String as a parameter. We don't want ownership of the string, so
* this is fine. But what should we return? We don't really have a way to talk about part of a
* string. However, we could return the index of the end of the word, indicated by a space.
*
*   fn first_word(s: &String) -> usize {
*       let bytes = s.as_bytes();
*
*       for (i, &item) in bytes.iter().enumerate() {
*           if item == b' ' {
*               return i;
*           }
*       }
*       s.len()
*   }
* Because we need to go through the String element by element and check whether a value is a space,
* we'll convert our String to an array of bytes using the as_bytes method.
*
*   let bytes = s.as_bytes();
* Next, we create an iterator over the array of bytes using the iter method:
*
*   for (i, &item) in bytes.iter().enumerate() {
* iter is a mmethod that returns each element in a collection and that enumerate wraps the result
* of iter and returns each element as part of a tuple instead. The first element of the tuple
* returned from enumerate is the index, and the second element is a reference to the element. This
* is a bit more convenient than calculating the index ourselves.
*
* Because the enumerate method returns a tuple, we can use patterns to destructure that tuple. In
* the for loop, we specify a pattern that has i for index in the tuple and &item for the single
* byte in the tuple. Because we get a reference to the element from .iter().enumerate(), we use &
* in the pattern.
*
* Inside the for loop, we search for the byte that represents the space by using the byte literal
* syntax. If we find a space, we return the position. Otherwise, we return the length of the string
* by using s.len():
*
*       if item == b' ' {
*           return i;
*       }
*   }
*   s.len()
* We now have a way to find out the index of the end of the first word in the string, but there's a
* problem. We're returning a usize on its own, but it's only a meaningful number in the context of
* the &String. In other words, because it's a separate value from the String, there's no guarantee
* that it will still be valid in the future. Consider the following program:
*
*   fn first_word(s: &String) -> usize {
*       -> s            | RO
*       -> *s           | R
*
*       let bytes = s.as_bytes();
*
*       -> bytes        | RO
*       -> *bytes       | R
*       -> (*bytes)[_]  | R
*
*       for (i, &item) in bytes.iter().enumerate() {
*
*       -> bytes        | No permissions
*       -> *bytes       | No permissions
*       -> (*bytes)[_]  | No permissions
*       -> i            | RO
*       -> item         | RO
*
*           if item == b' ' {
*
*       -> s            | No permissions
*       -> *s           | No permissions
*       -> item         | No permissions
*               return i;
*
*       -> i            | No permissions (goes out of scope)
*           }
*
*       -> i            | No permissions
*       -> item         | No permissions
*       }
*       s.len()
*   }
*
*   fn main() {
*       let mut s = String::from("hello world");
*
*       -> s               | RWO
*
*       let word = first_word(&s);  -> referencing requires R
*                                   -> would need W as well if it was &mut reference
*       s.clear();                  -> clear() requires both R and W
*   }
* This program compiles without any errors, as s retains write permissions after calling
* first_word. Because word isn't connected to the state of s at all, word still contains the value
* 5. We could use that value 5 with the variable s to try to extract the first word out, but this
* would be a bug because the contents of s have changed since we saved 5 in word.
*
* Having to worry about the index in word getting out of sync with the data in s is tedious and
* error prone! Managing these indices is even more brittle if we write a second_word function. Its
* signature would have to look like this:
*
*   fn second_word(s: &String) -> (usize, usize) {
* Now we're tracking a starting and an ending index, and we have even more values that were
* calculate from data in a particular state but aren't tied to that state at all. We have three
* unrelated variables floating around that need to be kept in sync.
*
* Luckily, Rust has a solution to this problem: string slices.
*
* String Slices
*
* A string slice is a reference to part of a String, and it looks like this:
*
*   let s = String::from("hello world");
*
*   let hello: &str = &s[0..5];
*   let world: &str = &s[6..11];
*   let s2: &String = &s;
* Rather than a reference to the entire String (like s2), hello is a reference to a portion of the
* String, specified in the extra [0..5] bit. We create slices using a range within brackets by
* specifying [starting_index..ending_index], where starting_index is the first position in the
* slice and ending_index is one more than the last position in the slice.
*
* Slices are special kinds of references because they are "fat" pointers, or pointers with
* metadata. Here, the metadata is the length of the slice. The variables hello and world have both
* a ptr and a len field, which together define the underlined regions of the string on the heap.
*
* Because slices are references, they also change the permissions on referenced data. For example,
* observe below that when hello is created as a slice of s, then s loses write and own permissions:
*
*   fn main() {
*       let mut s = String::from("hello");
*
*   -> s        | RWO
*
*       let hello: &str = &s[0..5];
*
*   -> s        | R
*   -> hello    | RO
*   -> *hello   | R
*
*       println!("{hello}");    -> requires R
*
*   -> s        | RWO
*   -> hello    | No permissions (goes out of scope)
*   -> *hello   | No permissions (goes out of scope)
*
*   s.push_str(" world");       -> requires RW
*
*   -> s        | No permissions (goes out of scope)
*   }
*/

// fn first_word(s: &String) -> usize {
//     let s = s.as_bytes();
//
//     for (i, &item) in s.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }
//
// fn main() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s);
//     s.clear();
// }

// fn main() {
//     let mut s = String::from("hello");
//     let hello: &str = &s[0..5];
//     println!("{hello}");
//     s.push_str(" world");
// }
