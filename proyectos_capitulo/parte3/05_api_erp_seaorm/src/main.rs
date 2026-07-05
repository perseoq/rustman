// 05_api_erp_seaorm - Resumen del equivalente SeaORM
// Esta es una versión simplificada. La versión "real" usaría SeaORM:
//   use sea_orm::*;
//   use crate::entity::producto::Entity as Producto;
//   Producto::find().all(&db).await
// Aquí mostramos la misma API pero con mysql (sync). El proyecto completo
// de SeaORM se entrega en proyecto_api/api_seaorm/ (estructura paralela).

use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware};
use mysql::prelude::*;
use mysql::Pool;
use serde::Serialize;

const DB_URL: &str = "mysql://root:secret@127.0.0.1:3306/erp_crm";

#[derive(Serialize)]
struct ProductoResumen {
    sku: String,
    nombre: String,
    stock: i32,
    valor: f64,
}

async fn inventario_valorizado(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    };
    let resultado: Result<Vec<(String, String, i32, f64)>, mysql::Error> = conn.query(
        "SELECT sku, nombre, stock, stock * costo AS valor FROM productos WHERE activo = TRUE"
    );
    match resultado {
        Ok(filas) => {
            let r: Vec<ProductoResumen> = filas.into_iter().map(|(sku, nombre, stock, valor)| ProductoResumen {
                sku, nombre, stock, valor
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
    println!("API ERP (SeaORM-style simplificado) en http://127.0.0.1:8084");
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(middleware::Logger::default())
            .route("/api/reportes/inventario-valorizado", web::get().to(inventario_valorizado))
    })
    .bind("127.0.0.1:8084")?
    .run()
    .await
}
