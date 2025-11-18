//! main.rs
//! Documents the module/crate itself
//! Used at the top of files

use std::net::TcpListener;

use sqlx::PgPool;

use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

// Attribute macro: #[...] applies transformations to the item below (func, etc...)
// tokio::main is a procedural macro that transforms async fn main() into a proper program entry point
// It sets up the async runtime (tokio) that can execute Futures
// Like IORuntime.global in cats-effect - without it, async code can't run
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // 12-factor: RUST_LOG is required config, fail fast if missing
    std::env::var("RUST_LOG")
        .expect("RUST_LOG environment variable must be set (e.g., RUST_LOG=info)");

    // Redirects all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("infos"));

    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        // output the formatted spans to stdout
        std::io::stdout,
    );

    let subscriber = Registry::default()
        // `.with` is provided by `SubscriberExt`
        // an extension trait for `Subscriber` exposed by `tracing_subscriber`
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    // specify which subscriber should process the span
    set_global_default(subscriber).expect("Failed to set subscriber");

    let config = get_configuration().expect("Failed to read configuration.");

    let listener = TcpListener::bind(config.server.tcp_socket_address())
        .expect("Failed to bind to the address");

    let db_conn_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    run(listener, db_conn_pool)? // unwrapp the result of run() , i.e Result<Server, Error>
        .await // Actually executes the Server (Future) (like unsafeRunSync in cats-effect)
}
