# Manual de Rocket + Arquitectura Granular para Sistema de Tickets

> **Construye un sistema de tickets de soporte con Rocket (Rust) siguiendo una arquitectura granular: clientes, agentes, tickets, SLAs, reportes.**

---

**Stack tecnológico**: Rocket 0.5 · Diesel 2.2 · MariaDB/MySQL · rock_db_pools · JWT

---

## 1. Introduccion a Rocket

Cada framework web tiene una filosofía diferente. Actix Web prioriza la velocidad y usa un modelo de actores. Axum prioriza la composición y usa Tower como base. Rocket prioriza algo distinto: la **experiencia del desarrollador**. Rocket está diseñado para que escribir código web se sienta como escribir código Rust normal, sin la sensación de estar "llamando a un framework". Esto se logra mediante un uso intensivo de macros de Rust: `#[get("/")]`, `#[post("/ruta")]`, `#[launch]`, `routes![]`. Donde otros frameworks te piden construir objetos, Rocket te pide escribir atributos.

La filosofía de Rocket se puede resumir en tres principios. El primero: **los atributos de función son mejores que los builders**. En Actix Web, registras rutas con `App::new().route("/", web::get().to(handler))`. En Axum, con `Router::new().route("/", get(handler))`. En Rocket, escribes `#[get("/")]` encima de la función y ya está. No hay Router, no hay builder, no hay método `.route()`. El compilador de Rust genera todo el registro de rutas automáticamente a partir de los atributos. Esto hace que el código sea más legible: la ruta está justo encima del handler, no en otra parte del archivo.

El segundo principio: **los tipos son los mejores validadores**. Rocket extrae datos de las peticiones HTTP mediante **guards**, que son tipos que implementan `FromRequest`. Cuando declaras un parámetro como `id: u32` en una ruta `#[get("/tickets/<id>")]`, Rocket valida que el parámetro exista y que sea un número. Cuando declaras `input: Json<CrearTicketInput>`, Rocket valida que el cuerpo sea JSON válido y que tenga todos los campos necesarios. Si alguna validación falla, Rocket devuelve automáticamente un error HTTP sin ejecutar el handler. El handler nunca recibe datos inválidos.

El tercer principio: **las respuestas son datos, no construcciones**. Un handler de Rocket devuelve un valor, y Rocket lo convierte en una respuesta HTTP. Si devuelves `String`, Rocket lo convierte en 200 OK con `text/plain`. Si devuelves `Json<Cliente>`, Rocket lo convierte en 200 OK con `application/json`. Si devuelves `(StatusCode, Json<Value>)`, Rocket usa el código de estado que le diste. No necesitas llamar a `HttpResponse::Ok().json(...)` como en Actix Web, ni construir una tupla `(StatusCode, Json<T>)` como en Axum. Solo devuelves el dato.

¿Por qué Rocket para un sistema de tickets de soporte? Porque un sistema de tickets tiene muchas rutas con diferentes métodos de entrada: formularios para crear tickets, JSON para APIs REST, parámetros de ruta para identificar tickets, parámetros de query para filtrar. La filosofía de Rocket de "escribe la ruta como atributo, define los parámetros con tipos, devuelve datos" hace que los handlers sean cortos, legibles y fáciles de mantener.

### 1.1 El ciclo de vida de una peticion en Rocket

Cuando una petición HTTP llega al servidor Rocket, ocurre lo siguiente:

1. Rocket recibe la conexión TCP y parsea la petición HTTP.
2. Rocket ejecuta los **fairings** de petición (middleware que se ejecuta antes del handler).
3. Rocket busca una ruta que coincida con el método y la URL.
4. Si encuentra la ruta, Rocket intenta ejecutar los **guards** de los parámetros del handler: primero extrae los parámetros de ruta (`<id>`), luego los de query (`?<filtros..>`), luego el cuerpo (`data = "<input>"`).
5. Si todos los guards tienen éxito, Rocket ejecuta el handler.
6. Rocket toma el valor devuelto por el handler y lo convierte en una respuesta HTTP usando el trait `Responder`.
7. Rocket ejecuta los fairings de respuesta y envía la respuesta al cliente.

Si en cualquier paso ocurre un error (ruta no encontrada → 404, guard falla → 422, error interno → 500), Rocket devuelve el error correspondiente sin ejecutar los pasos siguientes.

### 1.2 ¿Que construiremos?

Construiremos un sistema de tickets de soporte técnico con estas capacidades:

- **Clientes**: empresas o personas que reportan incidentes. Cada cliente tiene nombre, email, empresa.
- **Agentes**: personal de soporte que resuelve tickets. Cada agente tiene nombre, email, especialidad y disponibilidad.
- **Tickets**: el núcleo del sistema. Cada ticket pasa por estados: Abierto → En Progreso → Resuelto → Cerrado. Puede cancelarse o escalarse.
- **SLAs**: acuerdos de nivel de servicio. Definen tiempos máximos de respuesta y resolución según la prioridad.
- **Reportes**: tickets abiertos, tiempo promedio de resolución, SLAs incumplidos.

Cada funcionalidad será un **módulo granular**: autocontenido, con sus propios tipos, rutas y acceso a datos.

## 2. Configuracion del proyecto

Rocket tiene una forma particular de organizar el código fuente. No necesitas crear un `Router` ni llamar a métodos de configuración de rutas. En lugar de eso, usas atributos de Rust para marcar las funciones como handlers, y luego las "montas" en la aplicación con la macro `routes![]`. Esta macro genera en tiempo de compilación todo el código necesario para que Rocket pueda enrutar las peticiones.

La macro `mount` recibe un prefijo de ruta y una lista de rutas generada por `routes![]`. Por ejemplo, `mount("/api", routes![handler1, handler2])` monta los handlers bajo el prefijo `/api`. Esto permite organizar la API en secciones: `/api/clientes`, `/api/tickets`, `/api/reportes`.

### 2.1 Dependencias en Cargo.toml

```toml
[package]
name = "rocket_health"
version = "0.1.0"
edition = "2021"

[dependencies]
rocket = { version = "0.5", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

**Análisis línea por línea**:

- `rocket = { version = "0.5", features = ["json"] }`: el framework Rocket. La version 0.5 es la estable actual. La feature `json` habilita el modulo `rocket::serde::json` que incluye `Json<T>` tanto para recibir como para devolver JSON. Sin esta feature, no puedes usar `#[post("/ruta", data = "<input>")]` con `Json<T>`.

- `serde = { version = "1", features = ["derive"] }`: serialización. Necesaria para que `Json<T>` pueda deserializar el JSON de entrada y serializar el JSON de salida.

- `serde_json = "1"`: el backend JSON. Rocket usa serde_json internamente cuando serializa respuestas `Json<T>`.

### 2.2 Primer servidor HTTP

El primer servidor tendra dos rutas: `/health` que devuelve "OK" y `/` que devuelve información del servicio en JSON.

```rust
// archivo: src/main.rs
// Primer servidor con Rocket
// Endpoints: / (info), /health (health check)

#[macro_use] extern crate rocket;
use serde_json::{json, Value};
use rocket::serde::json::Json;

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

#[get("/")]
fn info() -> Json<Value> {
    Json(json!({
        "servicio": "Sistema de Tickets Rocket",
        "version": "0.1.0",
        "estado": "activo"
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![health, info])
}
```

**Análisis línea por línea**:

- `#[macro_use] extern crate rocket;`: esta línea importa todas las macros de Rocket (`get`, `launch`, `routes`, etc.) al alcance global. Sin ella, no podrías usar `#[get("/")]` ni `routes![]`. En Rust moderno (edición 2021) se puede usar `use rocket::{get, launch, routes};` en lugar de `#[macro_use]`, pero esta forma es la tradicional en la documentación de Rocket y la más usada en proyectos existentes.

- `use rocket::serde::json::Json;`: importa el tipo `Json<T>` de Rocket. No es el mismo `Json` de serde o Axum: es el wrapper específico de Rocket que implementa `Responder` (para respuestas) y `FromData` (para recibir datos). Ambos son posibles gracias a que `Json<T>` implementa tanto `Responder` como `FromData`.

- `#[get("/health")]`: este es un **atributo de función** (function attribute). Le dice al compilador: "esta función es un handler para peticiones GET a la ruta /health". Rocket procesa este atributo en tiempo de compilación y genera el código necesario para registrar la ruta. El nombre de la función (`health`) no importa para el enrutamiento: lo que importa es el atributo.

- `fn health() -> &'static str { "OK" }`: handler que devuelve un string estático. `&'static str` implementa `Responder`, así que Rocket lo convierte automáticamente en una respuesta HTTP 200 OK con `Content-Type: text/plain` y el cuerpo "OK". No necesitas crear un objeto `Response`: solo devuelves el string y Rocket hace el resto.

- `#[get("/")]`: ruta para la raíz del servicio. El path es `/`.

- `fn info() -> Json<Value> { Json(json!({...})) }`: handler que devuelve un objeto JSON. `Json<Value>` implementa `Responder`, así que Rocket serializa el `Value` a JSON y lo devuelve con `Content-Type: application/json`.

- `Json(json!({"servicio": "...", ...}))`: `json!` es una macro de serde_json que construye un `Value` a partir de sintaxis JSON literal. El resultado se envuelve en `Json()` para indicar que es una respuesta JSON.

