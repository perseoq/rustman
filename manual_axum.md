# Manual de Axum + Arquitectura Hexagonal para Sistema de Inventarios

> **Construye un sistema de gestión de inventarios multibodega con Axum (Tokio) siguiendo la arquitectura hexagonal: productos, almacenes, transferencias, lotes, alertas de stock.**

---

**Stack tecnológico**: Axum 0.8 · Tokio 1 · sqlx · MariaDB/MySQL · JWT · Tower

---

## 1. Introduccion a Axum

Cuando decides construir un sistema de inventarios, tienes que tomar una decisión fundamental: ¿qué framework web vas a usar? Esta no es una decisión técnica menor: el framework que elijas determinará cómo organizas tu código, cómo manejas la concurrencia, cómo se conecta con la base de datos, y hasta qué tan fácil será probar y mantener el sistema a lo largo de los años. En el manual principal de este repositorio usamos Actix Web, y fue una buena elección para un ERP/CRM porque Actix Web es maduro, rápido y tiene un ecosistema completo. Pero para un sistema de inventarios vamos a elegir un camino diferente: vamos a usar Axum, y la razón tiene que ver con la arquitectura que queremos implementar.

Axum es un framework web para Rust construido sobre Tokio, Tower y Hyper. Es mantenido directamente por el equipo de Tokio, que es el equipo que mantiene el runtime asíncrono más usado de Rust. Esto significa que Axum siempre será compatible con la última versión de Tokio, y que cualquier mejora en Tokio (y en Tower, y en Hyper) estará disponible en Axum desde el primer día. No hay dependencias de terceros ni adaptadores que mantener: Axum, Tokio, Tower y Hyper son todos parte del mismo ecosistema, desarrollados por las mismas personas, con la misma filosofía.

¿Y cuál es esa filosofía? La filosofía de Axum se puede resumir en una frase: **todo es un Layer**. Cada petición HTTP que llega al servidor pasa por una pila de capas (layers) antes de llegar al handler que la procesa. El logging es un layer, la compresión es un layer, la autenticación es un layer, CORS es un layer, el rate limiting es un layer, y el propio enrutador que dirige la petición al handler correcto también es un layer. Esta arquitectura de capas no es invento de Axum: viene de Tower, una librería genérica para construir middleware que es independiente de cualquier framework web. ¿Qué significa esto en la práctica? Que cualquier middleware que escribas para Tower funciona con Axum, pero también funciona con cualquier otro framework que use Tower (como `tonic` para gRPC, o `tower-http` para HTTP genérico). No estás aprendiendo middleware "para Axum", estás aprendiendo middleware "para Tower", que es un estándar en todo el ecosistema asíncrono de Rust.

Pensemos en esto desde la perspectiva de un sistema de inventarios. Un sistema de inventarios tiene operaciones que deben ser rápidas y atómicas: registrar una entrada de mercancía, transferir stock entre almacenes, consultar el stock actual en tiempo real. Cada una de estas operaciones implica una o más consultas a la base de datos, y el tiempo de respuesta total no debería superar los 50 milisegundos. Axum, al ser asíncrono nativo, maneja estas operaciones sin bloquear hilos. Mientras una consulta a la base de datos está en curso (esperando la respuesta de MySQL), el hilo de ejecución se libera para atender otras peticiones. Esto se llama "concurrencia sin paralelismo": un solo hilo puede manejar cientos de peticiones simultáneas porque ninguna espera activamente.

Pero hay una razón más profunda para elegir Axum en este manual, y esa razón es la **arquitectura hexagonal**. La arquitectura hexagonal (también llamada Puertos y Adaptadores) propone que la lógica de negocio —el dominio— debe estar en el centro del sistema, sin depender de ningún framework, base de datos o librería externa. Alrededor del dominio, los adaptadores se conectan a través de interfaces (puertos). El adaptador web recibe peticiones HTTP y las traduce a llamadas al dominio. El adaptador de persistencia ejecuta SQL y traduce los resultados a estructuras del dominio. El dominio no sabe que existe HTTP, no sabe que existe MySQL, no sabe que existe Axum. Solo conoce sus propias entidades y sus propios puertos.

Axum es especialmente adecuado para la arquitectura hexagonal porque no impone ninguna estructura. En Actix Web, los handlers deben devolver `impl Responder`, que es un trait específico de Actix. En Axum, los handlers devuelven `impl IntoResponse`, que también es un trait específico de Axum, pero la diferencia es que Axum facilita que el handler sea una capa delgada que solo traduce entre HTTP y el dominio. No tienes que luchar contra el framework para mantener el dominio limpio. El handler llama al caso de uso, el caso de uso usa los puertos, los puertos son implementados por adaptadores, y todo está desacoplado.

¿Y qué pasa con la base de datos? En Actix Web, cuando usamos Diesel (que es síncrono), necesitamos `web::block()` para no bloquear el hilo principal del servidor. Esto funciona, pero añade complejidad: estás moviendo la ejecución a un pool de hilos separado, esperando a que termine, y trayendo el resultado de vuelta. Con Axum, usaremos `sqlx`, que es un accesso a base de datos asíncrono nativo. Las consultas a la BD se hacen con `await`, igual que cualquier otra operación asíncrona. No hay cambios de contexto, no hay `web::block()`, no hay complejidad adicional. El hilo se libera automáticamente mientras la BD procesa la consulta.

### 1.1 Los tres pilares de Axum

Para entender Axum, tienes que entender las tres librerías sobre las que está construido. No porque tengas que usarlas directamente (rara vez lo harás), sino porque entender cómo se relacionan te ayudará a diagnosticar problemas, elegir middleware, y saber qué está pasando realmente cuando tu servidor recibe una petición.

El primer pilar es **Tokio**. Tokio es el runtime asíncrono de Rust. Es el equivalente a lo que el bucle de eventos es para Node.js, o el executor para Python asyncio. Tokio proporciona tres cosas: un bucle de eventos que gestiona la E/S de red, un temporizador para operaciones con timeout, y un executor de tareas que distribuye el trabajo entre los hilos del sistema. Sin Tokio, Axum no puede funcionar. Cuando tu servidor recibe una petición HTTP, Tokio crea una tarea asíncrona y la ejecuta en uno de sus hilos de trabajo. Cuando la tarea encuentra un `await` (por ejemplo, esperando una consulta a la BD), Tokio la pausa y ejecuta otra tarea en ese mismo hilo. Esto es la esencia de la programación asíncrona: un hilo maneja muchas tareas alternando entre ellas en cada punto de espera.

El segundo pilar es **Hyper**. Hyper es la implementación HTTP subyacente. Es la librería que entiende el protocolo HTTP: parsea las peticiones entrantes, construye las respuestas salientes, maneja las conexiones keep-alive, la codificación chunked, los headers, etc. Axum se construye sobre Hyper, pero Axum añade la capa de alto nivel que tú usas: rutas, extractores, respuestas JSON. Si quisieras, podrías usar Hyper directamente (y hay quien lo hace), pero Axum hace que sea mucho más cómodo.

El tercer pilar es **Tower**. Tower es una librería genérica para construir middleware. "Genérica" significa que no está atada a HTTP: puedes usar Tower para cualquier protocolo (HTTP, gRPC, WebSockets, TCP). Tower define un trait central llamado `Service` que tiene un método `call()` que recibe una petición y devuelve una respuesta. Cualquier cosa que implemente `Service` puede ser un layer de middleware. El logging implementa `Service`, la compresión implementa `Service`, la autenticación implementa `Service`. Y Axum usa Tower para todo su middleware.

Cuando agregas `axum = "0.8"` a tu `Cargo.toml`, también obtienes Tokio, Tower y Hyper como dependencias transitivas. No tienes que agregarlas manualmente a menos que quieras usarlas directamente (por ejemplo, para escribir middleware personalizado con Tower).

### 1.2 ¿Que construiremos en este manual?

A lo largo de este manual construiremos un sistema de gestión de inventarios multibodega completo. No será un "juguete" ni un "ejemplo académico": será un sistema funcional con persistencia en base de datos, API REST, autenticación, y desplegable con Docker. El sistema tendrá las siguientes capacidades:

- **Productos**: alta, baja, consulta y actualización de productos con SKU único. Cada producto tiene nombre, descripción, precio y un nivel de stock mínimo que dispara alertas.
- **Almacenes**: gestión de múltiples bodegas con nombre y ubicación. Cada producto puede tener stock en varios almacenes simultáneamente.
- **Movimientos**: registro de entradas (cuando llega mercancía), salidas (cuando se despacha), transferencias (entre almacenes) y ajustes (conteos físicos). Cada movimiento queda registrado para auditoría.
- **Lotes**: control de lotes con fecha de vencimiento, necesario para productos perecederos como alimentos o medicinas.
- **Alertas**: notificación automática cuando el stock de un producto baja del mínimo configurado.
- **Reportes**: valor total del inventario, rotación de productos, productos sin stock.

Cada una de estas funcionalidades se implementará como un caso de uso dentro de la arquitectura hexagonal. El dominio no sabrá que existe Axum ni que existe MySQL. Los adaptadores se conectarán a través de puertos (interfaces).

### 1.3 Para lectores del manual de Actix Web

Si vienes del manual principal de este repositorio (el de Actix Web), estos son los cambios conceptuales que debes tener en cuenta al aprender Axum. No son solo diferencias de sintaxis: son diferencias en la filosofía de diseño de cada framework.

| Aspecto | Actix Web | Axum | Por qué |
|---|---|---|---|
| Handler devuelve | `impl Responder` | `impl IntoResponse` | Axum permite que cualquier tipo se convierta en respuesta |
| Estado | `web::Data<T>` | `State<T>` | State es parte del sistema de tipos de Tower |
| Middleware | `.wrap()` (propio) | `.layer()` (Tower) | Tower es un estándar, no es específico de Axum |
| Enrutamiento | `.configure()` | `.nest()` y `.merge()` | Axum permite combinar routers como sub-módulos |
| Async runtime | actix-rt (integrado) | tokio (explícito) | Tokio es el estándar de facto en Rust |
| BD bloqueante | `web::block()` | No necesario | sqlx es async nativo |
| Extracción | `web::Path<T>` | `Path<T>` | Misma idea, nombres más cortos |

## 2. Configuracion del proyecto

Para crear un servidor HTTP con Axum, necesitas entender una diferencia fundamental con otros frameworks como Actix Web o Express.js. En esos frameworks, el servidor y la aplicación son un solo objeto: llamas a `HttpServer::new()` y dentro construyes la aplicación. En Axum, **el Router y el servidor están separados**. Primero construyes el Router (que es un mapa de rutas a handlers, sin ninguna conexión de red), luego creas un TcpListener (que es un socket de red escuchando en un puerto), y finalmente conectas ambos con `axum::serve()`.

Esta separación no es casual. Es consecuencia directa de la arquitectura de Tower que mencionamos en la introducción. El Router es un `Service` de Tower: recibe peticiones HTTP y devuelve respuestas HTTP. El TcpListener es un socket de red que acepta conexiones TCP. `axum::serve()` es la función que une ambos: acepta conexiones TCP del listener, parsea las peticiones HTTP, las pasa al Router, y envía las respuestas de vuelta por la conexión TCP.

