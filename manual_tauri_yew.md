# Manual de Tauri + Yew para aplicaciones de escritorio

> **Construye aplicaciones de escritorio modernas con Tauri (Rust nativo) y Yew (Rust/WASM). Dashboard del ERP, inventarios y tickets como aplicaciones nativas.**

---

**Stack tecnológico**: Tauri 2 · Yew 0.21 · WASM · Trunk · Rust 1.82+

---

## 1. Introduccion a Tauri y Yew

Durante décadas, construir una aplicación de escritorio significaba elegir entre dos opciones: usar un framework nativo como Qt o WinForms (rápido pero verboso y dependiente de la plataforma), o usar Electron (multiplataforma pero pesado, lento y con un consumo de memoria que supera los 500MB). Tauri ofrece una tercera vía: el backend en Rust nativo (rápido, seguro, binarios pequeños) y el frontend en tecnologías web (HTML, CSS, JavaScript o Rust/WASM). El resultado: aplicaciones nativas de menos de 10MB que consumen una fracción de la memoria de Electron.

Tauri se divide en dos partes: el **núcleo Rust** (core) que se ejecuta como un proceso nativo, y la **vista web** (webview) que renderiza la interfaz. La comunicación entre ambas partes se hace a través de **comandos** (commands): funciones Rust que el frontend puede llamar como si fueran APIs. Cuando el frontend necesita leer un archivo del disco, llamar a la base de datos, o ejecutar un cálculo pesado, invoca un comando Tauri. El backend Rust lo ejecuta en un hilo separado (sin bloquear la UI) y devuelve el resultado.

Yew es un framework frontend escrito en Rust que compila a WebAssembly (WASM). Es similar a React: tiene componentes, estado, propiedades, y un concepto de "virtual DOM". Pero a diferencia de React, Yew se escribe en Rust, lo que significa que el compilador verifica los tipos de las propiedades, el estado, y los mensajes de los eventos. No puedes pasar un número donde se espera un string: el compilador no te deja.

¿Por qué Tauri + Yew y no Tauri + React? Porque Yew te permite escribir todo el código en Rust. El frontend (Yew) y el backend (Tauri) están en el mismo lenguaje. Puedes compartir tipos entre frontend y backend (como las entidades del dominio) sin necesidad de serialización manual ni de mantener dos bases de código en lenguajes diferentes. El mismo struct `Producto` se usa tanto en el frontend para mostrar los datos como en el backend para procesarlos.

### 1.1 ¿Que construiremos?

A lo largo de este manual construiremos una aplicación de escritorio para el sistema de inventarios del repositorio:

- **Backend (Tauri)**: comandos para consultar productos, registrar entradas/salidas, transferir stock entre almacenes. El backend se conecta a MariaDB (la misma base de datos del manual_axum.md).
- **Frontend (Yew)**: interfaz gráfica con lista de productos, formulario de entrada de stock, tabla de movimientos recientes, y dashboard con alertas de stock bajo.

La aplicación tendrá este aspecto funcional:

```
┌────────────────────────────────────────────────┐
│  SISTEMA DE INVENTARIOS  ─── ▯ □ ✕             │
├────────────────────────────────────────────────┤
│ [Productos] [Movimientos] [Reportes]           │
├────────────────────────────────────────────────┤
│                                                │
│  Productos (25)              Buscar: [______]  │
│  ┌──────┬──────────────────┬────────┬────────┐ │
│  │ SKU  │ Nombre           │ Stock  │ Precio │ │
│  ├──────┼──────────────────┼────────┼────────┤ │
│  │ LAP  │ Laptop HP        │ 35     │ $18999 │ │
│  │ MON  │ Monitor Dell 24" │ 30     │ $4500  │ │
│  │ MOUS │ Mouse Óptico     │ 150    │ $350   │ │
│  └──────┴──────────────────┴────────┴────────┘ │
│                                                │
│  [Registrar Entrada]  [Transferir Stock]      │
└────────────────────────────────────────────────┘
```

### 1.2 Arquitectura Tauri + Yew

```
┌──────────────────────────────────────────────────┐
│                  Tauri App                        │
│  ┌─────────────────┐      ┌──────────────────┐   │
│  │  Yew Frontend    │◄────►  Rust Backend    │   │
│  │  (WASM)          │ cmd   │  (Tauri Core)   │   │
│  │                  │──────►│                  │   │
│  │  Componentes     │       │  Comandos Rust   │   │
│  │  Estado          │       │  Acceso a BD     │   │
│  │  Eventos         │       │  Archivos        │   │
│  └─────────────────┘       └──────────────────┘   │
│         │                          │              │
│         │ webview                  │ nativo       │
│         ▼                          ▼              │
│  ┌─────────────┐          ┌──────────────┐       │
│  │ WebView     │          │ MariaDB      │       │
│  │ (HTML/CSS)  │          │ (via sqlx)   │       │
│  └─────────────┘          └──────────────┘       │
└──────────────────────────────────────────────────┘
```

### 1.3 Comparacion con Electron

Si has usado Electron antes (aplicaciones de escritorio con JavaScript), la diferencia con Tauri es abismal. Electron incluye un Chromium completo dentro de cada aplicación, lo que hace que cada app pese ~150MB y consuma ~300MB de RAM. Si tienes 5 aplicaciones de Electron abiertas, has consumido 1.5GB de RAM solo en los Chromium de cada una. Tauri, en cambio, usa la webview del sistema operativo (WebView2 en Windows, WebKit en macOS, WebKitGTK en Linux). Esto significa que Tauri no incluye un navegador: usa el que ya está instalado en el sistema. El resultado: aplicaciones de ~5MB que consumen ~30MB de RAM.