- `#[launch]`: atributo que marca esta función como el punto de entrada del programa. La función debe llamarse `rocket` y debe devolver `Rocket<Build>`. `#[launch]` genera automáticamente la función `main` que inicializa el runtime Tokio y ejecuta Rocket.

- `fn rocket() -> _ {`: el `_` en el tipo de retorno le dice al compilador "infiere el tipo por mí". El tipo real es `Rocket<Build>` (el tipo retornado por `rocket::build()`).

- `rocket::build()`: construye una instancia de Rocket en estado de configuración. A esta instancia se le pueden añadir rutas, fairings, estado, etc.

- `.mount("/", routes![health, info])`: monta las rutas en el prefijo `/`. `routes![]` es una macro que toma los nombres de las funciones handler y genera el código de registro. El prefijo `/` significa que las rutas serán `/` y `/health`. Si montaras con prefijo `/api`, las rutas serían `/api/` y `/api/health`.

**Salida esperada**:

Al ejecutar `cargo run`:

```
🔧 Configured for debug.
   >> address: 127.0.0.1
   >> port: 8000
   >> workers: 8
   >> ...
🛰  Mounting /:
   >> GET /health
   >> GET /
🚀 Rocket has launched on http://127.0.0.1:8000
```

```bash
curl http://127.0.0.1:8000/health
# OK

curl http://127.0.0.1:8000/
# {"servicio":"Sistema de Tickets Rocket","version":"0.1.0","estado":"activo"}
```

### 2.3 Errores tipicos

**Error 1: olvidar importar las macros**.

```rust
// CODIGO QUE FALLA
#[get("/health")]
fn health() -> &'static str { "OK" }
```

Mensaje:
```
error: cannot find attribute 'get' in this scope
```

Solución: añadir `#[macro_use] extern crate rocket;` al inicio del archivo, o `use rocket::get;`.

**Error 2: olvidar `routes![]` en mount**.

```rust
// CODIGO QUE FALLA
rocket::build().mount("/", health)
```

Mensaje:
```
error: expected a function, found `health`
```

Solución: `rocket::build().mount("/", routes![health])`.

**Error 3: funcion rocket con tipo incorrecto**.

```rust
// CODIGO QUE FALLA
#[launch]
fn rocket() -> Rocket<Build> {
    // ...
}
```

Mensaje: `error[E0433]: failed to resolve: use of undeclared type 'Rocket'`. Solucion: usar `_` como tipo de retorno y dejar que el compilador lo infiera, o importar `use rocket::Rocket;`.

### 2.4 Mini-proyecto: `01_rocket_health`

El proyecto `01_rocket_health` (en `proyectos_capitulo/parte5_rocket/01_rocket_health/`) contiene este servidor.

```bash
cd proyectos_capitulo/parte5_rocket/01_rocket_health
cargo run
# En otra terminal:
curl http://127.0.0.1:8000/health
```

## 3. Rutas, guards y datos

En un sistema de tickets de soporte, cada operacion se traduce en una peticion HTTP. Cuando un cliente reporta un incidente, el sistema frontal envia un POST a `/api/tickets` con los datos del ticket. Cuando un agente busca los tickets asignados, envia un GET a `/api/tickets?asignado_a=5`. Cuando un supervisor cambia el estado de un ticket, envia un PUT a `/api/tickets/42/estado` con el nuevo estado.

En Rocket, cada una de estas operaciones se implementa con tres elementos: un **atributo de ruta** que define el metodo y la URL, unos **guards** que extraen los datos de la peticion, y un tipo de **respuesta** que Rocket convierte en HTTP.

Los guards son el concepto central. Un guard es un tipo que implementa el trait `FromRequest`. Cuando declaras un parametro como `id: u32` en un handler con `#[get("/tickets/<id>")]`, Rocket crea un guard `Path<u32>` internamente. Cuando declaras `input: Json<CrearTicketInput>`, Rocket crea un guard `Json<T>`. La diferencia con Axum es que en Rocket los guards no se declaran con tipos anidados (`Path(id): Path<u32>`) sino con tipos directos (`id: u32`). Rocket infiere automaticamente que tipo de guard usar segun el nombre y el contexto del parametro.

¿Por que Rocket puede hacer esto y Axum no? Porque Rocket usa macros de atributos (`#[get("/tickets/<id>")]`) que pueden inspeccionar los nombres de los parametros de la funcion y generar el codigo de extraccion correspondiente. Axum, al no usar macros de atributos, necesita que especifiques explicitamente el extractor. Esta es la principal ventaja de Rocket sobre otros frameworks: menos codigo boilerplate, mas legibilidad.

### 3.1 Parametros de ruta con `<param>`

