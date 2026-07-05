fn main() {
    let nombre_erp = "ERP/CRM Rust México";
    let version = "0.1.0";
    let anio_inicio: u32 = 2026;

    println!("╔══════════════════════════════════════╗");
    println!("║  {} v{}            ║", nombre_erp, version);
    println!("║  Fundamentos de Rust - 2026          ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("Bienvenido al sistema. Año de inicio: {}", anio_inicio);
    println!("Compilado con Rust {} en modo {}", rustc_version_runtime(), modo_compilacion());
}

fn rustc_version_runtime() -> String {
    String::from("1.96.1 (stable)")
}

fn modo_compilacion() -> &'static str {
    if cfg!(debug_assertions) { "debug" } else { "release" }
}
