// fn greet(g1: String, g2: String) -> (String, String) {
//     println!("{} {}!", g1, g2);
//     (g1, g2)
// }
//
// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     let (m1_again, m2_again) = greet(m1, m2);
//     let _s = format!("{} {}!", m1_again, m2_again);
// }

// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     greet(&m1, &m2);
//     let _s = format!("{} {}!", m1, m2);
// }
//
// fn greet(g1: &String, g2: &String) {
//     println!("{} {}!", *g1, *g2);
// }

fn main() {
    let name = String::from("Seyfi");
    // This is read-only
    let prefix = " Jr.";
    let name = concat(name, prefix);

    println!("My name is {}", name);
}

fn concat(mut text: String, prefix: &str) -> String {
    text.push_str(prefix);
    text
}
