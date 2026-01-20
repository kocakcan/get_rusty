const NUMBER: u32 = 50;

fn fib(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;

            for _ in 2..=n {
                (a, b) = (b, a + b);
            }
            b
        }
    }
}

/* small program that returns the nth Fibonacci number without taking integer overflow into account
 * */
fn main() {
    println!("The {}th Fibonacci number is: {}", NUMBER, fib(NUMBER));
}