¿Por qué es importante esta separación? Porque te permite probar el Router sin necesidad de abrir un puerto de red. Puedes crear un Router, construir peticiones HTTP de prueba, pasarlas al Router directamente, y verificar las respuestas, todo sin abrir sockets, sin permisos de administrador, sin firewalls. Esto se llama "testing sin servidor" y hace que las pruebas sean rápidas y deterministas. En Actix Web, para probar handlers necesitas `actix_web::test`, que inicia un servidor de prueba interno. En Axum, llamas a `Router::new().with_state(state)` y luego a `app.call(req)` — no hay servidor involucrado.

Otra consecuencia de esta separación es que puedes usar el mismo Router con diferentes listeners. Por ejemplo, puedes crear un Router, conectarlo a un listener HTTP en el puerto 8080 para tráfico normal, y al mismo tiempo conectarlo a un listener HTTPS en el puerto 8443 con TLS, todo sin duplicar las rutas. O puedes crear un listener Unix socket para comunicación interna entre microservicios. El Router no sabe ni le importa cómo llegan las peticiones: solo sabe procesarlas.

Pensemos en esto como una cadena de montaje. El TcpListener es el trabajador que recibe la materia prima (la conexión TCP). `axum::serve()` es el transportador que lleva la materia prima a la cadena. El Router es la cadena de montaje donde cada estación (layer) hace una transformación: un layer parsea headers, otro verifica autenticación, otro extrae parámetros, y finalmente el handler produce la respuesta. La respuesta viaja de vuelta por la misma cadena hasta el TcpListener, que la envía al cliente.

En esta sección vamos a construir cada pieza de esta cadena paso a paso: primero las dependencias, luego el Router, luego el listener, y finalmente conectaremos todo.

### 2.1 Dependencias en Cargo.toml

Cada proyecto de Rust comienza con un archivo Cargo.toml. Aquí declaramos las librerías que necesitamos. Para nuestro primer servidor necesitamos cuatro crates: axum para el framework, tokio para el runtime asíncrono, serde y serde_json para JSON, y tower-http para el middleware básico.

```toml
[package]
name = "axum_health"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = { version = "0.6", features = ["cors", "trace"] }
```

**Análisis línea por línea**:

- `axum = "0.8"`: el crate principal. Incluye `Router`, `routing::{get, post}`, extractores (`Path`, `Query`, `Json`, `State`), respuestas (`IntoResponse`, `Json`, `Html`), y el trait `FromRequest`. La versión 0.8 es la estable a principios de 2026. La API de Axum ha sido estable desde la versión 0.7, y los cambios entre versiones son menores.

- `tokio = { version = "1", features = ["full"] }`: Tokio es el runtime asíncrono. La feature `full` es un atajo que incluye todas las características: `rt-multi-thread` (ejecutor multi-hilo), `net` (E/S de red), `time` (temporizadores), `signal` (señales del sistema), `sync` (primitivas de sincronización). Sin `rt-multi-thread`, `#[tokio::main]` no puede ejecutar tareas en paralelo. Sin `net`, no puedes crear un `TcpListener`. Sin `time`, no puedes hacer `sleep` o timeouts. Por eso usamos `full`: es más fácil que recordar qué características necesitas.

- `serde = { version = "1", features = ["derive"] }`: serde es la librería de serialización/deserialización de Rust. La feature `derive` permite usar `#[derive(Serialize, Deserialize)]` en los structs. Sin esta feature, tendrías que implementar los traits manualmente, lo que requiere mucho código repetitivo.

- `serde_json = "1"`: el backend JSON para serde. Convierte structs Rust a JSON y viceversa. Es la librería JSON más usada en Rust.

- `tower-http = { version = "0.6", features = ["cors", "trace"] }`: Tower HTTP es un conjunto de middlewares listos para usar. `cors` añade soporte CORS (necesario para que un frontend en otro dominio pueda llamar a tu API). `trace` registra cada petición en los logs con su método, ruta, código de estado y duración. En producción, el logging de peticiones es esencial para depurar problemas.

### 2.2 Primer servidor HTTP: el health check

El primer servidor que vamos a construir tiene solo dos rutas. La ruta `/health` devuelve "OK", que es el health check más básico: si el servidor responde, está vivo. La ruta `/` devuelve información del servicio en JSON, incluyendo el nombre y la versión. Es como la "página de inicio" de la API, que un desarrollador puede consultar para saber qué versión del sistema está corriendo.

En un sistema de producción, el health check es más que un simple "OK". Debería verificar que la base de datos responde, que el pool de conexiones no está agotado, y que los servicios externos están disponibles. Pero para empezar, un health check simple nos basta. Lo importante aquí es entender la estructura: cómo se define un handler, cómo se registra en el Router, y cómo se inicia el servidor.

```rust
// archivo: src/main.rs
// Primer servidor con Axum
// Endpoints: / (info del servicio), /health (health check básico)

use axum::{Router, routing::get, Json};
use serde_json::{json, Value};
use std::net::SocketAddr;

async fn health() -> &'static str {
    "OK"
}

async fn info() -> Json<Value> {
    Json(json!({
        "servicio": "Sistema de Inventarios Axum",
        "version": env!("CARGO_PKG_VERSION"),
        "estado": "activo"
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(info))
        .route("/health", get(health));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Inventarios API escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

**Análisis línea por línea**:

- `use axum::{Router, routing::get, Json};`: importamos los tres tipos que necesitamos del crate `axum`. `Router` es la estructura que contiene el mapa de rutas. `routing::get` es una función que crea un `MethodRouter` para el método GET. `Json` es un tipo que envuelve datos y los serializa como JSON en la respuesta.

- `use serde_json::{json, Value};`: `json!` es una macro que permite escribir JSON literal en Rust. `Value` es un enum que representa cualquier valor JSON: objeto, array, string, número, booleano o null.

- `async fn health() -> &'static str { "OK" }`: Este es el handler más simple que puedes escribir en Axum. Recibe cero parámetros (no necesita Path, ni Query, ni State). Devuelve `&'static str`, que es un string estático: vive toda la vida del programa. Axum sabe cómo convertir `&'static str` en una respuesta HTTP porque implementa el trait `IntoResponse`. La conversión produce un 200 OK con `Content-Type: text/plain`. No necesitas especificar el código de estado ni el tipo de contenido: Axum lo deduce del tipo de retorno.

- `async fn info() -> Json<Value> {`: este handler devuelve JSON. `Json<Value>` es un struct que envuelve un `Value` de serde_json y lo serializa como respuesta. `Json<T>` implementa `IntoResponse` cuando `T: Serialize`. Como `Value` implementa `Serialize`, funciona.

- `Json(json!({"servicio": "Sistema de Inventarios Axum", ...}))`: la macro `json!` toma un literal JSON y produce un `Value`. Es más legible que construir el `Value` manualmente con `Value::Object(...)`. El resultado se envuelve en `Json()` para indicar que es una respuesta JSON.

- `env!("CARGO_PKG_VERSION")`: esta macro se evalúa EN TIEMPO DE COMPILACIÓN. Lee el campo `version = "0.1.0"` del `Cargo.toml` y lo incrusta como un string literal en el binario. Si el campo `version` no existe en el `Cargo.toml`, la compilación falla con un error claro.

- `#[tokio::main]`: esta macro transforma `async fn main()` en el punto de entrada real del programa. Sin ella, Rust no permite que `main` sea asíncrona. Internamente, `#[tokio::main]` genera un `fn main()` síncrono que:
  1. Crea un runtime Tokio con un pool de hilos (tantos como CPUs tenga el sistema)
  2. Inicia la función asíncrona en ese runtime
  3. Espera a que la función termine (en nuestro caso, nunca termina porque el servidor corre indefinidamente)

- `let app = Router::new().route("/", get(info)).route("/health", get(health))`: construye el Router. `Router::new()` crea un Router vacío. `.route()` añade una ruta: el primer argumento es el path, el segundo es el `MethodRouter`. `get(info)` crea un `MethodRouter` que solo acepta GET y llama al handler `info`. La misma ruta puede tener varios métodos: `.route("/ruta", get(handler).post(otro_handler))`.

- `let addr = SocketAddr::from(([127, 0, 0, 1], 8080))`: `SocketAddr` es el tipo estándar de Rust para direcciones de red. `127.0.0.1` es la dirección de loopback: solo acepta conexiones desde tu propia máquina. Para que otros equipos puedan conectarse, usarías `0.0.0.0`. `8080` es el puerto HTTP alternativo (el estándar es 80, pero requiere permisos de administrador).

- `let listener = tokio::net::TcpListener::bind(addr).await.unwrap();`: crea un socket TCP y lo vincula a la dirección. `bind()` puede fallar si el puerto está ocupado (Error: Address already in use) o si el puerto es menor a 1024 y no tienes permisos (Error: Permission denied). `.await` espera a que el socket esté listo. `.unwrap()`: si `bind()` falla, el programa entra en pánico con el mensaje de error. En producción usarías `expect()` o manejarías el error.

- `axum::serve(listener, app).await.unwrap();`: conecta el listener TCP con el Router y comienza a servir peticiones. Esta función:

  1. Acepta conexiones TCP entrantes del listener
  2. Parsea cada conexión como HTTP/1.1 o HTTP/2
  3. Pasa cada petición a través de los layers del Router
  4. Enruta la petición al handler correspondiente
  5. Envía la respuesta de vuelta por la conexión TCP

  El `.await` nunca termina a menos que el servidor se detenga (Ctrl+C, señal de terminación). Si `axum::serve()` falla (por ejemplo, un error interno de Hyper), el programa termina con el error.

**Salida esperada**:

Al ejecutar `cargo run`:

```
Inventarios API escuchando en http://127.0.0.1:8080
```

```bash
# Probar health check
curl http://127.0.0.1:8080/health
# Output: OK

# Probar información del servicio
curl http://127.0.0.1:8080/
# Output: {"servicio":"Sistema de Inventarios Axum","version":"0.1.0","estado":"activo"}
```

### 2.3 Errores tipicos al configurar el servidor

**Error 1: olvidar `#[tokio::main]`**.

```rust
// CÓDIGO QUE FALLA
async fn main() {
    println!("Hola");
}
```

Mensaje del compilador:
```
error[E0277]: `main` function is not allowed to be `async`
 --> src/main.rs:1:1
  |
1 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```
Solución: añadir `#[tokio::main]` encima de `fn main`.

**Error 2: puerto ocupado o sin permisos**.

```rust
// CÓDIGO QUE FALLA (puerto 80 sin sudo)
let addr = SocketAddr::from(([127, 0, 0, 1], 80));
let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
```

Mensaje en ejecución:
```
thread 'main' panicked at src/main.rs:12:46:
called `Result::unwrap()` on an `Err` value: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }
```
Solución: usar puerto 8080 o ejecutar con `sudo`. Si el error es "Address already in use", otro proceso está usando ese puerto. En Linux: `sudo lsof -i :8080` para encontrar el proceso, luego `kill <PID>`.

**Error 3: olvidar `.await` en `axum::serve`**.

```rust
// CÓDIGO QUE FALLA
axum::serve(listener, app);
println!("Esto se imprime inmediatamente");
```

