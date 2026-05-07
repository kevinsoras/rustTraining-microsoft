/* // unsafe - Disable safety checks
unsafe
{
    int* ptr = &variable;
    *ptr = 42;
}

// fixed - Pin managed memory
unsafe
{
    fixed (byte* ptr = array)
    {
        // Use ptr
    }
} */

// unsafe - Disable borrow checker (use sparingly!)

pub mod unsafe_functions {
    pub fn main() {
        let variable = 5;

        // Unsafe block - Required to call unsafe functions or access unsafe features
        unsafe {
            let ptr = &variable as *const i32;
            let copy_value = *ptr; // Dereferencing a raw pointer is unsafe
            println!("Value from raw pointer: {}", copy_value);
        }
        // Raw pointer types (no C# equivalent - usually not needed)
        // You can create raw pointers, but you must use unsafe blocks to dereference them
        let ptr: *const i32 = &42; // Immutable raw pointer
        let ptr_mut: *mut i32 = &mut 42; // Mutable raw pointer
        unsafe {
            println!("Raw pointer value: {}", *ptr); // Dereferencing raw pointer
            *ptr_mut = 100; // Modifying value through mutable raw pointer
            println!("Modified value through mutable raw pointer: {}", *ptr_mut);
        }
    }
}