| Aspecto | Electron | Tauri |
|---|---|---|
| Tamaño del binario | ~150-200MB | ~3-10MB |
| Memoria RAM | ~200-500MB | ~20-50MB |
| Backend | Node.js | Rust (nativo) |
| Frontend | HTML/CSS/JS | HTML/CSS/JS o Rust/WASM |
| Seguridad | Baja (Node.js) | Alta (Rust + permisos) |
| Webview | Incluye Chromium (~150MB) | Usa la del SO (~0MB adicionales) |

**¿Por qué alguien usaría Electron entonces?** Porque Electron es más maduro, tiene más documentación, y no requiere Rust. Si tu equipo solo sabe JavaScript, Electron es la opción pragmática. Pero si ya sabes Rust y te importa el rendimiento y el tamaño del binario, Tauri es superior en casi todos los aspectos.

## 2. Configuracion del proyecto

Crear un proyecto Tauri + Yew requiere configurar dos entornos que trabajan juntos: el frontend (Yew compilado a WASM) y el backend (Tauri en Rust nativo). El frontend se sirve desde la webview del sistema operativo, y se comunica con el backend a través de comandos Tauri. La configuración inicial es el paso más crítico porque involucra múltiples herramientas: Trunk para compilar WASM, Tauri CLI para empaquetar la aplicación, y Cargo para las dependencias de ambos lados.

### 2.1 Prerrequisitos e instalacion

```bash
# 1. Verificar Rust
rustc --version          # debe ser 1.82+
cargo --version

# 2. Instalar Trunk (bundler para Yew/WASM)
cargo install trunk

# 3. Añadir target WASM
rustup target add wasm32-unknown-unknown

# 4. Instalar Tauri CLI
cargo install tauri-cli --version "^2"

# 5. Verificar Node.js (necesario para Tauri)
node --version           # 18+
```

**Análisis línea por línea**:

- `cargo install trunk`: Trunk es un bundler para aplicaciones WASM en Rust. Hace el mismo trabajo que Webpack o Vite pero para Rust. Compila tu código Yew a WASM, empaqueta los assets HTML/CSS, y levanta un servidor de desarrollo.

- `rustup target add wasm32-unknown-unknown`: añade el target de compilación para WebAssembly. Sin este target, no puedes compilar Yew porque Yew genera WASM, no código nativo.

- `cargo install tauri-cli --version "^2"`: instala la CLI de Tauri versión 2. La CLI proporciona el comando `cargo tauri` para iniciar, compilar y empaquetar aplicaciones.

### 2.2 Creacion del proyecto

```bash
# Crear el proyecto con Tauri + frontend
cargo tauri init --app-name inventarios-tauri \
    --window-title "Sistema de Inventarios" \
    --dev-url http://localhost:1420 \
    --before-dev-command "trunk serve" \
    --before-build-command "trunk build"
```

Esto crea la siguiente estructura:

```
inventarios-tauri/
├── src-tauri/           ← Backend Rust (Tauri core)
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── src/
│   │   ├── main.rs
│   │   └── lib.rs
│   └── capabilities/
├── src/                 ← Frontend Yew
│   ├── main.rs
│   ├── app.rs
│   └── components/
├── index.html           ← Punto de entrada HTML
├── Trunk.toml           ← Config de Trunk
└── package.json         ← Scripts de desarrollo
```

**Análisis de la estructura**:

- `src-tauri/`: contiene el backend Rust. Aquí se definen los comandos que el frontend puede llamar. También contiene la configuración de la ventana (`tauri.conf.json`) y los permisos de seguridad.

- `src/`: contiene el frontend Yew. Cada archivo es un componente o módulo. `main.rs` es el punto de entrada, `app.rs` es el componente raíz.

- `index.html`: el archivo HTML que la webview carga. Contiene un `<div id="app"></div>` donde Yew monta la aplicación.

- `Trunk.toml`: configura Trunk: puerto, assets, etc.

### 2.3 Dependencias del backend (src-tauri/Cargo.toml)

```toml
[package]
name = "inventarios-tauri"
version = "0.1.0"
edition = "2021"

[lib]
name = "inventarios_tauri_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.8", features = ["runtime-tokio", "mysql"] }
tokio = { version = "1", features = ["full"] }
```

### 2.4 Dependencias del frontend (Cargo.toml raiz)

```toml
[package]
name = "inventarios-frontend"
version = "0.1.0"
edition = "2021"

[dependencies]
yew = { version = "0.21", features = ["csr"] }
yew-router = "0.19"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
gloo-net = "0.6"         # HTTP requests desde WASM
```

**Análisis**:

- `yew = { version = "0.21", features = ["csr"] }`: Yew con renderizado en cliente (Client-Side Rendering). La feature `csr` indica que la aplicación se renderiza en el navegador/webview, no en el servidor.

- `yew-router = "0.19"`: enrutador para Yew, similar a React Router. Permite tener múltiples páginas (Productos, Movimientos, Reportes).

- `wasm-bindgen-futures`: permite usar `async/await` en WASM. Sin esto, no puedes hacer peticiones HTTP ni llamar a comandos de Tauri desde Yew.

- `gloo-net`: cliente HTTP para WASM. Se usa para llamar a comandos Tauri desde el frontend.

### 2.5 index.html: el punto de entrada HTML

El frontend de Yew necesita un archivo HTML que la webview cargue. Trunk procesa este archivo, inyecta el WASM compilado y los assets CSS, y lo sirve. El HTML mínimo contiene un `<div id="app">` donde Yew monta la aplicación, y un link a la hoja de estilos.

