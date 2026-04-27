mod i_foundations;
use i_foundations::main;
#[tokio::main]
async fn main() {
    // Run the introduction function that demonstrates various concepts
    // Setup, types, control flow
    main::introduction().await;
}
