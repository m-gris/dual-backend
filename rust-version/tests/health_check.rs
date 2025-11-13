//! tests/health_check.rs

use std::net::TcpListener; // For compile-time string formatting

use sqlx::{Connection, PgConnection};

use zero2prod::configuration::{Settings, get_configuration};

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute. //
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // ARRANGE
    let root_address = spawn_app();
    // nota: no http:// in the string... since it already is baked in root_address
    let health_address = &format!("{}/health_check", &root_address);
    // use REQWEST to perform HTTP requests against our app
    let client = reqwest::Client::new();

    // ACT
    let response = client
        .get(health_address)
        .send()
        .await
        .expect("Failed to execute request");

    // ASSERT
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    // A NOTE ON CLEAN-UP / TEARDOWN
    // when a tokio runtime is shut down all tasks spawned on it are dropped.
    // tokio::test spins up a new runtime at the beginning of each test case and they shut down at the end of each test case.
}

#[tokio::test]
async fn can_connect_to_db() {
    let config: Settings = get_configuration().expect("Failed to read config");
    let db_conn = PgConnection::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
}

#[tokio::test]
async fn subscribe_returns_200_ok_for_valid_form_data() {
    // ARRANGE
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let config: Settings = get_configuration().expect("Failed to read config");
    let db_conn = PgConnection::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    // ACT
    let response = client
        .post(&format!("{}/subscription", app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("name=le%20guin&email=ursula_le_guin%40gmail.com")
        .send()
        .await
        .expect("Failed to execute request.");

    // ASSERT
    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    // ARRANGE
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // NOTE: These tests pass even though the subscribe handler only returns 200 OK.
    // The 400 Bad Request responses come from actix-web's Form extractor validation.
    // When FormData cannot be deserialized from the request body (missing required fields),
    // the web::Form<FormData> extraction fails BEFORE the handler runs.
    // actix-web then automatically converts this extraction failure into a 400 response.
    //
    // This is the power of the FromRequest trait: type-safe validation at the framework level.
    //
    // SCALA EQUIVALENT (http4s):
    //   case req @ POST -> Root / "subscription" =>
    //     req.as[FormData].flatMap { form => Ok() }
    //
    // If req.as[FormData] fails (missing fields, invalid format), http4s automatically
    // returns 400 Bad Request via DecodeFailure → MalformedMessageBodyFailure handling.
    // The Ok() block never runs, just like our Rust handler never runs on extraction failure.
    //
    // Both frameworks use the same pattern: typeclass-based decoding with automatic error handling.
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_msg) in test_cases {
        // ACT
        let response = client
            .post(&format!("{}/subscription", app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // ASSERT
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_msg
        )
    }
}

// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and crash.
fn spawn_app() -> String {
    let config = get_configuration().expect("Failed to read config");
    let testing_address = config.server.with_random_port();
    let listener: TcpListener =
        TcpListener::bind(testing_address).expect("Failed to bind to the address");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener).expect("Failed to bind address"); // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);

    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}