```html
<!DOCTYPE html>
<html lang="es">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Sistema de Inventarios</title>
  <!-- Trunk procesa este CSS y lo incrusta en el WASM -->
  <link data-trunk rel="css" href="styles.css">
</head>
<body>
  <!-- Yew monta la aplicación aquí -->
  <div id="app"></div>
</body>
</html>
```

**Análisis línea por línea**:

- `data-trunk rel="css" href="styles.css"`: Trunk busca este atributo y procesa el archivo CSS. Lo optimiza, lo minifica (en producción), y lo empaqueta junto con el WASM. Sin `data-trunk`, tendrías que cargar el CSS manualmente.

- `<div id="app"></div>`: Yew busca este div por su ID y monta la aplicación allí. El contenido del div se reemplaza completamente por el HTML generado por Yew.

## 3. Primer componente en Yew

En Yew, todo es un componente. Un componente es una función que recibe propiedades (props) y devuelve HTML. Pero a diferencia de otros frameworks donde los componentes son clases con métodos de ciclo de vida, en Yew los componentes son funciones que usan **hooks** para gestionar estado, efectos secundarios, y referencias. Este enfoque funcional es similar a React con hooks, pero con la diferencia fundamental de que Yew se ejecuta en WebAssembly y se escribe en Rust.

Cuando dices "componente" en Yew, te refieres a una función marcada con `#[function_component]`. Esta función se ejecuta cada vez que el componente necesita renderizarse, ya sea porque su estado cambió o porque sus propiedades cambiaron. Yew compara el HTML devuelto con el HTML anterior y solo actualiza las partes que cambiaron (esto se llama "reconciliación" y es el mismo concepto del virtual DOM de React).

Los componentes pueden tener:
- **Estado local**: datos que el componente gestiona internamente, creados con `use_state`.
- **Propiedades**: datos que el componente recibe de su padre, definidos con un struct que implementa `Properties`.
- **Efectos secundarios**: operaciones que se ejecutan después del renderizado, como llamadas a APIs o suscripciones.

### 3.1 Componente basico: Contador

El componente "Hola Mundo" de Yew es un contador. Tiene un botón que incrementa un número cada vez que se hace clic. Este ejemplo introduce tres conceptos fundamentales: el hook `use_state` para el estado, `Callback` para los eventos, y la macro `html!` para escribir la interfaz.

```rust
// src/main.rs
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // Estado local: un contador que empieza en 0
    let contador = use_state(|| 0);

    // Callback que se ejecuta al hacer clic en el botón
    let onclick = {
        let contador = contador.clone();
        Callback::from(move |_| contador.set(*contador + 1))
    };

    // HTML que Yew renderiza
    html! {
        <div class="container">
            <h1>{ "Sistema de Inventarios" }</h1>
            <p>{ format!("Has hecho clic {} veces", *contador) }</p>
            <button {onclick}>{ "Incrementar" }</button>
        </div>
    }
}

fn main() {
    // Montar el componente en el <div id="app"> del HTML
    yew::Renderer::<App>::new().render();
}
```

**Análisis línea por línea**:

- `use yew::prelude::*;`: importa todos los tipos y macros que necesitas para escribir componentes Yew. Esto incluye `html!` (la macro para escribir HTML), `use_state` (el hook de estado), `Callback` (para eventos), `Html` (el tipo de retorno), `#[function_component]` (para definir componentes), etc. Sin este `use`, el compilador no encontraría ninguna de estas herramientas.

- `#[function_component(App)]`: esta macro transforma la función `app` en un componente de Yew. El nombre `App` es el identificador del componente. Puedes usarlo en otros componentes con `html! { <App /> }`. La macro genera internamente la implementación del trait `Component` que Yew necesita para gestionar el ciclo de vida y el renderizado.

- `fn app() -> Html {`: la función del componente. No recibe props (por ahora) y debe devolver `Html`. Esta función se ejecuta cada vez que el estado del componente cambia. Yew compara el resultado con el renderizado anterior y aplica los cambios mínimos al DOM real.

- `let contador = use_state(|| 0);`: `use_state` es un hook que crea una variable de estado. Recibe un closure que devuelve el valor inicial (en este caso, `0`). Devuelve un `UseStateHandle<i32>`, que es un tipo que se comporta como una referencia inteligente: puedes leer el valor actual con `*contador` y actualizarlo con `contador.set(nuevo_valor)`.

- `let contador = contador.clone();`: clonamos el handle antes de moverlo al closure del callback. Esto es necesario porque el closure se ejecutará más tarde (cuando el usuario haga clic), y necesita capturar `contador` por movimiento. Si no clonamos, el closure tomaría posesión del handle original y el componente no podría usarlo.

- `Callback::from(move |_| contador.set(*contador + 1))`: crea un callback que incrementa el contador. `Callback::from` envuelve un closure de Rust en un tipo que Yew puede pasar al HTML. El `move` captura las variables por movimiento (necesario porque el closure vive más que la función `app`). El `|_|` ignora el argumento (que es el evento del clic).

- `html! { ... }`: la macro más importante de Yew. Escribe HTML con sintaxis similar a JSX. Las expresiones Rust van entre llaves `{ }`. Los strings literales van entre `{ "..." }`. Los atributos se escriben como en HTML normal. Los eventos se asignan con la sintaxis `{onclick}`.

- `yew::Renderer::<App>::new().render();`: el punto de entrada de la aplicación. Crea un renderizador para el componente `App` y lo monta en el elemento HTML con `id="app"`.

**Salida esperada**:

