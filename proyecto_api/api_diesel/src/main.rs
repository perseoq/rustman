mod auth;
mod errores;
mod handlers;
mod modelos;
mod repositorios;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use handlers::AppState;
use mysql::Pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let pool = Pool::new(database_url.as_str()).expect("Pool");
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
    let puerto = std::env::var("PUERTO").unwrap_or_else(|_| "8080".into());
    let bind = format!("127.0.0.1:{}", puerto);

    println!("╔══════════════════════════════════════╗");
    println!("║  ERP/CRM API (Diesel-style)         ║");
    println!("║  http://{}               ", bind);
    println!("╚══════════════════════════════════════╝");

    let state = web::Data::new(AppState { pool, jwt_secret });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(cors)
            .app_data(web::JsonConfig::default().limit(1024 * 1024))  // 1 MB
            .route("/health", web::get().to(handlers::health))
            .route("/api/auth/login", web::post().to(handlers::login))
            .route("/api/clientes", web::get().to(handlers::listar_clientes))
            .route("/api/clientes", web::post().to(handlers::crear_cliente))
            .route("/api/clientes/{id}", web::get().to(handlers::obtener_cliente))
            .route("/api/clientes/{id}", web::put().to(handlers::actualizar_cliente))
            .route("/api/clientes/{id}", web::delete().to(handlers::eliminar_cliente))
            .route("/api/productos", web::get().to(handlers::listar_productos))
            .route("/api/productos", web::post().to(handlers::crear_producto))
            .route("/api/productos/{sku}", web::get().to(handlers::obtener_producto))
            .route("/api/categorias", web::get().to(handlers::listar_categorias))
            .route("/api/proveedores", web::get().to(handlers::listar_proveedores))
            .route("/api/pedidos", web::get().to(handlers::listar_pedidos))
            .route("/api/pedidos", web::post().to(handlers::crear_pedido))
            .route("/api/pedidos/{folio}", web::get().to(handlers::obtener_pedido))
            .route("/api/pedidos/{folio}/lineas", web::get().to(handlers::listar_lineas_pedido))
            .route("/api/reportes/ventas", web::get().to(handlers::reporte_ventas))
            .route("/api/usuarios", web::get().to(handlers::listar_usuarios))
    })
    .bind(bind)?
    .run()
    .await
}
