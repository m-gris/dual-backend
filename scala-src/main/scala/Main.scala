import cats.effect.*
import cats.implicits.*

import org.http4s.*
import org.http4s.dsl.io.*
import org.http4s.ember.server.EmberServerBuilder
import org.http4s.server.Router

import com.comcast.ip4s.*

// RUST EQUIVALENT: #[tokio::main]
object Main extends IOApp.Simple:

  // RUST EQUIVALENT: async fn greet(req: HttpRequest) -> impl Responder
  // case GET -> Root / "path" => ... is like .route("/path", web::get().to(handler))
  def greetRoutes: HttpRoutes[IO] =
    HttpRoutes.of[IO] {
      case GET -> Root / "greet"        => Ok("Hello World")
      case GET -> Root / "greet" / name => Ok(s"Hello ${name}")
    }

  def healthCheck: HttpRoutes[IO] =
    HttpRoutes.of[IO] { case GET -> Root / "health_check" =>
      Ok()
    }

  def routes = (greetRoutes <+> healthCheck)

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
