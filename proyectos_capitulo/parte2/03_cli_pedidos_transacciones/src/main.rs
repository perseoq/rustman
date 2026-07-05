use mysql::prelude::*;
use mysql::{Pool, TxOpts};
use std::io::{self, Write};

const DB_URL: &str = "mysql://root:secret@127.0.0.1:3306/erp_crm";

fn leer_linea(prompt: &str) -> String {
    let mut s = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

#[derive(Debug, Clone)]
struct Linea {
    sku: String,
    nombre: String,
    precio: f64,
    stock: i64,
    cantidad: u32,
}

fn crear_pedido(pool: &Pool) -> mysql::Result<()> {
    let cliente_id: u32 = match leer_linea("ID del cliente: ").parse() {
        Ok(v) => v,
        Err(_) => { println!("ID inválido"); return Ok(()); }
    };

    let productos: Vec<(String, String, f64, i64)> = {
        let mut conn = pool.get_conn()?;
        conn.query("SELECT sku, nombre, precio, stock FROM productos WHERE activo = TRUE AND stock > 0")
    }?;

    if productos.is_empty() {
        println!("No hay productos disponibles");
        return Ok(());
    }

    println!("\nProductos disponibles:");
    for (sku, nombre, precio, stock) in &productos {
        println!("  [{}] {} - ${:.2} (stock: {})", sku, nombre, precio, stock);
    }

    let mut lineas: Vec<Linea> = Vec::new();
    loop {
        let sku = leer_linea("\nSKU (o ENTER para terminar): ");
        if sku.is_empty() { break; }
        let cantidad: u32 = match leer_linea("Cantidad: ").parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if cantidad == 0 { continue; }
        match productos.iter().find(|(s, _, _, _)| s == &sku) {
            Some((s, n, p, st)) => lineas.push(Linea { sku: s.clone(), nombre: n.clone(), precio: *p, stock: *st, cantidad }),
            None => println!("SKU {} no encontrado", sku),
        }
    }

    if lineas.is_empty() {
        println!("Pedido cancelado (sin líneas)");
        return Ok(());
    }

    for l in &lineas {
        if (l.cantidad as i64) > l.stock {
            println!("✗ Stock insuficiente para {}: tienes {}, necesitas {}", l.sku, l.stock, l.cantidad);
            return Ok(());
        }
    }

    let subtotal: f64 = lineas.iter().map(|l| l.cantidad as f64 * l.precio).sum();
    let iva = subtotal * 0.16;
    let total = subtotal + iva;
    let folio = format!("PED-2026-{:04}", rand::random::<u16>() % 10000);

    let mut conn = pool.get_conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;

    tx.exec_drop(
        "INSERT INTO pedidos (folio, cliente_id, subtotal, iva, total, estado) VALUES (?, ?, ?, ?, ?, 'CONFIRMADO')",
        (&folio, cliente_id, subtotal, iva, total)
    )?;

    for l in &lineas {
        tx.exec_drop(
            "INSERT INTO lineas_pedido (pedido_folio, sku, cantidad, precio_unitario) VALUES (?, ?, ?, ?)",
            (&folio, &l.sku, l.cantidad, l.precio)
        )?;
        tx.exec_drop(
            "UPDATE productos SET stock = stock - ? WHERE sku = ? AND stock >= ?",
            (l.cantidad, &l.sku, l.cantidad)
        )?;
    }

    tx.commit()?;
    println!("\n✓ Pedido {} creado. Total: ${:.2}", folio, total);
    println!("  Subtotal: ${:.2}  IVA: ${:.2}", subtotal, iva);
    Ok(())
}

fn main() -> mysql::Result<()> {
    let pool = Pool::new(DB_URL)?;
    crear_pedido(&pool)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conexion_y_listar_productos() {
        let pool = Pool::new(DB_URL).expect("conexión");
        let productos: Vec<(String, String, f64, i64)> = pool.get_conn().expect("conn")
            .query("SELECT sku, nombre, precio, stock FROM productos WHERE activo = TRUE AND stock > 0")
            .expect("query");
        assert!(!productos.is_empty());
    }
}
