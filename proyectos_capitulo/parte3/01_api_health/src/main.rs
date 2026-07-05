use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde_json::json;

async fn raiz() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "servicio": "ERP/CRM Rust México",
        "version": env!("CARGO_PKG_VERSION"),
        "estado": "activo"
    }))
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "ok", "timestamp": chrono::Utc::now().to_rfc3339()}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let puerto = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind = format!("127.0.0.1:{}", puerto);
    println!("ERP/CRM API escuchando en http://{}", bind);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(raiz))
            .route("/health", web::get().to(health))
    })
    .bind(bind)?
    .run()
    .await
}
