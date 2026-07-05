use crate::errores::ResultadoNegocio;

#[derive(Debug, Clone)]
pub struct Producto {
    pub sku: String,
    pub nombre: String,
    pub precio: f64,
    pub costo: f64,
    pub stock: u32,
}

impl Producto {
    pub fn new(sku: &str, nombre: &str, precio: f64, costo: f64, stock: u32) -> ResultadoNegocio<Self> {
        if sku.is_empty() { return Err(crate::errores::ErrorNegocio::Validacion("SKU vacío".into())); }
        if precio < costo { return Err(crate::errores::ErrorNegocio::Validacion("Precio < costo".into())); }
        Ok(Producto { sku: sku.into(), nombre: nombre.into(), precio, costo, stock })
    }
    pub fn margen(&self) -> f64 { if self.precio <= 0.0 { 0.0 } else { (self.precio - self.costo) / self.precio * 100.0 } }
    pub fn descontar_stock(&mut self, cantidad: u32) -> ResultadoNegocio<()> {
        if cantidad > self.stock {
            return Err(crate::errores::ErrorNegocio::StockInsuficiente { disponible: self.stock, solicitado: cantidad });
        }
        self.stock -= cantidad;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn producto_valido() {
        let p = Producto::new("X", "Test", 100.0, 50.0, 10);
        assert!(p.is_ok());
    }

    #[test]
    fn precio_menor_que_costo_falla() {
        let p = Producto::new("X", "Test", 50.0, 100.0, 10);
        assert!(p.is_err());
    }

    #[test]
    fn descontar_stock_funciona() {
        let mut p = Producto::new("X", "Test", 100.0, 50.0, 10).unwrap();
        p.descontar_stock(3).unwrap();
        assert_eq!(p.stock, 7);
    }

    #[test]
    fn descontar_mas_del_stock_falla() {
        let mut p = Producto::new("X", "Test", 100.0, 50.0, 10).unwrap();
        assert!(p.descontar_stock(20).is_err());
    }
}
