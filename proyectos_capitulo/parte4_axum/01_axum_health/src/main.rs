use axum::{Router, routing::get, Json};
use serde_json::{json, Value};
use std::net::SocketAddr;

async fn health() -> &'static str {
    "OK"
}

async fn info() -> Json<Value> {
    Json(json!({
        "servicio": "Sistema de Inventarios Axum",
        "version": env!("CARGO_PKG_VERSION"),
        "estado": "activo"
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(info))
        .route("/health", get(health));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Inventarios API escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