No hay error de compilación, pero el programa imprime el mensaje y termina inmediatamente sin escuchar peticiones. Solución: `axum::serve(listener, app).await.unwrap();`.

### 2.4 Mini-proyecto: `01_axum_health`

El proyecto `01_axum_health` (en `proyectos_capitulo/parte4_axum/01_axum_health/`) contiene exactamente el código que acabamos de ver. Para ejecutarlo:

```bash
cd proyectos_capitulo/parte4_axum/01_axum_health
cargo run
```

En otra terminal:
```bash
curl http://127.0.0.1:8080/health
```

Este primer proyecto verifica que todo el entorno está configurado correctamente: Rust, Cargo, las dependencias, y la capacidad de abrir un puerto de red. Si este proyecto funciona, todos los siguientes también funcionarán.

## 3. Rutas, extractores y respuestas

En un sistema de inventarios, cada operación de negocio se traduce en una petición HTTP. Cuando un almacenero registra una entrada de mercancía, el sistema frontal envía un POST a `/api/movimientos/entrada`. Cuando un gerente consulta el stock, envía un GET a `/api/stock?producto=LAP-001`. Cada petición HTTP tiene tres partes: un método (GET, POST, PUT, DELETE), una ruta (`/api/productos/{sku}`), y opcionalmente un cuerpo (JSON con los datos). Axum maneja estas tres partes con el Router (para las rutas), los extractores (para extraer datos de la petición) y las respuestas (para devolver datos al cliente).

La clave para entender Axum es el concepto de **extractor**. Un extractor es un tipo que implementa el trait `FromRequest` (o `FromRequestParts`). Cuando declaras un handler como `async fn mi_handler(Path(id): Path<u32>, Json(body): Json<MiInput>)`, Axum ejecuta los extractores en orden: primero extrae `id` de la ruta, luego extrae `body` del cuerpo JSON. Si algún extractor falla (por ejemplo, el JSON no tiene el formato esperado), la petición nunca llega al handler: Axum devuelve automáticamente un error 422 Unprocessable Entity o 400 Bad Request.

Los extractores en Axum tienen un orden estricto. Deben declararse en este orden:

1. `Method` (el método HTTP)
2. `Path` (parámetros de ruta: `/api/productos/{sku}`)
3. `Query` (parámetros de query string: `?page=1`)
4. `Json` o `Form` (cuerpo de la petición)
5. `State` (datos compartidos de la aplicación)
6. `Extension` (datos de extensiones)

Si pones `State` antes que `Path`, el compilador te dará un error. Este orden no es caprichoso: refleja cómo Axum construye la petición. Primero identifica el método, luego extrae los parámetros de la ruta, luego los de query, luego parsea el cuerpo, y finalmente inyecta el estado. Cada paso construye sobre el anterior.

En las siguientes subsecciones veremos cada extractor en detalle, aplicado a nuestro sistema de inventarios.

### 3.1 Metodos HTTP y configuracion de rutas

El primer paso para construir una API REST es definir qué rutas existen y qué método HTTP acepta cada una. En Axum, las rutas se construyen con `Router::new().route()` y los métodos con funciones como `get()`, `post()`, etc.

```rust
use axum::{Router, routing::{get, post, put, delete}};

fn router_inventarios() -> Router {
    Router::new()
        .route("/api/productos", get(listar_productos).post(crear_producto))
        .route("/api/productos/{sku}", get(obtener_producto)
            .put(actualizar_producto)
            .delete(eliminar_producto))
        .route("/api/almacenes", get(listar_almacenes))
        .route("/api/stock", get(consultar_stock))
        .route("/api/movimientos/entrada", post(registrar_entrada))
        .route("/api/movimientos/transferencia", post(transferir_stock))
}
```

**Análisis línea por línea**:

- `use axum::{Router, routing::{get, post, put, delete}};`: importamos el Router y las funciones de método. Cada función devuelve un `MethodRouter` que solo acepta ese método HTTP.

- `.route("/api/productos", get(listar_productos).post(crear_producto))`: una misma ruta (`/api/productos`) con dos métodos diferentes. GET llama a `listar_productos`, POST llama a `crear_producto`. Esto es REST puro: el mismo recurso (productos) se comporta diferente según el método.

- `.route("/api/productos/{sku}", ...)`: `{sku}` es un parámetro de ruta. Axum captura el valor de la URL en esa posición y lo pone a disposición del extractor `Path`. Si la URL es `/api/productos/LAP-001`, `sku` vale `"LAP-001"`.

- `.route("/api/movimientos/entrada", post(registrar_entrada))`: las operaciones que modifican datos (crear un movimiento) usan POST. No hay GET para esta ruta porque no tiene sentido "listar entradas" desde esta URL específica (las entradas se listan desde `/api/movimientos` o desde el producto).

### 3.2 Extraccion de parametros de ruta con `Path`

Los parámetros de ruta se usan para identificar un recurso específico: `/api/productos/{sku}` identifica un producto concreto. Axum extrae el valor y lo convierte al tipo que necesites.

```rust
use axum::extract::Path;
use serde::Deserialize;

// Opción 1: con struct (recomendado cuando hay varios parámetros)
#[derive(Deserialize)]
struct ProductoPath {
    sku: String,
}

async fn obtener_producto(Path(path): Path<ProductoPath>) -> String {
    format!("Producto SKU: {}", path.sku)
}

// Opción 2: directo (para un solo parámetro)
async fn obtener_producto_directo(Path(sku): Path<String>) -> String {
    format!("Producto SKU: {}", sku)
}
```

**Análisis línea por línea**:

- `#[derive(Deserialize)] struct ProductoPath { sku: String }`: el struct debe tener un campo por cada parámetro de ruta. El nombre del campo (`sku`) debe coincidir exactamente con el nombre del parámetro en la ruta (`{sku}`). Si el ruta es `/api/productos/{id}`, el campo debe llamarse `id`. Si no coinciden, el extractor falla en tiempo de ejecución con 422.

- `Path(path): Path<ProductoPath>`: `Path<T>` es un extractor. Toma el valor del parámetro de ruta y lo deserializa en `T`. Si `T` es un struct, deserializa todos los parámetros de ruta en los campos del struct. Si la URL es `/api/productos/SKU-001`, `path.sku` vale `"SKU-001"`.

- `Path(sku): Path<String>`: variante directa para un solo parámetro. `Path<String>` extrae el único parámetro de ruta y lo convierte a `String`. Si hay más de un parámetro en la ruta, esta variante falla. Úsala solo cuando haya exactamente un parámetro.

- Si el parámetro no existe en la URL, o si el valor no se puede convertir al tipo solicitado (por ejemplo, `Path<u32>` con una URL `/api/productos/abc`), Axum devuelve automáticamente 422 Unprocessable Entity sin ejecutar el handler.

### 3.3 Extraccion de parametros de query con `Query`

Los parámetros de query se usan para filtros, paginación y opciones: `/api/stock?producto=LAP-001&almacen=1`. No identifican un recurso, sino que modifican la consulta.

```rust
use axum::extract::Query;

#[derive(Deserialize)]
struct StockQuery {
    producto: Option<String>,
    almacen: Option<u32>,
    stock_minimo: Option<f64>,
}

async fn consultar_stock(Query(params): Query<StockQuery>) -> String {
    let producto = params.producto.unwrap_or_default();
    let almacen = params.almacen.unwrap_or(0);
    format!("Stock para producto: {}, almacén: {}", producto, almacen)
}
```

**Análisis línea por línea**:

- `#[derive(Deserialize)] struct StockQuery { producto: Option<String>, ... }`: los campos son `Option` porque los parámetros de query son opcionales. Si el campo no es `Option` y el parámetro no se envía, Axum devuelve 422. Si es `Option` y no se envía, vale `None`.

- `Query(params): Query<StockQuery>`: extrae los parámetros de query string y los deserializa en el struct. Si la URL es `/api/stock?producto=LAP-001&almacen=1`, `params.producto = Some("LAP-001")`, `params.almacen = Some(1)`.

- `params.producto.unwrap_or_default()`: si no se envió `producto`, usamos string vacío. Es importante dar valores por defecto sensatos para que la API no falle cuando faltan parámetros opcionales.

### 3.4 Extraccion de JSON del cuerpo con `Json`

Para crear o actualizar recursos, el cliente envía un JSON en el cuerpo de la petición. Axum extrae y valida ese JSON con `Json<T>`.

```rust
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct CrearProductoInput {
    sku: String,
    nombre: String,
    precio: f64,
    stock_minimo: f64,
}

async fn crear_producto(Json(input): Json<CrearProductoInput>) -> Json<Value> {
    Json(json!({
        "mensaje": format!("Producto {} creado", input.sku)
    }))
}
```

**Análisis línea por línea**:

- `#[derive(Deserialize)] struct CrearProductoInput { ... }`: define el formato del JSON que espera el endpoint. Cada campo debe coincidir con una clave del JSON. Si el JSON tiene claves adicionales, se ignoran. Si faltan campos obligatorios, Axum devuelve 422.

- `Json(input): Json<CrearProductoInput>`: extrae el cuerpo de la petición, lo parsea como JSON, y lo deserializa a `CrearProductoInput`. Si el cuerpo no es JSON válido, o si los tipos no coinciden, Axum devuelve 400 Bad Request.

- `Json(json!({"mensaje": ...}))`: devuelve una respuesta JSON. El handler podría devolver el objeto creado completo (con ID generado), pero por ahora devolvemos solo un mensaje de confirmación.

### 3.5 Errores tipicos con extractores

**Error 1: orden incorrecto de extractores**.

```rust
// CÓDIGO QUE FALLA: State debe ir después de Path
async fn handler(State(state): State<AppState>, Path(id): Path<u32>) -> String {
    format!("{}", id)
}
```

Mensaje del compilador:
```
error[E0277]: the trait bound `State<AppState>: FromRequestParts<()>` is not satisfied
```
Solución: poner `Path` antes de `State`:
```rust
async fn handler(Path(id): Path<u32>, State(state): State<AppState>) -> String {
    format!("{}", id)
}
```

**Error 2: devolver Json sin Serialize**.

```rust
// CÓDIGO QUE FALLA: Producto no implementa Serialize
struct Producto { sku: String, nombre: String }

async fn handler() -> Json<Producto> {
    Json(Producto { sku: "X".into(), nombre: "Test".into() })
}
```

Mensaje del compilador:
```
error[E0277]: the trait bound `Producto: Serialize` is not satisfied
```
Solución: añadir `#[derive(Serialize)]` al struct.

**Error 3: campo obligatorio en Query que no se envía**.

```rust
// CÓDIGO QUE FALLA: page no es Option
#[derive(Deserialize)]
struct Paginacion { page: u32 }

async fn listar(Query(params): Query<Paginacion>) -> String {
    format!("Página {}", params.page)
}
```

Si el cliente llama a `/api/productos` sin `?page=...`, Axum devuelve 422. Solución: usar `Option<u32>` y valor por defecto.

## 4. Estado compartido

