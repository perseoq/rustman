# Manual de testing de APIs REST en Rust

> **Tests de integración, carga y contrato sobre las APIs del repositorio: ERP/CRM (Actix Web), inventarios (Axum), tickets (Rocket) con reqwest, sqlx y herramientas de testing HTTP.**

---

## 1. Introduccion

Cuando terminas de escribir una API, tienes cierta confianza de que funciona porque los handlers se compilaron sin errores y los tests unitarios pasaron. Pero esa confianza es limitada: los tests unitarios prueban funciones aisladas, no prueban que el enrutamiento HTTP funciona, que el middleware de autenticación rechaza peticiones sin token, que la base de datos persiste los datos correctamente, o que el formato JSON de respuesta tiene los campos que el frontend espera.

Los **tests de integración de APIs** llenan este vacío. La idea es simple: levantar el servidor (o una instancia de prueba), enviar peticiones HTTP reales con un cliente HTTP como `reqwest`, y verificar que las respuestas son correctas. Estos tests detectan problemas que los tests unitarios no pueden detectar:

- **Errores de serialización**: el handler devuelve `Ok(Json(producto))` pero `Producto` no implementa `Serialize`. El compilador no lo detecta si usas `#[derive(Serialize)]` incorrectamente.
- **Rutas mal configuradas**: el path en el atributo `#[get("/clientes")]` no coincide con el que espera el cliente.
- **Middlewares que no se ejecutan**: el middleware de logging está registrado pero no se llama.
- **Códigos de estado incorrectos**: crear un cliente devuelve 200 OK en lugar de 201 Created.
- **Headers de respuesta incorrectos**: falta `Content-Type: application/json`.

Este manual cubre el testing de integración para las tres APIs del repositorio. Cada una usa un framework diferente, pero el enfoque de testing es el mismo: `reqwest` para hacer peticiones HTTP, `serde_json` para parsear respuestas, y `#[tokio::test]` para ejecutar tests asíncronos.

Pero hay un problema fundamental con los tests de integración: dependen de que el servidor esté corriendo. No puedes ejecutar `cargo test` sin antes haber ejecutado `cargo run` en otro terminal. Esto hace que los tests de integración sean más lentos y frágiles que los unitarios. La solución es usar `actix_web::test`, `axum::test` o levantar el servidor dentro del propio test con `once_cell`. Cada enfoque tiene sus ventajas y desventajas, y las veremos en las secciones siguientes.

### 1.1 ¿Por que reqwest y no el cliente HTTP de cada framework?

Cada framework tiene su propio mecanismo de testing:

- Actix Web tiene `actix_web::test::init_service()` y `actix_web::test::call_service()`.
- Axum tiene `axum::Router::with_state()` y puedes llamar a `app.call(req)` directamente.
- Rocket tiene su propio `rocket::local::blocking::Client`.

Pero estos mecanismos son específicos de cada framework. Si usas `actix_web::test`, tus tests solo funcionan con Actix Web. Si mañana cambias de framework, tienes que reescribir todos los tests.

`reqwest` es independiente del framework. Hace peticiones HTTP reales a través del protocolo HTTP, sin importar qué framework esté del otro lado. Puedes testear Actix, Axum, Rocket, o incluso APIs escritas en otros lenguajes, con el mismo código. La desventaja es que necesitas el servidor corriendo, pero en CI/CD puedes levantar el servidor en un paso previo del pipeline.

## 2. Test de integracion con reqwest

`reqwest` es el cliente HTTP más usado en Rust. Es mantenido por Sean McArthur (el mismo creador de Hyper) y es la opción recomendada para hacer peticiones HTTP desde Rust. Es asíncrono, soporta JSON, JWT, headers personalizados, cookies, y se integra naturalmente con `#[tokio::test]`.

### 2.1 Dependencias

Antes de escribir tests, necesitamos añadir las dependencias al `Cargo.toml`. Las dependencias de testing se colocan en `[dev-dependencies]` para que no se incluyan en el binario de producción.

