use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorNegocio {
    Validacion(String),
    NoEncontrado(String),
    StockInsuficiente { disponible: u32, solicitado: u32 },
    CreditoInsuficiente { disponible: f64, requerido: f64 },
}

impl fmt::Display for ErrorNegocio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorNegocio::Validacion(m) => write!(f, "Validación: {}", m),
            ErrorNegocio::NoEncontrado(q) => write!(f, "No encontrado: {}", q),
            ErrorNegocio::StockInsuficiente { disponible, solicitado } =>
                write!(f, "Stock insuficiente: {} disponibles, {} solicitados", disponible, solicitado),
            ErrorNegocio::CreditoInsuficiente { disponible, requerido } =>
                write!(f, "Crédito insuficiente: ${} disponibles, ${} requeridos", disponible, requerido),
        }
    }
}

impl std::error::Error for ErrorNegocio {}

pub type ResultadoNegocio<T> = Result<T, ErrorNegocio>;