En una aplicación web, los handlers rara vez son funciones aisladas. Casi siempre necesitan acceso a recursos compartidos: el pool de conexiones a la base de datos, la configuración del sistema, un caché, un cliente de correo electrónico, etc. En Axum, estos recursos se comparten a través de `State<T>`, que es un extractor especial que inyecta datos en los handlers.

La diferencia fundamental entre Axum y Actix Web en este aspecto es que en Actix Web el estado se registra con `app_data()` y se extrae con `web::Data<T>`. En Axum, el estado se registra con `with_state()` y se extrae con `State<T>`. Ambas hacen lo mismo conceptualmente, pero la implementación es diferente: en Actix Web, `web::Data<T>` es un wrapper sobre `Arc<T>`. En Axum, `State<T>` es un extractor que busca el tipo `T` en un mapa interno del Router.

¿Por qué Axum usa un mapa en lugar de `Arc`? Porque el Router de Axum puede contener múltiples estados de diferentes tipos. Puedes tener `State<Pool>`, `State<Config>`, `State<ClienteHttp>`, todos registrados en el mismo Router pero con tipos diferentes. Cada handler extrae solo el estado que necesita. En Actix Web, también puedes tener múltiples `web::Data<T>`, pero la sintaxis es diferente.

Otra diferencia importante: en Axum, el estado debe implementar `Clone`. Esto es porque `with_state()` clona el estado para cada worker (hilo) del servidor. Si tu estado contiene un `MySqlPool`, clonarlo es barato porque `MySqlPool` internamente es un `Arc`. Si tu estado contiene datos grandes, debes envolverlos en `Arc` para que la clonación sea eficiente.

`State<T>` tiene una limitación importante: solo puede haber UNA instancia de cada tipo `T` en el Router. No puedes tener dos `State<String>` porque Axum no sabría cuál usar. Si necesitas múltiples valores del mismo tipo, envuélvelos en un struct: `State<Config>` donde `Config` tiene varios campos.

### 4.1 AppState con pool de sqlx

El caso de uso más común es compartir el pool de conexiones a la base de datos. Creamos un struct `AppState` que contiene el pool, lo registramos en el Router con `with_state()`, y los handlers lo reciben con `State<AppState>`.

```rust
use axum::{Router, extract::State};
use sqlx::MySqlPool;

#[derive(Clone)]
struct AppState {
    pool: MySqlPool,
}

async fn listar_productos(State(state): State<AppState>) -> Json<Value> {
    Json(json!({"productos": []}))
}

#[tokio::main]
async fn main() {
    let pool = MySqlPool::connect("mysql://root:secret@127.0.0.1:3306/inventarios")
        .await
        .expect("Error al conectar a la BD");

    let state = AppState { pool };

    let app = Router::new()
        .route("/api/productos", get(listar_productos))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

**Análisis línea por línea**:

- `#[derive(Clone)]`: obligatorio. Axum clona el estado cada vez que crea un nuevo worker (hilo) del servidor. Si olvidas `Clone`, el compilador da un error. No es negociable.

- `struct AppState { pool: MySqlPool }`: el pool de sqlx es el corazón del estado. `MySqlPool` implementa `Clone` eficientemente (internamente es un `Arc`), así que clonar el estado es barato.

- `MySqlPool::connect("mysql://...").await`: conecta a la base de datos y crea el pool. Es `async` porque establece la conexión inicial. `.await` espera a que la conexión esté lista.

- `State(state): State<AppState>`: el extractor. Axum busca en el Router un valor de tipo `AppState`. Si no está registrado, entra en pánico. La sintaxis `State(state)` extrae el valor y lo asigna a la variable `state`.

- `.with_state(state)`: registra el estado en el Router. Todos los handlers que usen `State<AppState>` recibirán este estado.

### 4.2 Errores tipicos

**Error 1: olvidar `#[derive(Clone)]`**.

```rust
// CÓDIGO QUE FALLA
struct AppState { pool: MySqlPool }
```

Mensaje:
```
error[E0277]: the trait bound `AppState: Clone` is not satisfied
```
Solución: añadir `#[derive(Clone)]` encima del struct.

**Error 2: no registrar el estado con `with_state()`**.

```rust
// CÓDIGO QUE FALLA: falta .with_state(state)
let app = Router::new()
    .route("/api/productos", get(listar_productos));
//                         ^^^  cuando se invoque este handler, entrará en pánico
```

No hay error de compilación, pero en ejecución:
```
thread 'tokio-runtime-worker' panicked at 'called `Option::unwrap()` on a `None` value'
```
Solución: siempre llamar a `.with_state(state)` en el Router.

## 5. Base de datos con sqlx

En la Parte 2 de este repositorio usamos el crate `mysql` para conectarnos a MariaDB, y en la Parte 3 usamos Diesel como ORM. En este manual usaremos **sqlx**, que es diferente a ambos. sqlx no es un ORM como Diesel (no genera migraciones automáticas ni tiene un DSL de tipos). Tampoco es un driver puro como el crate `mysql`. sqlx está en un punto intermedio: es un "query builder" asíncrono con verificación de tipos en tiempo de compilación opcional.

¿Por qué sqlx para un sistema de inventarios? Por tres razones. Primera, sqlx es **asíncrono nativo**. Todas las operaciones de base de datos son `async` y se ejecutan con `await`. Esto significa que no necesitamos `web::block()` como con Diesel en Actix Web. El hilo de ejecución se libera mientras la base de datos procesa la consulta, permitiendo que el servidor maneje otras peticiones simultáneamente.

Segunda, sqlx trabaja con **SQL directo**. No hay un DSL intermedio que aprender. Escribes `SELECT * FROM productos WHERE sku = ?` y sqlx lo ejecuta. Esto es importante para un sistema de inventarios porque las consultas de stock, movimientos y reportes suelen ser complejas y específicas. Con un ORM como Diesel, las consultas complejas requieren mezclar el DSL con SQL raw, lo que resta claridad. Con sqlx, escribes SQL directamente y punto.

Tercera, sqlx verifica las consultas en **tiempo de compilación** (con la macro `query!`) o en **tiempo de ejecución** (con `query_as`). La verificación en tiempo de compilación se conecta a la base de datos durante la compilación y verifica que las tablas, columnas y tipos existen. Si cambias el esquema de la BD y olvidas actualizar una consulta, el código no compila. Es la misma seguridad que ofrece Diesel, pero con SQL directo en lugar de un DSL.

### 5.1 Dependencias

```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "mysql", "chrono"] }
```

**Análisis línea por línea**:

- `sqlx = "0.8"`: la librería principal. Versión 0.8 es la estable a principios de 2026.
- `features = ["runtime-tokio"]`: integración con Tokio. sqlx necesita saber qué runtime asíncrono estás usando. Sin esta feature, no puedes usar sqlx con Axum.
- `features = ["mysql"]`: añade el driver MySQL/MariaDB. Sin esta feature, sqlx no sabe cómo conectarse a MySQL.
- `features = ["chrono"]`: integración con la librería chrono para tipos de fecha/hora. Permite usar `DateTime<Utc>` y `NaiveDate` en las consultas, necesarios para las fechas de vencimiento de lotes.

### 5.2 Esquema de la base de datos (inventarios)

```sql
-- sql/init.sql
CREATE DATABASE IF NOT EXISTS inventarios
    CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE inventarios;

CREATE TABLE productos (
    sku         VARCHAR(30) PRIMARY KEY,
    nombre      VARCHAR(150) NOT NULL,
    descripcion VARCHAR(255),
    precio      DECIMAL(12,2) NOT NULL DEFAULT 0,
    stock_minimo DECIMAL(12,2) NOT NULL DEFAULT 5,
    activo      BOOLEAN NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE almacenes (
    id      INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    nombre  VARCHAR(100) NOT NULL,
    ubicacion VARCHAR(255),
    activo  BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE stock (
    producto_sku VARCHAR(30) NOT NULL,
    almacen_id   INT UNSIGNED NOT NULL,
    cantidad     DECIMAL(12,2) NOT NULL DEFAULT 0,
    PRIMARY KEY (producto_sku, almacen_id),
    FOREIGN KEY (producto_sku) REFERENCES productos(sku),
    FOREIGN KEY (almacen_id) REFERENCES almacenes(id)
);

CREATE TABLE movimientos (
    id              BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    tipo            ENUM('ENTRADA','SALIDA','TRANSFERENCIA','AJUSTE') NOT NULL,
    producto_sku    VARCHAR(30) NOT NULL,
    almacen_origen  INT UNSIGNED,
    almacen_destino INT UNSIGNED,
    cantidad        DECIMAL(12,2) NOT NULL,
    referencia      VARCHAR(100),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (producto_sku) REFERENCES productos(sku),
    FOREIGN KEY (almacen_origen) REFERENCES almacenes(id),
    FOREIGN KEY (almacen_destino) REFERENCES almacenes(id)
);

CREATE TABLE lotes (
    id              BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    producto_sku    VARCHAR(30) NOT NULL,
    codigo_lote     VARCHAR(50) NOT NULL,
    cantidad        DECIMAL(12,2) NOT NULL,
    fecha_vencimiento DATE,
    almacen_id      INT UNSIGNED NOT NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (producto_sku) REFERENCES productos(sku),
    FOREIGN KEY (almacen_id) REFERENCES almacenes(id)
);

-- Datos de ejemplo
INSERT INTO almacenes (nombre, ubicacion) VALUES
    ('Bodega Central', 'Av. Principal 123'),
    ('Bodega Norte', 'Calle Secundaria 456'),
    ('Bodega Sur', 'Blvd. Tercero 789');

INSERT INTO productos (sku, nombre, precio, stock_minimo) VALUES
    ('LAP-001', 'Laptop HP ProBook', 18999.00, 5),
    ('MON-001', 'Monitor Dell 24"', 4500.00, 10),
    ('MOUS-001', 'Mouse Óptico USB', 350.00, 20),
    ('TEC-001', 'Teclado Mecánico', 1200.00, 15);

INSERT INTO stock (producto_sku, almacen_id, cantidad) VALUES
    ('LAP-001', 1, 25),
    ('LAP-001', 2, 10),
    ('MON-001', 1, 30),
    ('MOUS-001', 1, 100),
    ('MOUS-001', 2, 50),
    ('TEC-001', 1, 40);
```

**Análisis del esquema**:

- `productos`: catálogo de productos con SKU como clave primaria (no auto-incremental, porque el SKU es un código definido por el negocio).
- `almacenes`: lista de bodegas. Cada una tiene un nombre y ubicación.
- `stock`: relación producto-almacén con cantidad. Clave primaria compuesta.
- `movimientos`: registro de cada operación (entrada, salida, transferencia, ajuste). Esto permite auditoría y trazabilidad.
- `lotes`: para productos que requieren control de lote y fecha de vencimiento.

### 5.3 Conexion y consulta basica con sqlx

```rust
use sqlx::MySqlPool;

#[derive(Debug, sqlx::FromRow)]
struct Producto {
    sku: String,
    nombre: String,
    precio: f64,
    stock_minimo: f64,
}

async fn listar_productos(pool: &MySqlPool) -> Result<Vec<Producto>, sqlx::Error> {
    let productos = sqlx::query_as::<_, Producto>(
        "SELECT sku, nombre, precio, stock_minimo FROM productos WHERE activo = TRUE"
    )
    .fetch_all(pool)
    .await?;

    Ok(productos)
}
```

