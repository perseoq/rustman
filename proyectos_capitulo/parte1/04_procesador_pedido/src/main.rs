#[derive(Debug, Clone)]
#[allow(dead_code)]
struct LineaPedido {
    sku: String,
    descripcion: String,
    cantidad: u32,
    precio_unitario: f64,
}

impl LineaPedido {
    fn new(sku: &str, descripcion: &str, cantidad: u32, precio: f64) -> Self {
        LineaPedido {
            sku: sku.to_string(),
            descripcion: descripcion.to_string(),
            cantidad,
            precio_unitario: precio,
        }
    }

    fn subtotal(&self) -> f64 {
        self.cantidad as f64 * self.precio_unitario
    }
}

struct Pedido {
    folio: String,
    lineas: Vec<LineaPedido>,
}

impl Pedido {
    fn new(folio: &str) -> Self {
        Pedido { folio: folio.to_string(), lineas: Vec::new() }
    }

    fn agregar(&mut self, linea: LineaPedido) {
        self.lineas.push(linea);
    }

    fn calcular_subtotal(&self) -> f64 {
        self.lineas.iter().map(|l| l.subtotal()).sum()
    }
}

fn imprimir_pedido(pedido: &Pedido) {
    println!("Pedido {}", pedido.folio);
    for (i, linea) in pedido.lineas.iter().enumerate() {
        println!("  {}. {} - {} x ${:.2} = ${:.2}",
                 i + 1, linea.sku, linea.cantidad, linea.precio_unitario, linea.subtotal());
    }
    println!("  ------------------------------------");
    println!("  Subtotal: ${:.2}", pedido.calcular_subtotal());
}

fn main() {
    let mut pedido = Pedido::new("PED-0001");
    pedido.agregar(LineaPedido::new("SKU-001", "Laptop HP",    2, 18999.0));
    pedido.agregar(LineaPedido::new("SKU-002", "Mouse óptico",  5,   350.0));
    pedido.agregar(LineaPedido::new("SKU-003", "Teclado",       3,  1200.0));

    imprimir_pedido(&pedido);
    println!("\nTotal de líneas: {}", pedido.lineas.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtotal_de_linea() {
        let l = LineaPedido::new("SKU-1", "Test", 3, 10.0);
        assert!((l.subtotal() - 30.0).abs() < 0.01);
    }

    #[test]
    fn subtotal_pedido_multiple_lineas() {
        let mut p = Pedido::new("PED-T");
        p.agregar(LineaPedido::new("A", "a", 2, 50.0));   // 100
        p.agregar(LineaPedido::new("B", "b", 1, 25.0));   //  25
        assert!((p.calcular_subtotal() - 125.0).abs() < 0.01);
    }

    #[test]
    fn agregar_incrementa_longitud() {
        let mut p = Pedido::new("PED-T2");
        assert_eq!(p.lineas.len(), 0);
        p.agregar(LineaPedido::new("X", "x", 1, 1.0));
        assert_eq!(p.lineas.len(), 1);
    }
}
