use crate::errores::{ErrorApi, Resultado};
use crate::modelos::Usuario;
use actix_web::HttpRequest;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use mysql::prelude::*;
use mysql::Pool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub rol: String,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

pub fn login(pool: &Pool, input: LoginInput, secret: &str) -> Resultado<String> {
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    let fila: Option<(u32, String, String, String, String, bool)> = conn.exec_first(
        "SELECT id, username, nombre, email, rol, activo FROM usuarios WHERE username = ?",
        (&input.username,)
    ).map_err(ErrorApi::from)?;

    let user = fila.ok_or(ErrorApi::NoAutorizado)?;
    let (id, username, _nombre, _email, rol, _activo) = user;
    let _ = id;

    // En producción, verificar password hasheado con bcrypt/argon2.
    // Aquí aceptamos cualquier password distinto de vacío como demo.
    if input.password.is_empty() {
        return Err(ErrorApi::NoAutorizado);
    }

    let exp = (chrono::Utc::now() + chrono::Duration::hours(8)).timestamp() as usize;
    let claims = Claims { sub: username, rol, exp };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    Ok(token)
}

pub fn verificar_token(req: &HttpRequest, secret: &str) -> Resultado<Claims> {
    let auth = req.headers().get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(ErrorApi::TokenInvalido)?;

    let validation = Validation::default();
    let token_data = decode::<Claims>(auth, &DecodingKey::from_secret(secret.as_bytes()), &validation)
        .map_err(|_| ErrorApi::TokenInvalido)?;
    Ok(token_data.claims)
}

pub fn listar_usuarios(pool: &Pool) -> Resultado<Vec<Usuario>> {
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    let filas: Vec<(u32, String, String, String, String, bool)> = conn.query(
        "SELECT id, username, nombre, email, rol, activo FROM usuarios WHERE activo = TRUE ORDER BY id"
    ).map_err(ErrorApi::from)?;
    Ok(filas.into_iter().map(|(id, username, nombre, email, rol, activo)| Usuario {
        id, username, nombre, email, rol, activo
    }).collect())
}
