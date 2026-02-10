struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point { x: 5, y: 6 };
    let x = &mut p.x;
    *x += 1;
    let y = &mut p.y;
    *y += 1;
    println!("({}, {})", p.x, p.y);
}
