//! main.rs
//! Documents the module/crate itself
//! Used at the top of files

use std::net::TcpListener;

use sqlx::PgPool;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

// Attribute macro: #[...] applies transformations to the item below (func, etc...)
// tokio::main is a procedural macro that transforms async fn main() into a proper program entry point
// It sets up the async runtime (tokio) that can execute Futures
// Like IORuntime.global in cats-effect - without it, async code can't run
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration.");

    let listener = TcpListener::bind(config.server.tcp_socket_address())
        .expect("Failed to bind to the address");

    let db_conn_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    run(listener, db_conn_pool)? // unwrapp the result of run() , i.e Result<Server, Error>
        .await // Actually executes the Server (Future) (like unsafeRunSync in cats-effect)
}
