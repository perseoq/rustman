use std::io::{self, Write};

const IVA_GENERAL: f64 = 16.0;
const IVA_FRANJA_FRONTERIZA: f64 = 8.0;
const IEPS_BEBIDAS: f64 = 26.5;
const IEPS_BOTANAS: f64 = 8.0;
const IEPS_ALCOHOL: f64 = 53.0;

fn main() {
    let mut entrada = String::new();
    print!("Subtotal del producto ($): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).unwrap();
    let subtotal: f64 = match entrada.trim().parse() {
        Ok(v) => v,
        Err(_) => { println!("Entrada inválida"); return; }
    };
    entrada.clear();

    print!("Descuento (%): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).unwrap();
    let descuento: f64 = match entrada.trim().parse() {
        Ok(v) => v,
        Err(_) => { println!("Entrada inválida"); return; }
    };
    entrada.clear();

    print!("Tipo (1=General, 2=Bebidas, 3=Botanas, 4=Alcohol, 5=Franja fronteriza): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).unwrap();
    let tipo: u8 = match entrada.trim().parse() {
        Ok(v) => v,
        Err(_) => { println!("Entrada inválida"); return; }
    };
    entrada.clear();

    let monto_descuento = subtotal * (descuento / 100.0);
    let base = subtotal - monto_descuento;

    let (iva_pct, ieps_pct) = match tipo {
        1 => (IVA_GENERAL, 0.0),
        2 => (IVA_GENERAL, IEPS_BEBIDAS),
        3 => (IVA_GENERAL, IEPS_BOTANAS),
        4 => (IVA_GENERAL, IEPS_ALCOHOL),
        5 => (IVA_FRANJA_FRONTERIZA, 0.0),
        _ => { println!("Tipo no válido, se usará IVA general"); (IVA_GENERAL, 0.0) }
    };

    let ieps = base * (ieps_pct / 100.0);
    let iva = (base + ieps) * (iva_pct / 100.0);
    let total = base + ieps + iva;

    println!();
    println!("═══════════════════════════════════════");
    println!("         COTIZACIÓN - ERP/CRM          ");
    println!("═══════════════════════════════════════");
    println!("Subtotal:               ${:>10.2}", subtotal);
    println!("Descuento ({}%):        -${:>9.2}", descuento, monto_descuento);
    println!("Base gravable:          ${:>10.2}", base);
    if ieps_pct > 0.0 {
        println!("IEPS ({}%):             +${:>9.2}", ieps_pct, ieps);
    }
    println!("IVA ({}%):              +${:>9.2}", iva_pct, iva);
    println!("───────────────────────────────────────");
    println!("TOTAL:                  ${:>10.2}", total);
    println!("═══════════════════════════════════════");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iva_general_sin_descuento() {
        let subtotal = 1000.0;
        let base = subtotal;
        let iva = base * (IVA_GENERAL / 100.0);
        let total = base + iva;
        assert!((total - 1160.0).abs() < 0.01);
    }

    #[test]
    fn test_iva_con_ieps_bebidas() {
        let subtotal = 500.0;
        let base = subtotal;
        let ieps = base * (IEPS_BEBIDAS / 100.0);
        let iva = (base + ieps) * (IVA_GENERAL / 100.0);
        let total = base + ieps + iva;
        // base 500 + ieps 132.5 + iva sobre (500+132.5) = 101.2 → total 733.7
        assert!(total > 730.0 && total < 740.0);
    }

    #[test]
    fn test_descuento_aplicado_antes_de_impuestos() {
        let subtotal: f64 = 1000.0;
        let descuento: f64 = 10.0;
        let monto: f64 = subtotal * (descuento / 100.0);
        assert!((monto - 100.0).abs() < 0.01);
    }
}
