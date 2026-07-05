use mysql::prelude::*;
use mysql::Pool;

const DB_URL: &str = "mysql://root:secret@127.0.0.1:3306/erp_crm";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = Pool::new(DB_URL)?;

    let version: String = pool.get_conn()?.query_first("SELECT VERSION()")?
        .ok_or("No se pudo obtener la versión")?;
    println!("Versión del servidor: {}", version);

    let tablas: Vec<String> = pool.get_conn()?
        .query("SHOW TABLES")?;
    println!("\nTablas en la BD ({}):", tablas.len());
    for t in &tablas {
        println!("  - {}", t);
    }

    let total: i64 = pool.get_conn()?
        .query_first("SELECT COUNT(*) FROM clientes")?
        .unwrap_or(0);
    println!("\nTotal de clientes: {}", total);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conexion_funciona() {
        let pool = Pool::new(DB_URL).expect("no se pudo crear el pool");
        let version: String = pool.get_conn()
            .expect("no se pudo conectar")
            .query_first("SELECT VERSION()")
            .expect("falló el query")
            .expect("sin resultados");
        assert!(version.contains("MariaDB") || version.contains("MySQL"));
    }
}
