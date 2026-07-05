use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Cliente {
    id: u32,
    nombre: String,
    rfc: String,
    email: String,
}

struct AppState {
    clientes: Mutex<Vec<Cliente>>,
    siguiente_id: Mutex<u32>,
}

async fn listar(state: web::Data<AppState>) -> impl Responder {
    let c = state.clientes.lock().unwrap();
    HttpResponse::Ok().json(&*c)
}

async fn crear(state: web::Data<AppState>, body: web::Json<Cliente>) -> impl Responder {
    let mut c = state.clientes.lock().unwrap();
    let mut id = state.siguiente_id.lock().unwrap();
    let mut nuevo = body.into_inner();
    nuevo.id = *id;
    *id += 1;
    c.push(nuevo.clone());
    HttpResponse::Created().json(nuevo)
}

async fn obtener(state: web::Data<AppState>, path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let c = state.clientes.lock().unwrap();
    match c.iter().find(|x| x.id == id) {
        Some(cliente) => HttpResponse::Ok().json(cliente),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "no encontrado"})),
    }
}

async fn eliminar(state: web::Data<AppState>, path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let mut c = state.clientes.lock().unwrap();
    let pos = c.iter().position(|x| x.id == id);
    match pos {
        Some(i) => { c.remove(i); HttpResponse::NoContent().finish() },
        None => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        clientes: Mutex::new(vec![
            Cliente { id: 1, nombre: "Constructora del Bajío".into(), rfc: "CDB010101AB3".into(), email: "c@cdb.mx".into() },
            Cliente { id: 2, nombre: "Distribuidora del Norte".into(),  rfc: "DDN020202CD4".into(), email: "v@ddn.mx".into() },
        ]),
        siguiente_id: Mutex::new(3),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/clientes", web::get().to(listar))
            .route("/clientes", web::post().to(crear))
            .route("/clientes/{id}", web::get().to(obtener))
            .route("/clientes/{id}", web::delete().to(eliminar))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
