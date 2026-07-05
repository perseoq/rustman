use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cliente {
    pub id: u32,
    pub nombre: String,
    pub rfc: String,
    pub email: Option<String>,
    pub telefono: Option<String>,
    pub credito: f64,
    pub activo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Producto {
    pub sku: String,
    pub nombre: String,
    pub categoria_id: Option<u32>,
    pub precio: f64,
    pub costo: f64,
    pub stock: i32,
    pub activo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Categoria {
    pub id: u32,
    pub nombre: String,
    pub descripcion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proveedor {
    pub id: u32,
    pub nombre: String,
    pub rfc: String,
    pub dias_pago: u8,
    pub activo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pedido {
    pub folio: String,
    pub cliente_id: u32,
    pub subtotal: f64,
    pub iva: f64,
    pub total: f64,
    pub estado: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineaPedido {
    pub id: u64,
    pub pedido_folio: String,
    pub sku: String,
    pub cantidad: u32,
    pub precio_unitario: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usuario {
    pub id: u32,
    pub username: String,
    pub nombre: String,
    pub email: String,
    pub rol: String,
    pub activo: bool,
}
