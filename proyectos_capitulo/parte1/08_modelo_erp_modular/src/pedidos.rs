use crate::clientes::Cliente;
use crate::errores::ResultadoNegocio;

#[derive(Debug, Clone)]
pub struct LineaPedido {
    pub sku: String,
    pub cantidad: u32,
    pub precio_unitario: f64,
}

impl LineaPedido {
    pub fn new(sku: &str, cantidad: u32, precio_unitario: f64) -> Self {
        LineaPedido { sku: sku.into(), cantidad, precio_unitario }
    }
    pub fn subtotal(&self) -> f64 { self.cantidad as f64 * self.precio_unitario }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EstadoPedido { Borrador, Confirmado, Enviado, Entregado, Cancelado }

#[derive(Debug, Clone)]
pub struct Pedido {
    pub folio: String,
    pub cliente_id: u32,
    pub lineas: Vec<LineaPedido>,
    pub estado: EstadoPedido,
}

impl Pedido {
    pub fn new(folio: &str, cliente: &Cliente) -> Self {
        Pedido { folio: folio.into(), cliente_id: cliente.id, lineas: Vec::new(), estado: EstadoPedido::Borrador }
    }
    pub fn agregar_linea(&mut self, linea: LineaPedido) { self.lineas.push(linea); }
    pub fn subtotal(&self) -> f64 { self.lineas.iter().map(|l| l.subtotal()).sum() }
    pub fn iva(&self) -> f64 { self.subtotal() * 0.16 }
    pub fn total(&self) -> f64 { self.subtotal() + self.iva() }
    pub fn confirmar(&mut self) -> ResultadoNegocio<()> {
        if self.lineas.is_empty() { return Err(crate::errores::ErrorNegocio::Validacion("Pedido sin líneas".into())); }
        self.estado = EstadoPedido::Confirmado;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cliente_test() -> Cliente {
        Cliente::new(1, "X", "XEXX010101000", "a@b.co", 10000.0).unwrap()
    }

    #[test]
    fn total_con_iva() {
        let mut p = Pedido::new("PED-T1", &cliente_test());
        p.agregar_linea(LineaPedido::new("A", 1, 1000.0));
        assert!((p.total() - 1160.0).abs() < 0.01);
    }

    #[test]
    fn confirmar_pedido_vacio() {
        let mut p = Pedido::new("PED-T2", &cliente_test());
        assert!(p.confirmar().is_err());
    }

    #[test]
    fn confirmar_pedido_con_lineas() {
        let mut p = Pedido::new("PED-T3", &cliente_test());
        p.agregar_linea(LineaPedido::new("A", 1, 100.0));
        p.confirmar().unwrap();
        assert_eq!(p.estado, EstadoPedido::Confirmado);
    }
}
