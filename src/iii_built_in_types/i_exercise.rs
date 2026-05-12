#[allow(dead_code)]
pub mod type_safe_temperature {
    use std::{
        io::{self, Write},
        sync::atomic::{AtomicU32, Ordering},
    };

    const ABSOLUTE_ZERO: f64 = -273.15;
    static COUNT_CONVERSIONS: AtomicU32 = AtomicU32::new(0);
    fn celsius_to_fahrenheit(c: f64) -> f64 {
        if c < ABSOLUTE_ZERO {
            return f64::NAN;
        }
        let f = (c * 1.8) + 32.0;

        COUNT_CONVERSIONS.fetch_add(1, Ordering::SeqCst);

        f
    }
    pub fn input_celsius_to_fahrenheit() {
        io::stdout().flush().unwrap();

        print!("Enter temperature in Celsius: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().parse::<f64>().expect("Failed to parse input");
        let result = celsius_to_fahrenheit(input);
        println!("Fahrenheit: {}", result);
        println!("Total conversions: {}", COUNT_CONVERSIONS.load(Ordering::SeqCst));
    }
}