Los parametros de ruta se usan para identificar recursos especificos: `/tickets/42` identifica el ticket con ID 42. En Rocket, los parametros de ruta se escriben entre `<>` en el path y se reciben como parametros con nombre en el handler.

```rust
// Obtener un ticket por su ID
#[get("/tickets/<id>")]
fn obtener_ticket(id: u32) -> String {
    format!("Ticket ID: {}", id)
}

// Accion sobre un ticket (ej: /tickets/42/asignar)
#[get("/tickets/<id>/<accion>")]
fn accion_ticket(id: u32, accion: String) -> String {
    format!("Ticket {} accion: {}", id, accion)
}
```

**Analisis linea por linea**:

- `#[get("/tickets/<id>")]`: el atributo define la ruta. `<id>` es un marcador de posicion. Rocket extrae el valor de la URL en esa posicion y lo pasa al parametro `id` del handler. Si la URL es `/tickets/42`, el valor es `42`. Si la URL es `/tickets/abc`, Rocket intenta convertir "abc" a `u32`, falla, y devuelve 422 Unprocessable Entity automaticamente, sin ejecutar el handler.

- `fn obtener_ticket(id: u32) -> String`: el handler recibe `id` como `u32`. No necesitas escribir `Path(id): Path<u32>` como en Axum: Rocket infiere el guard a partir del nombre del parametro y el contexto de la ruta. `String` como retorno implementa `Responder`, asi que Rocket lo convierte en 200 OK con `Content-Type: text/plain`.

- `#[get("/tickets/<id>/<accion>")]`: multiples parametros en la misma ruta. Rocket extrae cada segmento en orden: el primer `<id>` se asigna al primer parametro `id`, el segundo `<accion>` se asigna al segundo parametro `accion`. El orden debe coincidir.

- `fn accion_ticket(id: u32, accion: String) -> String`: el handler recibe dos parametros. `id` se convierte a `u32` (si no puede, 422). `accion` es `String` (siempre funciona porque cualquier segmento de URL es un string).

### 3.2 Parametros de query con `FromForm`

Los parametros de query se usan para filtros, busqueda y paginacion: `/tickets?estado=Abierto&prioridad=3`. No identifican un recurso, sino que modifican la consulta. En Rocket, los parametros de query se capturan con `FromForm`, que es un trait que permite deserializar query strings en structs.

```rust
use rocket::form::FromForm;

#[derive(FromForm)]
struct TicketFiltros {
    estado: Option<String>,
    prioridad: Option<u8>,
    asignado_a: Option<u32>,
}

#[get("/tickets?<filtros..>")]
fn listar_tickets(filtros: TicketFiltros) -> String {
    let estado = filtros.estado.unwrap_or_else(|| "todos".into());
    format!("Filtrando tickets: estado={}", estado)
}
```

**Analisis linea por linea**:

- `#[derive(FromForm)]`: este trait le dice a Rocket como deserializar los parametros de query string en el struct. Cada campo del struct se mapea a un parametro de query por nombre. `estado` se mapea a `?estado=...`, `prioridad` se mapea a `?prioridad=...`.

- `estado: Option<String>`: los campos deben ser `Option` si el parametro es opcional. Si el cliente no envia `?estado=...`, el campo vale `None` y el handler se ejecuta normalmente. Si el campo NO es `Option` y el parametro no se envia, Rocket devuelve 422 Bad Request.

- `prioridad: Option<u8>`: Rocket convierte el string del query a `u8`. Si el cliente envia `?prioridad=abc`, Rocket devuelve 422 porque "abc" no es un numero valido.

- `#[get("/tickets?<filtros..>")]`: el `<filtros..>` con dos puntos indica "capturar TODOS los parametros de query y deserializarlos en el struct `TicketFiltros`". Sin el doble punto (`<filtros>`), Rocket esperaria un solo parametro llamado `filtros` en la URL.

- `let estado = filtros.estado.unwrap_or_else(|| "todos".into())`: proporciona un valor por defecto cuando el parametro no se envio. Esto es mejor que usar `unwrap()` porque no entra en panico: si no hay filtro, se usa "todos".

### 3.3 Recepcion de JSON con `Json<T>`

Cuando el cliente crea o actualiza un recurso, envia un JSON en el cuerpo de la peticion. Rocket extrae ese JSON con el guard `Json<T>`, que implementa `FromData` (el trait para extraer datos del cuerpo de la peticion).

```rust
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct CrearTicketInput {
    titulo: String,
    descripcion: String,
    cliente_id: u32,
    prioridad: u8,
}

#[post("/tickets", data = "<input>")]
fn crear_ticket(input: Json<CrearTicketInput>) -> Json<Value> {
    Json(json!({
        "mensaje": format!("Ticket '{}' creado", input.titulo)
    }))
}
```

**Analisis linea por linea**:

