// Imports
// :: is the path/namespace separator (for modules, types, static functions)
// . is for method calls on instances
// Example: String::from("text") vs my_string.len()
use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
/*
* EXTRACTORS - Type-safe request parsing (like http4s EntityDecoder)
*
* actix-web provides several extractors out of the box to cater for the most common usecases:
* • Path, Query, Json, Form, etc.
* Extractors can be accessed as an argument to a handler function.
*
* KEY CONCEPT: An extractor is a type that implements the FromRequest trait.
*              (SCALA: Like http4s's EntityDecoder[F, A] typeclass)
*
* All arguments in the signature of a route handler must implement the FromRequest trait.
* actix-web will invoke from_request for each argument and, if the extraction succeeds for all of them,
* it will then run the actual handler function.
*
* If one of the extractions fails:
*   - The corresponding error is returned to the caller (typically 400 Bad Request)
*   - The handler is NEVER invoked
*   - SCALA EQUIVALENT: req.as[FormData].flatMap { form => ... }
*                       If .as[FormData] decode fails, http4s returns 400 automatically
*
* This is extremely convenient: your handler does not have to deal with the raw incoming request
* and can instead work directly with strongly-typed information, significantly simplifying the code.
* */

// SCALA EQUIVALENT: case class FormData(email: String, name: String) derives Decoder
// Both Rust #[derive(...)] and Scala 3 derives use compile-time code generation
// to auto-implement typeclass instances (Deserialize in Rust, Decoder in Scala)
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// NOTE: thanks to TRACING’s log feature flag,
// every time an event or a span are created using tracing’s macros
// a corresponding log event is emitted, allowing loggers to pick up on it
#[tracing::instrument(
    name="Adding a new subscriber", // default: func name, i.e subscribe
    skip(_form, _db_conn),
    fields(
        // CLAUDE: please remind me about this % syntax...
        // unique id to CORRELATE all logs related to the same request.
        request_id=%Uuid::new_v4(),
        subscriber_email=%_form.email,
        subscriber_name=%_form.name
    )
)]
pub async fn subscribe(
    // web::Form<FormData> implements FromRequest trait
    // When actix-web sees this parameter:
    //   1. It calls FromRequest::from_request()
    //   2. That uses serde::Deserialize to parse form data
    //   3. Success → handler runs with parsed data
    //   4. Failure → automatic 400 Bad Request (handler never runs)
    //
    // SCALA: This is like req.as[FormData] using EntityDecoder + Decoder typeclasses
    _form: web::Form<FormData>,
    // Retrieving a connection from the application state!
    // by getting our hands on an Arc<PgPool> in the request handler, using the web::Data extractor:
    _db_conn: web::Data<PgPool>,
    // The key difference:
    //   RUST: Extraction happens as parameter (web::Form<FormData>)
    //         Type-level composition: FromRequest trait + serde Deserialize
    //   SCALA: Extraction happens explicitly via .as[FormData]
    //          Type-level composition: EntityDecoder[IO, FormData] + circe Decoder
) -> HttpResponse {
    // NOTE: We only return 200 OK here, but the endpoint automatically returns
    // 400 Bad Request when form data is invalid/missing.
    // This happens because web::Form<FormData> extraction fails before this handler runs,
    // and actix-web converts the extraction error into a 400 response automatically.
    //
    // SCALA: Same behavior - if req.as[FormData] fails to decode, http4s middleware
    //        automatically returns 400 Bad Request via DecodeFailure handling
    match insert_subscriber(&_form, &_db_conn).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(), // CLAUDE: what are those .finish() ???
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(_form, _db_conn)
)]
pub async fn insert_subscriber(_form: &FormData, _db_conn: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
    // .execute(_db_conn)
    // CLAUDE: why don't we get a ref in this case ????
    .execute(_db_conn) // an immutable reference to the `PgPool` wrapped by `web::Data`.
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
