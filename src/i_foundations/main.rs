use std::time::Instant;

use crate::i_foundations::data_processor::DataProcessor;
use crate::i_foundations::hidden_execeptions::HiddenExeceptions;
use crate::i_foundations::hidden_execeptions::UserError;
use crate::i_foundations::i_iii_type_system_proof_engine::adts::area;
use crate::i_foundations::null_references::Profile;
use crate::i_foundations::null_references::User;
use crate::i_foundations::null_references::UserService;
use crate::i_foundations::safe_operations::SafeOperations;

use crate::i_foundations::i_iii_type_system_proof_engine::adts::Shape;
use crate::i_foundations::i_iii_type_system_proof_engine::immutability_default;

use crate::i_foundations::i_iii_type_system_proof_engine::first_class_vs_afterthought::Order;
use crate::i_foundations::i_iii_type_system_proof_engine::first_class_vs_afterthought::get_high_value_orders;

pub async fn introduction() {
    // Aproach : Data processing with benchmarking
    data_processing();
    safe_operations();
    // i
    null_references();
    // ii
    hidden_exceptions().await;
    // iii
    adt_area();
    // iv
    immutability_default::main();
    // v
    first_class_vs_afterthought();
}

fn data_processing() {
    println!("Running data processing...");
    // Benchmarking the data processing
    let start = Instant::now();

    let mut processor: DataProcessor = DataProcessor { data: Vec::new() };
    processor.process_large_dataset();

    let duration = start.elapsed();
    println!("Processed data length: {}", processor.data.len());
    println!("Time taken to process large dataset: {:?}", duration);
}
fn safe_operations() {
    println!("Running safe operations - process_array... ");
    let array = vec![1111, 2];
    let result = SafeOperations::process_array(&array);
    println!("safe_operations: {}", result.unwrap_or("No data".to_string()));

    println!("Running safe operations - process_concurrently... ");
    SafeOperations::process_concurrently();
}
fn null_references() {
    println!("Running null references... ");
    let profile = Profile {
        age: Some(25),
        display_name: Some(String::from("Kevin")),
    };
    let new_user = User {
        active: false,
        profile: Some(profile),
    };
    let user_display_name = UserService::get_user_display_name(&new_user);
    println!("User UpperCased display name: {}", user_display_name.unwrap());
}
async fn hidden_exceptions() {
    println!("Running hidden exceptions... ");
    // This function is async, so we need to use an async runtime to run it
    // For simplicity, we will just block on it using a simple executor
    let user_data = HiddenExeceptions::get_user_data(2).await;
    match user_data {
        Ok(data) => {
            // Desestructuramos para usar todos los campos
            println!(
                "User data retrieved successfully: id={}, permissions={:?}, preferences={}",
                data.user.user_id, data.permissions, data.preferences
            );
        }
        Err(e) => {
            // Aquí abrimos otro match sobre el enum UserError
            match e {
                UserError::DatabaseError(msg) => {
                    println!("Database error: {}", msg);
                }
                UserError::NetworkError(msg) => {
                    println!("Network error: {}", msg);
                }
                UserError::Timeout => {
                    println!("Operation timed out");
                }
            }
        }
    }
}
fn adt_area() {
    println!("Running ADTs... ");
    let circle = Shape::Circle { radius: (22.0) };
    let rectangle = Shape::Rectangle { w: 10.0, h: 5.0 };
    let triangle = Shape::Triangle {
        a: 3.0,
        b: 4.0,
        c: 5.0,
    };
    println!("Area of circle: {}", area(&circle));
    println!("Area of rectangle: {}", area(&rectangle));
    println!("Area of triangle: {}", area(&triangle));
}
fn first_class_vs_afterthought() {
    println!("Running first class vs afterthought... ");
    let orders = vec![
        Order { id: 1, total: 500 },
        Order { id: 2, total: 1500 },
        Order { id: 3, total: 2500 },
    ];
    let high_value_orders = get_high_value_orders(&orders);
    println!("High value orders: {:?}", high_value_orders);
}
