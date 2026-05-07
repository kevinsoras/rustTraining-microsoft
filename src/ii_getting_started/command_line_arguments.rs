use std::env;

use dotenv::dotenv;


pub fn get_arguments() {
    let mut args: Vec<String> = env::args().collect();
    // args[0] = name of the program
    // args[1..] = arguments
    // Set default value for filename if not provided, and exit with error message
    args.push("default.txt".into());
    args.push("another_arg".into());
    if args.len() < 2 {
        eprintln!("Usage : {} <filename> ,and exit program", args[0]);
        //  Exit with a non-zero code to indicate an error
        std::process::exit(1)
    }
    let filename = &args[1];
    println!("You provided the filename: {}", filename);
    // Get .env variable ,you need dotenv crate to load .env file
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "localhost".to_string());
    println!("Database URL: {}", database_url);
}
