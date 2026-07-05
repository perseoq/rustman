mod clientes;
mod productos;
mod pedidos;
mod errores;

use clientes::Cliente;
use productos::Producto;
use pedidos::{Pedido, LineaPedido};

fn main() {
    let cliente = Cliente::new(1, "Constructora del Bajío", "CDB010101AB3", "contacto@cdb.mx", 100_000.0).unwrap();
    let mut laptop = Producto::new("SKU-001", "Laptop HP", 18999.0, 12500.0, 10).unwrap();
    let mouse = Producto::new("SKU-002", "Mouse óptico", 350.0, 150.0, 50).unwrap();

    let mut pedido = Pedido::new("PED-0001", &cliente);
    pedido.agregar_linea(LineaPedido::new(&laptop.sku, 2, laptop.precio));
    pedido.agregar_linea(LineaPedido::new(&mouse.sku, 5, mouse.precio));
    pedido.confirmar().unwrap();

    laptop.descontar_stock(2).unwrap();

    println!("Pedido {} confirmado. Total: ${:.2}", pedido.folio, pedido.total());
    println!("Stock restante de {}: {}", laptop.sku, laptop.stock);

    match laptop.descontar_stock(1000) {
        Ok(()) => println!("Algo falló"),
        Err(e) => println!("Error esperado: {}", e),
    }
}
