use actix_web::{HttpRequest, Responder};

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req
        .match_info() // Extracts ROUTE PARAMETERS from URL path (e.g., "/greet/{name}")
        .get("name")
        .unwrap_or("World");
    format!("Hello {}", name) // Expression returning String (not unit like println!)
}
