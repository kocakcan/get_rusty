/* Given a list of integers, use a vector and return the median (when sorted, the value in the
* middle position) and mode (the value that occurs most often; a hash map will be helpful here) of
* the list.
*/

/* Convert strings to pig latin. The first consonant of each word is moved to the end of the word
* and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the
* end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
*/

/* Using a hash map and vectors, create a text interface to allow a user to add employee names to a
* department in a company, for example, "Add Sally to Engineering" or "Add Amir to Sales." Then let
* the user retrieve a list of all people in a department or all people in the company by
* department, sorted alphabetically.
*/
use std::collections::HashMap;

/// Return the median of the slice.
/// NOTE: Assumes that the slice is sorted.
fn median(v: &[i32]) -> i32 {
    let len = v.len();
    match len {
        0 => 0,
        1 => v[0],
        _ if len % 2 == 0 => (v[len / 2] + v[(len / 2) - 1]) / 2,
        _ => v[len / 2],
    }
}

fn mode(v: &[i32]) -> i32 {
    let mut map = HashMap::new();

    for num in v {
        map.entry(*num).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut mode = 0;
    let mut max_count = 0;

    for (num, count) in &map {
        if *count > max_count {
            max_count = *count;
            mode = *num;
        }
    }
    mode
}

fn piglatin(text: &str) -> String {
    match text.chars().next().unwrap() {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
            format!("{text}-hay")
        }
        c => {
            let rest = &text[c.len_utf8()..];
            format!("{rest}-{c}ay")
        }
    }
}

/// Print the employees in a specific department
/// Nothing is printed if the department does not exist.
fn printd(company: &HashMap<String, Vec<String>>, dept: &str) {
    match company.get(dept) {
        Some(e) => println!("{e:?} works in {}", dept),
        _ => (),
    }
}

fn hire(company: &mut HashMap<String, Vec<String>>, employee: &str, dept: &str) {
    company
        .entry(dept.to_string())
        .or_insert(Vec::new())
        .push(employee.to_string());
}

fn main() {
    let mut v: Vec<i32> = vec![
        55, 34, 57, 40, 5, 29, 10, 92, 17, 76, 10, 43, 61, 97, 33, 54, 73, 81, 17, 57, 27, 72, 10,
        96, 36, 38, 85, 95, 99, 99, 37, 50, 68, 9, 56, 10, 43, 95, 77, 66, 73, 57, 48, 2, 30, 54,
        84, 30, 84, 88, 70, 78, 99, 12, 56, 14, 8, 81, 3, 18, 5, 39, 54, 42, 45, 51, 37, 49, 83,
        13, 58, 88, 11, 64, 60, 66, 0, 17, 81, 82, 1, 9, 68, 94, 6, 67, 58, 42, 89, 35, 60, 13, 0,
        65, 55, 75, 41, 59, 10, 14,
    ];
    v.sort();

    println!("Mode: {:?}", mode(&v));
    println!("Median: {:?}", median(&v));

    println!("Result: {}", piglatin(&String::from("Ciaran")));
    println!("Result: {}", piglatin(&String::from("Artorias")));

    let mut from_software: HashMap<String, Vec<String>> = HashMap::new();

    hire(&mut from_software, "Yuno Ito", "Development");
    hire(&mut from_software, "Masanori Takeuchi", "Development");
    hire(&mut from_software, "Yui Tanimura", "Development");
    hire(&mut from_software, "Kazuhiro Hamatani", "Development");
    hire(&mut from_software, "Yazuhiro Kitao", "Marketing");
    printd(&from_software, "Development");
}
