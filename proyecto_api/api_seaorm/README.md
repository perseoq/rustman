# API ERP/CRM - Variante SeaORM (esqueleto)

Este directorio contiene la variante con **SeaORM** de la API del ERP/CRM. SeaORM es un ORM asíncrono moderno construido sobre `sqlx` con un sistema de entidades generadas a partir de la base de datos.

## Diferencias con la versión Diesel

| Aspecto | Diesel (`api_diesel/`) | SeaORM (`api_seaorm/`) |
|---|---|---|
| Ejecución | Síncrona (bloquea el hilo) | Asíncrona (no bloquea) |
| Esquema | `schema.rs` generado por diesel CLI | Entities generadas por sea-orm-cli |
| Pool | `r2d2::Pool` | `sqlx::MySqlPool` |
| Consultas | DSL de macros `clientes::table` | Métodos fluidos `Entity::find()` |
| Transacciones | `conn.transaction(\|tx\| { ... })` | `tx.commit()` / rollback automático |
| Setup | Más maduro, ecosistema más amplio | Más moderno, mejor con `tokio` |

## Estructura paralela

```
api_seaorm/
├── Cargo.toml
├── Dockerfile
├── migration/        # migraciones de SeaORM
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── errores.rs
│   ├── auth.rs       # idéntico a api_diesel
│   ├── handlers.rs   # idéntico a api_diesel
│   └── entity/       # generado por sea-orm-cli
│       ├── cliente.rs
│       ├── producto.rs
│       └── ...
```

## Comandos clave

```bash
# Instalar sea-orm-cli
cargo install sea-orm-cli

# Generar entities desde la BD
sea-orm-cli generate entity -u mysql://root:secret@127.0.0.1:3306/erp_crm -o src/entity

# Compilar
cargo build --release

# Ejecutar
./target/release/api_seaorm
```

## Endpoints

Idénticos a la versión Diesel — ver `manual_rust.md` sección 3.14.

## Migración conceptual Diesel → SeaORM

Si tienes una API en Diesel y quieres migrar a SeaORM, los pasos son:

1. **Instalar sea-orm-cli** y generar las entities desde la BD existente.
2. **Reemplazar el pool**: `r2d2::Pool` → `sea_orm::DatabaseConnection`.
3. **Reescribir el repositorio**: las llamadas `clientes::table.find(id).first(&mut conn)` se convierten en `Cliente::find_by_id(id).one(&db).await`.
4. **Convertir handlers** de síncronos a asíncronos (añadir `.await` en cada llamada).
5. **Mantener la misma API HTTP** — los handlers HTTP no cambian.
