//! main.rs
//! Documents the module/crate itself
//! Used at the top of files

use std::net::TcpListener;

use sqlx::PgPool;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

// Attribute macro: #[...] applies transformations to the item below (func, etc...)
// tokio::main is a procedural macro that transforms async fn main() into a proper program entry point
// It sets up the async runtime (tokio) that can execute Futures
// Like IORuntime.global in cats-effect - without it, async code can't run
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber(
        // .into() - Type conversion using Into trait.
        // Compiler infers target type from context.
        // Scala equivalent: implicit conversions, but explicit call in Rust
        "zero2prod".into(),
        "info".into(),
        std::io::stdout, // i.e the "sink" to which logs should be written
    );

    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration.");

    let address = config.server.tcp_socket_address();
    let error_msg = format!("Failed to bind to the address {:?}", address);
    let listener = TcpListener::bind(&address).expect(&error_msg);

    let db_conn_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    run(listener, db_conn_pool)? // unwrapp the result of run() , i.e Result<Server, Error>
        .await // Actually executes the Server (Future) (like unsafeRunSync in cats-effect)
}
