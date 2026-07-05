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

fn listar_clientes(pool: &Pool) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    let clientes: Vec<(u32, String, String, f64, bool)> =
        conn.query("SELECT id, nombre, rfc, credito, activo FROM clientes ORDER BY id")?;
    println!("\nClientes:");
    for (id, nombre, rfc, credito, activo) in clientes {
        println!("  [{}] {} ({}) - ${:.2} {}", id, nombre, rfc, credito, if activo {"✓"} else {"✗"});
    }
    Ok(())
}

fn listar_productos(pool: &Pool) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    let productos: Vec<(String, String, f64, i64)> =
        conn.query("SELECT sku, nombre, precio, stock FROM productos WHERE activo = TRUE ORDER BY sku")?;
    println!("\nProductos:");
    for (sku, nombre, precio, stock) in productos {
        let alerta = if stock < 10 { " ⚠ STOCK BAJO" } else { "" };
        println!("  [{}] {} - ${:.2} (stock: {}){}", sku, nombre, precio, stock, alerta);
    }
    Ok(())
}

fn crear_pedido(pool: &Pool) -> mysql::Result<()> {
    let cliente_id: u32 = match leer_linea("ID del cliente: ").parse() {
        Ok(v) => v,
        Err(_) => { println!("ID inválido"); return Ok(()); }
    };

    // Verificar que el cliente existe
    {
        let mut conn = pool.get_conn()?;
        let existe: Option<u32> = conn.exec_first("SELECT id FROM clientes WHERE id = ? AND activo = TRUE", (cliente_id,))?;
        if existe.is_none() {
            println!("✗ Cliente no existe o no está activo");
            return Ok(());
        }
    }

    // Listar productos
    let productos: Vec<(String, String, f64, i64)> = {
        let mut conn = pool.get_conn()?;
        conn.query("SELECT sku, nombre, precio, stock FROM productos WHERE activo = TRUE AND stock > 0")?
    };
    println!("\nProductos disponibles:");
    for (sku, nombre, precio, stock) in &productos {
        println!("  [{}] {} - ${:.2} (stock: {})", sku, nombre, precio, stock);
    }

    // Construir líneas
    let mut lineas: Vec<(String, f64, u32)> = Vec::new();
    loop {
        let sku = leer_linea("\nSKU (ENTER para terminar): ");
        if sku.is_empty() { break; }
        let cantidad: u32 = leer_linea("Cantidad: ").parse().unwrap_or(0);
        if cantidad == 0 { continue; }
        match productos.iter().find(|(s, _, _, _)| s == &sku) {
            Some((s, _, p, st)) => {
                if (cantidad as i64) > *st {
                    println!("✗ Stock insuficiente ({})", st);
                    continue;
                }
                lineas.push((s.clone(), *p, cantidad));
            }
            None => println!("SKU no encontrado"),
        }
    }

    if lineas.is_empty() {
        println!("Cancelado");
        return Ok(());
    }

    let subtotal: f64 = lineas.iter().map(|(_, p, c)| p * *c as f64).sum();
    let iva = subtotal * 0.16;
    let total = subtotal + iva;
    let folio = format!("PED-2026-{:04}", (lineas.len() as u16) * 100 + (total as u16) % 100);

    let mut conn = pool.get_conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;

    tx.exec_drop(
        "INSERT INTO pedidos (folio, cliente_id, subtotal, iva, total, estado) VALUES (?, ?, ?, ?, ?, 'CONFIRMADO')",
        (&folio, cliente_id, subtotal, iva, total)
    )?;

    for (sku, precio, cantidad) in &lineas {
        tx.exec_drop(
            "INSERT INTO lineas_pedido (pedido_folio, sku, cantidad, precio_unitario) VALUES (?, ?, ?, ?)",
            (&folio, sku, cantidad, precio)
        )?;
        tx.exec_drop(
            "UPDATE productos SET stock = stock - ? WHERE sku = ?",
            (cantidad, sku)
        )?;
    }

    tx.commit()?;
    println!("\n✓ Pedido {} creado con {} líneas. Total: ${:.2}", folio, lineas.len(), total);
    Ok(())
}

fn reporte_stock_bajo(pool: &Pool) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    let productos: Vec<(String, String, i64)> = conn.query(
        "SELECT sku, nombre, stock FROM productos WHERE activo = TRUE AND stock < 10 ORDER BY stock"
    )?;
    println!("\nProductos con stock bajo ({}):", productos.len());
    for (sku, nombre, stock) in productos {
        println!("  ⚠ {} ({}) - {} unidades", sku, nombre, stock);
    }
    Ok(())
}

fn main() -> mysql::Result<()> {
    let pool = Pool::new(DB_URL)?;
    loop {
        println!("\n=== ERP/CRM - CLI Completo ===");
        println!("1) Clientes");
        println!("2) Productos");
        println!("3) Crear pedido");
        println!("4) Reporte: stock bajo");
        println!("0) Salir");
        let op = leer_linea("Opción: ");
        let r = match op.as_str() {
            "1" => listar_clientes(&pool),
            "2" => listar_productos(&pool),
            "3" => crear_pedido(&pool),
            "4" => reporte_stock_bajo(&pool),
            "0" => break,
            _ => { println!("Opción inválida"); Ok(()) }
        };
        if let Err(e) = r { eprintln!("Error: {}", e); }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn listar_clientes_no_falla() {
        let pool = Pool::new(DB_URL).expect("conexión");
        let _: Vec<(u32, String, String, f64, bool)> = pool.get_conn().expect("conn")
            .query("SELECT id, nombre, rfc, credito, activo FROM clientes")
            .expect("query");
    }
}
