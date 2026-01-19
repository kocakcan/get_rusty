const LOWER: u32 = 0;
const UPPER: u32 = 300;
const STEP: usize = 20;

fn main() {
    println!("Fahrenheit\tCelsius");
    println!(
        "{}\t{}",
        "-".repeat("Fahrenheit".len()),
        "-".repeat("Celsius".len())
    );
    for fahr in (LOWER..=UPPER).step_by(STEP) {
        let celsius = 5.0 * (fahr as f64 - 32.0) / 9.0;
        println!("{fahr:>10}\t{celsius:>7.2}");
    }
}
