use std::io::{self, Write};

pub fn console_input() {
    // println!("Please enter your name: ");
    // If you use println!, it will add a newline after the prompt, which can be confusing for the user. println! dont need flush, but print! does, because it doesn't end with a newline. So we need to flush stdout to ensure the prompt is displayed before reading input.
    print!("Please enter your name: ");

    // Flush stdout to ensure the prompt is displayed before reading input
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Hello, {}!", name.trim());

    // Parsing input
    print!("Please enter your age: ");
    io::stdout().flush().unwrap();
    let mut age_input = String::new();
    io::stdin()
        .read_line(&mut age_input)
        .expect("Falied to read line");
    // We use trim to remove any whitespace characters,and parse i32 to convert the string input into a positive integer.
    match age_input.trim().parse::<i32>() {
        Ok(age) => println!("You are {} years old.", age),
        Err(_) => println!("Invalid age input."),
    }
}
