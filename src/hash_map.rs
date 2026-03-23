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
}
