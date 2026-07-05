// 04_api_erp_diesel - Resumen de la API Diesel (delegada al proyecto final)
// Esta es una versión simplificada que muestra la integración con Actix + MySQL.
// El proyecto completo con 15+ endpoints está en proyecto_api/api_diesel/.

use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware};
use mysql::prelude::*;
use mysql::Pool;
use serde::Serialize;

const DB_URL: &str = "mysql://root:secret@127.0.0.1:3306/erp_crm";

#[derive(Serialize)]
struct ClienteResumen {
    id: u32,
    nombre: String,
    total_pedidos: i64,
    total_comprado: f64,
}

async fn top_clientes(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    };
    let resultado: Result<Vec<(u32, String, Option<i64>, Option<f64>)>, mysql::Error> = conn.query(
        "SELECT c.id, c.nombre, COUNT(p.folio) AS total_pedidos, COALESCE(SUM(p.total),0) AS total_comprado
         FROM clientes c LEFT JOIN pedidos p ON p.cliente_id = c.id
         GROUP BY c.id, c.nombre ORDER BY total_comprado DESC LIMIT 10"
    );
    match resultado {
        Ok(filas) => {
            let r: Vec<ClienteResumen> = filas.into_iter().map(|(id, nombre, total_pedidos, total_comprado)| ClienteResumen {
                id, nombre,
                total_pedidos: total_pedidos.unwrap_or(0),
                total_comprado: total_comprado.unwrap_or(0.0),
            }).collect();
            HttpResponse::Ok().json(r)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = web::Data::new(Pool::new(DB_URL).expect("pool"));
    println!("API ERP (Diesel-style) en http://127.0.0.1:8083");
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(middleware::Logger::default())
            .route("/api/reportes/top-clientes", web::get().to(top_clientes))
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}
