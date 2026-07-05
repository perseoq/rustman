use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum UnidadMedida { Pieza, Kilogramo, Litro, Metro, Caja }

#[derive(Debug, Clone)]
pub struct Cliente {
    pub id: u32,
    pub nombre: String,
    pub rfc: String,
    pub email: String,
    pub telefono: String,
    pub credito: f64,
    pub activo: bool,
}

impl Cliente {
    pub fn new(id: u32, nombre: &str, rfc: &str, email: &str, telefono: &str, credito: f64) -> Self {
        Cliente { id, nombre: nombre.into(), rfc: rfc.into(), email: email.into(), telefono: telefono.into(), credito, activo: true }
    }
    pub fn desactivar(&mut self) { self.activo = false; }
    pub fn tiene_credito(&self, monto: f64) -> bool { monto <= self.credito && self.activo }
}

impl fmt::Display for Cliente {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.nombre, self.rfc)
    }
}

#[derive(Debug, Clone)]
pub struct Producto {
    pub sku: String,
    pub nombre: String,
    pub precio: f64,
    pub costo: f64,
    pub stock: u32,
    pub unidad: UnidadMedida,
}

impl Producto {
    pub fn new(sku: &str, nombre: &str, precio: f64, costo: f64, stock: u32, unidad: UnidadMedida) -> Self {
        Producto { sku: sku.into(), nombre: nombre.into(), precio, costo, stock, unidad }
    }
    pub fn margen(&self) -> f64 {
        if self.precio <= 0.0 { 0.0 } else { (self.precio - self.costo) / self.precio * 100.0 }
    }
    pub fn valor_inventario(&self) -> f64 { self.costo * self.stock as f64 }
}

#[derive(Debug, Clone)]
pub struct LineaPedido {
    pub sku: String,
    pub cantidad: u32,
    pub precio_unitario: f64,
    pub descuento_pct: f64,
}

impl LineaPedido {
    pub fn new(sku: &str, cantidad: u32, precio_unitario: f64, descuento_pct: f64) -> Self {
        LineaPedido { sku: sku.into(), cantidad, precio_unitario, descuento_pct }
    }
    pub fn subtotal(&self) -> f64 {
        let base = self.cantidad as f64 * self.precio_unitario;
        base * (1.0 - self.descuento_pct / 100.0)
    }
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
    pub fn new(folio: &str, cliente_id: u32) -> Self {
        Pedido { folio: folio.into(), cliente_id, lineas: Vec::new(), estado: EstadoPedido::Borrador }
    }
    pub fn agregar_linea(&mut self, linea: LineaPedido) { self.lineas.push(linea); }
    pub fn subtotal(&self) -> f64 { self.lineas.iter().map(|l| l.subtotal()).sum() }
    pub fn iva(&self) -> f64 { self.subtotal() * 0.16 }
    pub fn total(&self) -> f64 { self.subtotal() + self.iva() }
    pub fn confirmar(&mut self) -> Result<(), String> {
        if self.lineas.is_empty() { return Err("Pedido sin líneas".into()); }
        if self.estado != EstadoPedido::Borrador { return Err(format!("No se puede confirmar un pedido en estado {:?}", self.estado)); }
        self.estado = EstadoPedido::Confirmado;
        Ok(())
    }
}

fn main() {
    let cliente = Cliente::new(1, "Constructora del Bajío", "CDB010101AB3", "contacto@cdb.mx", "5551234567", 100_000.0);
    let laptop = Producto::new("SKU-001", "Laptop HP", 18999.0, 12500.0, 10, UnidadMedida::Pieza);
    let mouse  = Producto::new("SKU-002", "Mouse óptico", 350.0, 150.0, 50, UnidadMedida::Pieza);

    let mut pedido = Pedido::new("PED-0001", cliente.id);
    pedido.agregar_linea(LineaPedido::new(&laptop.sku, 2, laptop.precio, 5.0));
    pedido.agregar_linea(LineaPedido::new(&mouse.sku, 5, mouse.precio, 0.0));
    pedido.confirmar().unwrap();

    println!("Cliente: {}", cliente);
    println!("Pedido:  {}", pedido.folio);
    println!("Estado:  {:?}", pedido.estado);
    println!("Subtotal: ${:.2}", pedido.subtotal());
    println!("IVA:      ${:.2}", pedido.iva());
    println!("Total:    ${:.2}", pedido.total());
    println!("Cliente tiene crédito? {}", cliente.tiene_credito(pedido.total()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cliente_tiene_credito_suficiente() {
        let c = Cliente::new(1, "X", "XEXX010101000", "a@b.co", "1234567890", 5000.0);
        assert!(c.tiene_credito(1000.0));
    }

    #[test]
    fn cliente_sin_credito_suficiente() {
        let c = Cliente::new(1, "X", "XEXX010101000", "a@b.co", "1234567890", 1000.0);
        assert!(!c.tiene_credito(5000.0));
    }

    #[test]
    fn cliente_desactivado_no_tiene_credito() {
        let mut c = Cliente::new(1, "X", "XEXX010101000", "a@b.co", "1234567890", 10000.0);
        c.desactivar();
        assert!(!c.tiene_credito(1000.0));
    }

    #[test]
    fn producto_margen() {
        let p = Producto::new("X", "Test", 200.0, 100.0, 5, UnidadMedida::Pieza);
        assert!((p.margen() - 50.0).abs() < 0.01);
    }

    #[test]
    fn linea_subtotal_con_descuento() {
        let l = LineaPedido::new("X", 4, 100.0, 25.0);
        // 4 * 100 = 400; 25% descuento = 300
        assert!((l.subtotal() - 300.0).abs() < 0.01);
    }

    #[test]
    fn pedido_total_con_iva() {
        let mut p = Pedido::new("PED-T", 1);
        p.agregar_linea(LineaPedido::new("A", 1, 1000.0, 0.0));
        p.confirmar().unwrap();
        // 1000 + 16% = 1160
        assert!((p.total() - 1160.0).abs() < 0.01);
    }

    #[test]
    fn confirmar_pedido_vacio_falla() {
        let mut p = Pedido::new("PED-T", 1);
        assert!(p.confirmar().is_err());
    }
}
