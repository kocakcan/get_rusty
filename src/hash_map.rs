/*
 * Storing Keys with Associated Values in Hash Maps
 *
 * The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing
 * function, which determines how it places these keys and values into memory. Many programming
 * languages support this kind of data structure, but they often use a different name, such as
 * hash, map, object, hash table, dictionary, or associative array, just to name a few.
 *
 * Hash maps are useful when you want to look up data by using an index, as you can with vectors,
 * but by using a key that can be of any type. For example, in a game, you could keep track of each
 * team's score in a hash map in which each key is a team's name and the values are each team's
 * score. Given a team name, you can retrieve its score.
 *
 * Creating a New Hash Map
 *
 * One way to create an empty hash map is to use new and to add elements with insert. Below, we're
 * keeping track of the scores of two teams whose names are Blue and Yellow. The Blue team starts
 * with 10 points, and the Yellow team starts with 50.
 *
 *  use std::collections::HashMap;
 *
 *  let mut scores = HashMap::new();
 *
 *  scores.insert(String::from("Blue"), 10);
 *  scores.insert(String::from("Yellow"), 50);
 * Note that we need to first use the HashMap from the collections portion of the standard library.
 * Of our three common collections, this one is the least often used, so it's not included in the
 * features brought into scope automatically in the prelude. Hash maps also have less support form
 * the standard library; there's no built-in macro to construct them, for example.
 *
 * Just like vectors, hash maps store their data on the heap. This HashMap has keys of type String
 * and values of type i32. Like vectors, hash maps are homogenous: all of the keys must have the
 * same type, and all of the values must have the same type.
 *
 * Accessing Values in a Hash Map
 *
 * We can get a value out of the hash map by providing its key to the get method, as shown below.
 *
 *  use std::collections::HashMap;
 *
 *  let mut scores = HashMap::new();
 *
 *  scores.insert(String::from("Blue"), 10);
 *  scores.insert(String::from("Yellow"), 50);
 *
 *  let team_name = String::from("Blue");
 *  let score = scores.get(&team_name).copied().unwrapped_or(0);
 * Here, score will have the value that's associated with the Blue team, and the result will be 10.
 * The get method returns an Option<&T>; if there's no value for that key in the hash map, get will
 * return None. This program handles the Option by calling copied to get an Option<i32> rather than
 * Option<&i32>, then unwrap_or to set score to zero if zero doesn't have an entry for the key.
 *
 * We can iterate over each key-value pair in a hash map in a similar manner as we do with vectors,
 * using a for loop:
 *
 *  use std::collections:HashMap;
 *
 *  let mut scores = HashMap::new();
 *
 *  scores.insert(String::from("Blue"), 10);
 *  scores.insert(String::from("Yellow"), 50);
 *
 *  for (key, value) in &scores {
 *      println!("{key}: {value}");
 *  }
 * This code will print each pair in an arbitrary order:
 *
 *  Yellow: 50
 *  Blue: 10
 *
 * Hash Maps and Ownership
 *
 * For types that implement the Copy trait, like i32, the values are copied into the hash map. For
 * owned values like String, the values will be moved and the hash map will be the owner of those
 * values, as shown below.
 *
 *  use std::collections::HashMap;
 *
 *  let field_name = String::from("Favourite color");
 *  let field_value = String::from("Blue");
 *
 *  let mut map = HashMap::new();
 *  map.insert(field_name, field_value);
 *  -> field_name and field_value are invalid at this point, try using them and
 *  -> see what compiler error you get!
 * We aren't able to use the variable field_name and field_value after they've been moved into the
 * hash map with the call to insert.
 *
 * If we insert references to values into the hash map, the values won't be moved into the hash
 * map. The values that the references point to must be valid for at least as long as the hash map
 * is valid.
 *
 * Updating a Hash Map
 *
 * Although the number of key and value pairs is growable, each unique key can only have one value
 * associated with it at a time (but not vice versa: for example, both the Blue team and the Yellow
 * team could have the value 10 stored in the scores hash map).
 *
 * When you want to change the data in a hash map, you have to decide how to handle the case when a
 * key already has a value assigned. You could replace the old value with the new value, completely
 * disregarding the old value. You could keep the old value and ignore the new value, only adding
 * the new value if the key doesn't already have a value. Or you could combine the old value and
 * the new value.
 *
 * Overwriting a Value
 *
 * If we insert a key and a value into a hash map and then insert that same key with a different
 * value, the value associated with that key will be replaced. Even though the code below calls
 * insert twice, the hash map will only contain one key-value pair because we're inserting the
 * value for the Blue team's key both times.
 *
 *  use std::collections::HashMap;
 *
 *  let mut scores = HashMap::new();
 *
 *  scores.insert(String::from("Blue"), 10);
 *  scores.insert(String::from("Blue"), 25);
 *  println!("{scores:?}");
 * This code will print {"Blue": 25}. The original value of 10 has been overwritten.
 *
 * Adding a Key and Value Only If a Key Isn't Present
 *
 * It's common to check whether a particular key already exists in the hash map with a value and
 * then to take the following actions: if the key does exist in the hash map, the existing value
 * should remain the way it is; if the key doesn't exist, insert it and a value for it.
 *
 * Hash maps have a special API for this called entry that takes the key you want to check as a
 * parameter. The return value of the entry method is an enum called Entry that represents a value
 * that might or might not exist. Let's say we want to check whether the key for the Yellow team
 * has a value associated with it. If it doesn't we want to insert the value 50, and the same for
 * the Blue team. Using the entry API, the code looks like below.
 *
 *  use std::collections::HashMap;
 *
 *  let mut scores = HashMap::new();
 *  scores.insert(String::from("Blue"), 10);
 *
 *  scores.entry(String::from("Yellow")).or_insert(50);
 *  scores.entry(String::from("Blue")).or_insert(50);
 *  println!("{scores:?}");
 * The or_insert method on Entry is defined to return a mutable reference to the value for the
 * corresponding Entry key if that exists, and if not, it inserts the parameter as the new value
 * for this key and returns a mutable reference to the new value. This technique is much cleaner
 * than writing the logic ourselves, in addition, plays more nicely with the borrow checker.
 *
 * Running the code above will print {"Yellow": 50, "Blue": 10}. The first call to entry will
 * insert the key for the Yellow team with the value 50 because the Yellow team doesn't have a
 * value already. The second call to entry will not change the hash map because the Blue team
 * already has the value 10.
 *
 * Updating a Value Based on the Old Value
 *
 * Another common use case for hash maps is to look up a key's value and then update it based on
 * the old value. For instance, below shows code that counts how many times each word appears in
 * some text. We use a hash map with the words as keys and increment the value to keep track of how
 * many times we've seen that word. If it's the first time we've seen a word, we'll first insert
 * the value 0.
 *
 *  use std::collections::HashMap;
 *
 *  let text = "hello world wonderful world";
 *  let mut map = HashMap::new();
 *
 *  for word in text.split_whitespace() {
 *      let count = map.entry(word).or_insert(0);
 *      *count += 1;
 *  }
 *
 *  println!("{map:?}");
 * This code will print {"world": 2, "hello": 1, "wonderful": 1}. You might see the same key=value
 * pairs printed in different order as iterating over a hash map happens in an arbitrary order.
 *
 * The split_whitespace method returns an iterator over subslices, separated by whitespace, of the
 * value in text. The or_insert method returns a mutable reference (&mut V) to the value for the
 * specified key. Here, we store that mutable reference in the count variable, so in order to
 * assign to that value, we must first dereference count using the asterisk (*). The mutable
 * reference goes out of scope at the end of the for loop, so all of these changes are safe and
 * allowed by the borrowing rules.
 *
 * Hashing Functions
 *
 * By default, HashMap uses a hashing function called SipHash that can provide resistance to
 * denial-of-service (DoS) attacks involving hash tables. This is not the fastest hashing algorithm
 * available, but the trade-off for better security that comes with the drop in performance is
 * worth it. If you profile your code and find that the default hash function is too slow for your
 * purposes, you can switch to another function by specifying a different hasher. A hasher is a
 * type that implements the BuildHasher trait.
 *
 * Summary
 *
 * Vectors, strings, and hash maps will provide a large amount of functionality necessary in
 * programs when you need to store, access, and modify data.
 */
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("score: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let new_team = String::from("Red");
    let new_score = 90;

    scores.insert(new_team, new_score);

    // new_score can be used after being inserted to the hash map as
    // it implements the Copy trait
    println!("New score: {new_score}");

    // but new_team is of type String which doesn't implement Copy trait
    // so it will end up being moved to the hash map and can't be used after that.
    // println!("New team: {new_team}");

    scores.entry(String::from("Red")).or_insert(100);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
