use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::greet;
use crate::routes::health_check;
use crate::routes::subscribe;

// NOTE: pub fn: public since it is not a binary entrypoint
pub fn run(listener: TcpListener, db_conn_pool: PgPool) -> Result<Server, std::io::Error> {
    // Result is left-biased vs. Scala Either 'conventionally' right-biased

    /*
     * web::Data wraps our connection in an Atomic Reference Counted pointer, an Arc:
     * each instance of the application, instead of getting a raw copy of a PgPool,
     * will get a pointer to one Arc<T> is always cloneable, no matter who T is:
     * cloning an Arc increments the number of active references
     * and hands over a new copy of the memory address of the wrapped value.
     */
    let wrapped_clonable_db_conn = web::Data::new(db_conn_pool);

    // HttpServer handles all transport level concerns
    let server = HttpServer::new(
        // `move` transfers the ownership of `wrapped_clonable_db_conn`
        // from`server` to this zero-lambda closure
        move || {
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
                // Register a PgPool as part of our application state
                // byt getting a pointer copy and attach it to the application state
                .app_data(wrapped_clonable_db_conn.clone())
        },
    )
    .listen(listener)? // ? operator: if bind() fails, return the error immediately
    // if success, unwrap the Ok value and continue
    // Requires function to return Result<T, E>
    // Like early exit in Scala for-comprehension, but for errors
    .run(); // Returns a Future (NOTA: lazy in rust - pure description of work - doesn't execute yet!)

    // We return the server without awaiting it,
    // i.e, it can run in the background, concurrently with downstream futures and tasks
    Ok(server) // NOTE: Server IS A FUTURE WRAPPED IN A RESULT !!!
}