Al ejecutar `trunk serve` y abrir `http://localhost:1420`:

```
┌──────────────────────────────────────────┐
│  Sistema de Inventarios                  │
│                                          │
│  Has hecho clic 0 veces                  │
│  [Incrementar]                           │
└──────────────────────────────────────────┘
```

Cada clic en el botón incrementa el contador y Yew actualiza solo el texto del párrafo, no toda la página.

### 3.2 Componente con propiedades


Los componentes a menudo necesitan recibir datos de su padre. Por ejemplo, un componente `ProductoCard` necesita el SKU, nombre, precio y stock para mostrar una tarjeta de producto. Estos datos se llaman **props** (propiedades). En Yew, las props se definen con un struct que implementa `Properties` y `PartialEq`.

`Properties` le dice a Yew qué props espera el componente. `PartialEq` le permite a Yew detectar cuándo las props cambiaron para decidir si debe re-renderizar el componente. Si las props no cambiaron, Yew reusa el HTML anterior (esto es una optimización importante para el rendimiento).

```rust
#[derive(Properties, PartialEq)]
struct ProductoProps {
    sku: String,
    nombre: String,
    precio: f64,
    stock: i32,
}

#[function_component(ProductoCard)]
fn producto_card(props: &ProductoProps) -> Html {
    html! {
        <div class="producto-card">
            <h3>{ &props.nombre }</h3>
            <p><strong>SKU:</strong> { &props.sku }</p>
            <p><strong>Precio:</strong> { format!("${:.2}", props.precio) }</p>
            <p class={if props.stock < 10 { "bajo" } else { "ok" }}>
                <strong>Stock:</strong> { props.stock }
            </p>
        </div>
    }
}
```

**Análisis línea por línea**:

- `#[derive(Properties, PartialEq)]`: `Properties` permite que el struct `ProductoProps` se use como props de un componente. Sin este derive, Yew no sabe cómo pasar los datos. `PartialEq` es necesario porque Yew compara las props viejas con las nuevas para decidir si debe re-renderizar.

- `struct ProductoProps { sku: String, nombre: String, precio: f64, stock: i32 }`: todos los campos que el componente necesita. Cuando otro componente use `<ProductoCard sku="LAP-001" nombre="Laptop" precio={18999.0} stock={35} />`, estos valores se asignan a los campos del struct.

- `fn producto_card(props: &ProductoProps) -> Html`: el componente recibe las props como referencia inmutable. No puedes modificar las props: si necesitas cambiarlas, el padre debe pasar props nuevas.

- `class={if props.stock < 10 { "bajo" } else { "ok" }}`: Yew permite expresiones Rust dentro de `{ }`. Aquí elegimos una clase CSS diferente según el nivel de stock. Si hay poco stock, se aplica la clase "bajo" (fondo rojo en el CSS).

**Uso desde otro componente**:

```rust
html! {
    <ProductoCard sku={"LAP-001"} nombre={"Laptop HP"} precio={18999.0} stock={35} />
}
```

Cada campo del struct `ProductoProps` se pasa como un atributo HTML.


### 3.3 Errores tipicos en Yew


**Error 1: olvidar clonar el estado antes del callback**.

```rust
// CODIGO QUE FALLA
let contador = use_state(|| 0);
let onclick = Callback::from(move |_| contador.set(*contador + 1));
```

Mensaje del compilador:
```
error[E0382]: borrow of moved value: `contador`
  --> src/main.rs:10:42
   |
10 |     html! { <button onclick={onclick}>{ "Incrementar" }</button> }
   |                                                       ^^^^^^^^ value borrowed after move
```
Solución: clonar el handle antes de moverlo: `let c = contador.clone();` y mover `c` al closure.

**Error 2: olvidar `#[function_component]`**.

```rust
// CODIGO QUE FALLA
fn App() -> Html {
    html! { <p>{ "Hola" }</p> }
}
```

Mensaje:
```
error[E0277]: the trait bound `App: BaseComponent` is not satisfied
```
Solución: añadir `#[function_component(App)]` encima de la función.

## 4. Comandos Tauri

El concepto central de Tauri es el **comando**. Un comando es una función Rust que el frontend puede llamar de forma remota, como si fuera una API REST pero sin HTTP. La diferencia con una API REST es que la comunicación no pasa por la red: el frontend (WASM) y el backend (nativo) están en el mismo proceso, separados por una capa de seguridad que Tauri gestiona automáticamente.

Cuando el frontend llama a `invoke("listar_productos", {})`, ocurre lo siguiente:
1. Tauri serializa los argumentos a JSON.
2. Tauri pasa el JSON al backend a través del puente IPC (comunicación entre procesos).
3. El backend deserializa los argumentos, ejecuta la función Rust, y serializa el resultado a JSON.
4. Tauri pasa el JSON de vuelta al frontend.
5. El frontend deserializa el resultado y lo usa.

Todo esto ocurre en milisegundos y sin que el desarrollador tenga que escribir una sola línea de serialización manual. Tauri usa `serde` para todo.

Los comandos se definen con el atributo `#[command]` y se registran con `.invoke_handler(tauri::generate_handler![...])`. Pueden ser síncronos o asíncronos, pueden recibir parámetros de cualquier tipo que implemente `Deserialize`, y pueden devolver cualquier tipo que implemente `Serialize`. También pueden acceder al estado compartido de la aplicación a través de `tauri::State<T>`.

### 4.1 Comando basico: saludar

Vamos a crear el comando más simple posible: recibe un nombre y devuelve un saludo. Este comando no necesita base de datos ni estado compartido, solo demuestra la comunicación frontend-backend.

