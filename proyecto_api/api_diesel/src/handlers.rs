use crate::auth::{self, LoginInput};
use crate::errores::Resultado;
use crate::modelos::*;
use crate::repositorios;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use mysql::Pool;
use serde::Deserialize;

pub struct AppState {
    pub pool: Pool,
    pub jwt_secret: String,
}

// === Health ===
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok", "servicio": "ERP/CRM API"}))
}

// === Auth ===
pub async fn login(state: web::Data<AppState>, body: web::Json<LoginInput>) -> Resultado<HttpResponse> {
    let token = auth::login(&state.pool, body.into_inner(), &state.jwt_secret)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"token": token})))
}

// === Clientes ===
#[derive(Deserialize)]
pub struct ClientesQuery {
    pub limite: Option<u32>,
}

pub async fn listar_clientes(state: web::Data<AppState>, q: web::Query<ClientesQuery>) -> Resultado<HttpResponse> {
    let clientes = repositorios::listar_clientes(&state.pool, q.limite.unwrap_or(50))?;
    Ok(HttpResponse::Ok().json(clientes))
}

pub async fn obtener_cliente(state: web::Data<AppState>, path: web::Path<u32>) -> Resultado<HttpResponse> {
    let c = repositorios::obtener_cliente(&state.pool, path.into_inner())?;
    Ok(HttpResponse::Ok().json(c))
}

pub async fn crear_cliente(state: web::Data<AppState>, body: web::Json<repositorios::ClienteNuevo>) -> Resultado<HttpResponse> {
    let id = repositorios::crear_cliente(&state.pool, body.into_inner())?;
    Ok(HttpResponse::Created().json(serde_json::json!({"id": id})))
}

pub async fn actualizar_cliente(state: web::Data<AppState>, path: web::Path<u32>, body: web::Json<repositorios::ClienteNuevo>) -> Resultado<HttpResponse> {
    repositorios::actualizar_cliente(&state.pool, path.into_inner(), body.into_inner())?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"ok": true})))
}

pub async fn eliminar_cliente(state: web::Data<AppState>, path: web::Path<u32>) -> Resultado<HttpResponse> {
    repositorios::eliminar_cliente(&state.pool, path.into_inner())?;
    Ok(HttpResponse::NoContent().finish())
}

// === Productos ===
pub async fn listar_productos(state: web::Data<AppState>) -> Resultado<HttpResponse> {
    let p = repositorios::listar_productos(&state.pool)?;
    Ok(HttpResponse::Ok().json(p))
}

pub async fn obtener_producto(state: web::Data<AppState>, path: web::Path<String>) -> Resultado<HttpResponse> {
    let p = repositorios::obtener_producto(&state.pool, &path.into_inner())?;
    Ok(HttpResponse::Ok().json(p))
}

pub async fn crear_producto(state: web::Data<AppState>, body: web::Json<repositorios::ProductoNuevo>) -> Resultado<HttpResponse> {
    repositorios::crear_producto(&state.pool, body.into_inner())?;
    Ok(HttpResponse::Created().json(serde_json::json!({"ok": true})))
}

// === Categorías ===
pub async fn listar_categorias(state: web::Data<AppState>) -> Resultado<HttpResponse> {
    let c = repositorios::listar_categorias(&state.pool)?;
    Ok(HttpResponse::Ok().json(c))
}

// === Proveedores ===
pub async fn listar_proveedores(state: web::Data<AppState>) -> Resultado<HttpResponse> {
    let p = repositorios::listar_proveedores(&state.pool)?;
    Ok(HttpResponse::Ok().json(p))
}

// === Pedidos ===
pub async fn listar_pedidos(state: web::Data<AppState>, q: web::Query<ClientesQuery>) -> Resultado<HttpResponse> {
    let p = repositorios::listar_pedidos(&state.pool, q.limite.unwrap_or(50))?;
    Ok(HttpResponse::Ok().json(p))
}

pub async fn obtener_pedido(state: web::Data<AppState>, path: web::Path<String>) -> Resultado<HttpResponse> {
    let p = repositorios::obtener_pedido(&state.pool, &path.into_inner())?;
    Ok(HttpResponse::Ok().json(p))
}

pub async fn listar_lineas_pedido(state: web::Data<AppState>, path: web::Path<String>) -> Resultado<HttpResponse> {
    let l = repositorios::listar_lineas_pedido(&state.pool, &path.into_inner())?;
    Ok(HttpResponse::Ok().json(l))
}

pub async fn crear_pedido(state: web::Data<AppState>, body: web::Json<repositorios::PedidoInput>) -> Resultado<HttpResponse> {
    let folio = repositorios::crear_pedido(&state.pool, body.into_inner())?;
    Ok(HttpResponse::Created().json(serde_json::json!({"folio": folio, "ok": true})))
}

// === Reportes ===
pub async fn reporte_ventas(state: web::Data<AppState>) -> Resultado<HttpResponse> {
    let r = repositorios::reporte_ventas(&state.pool)?;
    let items: Vec<_> = r.into_iter().map(|(dia, total, num)| serde_json::json!({
        "dia": dia, "total": total, "num_pedidos": num
    })).collect();
    Ok(HttpResponse::Ok().json(items))
}

// === Usuarios (requieren auth) ===
pub async fn listar_usuarios(state: web::Data<AppState>, req: HttpRequest) -> Resultado<HttpResponse> {
    auth::verificar_token(&req, &state.jwt_secret)?;
    let u = auth::listar_usuarios(&state.pool)?;
    Ok(HttpResponse::Ok().json(u))
}