```toml
[dev-dependencies]
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

**Análisis línea por línea**:

- `reqwest = { version = "0.12", features = ["json"] }`: el cliente HTTP. La feature `json` habilita los métodos `.json()` para enviar y recibir JSON automáticamente. Sin esta feature, tendrías que serializar/deserializar manualmente con `serde_json`.

- `tokio = { version = "1", features = ["full"] }`: el runtime asíncrono. `reqwest` es asíncrono y necesita un runtime Tokio para ejecutarse. Los tests deben usar `#[tokio::test]` en lugar de `#[test]`.

- `serde = { version = "1", features = ["derive"] }`: necesario para deserializar las respuestas JSON de la API en structs Rust.

- `serde_json = "1"`: para usar `serde_json::Value` cuando no sabemos la estructura exacta de la respuesta.

### 2.2 Test basico: health check

El primer test que deberías escribir para cualquier API es un health check. Verifica que el servidor está corriendo, que acepta conexiones y que responde con el código de estado correcto.

```rust
// tests/api_erp/health_check.rs
use reqwest::Client;

#[tokio::test]
async fn test_health_check_devuelve_200() {
    // Crear un cliente HTTP (reutilizable para múltiples peticiones)
    let client = Client::new();

    // Enviar petición GET a /health
    let respuesta = client
        .get("http://localhost:8080/health")
        .send()
        .await
        .expect("Error al conectar con la API. ¿Está corriendo el servidor?");

    // Verificar que el código de estado es 200 OK
    assert!(respuesta.status().is_success());
    assert_eq!(respuesta.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_health_check_devuelve_ok_texto() {
    let client = Client::new();
    let respuesta = client
        .get("http://localhost:8080/health")
        .send()
        .await
        .unwrap();

    // Obtener el cuerpo de la respuesta como texto
    let texto = respuesta.text().await.unwrap();
    assert_eq!(texto, "OK");
}
```

**Análisis línea por línea**:

- `use reqwest::Client;`: `Client` es el struct principal de reqwest. Internamente mantiene un pool de conexiones, cookies, y configuración. Se crea una vez y se reutiliza. No necesitas crear un Client nuevo para cada petición.

- `#[tokio::test]`: esta macro convierte la función en un test que es ejecutado por el runtime Tokio. Sin ella, no puedes usar `await` dentro del test. Si usas `#[test]` en lugar de `#[tokio::test]`, el compilador te dará un error: `only async functions can be awaited`.

- `let client = Client::new();`: crea un cliente HTTP con configuración por defecto: timeout de 30 segundos, sin cookies persistentes, sin headers por defecto. Para tests más avanzados, puedes usar `Client::builder()` para configurar timeouts específicos.

- `client.get("http://localhost:8080/health")`: construye una petición GET. reqwest sigue el patrón Builder: `get()` devuelve un `RequestBuilder` al que puedes añadir headers, cuerpo, query params, etc.

- `.send().await`: envía la petición y espera la respuesta. `send()` consume el builder y devuelve un `Result<Response>`. El `await` es necesario porque la petición es asíncrona: el hilo se libera mientras espera la respuesta.

- `.expect("Error al conectar...")`: si `send()` falla (por ejemplo, porque el servidor no está corriendo), el mensaje de error ayuda a diagnosticar el problema. Si usas `unwrap()` en su lugar, el mensaje de error es menos descriptivo.

- `respuesta.status().is_success()`: devuelve `true` si el código de estado HTTP está entre 200 y 299.

- `respuesta.text().await.unwrap()`: obtiene el cuerpo de la respuesta como un String. También puedes usar `.json::<T>()` para deserializar directamente a un struct.

### 2.3 Test de autenticacion JWT

Muchos endpoints del ERP requieren un token JWT. El test de autenticación verifica que el login funciona y que los endpoints protegidos rechazan peticiones sin token.