```rust
// src-tauri/src/lib.rs
use tauri::command;

#[command]
fn saludar(nombre: &str) -> String {
    format!("Hola, {}! Bienvenido al sistema de inventarios.", nombre)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![saludar])
        .run(tauri::generate_context!())
        .expect("Error al iniciar la aplicación Tauri");
}
```

**Análisis línea por línea**:

- `use tauri::command;`: importa la macro `command`. Sin ella, no puedes crear comandos.

- `#[command]`: esta macro transforma la función `saludar` en un comando Tauri. Internamente, genera el código necesario para que Tauri pueda: (1) recibir la llamada del frontend, (2) deserializar los argumentos (`nombre` en este caso), (3) ejecutar la función, (4) serializar el resultado, y (5) devolverlo al frontend.

- `fn saludar(nombre: &str) -> String {`: la firma de la función. `nombre` se pasa desde el frontend y se deserializa automáticamente. El tipo de retorno `String` se serializa automáticamente. Si el frontend pasa un argumento que no coincide en tipo o nombre, Tauri devuelve un error de serialización.

- `.invoke_handler(tauri::generate_handler![saludar])`: registra el comando en el builder de Tauri. `generate_handler!` es una macro que genera el código de despacho para todos los comandos listados. Si olvidas registrar un comando aquí, el frontend recibe un error "command not found" cuando intente llamarlo.

- `.run(tauri::generate_context!())`: inicia la aplicación Tauri. `generate_context!()` lee la configuración de `tauri.conf.json` y prepara los recursos necesarios (iconos, permisos, ventanas).

### 4.2 Comando con base de datos

El comando que realmente necesitamos para el sistema de inventarios es el que consulta productos desde MariaDB. Este comando es asíncrono (usa `sqlx`), accede al pool de conexiones compartido (con `tauri::State`), y devuelve una lista de productos.

```rust
use sqlx::MySqlPool;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Producto {
    pub sku: String,
    pub nombre: String,
    pub precio: f64,
    pub stock_minimo: f64,
    pub stock_actual: f64,  // se calcula en la consulta
}

#[command]
async fn listar_productos(pool: tauri::State<'_, MySqlPool>) -> Result<Vec<Producto>, String> {
    let productos = sqlx::query_as::<_, (String, String, f64, f64, f64)>(
        r#"SELECT p.sku, p.nombre, p.precio, p.stock_minimo,
                  COALESCE(SUM(s.cantidad), 0) as stock_actual
           FROM productos p
           LEFT JOIN stock s ON p.sku = s.producto_sku
           GROUP BY p.sku, p.nombre, p.precio, p.stock_minimo
           ORDER BY p.nombre"#
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| format!("Error al consultar productos: {}", e))?
    .into_iter()
    .map(|(sku, nombre, precio, stock_minimo, stock_actual)| Producto {
        sku, nombre, precio, stock_minimo, stock_actual,
    })
    .collect();

    Ok(productos)
}
```

**Análisis línea por línea**:

- `pool: tauri::State<'_, MySqlPool>`: `State` es un extractor de Tauri que permite acceder al estado compartido de la aplicación. El pool de base de datos se registra en el `tauri::Builder` al inicio y Tauri lo inyecta en cualquier comando que lo solicite. `State` es similar a `web::Data<T>` de Actix Web.

- `async fn listar_productos(...) -> Result<Vec<Producto>, String>`: el comando es asíncrono. Tauri ejecuta los comandos asíncronos en el runtime Tokio sin bloquear el hilo principal de la UI. El tipo de retorno es `Result`: si la consulta falla, devolvemos `Err` con el mensaje de error, y Tauri lo envía al frontend como una excepción.

- `sqlx::query_as::<_, (...)>`: ejecuta una consulta SQL y mapea las filas a una tupla. El tipo de la tupla debe coincidir con las columnas del SELECT.

- `pool.inner()`: accede al valor interno de `State`. `State` es un wrapper sobre `Arc<T>`, e `inner()` devuelve una referencia a `T`.

- `.map_err(|e| format!("Error al consultar productos: {}", e))?`: convierte el error de sqlx en un `String` y lo propaga con `?`. Si la BD no responde, el frontend recibe un mensaje descriptivo.

### 4.3 Errores tipicos con comandos

**Error 1: comando no registrado**.

```rust
// CODIGO QUE FALLA
#[command]
fn mi_comando() -> String { "ok".into() }
// falta registrar: .invoke_handler(tauri::generate_handler![mi_comando])
```

El frontend llama a `invoke("mi_comando")` y recibe:
```
Error: command "mi_comando" not found
```
Solución: añadir `mi_comando` al `generate_handler!`.

**Error 2: tipo de retorno no serializable**.

```rust
// CODIGO QUE FALLA
#[command]
fn obtener_fecha() -> std::time::Instant {
    std::time::Instant::now()
}
```

Mensaje: `error: the trait bound Instant: Serialize is not satisfied`. Solución: devolver un tipo serializable como `String` o un struct con `#[derive(Serialize)]`.

## 5. Conectar Yew con Tauri

El paso más importante (y más complejo) de una aplicación Tauri + Yew es la comunicación entre el frontend (Yew en WASM) y el backend (Tauri en Rust nativo). El frontend necesita llamar a los comandos Tauri, y el backend necesita devolver resultados.

La comunicación se hace a través de la función `invoke` que Tauri expone en el objeto global `window.__TAURI__.core.invoke`. Esta función recibe dos argumentos: el nombre del comando (un string) y un objeto JSON con los argumentos. Devuelve una promesa (Promise) que se resuelve con el resultado del comando.