- `#[derive(Deserialize)] struct CrearTicketInput { ... }`: define el formato del JSON esperado. Cada campo se mapea a una clave del JSON. Si el JSON tiene claves extra, se ignoran. Si falta un campo obligatorio (`titulo`, `descripcion`, `cliente_id`, `prioridad`), Rocket devuelve 422.

- `#[post("/tickets", data = "<input>")]`: el argumento `data = "<input>"` le dice a Rocket: "el cuerpo de la peticion se asigna al parametro `input`". El nombre entre comillas debe coincidir con el nombre del parametro en la funcion. Sin `data = "..."`, Rocket no sabria de donde sacar los datos del parametro y el compilador daria un error.

- `input: Json<CrearTicketInput>`: el guard `Json<T>` implementa `FromData`. Rocket lee el cuerpo de la peticion, lo parsea como JSON, y deserializa los campos en `CrearTicketInput`. Si el cuerpo no es JSON valido, Rocket devuelve 422.

- `Json(json!({"mensaje": ...}))`: devuelve una respuesta JSON. `Json<Value>` implementa `Responder`, asi que Rocket serializa el `Value` a JSON y lo envia con `Content-Type: application/json`. El codigo de estado por defecto es 200 OK; para 201 Created necesitas una tupla.

**Respuesta 201 Created**:

```rust
#[post("/tickets", data = "<input>")]
fn crear_ticket(input: Json<CrearTicketInput>) -> (Status, Json<Value>) {
    (Status::Created, Json(json!({"id": 1, "mensaje": "Ticket creado"})))
}
```

### 3.4 Errores tipicos con rutas y guards

**Error 1: olvidar `data = "<param>"` en POST**.

```rust
// CODIGO QUE FALLA
#[post("/tickets")]
fn crear(input: Json<CrearTicketInput>) -> Json<Value> { ... }
```

Mensaje:
```
error: parameter `input` has no source.
  --> src/main.rs
   |
   | fn crear(input: Json<CrearTicketInput>)
   |          ^^^^^ Rocket can only accept parameters with a source
```

Solucion: anadir `data = "<input>"` en el atributo.

**Error 2: FromForm sin `#[derive(FromForm)]`**.

```rust
// CODIGO QUE FALLA
struct TicketFiltros { estado: Option<String> }

#[get("/tickets?<f..>")]
fn listar(f: TicketFiltros) -> String { ... }
```

Mensaje:
```
error: the trait bound `TicketFiltros: FromForm` is not satisfied
```

Solucion: anadir `#[derive(FromForm)]` al struct.

**Error 3: devolver `Json<T>` sin Serialize**.

```rust
// CODIGO QUE FALLA
struct Ticket { id: i32 }

#[get("/tickets/<id>")]
fn obtener(id: u32) -> Json<Ticket> {
    Json(Ticket { id: id as i32 })
}
```

Mensaje: `error: the trait bound `Ticket: Serialize` is not satisfied`. Solucion: anadir `#[derive(Serialize)]` al struct.

## 4. Conexion a la base de datos con Diesel

En un sistema de tickets, necesitamos persistir clientes, agentes, tickets y SLAs. Usaremos Diesel como ORM (el mismo que en el manual principal), pero integrado con Rocket a traves de `rocket_db_pools`.

### 4.1 Dependencias

```toml
[dependencies]
diesel = { version = "2.2", features = ["mysql", "r2d2"] }
rocket_db_pools = { version = "0.2", features = ["diesel_mysql"] }
dotenvy = "0.15"
```

**Analisis**: `rocket_db_pools` proporciona el trait `Database` y el tipo `Connection<T>` que integra Diesel con Rocket. `diesel_mysql` habilita el backend MySQL para `rocket_db_pools`.

### 4.2 Esquema de la base de datos

```sql
CREATE DATABASE IF NOT EXISTS tickets_soporte CHARACTER SET utf8mb4;
USE tickets_soporte;

CREATE TABLE clientes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(150) NOT NULL,
    email VARCHAR(200) NOT NULL,
    empresa VARCHAR(200),
    activo BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tickets (
    id INT AUTO_INCREMENT PRIMARY KEY,
    titulo VARCHAR(200) NOT NULL,
    descripcion TEXT NOT NULL,
    estado VARCHAR(20) NOT NULL DEFAULT 'Abierto',
    prioridad TINYINT NOT NULL DEFAULT 3,
    cliente_id INT NOT NULL,
    agente_id INT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (cliente_id) REFERENCES clientes(id),
    FOREIGN KEY (agente_id) REFERENCES agentes(id)
);
```

### 4.3 Configuracion de Rocket.toml

```toml
[default.databases.tickets_db]
url = "mysql://root:secret@127.0.0.1:3306/tickets_soporte"
```

