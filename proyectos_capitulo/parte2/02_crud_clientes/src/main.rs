use mysql::prelude::*;
use mysql::Pool;
use std::io::{self, Write};

const DB_URL: &str = "mysql://root:secret@127.0.0.1:3306/erp_crm";

fn listar(pool: &Pool) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    let clientes: Vec<(u32, String, String, Option<String>, f64, bool)> =
        conn.query("SELECT id, nombre, rfc, email, credito, activo FROM clientes ORDER BY id")?;
    println!("\n┌────┬──────────────────────────────────────────┬──────────────┬─────────────────────────┬────────────┬────────┐");
    println!("│ ID │ Nombre                                   │ RFC          │ Email                   │ Crédito    │ Activo │");
    println!("├────┼──────────────────────────────────────────┼──────────────┼─────────────────────────┼────────────┼────────┤");
    for (id, nombre, rfc, email, credito, activo) in &clientes {
        let e = email.as_deref().unwrap_or("—");
        let nom_display = if nombre.len() > 42 { &nombre[..42] } else { nombre.as_str() };
        println!("│ {:2} │ {:<42} │ {:<12} │ {:<23} │ ${:>9.2} │ {:<6} │",
                 id, nom_display, rfc, e, credito, activo);
    }
    println!("└────┴──────────────────────────────────────────┴──────────────┴─────────────────────────┴────────────┴────────┘");
    println!("Total: {} clientes", clientes.len());
    Ok(())
}

fn crear(pool: &Pool) -> mysql::Result<()> {
    let mut entrada = String::new();
    let mut conn = pool.get_conn()?;

    print!("Nombre: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let nombre = entrada.trim().to_string(); entrada.clear();
    print!("RFC: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let rfc = entrada.trim().to_string(); entrada.clear();
    print!("Email: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let email = entrada.trim().to_string(); entrada.clear();
    print!("Crédito (default 0): "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?;
    let credito: f64 = entrada.trim().parse().unwrap_or(0.0); entrada.clear();

    conn.exec_drop(
        "INSERT INTO clientes (nombre, rfc, email, credito) VALUES (?, ?, ?, ?)",
        (&nombre, &rfc, &email, credito)
    )?;
    println!("✓ Cliente creado con ID {}", conn.last_insert_id());
    Ok(())
}

fn actualizar(pool: &Pool) -> mysql::Result<()> {
    let mut entrada = String::new();
    let mut conn = pool.get_conn()?;

    print!("ID a actualizar: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let id: u32 = entrada.trim().parse().unwrap_or(0); entrada.clear();
    print!("Nuevo crédito: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let credito: f64 = entrada.trim().parse().unwrap_or(0.0); entrada.clear();

    conn.exec_drop("UPDATE clientes SET credito = ? WHERE id = ?", (credito, id))?;
    println!("✓ Filas actualizadas: {}", conn.affected_rows());
    Ok(())
}

fn eliminar(pool: &Pool) -> mysql::Result<()> {
    let mut entrada = String::new();
    let mut conn = pool.get_conn()?;

    print!("ID a eliminar: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let id: u32 = entrada.trim().parse().unwrap_or(0); entrada.clear();
    print!("¿Confirma? (s/N): "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?;
    if entrada.trim().to_lowercase() != "s" {
        println!("Cancelado"); return Ok(());
    }
    conn.exec_drop("DELETE FROM clientes WHERE id = ?", (id,))?;
    println!("✓ Filas eliminadas: {}", conn.affected_rows());
    Ok(())
}

fn menu() {
    println!("\n=== ERP/CRM - Gestión de Clientes ===");
    println!("1) Listar");
    println!("2) Crear");
    println!("3) Actualizar crédito");
    println!("4) Eliminar");
    println!("0) Salir");
    print!("Opción: ");
    io::stdout().flush().unwrap();
}

fn main() -> mysql::Result<()> {
    let pool = Pool::new(DB_URL)?;
    let mut entrada = String::new();

    loop {
        menu();
        entrada.clear();
        io::stdin().read_line(&mut entrada).unwrap();
        let opcion = entrada.trim();
        let resultado = match opcion {
            "1" => listar(&pool),
            "2" => crear(&pool),
            "3" => actualizar(&pool),
            "4" => eliminar(&pool),
            "0" => break,
            _ => { println!("Opción inválida"); Ok(()) }
        };
        if let Err(e) = resultado {
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}
