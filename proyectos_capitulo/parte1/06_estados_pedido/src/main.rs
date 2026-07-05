#[derive(Debug, Clone, PartialEq)]
enum EstadoPedido {
    Borrador,
    Confirmado { fecha_confirmacion: String },
    EnPreparacion { almacen: String },
    Enviado { paqueteria: String, guia: String },
    Entregado { fecha_entrega: String },
    Cancelado { motivo: String, fecha: String },
}

#[derive(Debug, Clone)]
struct Pedido {
    folio: String,
    estado: EstadoPedido,
}

impl Pedido {
    fn new(folio: &str) -> Self {
        Pedido { folio: folio.into(), estado: EstadoPedido::Borrador }
    }

    fn confirmar(&mut self, fecha: &str) -> Result<(), String> {
        if self.estado != EstadoPedido::Borrador {
            return Err(format!("No se puede confirmar desde {:?}", self.estado));
        }
        self.estado = EstadoPedido::Confirmado { fecha_confirmacion: fecha.into() };
        Ok(())
    }

    fn preparar(&mut self, almacen: &str) -> Result<(), String> {
        if let EstadoPedido::Confirmado { .. } = self.estado {
            self.estado = EstadoPedido::EnPreparacion { almacen: almacen.into() };
            Ok(())
        } else {
            Err(format!("Para preparar debe estar Confirmado, está {:?}", self.estado))
        }
    }

    fn enviar(&mut self, paqueteria: &str, guia: &str) -> Result<(), String> {
        if let EstadoPedido::EnPreparacion { .. } = self.estado {
            self.estado = EstadoPedido::Enviado { paqueteria: paqueteria.into(), guia: guia.into() };
            Ok(())
        } else {
            Err(format!("Para enviar debe estar EnPreparacion, está {:?}", self.estado))
        }
    }

    fn entregar(&mut self, fecha: &str) -> Result<(), String> {
        if let EstadoPedido::Enviado { .. } = self.estado {
            self.estado = EstadoPedido::Entregado { fecha_entrega: fecha.into() };
            Ok(())
        } else {
            Err(format!("Para entregar debe estar Enviado, está {:?}", self.estado))
        }
    }

    fn cancelar(&mut self, motivo: &str, fecha: &str) -> Result<(), String> {
        if matches!(self.estado, EstadoPedido::Entregado { .. }) {
            return Err("No se puede cancelar un pedido ya entregado".into());
        }
        self.estado = EstadoPedido::Cancelado { motivo: motivo.into(), fecha: fecha.into() };
        Ok(())
    }
}

fn main() {
    let mut p = Pedido::new("PED-0001");
    println!("[Inicio]        {:?}", p.estado);

    p.confirmar("2026-07-05").unwrap();
    println!("[Confirmado]    {:?}", p.estado);

    p.preparar("CEDIS-Tlanepantla").unwrap();
    println!("[En preparación] {:?}", p.estado);

    p.enviar("DHL", "1234567890").unwrap();
    println!("[Enviado]       {:?}", p.estado);

    p.entregar("2026-07-08").unwrap();
    println!("[Entregado]     {:?}", p.estado);

    match p.cancelar("Cliente lo rechazó", "2026-07-09") {
        Ok(()) => println!("Cancelado (inesperado)"),
        Err(e) => println!("[Error esperado] {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transicion_completa() {
        let mut p = Pedido::new("PED-T1");
        assert_eq!(p.estado, EstadoPedido::Borrador);
        p.confirmar("2026-01-01").unwrap();
        p.preparar("CEDIS").unwrap();
        p.enviar("DHL", "12345").unwrap();
        p.entregar("2026-01-02").unwrap();
        assert!(matches!(p.estado, EstadoPedido::Entregado { .. }));
    }

    #[test]
    fn no_se_puede_preparar_borrador() {
        let mut p = Pedido::new("PED-T2");
        assert!(p.preparar("CEDIS").is_err());
    }

    #[test]
    fn no_se_puede_cancelar_entregado() {
        let mut p = Pedido::new("PED-T3");
        p.confirmar("2026-01-01").unwrap();
        p.preparar("CEDIS").unwrap();
        p.enviar("DHL", "1").unwrap();
        p.entregar("2026-01-02").unwrap();
        assert!(p.cancelar("X", "2026-01-03").is_err());
    }

    #[test]
    fn cancelar_borrador_es_valido() {
        let mut p = Pedido::new("PED-T4");
        p.cancelar("Cliente no quería", "2026-01-01").unwrap();
        assert!(matches!(p.estado, EstadoPedido::Cancelado { .. }));
    }
}
