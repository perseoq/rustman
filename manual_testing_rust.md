# Manual de testing unitario y avanzado en Rust

> **Desde `#[test]` hasta property-based testing, mocking, fuzzing y cobertura. Aplicado a los proyectos del repositorio: ERP, inventarios, tickets.**

---

## 1. Introduccion

Rust tiene uno de los mejores sistemas de testing de cualquier lenguaje de programación, y no es casualidad. El equipo de Rust diseñó el testing como una característica de primera clase desde el principio, no como una ocurrencia tardía. Cuando ejecutas `cargo new`, Cargo crea automáticamente un módulo de tests en `src/lib.rs`. Cuando ejecutas `cargo test`, Cargo compila y ejecuta todos los tests del proyecto sin configuración adicional. Esta integración nativa hace que escribir tests sea parte natural del desarrollo, no una tarea separada.

El sistema de testing de Rust se organiza en tres niveles, cada uno con un propósito diferente:

1. **Tests unitarios** (`#[test]`): verifican funciones individuales del dominio. Se colocan dentro del mismo archivo que el código que prueban, en un módulo `#[cfg(test)]`. Son rápidos (microsegundos) y no requieren base de datos ni servidor. Deberían ser la mayoría de tus tests.

2. **Tests de integración**: verifican que múltiples módulos funcionan juntos. Se colocan en archivos dentro de `tests/`. Cada archivo es un crate independiente que importa tu librería. Pueden usar bases de datos de prueba, servidores HTTP, etc.

3. **Tests de documentación** (`/// ```rust`): verifican que los ejemplos en la documentación compilan y funcionan. Son ejecutados por `cargo test` y garantizan que la documentación no se queda obsoleta. Si cambias una función y olvidas actualizar el ejemplo, el test de documentación falla.

Además de estos tres niveles, Rust tiene herramientas para técnicas avanzadas: `proptest` para property-based testing (generar cientos de casos automáticamente), `mockall` para mocking (simular dependencias externas), `cargo-fuzz` para fuzzing (enviar datos aleatorios buscando bugs), y `tarpaulin` para cobertura de código.

### 1.1 La filosofia del testing en Rust

Rust tiene una filosofía particular sobre los tests: **los tests son código, y el código debe estar cerca de lo que prueba**. Por eso los tests unitarios se escriben en el mismo archivo que el código que prueban, no en un archivo separado como en otros lenguajes. Esto tiene ventajas: ves el test justo después de la función, puedes probar funciones privadas, y los tests se actualizan junto con el código.

Otra característica única de Rust es que los tests se compilan condicionalmente con `#[cfg(test)]`. Esto significa que el código de testing NO se incluye en el binario de producción. En otros lenguajes como Java o Python, el código de test está presente en producción (aunque no se ejecute). En Rust, el binario de producción ni siquiera sabe que existen los tests.

### 1.2 Estructura de proyectos de testing

```
proyectos_testing/
├── Cargo.toml
├── src/
│   └── lib.rs              ← Código principal con tests unitarios dentro
├── tests/                   ← Tests de integración
│   ├── api_erp/            ← Tests HTTP contra el ERP
│   ├── common/
│       └── mod.rs          ← Helpers compartidos
├── fuzz/                    ← Targets de fuzzing
└── test_rapidos/            ← Tests unitarios del dominio
```

## 2. Tests unitarios basicos

Los tests unitarios son la base de cualquier estrategia de testing. Verifican que una función específica devuelve el resultado esperado para una entrada específica.

### 2.1 Funciones de calculo de IVA

Vamos a escribir tests para las funciones de cálculo fiscal del ERP. Estas funciones son ideales para tests unitarios porque son deterministas: misma entrada siempre produce misma salida.

```rust
// src/dominio/calculos.rs

/// Calcula el IVA de un subtotal a una tasa dada.
pub fn calcular_iva(subtotal: f64, tasa: f64) -> f64 {
    subtotal * (tasa / 100.0)
}

/// Calcula el total sumando subtotal e IVA.
pub fn calcular_total(subtotal: f64, iva: f64) -> f64 {
    subtotal + iva
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calcular_iva_16_porciento() {
        let iva = calcular_iva(1000.0, 16.0);
        assert_eq!(iva, 160.0);
    }

    #[test]
    fn test_calcular_total_con_iva() {
        let total = calcular_total(1000.0, 160.0);
        assert_eq!(total, 1160.0);
    }

    #[test]
    fn test_iva_de_cero() {
        let iva = calcular_iva(500.0, 0.0);
        assert_eq!(iva, 0.0);
    }

    #[test]
    fn test_calcular_total_devuelve_f64_positivo() {
        let total = calcular_total(0.0, 0.0);
        assert!(total >= 0.0);
    }
}
```

**Análisis línea por línea**:

- `#[cfg(test)]`: este atributo le dice al compilador que el módulo `tests` solo debe compilarse cuando se ejecuta `cargo test`. En `cargo build`, este módulo se ignora completamente. El código de test no termina en el binario de producción.

- `use super::*;`: importa todas las funciones del módulo padre (`calculos.rs`). Sin esta línea, no podrías llamar a `calcular_iva()` porque el módulo `tests` está en un ámbito hijo.

- `#[test]`: esta macro convierte la función en un test. Cargo busca todas las funciones marcadas con `#[test]`, las compila y las ejecuta.

- `assert_eq!(iva, 160.0)`: verifica que `iva` es exactamente `160.0`. Si no lo es, el test falla con un mensaje como `assertion failed: (left == right), left: 161.0, right: 160.0`.

- `assert!(total >= 0.0)`: verifica que la condición booleana es `true`. Si es `false`, el test falla.

**Salida esperada**:

```bash
$ cargo test

running 4 tests
test tests::test_calcular_iva_16_porciento ... ok
test tests::test_calcular_total_con_iva ... ok
test tests::test_iva_de_cero ... ok
test tests::test_calcular_total_devuelve_f64_positivo ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 2.2 Tests con Result

A veces es más cómodo que un test devuelva `Result<T, E>` en lugar de hacer `assert!`:

```rust
#[test]
fn test_calcular_iva_return_result() -> Result<(), String> {
    let iva = calcular_iva(1000.0, 16.0);
    if iva == 160.0 {
        Ok(())
    } else {
        Err(format!("Se esperaba 160.0, se obtuvo {}", iva))
    }
}
```

Si el `Result` es `Ok`, el test pasa. Si es `Err`, el test falla con el mensaje de error.

### 2.3 Errores tipicos

**Error 1: olvidar `#[cfg(test)]`**.

```rust
// CÓDIGO QUE FALLA
mod tests {
    #[test]
    fn mi_test() { }
}
```

El módulo se compila SIEMPRE, incluso en producción. `cargo build` incluye el código de test en el binario. Solución: añadir `#[cfg(test)]` encima de `mod tests`.

**Error 2: no importar del módulo padre**.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_calcular_iva() {
        let iva = calcular_iva(100.0, 16.0);  // ERROR
    }
}
```

Mensaje: `error[E0425]: cannot find function 'calcular_iva' in this scope`. Solución: añadir `use super::*;` dentro del módulo de tests.

## 3. Tests de integracion

Los tests de integración verifican que los módulos funcionan juntos. Se colocan en archivos `.rs` dentro de `tests/`.

```rust
// tests/test_clientes.rs
use proyecto_erp::dominio::clientes::*;

