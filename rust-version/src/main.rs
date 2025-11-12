//! main.rs
//! Documents the module/crate itself
//! Used at the top of files

use std::net::TcpListener;

use const_format::formatcp; // For compile-time string formatting

use zero2prod::startup::run;

const HOST: &str = "127.0.0.1";
const PORT: &str = "8000";

// LESSON: Compile-time vs Runtime String Composition
// ====================================================
// formatcp! = "format compile" - compile-time string concatenation
// Works exactly like format! but evaluates at compile time
// The result is baked into the binary as a string literal
// NOTE: const VALUES MUST BE COMPUTED AT COMPILE TIME, hence we must use formatcp!
const TCP_SOCKET_ADDRESS: &str = formatcp!("{}:{}", HOST, PORT);

// TERMINOLOGY CLARIFICATION:
// - TCP_SOCKET_ADDRESS: A string representing where to bind ("127.0.0.1:8000")
// - TCP Socket: The actual OS resource created when .bind() is called
// - TCP Connection: An accepted connection on that socket

// Attribute macro: #[...] applies transformations to the item below (func, etc...)
// tokio::main is a procedural macro that transforms async fn main() into a proper program entry point
// It sets up the async runtime (tokio) that can execute Futures
// Like IORuntime.global in cats-effect - without it, async code can't run
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(TCP_SOCKET_ADDRESS).expect("Failed to bind to the address");

    run(listener)? // unwrapp the result of run() , i.e Result<Server, Error>
        .await // Actually executes the Server (Future) (like unsafeRunSync in cats-effect)
}
