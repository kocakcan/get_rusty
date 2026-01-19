const LOWER: u32 = 0;
const UPPER: u32 = 300;
const STEP: usize = 20;

fn main() {
    println!("┌────────────┬─────────┐");
    println!("│ Fahrenheit │ Celsius │");
    println!("├────────────┼─────────┤");
    for fahr in (LOWER..=UPPER).step_by(STEP) {
        let celsius = 5.0 * (fahr as f64 - 32.0) / 9.0;
        println!("│ {:>10} │ {:>7.2} │", fahr, celsius);
    }
    println!("└────────────┴─────────┘");
}
