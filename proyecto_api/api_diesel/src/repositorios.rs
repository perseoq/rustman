use crate::errores::{ErrorApi, Resultado};
use crate::modelos::*;
use mysql::prelude::*;
use mysql::Pool;
use serde::Deserialize;

pub fn listar_clientes(pool: &Pool, limite: u32) -> Resultado<Vec<Cliente>> {
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    let filas: Vec<(u32, String, String, Option<String>, Option<String>, f64, bool)> = conn.query(
        format!("SELECT id, nombre, rfc, email, telefono, credito, activo FROM clientes ORDER BY id LIMIT {}", limite)
    ).map_err(ErrorApi::from)?;
    Ok(filas.into_iter().map(|(id, nombre, rfc, email, telefono, credito, activo)| Cliente {
        id, nombre, rfc, email, telefono, credito, activo
    }).collect())
}

pub fn obtener_cliente(pool: &Pool, id: u32) -> Resultado<Cliente> {
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    let fila: Option<(u32, String, String, Option<String>, Option<String>, f64, bool)> = conn.exec_first(
        "SELECT id, nombre, rfc, email, telefono, credito, activo FROM clientes WHERE id = ?", (id,)
    ).map_err(ErrorApi::from)?;
    fila.map(|(id, nombre, rfc, email, telefono, credito, activo)| Cliente { id, nombre, rfc, email, telefono, credito, activo })
        .ok_or_else(|| ErrorApi::NoEncontrado(format!("Cliente {}", id)))
}

#[derive(Deserialize)]
pub struct ClienteNuevo {
    pub nombre: String,
    pub rfc: String,
    pub email: Option<String>,
    pub telefono: Option<String>,
    pub credito: f64,
}

pub fn crear_cliente(pool: &Pool, c: ClienteNuevo) -> Resultado<u32> {
    if c.nombre.is_empty() { return Err(ErrorApi::Validacion("Nombre vacío".into())); }
    if c.rfc.len() < 12 { return Err(ErrorApi::Validacion("RFC debe tener al menos 12 caracteres".into())); }
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    conn.exec_drop(
        "INSERT INTO clientes (nombre, rfc, email, telefono, credito) VALUES (?, ?, ?, ?, ?)",
        (&c.nombre, &c.rfc, &c.email, &c.telefono, c.credito)
    ).map_err(ErrorApi::from)?;
    Ok(conn.last_insert_id() as u32)
}

pub fn actualizar_cliente(pool: &Pool, id: u32, c: ClienteNuevo) -> Resultado<()> {
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    conn.exec_drop(
        "UPDATE clientes SET nombre = ?, rfc = ?, email = ?, telefono = ?, credito = ? WHERE id = ?",
        (&c.nombre, &c.rfc, &c.email, &c.telefono, c.credito, id)
    ).map_err(ErrorApi::from)?;
    if conn.affected_rows() == 0 { return Err(ErrorApi::NoEncontrado(format!("Cliente {}", id))); }
    Ok(())
}

pub fn eliminar_cliente(pool: &Pool, id: u32) -> Resultado<()> {
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    conn.exec_drop("DELETE FROM clientes WHERE id = ?", (id,)).map_err(ErrorApi::from)?;
    if conn.affected_rows() == 0 { return Err(ErrorApi::NoEncontrado(format!("Cliente {}", id))); }
    Ok(())
}

pub fn listar_productos(pool: &Pool) -> Resultado<Vec<Producto>> {
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    let filas: Vec<(String, String, Option<u32>, f64, f64, i32, bool)> = conn.query(
        "SELECT sku, nombre, categoria_id, precio, costo, stock, activo FROM productos WHERE activo = TRUE ORDER BY sku"
    ).map_err(ErrorApi::from)?;
    Ok(filas.into_iter().map(|(sku, nombre, categoria_id, precio, costo, stock, activo)| Producto {
        sku, nombre, categoria_id, precio, costo, stock, activo
    }).collect())
}

