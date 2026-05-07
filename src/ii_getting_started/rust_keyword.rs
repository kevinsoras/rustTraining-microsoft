#[allow(dead_code)]
mod rust_keywords {
    use std::sync::atomic::AtomicUsize;

    // let - Variable binding (like C# var)
    const NAME: &str = "John"; // Immutable by default

    // const - Compile-time constant (like C# const)
    const MAX_SIZE: usize = 100;

    // static - Global variable (like C# static)
    static INSTANCE_COUNT: AtomicUsize = AtomicUsize::new(0);
}
