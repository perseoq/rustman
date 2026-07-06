#[macro_use] extern crate rocket;
use serde_json::{json, Value};
use rocket::serde::json::Json;

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

#[get("/")]
fn info() -> Json<Value> {
    Json(json!({
        "servicio": "Sistema de Tickets Rocket",
        "version": "0.1.0",
        "estado": "activo"
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![health, info])
}
