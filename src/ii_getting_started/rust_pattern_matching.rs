#[allow(dead_code)]
mod rust_pattern_matching {

    fn main() {
        let number = 3;
        // match - Pattern matching (like C# switch but much more powerful)

        let result = match number {
            1 => "one",
            2 => "two",
            3..=5 => "three to five", // Range patterns
            _ => "other",
        };
        println!("The number is: {}", result);
        // if let - Conditional pattern matching

        // This is the way if you want to use the value of the pattern matching
        // Different from match, if let only allows you to match one pattern and ignore the rest
        let optional_number = Some(number);
        if let Some(name_value) = optional_number {
            println!("The number is: {}", name_value);
        }

        /* match optional_number {
            Some(value) => println!("The number is: {}", value),
            None => println!("No number provided"),
        } */
        let optional_numbers = [1; 10];
        while let Some(number) = optional_numbers.iter().next() {
            println!("Item: {}", number);
        }

        let Some(value) = optional_number else {
            println!("No number provided");
            return; // Early return if pattern doesn't match
        };
        println!("The number is: {}", value);
    }
}