**Análisis línea por línea**:

- `#[derive(sqlx::FromRow)]`: genera el código para mapear una fila SQL al struct `Producto`. Los nombres de los campos deben coincidir con las columnas del SELECT.
- `sqlx::query_as::<_, Producto>("SELECT ...")`: `query_as` ejecuta una consulta y mapea cada fila al tipo especificado. `_` indica que el tipo de los parámetros se infiere.
- `.fetch_all(pool).await?`: ejecuta la consulta en el pool y espera el resultado. `fetch_all` devuelve todas las filas. `?` propaga errores de BD.

### 5.4 Consulta con parametros

```rust
async fn obtener_producto(pool: &MySqlPool, sku: &str) -> Result<Option<Producto>, sqlx::Error> {
    let producto = sqlx::query_as::<_, Producto>(
        "SELECT sku, nombre, precio, stock_minimo FROM productos WHERE sku = ?"
    )
    .bind(sku)
    .fetch_optional(pool)
    .await?;

    Ok(producto)
}
```

**Análisis**: `?` es un marcador de posición (como en el crate `mysql` de la Parte 2). `.bind(sku)` asigna el valor al primer `?`. `fetch_optional` devuelve `Option<Producto>`: `Some` si existe, `None` si no se encontró.

### 5.5 Insert y obtener el ID generado

```rust
async fn crear_almacen(pool: &MySqlPool, nombre: &str, ubicacion: &str) -> Result<u64, sqlx::Error> {
    let resultado = sqlx::query(
        "INSERT INTO almacenes (nombre, ubicacion) VALUES (?, ?)"
    )
    .bind(nombre)
    .bind(ubicacion)
    .execute(pool)
    .await?;

    Ok(resultado.last_insert_id())
}
```

**Análisis**: `.execute(pool)` ejecuta INSERT, UPDATE o DELETE. Devuelve `sqlx::mysql::MySqlQueryResult` que contiene `last_insert_id()` y `rows_affected()`.

### 5.6 Transaccion con sqlx

```rust
async fn registrar_entrada(
    pool: &MySqlPool,
    sku: &str,
    almacen_id: u32,
    cantidad: f64,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        "INSERT INTO movimientos (tipo, producto_sku, almacen_destino, cantidad)
         VALUES ('ENTRADA', ?, ?, ?)"
    )
    .bind(sku)
    .bind(almacen_id)
    .bind(cantidad)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO stock (producto_sku, almacen_id, cantidad)
         VALUES (?, ?, ?)
         ON DUPLICATE KEY UPDATE cantidad = cantidad + ?"
    )
    .bind(sku)
    .bind(almacen_id)
    .bind(cantidad)
    .bind(cantidad)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}
```

**Análisis línea por línea**:

- `let mut tx = pool.begin().await?;`: inicia una transacción. Devuelve un `Transaction` que implementa `Deref<Target = MySqlConnection>`.
- `.execute(&mut *tx)`: ejecuta dentro de la transacción. `*tx` desreferencia el Transaction a la conexión subyacente.
- `INSERT ... ON DUPLICATE KEY UPDATE`: si el producto ya tiene stock en ese almacén, suma la cantidad. Si no existe, lo inserta. Esto se llama **upsert** y evita la condición de carrera de "verificar si existe → insertar o actualizar".
- `tx.commit().await?;`: confirma la transacción. Si alguna operación falla antes, `tx` se descarta y se hace rollback automático.

## 6. Manejo de errores

Uno de los aspectos que distingue a una API profesional de una API amateur es cómo maneja los errores. Una API amateur devuelve un 500 Internal Server Error con un mensaje genérico como "algo salió mal". Una API profesional devuelve el código de estado correcto (404 si no se encontró el recurso, 400 si los datos son inválidos, 409 si hay un conflicto, 401 si no está autenticado), un mensaje descriptivo en el idioma del cliente, y un código de error que el cliente puede usar para diagnosticar.

En Axum, el mecanismo para lograr esto es el trait `IntoResponse`. Cualquier tipo que implemente `IntoResponse` puede ser devuelto por un handler. Esto incluye tipos estándar como `&str`, `String`, `Json<T>`, `StatusCode`, y tuplas como `(StatusCode, Json<Value>)`. También puedes implementar `IntoResponse` para tus propios tipos de error.

La filosofía de Axum es que los errores son solo **otro tipo de respuesta**. Un handler no debería hacer un `match` para decidir si devuelve éxito o error. En lugar de eso, el handler devuelve `Result<T, E>` donde tanto `T` como `E` implementan `IntoResponse`. Si el handler devuelve `Ok(producto)`, el cliente recibe 200 con el producto. Si devuelve `Err(AppError::NoEncontrado(...))`, el cliente recibe 404 con JSON de error. No hay condicionales en el handler.

Esto es diferente de Actix Web, donde los errores se manejan con `ResponseError`. En Actix Web, implementas `ResponseError` para tu tipo de error, y luego devuelves `Result<T, MiError>`. En Axum, implementas `IntoResponse` directamente. La diferencia es sutil: `ResponseError` es específico de Actix Web, mientras que `IntoResponse` es el mecanismo general de Axum para convertir cualquier cosa en una respuesta HTTP. Es más simple y más consistente.

### 6.1 AppError personalizado

Vamos a definir un tipo `AppError` que cubra todos los errores que puede devolver nuestro sistema de inventarios. Cada variante del enum se mapea a un código HTTP diferente:

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NoEncontrado(String),
    Validacion(String),
    BaseDatos(String),
    Conflicto(String),
    NoAutorizado,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, mensaje) = match self {
            AppError::NoEncontrado(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Validacion(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::BaseDatos(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Conflicto(msg) => (StatusCode::CONFLICT, msg),
            AppError::NoAutorizado => (StatusCode::UNAUTHORIZED, "No autorizado".into()),
        };

        (status, Json(json!({
            "error": mensaje,
            "codigo": status.as_u16()
        }))).into_response()
    }
}
```

**Análisis línea por línea**:

- `enum AppError`: define todos los errores que puede devolver nuestra API. Cada variante lleva un mensaje, excepto `NoAutorizado`.
- `impl IntoResponse for AppError`: implementa el trait de Axum. Convierte el error en una respuesta HTTP.
- `self.status_code()`: devuelve el código HTTP para cada variante.
- `(status, Json(json!({...})))`: construye una respuesta con código de estado y cuerpo JSON. Las tuplas `(StatusCode, Json<Value>)` implementan `IntoResponse`.
- `StatusCode::CONFLICT`: código 409, usado cuando hay un conflicto como stock insuficiente o SKU duplicado.

### 6.2 Uso de AppError en handlers

```rust
async fn obtener_producto_handler(
    State(state): State<AppState>,
    Path(sku): Path<String>,
) -> Result<Json<Producto>, AppError> {
    let producto = sqlx::query_as::<_, Producto>(
        "SELECT sku, nombre, precio, stock_minimo FROM productos WHERE sku = ?"
    )
    .bind(&sku)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| AppError::BaseDatos(e.to_string()))?
    .ok_or_else(|| AppError::NoEncontrado(format!("Producto {} no encontrado", sku)))?;

    Ok(Json(producto))
}
```

**Análisis línea por línea**:

- `Result<Json<Producto>, AppError>`: el handler devuelve `Ok(Json(producto))` o `Err(AppError)`. Axum convierte automáticamente el `AppError` en una respuesta HTTP usando `IntoResponse`.
- `.map_err(|e| AppError::BaseDatos(e.to_string()))?`: convierte el error de sqlx a `AppError::BaseDatos`. Sin esto, el compilador se quejaría de que `sqlx::Error` no implementa `IntoResponse`.
- `.ok_or_else(|| AppError::NoEncontrado(...))?`: convierte `None` (producto no encontrado) en un error 404.

### 6.3 Conversion automatica de sqlx::Error

Para no tener que escribir `.map_err()` en cada handler, podemos implementar `From<sqlx::Error>` para `AppError`:

```rust
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => {
                AppError::NoEncontrado("Recurso no encontrado".into())
            }
            _ => AppError::BaseDatos(e.to_string()),
        }
    }
}
```

Con esto, los handlers pueden usar `?` directamente:

```rust
async fn obtener_producto_handler(
    State(state): State<AppState>,
    Path(sku): Path<String>,
) -> Result<Json<Producto>, AppError> {
    let producto = sqlx::query_as::<_, Producto>(
        "SELECT sku, nombre, precio, stock_minimo FROM productos WHERE sku = ?"
    )
    .bind(&sku)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NoEncontrado(format!("Producto {} no encontrado", sku)))?;

    Ok(Json(producto))
}
```

### 6.4 Errores tipicos

**Error: no implementar `IntoResponse` para `AppError`**.

Si devuelves `Result<Json<T>, AppError>` sin implementar `IntoResponse`, el compilador dice: `the trait 'IntoResponse' is not implemented for 'AppError'`. Solución: implementar `IntoResponse`.

## 7. Arquitectura Hexagonal

Hasta ahora hemos escrito handlers que acceden directamente a la base de datos dentro de la misma función. El handler recibe una petición, ejecuta SQL, procesa los resultados y devuelve una respuesta. Esto funciona y es rápido de escribir, pero tiene un problema grave: **la lógica de negocio está acoplada a la base de datos**.

¿Qué significa esto en la práctica? Significa que si cambias de MySQL a PostgreSQL, tienes que revisar handler por handler para cambiar las consultas. Significa que si quieres probar la lógica de negocio (por ejemplo, "una salida no puede superar el stock disponible"), necesitas una base de datos funcionando. Significa que el conocimiento del negocio (las reglas de inventarios) está disperso entre los handlers, sin un lugar central donde entenderlo.

La **arquitectura hexagonal** (también llamada Puertos y Adaptadores, o Ports & Adapters) resuelve esto organizando el código en tres zonas concéntricas:

1. **El núcleo (dominio)**: contiene las entidades (Producto, Almacen) y los casos de uso (RegistrarEntrada, TransferirStock). No importa nada del mundo exterior: ni HTTP, ni SQL, ni JSON. Es Rust puro.
2. **Los puertos**: son interfaces (traits) que definen cómo el dominio se comunica con el exterior. `ProductoRepositorio` es un puerto: dice "puedo guardar y recuperar productos", pero no dice cómo.
3. **Los adaptadores**: son implementaciones concretas de los puertos. `ProductoRepositorioSqlx` implementa `ProductoRepositorio` usando sqlx y MySQL. `ProductoHandler` implementa la comunicación con HTTP usando Axum.

La regla de hierro de la arquitectura hexagonal es que **las dependencias apuntan hacia adentro**. El dominio no sabe que existen los adaptadores. Los adaptadores sí conocen el dominio (porque implementan sus puertos). El dominio define el "contrato" (los puertos) y los adaptadores lo cumplen.

Esta separación tiene beneficios prácticos inmediatos:
- Puedes probar los casos de uso sin base de datos, usando implementaciones mock de los puertos.
- Puedes cambiar de base de datos sin tocar la lógica de negocio.
- Puedes cambiar de framework web sin tocar la lógica de negocio.
- La lógica de negocio está centralizada: si la regla "stock mínimo" cambia, sabes exactamente dónde modificarla.

En las siguientes secciones implementaremos cada capa del hexágono para nuestro sistema de inventarios.

### 7.1 Estructura de directorios

```
                         ┌─────────────────┐
                         │    Axum (HTTP)   │
                         │  (API handlers)  │
                         └────────┬────────┘
                                  │
                    ┌─────────────▼──────────────┐
                    │       CASOS DE USO          │
                    │  (lógica de negocio pura)   │
                    │                             │
                    │  ┌─────────────────────┐    │
                    │  │    PUERTOS (traits)  │    │
                    │  │  ProductoRepositorio │    │
                    │  │  AlmacenRepositorio │    │
                    │  │  MovimientoRepositor│    │
                    │  └──────────┬──────────┘    │
                    └─────────────┼────────────────┘
                                  │
         ┌────────────────────────┼────────────────────────┐
         │                        │                        │
         ▼                        ▼                        ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   sqlx (MySQL)   │    │  Archivo JSON    │    │  API externa     │
