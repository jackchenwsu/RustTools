// C = (F - 32) × 5 / 9
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

// F = C × 9 / 5 + 32
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    // Example usage
    let celsius = 25.0;
    println!("{}°C is {}°F", celsius, celsius_to_fahrenheit(celsius));
    let fahrenheit = 77.0;
    println!(
        "{}°F is {}°C",
        fahrenheit,
        fahrenheit_to_celsius(fahrenheit)
    );
}