### 4.4 Conexion a la BD en Rocket

```rust
use rocket_db_pools::Database;

#[derive(Database)]
#[database("tickets_db")]
pub struct TicketsDb(diesel::MysqlConnection);

#[get("/tickets")]
async fn listar_tickets(mut db: Connection<TicketsDb>) -> Json<Value> {
    let resultados = diesel::sql_query("SELECT id, titulo FROM tickets")
        .load::<(i32, String)>(&mut *db)
        .unwrap_or_default();
    Json(json!({"tickets": resultados}))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(TicketsDb::init())
        .mount("/", routes![listar_tickets])
}
```

**Analisis linea por linea**:

- `#[derive(Database)]`: genera el codigo necesario para que Rocket gestione el pool de conexiones. El pool se crea al iniciar el servidor y se destruye al cerrarlo.

- `#[database("tickets_db")]`: el nombre debe coincidir con la clave en `Rocket.toml`. Si no coinciden, Rocket falla al iniciar.

- `pub struct TicketsDb(diesel::MysqlConnection)`: el tipo de conexion. `diesel::MysqlConnection` es la conexion sincrona de Diesel (no bloquea el hilo de Rocket porque Diesel usa pools internos).

- `mut db: Connection<TicketsDb>`: obtiene una conexion del pool. El tipo `Connection<T>` es un guard de Rocket que extrae la conexion automaticamente.

- `diesel::sql_query("SELECT ...").load::<(i32, String)>(&mut *db)`: ejecuta SQL directo con Diesel. `&mut *db` desreferencia el wrapper `Connection` para obtener la conexion subyacente.

- `.attach(TicketsDb::init())`: registra la base de datos en Rocket. Sin esto, los handlers que usen `Connection<TicketsDb>` fallaran.

### 4.5 Errores tipicos con la base de datos

**Error 1: nombre de BD incorrecto en Rocket.toml**.

```toml
# Rocket.toml
[default.databases.bd_equivocada]
url = "mysql://root:secret@127.0.0.1:3306/tickets_soporte"
```

```rust
#[database("tickets_db")]  // ← no coincide con "bd_equivocada"
pub struct TicketsDb(diesel::MysqlConnection);
```

Mensaje al iniciar el servidor:
```
error: database 'tickets_db' is not configured.
  --> src/db.rs
   |
   | #[database("tickets_db")]
   |             ^^^^^^^^^^^ no matching database configuration found
```

Solucion: el nombre en `#[database("...")]` debe coincidir exactamente con la clave en `Rocket.toml`. En este caso, ambos deben ser `"tickets_db"`.

**Error 2: olvidar `.attach()`**.

```rust
// CODIGO QUE FALLA: falta .attach(TicketsDb::init())
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![listar_tickets])
}
```

No hay error de compilacion, pero al recibir la primera peticion que usa `Connection<TicketsDb>`:
```
thread 'rocket-worker-thread' panicked at 'called `Result::unwrap()` on an `Err` value: No database pool configured'
```

Solucion: llamar a `.attach(TicketsDb::init())` antes de montar las rutas.

**Error 3: olvidar ejecutar migraciones Diesel**.

Si la tabla `tickets` no existe en la BD, cualquier consulta falla con:
```
Error: Table 'tickets_soporte.tickets' doesn't exist
```

Solucion: ejecutar `diesel migration run` o cargar el `init.sql` manualmente.

### 4.6 Mini-proyecto: `02_rocket_crud_diesel`

El proyecto `02_rocket_crud_diesel` (en `proyectos_capitulo/parte5_rocket/02_rocket_crud_diesel/`) implementa un CRUD completo de clientes con Diesel y Rocket. Incluye las migraciones iniciales y un endpoint para cada operacion.

## 5. Arquitectura Granular

La arquitectura granular es una forma de organizar el codigo donde cada funcionalidad del negocio es un **modulo autocontenido**. Un modulo contiene todo lo que necesita: sus propios tipos (structs, enums), sus propias rutas (handlers), su propio acceso a datos (consultas Diesel), y su propia logica de negocio. No hay una carpeta central `repositories/` donde todos los modulos comparten el acceso a datos. No hay una carpeta central `handlers/` donde todas las rutas estan mezcladas. Cada modulo es una isla.

¿Que diferencia hay entre arquitectura granular y arquitectura hexagonal? La granularidad organiza el codigo por **responsabilidad de negocio** (clientes, tickets, reportes). La hexagonal organiza el codigo por **capa tecnica** (dominio, puertos, adaptadores). En un sistema de tickets, las reglas de negocio son relativamente simples: transiciones de estado, calculo de SLAs, asignacion de agentes. No necesitas la separacion rigurosa que impone hexagonal. En cambio, necesitas que cada funcionalidad sea independiente para que varios desarrolladores puedan trabajar en paralelo sin pisarse el codigo.