Desde Yew (que se ejecuta en WASM), no podemos acceder directamente a `window.__TAURI__` porque WASM no tiene acceso al objeto `window` de JavaScript. Necesitamos usar `wasm-bindgen` para declarar funciones JavaScript que WASM puede llamar. Esto se hace con el bloque `#[wasm_bindgen] extern "C" { ... }`, que le dice al compilador: "esta función existe en JavaScript, confía en mí y genera el código para llamarla".

El flujo completo es:

```
Yew (Rust/WASM) ──invoke()──► Tauri (JS bridge) ──IPC──► Rust nativo ──► BD
       ▲                                                          │
       └─────────────────────── JSON ─────────────────────────────┘
```

### 5.1 Declarar invoke para WASM

El primer paso es declarar la función `invoke` de Tauri para que WASM pueda llamarla.

```rust
use wasm_bindgen::prelude::*;

// Declarar la función invoke de Tauri para WASM
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
```

**Análisis línea por línea**:

- `use wasm_bindgen::prelude::*;`: importa los tipos necesarios para la interoperabilidad WASM-JavaScript, incluyendo `JsValue` (el tipo que representa cualquier valor JavaScript), `#[wasm_bindgen]`, y `extern "C"`.

- `#[wasm_bindgen] extern "C" { ... }`: este bloque declara funciones JavaScript que existen en el entorno de ejecución. El compilador confía en que estas funciones existen y genera el código WASM para llamarlas. Si la función no existe en tiempo de ejecución, el programa entra en pánico.

- `#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]`: indica la ruta al objeto donde se encuentra la función. En este caso, `window.__TAURI__.core.invoke`. El array anidado indica niveles de anidamiento del objeto.

- `async fn invoke(cmd: &str, args: JsValue) -> JsValue`: declara la función. `cmd` es el nombre del comando Tauri (ej: "listar_productos"). `args` son los argumentos del comando como un objeto JSON (ej: `{"sku": "LAP-001"}`). Devuelve una promesa que se resuelve con el resultado del comando.

### 5.2 Componente que llama a un comando

Con `invoke` declarado, podemos crear un componente que llame al comando `listar_productos` y muestre los resultados.

```rust
use wasm_bindgen_futures::spawn_local;

#[function_component(ProductosPage)]
fn productos_page() -> Html {
    // Estado: lista de productos y bandera de carga
    let productos = use_state(|| Vec::<Producto>::new());
    let cargando = use_state(|| false);

    // Callback que se ejecuta al hacer clic en "Cargar"
    let cargar = {
        let productos = productos.clone();
        let cargando = cargando.clone();
        Callback::from(move |_| {
            let productos = productos.clone();
            let cargando = cargando.clone();

            // spawn_local ejecuta código asíncrono en WASM
            spawn_local(async move {
                cargando.set(true);

                // Preparar argumentos (vacíos en este caso)
                let args = serde_wasm_bindgen::to_value(&serde_json::json!({}))
                    .unwrap();

                // Llamar al comando Tauri
                let resultado = invoke("listar_productos", args).await;

                // Deserializar el resultado
                if let Ok(lista) = serde_wasm_bindgen::from_value::<Vec<Producto>>(resultado) {
                    productos.set(lista);
                }

                cargando.set(false);
            });
        })
    };

    // Renderizar la lista de productos
    html! {
        <div class="productos-page">
            <h2>{ "Productos" }</h2>
            <button onclick={cargar} disabled={*cargando}>
                { if *cargando { "Cargando..." } else { "Cargar Productos" } }
            </button>

            <table>
                <thead>
                    <tr><th>SKU</th><th>Nombre</th><th>Precio</th><th>Stock</th></tr>
                </thead>
                <tbody>
                    { for productos.iter().map(|p| html! {
                        <tr>
                            <td>{ &p.sku }</td>
                            <td>{ &p.nombre }</td>
                            <td>{ format!("${:.2}", p.precio) }</td>
                            <td class={if p.stock_actual < p.stock_minimo { "bajo" } else { "normal" }}>
                                { p.stock_actual }
                            </td>
                        </tr>
                    }) }
                </tbody>
            </table>
        </div>
    }
}
```

**Análisis línea por línea**:

- `use wasm_bindgen_futures::spawn_local;`: `spawn_local` ejecuta un bloque `async` en el contexto WASM. Sin esto, no podrías usar `await` dentro de un callback de clic. Es el equivalente de `tokio::spawn` pero para WASM.

- `let productos = use_state(|| Vec::<Producto>::new());`: estado que almacena la lista de productos. Inicialmente vacío. Se actualiza cuando el comando Tauri devuelve resultados.

- `let args = serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap();`: convierte un objeto JSON (en este caso vacío) a `JsValue`, que es lo que `invoke` espera como argumento. `serde_wasm_bindgen` es el puente entre los tipos Rust y `JsValue`.

- `let resultado = invoke("listar_productos", args).await;`: llama al comando Tauri. El `await` espera a que el comando se ejecute en el backend nativo y devuelva el resultado. Durante la espera, la UI no se congela porque WASM es asíncrono.

- `serde_wasm_bindgen::from_value::<Vec<Producto>>(resultado)`: convierte el `JsValue` devuelto por Tauri en un `Vec<Producto>`. Si el tipo no coincide (por ejemplo, Tauri devolvió un error), la conversión falla y el vector no se actualiza.

### 5.3 Errores tipicos

**Error 1: invoke no encontrado**.

```rust
// CODIGO QUE FALLA
#[wasm_bindgen]
extern "C" {
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;  // falta js_namespace
}
```

