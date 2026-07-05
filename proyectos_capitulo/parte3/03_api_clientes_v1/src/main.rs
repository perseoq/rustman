use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware};
use mysql::prelude::*;
use mysql::Pool;
use serde::{Deserialize, Serialize};

const DB_URL: &str = "mysql://root:secret@127.0.0.1:3306/erp_crm";

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Cliente {
    id: u32,
    nombre: String,
    rfc: String,
    email: String,
    credito: f64,
    activo: bool,
}

#[derive(Deserialize)]
struct ClienteInput {
    nombre: String,
    rfc: String,
    email: String,
    credito: f64,
}

async fn listar(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = match pool.get_conn() {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    };
    let resultado: Result<Vec<(u32, String, String, String, f64, bool)>, mysql::Error> =
        conn.query("SELECT id, nombre, rfc, email, credito, activo FROM clientes ORDER BY id LIMIT 100");
    match resultado {
        Ok(filas) => {
            let clientes: Vec<Cliente> = filas.into_iter().map(|(id, nombre, rfc, email, credito, activo)| Cliente {
                id, nombre, rfc, email, credito, activo
            }).collect();
            HttpResponse::Ok().json(clientes)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    }
}

async fn crear(pool: web::Data<Pool>, body: web::Json<ClienteInput>) -> impl Responder {
    let input = body.into_inner();
    let mut conn = match pool.get_conn() {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    };
    let resultado = conn.exec_drop(
        "INSERT INTO clientes (nombre, rfc, email, credito) VALUES (?, ?, ?, ?)",
        (&input.nombre, &input.rfc, &input.email, input.credito)
    );
    match resultado {
        Ok(()) => HttpResponse::Created().json(serde_json::json!({"id": conn.last_insert_id(), "ok": true})),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string()})),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = web::Data::new(Pool::new(DB_URL).expect("Pool"));

    println!("API Clientes v1 con MySQL escuchando en http://127.0.0.1:8082");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .app_data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .route("/clientes", web::get().to(listar))
            .route("/clientes", web::post().to(crear))
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}
