use crate::errores::ResultadoNegocio;

#[derive(Debug, Clone)]
pub struct Cliente {
    pub id: u32,
    pub nombre: String,
    pub rfc: String,
    pub email: String,
    pub credito: f64,
    pub activo: bool,
}

impl Cliente {
    pub fn new(id: u32, nombre: &str, rfc: &str, email: &str, credito: f64) -> ResultadoNegocio<Self> {
        if nombre.is_empty() { return Err(crate::errores::ErrorNegocio::Validacion("Nombre vacío".into())); }
        if rfc.len() < 12 { return Err(crate::errores::ErrorNegocio::Validacion("RFC inválido".into())); }
        if !email.contains('@') { return Err(crate::errores::ErrorNegocio::Validacion("Email inválido".into())); }
        Ok(Cliente { id, nombre: nombre.into(), rfc: rfc.into(), email: email.into(), credito, activo: true })
    }
    pub fn tiene_credito(&self, monto: f64) -> bool { monto <= self.credito && self.activo }
    pub fn desactivar(&mut self) { self.activo = false; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cliente_valido() {
        let c = Cliente::new(1, "X", "XEXX010101000", "a@b.co", 1000.0);
        assert!(c.is_ok());
    }

    #[test]
    fn cliente_sin_email_es_invalido() {
        let c = Cliente::new(1, "X", "XEXX010101000", "sin_arroba", 1000.0);
        assert!(c.is_err());
    }
}