#[test]
fn test_crear_cliente_valido() {
    let cliente = Cliente::nuevo(
        "Empresa Test",
        "TST010101XXX",
        "test@empresa.mx",
        50000.0
    );
    assert!(cliente.es_valido());
}
```

Cada archivo en `tests/` es un crate independiente. Deben importar tu librería como si fueran un usuario externo.

## 4. Property-based testing con `proptest`

Los tests tradicionales prueban casos que el programador elige. El property-based testing genera automáticamente cientos de casos y verifica propiedades.

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn iva_nunca_negativo(subtotal: f64, tasa in 0.0..100.0) {
        let iva = calcular_iva(subtotal.abs(), tasa);
        assert!(iva >= 0.0, "El IVA nunca debe ser negativo");
    }
}
```

**Análisis**: `proptest!` genera 256 combinaciones de `subtotal` y `tasa`. Si alguna combinación hace que el IVA sea negativo, el test falla.

## 5. Mocking con `mockall`

Cuando un caso de uso depende de un repositorio, necesitas mocks para evitar la base de datos real.

```rust
use mockall::automock;

#[automock]
pub trait ProductoRepositorio {
    async fn obtener(&self, sku: &str) -> Result<Option<Producto>, String>;
}

#[tokio::test]
async fn test_registrar_entrada_producto_no_existe() {
    let mut mock_repo = MockProductoRepositorio::new();
    mock_repo
        .expect_obtener()
        .with(predicate::eq("NO_EXISTE"))
        .returning(|_| Ok(None));

    let service = InventarioService::new(Box::new(mock_repo));
    let resultado = service.registrar_entrada("NO_EXISTE", 1, 10.0, None).await;
    assert!(resultado.is_err());
}
```

## 6. Fuzzing con cargo-fuzz

```bash
cargo install cargo-fuzz
cargo fuzz init
```

```rust
// fuzz/targets/validate_rfc.rs
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(rfc) = std::str::from_utf8(data) {
        let _ = validar_rfc(rfc);
    }
});
```

## 7. Cobertura con tarpaulin

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --ignore-tests
cargo tarpaulin --out Html --output-dir coverage/
```

## 8. Ejercicios

1. Escribir tests unitarios para `validar_rfc()` del módulo de clientes del ERP.
2. Implementar un mock del repositorio de productos y testear `transferir_stock()`.
3. Agregar un test de integración que verifique flujo completo de pedido.
4. Escribir una propiedad con `proptest` para `calcular_iva`.
5. Configurar `tarpaulin` y generar reporte de cobertura.
6. Agregar fuzzing a la validación de RFC.

## 9. Soluciones

Las soluciones están en `proyectos_testing/tests/` y `proyectos_testing/fuzz/`.