│ (persistencia)   │    │   (testing)      │    │   (proveedor)    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

El dominio (centro) solo conoce los puertos. Los adaptadores (bordes) implementan los puertos. Los casos de uso (en el centro) orquestan la lógica llamando a los puertos.

## 8. Dominio: entidades y puertos

El dominio es la capa más importante de la arquitectura hexagonal, y también la más ignorada por los desarrolladores que empiezan con esta arquitectura. El error más común es pensar que el dominio es "lo mismo que antes, pero en una carpeta separada". No es así. El dominio no es solo una carpeta: es una **frontera**. Todo lo que está dentro del dominio no puede depender de nada que esté fuera. No puede importar `axum`, no puede importar `sqlx`, no puede importar `serde_json`. Solo puede importar Rust estándar y, como mucho, `serde` y `thiserror`.

¿Por qué esta restricción tan severa? Porque si el dominio depende de una librería externa, cualquier cambio en esa librería puede afectar la lógica de negocio. Si sqlx cambia su API, no solo tienes que actualizar el adaptador de persistencia: también tienes que revisar el dominio. Si Axum cambia su API, también afecta al dominio. La regla de "no dependencias externas en el dominio" garantiza que la lógica de negocio sea estable, portable y testeable.

Pensemos en esto desde la perspectiva de un sistema de inventarios. La regla "una salida no puede superar el stock disponible" es una regla de negocio que no debería cambiar nunca, independientemente de si usas MySQL, PostgreSQL, MongoDB o archivos CSV. Si esta regla está en el dominio (en un caso de uso), puedes cambiarla sin tocar la base de datos. Si está mezclada con SQL en un handler, cambiarla implica tocar SQL, HTTP, y todo lo demás.

El dominio de nuestro sistema de inventarios tiene tres componentes:
1. **Entidades**: structs que representan los datos del negocio: Producto, Almacen, Movimiento, StockEnAlmacen.
2. **Puertos**: traits que definen cómo el dominio se comunica con el exterior: `ProductoRepositorio`, `StockRepositorio`.
3. **Casos de uso**: funciones que implementan la lógica de negocio usando los puertos: `registrar_entrada()`, `transferir_stock()`.

En esta sección veremos las entidades y los puertos. Los casos de uso los veremos en la sección 9.

### 8.1 Entidades del dominio

Las entidades son structs simples que representan los conceptos del sistema de inventarios. No tienen métodos complejos ni lógica de negocio: solo datos. Usan `#[derive(Serialize, Deserialize)]` para poder convertirse a/desde JSON, pero no dependen de Axum para esto: `serde` es una librería de serialización independiente que no añade acoplamiento.

```rust
// dominio/entidades/producto.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Producto {
    pub sku: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio: f64,
    pub stock_minimo: f64,
    pub activo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoProducto {
    pub sku: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio: f64,
    pub stock_minimo: f64,
}
```

```rust
// dominio/entidades/almacen.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Almacen {
    pub id: u32,
    pub nombre: String,
    pub ubicacion: Option<String>,
    pub activo: bool,
}
```

```rust
// dominio/entidades/movimiento.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TipoMovimiento {
    Entrada,
    Salida,
    Transferencia,
    Ajuste,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movimiento {
    pub id: u64,
    pub tipo: TipoMovimiento,
    pub producto_sku: String,
    pub almacen_origen: Option<u32>,
    pub almacen_destino: Option<u32>,
    pub cantidad: f64,
    pub referencia: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockEnAlmacen {
    pub producto_sku: String,
    pub almacen_id: u32,
    pub cantidad: f64,
}
```

**Análisis**: todas las entidades usan `#[derive(Serialize, Deserialize)]` porque el dominio puede necesitar serializar datos (para respuestas HTTP), pero no depende de Axum ni de ningún framework. `serde` es la única dependencia externa del dominio.

### 8.2 Puertos (repositorios)

Los puertos son **traits** que definen las operaciones de persistencia que el dominio necesita. No tienen implementación; solo definen el contrato.

```rust
// dominio/puertos/repositorios.rs
use crate::dominio::entidades::*;
use async_trait::async_trait;

// NOTA: async_trait es necesario porque los traits con métodos async
// requieren esta macro hasta que los async traits estén estabilizados en Rust.

#[async_trait]
pub trait ProductoRepositorio: Send + Sync {
    async fn listar(&self) -> Result<Vec<Producto>, String>;
    async fn obtener(&self, sku: &str) -> Result<Option<Producto>, String>;
    async fn crear(&self, producto: &NuevoProducto) -> Result<(), String>;
    async fn actualizar(&self, sku: &str, producto: &NuevoProducto) -> Result<(), String>;
    async fn eliminar(&self, sku: &str) -> Result<(), String>;
}

#[async_trait]
pub trait AlmacenRepositorio: Send + Sync {
    async fn listar(&self) -> Result<Vec<Almacen>, String>;
    async fn obtener(&self, id: u32) -> Result<Option<Almacen>, String>;
}

#[async_trait]
pub trait StockRepositorio: Send + Sync {
    async fn consultar(&self, sku: &str) -> Result<Vec<StockEnAlmacen>, String>;
    async fn consultar_en_almacen(&self, sku: &str, almacen_id: u32) -> Result<f64, String>;
    async fn ajustar(&self, sku: &str, almacen_id: u32, cantidad: f64) -> Result<(), String>;
}

#[async_trait]
pub trait MovimientoRepositorio: Send + Sync {
    async fn registrar(&self, movimiento: &Movimiento) -> Result<(), String>;
    async fn listar_por_producto(&self, sku: &str) -> Result<Vec<Movimiento>, String>;
}
```

**Análisis línea por línea**:

- `#[async_trait]`: necesaria para usar métodos `async` en traits. Sin ella, Rust no permite `async fn` en traits.
- `pub trait ProductoRepositorio: Send + Sync {`: `Send` permite enviar el trait entre hilos. `Sync` permite compartirlo entre hilos. Ambas son necesarias porque Axum ejecuta handlers en múltiples hilos.
- Todos los métodos devuelven `Result<T, String>`. El tipo de error es `String` para mantener el dominio puro (sin dependencias de sqlx o axum).
- Los métodos son de alto nivel: `listar()`, `obtener()`, `crear()`, etc. No mencionan SQL ni tablas.

### 8.3 El secreto de la arquitectura hexagonal

La clave está en que **el dominio define los puertos**. El adaptador de persistencia **implementa** esos puertos. El caso de uso **llama** a los puertos sin saber cómo están implementados.

```
CasoUso → llama → ProductoRepositorio (trait)
                        ↑
                        │ implementa
                   ProductoRepositorioSqlx (struct con sqlx)
```

Si mañana cambiamos de sqlx a Diesel, creamos `ProductoRepositorioDiesel` que implementa el mismo trait, y el dominio no cambia. Si cambiamos de MySQL a PostgreSQL, solo cambia el adaptador. El dominio permanece intacto.

## 9. Casos de uso

Los casos de uso son el componente más importante de la arquitectura hexagonal porque contienen la **lógica de negocio**. Son los que responden preguntas como: "¿se puede registrar esta entrada?", "¿hay suficiente stock para esta salida?", "¿la transferencia es entre almacenes diferentes?". Los casos de uso no saben de HTTP ni de SQL: solo usan los puertos (traits) para obtener y guardar datos.

Un caso de uso típico sigue esta estructura:

1. **Recibe datos de entrada** (por ejemplo, SKU, almacén, cantidad).
2. **Valida las reglas de negocio** (el producto existe, la cantidad es positiva, hay stock suficiente).
3. **Usa los puertos para obtener datos** (consulta el producto, consulta el stock actual).
4. **Ejecuta la operación** (registra el movimiento, ajusta el stock).
5. **Devuelve el resultado** (el movimiento creado, o un error si algo falló).

Todo esto sin importar nada de Axum, ni de sqlx, ni de ningún framework. Los casos de uso reciben los puertos por constructor (inyección de dependencias) y los llaman a través de los traits.

### 9.1 Caso de uso: Registrar entrada de stock

```rust
// dominio/casos_uso/inventario_services.rs
use crate::dominio::entidades::*;
use crate::dominio::puertos::*;

pub struct InventarioService {
    pub producto_repo: Box<dyn ProductoRepositorio>,
    pub stock_repo: Box<dyn StockRepositorio>,
    pub movimiento_repo: Box<dyn MovimientoRepositorio>,
}

impl InventarioService {
    pub fn new(
        producto_repo: Box<dyn ProductoRepositorio>,
        stock_repo: Box<dyn StockRepositorio>,
        movimiento_repo: Box<dyn MovimientoRepositorio>,
    ) -> Self {
        Self { producto_repo, stock_repo, movimiento_repo }
    }

    pub async fn registrar_entrada(
        &self,
        sku: &str,
        almacen_id: u32,
        cantidad: f64,
        referencia: Option<String>,
    ) -> Result<Movimiento, String> {
        // 1. Validar que el producto existe
        let producto = self.producto_repo.obtener(sku).await?
            .ok_or_else(|| format!("Producto {} no encontrado", sku))?;

        // 2. Validar que la cantidad es positiva
        if cantidad <= 0.0 {
            return Err("La cantidad debe ser positiva".into());
        }

        // 3. Registrar el movimiento
        let movimiento = Movimiento {
            id: 0,
            tipo: TipoMovimiento::Entrada,
            producto_sku: sku.to_string(),
            almacen_origen: None,
            almacen_destino: Some(almacen_id),
            cantidad,
            referencia,
        };
        self.movimiento_repo.registrar(&movimiento).await?;

        // 4. Actualizar el stock
        self.stock_repo.ajustar(sku, almacen_id, cantidad).await?;

        Ok(movimiento)
    }

    pub async fn registrar_salida(
        &self,
        sku: &str,
        almacen_id: u32,
        cantidad: f64,
        referencia: Option<String>,
    ) -> Result<Movimiento, String> {
        // 1. Validar producto
        self.producto_repo.obtener(sku).await?
            .ok_or_else(|| format!("Producto {} no encontrado", sku))?;

        // 2. Validar cantidad positiva
        if cantidad <= 0.0 {
            return Err("La cantidad debe ser positiva".into());
        }

        // 3. Validar stock suficiente
        let stock_actual = self.stock_repo.consultar_en_almacen(sku, almacen_id).await?;
        if stock_actual < cantidad {
            return Err(format!(
                "Stock insuficiente en almacén {}. Disponible: {}, requerido: {}",
                almacen_id, stock_actual, cantidad
            ));
        }

        // 4. Registrar movimiento (cantidad negativa para salida)
        let movimiento = Movimiento {
            id: 0,
            tipo: TipoMovimiento::Salida,
            producto_sku: sku.to_string(),
            almacen_origen: Some(almacen_id),
            almacen_destino: None,
            cantidad: -cantidad,
            referencia,
        };
        self.movimiento_repo.registrar(&movimiento).await?;

        // 5. Actualizar stock (restar)
        self.stock_repo.ajustar(sku, almacen_id, -cantidad).await?;

        Ok(movimiento)
    }
}
```

