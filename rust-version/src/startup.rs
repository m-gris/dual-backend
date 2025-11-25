use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes::health_check;
use crate::routes::subscribe;

// NOTE: pub fn: public since it is not a binary entrypoint
pub fn run(listener: TcpListener, db_conn_pool: PgPool) -> Result<Server, std::io::Error> {
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
            // Closure syntax: || { ... } for a zero lambda,
            // |x| { ... } for a unary-lambda
            // Can add types: |a: i32, b: String| { ... }

            // App is where all your application logic lives: routing, middlewares, request handlers, etc.
            // App is the component whose job is to take an incoming request as input and spit out a response.
            App::new()
                // Adding Middlewares with the `wrap` method on `App`
                .wrap(TracingLogger::default()) // emits a log record for every incoming request.
                .route(
                    "/health_check",
                    // web::get() creates a route guard that only matches HTTP GET requests
                    // .to(health_check) binds the greet handler function to this route
                    web::get().to(health_check),
                )
                .route(
                    "/subscription",           // PATH: &str
                    web::post().to(subscribe), // ROUTE: Route (an instance of the Route struct)
                )
                // Register a PgPool as part of our application state
                // byt getting a pointer copy and attach it to the application state
                .app_data(wrapped_clonable_db_conn.clone())
        },
    )
    .listen(listener)?
    .run(); // Returns a Future (NOTA: lazy in rust - pure description of work - doesn't execute yet!)

    // We return the server without awaiting it,
    // i.e, it can run in the background, concurrently with downstream futures and tasks
    Ok(server) // NOTE: Server IS A FUTURE WRAPPED IN A RESULT !!!
}
