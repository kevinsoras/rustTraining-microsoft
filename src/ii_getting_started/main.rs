use crate::ii_getting_started::rust_unsafe;
use crate::ii_getting_started::{command_line_arguments, console_input};
#[allow(dead_code)]
pub fn getting_started() {
    // Run the introduction function that demonstrates various concepts
    // Setup, types, control flow
    console_input::console_input();
    command_line_arguments::get_arguments();
    rust_unsafe::unsafe_functions::main();
}