Mensaje en consola del navegador:
```
Error: invoke is not defined
```
Solución: añadir `#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]`.

**Error 2: comando Tauri no registrado**.

```rust
// CODIGO QUE FALLA
let resultado = invoke("comando_inexistente", args).await;
```

Tauri devuelve: `Error: command "comando_inexistente" not found`. Solución: registrar el comando en `generate_handler!` en el backend.

**Error 3: tipo de retorno incorrecto**.

Si el comando Tauri devuelve `Result<Vec<Producto>, String>` pero intentas deserializar como `Vec<OtroTipo>`:
```
Error: JsValue cannot be deserialized to OtroTipo
```
Solución: asegurar que el tipo en el frontend coincide exactamente con el tipo del backend.

## 6. Aplicacion completa: Dashboard de inventarios

Hasta ahora hemos visto piezas sueltas: un componente contador, un comando Tauri, una llamada `invoke`. Ahora vamos a integrar todo en una aplicación funcional: un dashboard de inventarios con tres secciones (Dashboard, Productos, Movimientos) y navegación entre ellas.

La aplicación usará `yew_router` para la navegación entre páginas y llamará a comandos Tauri desde cada componente. El backend Tauri expondrá tres comandos: `listar_productos`, `obtener_productos_stock_bajo`, y `registrar_movimiento`.

### 6.1 Backend: comando para stock bajo

Este comando es similar al de listar productos, pero añade una cláusula `HAVING` para filtrar solo aquellos productos cuyo stock actual está por debajo del mínimo. Es útil para la sección de alertas del dashboard.

```rust
#[command]
async fn obtener_productos_stock_bajo(
    pool: tauri::State<'_, MySqlPool>
) -> Result<Vec<Producto>, String> {
    let productos = sqlx::query_as::<_, (String, String, f64, f64, f64)>(
        r#"SELECT p.sku, p.nombre, p.precio, p.stock_minimo,
                  COALESCE(SUM(s.cantidad), 0) as stock_actual
           FROM productos p
           LEFT JOIN stock s ON p.sku = s.producto_sku
           GROUP BY p.sku, p.nombre, p.precio, p.stock_minimo
           HAVING stock_actual < p.stock_minimo
           ORDER BY stock_actual ASC"#
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| format!("Error BD: {}", e))?
    .into_iter()
    .map(|(sku, nombre, precio, stock_minimo, stock_actual)| Producto {
        sku, nombre, precio, stock_minimo, stock_actual,
    })
    .collect();

    Ok(productos)
}
```

**Análisis**: la cláusula `HAVING stock_actual < p.stock_minimo` filtra los productos cuyo stock está por debajo del mínimo. `COALESCE(SUM(s.cantidad), 0)` suma el stock de todas las filas de stock para ese producto, y si no hay ninguna (LEFT JOIN), devuelve 0.

### 6.2 Frontend: componente de alertas con carga automatica

El componente `AlertasStock` se carga automáticamente al montarse (sin necesidad de que el usuario haga clic en un botón), usando el hook `use_effect_with_deps`.

```rust
#[function_component(AlertasStock)]
fn alertas_stock() -> Html {
    let productos = use_state(|| vec![]);

    // useEffect: se ejecuta UNA VEZ cuando el componente se monta
    {
        let productos = productos.clone();
        use_effect_with_deps(move |_| {
            let productos = productos.clone();
            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap();
                let resultado = invoke("obtener_productos_stock_bajo", args).await;
                if let Ok(lista) = serde_wasm_bindgen::from_value::<Vec<Producto>>(resultado) {
                    productos.set(lista);
                }
            });
        }, ()); // El array vacío () significa "solo ejecutar una vez al montar"
    }

    html! {
        <div class="alertas">
            <h2>{ format!("Alertas de Stock Bajo ({})", productos.len()) }</h2>
            { for productos.iter().map(|p| html! {
                <div class="alerta-roja">
                    <strong>{ &p.nombre }</strong>
                    { format!(" - Stock: {:.0} (Mínimo: {:.0})", p.stock_actual, p.stock_minimo) }
                </div>
            }) }
        </div>
    }
}
```

**Análisis línea por línea**:

- `use_effect_with_deps(move |_| { ... }, ())`: hook que ejecuta un closure después de que el componente se renderice. El segundo argumento `()` es la lista de dependencias: cuando está vacía, el efecto solo se ejecuta una vez, cuando el componente se monta. Es el equivalente de `useEffect(() => {}, [])` en React.

- `spawn_local(async move { ... })`: ejecuta una tarea asíncrona. Necesario porque el closure de `use_effect_with_deps` no puede ser `async` directamente.

- El resultado: cuando el usuario navega a la página Dashboard, el componente `AlertasStock` se monta, ejecuta el efecto, llama al comando Tauri, y muestra las alertas automáticamente.

### 6.3 Routing y main.rs final

El archivo `main.rs` unifica todo: define las rutas de la aplicación con `yew_router`, monta los componentes en cada ruta, y proporciona la navegación.

