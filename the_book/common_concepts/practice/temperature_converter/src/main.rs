fn main() {
    let temp = 100.0;
    println!("{}", fahrenheit_to_celsius(temp));
    println!("{}", celsius_to_fahrenheit(temp));
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}