```rust
// tests/api_erp/auth_test.rs
use reqwest::Client;
use serde_json::{json, Value};

#[tokio::test]
async fn test_login_devuelve_token() {
    let client = Client::new();

    // Enviar credenciales al endpoint de login
    let respuesta = client
        .post("http://localhost:8080/api/auth/login")
        .json(&json!({
            "username": "admin",
            "password": "admin123"
        }))
        .send()
        .await
        .unwrap();

    // Verificar que el login fue exitoso
    assert_eq!(respuesta.status(), 200);

    // Parsear la respuesta como JSON genérico
    let body: Value = respuesta.json().await.unwrap();

    // Verificar que el campo "token" existe
    assert!(body.get("token").is_some(),
        "La respuesta debe contener un campo 'token'");
}

#[tokio::test]
async fn test_login_credenciales_invalidas_devuelve_401() {
    let client = Client::new();

    let respuesta = client
        .post("http://localhost:8080/api/auth/login")
        .json(&json!({
            "username": "usuario_inexistente",
            "password": "clave_incorrecta"
        }))
        .send()
        .await
        .unwrap();

    // Con credenciales inválidas, debe devolver 401 Unauthorized
    assert_eq!(respuesta.status(), 401);
}
```

**Análisis línea por línea**:

- `.post("...")`: construye una petición POST. reqwest soporta GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS.

- `.json(&json!({...}))`: establece el cuerpo de la petición como JSON. `json!()` es una macro de serde_json que construye un `Value`. reqwest lo serializa y añade `Content-Type: application/json`.

- `respuesta.json::<Value>().await.unwrap()`: deserializa el cuerpo de la respuesta como `serde_json::Value`. `Value` es un enum que puede representar cualquier JSON: objeto, array, string, número, booleano o null. Es útil cuando no sabes la estructura exacta de la respuesta.

- `body.get("token").is_some()`: `get()` devuelve `Option<&Value>`. Si la clave "token" existe en el JSON, devuelve `Some`. Si no existe, devuelve `None`.

### 2.4 Test de CRUD completo

El test más valioso es el que prueba un flujo completo: login, crear un recurso, verificarlo, actualizarlo y eliminarlo.

```rust
// Helper: función auxiliar para obtener el token
async fn obtener_token(client: &Client) -> String {
    let resp = client
        .post("http://localhost:8080/api/auth/login")
        .json(&json!({"username": "admin", "password": "admin123"}))
        .send().await.unwrap();
    let body: Value = resp.json().await.unwrap();
    body["token"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn test_crud_completo_cliente() {
    let client = Client::new();
    let token = obtener_token(&client).await;

    // 1. CREAR cliente
    let crear = client
        .post("http://localhost:8080/api/clientes")
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "nombre": "Test Integración",
            "rfc": "TST010101XXX",
            "email": "test@integracion.mx",
            "credito": 100000.0
        }))
        .send().await.unwrap();

    assert_eq!(crear.status(), 201, "Crear cliente debe devolver 201 Created");
    let cliente_creado: Value = crear.json().await.unwrap();
    let cliente_id = cliente_creado["id"].as_u64()
        .expect("La respuesta debe contener un 'id' numérico");

    // 2. OBTENER cliente por ID
    let obtener = client
        .get(&format!("http://localhost:8080/api/clientes/{}", cliente_id))
        .header("Authorization", format!("Bearer {}", token))
        .send().await.unwrap();

    assert_eq!(obtener.status(), 200);
    let cliente: Value = obtener.json().await.unwrap();
    assert_eq!(cliente["nombre"], "Test Integración",
        "El nombre del cliente debe coincidir");

    // 3. ELIMINAR cliente
    let eliminar = client
        .delete(&format!("http://localhost:8080/api/clientes/{}", cliente_id))
        .header("Authorization", format!("Bearer {}", token))
        .send().await.unwrap();

    assert_eq!(eliminar.status(), 204,
        "Eliminar cliente debe devolver 204 No Content");
}
```

**Análisis línea por línea**:

- `async fn obtener_token(client: &Client) -> String {`: función auxiliar que encapsula la lógica de login. Todos los tests que necesiten autenticación pueden llamarla.

- `.header("Authorization", format!("Bearer {}", token))`: añade un header HTTP. El formato `Bearer <token>` es el estándar para JWT.

- `.as_u64().expect(...)`: `body["id"]` devuelve un `&Value`. Para convertirlo a `u64`, necesitas `.as_u64()`. Si el campo no es un número, devuelve `None` y el `expect()` da un mensaje claro.