```rust
use yew::prelude::*;
use yew_router::prelude::*;

// Definición de rutas con yew_router
#[derive(Clone, Routable, PartialEq)]
enum Ruta {
    #[at("/")]
    Dashboard,
    #[at("/productos")]
    Productos,
    #[at("/movimientos")]
    Movimientos,
}

// Función que renderiza el componente según la ruta activa
fn switch(ruta: Ruta) -> Html {
    match ruta {
        Ruta::Dashboard => html! { <AlertasStock /> },
        Ruta::Productos => html! { <ProductosPage /> },
        Ruta::Movimientos => html! { <h2>{ "Movimientos - Próximamente" }</h2> },
    }
}

// Componente raíz de la aplicación
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <nav class="navbar">
                <Link<Ruta> to={Ruta::Dashboard}>{ "Dashboard" }</Link<Ruta>>
                <Link<Ruta> to={Ruta::Productos}>{ "Productos" }</Link<Ruta>>
                <Link<Ruta> to={Ruta::Movimientos}>{ "Movimientos" }</Link<Ruta>>
            </nav>
            <main>
                <Switch<Ruta> render={switch} />
            </main>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

**Análisis línea por línea**:

- `#[derive(Clone, Routable, PartialEq)] enum Ruta { ... }`: define las rutas de la aplicación. `Routable` es un trait de `yew_router` que permite que el enum se use como ruta. `#[at("/productos")]` asocia la variante `Productos` con la URL `/productos`.

- `fn switch(ruta: Ruta) -> Html { match ruta { ... } }`: función que recibe la ruta actual y devuelve el componente correspondiente. Yew Router llama a esta función cada vez que la URL cambia.

- `BrowserRouter`: componente que envuelve toda la aplicación y proporciona el contexto de enrutamiento. Sin él, `Link` y `Switch` no funcionan.

- `Link<Ruta> to={Ruta::Productos}`: componente que genera un enlace `<a href="/productos">`. Cuando el usuario hace clic, Yew Router intercepta la navegación y cambia la ruta sin recargar la página.

## 7. Compilacion y empaquetado

El ciclo de desarrollo de Tauri tiene dos modos: desarrollo (con recarga en caliente) y producción (con optimizaciones y empaquetado para distribución).

En modo desarrollo, `cargo tauri dev` inicia dos procesos simultáneamente: Trunk sirve el frontend Yew en `http://localhost:1420` con recarga automática cada vez que guardas un archivo, y Tauri abre una ventana nativa que carga esa URL. Los cambios en el frontend se ven al instante sin recompilar el backend. Los cambios en el backend Rust requieren reiniciar la aplicación.

En modo producción, `cargo tauri build` compila el frontend Yew a WASM optimizado, compila el backend Rust en modo release, une ambos en un solo binario, y genera un instalador nativo (`.msi` para Windows, `.dmg` para macOS, `.deb` o `.AppImage` para Linux). El resultado es un instalador de menos de 10MB que incluye la aplicación completa.

```bash
# 1. Desarrollo con recarga en caliente
# Trunk sirve el frontend en :1420, Tauri abre la ventana
cargo tauri dev

# 2. Compilación para producción
# Genera el binario optimizado y el instalador
cargo tauri build

# 3. El instalador se genera en:
ls src-tauri/target/release/bundle/
# Resultado: inventarios-tauri_0.1.0_amd64.deb (Linux)
#           inventarios-tauri_0.1.0_x64.msi   (Windows)
#           inventarios-tauri_0.1.0_x64.dmg   (macOS)
```

**Análisis de cada comando**:

- `cargo tauri dev`: inicia el servidor de desarrollo de Trunk en `:1420`, compila el backend y abre la ventana de la aplicación. Cualquier cambio en `src/` (frontend Yew) se refleja automáticamente gracias al hot reload de Trunk.

- `cargo tauri build`: compila todo para producción. El frontend Yew se compila con optimizaciones (`--release`), el backend Tauri también, y `tauri-bundler` genera el instalador nativo según el sistema operativo.

## 8. Errores tipicos

**Error 1: comando no registrado en generate_handler!**.

```rust
// CÓDIGO QUE FALLA
#[command]
fn mi_comando() -> String { "ok".into() }
// No se registró en invoke_handler
```

El frontend llama a `invoke("mi_comando")` y recibe en la consola:
```
Error: command "mi_comando" not found
```
**Causa**: el comando existe como función Rust pero no fue registrado en el builder de Tauri. Tauri solo expone los comandos que están listados en `generate_handler!`.
**Solución**: añadir `mi_comando` a la lista de `generate_handler!`:
```rust
.invoke_handler(tauri::generate_handler![saludar, mi_comando])
```

**Error 2: olvidar la feature "csr" en Yew**.

```toml
# CÓDIGO QUE FALLA
[dependencies]
yew = { version = "0.21" }  # falta features = ["csr"]
```

Mensaje del compilador:
```
error[E0277]: the trait bound `App: BaseComponent` is not satisfied
```
**Causa**: Yew necesita la feature `csr` (Client-Side Rendering) para poder renderizar componentes en el navegador o webview. Sin ella, Yew compila para el servidor (SSR), que no es compatible con Tauri.
**Solución**: añadir `features = ["csr"]`:
```toml
yew = { version = "0.21", features = ["csr"] }
```

**Error 3: webview no encontrada en Linux**.

```bash
cargo tauri dev
```

Mensaje:
```
Error: Failed to create webview: `webkit2gtk-4.1` not found
```
**Causa**: Tauri en Linux necesita WebKit2GTK para renderizar la webview. Si no está instalado, Tauri no puede abrir la ventana.
**Solución**: instalar las dependencias:
```bash
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev
```

## 9. Ejercicios

1. Agregar un comando Tauri que inserte un nuevo producto en la BD.
2. Crear un componente Yew que muestre un formulario para registrar entradas de stock.
3. Implementar la navegación entre la página de productos y la página de movimientos.
4. Agregar una alerta visual cuando el stock de un producto esté por debajo del mínimo.
5. Compilar la aplicación para producción y generar el instalador.

## 10. Soluciones

Las soluciones detalladas están en `proyectos_tauri_yew/src/` con los componentes completos.
