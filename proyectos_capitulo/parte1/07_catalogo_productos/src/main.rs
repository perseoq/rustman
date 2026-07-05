use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Producto {
    sku: String,
    nombre: String,
    categoria: String,
    precio: f64,
    costo: f64,
    stock: u32,
}

impl Producto {
    fn margen(&self) -> f64 {
        if self.precio <= 0.0 { 0.0 } else { (self.precio - self.costo) / self.precio * 100.0 }
    }
    fn valor_stock(&self) -> f64 { self.costo * self.stock as f64 }
}

fn main() {
    let catalogo = vec![
        Producto { sku: "SKU-001".into(), nombre: "Laptop HP".into(),       categoria: "Cómputo".into(),   precio: 18999.0, costo: 12500.0, stock: 10 },
        Producto { sku: "SKU-002".into(), nombre: "Mouse óptico".into(),    categoria: "Accesorios".into(), precio:   350.0, costo:   150.0, stock: 50 },
        Producto { sku: "SKU-003".into(), nombre: "Teclado mecánico".into(), categoria: "Accesorios".into(), precio:  1200.0, costo:   600.0, stock: 20 },
        Producto { sku: "SKU-004".into(), nombre: "Monitor 24".into(),      categoria: "Cómputo".into(),   precio:  4500.0, costo:  3000.0, stock: 15 },
        Producto { sku: "SKU-005".into(), nombre: "Cable HDMI".into(),       categoria: "Accesorios".into(), precio:    89.0, costo:    35.0, stock: 100 },
    ];

    let margen_bajo: Vec<&Producto> = catalogo.iter().filter(|p| p.margen() < 20.0).collect();
    println!("Productos con margen bajo ({}):", margen_bajo.len());
    for p in &margen_bajo {
        println!("  {} ({}) margen: {:.1}%", p.nombre, p.sku, p.margen());
    }

    let total_inventario: f64 = catalogo.iter().map(|p| p.valor_stock()).sum();
    println!("\nValor total del inventario: ${:.2}", total_inventario);

    let mut ordenados: Vec<&Producto> = catalogo.iter().collect();
    ordenados.sort_by(|a, b| b.precio.partial_cmp(&a.precio).unwrap());
    println!("\nTop 3 más caros:");
    for p in ordenados.iter().take(3) {
        println!("  {} - ${}", p.nombre, p.precio);
    }

    let mut por_categoria: HashMap<String, Vec<&Producto>> = HashMap::new();
    for p in &catalogo {
        por_categoria.entry(p.categoria.clone()).or_default().push(p);
    }
    println!("\nProductos por categoría:");
    for (cat, prods) in &por_categoria {
        println!("  {} ({}): {}", cat, prods.len(),
                 prods.iter().map(|p| p.nombre.as_str()).collect::<Vec<_>>().join(", "));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prod_test() -> Producto {
        Producto { sku: "X".into(), nombre: "Test".into(), categoria: "Cat".into(), precio: 100.0, costo: 60.0, stock: 5 }
    }

    #[test]
    fn margen_40_por_ciento() {
        let p = prod_test();
        assert!((p.margen() - 40.0).abs() < 0.01);
    }

    #[test]
    fn margen_cero_si_precio_cero() {
        let mut p = prod_test();
        p.precio = 0.0;
        assert_eq!(p.margen(), 0.0);
    }

    #[test]
    fn valor_stock() {
        let p = prod_test();
        assert_eq!(p.valor_stock(), 300.0);
    }

    #[test]
    fn filtrar_margen_bajo() {
        let catalogo = vec![prod_test()];
        let bajos: Vec<&Producto> = catalogo.iter().filter(|p| p.margen() < 50.0).collect();
        assert_eq!(bajos.len(), 1);
    }

    #[test]
    fn total_inventario() {
        let catalogo = vec![prod_test(), prod_test()];
        let total: f64 = catalogo.iter().map(|p| p.valor_stock()).sum();
        assert_eq!(total, 600.0);
    }
}
