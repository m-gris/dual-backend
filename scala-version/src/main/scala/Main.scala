import cats.effect.*
import cats.implicits.*

import org.http4s.*
import org.http4s.dsl.io.*
import org.http4s.ember.server.EmberServerBuilder
import org.http4s.server.Router
import org.http4s.circe.CirceEntityDecoder.* // For automatic JSON/form decoding

import com.comcast.ip4s.*
import io.circe.Decoder

// RUST EQUIVALENT: #[tokio::main]
object Main extends IOApp.Simple:

  // RUST EQUIVALENT: async fn greet(req: HttpRequest) -> impl Responder
  // case GET -> Root / "path" => ... is like .route("/path", web::get().to(handler))
  def greetRoutes: HttpRoutes[IO] =
    HttpRoutes.of[IO] {
      case GET -> Root / "greet"        => Ok("Hello World")
      case GET -> Root / "greet" / name => Ok(s"Hello $name")
    }

  def healthCheck: HttpRoutes[IO] =
    HttpRoutes.of[IO] { case GET -> Root / "health_check" =>
      Ok()
    }

  // RUST EQUIVALENT: struct FormData with #[derive(serde::Deserialize)]
  // In Scala 3: case class + derives clause (like Rust's #[derive(...)])
  case class FormData(email: String, name: String) derives Decoder

  // RUST EQUIVALENT: async fn subscribe(form: web::Form<FormData>) -> HttpResponse
  // NOTE: In http4s, EntityDecoder[IO, FormData] plays the role of Rust's FromRequest trait
  // When decoding fails, http4s automatically returns 400 Bad Request (just like actix-web!)
  def subscriptionRoutes: HttpRoutes[IO] =
    HttpRoutes.of[IO] {
      case req @ POST -> Root / "subscription" =>
        // RUST: web::Form<FormData> extraction happens automatically as function parameter
        // SCALA: We explicitly call .as[FormData] which uses the implicit Decoder
        // BOTH: Decode failure → 400 Bad Request automatically (no explicit validation!)
        req.as[FormData].flatMap { formData =>
          // For now, just return 200 OK (not using formData yet, just like Rust version)
          // The power is in the automatic validation via type-safe decoding
          Ok()
        }
        // NOTE: We don't need .handleErrorWith for decode failures!
        // http4s middleware automatically converts DecodeFailure → 400 Bad Request
    }

  def routes = (greetRoutes <+> healthCheck <+> subscriptionRoutes)

  // Main entry point - this is where execution starts
  // RUST EQUIVALENT: async fn main() -> Result<(), std::io::Error>
  override def run: IO[Unit] =
    EmberServerBuilder
      .default[IO]
      .withHost(ipv4"127.0.0.1")
      .withPort(port"8001")
      .withHttpApp(routes.orNotFound)
      // KEY INSIGHT: Both .build (Scala) and .run() (Rust) are LAZY
      // They describe the work but don't execute it yet!
      .build
      // .use actually runs the server and manages its lifecycle
      // The _ => ... is the function that runs while server is alive
      // RUST EQUIVALENT: .await - actually executes the Future
      .use(_ => IO.println("Server ready at http://127.0.0.1:8001") *> IO.never)

// ========================================
// SIDE-BY-SIDE COMPARISON
// ========================================
//
// RUST (src/main.rs):
// ┌────────────────────────────────────────────────────────────────────
// │ #[tokio::main]                          // Setup async runtime
// │ async fn main() -> Result<(), Error> {  // Entry point
// │     HttpServer::new(|| {                // Create server
// │         App::new()
// │             .route("/", web::get().to(greet))
// │             .route("/name", web::get().to(greet))
// │     })
// │     .bind("127.0.0.1:8000")?            // Configure address
// │     .run()                              // Returns Future (lazy!)
// │     .await                              // Execute Future
// │ }
// └────────────────────────────────────────────────────────────────────
//
// SCALA (this file):
// ┌────────────────────────────────────────────────────────────────────
// │ object Main extends IOApp.Simple {      // Setup async runtime
// │   override def run: IO[Unit] =          // Entry point
// │     EmberServerBuilder                  // Create server
// │       .default[IO]
// │       .withHost(ipv4"127.0.0.1")        // Configure address
// │       .withPort(port"8001")             // Note: port 8001 vs Rust's 8000
// │       .withHttpApp(greetRoutes.orNotFound)
// │       .build                            // Returns Resource (lazy!)
// │       .use(_ => IO.never)               // Execute Resource
// │ }
// └────────────────────────────────────────────────────────────────────
//
// KEY PARALLELS:
// 1. Both separate DESCRIPTION (.run()/.build) from EXECUTION (.await/.use)
// 2. Both use type-safe effect systems (Future in Rust, IO in Scala)
// 3. Both have left-biased error handling (Result in Rust, IO in Scala)
// 4. Both frameworks use builder pattern for server configuration