**Análisis línea por línea**:

- `Box<dyn ProductoRepositorio>`: inyección de dependencias. El caso de uso recibe los repositorios por constructor. No sabe qué implementación concreta está usando.
- `self.producto_repo.obtener(sku).await?`: llama al puerto. El `?` propaga errores del repositorio.
- `.ok_or_else(|| format!(...))?`: convierte `None` (producto no encontrado) en un error.
- Validaciones de negocio: cantidad positiva, stock suficiente. Estas reglas están en el dominio, no en el handler ni en la base de datos.
- `self.stock_repo.ajustar(sku, almacen_id, cantidad)`: actualiza el stock. El repositorio implementa el `INSERT ... ON DUPLICATE KEY UPDATE`.

### 9.2 Caso de uso: Transferir stock entre almacenes

```rust
impl InventarioService {
    pub async fn transferir_stock(
        &self,
        sku: &str,
        origen_id: u32,
        destino_id: u32,
        cantidad: f64,
        referencia: Option<String>,
    ) -> Result<Movimiento, String> {
        if origen_id == destino_id {
            return Err("El almacén de origen y destino deben ser diferentes".into());
        }

        let stock_origen = self.stock_repo.consultar_en_almacen(sku, origen_id).await?;
        if stock_origen < cantidad {
            return Err(format!(
                "Stock insuficiente en origen. Disponible: {}, requerido: {}",
                stock_origen, cantidad
            ));
        }

        let movimiento = Movimiento {
            id: 0,
            tipo: TipoMovimiento::Transferencia,
            producto_sku: sku.to_string(),
            almacen_origen: Some(origen_id),
            almacen_destino: Some(destino_id),
            cantidad,
            referencia,
        };
        self.movimiento_repo.registrar(&movimiento).await?;
        self.stock_repo.ajustar(sku, origen_id, -cantidad).await?;
        self.stock_repo.ajustar(sku, destino_id, cantidad).await?;

        Ok(movimiento)
    }
}
```

**Análisis**: la transferencia es una salida del origen y una entrada al destino, pero se registra como un solo movimiento de tipo `Transferencia`. El stock se ajusta en ambos almacenes.

## 10. Adaptador de persistencia (sqlx)

Los adaptadores de persistencia son el otro lado de los puertos. Mientras que los puertos son interfaces en el dominio, los adaptadores son implementaciones concretas en el mundo exterior. El adaptador de persistencia es el que realmente ejecuta SQL, se conecta a MySQL, y convierte filas en entidades del dominio.

Este es el único lugar del sistema donde se importa `sqlx`. Aquí es donde se escriben las consultas SQL, se manejan las conexiones, y se ejecutan las transacciones. Si el dominio es el "qué" (qué operaciones están disponibles), el adaptador de persistencia es el "cómo" (cómo se ejecutan esas operaciones en MySQL).

La separación entre el puerto (definido en el dominio) y el adaptador (implementado aquí) permite cambiar de base de datos sin tocar la lógica de negocio. Si hoy usamos MySQL y mañana PostgreSQL, creamos un nuevo adaptador `ProductoRepositorioPostgres` que implemente el mismo `ProductoRepositorio`. El dominio no cambia.

### 10.1 Implementacion de ProductoRepositorio

```rust
// adaptadores/persistencia/producto_repo.rs
use async_trait::async_trait;
use sqlx::MySqlPool;
use crate::dominio::entidades::*;
use crate::dominio::puertos::ProductoRepositorio;

pub struct ProductoRepositorioSqlx {
    pool: MySqlPool,
}

impl ProductoRepositorioSqlx {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductoRepositorio for ProductoRepositorioSqlx {
    async fn listar(&self) -> Result<Vec<Producto>, String> {
        let productos = sqlx::query_as::<_, (String, String, Option<String>, f64, f64, bool)>(
            "SELECT sku, nombre, descripcion, precio, stock_minimo, activo FROM productos ORDER BY nombre"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Error al listar productos: {}", e))?
        .into_iter()
        .map(|(sku, nombre, descripcion, precio, stock_minimo, activo)| Producto {
            sku, nombre, descripcion, precio, stock_minimo, activo,
        })
        .collect();

        Ok(productos)
    }

    async fn obtener(&self, sku_buscar: &str) -> Result<Option<Producto>, String> {
        let resultado = sqlx::query_as::<_, (String, String, Option<String>, f64, f64, bool)>(
            "SELECT sku, nombre, descripcion, precio, stock_minimo, activo FROM productos WHERE sku = ?"
        )
        .bind(sku_buscar)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Error al obtener producto: {}", e))?
        .map(|(sku, nombre, descripcion, precio, stock_minimo, activo)| Producto {
            sku, nombre, descripcion, precio, stock_minimo, activo,
        });

        Ok(resultado)
    }

    async fn crear(&self, producto: &NuevoProducto) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO productos (sku, nombre, descripcion, precio, stock_minimo) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&producto.sku)
        .bind(&producto.nombre)
        .bind(&producto.descripcion)
        .bind(producto.precio)
        .bind(producto.stock_minimo)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("Duplicate entry") {
                format!("El SKU '{}' ya existe", producto.sku)
            } else {
                format!("Error al crear producto: {}", e)
            }
        })?;

        Ok(())
    }

    async fn actualizar(&self, sku: &str, producto: &NuevoProducto) -> Result<(), String> {
        let afectadas = sqlx::query(
            "UPDATE productos SET nombre = ?, descripcion = ?, precio = ?, stock_minimo = ? WHERE sku = ?"
        )
        .bind(&producto.nombre)
        .bind(&producto.descripcion)
        .bind(producto.precio)
        .bind(producto.stock_minimo)
        .bind(sku)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Error al actualizar producto: {}", e))?
        .rows_affected();

        if afectadas == 0 {
            return Err(format!("Producto {} no encontrado", sku));
        }

        Ok(())
    }

    async fn eliminar(&self, sku: &str) -> Result<(), String> {
        let afectadas = sqlx::query("DELETE FROM productos WHERE sku = ?")
            .bind(sku)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Error al eliminar producto: {}", e))?
            .rows_affected();

        if afectadas == 0 {
            return Err(format!("Producto {} no encontrado", sku));
        }

        Ok(())
    }
}
```

**Análisis línea por línea**:

- `sqlx::query_as::<_, (String, String, Option<String>, f64, f64, bool)>`: mapea cada fila a una tupla. Luego convertimos la tupla a `Producto` manualmente. Alternativamente, podríamos usar `#[derive(sqlx::FromRow)]` en `Producto`, pero eso crearía una dependencia de sqlx en el dominio, que queremos evitar.
- `.map_err(|e| format!("Error al listar productos: {}", e))`: convierte el error de sqlx a `String`, que es el tipo de error del puerto.
- `.rows_affected()`: devuelve el número de filas afectadas por UPDATE o DELETE. Si es 0, el producto no existía.

## 11. Adaptador web (Axum)

El adaptador web es la capa que conecta el dominio con HTTP. Está formado por los handlers de Axum, el Router, y los tipos de entrada/salida. Es la capa más delgada del sistema, y debe seguir siéndolo. Un handler no debería contener lógica de negocio: solo debería extraer los datos de la petición, llamar al caso de uso, y convertir el resultado en una respuesta HTTP.

La regla de oro del adaptador web es: **si ves un `if` con lógica de negocio en un handler, esa lógica debería estar en un caso de uso**. Si ves un SQL en un handler, ese SQL debería estar en un adaptador de persistencia. El handler es solo un traductor entre HTTP y el dominio.

En nuestro sistema de inventarios, los handlers reciben el estado (que contiene los servicios de casos de uso), extraen los parámetros con `Path`, `Query` y `Json`, llaman al servicio correspondiente, y devuelven `Json` con el resultado. Si el caso de uso falla, el error se convierte automáticamente en una respuesta HTTP gracias a `AppError: IntoResponse`.

### 11.1 Handlers de productos

```rust
// adaptadores/api/handlers/producto_handler.rs
use axum::{extract::State, Json};
use axum::extract::Path;
use std::sync::Arc;
use crate::adaptadores::api::AppState;
use crate::dominio::casos_uso::producto_services::ProductoService;
use crate::dominio::entidades::*;
use crate::errores::AppError;

pub async fn listar_productos(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Producto>>, AppError> {
    let productos = state.producto_service.listar().await
        .map_err(|e| AppError::BaseDatos(e))?;

    Ok(Json(productos))
}

pub async fn obtener_producto(
    State(state): State<Arc<AppState>>,
    Path(sku): Path<String>,
) -> Result<Json<Producto>, AppError> {
    let producto = state.producto_service.obtener(&sku).await
        .map_err(|e| AppError::BaseDatos(e))?
        .ok_or_else(|| AppError::NoEncontrado(format!("Producto {} no encontrado", sku)))?;

    Ok(Json(producto))
}

pub async fn crear_producto(
    State(state): State<Arc<AppState>>,
    Json(input): Json<NuevoProducto>,
) -> Result<Json<Producto>, AppError> {
    let producto = state.producto_service.crear(input).await
        .map_err(|e| {
            if e.contains("ya existe") {
                AppError::Conflicto(e)
            } else {
                AppError::BaseDatos(e)
            }
        })?;

    Ok(Json(producto))
}
```

**Análisis línea por línea**:

- `State(state): State<Arc<AppState>>`: inyecta el estado de la aplicación.
- `state.producto_service.listar().await`: llama al caso de uso. El handler no sabe cómo se implementa `listar()`. Podría usar sqlx, un archivo JSON, o una API externa.
- `.map_err(|e| AppError::BaseDatos(e))`: convierte el error del dominio (String) al error HTTP.
- `Json(producto)`: serializa el producto a JSON. El handler es una capa delgada: toda la lógica está en el dominio.

### 11.2 Handlers de inventario (entradas, salidas, transferencias)