- `format!("http://localhost:8080/api/clientes/{}", cliente_id)`: construye la URL dinámicamente. Es importante formatear bien la URL para evitar errores 404.

### 2.5 Errores tipicos

**Error 1: servidor no corriendo**.

```rust
let resp = client.get("http://localhost:8080/health").send().await.unwrap();
```

Mensaje: `called `Result::unwrap()` on an `Err` value: reqwest::Error { kind: Connect, message: "connection refused" }`. Solución: iniciar el servidor antes de ejecutar los tests, o usar `actix_web::test::init_service()` para levantar el servidor dentro del test.

**Error 2: timeout por defecto**.

Si la API tarda más de 30 segundos en responder (por ejemplo, porque la BD está lenta), reqwest lanza timeout. Solución: configurar un timeout más largo:

```rust
let client = Client::builder()
    .timeout(Duration::from_secs(60))
    .build().unwrap();
```

**Error 3: olvidar el header de autenticación**.

Si llamas a un endpoint protegido sin token, obtienes 401. El mensaje de error puede ser confuso si esperabas 200. Solución: siempre verificar el código de estado antes de procesar el cuerpo.

## 3. Test de la API de inventarios (Axum)

La API de inventarios usa Axum y no requiere autenticación para los endpoints públicos.

```rust
use reqwest::Client;

#[tokio::test]
async fn test_listar_productos_devuelve_array() {
    let client = Client::new();
    let resp = client
        .get("http://localhost:8081/api/productos")
        .send().await.unwrap();

    assert_eq!(resp.status(), 200);
    let productos = resp.json::<Vec<serde_json::Value>>().await.unwrap();
    assert!(productos.len() > 0, "Debe haber al menos un producto en la BD");
}

#[tokio::test]
async fn test_consultar_stock_por_producto() {
    let client = Client::new();
    let resp = client
        .get("http://localhost:8081/api/stock?producto=LAP-001")
        .send().await.unwrap();

    assert_eq!(resp.status(), 200);
    let stock = resp.json::<Vec<serde_json::Value>>().await.unwrap();
    assert!(!stock.is_empty(), "LAP-001 debería tener stock en algún almacén");
}
```

## 4. Test de la API de tickets (Rocket)

```rust
use reqwest::Client;

#[tokio::test]
async fn test_crear_ticket_devuelve_201() {
    let client = Client::new();
    let resp = client
        .post("http://localhost:8000/api/tickets")
        .json(&serde_json::json!({
            "titulo": "Fallo en Selenium",
            "descripcion": "Test automatizado desde Rust",
            "cliente_id": 1,
            "prioridad": 3
        }))
        .send().await.unwrap();

    assert!(resp.status().is_success());
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["estado"], "Abierto",
        "Todo ticket nuevo debe empezar en estado 'Abierto'");
}
```

## 5. Base de datos de prueba

Para tests que necesitan base de datos, `sqlx::test` crea una base de datos temporal y ejecuta las migraciones automáticamente.

```rust
use sqlx::MySqlPool;

#[sqlx::test]
async fn test_crear_producto_en_bd(pool: MySqlPool) {
    sqlx::query("INSERT INTO productos (sku, nombre, precio) VALUES (?, ?, ?)")
        .bind("TEST-001")
        .bind("Producto de prueba")
        .bind(100.0)
        .execute(&pool)
        .await
        .unwrap();

    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM productos WHERE sku = ?"
    )
    .bind("TEST-001")
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(count.0, 1, "Debe haber exactamente 1 producto con ese SKU");
}
```

## 6. Ejercicios

1. Escribir un test que verifique que `POST /api/productos` sin nombre devuelve 422.
2. Implementar un test de transacción: crear un pedido y verificar que el stock se descuenta.
3. Agregar un test de carga con 50 peticiones concurrentes a `/health`.
4. Escribir un test que verifique que la API de tickets rechaza tickets sin `cliente_id`.
5. Implementar fixtures con `sqlx::test` para poblar la BD antes de cada test.

## 7. Soluciones

Las soluciones detalladas están en `proyectos_testing/tests/` con los tests completos.
