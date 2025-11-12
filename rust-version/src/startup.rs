use actix_web::{App, HttpServer, dev::Server, web};
use std::net::TcpListener;

use crate::routes::greet;
use crate::routes::health_check;
use crate::routes::subscribe;

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