La granularidad tambien facilita las pruebas. Puedes probar el modulo de tickets sin necesidad de levantar el modulo de clientes, porque cada modulo tiene su propio acceso a datos y no depende de los otros modulos. Si dos modulos necesitan comunicarse, lo hacen a traves de tipos compartidos definidos en un modulo comun (como `errores.rs` o `db.rs`), no a traves de dependencias directas entre modulos.

### 5.1 Estructura de directorios

La estructura del proyecto refleja la granularidad. Cada carpeta dentro de `src/` es un modulo independiente:

```
src/
├── main.rs              ← Punto de entrada, monta todos los modulos
├── lib.rs               ← Reexportaciones y tipos compartidos
├── db.rs                ← TicketsDb (Database derive + pool)
├── schema.rs            ← Diesel schema (generado automaticamente)
├── errores.rs           ← Error personalizado con Responder
├── clientes/            ← MODULO: gestion de clientes
│   ├── mod.rs           ← rutas + funcion mount + reexportaciones
│   ├── modelo.rs        ← structs + traits Queryable/Insertable
│   └── repositorio.rs   ← consultas BD (listar, crear, etc.)
├── tickets/             ← MODULO: nucleo del sistema
│   ├── mod.rs
│   ├── modelo.rs
│   ├── repositorio.rs
│   └── maquina_estados.rs
├── agentes/
│   ├── mod.rs
│   ├── modelo.rs
│   └── repositorio.rs
└── reportes/
    ├── mod.rs
    └── queries.rs
```

**Analisis de la estructura**:

- `clientes/mod.rs`: exporta la funcion `mount()` y las rutas. No exporta los tipos internos como `Cliente` o `NuevoCliente` a menos que otros modulos los necesiten. Por defecto, todo es privado.

- `clientes/modelo.rs`: define los structs que se mapean a las tablas de la BD. Usa `#[derive(Queryable, Insertable, Serialize)]` de Diesel y Serde.

- `clientes/repositorio.rs`: implementa las consultas SQL usando Diesel. Cada funcion recibe una conexion y devuelve resultados. No hay estado compartido ni dependencias externas.

- `tickets/mod.rs`: similar estructura, pero ademas incluye `maquina_estados.rs` que define la logica de transiciones de estados. Esta logica esta separada del repositorio porque es logica de negocio, no acceso a datos.

### 5.2 Cada modulo expone su funcion de montaje

Para que los modulos sean verdaderamente independientes, cada uno expone una funcion `mount()` que recibe la instancia de Rocket y devuelve la instancia modificada con sus rutas montadas. La funcion `main` solo se encarga de llamar a estos mounts en orden.

```rust
// ========== clientes/mod.rs ==========
use rocket::{Rocket, Build, routes};

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/api/clientes", routes![
            listar, crear, obtener, actualizar, eliminar
        ])
}

// ========== tickets/mod.rs ==========
pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/api/tickets", routes![
            listar, crear, obtener, cambiar_estado
        ])
}

// ========== main.rs ==========
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(TicketsDb::init())
        .attach(clientes::mount)   // monta /api/clientes/*
        .attach(tickets::mount)    // monta /api/tickets/*
        .attach(agentes::mount)    // monta /api/agentes/*
        .attach(reportes::mount)   // monta /api/reportes/*
}
```

**Analisis linea por linea**:

- `pub fn mount(rocket: Rocket<Build>) -> Rocket<Build>`: firma de la funcion de montaje. Recibe una instancia de Rocket en estado de construccion y devuelve la misma instancia modificada. Esta firma es la que espera `.attach()`.

- `.attach(clientes::mount)`: `attach()` recibe una funcion o un fairing. En este caso, recibe una funcion con la firma correcta y la ejecuta, pasandole la instancia actual de Rocket y recibiendo la instancia modificada.

- `.mount("/api/clientes", routes![listar, crear, ...])`: monta las rutas bajo el prefijo `/api/clientes`. Las rutas definidas en `clientes/mod.rs` como `#[get("/")]` o `#[get("/<id>")]` se convierten en `/api/clientes/` y `/api/clientes/<id>`.

**Ventaja del patron attach**: puedes anadir o quitar modulos sin modificar el codigo de otros modulos. Si quieres deshabilitar temporalmente el modulo de agentes, solo comentas la linea `.attach(agentes::mount)` en `main.rs`. Ningun otro modulo se ve afectado.

### 5.3 Comunicacion entre modulos

Cuando un modulo necesita datos de otro (por ejemplo, tickets necesita el nombre del cliente), no debe acceder directamente al repositorio de clientes. En lugar de eso, hay dos opciones:

