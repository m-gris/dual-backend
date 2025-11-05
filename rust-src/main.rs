// Imports
// :: is the path/namespace separator (for modules, types, static functions)
// . is for method calls on instances
// Example: String::from("text") vs my_string.len()
use actix_web::{App, HttpRequest, HttpServer, Responder, web};
use const_format::formatcp; // For compile-time string formatting

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

async fn greet(req: HttpRequest) -> impl Responder {
    // impl Responder = "returns some concrete type that implements the Responder trait"
    // The caller doesn't know the exact type, just that it satisfies the Responder contract
    // Similar to Scala's abstract type members or existential types
    // Traits ≈ typeclasses (behavior contracts), but impl Trait is more like bounded existentials
    let name = req
        .match_info() // Extracts ROUTE PARAMETERS from URL path (e.g., "/greet/{name}")
        .get("name")
        .unwrap_or("World");
    format!("Hello {}", name) // Expression returning String (not unit like println!)
}

// Attribute macro: #[...] applies transformations to the item below <- CLAUDE ... what 'item below' ???
// tokio::main transforms async fn main() into a proper program entry point
// It sets up the async runtime (tokio) that can execute Futures
// Like IORuntime.global in cats-effect - without it, async code can't run
// CLAUDE: so... are you comparing tokio & cats-effect / zio ?????
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Result is left-biased vs. Scala Either 'conventionally' right-biased

    // HttpServer handles all transport level concerns
    HttpServer::new(|| {
        // Closure syntax: || { ... } for zero args, |a, b| { ... } for args
        // Can add types: |a: i32, b: String| { ... }

        // App is where all your application logic lives: routing, middlewares, request handlers, etc.
        // App is the component whose job is to take an incoming request as input and spit out a response.
        App::new()
            // web::get() creates a route guard that only matches HTTP GET requests
            // .to(greet) binds the greet handler function to this route
            // So: "on GET request to this path, call greet()"
            .route(
                "/",                  // PATH: &str
                web::get().to(greet), // ROUTE: Route (an instance of the Route struct)
            )
            .route("/name", web::get().to(greet))
    })
    .bind(TCP_SOCKET_ADDRESS)? // ? operator: if bind() fails, return the error immediately
    // if success, unwrap the Ok value and continue
    // Requires function to return Result<T, E>
    // Like early exit in Scala for-comprehension, but for errors
    .run() // Returns a Future (NOTA: lazy in rust - pure description of work - doesn't execute yet!)
    .await // Actually executes the Future (like unsafeRunSync in cats-effect)
}