pub fn obtener_producto(pool: &Pool, sku: &str) -> Resultado<Producto> {
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    let fila: Option<(String, String, Option<u32>, f64, f64, i32, bool)> = conn.exec_first(
        "SELECT sku, nombre, categoria_id, precio, costo, stock, activo FROM productos WHERE sku = ?", (sku,)
    ).map_err(ErrorApi::from)?;
    fila.map(|(sku, nombre, categoria_id, precio, costo, stock, activo)| Producto {
        sku, nombre, categoria_id, precio, costo, stock, activo
    }).ok_or_else(|| ErrorApi::NoEncontrado(format!("Producto {}", sku)))
}

#[derive(Deserialize)]
pub struct ProductoNuevo {
    pub sku: String,
    pub nombre: String,
    pub categoria_id: Option<u32>,
    pub precio: f64,
    pub costo: f64,
    pub stock: i32,
}

pub fn crear_producto(pool: &Pool, p: ProductoNuevo) -> Resultado<()> {
    if p.sku.is_empty() { return Err(ErrorApi::Validacion("SKU vacío".into())); }
    if p.precio < p.costo { return Err(ErrorApi::Validacion("Precio < costo".into())); }
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    conn.exec_drop(
        "INSERT INTO productos (sku, nombre, categoria_id, precio, costo, stock) VALUES (?, ?, ?, ?, ?, ?)",
        (&p.sku, &p.nombre, p.categoria_id, p.precio, p.costo, p.stock)
    ).map_err(ErrorApi::from)?;
    Ok(())
}

pub fn listar_categorias(pool: &Pool) -> Resultado<Vec<Categoria>> {
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    let filas: Vec<(u32, String, Option<String>)> = conn.query("SELECT id, nombre, descripcion FROM categorias ORDER BY id").map_err(ErrorApi::from)?;
    Ok(filas.into_iter().map(|(id, nombre, descripcion)| Categoria { id, nombre, descripcion }).collect())
}

pub fn listar_proveedores(pool: &Pool) -> Resultado<Vec<Proveedor>> {
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    let filas: Vec<(u32, String, String, u8, bool)> = conn.query("SELECT id, nombre, rfc, dias_pago, activo FROM proveedores WHERE activo = TRUE ORDER BY id").map_err(ErrorApi::from)?;
    Ok(filas.into_iter().map(|(id, nombre, rfc, dias_pago, activo)| Proveedor { id, nombre, rfc, dias_pago, activo }).collect())
}

pub fn listar_pedidos(pool: &Pool, limite: u32) -> Resultado<Vec<Pedido>> {
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    let filas: Vec<(String, u32, f64, f64, f64, String, String)> = conn.query(format!(
        "SELECT folio, cliente_id, subtotal, iva, total, estado, created_at FROM pedidos ORDER BY created_at DESC LIMIT {}", limite
    )).map_err(ErrorApi::from)?;
    Ok(filas.into_iter().map(|(folio, cliente_id, subtotal, iva, total, estado, created_at)| Pedido {
        folio, cliente_id, subtotal, iva, total, estado, created_at
    }).collect())
}

pub fn obtener_pedido(pool: &Pool, folio: &str) -> Resultado<Pedido> {
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    let fila: Option<(String, u32, f64, f64, f64, String, String)> = conn.exec_first(
        "SELECT folio, cliente_id, subtotal, iva, total, estado, created_at FROM pedidos WHERE folio = ?", (folio,)
    ).map_err(ErrorApi::from)?;
    fila.map(|(folio, cliente_id, subtotal, iva, total, estado, created_at)| Pedido {
        folio, cliente_id, subtotal, iva, total, estado, created_at
    }).ok_or_else(|| ErrorApi::NoEncontrado(format!("Pedido {}", folio)))
}