1. **Compartir tipos en db.rs o lib.rs**: define los structs basicos en un modulo comun y que cada modulo los use.
2. **Inyectar dependencias en el mount**: pasar la conexion a la BD como parametro (pero esto rompe la independencia).

La opcion recomendada es la 1: definir los structs compartidos en `lib.rs` o `db.rs` y que cada modulo los importe. Como todos los modulos usan la misma BD, pueden hacer JOINs en las consultas sin problemas.

## 6. Modulo Tickets con maquina de estados

El modulo Tickets es el corazon del sistema. Los tickets tienen un ciclo de vida definido por estados: `Abierto -> EnProgreso -> Resuelto -> Cerrado`. Pueden cancelarse o escalarse en cualquier momento.

### 6.1 Maquina de estados

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EstadoTicket {
    Abierto,
    EnProgreso,
    Resuelto,
    Cerrado,
    Cancelado,
    Escalado,
}

impl EstadoTicket {
    pub fn transiciones_validas(&self) -> Vec<EstadoTicket> {
        match self {
            EstadoTicket::Abierto => vec![
                EstadoTicket::EnProgreso,
                EstadoTicket::Cancelado,
            ],
            EstadoTicket::EnProgreso => vec![
                EstadoTicket::Resuelto,
                EstadoTicket::Escalado,
                EstadoTicket::Cancelado,
            ],
            EstadoTicket::Resuelto => vec![
                EstadoTicket::Cerrado,
                EstadoTicket::EnProgreso, // reabrir
            ],
            _ => vec![], // Cerrado, Cancelado, Escalado son terminales
        }
    }

    pub fn transicion_valida(&self, nuevo: &EstadoTicket) -> Result<(), String> {
        if self.transiciones_validas().contains(nuevo) {
            Ok(())
        } else {
            Err(format!("No se puede pasar de {:?} a {:?}", self, nuevo))
        }
    }
}
```

**Analisis**: la maquina de estados define que transiciones son permitidas. `transiciones_validas()` devuelve los estados a los que se puede llegar desde el estado actual. `transicion_valida()` verifica si una transicion es permitida y devuelve un error si no lo es. Esta logica de negocio esta centralizada en un solo lugar.

### 6.2 Repositorio de tickets

```rust
use diesel::prelude::*;
use crate::tickets::modelo::{Ticket, NuevoTicket};
use crate::tickets::maquina_estados::EstadoTicket;

pub fn listar(conn: &mut MysqlConnection) -> Vec<Ticket> {
    tickets::table
        .order(tickets::id.desc())
        .limit(50)
        .load::<Ticket>(conn)
        .unwrap_or_default()
}

pub fn cambiar_estado(
    conn: &mut MysqlConnection,
    ticket_id: i32,
    estado_actual: &EstadoTicket,
    estado_nuevo: &EstadoTicket,
) -> Result<Ticket, String> {
    // Validar la transicion
    estado_actual.transicion_valida(estado_nuevo)?;

    // Ejecutar el cambio
    let afectadas = diesel::update(tickets::table.find(ticket_id))
        .set(tickets::estado.eq(format!("{:?}", estado_nuevo)))
        .execute(conn)
        .map_err(|e| format!("Error BD: {}", e))?;

    if afectadas == 0 {
        return Err("Ticket no encontrado".into());
    }

    tickets::table.find(ticket_id)
        .first::<Ticket>(conn)
        .map_err(|e| format!("Error BD: {}", e))
}
```

**Analisis**: el repositorio valida la transicion de estado ANTES de ejecutar el UPDATE. Si la transicion no es valida, la BD no se modifica. Esta es una aplicacion del principio "fail fast": fallar antes de hacer cambios irreversibles.

## 7. Proyecto final y ejercicios

El proyecto final integra todos los modulos. El codigo completo esta en `proyecto_api_rocket_tickets/`.

```rust
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(TicketsDb::init())
        .attach(clientes::mount)
        .attach(agentes::mount)
        .attach(tickets::mount)
        .attach(sla::mount)
        .attach(reportes::mount)
}
```

### Ejercicios

1. Implementar el modulo Agentes con CRUD completo y validacion de email unico.
2. Agregar un endpoint `POST /tickets/<id>/transicion` que reciba `{"nuevo_estado": "Resuelto"}` y valide la maquina de estados.
3. Implementar el calculo de SLA: al crear un ticket, calcular la fecha limite de respuesta segun la prioridad.
4. Agregar autenticacion JWT con Rocket guards (`#[guard]`).
5. Crear un reporte `GET /reportes/tickets-por-estado` que agrupe tickets por estado.
6. Implementar paginacion en el listado de tickets.

### Soluciones

Las soluciones detalladas estan en `proyecto_api_rocket_tickets/`.
