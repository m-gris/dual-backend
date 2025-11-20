// Imports
// :: is the path/namespace separator (for modules, types, static functions)
// . is for method calls on instances
// Example: String::from("text") vs my_string.len()
use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
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

// SCALA EQUIVALENT:
//   case req @ POST -> Root / "subscription" =>
//     req.as[FormData].flatMap { formData => Ok() }
//
// The key difference:
//   RUST: Extraction happens as parameter (web::Form<FormData>)
//         Type-level composition: FromRequest trait + serde Deserialize
//   SCALA: Extraction happens explicitly via .as[FormData]
//          Type-level composition: EntityDecoder[IO, FormData] + circe Decoder
//
// Both achieve the same: decode failure → 400 Bad Request, success → handler runs
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
) -> HttpResponse {
    // NOTE: We only return 200 OK here, but the endpoint automatically returns
    // 400 Bad Request when form data is invalid/missing.
    // This happens because web::Form<FormData> extraction fails before this handler runs,
    // and actix-web converts the extraction error into a 400 response automatically.
    //
    // SCALA: Same behavior - if req.as[FormData] fails to decode, http4s middleware
    //        automatically returns 400 Bad Request via DecodeFailure handling

    // unique id to CORRELATE all logs related to the same request.
    let request_id = Uuid::new_v4();

    // creating a SPAN, to capture the whole http request
    let request_span = tracing::info_span!(
    "Adding a new subscriber",
        // associate structured information to our spans
        // as a collection of key-value pairs.
        // the % symbol tells tracing to use their Display implementation for logging purposes
        // IMPLICITLY
        %request_id,
        // EXPLICITY
        subscriber_email = %_form.email,
        subscriber_name = %_form.name
    );

    let _request_span_guard = request_span.enter();

    // NOTE: thanks to TRACING’s log feature flag,
    // every time an event or a span are created using tracing’s macros
    // a corresponding log event is emitted, allowing loggers to pick up on it

    // WARNING: we do NOT call `.enter` on the query_span !
    // This is handled by `.instrument` (invoked downstream)
    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    // `Result` of sqlx::query! has two variants: `Ok` and `Err`.
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
    .execute(_db_conn.get_ref()) // an immutable reference to the `PgPool` wrapped by `web::Data`.
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("request_id {request_id} - New subscriber details saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            /*
            using {:?}, the std::fmt::Debug format,
            to capture the query error and extract as much information as possible
            */
            tracing::error!("request_id {request_id} - Failed to execute query:{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