pub fn listar_lineas_pedido(pool: &Pool, folio: &str) -> Resultado<Vec<LineaPedido>> {
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    let filas: Vec<(u64, String, String, u32, f64)> = conn.query(
        "SELECT id, pedido_folio, sku, cantidad, precio_unitario FROM lineas_pedido WHERE pedido_folio = ? ORDER BY id"
    ).map_err(ErrorApi::from)?;
    Ok(filas.into_iter().map(|(id, pedido_folio, sku, cantidad, precio_unitario)| LineaPedido {
        id, pedido_folio, sku, cantidad, precio_unitario
    }).collect())
}

#[derive(Deserialize)]
pub struct LineaInput {
    pub sku: String,
    pub cantidad: u32,
    pub precio_unitario: f64,
}

#[derive(Deserialize)]
pub struct PedidoInput {
    pub cliente_id: u32,
    pub lineas: Vec<LineaInput>,
}

pub fn crear_pedido(pool: &Pool, input: PedidoInput) -> Resultado<String> {
    if input.lineas.is_empty() { return Err(ErrorApi::Validacion("Pedido sin líneas".into())); }
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    let subtotal: f64 = input.lineas.iter().map(|l| l.cantidad as f64 * l.precio_unitario).sum();
    let iva = subtotal * 0.16;
    let total = subtotal + iva;
    let folio = format!("PED-2026-{:05}", chrono::Utc::now().timestamp() % 100000);

    let mut tx = conn.start_transaction(mysql::TxOpts::default()).map_err(ErrorApi::from)?;

    // Validar cliente
    let existe: Option<u32> = tx.exec_first("SELECT id FROM clientes WHERE id = ? AND activo = TRUE", (input.cliente_id,)).map_err(ErrorApi::from)?;
    if existe.is_none() { return Err(ErrorApi::Validacion("Cliente no existe".into())); }

    // Validar stock
    for l in &input.lineas {
        let stock: Option<i32> = tx.exec_first("SELECT stock FROM productos WHERE sku = ?", (&l.sku,)).map_err(ErrorApi::from)?;
        match stock {
            None => return Err(ErrorApi::Validacion(format!("Producto {} no existe", l.sku))),
            Some(s) if s < l.cantidad as i32 => return Err(ErrorApi::Conflicto(format!("Stock insuficiente para {}: tiene {}, necesita {}", l.sku, s, l.cantidad))),
            _ => {}
        }
    }

    tx.exec_drop(
        "INSERT INTO pedidos (folio, cliente_id, subtotal, iva, total, estado) VALUES (?, ?, ?, ?, ?, 'CONFIRMADO')",
        (&folio, input.cliente_id, subtotal, iva, total)
    ).map_err(ErrorApi::from)?;

    for l in &input.lineas {
        tx.exec_drop(
            "INSERT INTO lineas_pedido (pedido_folio, sku, cantidad, precio_unitario) VALUES (?, ?, ?, ?)",
            (&folio, &l.sku, l.cantidad, l.precio_unitario)
        ).map_err(ErrorApi::from)?;
        tx.exec_drop(
            "UPDATE productos SET stock = stock - ? WHERE sku = ?",
            (l.cantidad, &l.sku)
        ).map_err(ErrorApi::from)?;
    }

    tx.commit().map_err(ErrorApi::from)?;
    Ok(folio)
}

pub fn reporte_ventas(pool: &Pool) -> Resultado<Vec<(String, f64, i64)>> {
    let mut conn = pool.get_conn().map_err(|e| ErrorApi::BaseDatos(e.to_string()))?;
    let filas: Vec<(String, f64, i64)> = conn.query(
        "SELECT DATE(created_at) AS dia, SUM(total) AS total_dia, COUNT(*) AS num_pedidos
         FROM pedidos WHERE estado != 'CANCELADO' GROUP BY DATE(created_at) ORDER BY dia DESC LIMIT 30"
    ).map_err(ErrorApi::from)?;
    Ok(filas)
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
