//! src/lib.rs
//! Documents the module/crate itself
//! Used at the top of files

// Imports
// :: is the path/namespace separator (for modules, types, static functions)
// . is for method calls on instances
// Example: String::from("text") vs my_string.len()
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, web};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    // impl Responder = "returns some concrete type that implements the Responder trait"
    // The caller doesn't know the exact type, just that it satisfies the Responder contract
    // Similar to Scala's abstract type members or existential types
    // Traits ≈ typeclasses (behavior contracts), but impl Trait is more like bounded existentials
    HttpResponse::Ok()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req
        .match_info() // Extracts ROUTE PARAMETERS from URL path (e.g., "/greet/{name}")
        .get("name")
        .unwrap_or("World");
    format!("Hello {}", name) // Expression returning String (not unit like println!)
}

/*
* actix-web provides several extractors out of the box to cater for the most common usecases:
• Path, Query, Json, ... FormData
Extractors can be accessed as an argument to a handler function.
* NOTE: An extractor is a type that implements the FromRequest trait.
*
* All arguments in the signature of a route handler must implement the FromRequest trait:
* actix-web will invoke from_request for each argument and, if the extraction succeeds for all of them,
* it will then run the actual handler function.
* If one of the extractions fails, the corresponding error is returned to the caller and the handler is never invoked (actix_web::Error can be converted to a HttpResponse).
* This is extremely convenient: your handler does not have to deal with the raw incoming request and can instead work directly with strongly-typed information, significantly simplifying the code that you need to write to handle a request.
* */

#[derive(serde::Deserialize)]
#[allow(dead_code)] // to prevent clippy from blocking our commit (not using the fields... yet)
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(
    // "slap" it there as an argument to the handler
    // and when a request comes in,
    // actix-web will somehow do the heavy-lifting for you
    _form: web::Form<FormData>,
) -> HttpResponse {
    // NOTE: We only return 200 OK here, but the endpoint automatically returns
    // 400 Bad Request when form data is invalid/missing.
    // This happens because web::Form<FormData> extraction fails before this handler runs,
    // and actix-web converts the extraction error into a 400 response automatically.
    HttpResponse::Ok().finish()
}

// NOTE: pub fn: public since it is not a binary entrypoint
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // Result is left-biased vs. Scala Either 'conventionally' right-biased

    // HttpServer handles all transport level concerns
    let server = HttpServer::new(|| {
        // Closure syntax: || { ... } for zero args, |a, b| { ... } for args
        // Can add types: |a: i32, b: String| { ... }

        // App is where all your application logic lives: routing, middlewares, request handlers, etc.
        // App is the component whose job is to take an incoming request as input and spit out a response.
        App::new()
            // web::get() creates a route guard that only matches HTTP GET requests
            // .to(greet) binds the greet handler function to this route
            // So: "on GET request to this path, call greet()"
            .route("/health_check", web::get().to(health_check))
            .route(
                "/greet",             // PATH: &str
                web::get().to(greet), // ROUTE: Route (an instance of the Route struct)
            )
            .route("/greet/{name}", web::get().to(greet))
            .route("/subscription", web::post().to(subscribe))
    })
    .listen(listener)? // ? operator: if bind() fails, return the error immediately
    // if success, unwrap the Ok value and continue
    // Requires function to return Result<T, E>
    // Like early exit in Scala for-comprehension, but for errors
    .run(); // Returns a Future (NOTA: lazy in rust - pure description of work - doesn't execute yet!)

    // We return the server without awaiting it,
    // i.e, it can run in the background, concurrently with downstream futures and tasks
    Ok(server) // NOTE: Server IS A FUTURE WRAPPED IN A RESULT !!!
}
