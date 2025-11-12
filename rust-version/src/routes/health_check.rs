// Imports
// :: is the path/namespace separator (for modules, types, static functions)
// . is for method calls on instances
// Example: String::from("text") vs my_string.len()
use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    // impl Responder = "returns some concrete type that implements the Responder trait"
    // The caller doesn't know the exact type, just that it satisfies the Responder contract
    // Similar to Scala's abstract type members or existential types
    // Traits ≈ typeclasses (behavior contracts), but impl Trait is more like bounded existentials
    HttpResponse::Ok()
}
