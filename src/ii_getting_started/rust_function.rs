#[allow(dead_code)]

// fn - Function definition (like C# method but standalone)
fn regular_function() {
    println!("Hello");
}
#[allow(dead_code)]

// const fn - Compile-time function (like C# const but for functions)
const fn compile_time_function() -> i32 {
    42 // Can be evaluated at compile time
}
#[allow(dead_code)]

// async fn - Asynchronous function (like C# async)
async fn async_function() -> i32 {
    1
    /* some_async_operation().await */
}
#[allow(dead_code)]

// unsafe fn - Function that may violate memory safety
unsafe fn unsafe_function() {
    // Can perform unsafe operations
}
#[allow(dead_code)]

// extern fn - Foreign function interface
extern "C" fn c_compatible_function() {
    // Can be called from C
}