```rust
use axum::{extract::State, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EntradaInput {
    pub sku: String,
    pub almacen_id: u32,
    pub cantidad: f64,
    pub referencia: Option<String>,
}

#[derive(Deserialize)]
pub struct TransferenciaInput {
    pub sku: String,
    pub origen_id: u32,
    pub destino_id: u32,
    pub cantidad: f64,
    pub referencia: Option<String>,
}

pub async fn registrar_entrada(
    State(state): State<Arc<AppState>>,
    Json(input): Json<EntradaInput>,
) -> Result<Json<Value>, AppError> {
    let movimiento = state.inventario_service.registrar_entrada(
        &input.sku,
        input.almacen_id,
        input.cantidad,
        input.referencia,
    ).await.map_err(|e| AppError::Validacion(e))?;

    Ok(Json(json!({
        "mensaje": "Entrada registrada",
        "movimiento_id": movimiento.id
    })))
}

pub async fn transferir_stock(
    State(state): State<Arc<AppState>>,
    Json(input): Json<TransferenciaInput>,
) -> Result<Json<Value>, AppError> {
    let movimiento = state.inventario_service.transferir_stock(
        &input.sku,
        input.origen_id,
        input.destino_id,
        input.cantidad,
        input.referencia,
    ).await.map_err(|e| AppError::Validacion(e))?;

    Ok(Json(json!({
        "mensaje": "Transferencia realizada",
        "movimiento_id": movimiento.id
    })))
}
```

### 11.3 Router principal

```rust
// adaptadores/api/router.rs
use axum::{Router, routing::{get, post, put, delete}};
use std::sync::Arc;
use crate::adaptadores::api::handlers::*;
use crate::adaptadores::api::AppState;

pub fn crear_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Productos
        .route("/api/productos", get(producto_handler::listar_productos)
            .post(producto_handler::crear_producto))
        .route("/api/productos/{sku}", get(producto_handler::obtener_producto)
            .put(producto_handler::actualizar_producto)
            .delete(producto_handler::eliminar_producto))
        // Almacenes
        .route("/api/almacenes", get(almacen_handler::listar_almacenes))
        // Stock
        .route("/api/stock", get(stock_handler::consultar_stock))
        // Movimientos
        .route("/api/movimientos/entrada", post(inventario_handler::registrar_entrada))
        .route("/api/movimientos/salida", post(inventario_handler::registrar_salida))
        .route("/api/movimientos/transferencia", post(inventario_handler::transferir_stock))
        // Reportes
        .route("/api/reportes/valor-inventario", get(reporte_handler::valor_inventario))
        // Health
        .route("/api/health", get(health))
        .with_state(state)
}
```

## 12. Pruebas

Una de las ventajas más importantes de la arquitectura hexagonal es que hace que las pruebas sean rápidas, deterministas y sencillas. Como el dominio no depende de la base de datos ni del framework web, puedes probar los casos de uso en milisegundos, sin levantar MySQL ni Axum.

Para probar un caso de uso, creas implementaciones **mock** de los puertos. Un mock es una implementación falsa que devuelve datos prefijados en lugar de consultar una base de datos real. Por ejemplo, un `MockProductoRepo` devuelve un producto cuando el SKU es "EXISTE" y `None` cuando es cualquier otro SKU. Un `MockStockRepo` devuelve 50 unidades de stock siempre.

Con estos mocks, puedes probar todas las ramas de la lógica de negocio:
- ¿Qué pasa si el producto no existe? → Mock devuelve `None` → caso de uso debe devolver error "no encontrado".
- ¿Qué pasa si la cantidad es negativa? → caso de uso debe devolver error "cantidad positiva".
- ¿Qué pasa si el stock es insuficiente? → Mock devuelve stock=5, caso de uso pide 100 → error "insuficiente".
- ¿Qué pasa si todo está bien? → caso de uso debe devolver Ok(movimiento).

Estas pruebas se ejecutan sin base de datos, sin servidor HTTP, sin nada. Son rápidas (microsegundos), deterministas (siempre dan el mismo resultado), y fáciles de escribir (solo implementas un trait con 2-3 métodos).

### 12.1 Test del caso de uso con mock

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::dominio::entidades::*;
    use crate::dominio::puertos::*;
    use async_trait::async_trait;

    struct MockProductoRepo;
    struct MockStockRepo;
    struct MockMovimientoRepo;

    #[async_trait]
    impl ProductoRepositorio for MockProductoRepo {
        async fn listar(&self) -> Result<Vec<Producto>, String> { Ok(vec![]) }
        async fn obtener(&self, sku: &str) -> Result<Option<Producto>, String> {
            if sku == "EXISTE" {
                Ok(Some(Producto {
                    sku: "EXISTE".into(),
                    nombre: "Test".into(),
                    descripcion: None,
                    precio: 100.0,
                    stock_minimo: 5.0,
                    activo: true,
                }))
            } else {
                Ok(None)
            }
        }
        async fn crear(&self, _: &NuevoProducto) -> Result<(), String> { Ok(()) }
        async fn actualizar(&self, _: &str, _: &NuevoProducto) -> Result<(), String> { Ok(()) }
        async fn eliminar(&self, _: &str) -> Result<(), String> { Ok(()) }
    }

    #[async_trait]
    impl StockRepositorio for MockStockRepo {
        async fn consultar(&self, _: &str) -> Result<Vec<StockEnAlmacen>, String> { Ok(vec![]) }
        async fn consultar_en_almacen(&self, _: &str, _: u32) -> Result<f64, String> { Ok(50.0) }
        async fn ajustar(&self, _: &str, _: u32, _: f64) -> Result<(), String> { Ok(()) }
    }

    #[async_trait]
    impl MovimientoRepositorio for MockMovimientoRepo {
        async fn registrar(&self, _: &Movimiento) -> Result<(), String> { Ok(()) }
        async fn listar_por_producto(&self, _: &str) -> Result<Vec<Movimiento>, String> { Ok(vec![]) }
    }

    #[tokio::test]
    async fn test_registrar_entrada_producto_no_existe() {
        let service = InventarioService::new(
            Box::new(MockProductoRepo),
            Box::new(MockStockRepo),
            Box::new(MockMovimientoRepo),
        );

        let resultado = service.registrar_entrada("NO_EXISTE", 1, 10.0, None).await;
        assert!(resultado.is_err());
        assert!(resultado.unwrap_err().contains("no encontrado"));
    }

    #[tokio::test]
    async fn test_registrar_entrada_cantidad_negativa() {
        let service = InventarioService::new(
            Box::new(MockProductoRepo),
            Box::new(MockStockRepo),
            Box::new(MockMovimientoRepo),
        );

        let resultado = service.registrar_entrada("EXISTE", 1, -5.0, None).await;
        assert!(resultado.is_err());
        assert!(resultado.unwrap_err().contains("positiva"));
    }

    #[tokio::test]
    async fn test_registrar_salida_stock_insuficiente() {
        let service = InventarioService::new(
            Box::new(MockProductoRepo),
            Box::new(StockInsuficienteMock),
            Box::new(MockMovimientoRepo),
        );

        let resultado = service.registrar_salida("EXISTE", 1, 100.0, None).await;
        assert!(resultado.is_err());
        assert!(resultado.unwrap_err().contains("insuficiente"));
    }

    struct StockInsuficienteMock;
    #[async_trait]
    impl StockRepositorio for StockInsuficienteMock {
        async fn consultar_en_almacen(&self, _: &str, _: u32) -> Result<f64, String> { Ok(5.0) }
        // ... otros métodos igual que MockStockRepo
    }
}
```

**Análisis**: los tests prueban la lógica de negocio pura (validaciones de stock, cantidad positiva, producto existe) sin necesidad de base de datos. Esto hace que los tests sean rápidos y deterministas.

## 13. Proyecto final: API de inventarios completa

El proyecto final integra todas las piezas que hemos construido por separado: el dominio con sus entidades y puertos, los casos de uso con la lógica de negocio, el adaptador de persistencia con sqlx, y el adaptador web con Axum. Todo conectado mediante inyección de dependencias.

La inyección de dependencias es el mecanismo que une las capas. En el `main.rs`, creamos las implementaciones concretas de los puertos (los adaptadores sqlx), las envolvemos en los casos de uso (que solo conocen los traits), y registramos los casos de uso en el estado de Axum. Los handlers reciben el estado y llaman a los casos de uso sin saber qué implementación concreta hay detrás.

Este es el patrón que sigue `main.rs`:

```ascii
Crear pool sqlx → Crear repositorios (adaptadores) → Crear servicios (casos de uso) → Crear estado → Crear Router → Iniciar servidor
```

Cada paso es explícito y visible. No hay magia. No hay "auto-wiring". La inyección es manual, lo que hace que el flujo de dependencias sea claro y fácil de modificar.

El código completo está en `proyecto_api_axum_inventarios/`.

### 13.1 main.rs final

```rust
#[tokio::main]
async fn main() {
    // Cargar configuración
    let config = config::cargar();

    // Crear pool de BD
    let pool = sqlx::MySqlPool::connect(&config.database_url)
        .await
        .expect("Error al conectar a la BD");

    // Crear repositorios (adaptadores de persistencia)
    let producto_repo = Box::new(ProductoRepositorioSqlx::new(pool.clone()));
    let stock_repo = Box::new(StockRepositorioSqlx::new(pool.clone()));
    let movimiento_repo = Box::new(MovimientoRepositorioSqlx::new(pool.clone()));
    let almacen_repo = Box::new(AlmacenRepositorioSqlx::new(pool.clone()));

    // Crear servicios (casos de uso)
    let producto_service = ProductoService::new(producto_repo);
    let inventario_service = InventarioService::new(
        Box::new(ProductoRepositorioSqlx::new(pool.clone())),
        stock_repo,
        movimiento_repo,
    );

    // Estado de la aplicación
    let state = Arc::new(AppState {
        pool,
        producto_service,
        inventario_service,
        almacen_repo,
    });

    // Router
    let app = crear_router(state);

    // Iniciar servidor
    let addr = format!("{}:{}", config.host, config.puerto);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("API de Inventarios escuchando en http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
```

**Análisis**: la inyección de dependencias es manual (creamos los repositorios y los pasamos a los servicios). Cada servicio recibe solo los puertos que necesita.

## 14. Ejercicios

1. **Implementar caso de uso `AjustarStock`**: permite ajustar el stock de un producto en un almacén manualmente (para conteos físicos). Debe registrar un movimiento de tipo `Ajuste`.

2. **Agregar alerta de stock bajo**: modificar `InventarioService` para que después de cada salida o transferencia, verifique si el stock de algún producto bajó del mínimo y devuelva una advertencia.

3. **Implementar búsqueda de productos**: agregar un endpoint `GET /api/productos?q=termino` que busque productos por nombre o SKU.

4. **Agregar control de lotes**: implementar el caso de uso `RegistrarEntradaConLote` que asocie la entrada a un lote específico con fecha de vencimiento.

5. **Implementar reporte de valor de inventario**: crear un endpoint que calcule el valor total del inventario (suma de stock × precio en todos los almacenes).

6. **Agregar paginación al listado de productos**: modificar `listar_productos` para aceptar `?page=1&per_page=20`.

7. **Implementar autenticación JWT**: agregar un middleware que verifique tokens JWT en los endpoints protegidos.

8. **Test de integración con BD de prueba**: crear un test que use una base de datos MySQL de prueba (con `sqlx::test`) y verifique el flujo completo de entrada-salida.

## 15. Soluciones

(Las soluciones detalladas de los ejercicios se entregan junto con el proyecto final en `proyecto_api_axum_inventarios/`.)
