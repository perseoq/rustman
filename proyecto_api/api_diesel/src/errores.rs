use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum ErrorApi {
    NoEncontrado(String),
    Validacion(String),
    BaseDatos(String),
    NoAutorizado,
    TokenInvalido,
    Conflicto(String),
}

impl fmt::Display for ErrorApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoEncontrado(s) => write!(f, "No encontrado: {}", s),
            Self::Validacion(s) => write!(f, "Validación: {}", s),
            Self::BaseDatos(s) => write!(f, "BD: {}", s),
            Self::NoAutorizado => write!(f, "No autorizado"),
            Self::TokenInvalido => write!(f, "Token inválido o expirado"),
            Self::Conflicto(s) => write!(f, "Conflicto: {}", s),
        }
    }
}

impl ResponseError for ErrorApi {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NoEncontrado(_) => StatusCode::NOT_FOUND,
            Self::Validacion(_) => StatusCode::BAD_REQUEST,
            Self::BaseDatos(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NoAutorizado | Self::TokenInvalido => StatusCode::UNAUTHORIZED,
            Self::Conflicto(_) => StatusCode::CONFLICT,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(json!({ "error": self.to_string(), "tipo": format!("{:?}", self) }))
    }
}

impl From<mysql::Error> for ErrorApi {
    fn from(e: mysql::Error) -> Self {
        let s = e.to_string();
        if s.contains("Duplicate entry") {
            ErrorApi::Conflicto(s)
        } else if s.contains("foreign key constraint") {
            ErrorApi::Validacion("Referencia inválida".into())
        } else {
            ErrorApi::BaseDatos(s)
        }
    }
}

pub type Resultado<T> = Result<T, ErrorApi>;
