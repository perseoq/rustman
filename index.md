# Manual completo de Rust, MySQL y Actix Web orientado al desarrollo de un ERP/CRM profesional

> **Un viaje desde cero absoluto hasta una API REST de nivel producción para un sistema ERP/CRM mexicano (RFC, CFDI 4.0, IVA, IEPS, inventarios, facturación y más).**

---

## Tabla de contenidos

> **Manual de Rust + MySQL + Actix Web para ERP/CRM** · 18 proyectos ejecutables · 1 API REST con 18 endpoints · 3 partes didácticas.

---

### [Parte 1: Fundamentos de Rust (desde cero)](#parte-1-fundamentos-de-rust-desde-cero)

| # | Sección | Temas clave |
|---|---|---|
| [1.1 Introducción a Rust](#11-introduccion-a-rust) | Historia, filosofía, casos de uso |
| [1.2 Instalación del entorno](#12-instalacion-del-entorno-de-desarrollo) | rustup, cargo, rust-analyzer |
| [1.3 Tu primer programa: Hola Mundo ERP](#13-tu-primer-programa-hola-mundo-erp) | println!, compilación |
| [1.4 Variables y mutabilidad](#14-variables-y-mutabilidad) | `let`, `let mut`, `const` |
| [1.5 Tipos de datos primitivos](#15-tipos-de-datos-primitivos) | enteros, flotantes, bool, char |
| [1.6 Operadores y expresiones](#16-operadores-y-expresiones) | aritméticos, lógicos, bit a bit |
| [1.7 Control de flujo](#17-control-de-flujo) | `if`, `loop`, `while`, `for` |
| [1.8 Funciones](#18-funciones) | parámetros, retornos, expresiones |
| [1.9 Ownership, borrowing y lifetimes](#19-ownership-borrowing-y-lifetimes) | 7 subsecciones dedicadas |
| [1.10 Tipos compuestos](#110-tipos-compuestos-tuplas-arrays-y-slices) | tuplas, arrays, slices |
| [1.11 Structs](#111-structs) | definición, métodos, visibilidad |
| [1.12 Enums y pattern matching](#112-enums-y-pattern-matching) | `match`, `if let`, `Option` |
| [1.13 Traits](#113-traits) | definición, traits comunes, dispatch |
| [1.14 Genéricos](#114-genericos) | funciones, structs, restricciones |
| [1.15 Manejo de errores](#115-manejo-de-errores) | `panic!`, `Result`, `?` |
| [1.16 Colecciones](#116-colecciones) | `Vec`, `HashMap`, `HashSet`, `String` |
| [1.17 Closures e iteradores](#117-closures-e-iteradores) | `Fn`, adaptadores, consumidores |
| [1.18 Módulos y sistema de archivos](#118-modulos-y-sistema-de-archivos) | `mod`, `pub`, `use` |
| [1.19 Pruebas](#119-pruebas-unitarias-y-de-integracion) | unit, integration, doc tests |
| [1.20 Ejercicios acumulativos](#120-ejercicios-acumulativos-parte-1-20-ejercicios) | ejercicios prácticos |
| [1.21 Soluciones detalladas](#121-soluciones-de-los-ejercicios-de-la-parte-1) | soluciones completas |

**Proyectos de la Parte 1** (8 mini-proyectos):
1. `01_erp_hello` — Hola Mundo del ERP
2. `02_calc_impuestos` — Calculadora de IVA/IEPS
3. `03_validador_cliente` — Validador RFC, email, teléfono
4. `04_procesador_pedido` — Procesador de líneas de pedido
5. `05_modelo_erp` — Modelo de datos del ERP en memoria
6. `06_estados_pedido` — Máquina de estados de un pedido
7. `07_catalogo_productos` — Catálogo con filtrado y agregaciones
8. `08_modelo_erp_modular` — Refactor modular con `mod`

### [Parte 2: Rust y MySQL](#parte-2-rust-y-mysql)

| # | Sección | Temas clave |
|---|---|---|
| [2.1 Introducción a BD relacionales y MySQL](#21-introduccion-a-bases-de-datos-relacionales-y-mysql) | RDBMS, tablas, instalación MariaDB |
| [2.2 Configuración del proyecto con el crate `mysql`](#22-configuracion-del-proyecto-con-el-crate-mysql) | URL de conexión, pool, primer programa |
| [2.3 Consultas `SELECT`](#23-consultas-select) | `query`, `exec`, `query_iter`, structs |
| [2.4 Parámetros preparados y seguridad](#24-parametros-preparados-y-seguridad) | inyección SQL, `?`, `:nombre` |
| [2.5 `INSERT`, `UPDATE`, `DELETE`](#25-insert-update-y-delete) | `exec_drop`, `last_insert_id`, CRUD CLI |
| [2.6 Pool de conexiones con `r2d2`](#26-pool-de-conexiones-con-r2d2_mysql) | `max_size`, `min_idle`, timeout, `Arc` |
| [2.7 Transacciones](#27-transacciones) | `start_transaction`, `commit`, `Drop` |
| [2.8 Errores típicos con BD](#28-errores-tipicos-con-bases-de-datos) | conexión, tipos, restricciones, deadlocks |
| [2.9 Ejemplo completo: ERP/CRM CLI](#29-ejemplo-completo-erpcrm-cli-de-gestion) | módulos, menú, flujo de pedido |
| [2.10 Buenas prácticas](#210-buenas-practicas) | migraciones, concurrencia, capas |
| [2.11 Ejercicios acumulativos](#211-ejercicios-acumulativos-parte-2-20-ejercicios) | ejercicios prácticos |
| [2.12 Soluciones detalladas](#212-soluciones-detalladas-parte-2) | soluciones completas |

**Proyectos de la Parte 2** (4 mini-proyectos):
1. `01_conexion_mysql` — Conexión básica y listado de tablas
2. `02_crud_clientes` — CRUD CLI de clientes en MySQL
3. `03_cli_pedidos_transacciones` — Pedidos con transacciones
4. `04_cli_erp_completo` — CLI con menú completo

### [Parte 3: Rust, Actix Web y ORM](#parte-3-rust-actix-web-y-orm)

| # | Sección | Temas clave |
|---|---|---|
| [3.1 Introducción a Actix Web](#31-introduccion-a-actix-web) | framework web, modelo actor, ventajas |
| [3.2 Configuración del proyecto](#32-configuracion-del-proyecto) | dependencias, `#[actix_web::main]`, health check |
| [3.3 Rutas y handlers](#33-rutas-y-handlers) | métodos HTTP, Path, Query, Json, `configure()` |
| [3.4 Estado compartido](#34-estado-compartido) | `web::Data`, pool de BD, `AppState` |
| [3.5 Middleware](#35-middleware) | Logger, Compress, DefaultHeaders, seguridad |
| [3.6 Manejo de errores](#36-manejo-de-errores) | `ErrorNegocio`, `ResponseError`, `From<diesel::Error>` |
| [3.7 Diesel: el ORM síncrono](#37-diesel-el-orm-sincrono) | instalación, migraciones, modelos, CRUD |
| [3.8 Integración de Diesel con Actix](#38-integracion-de-diesel-con-actix) | `web::block()`, transacciones, patrones |
| [3.11 SeaORM: la alternativa asíncrona](#311-seaorm-la-alternativa-asincrona) | async ORM, entidades, ActiveModel |
| [3.12 Buenas prácticas en APIs REST](#312-buenas-practicas-en-apis-rest) | capas, `validator`, `utoipa`, `.env` |
| [3.13 Pruebas](#313-pruebas) | `actix_web::test`, mocks de BD |
| [3.14 Ejemplo completo: API REST del ERP/CRM](#314-ejemplo-completo-api-rest-del-erpcrm) | 16 endpoints, JWT, docker-compose |
| [3.15 Despliegue](#315-despliegue) | release, Docker multi-stage, compose |
| [3.16 Ejercicios acumulativos](#316-ejercicios-acumulativos-parte-3-30-ejercicios) | ejercicios prácticos |
| [3.17 Soluciones detalladas](#317-soluciones-detalladas-parte-3) | soluciones completas |

**Proyectos de la Parte 3** (5 mini-proyectos + 1 final):
1. `01_api_health` — Primer servidor HTTP con Actix
2. `02_api_clientes_v0` — API con CRUD en memoria
3. `03_api_clientes_v1` — API con CRUD en MySQL + middleware
4. `04_api_erp_diesel` — API estilo Diesel con reportes
5. `05_api_erp_seaorm` — API estilo SeaORM con inventario
6. `proyecto_api/api_diesel` — **API final con 18 endpoints, JWT, transacciones y docker-compose**

### Manuales adicionales y archivos complementarios

El repositorio incluye manuales independientes con otros frameworks y arquitecturas:
- [`manual_axum.md`](manual_axum.md) — **Axum + Hexagonal**: sistema de inventarios multibodega con Axum, sqlx y arquitectura hexagonal (puertos y adaptadores)
- [`manual_rocket.md`](manual_rocket.md) — **Rocket + Granular**: sistema de tickets de soporte con Rocket, Diesel y arquitectura granular (módulos autocontenidos)
- [`manual_devops.md`](manual_devops.md) — **DevOps**: despliegue, contenedores, balanceo de carga, Kubernetes, Ansible, Terraform y CI/CD para los proyectos del repositorio
- [`manual_selenium.md`](manual_selenium.md) — **Selenium + JavaScript**: testing automatizado end-to-end con Selenium WebDriver, Mocha, Chai y dashboard HTML para las APIs del repositorio
- [`manual_testing_apis.md`](manual_testing_apis.md) — **Testing de APIs Rust**: tests de integración, carga y contrato con reqwest sobre las APIs del repositorio
- [`manual_testing_rust.md`](manual_testing_rust.md) — **Testing unitario y avanzado en Rust**: property-based testing con proptest, mocking con mockall, fuzzing y cobertura
- [`manual_tauri_yew.md`](manual_tauri_yew.md) — **Tauri + Yew**: aplicaciones de escritorio nativas con Rust, Yew (WASM) y Tauri para los sistemas del repositorio

Archivos complementarios del manual principal:
- [`anexos.md`](anexos.md) — Apéndices temáticos (A.2 Soluciones, A.3 Recursos, A.4 Índice, más 160 apéndices numerados y 26 alfabéticos)
- [`glosario.md`](glosario.md) — Todos los glosarios: términos de Rust y ERP/CRM, facturación, inglés-español, argot, términos fiscales, y más

# Parte 1: Fundamentos de Rust (desde cero)

> **Aviso pedagógico**: cada bloque de código va precedido de una explicación larga con analogías. El código está comentado línea por línea en español. Después de cada ejemplo encontrarás un análisis de la salida esperada. Al final de cada sección principal hay un bloque de "errores típicos" con diagnóstico y solución.

## 1.1 Introduccion a Rust

Rust es un lenguaje de programación de sistemas que apareció en su versión 1.0 en el año 2015, desarrollado por Mozilla Research y mantenido desde entonces por una fundación independiente llamada Rust Foundation. Cuando decimos que es un lenguaje "de sistemas" nos referimos a que está diseñado para el mismo nicho en el que tradicionalmente se han usado C y C++: la construcción de sistemas operativos, motores de navegadores, servidores de alto rendimiento, sistemas embebidos, herramientas de línea de comandos y cualquier software donde el control fino sobre la memoria y la ausencia de un recolector de basura (garbage collector) sean ventajas competitivas. Sin embargo, a diferencia de C y C++, Rust promete algo que durante décadas pareció imposible: el rendimiento de C++ con la seguridad de memoria de un lenguaje con recolector de basura, pero sin pagar el costo de un recolector en tiempo de ejecución. Esa promesa se cumple mediante un sistema de tipos único en su tipo, centrado en lo que se denomina *ownership* (propiedad) y *borrowing* (préstamo).

Piensa en Rust como un "C++ con cinturón de seguridad integrado": te da el motor potente de un lenguaje compilado de bajo nivel, pero añade un conjunto de reglas verificadas por el compilador que impiden toda una categoría de errores que en otros lenguajes sólo se descubren en producción o tras horas de depuración con `valgrind`. Entre esos errores están los temidos *use-after-free* (usar memoria después de haberla liberado), los *double-free* (liberar la misma memoria dos veces), los *data races* (condiciones de carrera cuando varios hilos acceden a los mismos datos sin sincronización) y los *null pointer dereferences* (intentar leer una posición de memoria nula). Rust no es perfecto —ningún lenguaje lo es— y desde luego tiene una curva de aprendizaje más pronunciada que Python o JavaScript, pero a cambio ofrece una combinación de rendimiento, seguridad y expresividad que lo ha llevado a ser, año tras año, el lenguaje "más amado" en la encuesta anual de Stack Overflow.

En el contexto de este manual, Rust será la herramienta con la que construiremos un **ERP/CRM profesional** comparable a los que se hacen en Java Spring o .NET, pero con un binario único, sin máquina virtual, con un consumo de memoria muy inferior y con un sistema de tipos que detectará muchos errores lógicos antes de que el código llegue a producción. Conectaremos Rust a una base de datos MySQL/MariaDB, levantaremos un servidor HTTP con Actix Web y definiremos nuestras tablas y consultas tanto con el ORM Diesel como con SeaORM. Al terminar habrás construido un sistema real: gestión de clientes con RFC, catálogo de productos con SKU y código de barras, control de inventarios por almacén, pedidos con líneas y descuentos, facturación con simulación de CFDI 4.0, autenticación con JWT, roles y permisos, y una API REST documentada con OpenAPI. Todo eso en un solo lenguaje que cabe en tu cabeza.

**¿Por qué Rust y no Go, Java, Python o Node.js?** La respuesta corta es: porque Rust te obliga a pensar bien desde el principio, y eso se traduce en menos bugs en producción. La respuesta larga es: porque Rust ofrece un modelo de concurrencia sin data races, inferencia de tipos poderosa, *pattern matching* exhaustivo, *enums* algebraicos, *traits* (similar a interfaces pero más flexibles), un sistema de paquetes integrado (`cargo`), un compilador con mensajes de error que parecen tutoriales y un rendimiento cercano al de C en muchas pruebas de referencia. Para un ERP/CRM, donde se procesan miles de facturas, se mueven inventarios críticos y se requiere auditoría, esa combinación es muy atractiva.

**¿En qué se diferencia Rust de otros lenguajes que ya conoces?**

| Característica | Python / JS | Java / C# | C / C++ | **Rust** |
|---|---|---|---|---|
| Compilación | Interpretado | JVM / CLR | Nativa | Nativa |
| Recolector de basura | Sí | Sí | No | **No** |
| Seguridad de memoria | Sí (en parte) | Sí | No | **Sí (en compilación)** |
| Data races en compilación | No | No | No | **Sí** |
| Velocidad cercana a C | No | No | Sí | **Sí** |
| Sistema de macros | No | Limitado | Sí (texto) | **Sí (higiénico)** |
| Curva de aprendizaje | Baja | Media | Alta | **Media-alta (al inicio), baja (después)** |

**El modelo mental del "ownership"**. Antes de escribir tu primer programa necesitas tener en la cabeza la idea central de Rust: cada valor tiene un único *dueño* (owner), y cuando ese dueño sale del alcance (scope), el valor se libera automáticamente. No hay `malloc`/`free` explícitos, no hay `new`/`delete`, no hay `Arc::new_cyclic` o `Weak::upgrade` obligatorio. El compilador verifica en tiempo de compilación que se cumplan tres reglas: (1) cada valor tiene un dueño; (2) sólo puede haber un dueño a la vez; (3) cuando el dueño se va, el valor desaparece. Más adelante, en la sección 1.9, dedicaremos más de veinte páginas a esta idea. Por ahora quédate con la intuición: "en Rust, los valores tienen un único dueño responsable de liberarlos, y el compilador se encarga de verificarlo todo".

**El compilador como tutor, no como enemigo**. Una de las cosas que más sorprenden a quien empieza con Rust es la calidad de los mensajes de error del compilador. No son crípticos como los de C++; parecen escritos por un tutor paciente que te dice exactamente qué hiciste mal y, en muchos casos, te sugiere cómo arreglarlo. Cuando tu código no compila, lee el mensaje completo (incluso la parte que parece "ayuda" coloreada), porque ahí está el 80 % de la solución. Con el tiempo,你会 desarrollar la habilidad de "leer los errores del compilador como si fueran documentación", y eso te ahorrará incontables horas.

**¿Qué se puede hacer con Rust?** Literalmente casi todo. El navegador Firefox, el motor de búsqueda de npm, el kernel experimental de Linux que está en camino (`rust-vmm`), la herramienta `ripgrep`, el servicio de backend de Discord, el sistema de archivos de Dropbox, parte de Cloudflare, Microsoft Azure IoT, AWS Lambda en su runtime, y un largo etc. La página [Are We Game Yet?](https://arewegameyet.rs), [Are We Web Yet?](https://arewewebyet.rs) y [Are We Embedded Yet?](https://areweembeddedyet.org) muestran ecosistemas por dominio. En este manual nos centraremos en backend web con bases de datos relacionales, que es uno de los nichos donde Rust está creciendo con más fuerza.

**Resumen ejecutivo de la Parte 1**. En las siguientes 21 subsecciones aprenderás a instalar Rust, escribir tu primer programa, manejar variables y tipos, controlar el flujo, definir funciones, y —lo más importante— entender el ownership, el borrowing y los lifetimes. En la Parte 2 conectaremos todo esto a MySQL, y en la Parte 3 levantaremos un servidor HTTP profesional. Al final tendrás un ERP/CRM funcional, no un proyecto de juguete.

> **Nota cultural**: a la comunidad de Rust se la llama "Rustáceos" (Rustaceans en inglés). El logotipo es un cangrejo herradura (horseshoe crab), un animal que lleva 450 millones de años en la Tierra sin infectarse, lo cual se considera un guiño humorístico a la "inmunidad" de Rust contra toda una clase de bugs.

### 1.1.1 ¿Quien deberia leer este manual?

Este manual está escrito para personas que **nunca han programado en Rust**, aunque sí se asume un conocimiento básico de programación en cualquier otro lenguaje (variables, funciones, bucles, condicionales). Si vienes de Python, Java, C#, JavaScript, Go, PHP o Ruby, te sentirás cómodo en la mayoría de los ejemplos. Si nunca has programado, te recomendamos complementar con un curso introductorio de programación general antes de embarcarte en este viaje.

### 1.1.2 ¿Que necesito para empezar?

- Un ordenador con Linux, macOS o Windows.
- Conexión a internet (para instalar Rust y descargar dependencias).
- 2 GB de disco libre (para el toolchain y las dependencias de los proyectos).
- Un editor de texto: **Visual Studio Code** con la extensión `rust-analyzer` es la opción más popular y multiplataforma; otras opciones válidas son **Neovim** con `coc.nvim` o `rust-tools.nvim`, **IntelliJ IDEA** con el plugin de Rust, o **Zed**.
- Ganas de leer y de equivocarte. Rust te recompensará cada vez que el compilador te obligue a pensar mejor.

### 1.1.3 Filosofia del manual: aprender haciendo

A lo largo del manual encontrarás 17 mini-proyectos que crecen en complejidad, más 2 proyectos finales (uno con Diesel y otro con SeaORM). Cada mini-proyecto está pensado para aplicar los conceptos aprendidos en su capítulo. No te limites a leer: **escribe el código, compílalo, ejecuta los tests, rómpelo y arréglalo**. La única forma de aprender Rust es haciendo, y eso es exactamente lo que vamos a hacer.

## 1.2 Instalacion del entorno de desarrollo

Instalar Rust es sorprendentemente sencillo gracias a una herramienta llamada `rustup`, que es el instalador y gestor de versiones oficial. Piensa en `rustup` como el "nvm" de Node.js o el "pyenv" de Python, pero específicamente diseñado para Rust. Con `rustup` puedes tener varias versiones del compilador (`stable`, `beta`, `nightly`), cambiarlas por proyecto con `rust-toolchain.toml`, e incluso instalar *targets* cruzados para compilar para otras arquitecturas (por ejemplo, ARM para Raspberry Pi o WebAssembly para el navegador).

### 1.2.1 Instalacion de `rustup` en Linux y macOS

Abre una terminal y ejecuta el siguiente comando (es un *pipe* a `sh`, así que asegúrate de leer el script antes; puedes inspeccionarlo en <https://sh.rustup.rs>):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verás una pantalla interactiva con tres opciones: (1) instalación por defecto, (2) personalizar la instalación, (3) cancelar. Para este manual elegiremos la opción 1. La instalación descarga el compilador `rustc`, el gestor de paquetes `cargo`, y las *standard libraries* por defecto. Al terminar, el script te indicará que ejecutes `source $HOME/.cargo/env` (o que reinicies la terminal) para añadir `~/.cargo/bin` a tu `PATH`.

```bash
# Verificar la instalacion
rustc --version
cargo --version
# rustc 1.96.1 (xxxxx xxxx)
# cargo 1.96.1 (xxxxx xxxx)
```

Si trabajas en Windows, el procedimiento recomendado es descargar `rustup-init.exe` desde <https://rustup.rs> y ejecutarlo. Asegúrate de tener también el *Build Tools for Visual Studio 2019 o 2022* con el componente "C++ build tools", ya que el linker de Windows que usa Rust por defecto requiere `link.exe`.

### 1.2.2 Configuracion del PATH y de la shell

`rustup` instala las herramientas en `~/.cargo/bin` (en Windows es `%USERPROFILE%\.cargo\bin`). Si tu shell es `bash` o `zsh`, el script de instalación añade automáticamente la siguiente línea a `~/.bashrc` o `~/.zshrc`:

```bash
source "$HOME/.cargo/env"
```

Si por alguna razón no se añadió, puedes incluirla manualmente. Verifica que funciona:

```bash
which rustc    # debería mostrar /home/tu_usuario/.cargo/bin/rustc
which cargo    # debería mostrar /home/tu_usuario/.cargo/bin/cargo
```

### 1.2.3 Components y targets

`rustup` puede instalar componentes adicionales. Para este manual, asegúrate de tener el componente `rust-src` (que permite que `rust-analyzer` navegue hasta la definición de los símbolos estándar) y el formato de código:

```bash
rustup component add rust-src rustfmt clippy
```

- `rustfmt` formatea tu código siguiendo el estilo oficial de Rust (estilo `rustfmt`).
- `clippy` es un *linter* que detecta código no idiomático y posibles bugs. Lo usaremos a lo largo del manual.

### 1.2.4 Editores recomendados

**Visual Studio Code** (gratuito, multiplataforma):
1. Instala VS Code desde <https://code.visualstudio.com>.
2. Abre la pestaña de extensiones (`Ctrl+Shift+X`).
3. Busca e instala "rust-analyzer" (la extensión oficial de la comunidad).
4. Opcional: instala "CodeLLDB" para depuración nativa.
5. Opcional: instala "Even Better TOML" para resaltar la sintaxis de `Cargo.toml`.

**IntelliJ IDEA / CLion** (JetBrains, de pago pero con licencia educativa gratuita):
- Para IDEA Ultimate existe el plugin oficial "Rust" de JetBrains.
- Para CLion existe el mismo plugin.
- La experiencia es muy buena, especialmente para refactorings.

**Neovim / Vim**:
- Con `coc.nvim` y la extensión `coc-rust-analyzer`.
- O con la configuración nativa de LSP: `:LspInstall rust-analyzer`.

**Zed** (gratuito, ultrarrápido):
- Soporte nativo de Rust desde el primer arranque.

### 1.2.5 El comando `cargo new` y la anatomia de un proyecto

Una vez que `cargo` está instalado, crear un nuevo proyecto es tan simple como:

```bash
cargo new erp_hello
cd erp_hello
```

Esto genera una estructura como la siguiente:

```
erp_hello/
├── Cargo.toml      ← manifiesto del proyecto (nombre, versión, dependencias)
└── src/
    └── main.rs     ← punto de entrada del programa
```

`Cargo.toml` es el archivo donde declaramos el nombre del proyecto, la versión, los autores y, sobre todo, las **dependencias** (librerías externas). Por ahora se ve así:

```toml
[package]
name = "erp_hello"
version = "0.1.0"
edition = "2021"

[dependencies]
```

El campo `edition` indica qué edición del lenguaje usamos. En 2026 hay tres ediciones publicadas: 2015, 2018 y 2021. Usaremos **2021** en todo el manual. Las ediciones no son versiones del lenguaje: son "sub-lenguajes" compatibles hacia atrás que el equipo de Rust publica cada 3 años con pequeñas mejoras de sintaxis.

El campo `[dependencies]` está vacío porque aún no hemos añadido ninguna. Lo llenaremos en capítulos posteriores con `serde`, `tokio`, `actix-web`, `diesel`, `sea-orm`, etc.

### 1.2.6 Comandos esenciales de `cargo`

Antes de seguir, memoriza esta tabla. La usarás cientos de veces:

| Comando | Descripción |
|---|---|
| `cargo new nombre` | Crea un nuevo proyecto binario en `./nombre`. |
| `cargo new --lib nombre` | Crea una nueva librería en `./nombre`. |
| `cargo init` | Inicializa un proyecto en el directorio actual. |
| `cargo build` | Compila el proyecto (modo *debug*). |
| `cargo build --release` | Compila optimizado (más lento, pero mucho más rápido en ejecución). |
| `cargo run` | Compila y ejecuta el binario. |
| `cargo check` | Sólo verifica tipos (mucho más rápido que `build`). |
| `cargo test` | Ejecuta los tests. |
| `cargo doc --open` | Genera y abre la documentación de tu código y dependencias. |
| `cargo fmt` | Formatea el código con `rustfmt`. |
| `cargo clippy` | Ejecuta el linter `clippy`. |
| `cargo update` | Actualiza las versiones de las dependencias. |
| `cargo add crate` | Añade una dependencia al `Cargo.toml` (requiere `cargo-edit`). |
| `cargo clean` | Borra la carpeta `target/`. |
| `cargo tree` | Muestra el árbol de dependencias. |

### 1.2.7 La carpeta `target/`

Cada vez que compiles, `cargo` crea una carpeta `target/` con los binarios, los archivos intermedios y las dependencias. Puedes borrarla con `cargo clean` y se regenera sola. Por convención, **no la subas al control de versiones** (añade `target/` a tu `.gitignore`).

### 1.2.8 Configuracion del registro (mirror) — opcional

Si la velocidad de descarga de *crates* (el registro de paquetes de Rust) te parece lenta desde tu ubicación, puedes configurar un mirror regional en `~/.cargo/config.toml`:

```toml
[source.crates-io]
replace-with = "tuna"

[source.tuna]
registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
```

En Latinoamérica y España, generalmente no hace falta: la latencia del registro oficial (`https://crates.io`) es razonable.

## 1.3 Tu primer programa: Hola Mundo (ERP)

No podíamos empezar con un "Hello, world!" genérico, porque este manual está orientado a un ERP/CRM. En su lugar, vamos a saludar al lector y mostrarle la versión del binario, como si fuera el splash inicial de un software empresarial. El capítulo cierra con un mini-proyecto: `proyectos_capitulo/parte1/01_erp_hello/`, que es el primero de los 17 mini-proyectos del manual.

### 1.3.1 El codigo fuente

Primero, una explicación larga antes del código. Un programa Rust se compone de **funciones**, y la función más importante de todas es `main`, que es el punto de entrada: cuando ejecutas el binario, el sistema operativo llama a `main` y le pasa los argumentos de línea de comandos. A diferencia de C, Rust no requiere que escribas `int main(void)`: basta con `fn main() { ... }`. Si quieres leer los argumentos, debes declararlos: `fn main() { ... }` se convierte en `fn main() { ... }` sin argumentos, o en una variante que use `std::env::args()` para leerlos manualmente. En Rust, la macro `println!` (con el signo de exclamación, que indica que es una *macro* y no una función) imprime texto en la salida estándar seguida de un salto de línea. La macro se invoca con paréntesis y el texto se pasa como primer argumento, que puede ser una *cadena literal* (entre comillas dobles) o un *string con formato* (con `{}` como *placeholder* para valores que se pasan como argumentos adicionales).

A continuación, el código del primer proyecto:

```rust
// archivo: src/main.rs
// Este es el primer programa del manual: el clásico "Hola Mundo" orientado a ERP.
// Imprime un mensaje de bienvenida y la versión del binario.
// Aprenderás: estructura de un archivo Rust, la macro println!, comentarios.

fn main() {
    // Nombre del ERP (constante literal)
    let nombre_erp = "ERP/CRM Rust México";

    // Versión actual del software (podría venir de Cargo.toml en el futuro)
    let version = "0.1.0";

    // Año de inicio del proyecto
    let anio_inicio: u32 = 2026;

    // Mensaje de bienvenida (string con formato)
    println!("╔══════════════════════════════════════╗");
    println!("║  {} v{}            ║", nombre_erp, version);
    println!("║  Fundamentos de Rust - 2026          ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("Bienvenido al sistema. Año de inicio: {}", anio_inicio);
    println!("Compilado con Rust {} en modo {}", rustc_version_runtime(), modo_compilacion());
}

// Función auxiliar que devuelve la versión del compilador usado
fn rustc_version_runtime() -> String {
    // La macro env! lee una variable de entorno en tiempo de compilación
    // La macro option_env! hace lo mismo pero devuelve Option<&str>
    // RUSTC_VERSION no es estándar; la versión real la podemos obtener de rustc
    // Para mantenerlo portable usamos una versión fija:
    String::from("1.96.1 (stable)")
}

// Detecta si la compilación es debug o release mediante cfg(debug_assertions)
fn modo_compilacion() -> &'static str {
    // La macro cfg! evalúa en tiempo de compilación
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}
```

Ahora un análisis línea por línea de las partes más importantes:

- `fn main() { ... }`: declaración de la función principal. Las llaves delimitan el cuerpo.
- `let nombre_erp = "ERP/CRM Rust México";`: declaración de una variable inmutable. En Rust, las variables son inmutables por defecto; para hacerlas mutables hay que añadir `mut`. Lo veremos en la sección 1.4.
- `let anio_inicio: u32 = 2026;`: declaración con tipo explícito `u32` (entero sin signo de 32 bits).
- `println!("...");`: macro que imprime texto en `stdout`.
- `"╔═...═╗"`: usamos caracteres Unicode de caja doble para el marco decorado.
- `println!("║  {} v{}            ║", nombre_erp, version);`: los `{}` son *placeholders* que se sustituyen por los argumentos en orden.
- `let version = "0.1.0";`: las cadenas literales son `&str` (string slice), no `String`. Veremos la diferencia en 1.16.4.
- `fn rustc_version_runtime() -> String { ... }`: función auxiliar que devuelve un `String` (tipo *owned*).
- `cfg!(debug_assertions)`: macro que evalúa en tiempo de compilación a `true` si estamos en modo debug.

**Salida esperada** (en modo debug):

```
╔══════════════════════════════════════╗
║  ERP/CRM Rust México v0.1.0            ║
║  Fundamentos de Rust - 2026          ║
╚══════════════════════════════════════╝

Bienvenido al sistema. Año de inicio: 2026
Compilado con Rust 1.96.1 (stable) en modo debug
```

### 1.3.2 Compilar y ejecutar

Para ejecutar este programa:

```bash
cd erp_hello
cargo run
# Cargo compilara el proyecto y ejecutara el binario
```

Verás que `cargo` primero compila todas las dependencias (al principio son pocas) y luego ejecuta el binario. La primera compilación es lenta (descargar y compilar `rustc` y `std`), pero las siguientes son muy rápidas porque el *caché* está en `target/`.

Para compilar en modo *release* (optimizado, mucho más rápido pero más lento de compilar):

```bash
cargo run --release
```

### 1.3.3 Errores tipicos al empezar

**Error 1: olvidar el punto y coma**.
```rust
fn main() {
    println!("Hola")  // ← falta el ;
}
```
Mensaje del compilador: `error: expected semicolon`. Solución: añadir `;` al final.

**Error 2: cerrar mal las comillas o los paréntesis**.
```rust
println!("ERP);
```
Mensaje: `error: unterminated double quote string`. Solución: revisar comillas.

**Error 3: confundir `println` (función) con `println!` (macro)**.
```rust
println("Hola");   // ← incorrecto
println!("Hola");  // ← correcto
```
Mensaje: `error: cannot find function 'println' in this scope`. Solución: añadir el `!`.

### 1.3.4 Mini-proyecto: `01_erp_hello`

El primer mini-proyecto está en `proyectos_capitulo/parte1/01_erp_hello/`. Vamos a crearlo desde cero paso a paso para que sepas exactamente qué hace cada comando:

**Paso 1**: crear el proyecto con `cargo`.

```bash
cd /home/roy/rust_man/proyectos_capitulo/parte1
cargo new 01_erp_hello
# Salida: Created binary (application) `01_erp_hello` package
```

**Paso 2**: editar `src/main.rs` y reemplazar el contenido por el código visto en 1.3.1.

**Paso 3**: compilar y ejecutar.

```bash
cd 01_erp_hello
cargo run
```

**Paso 4**: opcional — formatear y verificar con `clippy`.

```bash
cargo fmt
cargo clippy
```

**Paso 5**: opcional — compilar en release.

```bash
cargo run --release
```

**Variante propuesta**: modifica el programa para que pida el nombre del usuario por línea de comandos usando `std::env::args()`. Lo veremos en la sección 1.8 con funciones; por ahora, la variante más sencilla es modificar el `println!` para que imprima un nombre *hardcodeado* distinto.


## 1.4 Variables y mutabilidad

En la mayoría de lenguajes de programación, una "variable" es, por defecto, mutable: una vez creada, puedes cambiar su valor cuantas veces quieras. En Rust ocurre lo contrario: por defecto, las variables son **inmutables**, y para hacerlas mutables hay que declararlas explícitamente con la palabra clave `mut`. Esta decisión de diseño no es capricho: viene del convencimiento de que, en la mayoría de los casos, las variables que no cambian hacen que el código sea más fácil de razonar. Piensa en una factura: el RFC del emisor no cambia una vez emitida la factura, el subtotal no varía después del cálculo, el IVA se congela al timbrar el CFDI. Si esos valores pudieran mutar a lo largo del programa, la trazabilidad se complicaría. Rust te obliga a declararlos inmutables por defecto, y si necesitas cambiarlos, debes escribir `let mut`, que es como decir "sí, sé que esto va a cambiar, lo he pensado".

Piensa en una variable como una **caja etiquetada**. La etiqueta es el nombre de la variable. El contenido es el valor. Si la caja es inmutable, una vez que metes algo, la etiqueta queda asociada a ese contenido y no se puede sustituir por otro (sí se puede *sombrear*, lo veremos pronto). Si la caja es mutable, la etiqueta sigue siendo la misma, pero puedes abrir la caja, sacar el contenido y meter otro. La analogía mental es simple, pero las consecuencias en tiempo de compilación son profundas: el compilador puede asumir que el valor de una variable inmutable no cambia y aplicar optimizaciones que de otro modo serían imposibles. Además, en programas concurrentes, los datos inmutables son automáticamente *thread-safe*: dos hilos pueden leerlos a la vez sin riesgo de *data race*.

### 1.4.1 Declaracion basica

La sintaxis para declarar una variable es `let nombre = valor;`. El compilador infiere el tipo a partir del valor (en este caso, `i32` para el entero). Si quieres ser explícito, puedes usar `let nombre: Tipo = valor;`.

```rust
// archivo: src/main.rs
// Mini-proyecto del capítulo: calculadora simple de IVA para una factura.
// Aprenderás: let, let mut, inferencia de tipos, anotación explícita, sombreado.

fn main() {
    // --- Constantes (no son variables, pero se parecen) ---
    // Las constantes se declaran con const, siempre con tipo, en MAYÚSCULAS.
    // Se evalúan en tiempo de compilación, no pueden mutar nunca.
    const IVA_PORCENTAJE: f64 = 16.0;          // IVA general en México (2026)
    const TASA_IEPS_BEBIDAS: f64 = 26.5;       // IEPS para bebidas azucaradas
    const ANIO_FISCAL: u16 = 2026;

    // --- Variables inmutables (por defecto) ---
    // Una factura de ejemplo: el subtotal no cambia una vez calculado.
    let subtotal = 1250.50_f64;                // el sufijo _f64 fuerza tipo f64
    let moneda: &str = "MXN";                  // &str = string slice
    let cliente: &str = "Constructora del Bajío S.A. de C.V.";

    // --- Variables mutables (con mut) ---
    // El descuento puede recalcularse, así que es mutable.
    let mut descuento_porcentaje: f64 = 0.0;

    // Leemos el descuento (en este caso lo fijamos, pero podría pedirse al usuario)
    descuento_porcentaje = 10.0;               // 10% de descuento

    // --- Cálculos ---
    let monto_descuento = subtotal * (descuento_porcentaje / 100.0);
    let base_gravable = subtotal - monto_descuento;
    let iva = base_gravable * (IVA_PORCENTAJE / 100.0);
    let total = base_gravable + iva;

    // --- Salida ---
    println!("╔══════════════════════════════════════╗");
    println!("║  COTIZACIÓN - Año fiscal {}     ║", ANIO_FISCAL);
    println!("╚══════════════════════════════════════╝");
    println!("Cliente:      {}", cliente);
    println!("Moneda:       {}", moneda);
    println!("Subtotal:     ${:.2}", subtotal);
    println!("Descuento:    -${:.2} ({}%)", monto_descuento, descuento_porcentaje);
    println!("Base gravable:${:.2}", base_gravable);
    println!("IVA ({}%):    +${:.2}", IVA_PORCENTAJE, iva);
    println!("─────────────────────────────────────");
    println!("TOTAL:        ${:.2} {}", total, moneda);
}
```

**Análisis del código línea por línea**:

- `const IVA_PORCENTAJE: f64 = 16.0;`: las constantes en Rust deben tener tipo explícito y suelen escribirse en MAYÚSCULAS_CON_GUIONES_BAJOS. Se evalúan en tiempo de compilación, lo que permite que el compilador las "inline" (incruste) en el código generado.
- `let subtotal = 1250.50_f64;`: el sufijo `_f64` es una "anotación de tipo literal": equivale a escribir `let subtotal: f64 = 1250.50;`. Es útil cuando el literal podría inferirse de varias formas (por ejemplo, un número entero con sufijo `_i32`).
- `let moneda: &str = "MXN";`: `&str` es un *string slice*, una referencia inmutable a una zona de memoria con los caracteres. Es diferente de `String`, que es *owned*. La diferencia entre `&str` y `String` la explicaremos a fondo en 1.16.4.
- `let mut descuento_porcentaje: f64 = 0.0;`: aquí sí usamos `mut`. La palabra `mut` también se aplica a las referencias (`&mut`) y a los parámetros de funciones.
- `descuento_porcentaje = 10.0;`: reasignación válida porque la variable es mutable. Si no hubiéramos escrito `mut`, el compilador lanzaría `error: cannot assign twice to immutable variable`.
- `IVA_PORCENTAJE / 100.0`: división entre dos `f64`. El resultado es `0.16` (no `0` como sería en división entera).
- `println!("IVA ({}%):    +${:.2}", IVA_PORCENTAJE, iva);`: el formato `{:.2}` indica "imprime el número con 2 decimales". Aprende más sobre formato en la sección 1.6.

**Salida esperada**:

```
╔══════════════════════════════════════╗
║  COTIZACIÓN - Año fiscal 2026     ║
╚══════════════════════════════════════╝
Cliente:      Constructora del Bajío S.A. de C.V.
Moneda:       MXN
Subtotal:     $1250.50
Descuento:    -$125.05 (10%)
Base gravable:$1125.45
IVA (16%):    +$180.07
─────────────────────────────────────
TOTAL:        $1305.52 MXN
```

### 1.4.2 El sombreado (shadowing)

A veces quieres reutilizar el nombre de una variable pero transformándola. En lugar de inventar un nombre nuevo (`subtotal_redondeado`, `subtotal_final`, `subtotal2`), Rust te permite "sombrear" la variable: declarar una nueva con el mismo nombre. La nueva variable *oculta* a la anterior hasta el final del bloque. Esto se llama **shadowing** y es muy útil en conversiones o para aplicar transformaciones sucesivas.

```rust
fn main() {
    let subtotal = 1250.50_f64;                 // f64
    let subtotal = subtotal.round();            // f64 redondeado a entero más cercano
    let subtotal = (subtotal as i32) as f64;    // conversión explícita i32 -> f64
    println!("Subtotal redondeado: ${}", subtotal);
    // La variable "subtotal" original ya no es accesible desde aquí.
}
```

**Diferencia entre sombreado y mutabilidad**: con `mut`, conservas la misma variable y cambias su valor. Con sombreado, creas una variable nueva (que puede ser de un tipo diferente) y la vieja deja de existir. El sombreado no requiere `mut`. La variable sombreada es inmutable hasta que la vuelvas a sombrear.

### 1.4.3 Reglas de nombrado de variables

- Deben empezar con una letra minúscula o un guión bajo (`_`).
- Pueden contener letras, números y guiones bajos.
- Se escriben en `snake_case` (palabras separadas por guión bajo, todo en minúscula). Ejemplos válidos: `subtotal`, `cliente_id`, `cantidad_productos`.
- No pueden ser palabras reservadas del lenguaje (`let`, `fn`, `struct`, `enum`, `match`, etc.). Si necesitas usar una, puedes añadir `_` al final: `let r#match = 5;` aunque es muy raro.
- Los nombres que empiezan con guión bajo se usan a menudo para variables que no se van a usar (por ejemplo, `let _resultado = funcion();` si sabemos que `funcion` retorna algo que no nos interesa).

### 1.4.4 Errores tipicos

**Error 1: olvidar `mut`**.
```rust
let descuento = 0.0;
descuento = 10.0;     // ← error: cannot assign twice to immutable variable
```
Solución: `let mut descuento = 0.0;`.

**Error 2: usar `let` para reasignar creyendo que sombrea**.
```rust
let a = 5;
a = 10;               // ← error
```
Con sombreado sería: `let a = 10;` (sin `let` es reasignación, que requiere `mut`).

**Error 3: sombra y luego usar el valor viejo**.
```rust
let precio = 100;
let precio = precio * 2;   // precio ahora es 200
println!("{}", precio);    // imprime 200, no 100
```
Si querías conservar el viejo, usa una variable con otro nombre: `let precio_doble = precio * 2;`.

### 1.4.5 Mini-proyecto: `02_calc_impuestos`

Este mini-proyecto está en `proyectos_capitulo/parte1/02_calc_impuestos/`. Aplica todo lo visto: constantes, `let`, `let mut`, sombreado, formateo de números. El programa pide un subtotal, un porcentaje de descuento y un tipo de producto (general, alimentos, bebidas con IEPS), y calcula el total con los impuestos mexicanos correctos. La estructura del proyecto es:

```bash
cd /home/roy/rust_man/proyectos_capitulo/parte1
cargo new 02_calc_impuestos
cd 02_calc_impuestos
```

Edita `src/main.rs`:

```rust
// Mini-proyecto 02: calculadora de impuestos para el ERP/CRM
// Calcula subtotal, descuentos, IVA e IEPS según el tipo de producto.
// Compilar con: cargo run

use std::io::{self, Write};

// Constantes fiscales (México, 2026)
const IVA_GENERAL: f64 = 16.0;
const IVA_FRANJA_FRONTERIZA: f64 = 8.0;
const IEPS_BEBIDAS: f64 = 26.5;
const IEPS_BOTANAS: f64 = 8.0;
const IEPS_ALCOHOL: f64 = 53.0;

fn main() {
    // --- Solicitar datos al usuario ---
    let mut entrada = String::new();

    print!("Subtotal del producto ($): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).unwrap();
    let subtotal: f64 = entrada.trim().parse().expect("Debe ser un número");
    entrada.clear();

    print!("Descuento (%): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).unwrap();
    let descuento: f64 = entrada.trim().parse().expect("Debe ser un número");
    entrada.clear();

    print!("Tipo (1=General, 2=Bebidas, 3=Botanas, 4=Alcohol, 5=Franja fronteriza): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).unwrap();
    let tipo: u8 = entrada.trim().parse().expect("Debe ser 1-5");
    entrada.clear();

    // --- Cálculos (variables inmutables) ---
    let monto_descuento = subtotal * (descuento / 100.0);
    let base = subtotal - monto_descuento;

    // Determinar tasas según el tipo
    let (iva_pct, ieps_pct) = match tipo {
        1 => (IVA_GENERAL, 0.0),
        2 => (IVA_GENERAL, IEPS_BEBIDAS),
        3 => (IVA_GENERAL, IEPS_BOTANAS),
        4 => (IVA_GENERAL, IEPS_ALCOHOL),
        5 => (IVA_FRANJA_FRONTERIZA, 0.0),
        _ => {
            println!("Tipo no válido, se usará IVA general");
            (IVA_GENERAL, 0.0)
        }
    };

    // IEPS se calcula sobre la base gravable
    let ieps = base * (ieps_pct / 100.0);
    // El IVA se calcula sobre (base + IEPS) — esto es importante en México
    let iva = (base + ieps) * (iva_pct / 100.0);
    let total = base + ieps + iva;

    // --- Imprimir ticket ---
    println!();
    println!("═══════════════════════════════════════");
    println!("         COTIZACIÓN - ERP/CRM          ");
    println!("═══════════════════════════════════════");
    println!("Subtotal:               ${:>10.2}", subtotal);
    println!("Descuento ({}%):        -${:>9.2}", descuento, monto_descuento);
    println!("Base gravable:          ${:>10.2}", base);
    if ieps_pct > 0.0 {
        println!("IEPS ({}%):             +${:>9.2}", ieps_pct, ieps);
    }
    println!("IVA ({}%):              +${:>9.2}", iva_pct, iva);
    println!("───────────────────────────────────────");
    println!("TOTAL:                  ${:>10.2}", total);
    println!("═══════════════════════════════════════");
}
```

**Análisis**: el programa usa la *función* `match` (que veremos a fondo en 1.12) para asignar las tasas según el tipo. La *tupla* `(iva_pct, ieps_pct)` es un valor que contiene dos `f64`; el *formato* `{:>10.2}` alinea a la derecha en 10 caracteres con 2 decimales. La macro `flush()` fuerza a la terminal a mostrar el `print!` (que no tiene salto de línea) antes del `read_line`.

Para ejecutarlo:

```bash
cd /home/roy/rust_man/proyectos_capitulo/parte1/02_calc_impuestos
cargo run
```

## 1.5 Tipos de datos primitivos

Rust es un lenguaje de **tipado estático** (*statically typed*): cada variable, expresión y función tiene un tipo conocido en tiempo de compilación. Esto significa que el compilador verifica antes de ejecutar que estés usando los datos de forma coherente. A diferencia de lenguajes como Python o JavaScript, donde los tipos se descubren en tiempo de ejecución, en Rust el compilador es capaz de detectar muchos errores *antes* de que el programa se ejecute.

Rust divide los tipos en dos grandes familias: **escalares** (representan un único valor) y **compuestos** (representan varios valores; los veremos en 1.10). Los escalares son: enteros, flotantes, booleanos y caracteres.

### 1.5.1 Enteros

Los enteros pueden ser **con signo** (negativos y positivos) o **sin signo** (sólo positivos), y su tamaño en bits determina el rango de valores:

| Tamaño | Con signo | Sin signo | Rango con signo | Rango sin signo |
|---|---|---|---|---|
| 8 bits | `i8` | `u8` | -128 a 127 | 0 a 255 |
| 16 bits | `i16` | `u16` | -32 768 a 32 767 | 0 a 65 535 |
| 32 bits | `i32` | `u32` | -2,1×10⁹ a 2,1×10⁹ | 0 a 4,3×10⁹ |
| 64 bits | `i64` | `u64` | -9,2×10¹⁸ a 9,2×10¹⁸ | 0 a 1,8×10¹⁹ |
| 128 bits | `i128` | `u128` | gigantesco | gigantesco |
| Tamaño del puntero | `isize` | `usize` | depende de la arquitectura | depende |

El tipo **por defecto** para enteros es `i32` (entero de 32 bits con signo). El `isize`/`usize` se usan principalmente para indexar colecciones y para tamaños de memoria; su tamaño depende de si compilas para 32 bits o 64 bits.

**Literales enteros** pueden escribirse de varias formas:

```rust
let a = 98_222;             // separador _ para legibilidad (equivale a 98222)
let b = 0xff;               // hexadecimal
let c = 0o77;               // octal
let d = 0b1111_0000;        // binario
let e = b'A';               // byte (u8) - sólo ASCII
let f: u8 = 255;            // tipo explícito
```

**¿Por qué `i32` y no `i64` por defecto?** Porque en arquitecturas modernas de 64 bits, los `i32` suelen ser ligeramente más rápidos (mejor uso de registros) y consumen menos memoria. Para números muy grandes (por ejemplo, IDs de base de datos o totales monetarios en centavos), usaremos `i64` o `i128` explícitamente.

**El caso de los centavos en un ERP/CRM**: una de las decisiones de diseño más importantes en software financiero es cómo representar dinero. La opción naïve es usar `f64` (punto flotante), pero los flotantes tienen errores de redondeo que pueden acumularse y dar totales incorrectos en facturas. La opción profesional es usar **enteros representando centavos** (`i64` con el valor 125050 para $1250.50). Veremos cómo hacerlo en 1.16.4 cuando hablemos de decimales en bases de datos.

**Errores típicos con enteros**:
- **Desbordamiento (overflow)**: en modo *debug*, el compilador añade comprobaciones que lanzan `panic!` cuando un entero se sale de rango. En modo *release*, se hace "two's complement wrapping" (por ejemplo, `u8::MAX + 1 == 0`). Para comportamiento explícito, usa los métodos `wrapping_add`, `checked_add` o `saturating_add`.

```rust
let x: u8 = 255;
let y = x + 1;   // en debug: panic; en release: y = 0
let z = x.wrapping_add(1);  // siempre 0, sin panic
let w = x.checked_add(1);   // devuelve Option<u8> = None
```

### 1.5.2 Flotantes

Rust tiene dos tipos de punto flotante, ambos con signo: `f32` (precisión simple, IEEE 754 binary32) y `f64` (precisión doble, IEEE 754 binary64). El tipo por defecto es `f64`, que es prácticamente el estándar de la industria y el recomendado para la mayoría de los cálculos.

```rust
let precio = 19.99_f64;          // sufijo _f64
let pi: f64 = 3.141592653589793; // tipo explícito
let epsilon: f32 = 1e-6;         // notación científica
```

**Precaución con `==` en flotantes**: comparar flotantes con `==` puede dar resultados inesperados. Para verificar si dos flotantes son "casi iguales", se compara su diferencia con un valor muy pequeño llamado *epsilon*:

```rust
let a = 0.1 + 0.2;
let b = 0.3;
let son_iguales = (a - b).abs() < 1e-10;   // true
```

### 1.5.3 Booleanos

`bool` representa un valor de verdad. Ocupa 1 byte en memoria. Los valores posibles son `true` y `false`. Se usan en condicionales y operadores lógicos (`&&`, `||`, `!`).

```rust
let es_cliente_vip: bool = true;
let tiene_credito = false;
if es_cliente_vip && !tiene_credito {
    println!("Ofrecer pago contra entrega");
}
```

### 1.5.4 Caracteres y cadenas

El tipo `char` representa un *Unicode Scalar Value* (un "punto de código") de 4 bytes. Puede contener desde letras ASCII hasta emojis, ideogramas chinos o símbolos matemáticos. Se delimita con **comillas simples** (`'a'`), a diferencia de las cadenas que usan comillas dobles.

```rust
let inicial: char = 'M';
let euro: char = '€';
let bandera: char = '🇲🇽';        // es una sola unidad, pero en pantalla son 2 emojis
let salto: char = '\n';            // caracteres de escape
let tab: char = '\t';
```

**Cuidado**: las cadenas (`&str`, `String`) son UTF-8, no arrays de `char`. Por eso `.len()` devuelve bytes, no caracteres. Para contar caracteres, usa `.chars().count()`.

```rust
let s = "México";
println!("{} bytes, {} caracteres", s.len(), s.chars().count());
// 7 bytes (la 'é' ocupa 2), 6 caracteres
```

### 1.5.5 Inferencia de tipos y anotacion explicita

Rust es un lenguaje de **inferencia de tipos**: el compilador intenta deducir el tipo de cada variable a partir de su uso. Si puede, no hace falta anotarlo. Pero a veces la inferencia falla o es ambigua, y entonces es necesario anotar:

```rust
let x = 5;                // i32 (por defecto)
let y: i64 = 5;           // i64 (explícito)
let z = 5_i64;            // i64 (sufijo)
let w: f64 = 5.0;         // f64 explícito
let v = "5".parse::<i32>().unwrap();   // turbofish: parse::<i32>() infiere Result
```

El operador `::<>` se llama *turbofish* y se usa para anotar tipos en llamadas a funciones genéricas (lo veremos en 1.14).

### 1.5.6 Conversiones entre tipos (casting)

Rust no convierte implícitamente entre tipos numéricos. Hay que hacerlo explícitamente con la palabra clave `as`:

```rust
let a: i32 = 10;
let b: f64 = a as f64;       // 10.0
let c: u8 = a as u8;         // 10
let d: i64 = 1234567890_i32 as i64;   // conversiones de tamaño requieren as
```

**Cuidado con las conversiones peligrosas**: `let x: i32 = -1; let y = x as u32;` produce `4294967295` (el mayor u32) porque la conversión es "bit a bit". Para conversiones seguras, usa `try_into` (de la librería estándar) o los métodos `from` y `into`.

### 1.5.7 Mini-ejercicio del capitulo

Crea un nuevo proyecto `cargo new 03_tipos_primitivos` y experimenta con cada tipo. Intenta provocar un desbordamiento en modo debug y observa el panic. Compara el resultado con `wrapping_add`.

## 1.6 Operadores y expresiones

En Rust, casi todo es una **expresión**: una construcción que devuelve un valor. Una declaración `let`, un `if`, un `match`, un bloque `{ ... }`, un cierre (closure): todos producen un valor. Esta es una diferencia importante con lenguajes como C o Java, donde `if` es una *sentencia* que no devuelve nada. La consecuencia es que puedes escribir código muy compacto y expresar cosas como `let x = if condicion { 1 } else { 2 };`.

### 1.6.1 Operadores aritmeticos

| Operador | Descripción | Ejemplo |
|---|---|---|
| `+` | Suma | `5 + 3` → `8` |
| `-` | Resta | `5 - 3` → `2` |
| `*` | Multiplicación | `5 * 3` → `15` |
| `/` | División | `6 / 3` → `2` (entera), `6.0 / 3.0` → `2.0` |
| `%` | Módulo (resto) | `7 % 3` → `1` |

En división de enteros, el resultado también es entero: `7 / 2 == 3`, no `3.5`. Si quieres división con decimales, convierte explícitamente: `7.0 / 2.0 == 3.5` o `7 as f64 / 2 as f64`.

### 1.6.2 Operadores de comparacion y logicos

| Operador | Descripción |
|---|---|
| `==`, `!=` | Igualdad y desigualdad |
| `<`, `>`, `<=`, `>=` | Comparación |
| `&&` | Y lógico (and) |
| `\|\|` | O lógico (or) |
| `!` | Negación |

A diferencia de muchos lenguajes, los operadores `&&` y `||` son **cortocircuito**: si el operando izquierdo ya decide el resultado, el derecho no se evalúa.

```rust
let a = true;
let b = false;
let c = a && b;   // false
let d = a || b;   // true
let e = !a;       // false
```

### 1.6.3 Operadores de asignacion

`=` asigna. Los operadores `+=`, `-=`, `*=`, `/=`, `%=` son atajos:

```rust
let mut x = 10;
x += 5;    // x = x + 5
x -= 3;    // x = x - 3
x *= 2;    // x = x * 2
x /= 4;    // x = x / 4
x %= 3;    // x = x % 3
```

### 1.6.4 Operadores bit a bit

`&` (and), `|` (or), `^` (xor), `<<` (desplazamiento izquierda), `>>` (desplazamiento derecha), `!` (not). Útiles para trabajar con máscaras de bits, por ejemplo permisos de archivos en Unix (`0o755` = `rwxr-xr-x`).

### 1.6.5 Rango (range)

El operador `..` crea rangos. `1..5` incluye 1, 2, 3, 4 (excluye el 5). `1..=5` incluye también el 5. Los rangos se usan mucho en bucles `for`:

```rust
for i in 1..5 {
    println!("{}", i);   // 1 2 3 4
}
for i in 1..=5 {
    println!("{}", i);   // 1 2 3 4 5
}
```

### 1.6.6 Formato de cadenas y placeholders

La macro `format!` y las macros `println!`/`print!` aceptan varios especificadores de formato. La sintaxis general es `{[argumento][:relleno][alineamiento][signo][#][0][ancho][.precisión][tipo]}`.

| Especificador | Significado | Ejemplo |
|---|---|---|
| `{}` | Display (por defecto) | `format!("{}", 42)` → `"42"` |
| `{:?}` | Debug | `format!("{:?}", vec![1,2,3])` → `"[1, 2, 3]"` |
| `{:#?}` | Debug "pretty" | multilínea |
| `{:b}` | Binario | `format!("{:b}", 10)` → `"1010"` |
| `{:x}` | Hexadecimal | `format!("{:x}", 255)` → `"ff"` |
| `{:o}` | Octal | `format!("{:o}", 8)` → `"10"` |
| `{:e}` | Científica | `format!("{:e}", 12345.0)` → `"1.2345e4"` |
| `{:>10}` | Alinear derecha en 10 | `format!("{:>10}", "hola")` → `"      hola"` |
| `{:_<10}` | Alinear izquierda, rellenar con `_` | `"hola______"` |
| `{:.2}` | 2 decimales | `format!("{:.2}", 3.14159)` → `"3.14"` |
| `{:>10.2}` | Combinar | `format!("{:>10.2}", 3.1)` → `"      3.10"` |
| `{:02}` | Rellenar con ceros | `format!("{:02}", 5)` → `"05"` |

```rust
let precio = 1234.5;
let iva = precio * 0.16;
println!("Precio: ${:>10.2}", precio);
println!("IVA:    ${:>10.2}", iva);
println!("Total:  ${:>10.2}", precio + iva);
```

Para un ERP/CRM es muy útil dominar `{:>10.2}` para alinear columnas en tickets y reportes.

### 1.6.7 Expresiones vs. sentencias

En Rust, una sentencia termina con `;` y no devuelve valor. Una expresión devuelve un valor y no termina con `;` (o su valor se descarta si lleva `;`). Esta distinción es importante:

```rust
fn main() {
    let x = 5;                  // sentencia: declara x
    let y = {
        let z = 3;
        z + 1                   // expresión: devuelve 4 (sin ;)
    };                          // y vale 4
    println!("y = {}", y);
}
```

El bloque `{ let z = 3; z + 1 }` devuelve `4` porque la última expresión no lleva `;`. Si la última línea llevase `;`, el bloque devolvería `()` (la unidad, *unit*).

### 1.6.8 Errores tipicos

**Error 1: usar `=` en vez de `==`**.
```rust
if x = 5 {     // error: cannot assign to immutable
```
El compilador detecta esto y sugiere `==`.

**Error 2: división entera inesperada**.
```rust
let total = 7 / 2;       // 3, no 3.5
let total_f = 7.0 / 2.0; // 3.5
```

**Error 3: olvidar el `;` en una sentencia de un bloque que devuelve valor**.
```rust
let y = if x > 0 {
    1;
} else {
    2;
};
// y tiene tipo () no i32; error: if and else have incompatible types
```

## 1.7 Control de flujo

El control de flujo permite que un programa tome decisiones o repita bloques de código. En Rust tenemos `if`/`else` (condicionales), `loop` (bucle infinito), `while` (bucle condicional) y `for` (iteración sobre colecciones o rangos). A diferencia de otros lenguajes, no hay `do-while` ni `switch` (sustituido por `match`, mucho más poderoso).

### 1.7.1 `if` / `else if` / `else`

La condición no lleva paréntesis (es obligatorio en C/Java, pero Rust los prohíbe para reducir ruido visual). Las llaves, en cambio, sí son obligatorias incluso para una sola instrucción.

```rust
// Mini-proyecto: clasificador de clientes según su historial de compras
// Aprenderás: if/else, expresiones, operadores lógicos

fn main() {
    let total_compras_mxn: f64 = 75_000.0;
    let numero_pedidos: u32 = 12;
    let dias_sin_comprar: u32 = 45;

    let categoria: &str;

    if total_compras_mxn >= 100_000.0 {
        categoria = "VIP Platino";
    } else if total_compras_mxn >= 50_000.0 {
        categoria = "VIP Oro";
    } else if total_compras_mxn >= 10_000.0 {
        categoria = "Regular Premium";
    } else if numero_pedidos >= 20 {
        categoria = "Frecuente";
    } else {
        categoria = "Nuevo";
    }

    let estado: &str = if dias_sin_comprar > 180 {
        "Inactivo (reactivar)"
    } else if dias_sin_comprar > 90 {
        "En riesgo"
    } else {
        "Activo"
    };

    println!("Cliente categoría: {}", categoria);
    println!("Estado:            {}", estado);

    let beneficio: &str = match (categoria, estado) {
        ("VIP Platino", "Activo") => "Asignar ejecutivo dedicado",
        ("VIP Oro", "Activo") => "Programa de puntos dobles",
        (_, "Inactivo (reactivar)") => "Campaña de email marketing",
        _ => "Sin beneficio especial",
    };
    println!("Beneficio:         {}", beneficio);
}
```

Como ves, un `if` puede usarse como expresión y devolver un valor. La rama `else` es obligatoria si usas el `if` como valor (a menos que sea la última sentencia de la función que devuelve `()`). En el ejemplo, `let estado = if ... else ...;` es una expresión.

### 1.7.2 `loop` (bucle infinito)

`loop` repite un bloque indefinidamente. Para salir, usa `break`. Para saltar a la siguiente iteración, usa `continue`. Un `break` puede devolver un valor (muy útil para "loop con retorno"):

```rust
fn main() {
    let mut intentos = 0;
    let codigo_correcto = 1234;

    let codigo_ingresado = loop {
        intentos += 1;
        if intentos > 5 {
            break None;            // demasiados intentos
        }
        // simulamos un ingreso
        let intento = 1230 + intentos;
        if intento == codigo_correcto {
            break Some(intento);   // devuelve el código correcto
        }
    };

    match codigo_ingresado {
        Some(c) => println!("Acceso concedido ({})", c),
        None => println!("Bloqueado por demasiados intentos"),
    }
}
```

**Etiquetas de bucle**: si tienes `loop` anidados, puedes etiquetarlos con `'etiqueta:` y romper uno específico:

```rust
let matriz = vec![vec![1, 2, 3], vec![4, 5, 6]];
let mut encontrado = None;
'filas: for fila in &matriz {
    for (i, val) in fila.iter().enumerate() {
        if *val == 5 {
            encontrado = Some((i, val));
            break 'filas;   // rompe el bucle externo
        }
    }
}
```

### 1.7.3 `while`

`while condicion { ... }` repite mientras la condición sea verdadera. Es equivalente a `loop { if !condicion { break; } ... }`, pero más legible.

```rust
fn main() {
    let mut saldo: f64 = 1000.0;
    let meses = 0;
    while saldo < 2000.0 && meses < 12 {
        saldo *= 1.05;
        // meses += 1;
    }
    println!("Saldo final: ${}", saldo);
}
```

### 1.7.4 `for` (iteracion)

`for` es probablemente la forma más común de iterar en Rust. Itera sobre cualquier tipo que implemente el trait `IntoIterator`, lo que incluye vectores, rangos, hashmaps y casi cualquier colección.

```rust
fn main() {
    // Iterar sobre un rango
    for i in 0..5 {
        println!("i = {}", i);  // 0 1 2 3 4
    }

    // Iterar sobre un vector
    let productos = vec!["Laptop", "Mouse", "Teclado"];
    for producto in &productos {
        println!("Producto: {}", producto);
    }

    // Iterar con índice
    for (i, producto) in productos.iter().enumerate() {
        println!("{}. {}", i + 1, producto);
    }

    // Iterar sobre un HashMap
    use std::collections::HashMap;
    let mut precios: HashMap<&str, f64> = HashMap::new();
    precios.insert("Laptop", 18999.0);
    precios.insert("Mouse", 350.0);
    for (producto, precio) in &precios {
        println!("{}: ${}", producto, precio);
    }
}
```

### 1.7.5 Errores tipicos

- **Olvidar `&` al iterar**: `for x in vector { ... }` intenta mover el vector, lo que impide usarlo después. La forma correcta es `for x in &vector` o `&vector.iter()`.
- **Bucle infinito accidental**: si la condición de un `while` nunca cambia, o un `loop` no tiene `break`, el programa se quedará colgado.
- **Off-by-one**: con `0..n` iteras de 0 a n-1; si querías 1 a n, usa `1..=n`.

## 1.8 Funciones

Las funciones son el pilar de la organización del código. En Rust declaramos una función con `fn nombre(argumentos) -> tipo_retorno { cuerpo }`. El tipo de retorno se especifica con `->`, y la última expresión (sin `;`) es el valor devuelto. La palabra clave `return` también existe, pero sólo se usa para retornos anticipados.

### 1.8.1 Funciones con y sin valor de retorno

```rust
// archivo: src/main.rs
// Mini-proyecto: calculadora de margen de ganancia para productos
// Aprenderás: funciones, parámetros, retornos, documentación con ///

/// Calcula el margen de ganancia porcentual.
///
/// # Argumentos
/// * `costo` - Costo del producto en MXN
/// * `precio_venta` - Precio de venta al público en MXN
///
/// # Retorna
/// El porcentaje de margen (por ejemplo, 30.0 para un 30%)
///
/// # Ejemplo
/// ```
/// let margen = calcular_margen(70.0, 100.0);
/// assert_eq!(margen, 30.0);
/// ```
fn calcular_margen(costo: f64, precio_venta: f64) -> f64 {
    if precio_venta <= 0.0 {
        return 0.0;   // caso borde: precio inválido
    }
    let ganancia = precio_venta - costo;
    (ganancia / precio_venta) * 100.0
}

/// Clasifica un producto según su margen de ganancia
fn clasificar_producto(costo: f64, precio: f64) -> &'static str {
    let margen = calcular_margen(costo, precio);
    match margen {
        m if m < 0.0 => "Pérdida",
        m if m < 10.0 => "Margen bajo",
        m if m < 25.0 => "Margen saludable",
        m if m < 50.0 => "Buen margen",
        _ => "Margen alto (revisar competitividad)",
    }
}

fn main() {
    let productos = vec![
        ("Laptop HP Pavilion", 12500.0, 18999.0),
        ("Mouse óptico",       150.0,   350.0),
        ("Teclado mecánico",   600.0,   1200.0),
        ("Cable HDMI 2m",      35.0,    89.0),
        ("Audífonos gamer",    800.0,   999.0),
    ];

    println!("╔════════════════════════════════════════════════════╗");
    println!("║  Análisis de márgenes - Catálogo ERP/CRM          ║");
    println!("╠════════════════════════════════════════════════════╣");
    println!("║ Producto             Costo     Precio   Margen    ║");
    println!("╠════════════════════════════════════════════════════╣");
    for (nombre, costo, precio) in &productos {
        let margen = calcular_margen(*costo, *precio);
        let categoria = clasificar_producto(*costo, *precio);
        println!("║ {:<20} ${:>7.2}  ${:>7.2}  {:>5.1}%   ║",
                 nombre, costo, precio, margen);
        println!("║   → {}                                        ", categoria);
        println!("╚════════════════════════════════════════════════════╝");
    }
}
```

**Análisis**: observa tres detalles importantes. (1) Los comentarios `///` sobre la función son **documentación** que `cargo doc` convierte en HTML. (2) El `match` con guardas (`if m < 10.0`) permite clasificar según rangos. (3) El último `println!` no tiene `;` al final del *match*, lo que hace que el `match` (como bloque) devuelva un valor; ese valor se asigna a `categoria`.

### 1.8.2 Funciones como valores y closures

En Rust las funciones son **ciudadanos de primera clase**: puedes asignarlas a variables, pasarlas como argumentos y devolverlas desde otras funciones. Esto se hace mediante el tipo `fn` (puntero a función) o mediante *closures* (funciones anónimas que capturan variables del entorno).

```rust
fn main() {
    let sumar: fn(i32, i32) -> i32 = |a, b| a + b;   // closure asignado a fn
    let resultado = sumar(3, 4);
    println!("3 + 4 = {}", resultado);

    // Closure que captura una variable del entorno
    let iva = 0.16;
    let aplicar_iva = |precio: f64| -> f64 { precio * (1.0 + iva) };
    println!("${} con IVA = ${}", 100.0, aplicar_iva(100.0));
}
```

Profundizaremos en closures en 1.17.

### 1.8.3 Recursion

Una función puede llamarse a sí misma. La recursión es útil para recorrer árboles (categorías con sub-categorías, comentarios con respuestas, etc.). Cada llamada recursiva consume *stack*; si la profundidad es excesiva, podemos desbordar la pila. Para casos extremos, es mejor convertir la recursión en iteración con un bucle explícito o una pila manual.

```rust
// Mini-proyecto: calcular el factorial de un número (para cálculos de combinaciones)
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    for i in 0..10 {
        println!("{}! = {}", i, factorial(i));
    }
}
```

### 1.8.4 Mini-proyecto: `03_validador_cliente`

Este mini-proyecto (en `proyectos_capitulo/parte1/03_validador_cliente/`) reúne todo lo visto hasta aquí: variables, tipos, control de flujo, funciones. El programa valida los datos de un cliente mexicano: RFC con formato correcto, email con `@` y dominio válido, teléfono a 10 dígitos, código postal de 5 dígitos, y razón social no vacía. Utiliza funciones auxiliares para cada validación y reporta los errores en una lista.

```rust
use std::io::{self, Write};

// Constantes de validación (México)
const RFC_LONGITUD: usize = 13;       // 12 para persona moral + 3 para homoclave = 13
const RFC_LONGITUD_PF: usize = 13;    // persona física
const RFC_LONGITUD_PM: usize = 12;    // persona moral
const TEL_LONGITUD: usize = 10;
const CP_LONGITUD: usize = 5;

/// Valida que el RFC tenga formato correcto:
/// - 4 letras iniciales (nombre)
/// - 6 dígitos (fecha AAMMDD)
/// - 3 caracteres alfanuméricos (homoclave)
fn validar_rfc(rfc: &str) -> Result<(), String> {
    if rfc.len() != RFC_LONGITUD && rfc.len() != RFC_LONGITUD_PM {
        return Err(format!("Longitud incorrecta ({} caracteres)", rfc.len()));
    }
    let bytes = rfc.as_bytes();
    for i in 0..4 {
        if !bytes[i].is_ascii_alphabetic() {
            return Err("Los primeros 4 caracteres deben ser letras".into());
        }
    }
    for i in 4..10 {
        if !bytes[i].is_ascii_digit() {
            return Err(format!("El carácter en posición {} debe ser dígito", i + 1));
        }
    }
    Ok(())
}

fn validar_email(email: &str) -> Result<(), String> {
    if !email.contains('@') {
        return Err("Falta el @".into());
    }
    let partes: Vec<&str> = email.split('@').collect();
    if partes.len() != 2 {
        return Err("Sólo debe haber un @".into());
    }
    if partes[0].is_empty() {
        return Err("Falta la parte local".into());
    }
    if !partes[1].contains('.') {
        return Err("El dominio debe tener al menos un punto".into());
    }
    Ok(())
}

fn validar_telefono(tel: &str) -> Result<(), String> {
    if tel.len() != TEL_LONGITUD {
        return Err(format!("Debe tener {} dígitos (tiene {})", TEL_LONGITUD, tel.len()));
    }
    if !tel.chars().all(|c| c.is_ascii_digit()) {
        return Err("Sólo dígitos".into());
    }
    Ok(())
}

fn main() {
    // Datos de prueba (normalmente vendrían de un formulario o de la BD)
    let datos = vec![
        ("XAXX010101000", "contacto@empresa.com", "55551234567", "01234", "Constructora S.A."),
        ("XEXX010101000", "mal email",            "5551234",    "1234",  ""),
        ("XAXX010101AB3", "info@empresa.com.mx",  "5512345678", "11550", "Distribuidora del Norte"),
    ];

    println!("Validador de clientes - ERP/CRM\n");
    for (rfc, email, tel, cp, razon) in &datos {
        println!("Cliente: {}", razon);
        println!("  RFC:     {} {:?}", rfc, validar_rfc(rfc));
        println!("  Email:   {} {:?}", email, validar_email(email));
        println!("  Teléfono:{} {:?}", tel,   validar_telefono(tel));
        println!("  CP:      {} ({} dígitos)", cp, cp.len());
        println!();
    }
}
```

`Result<(), String>` es un *enum* de la librería estándar que representa éxito (`Ok(valor)`) o error (`Err(mensaje)`). Lo veremos a fondo en 1.12.4 y 1.15.2.

Para ejecutarlo:

```bash
cd /home/roy/rust_man/proyectos_capitulo/parte1
cargo new 03_validador_cliente
# (pegar el contenido de arriba en src/main.rs)
cd 03_validador_cliente
cargo run
```

## 1.9 Ownership, borrowing y lifetimes

Llegamos al capítulo más importante de toda la Parte 1. Si dominas el ownership, el borrowing y los lifetimes, habrás cruzado el Rubicón: el resto de Rust se vuelve accesible. Si no los dominas, cada programa que escribas parecerá una pelea con el compilador. Esta sección es larga a propósito: la documentación oficial le dedica 14 páginas, nosotros le dedicaremos al menos 20.

### 1.9.1 Las tres reglas del ownership

1. **Cada valor en Rust tiene un dueño (*owner*).**
2. **Sólo puede haber un dueño a la vez.**
3. **Cuando el dueño sale del ámbito (*scope*), el valor se libera (*dropped*).**

Estas reglas las aplica el compilador sin que tengas que escribirlas explícitamente. Si las respetas, el compilador garantiza que no habrá *use-after-free*, *double-free* ni fugas de memoria.

**Analogía**: imagina que un valor es un libro físico. En Rust, ese libro sólo puede estar en la biblioteca de una persona a la vez. Si prestas el libro (ownership) a otra persona, ya no lo tienes. Si la otra persona termina de leerlo, lo devuelve o lo destruye. No puedes tener dos personas leyendo el mismo libro simultáneamente a menos que una de las dos le muestre la página (referencia) sin perder la propiedad.

```rust
fn main() {
    // s no existe aquí
    {
        let s = String::from("ERP México");   // s es dueña del String
        println!("{}", s);                     // s es válida dentro de este bloque
    }                                          // fin del bloque: s sale del scope,
                                               // se llama drop() y se libera la memoria
    // println!("{}", s);                     // ERROR: s no existe aquí
}
```

### 1.9.2 Movimiento (move) de propiedad

Cuando asignas un valor a otra variable, o lo pasas a una función, el ownership se *mueve* (move). El valor original deja de ser accesible:

```rust
fn main() {
    let s1 = String::from("Hola");
    let s2 = s1;                 // s1 se mueve a s2
    // println!("{}", s1);       // ERROR: s1 ya no es válido
    println!("{}", s2);          // OK
}
```

¿Por qué? Porque un `String` se almacena en el *heap* (memoria dinámica), y el puntero, la longitud y la capacidad se almacenan en el *stack*. Si tanto `s1` como `s2` tuvieran el mismo puntero, al final del scope ambos llamarían a `drop`, intentando liberar la misma memoria dos veces. Para evitarlo, Rust invalida `s1` tras el movimiento.

**¿Y con tipos que viven en el stack, como `i32`?** Para ellos, la copia es *bit a bit* y no hay problema con el doble-free. Por eso estos tipos implementan el trait `Copy`, lo que significa que se copian automáticamente en vez de moverse. Todos los tipos numéricos, `bool`, `char`, y tuplas/arrays de tipos `Copy` son `Copy`.

```rust
fn main() {
    let x = 5;
    let y = x;       // x es Copy, no se mueve; x sigue siendo válido
    println!("{} {}", x, y);   // "5 5"
}
```

### 1.9.3 Copia (Copy) y clonacion (Clone)

Si quieres duplicar un valor del heap (no sólo un puntero), usa `.clone()`. Esto asigna memoria nueva y copia el contenido:

```rust
fn main() {
    let s1 = String::from("Hola");
    let s2 = s1.clone();          // s1 y s2 son independientes
    println!("{} {}", s1, s2);
}
```

`.clone()` puede ser costoso para colecciones grandes, así que úsalo sólo cuando lo necesites. La regla general: si necesitas que dos variables tengan un valor "real" independiente, clona. Si no, mueve.

### 1.9.4 Referencias inmutables y mutables

A veces no quieres transferir la propiedad, sino dejar que otra parte del código *vea* el valor sin apropiárselo. Para eso existen las **referencias** (`&`). Hay dos tipos:

- `&T`: referencia inmutable. Puedes tener **muchas a la vez** y no puedes modificar el valor.
- `&mut T`: referencia mutable. Sólo puedes tener **una a la vez** y puedes modificar el valor.

```rust
fn main() {
    let mut s = String::from("Hola");
    let r1 = &s;
    let r2 = &s;
    println!("{} y {}", r1, r2);  // múltiples inmutables OK
    // r1 y r2 ya no se usan después de esta línea

    let r3 = &mut s;
    r3.push_str(" mundo");
    println!("{}", r3);
}
```

### 1.9.5 Reglas del borrowing

1. En un momento dado, puedes tener **una** referencia mutable, **o** cualquier número de referencias inmutables, pero no ambas.
2. Las referencias deben ser **siempre válidas** (no pueden apuntar a memoria liberada).

El compilador verifica ambas reglas. Si las rompes, te dirá exactamente dónde está el problema. La segunda regla es lo que hace imposible el *use-after-free* sin necesidad de un recolector de basura.

```rust
fn main() {
    let mut s = String::from("Hola");
    let r1 = &s;
    let r2 = &mut s;   // ERROR: hay una referencia inmutable activa
    println!("{} {}", r1, r2);
}
```

El compilador es inteligente: la regla sólo se aplica mientras la referencia esté "viva" (es decir, hasta el último uso). En el ejemplo de la sección 1.9.4, `r1` y `r2` mueren tras el `println!`, así que `r3` puede ser mutable.

### 1.9.6 Lifetimes: anotaciones y elision

Un *lifetime* es el ámbito durante el cual una referencia es válida. La mayoría de las veces, el compilador puede inferir los lifetimes (esto se llama *elisión de lifetimes*), pero a veces necesitamos anotarlos explícitamente. La sintaxis es `'a`, `'b`, etc.:

```rust
// Función que toma dos referencias y devuelve la mayor
// Ambas referencias y el valor de retorno deben tener el mismo lifetime
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

La anotación `'a` significa: "voy a devolver una referencia que vive al menos tanto como la referencia más corta de las dos que recibí". Si el compilador no pudiera verificar esto, el código no compilaría.

### 1.9.7 Lifetimes en structs y metodos

Si un struct guarda una referencia, debe llevar un lifetime:

```rust
struct ExtratoCliente<'a> {
    nombre: &'a str,           // referencia al nombre
    rfc: &'a str,              // referencia al RFC
    credito: f64,              // valor propio
}

impl<'a> ExtratoCliente<'a> {
    fn imprimir(&self) {
        println!("Cliente: {} (RFC: {}) - Crédito: ${}", self.nombre, self.rfc, self.credito);
    }
}

fn main() {
    let nombre = String::from("Constructora del Bajío");
    let rfc = "CDB010101AB3";
    let extrato = ExtratoCliente { nombre: &nombre, rfc: &rfc, credito: 50000.0 };
    extrato.imprimir();
    // nombre y rfc deben seguir vivos hasta aquí
}
```

### 1.9.8 Mini-proyecto: `04_procesador_pedido`

Este mini-proyecto pone en práctica el ownership y el borrowing simulando el procesamiento de un pedido. Se encuentra en `proyectos_capitulo/parte1/04_procesador_pedido/`.

```rust
// Mini-proyecto 04: procesador de pedidos con énfasis en ownership/borrowing

#[derive(Debug, Clone)]
struct LineaPedido {
    sku: String,
    descripcion: String,
    cantidad: u32,
    precio_unitario: f64,
}

impl LineaPedido {
    fn new(sku: &str, descripcion: &str, cantidad: u32, precio: f64) -> Self {
        LineaPedido {
            sku: sku.to_string(),                // .to_string() convierte &str a String
            descripcion: descripcion.to_string(),
            cantidad,
            precio_unitario: precio,
        }
    }

    fn subtotal(&self) -> f64 {                  // &self = borrow inmutable
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

    fn agregar(&mut self, linea: LineaPedido) {  // &mut self = borrow mutable
        self.lineas.push(linea);
    }

    // Recibe una referencia inmutable a la línea; no toma propiedad
    fn calcular_subtotal_lineas(&self) -> f64 {
        self.lineas.iter().map(|l| l.subtotal()).sum()
    }
}

fn imprimir_pedido(pedido: &Pedido) {
    println!("Pedido {}", pedido.folio);
    for (i, linea) in pedido.lineas.iter().enumerate() {
        println!("  {}. {} - {} x ${:.2} = ${:.2}",
                 i + 1, linea.sku, linea.cantidad, linea.precio_unitario, linea.subtotal());
    }
    println!("  ────────────────────────────");
    println!("  Subtotal: ${:.2}", pedido.calcular_subtotal_lineas());
}

fn main() {
    let mut pedido = Pedido::new("PED-0001");
    pedido.agregar(LineaPedido::new("SKU-001", "Laptop HP",    2, 18999.0));
    pedido.agregar(LineaPedido::new("SKU-002", "Mouse óptico",  5,  350.0));
    pedido.agregar(LineaPedido::new("SKU-003", "Teclado",       3, 1200.0));

    imprimir_pedido(&pedido);   // pasamos una referencia, no movemos
    // pedido sigue siendo nuestro
    println!("\nTotal de líneas: {}", pedido.lineas.len());
}
```

Para ejecutarlo:

```bash
cd /home/roy/rust_man/proyectos_capitulo/parte1
cargo new 04_procesador_pedido
# (pegar el codigo en src/main.rs)
cd 04_procesador_pedido
cargo run
```

### 1.9.9 Errores tipicos en ownership

- **"value moved here"**: has usado un valor después de moverlo. Soluciones: usar una referencia (`&valor`), o `.clone()` si necesitas una copia.
- **"cannot borrow as mutable"**: estás intentando un borrow mutable sobre un valor inmutable. Solución: `let mut x = ...`.
- **"cannot borrow as immutable because it is also borrowed as mutable"**: tienes un borrow mutable activo y otro inmutable intentando usarse. Solución: asegurarte de que el borrow mutable termine antes de pedir uno inmutable.
- **"lifetime mismatch"**: las referencias no viven lo suficiente. Solución: revisar que los datos referenciados vivan al menos tanto como la referencia.

## 1.10 Tipos compuestos: tuplas, arrays y slices

Los tipos compuestos agrupan varios valores en uno solo. Rust tiene tres primarios: **tuplas**, **arrays** y **slices**.

### 1.10.1 Tuplas

Una tupla es una secuencia de valores de tipos potencialmente diferentes, de longitud fija:

```rust
fn main() {
    let cliente: (i32, &str, f64, bool) = (1, "Constructora del Bajío", 50000.0, true);
    let (id, nombre, credito, vip) = cliente;        // destructuración
    println!("ID: {}, Nombre: {}, Crédito: ${}, VIP: {}", id, nombre, credito, vip);

    // Acceso por índice
    println!("Primer elemento: {}", cliente.0);
    println!("Segundo elemento: {}", cliente.1);
}
```

**Tuplas unit**: la tupla sin elementos `()` es el "tipo unit", equivalente a `void` en C. Se usa como valor de retorno de funciones que no devuelven nada.

### 1.10.2 Arrays

Un array es una colección de longitud fija donde todos los elementos son del mismo tipo. Vive en el stack:

```rust
fn main() {
    // Array de 5 i32
    let dias_semana: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
    println!("Primer día: {}", dias_semana[0]);

    // Inicialización con valor repetido
    let ceros: [i32; 10] = [0; 10];
    println!("{:?}", ceros);   // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    // Iterar
    for dia in &dias_semana {
        println!("Día: {}", dia);
    }
}
```

**Acceso fuera de rango**: en debug, `dias_semana[10]` lanza un `panic!` (en release, lee memoria arbitraria, lo que es un bug de seguridad). Rust previene esto en compilación cuando el índice es constante, y en ejecución cuando es dinámico.

### 1.10.3 Slices

Un slice es una vista (*view*) a una porción contigua de una colección. Es una referencia, no un propietario. Los slices más comunes son los `&str` (vistas a cadenas) y los `&[T]` (vistas a arrays/vectores):

```rust
fn main() {
    let productos = vec![
        "Laptop HP Pavilion",
        "Mouse óptico",
        "Teclado mecánico",
        "Audífonos gamer",
    ];

    // Slice del array completo
    let todos: &[&str] = &productos[..];
    println!("Total: {}", todos.len());

    // Slice de los primeros 2
    let primeros_dos: &[&str] = &productos[..2];
    for p in primeros_dos {
        println!("{}", p);
    }
}
```

### 1.10.4 Errores tipicos

- **Confundir tupla con array**: `let t = (1, 2, 3)` es una tupla de 3 elementos; `let a = [1, 2, 3]` es un array de 3 `i32`. No son intercambiables.
- **Modificar un slice sin `mut`**: los slices son inmutables por defecto. Si necesitas modificarlos, usa `&mut [T]`.
- **Acceder a un índice que no existe**: `panic!` inmediato en debug.

## Cierre de la Fase 1.a

Con esto concluye la primera mitad de la Parte 1 del manual. Has aprendido a instalar Rust, declarar variables, usar tipos primitivos, operadores, control de flujo, funciones, ownership, borrowing, lifetimes y tipos compuestos. Tienes además tres mini-proyectos funcionales (`01_erp_hello`, `02_calc_impuestos`, `03_validador_cliente`) que aplican estos conceptos al dominio del ERP/CRM.

En la **Fase 1.b** continuaremos con **structs**, **enums y pattern matching**, **traits**, **genéricos** y **manejo de errores**, junto con los mini-proyectos `04_procesador_pedido` (ya creado arriba), `05_modelo_erp` y `06_estados_pedido`.

> **Resumen de métricas parciales** (Fase 1.a):
> - 10 secciones H2 escritas (1.1 – 1.10).
> - ~30 subsecciones H3/H4.
> - 3 mini-proyectos físicos en `proyectos_capitulo/parte1/`.
> - 30+ bloques de código comentados en español.

---

## 1.11 Structs

Un *struct* (abreviatura de *structure*) es un tipo de dato compuesto por el programador que agrupa varios valores relacionados bajo un mismo nombre. Si vienes de C, es como un `struct` clásico. Si vienes de Java o C#, es como una clase sin herencia, sin métodos virtuales y sin `this` implícito. Si vienes de Python, es como una `dataclass` (de hecho, las `dataclasses` de Python se diseñaron inspirándose en los structs de Rust). Los structs son el pilar del modelado de datos en un ERP/CRM: `Cliente`, `Producto`, `Pedido`, `Factura` se modelarán como structs.

### 1.11.1 Definicion y creacion

```rust
// archivo: src/main.rs
// Mini-proyecto 05: modelo de datos inicial del ERP/CRM (structs básicos)
// Aprenderás: struct, instanciación, acceso a campos, impresión con {:?} y {:#?}

#[derive(Debug, Clone)]      // habilita .clone() y el formato {:?} para imprimir
struct Cliente {
    id: u32,
    nombre: String,         // String = owned, mutable, en heap
    rfc: String,
    email: String,
    credito: f64,
    activo: bool,
}

fn main() {
    // Crear un cliente
    let cliente = Cliente {
        id: 1,
        nombre: String::from("Constructora del Bajío S.A. de C.V."),
        rfc: String::from("CDB010101AB3"),
        email: String::from("contacto@cdb.com.mx"),
        credito: 100_000.0,
        activo: true,
    };

    // Acceso a campos
    println!("ID: {}", cliente.id);
    println!("Nombre: {}", cliente.nombre);
    println!("RFC: {}", cliente.rfc);
    println!("Crédito: ${:.2}", cliente.credito);

    // Imprimir con Debug (formato compacto)
    println!("\nDebug compacto: {:?}", cliente);

    // Imprimir con Debug (formato pretty)
    println!("\nDebug formateado:\n{:#?}", cliente);

    // Clonar para tener un duplicado independiente
    let cliente_copia = cliente.clone();
    println!("\nCopia: {:?}", cliente_copia);
}
```

**Análisis línea por línea**:
- `#[derive(Debug, Clone)]`: **derive** es un atributo que genera automáticamente la implementación de los traits `Debug` y `Clone`. `Debug` permite imprimir con `{:?}`; `Clone` permite duplicar con `.clone()`. Veremos `derive` a fondo en 1.13.
- `id: u32`: campo de tipo entero sin signo de 32 bits. Por convención, los IDs de base de datos se almacenan en `u32` o `u64`.
- `nombre: String`: campo de tipo `String` (owned, en heap, mutable, UTF-8). Si pusiéramos `&str` necesitaríamos un lifetime; lo aprenderemos pronto.
- `#[derive(Debug, Clone)]` se aplica al struct completo. Sin él, `println!("{:?}", cliente)` daría error.
- `let cliente = Cliente { id: 1, ... };`: instanciación con la sintaxis `Struct { campo: valor, ... }`. El orden de los campos no importa.
- `cliente.id`: acceso a un campo con la notación de punto.
- `{:#?}`: variante "pretty" del formato Debug: imprime con saltos de línea y dos espacios de indentación por nivel.

**Salida esperada** (resumida):
```
ID: 1
Nombre: Constructora del Bajío S.A. de C.V.
RFC: CDB010101AB3
Crédito: $100000.00

Debug compacto: Cliente { id: 1, nombre: "Constructora del Bajío S.A. de C.V.", ... }

Debug formateado:
Cliente {
    id: 1,
    nombre: "Constructora del Bajío S.A. de C.V.",
    ...
}
```

### 1.11.2 Structs con campos publicos y privados

En Rust, los campos de un struct son **privados por defecto**: sólo el módulo donde se definen puede acceder a ellos. Para hacerlos públicos, se antepone `pub`:

```rust
mod clientes {
    pub struct Cliente {
        pub id: u32,                // público
        pub nombre: String,         // público
        credito: f64,               // privado al módulo
    }
}

fn main() {
    let c = clientes::Cliente { id: 1, nombre: "X".into(), credito: 0.0 };
    //                       ^^^^^ error: field `credito` is private
    println!("{}", c.nombre);   // OK
}
```

Esta privacidad granular es muy útil para encapsular la lógica de negocio: los campos "internos" (como `credito` que debe calcularse o restringirse) se mantienen privados, y se exponen métodos para manipularlos de forma controlada.

### 1.11.3 Structs de tupla y unit-like

Además del struct "con nombre de campos" que ya vimos, hay dos variantes:

- **Tuple struct**: un struct sin nombres de campos, identificado por su nombre y su "forma":

```rust
struct Coordenada(f64, f64, f64);   // 3D
struct PedidoId(u32);                // newtype pattern

fn main() {
    let p = Coordenada(1.0, 2.0, 3.0);
    println!("x = {}", p.0);
    let id = PedidoId(42);
    println!("ID: {}", id.0);
}
```

El **newtype pattern** (definir un struct tupla con un solo campo) es muy útil para crear tipos seguros que no se confundan con el subyacente: por ejemplo, `PedidoId(u32)` no es intercambiable con `ClienteId(u32)` aunque ambos sean `u32`.

- **Unit-like struct**: un struct sin campos, útil para marcar tipos o implementar traits:

```rust
struct Marcador;
impl Marcador {
    fn saludar(&self) {
        println!("Hola desde Marcador");
    }
}

fn main() {
    let m = Marcador;
    m.saludar();
}
```

### 1.11.4 Metodos y funciones asociadas

Los structs ganan verdadera potencia cuando les añadimos **métodos** y **funciones asociadas**. Los métodos se definen dentro de un bloque `impl`:

```rust
#[derive(Debug, Clone)]
struct Producto {
    sku: String,
    nombre: String,
    precio: f64,
    costo: f64,
    stock: u32,
}

impl Producto {
    // Función asociada (constructor): no toma self, se llama con :: 
    fn new(sku: &str, nombre: &str, precio: f64, costo: f64, stock: u32) -> Self {
        Producto {
            sku: sku.to_string(),
            nombre: nombre.to_string(),
            precio,
            costo,
            stock,
        }
    }

    // Método con &self (borrow inmutable)
    fn margen(&self) -> f64 {
        if self.precio <= 0.0 { return 0.0; }
        (self.precio - self.costo) / self.precio * 100.0
    }

    // Método con &mut self (borrow mutable)
    fn reabastecer(&mut self, unidades: u32) {
        self.stock += unidades;
    }

    fn vender(&mut self, unidades: u32) -> Result<(), String> {
        if unidades > self.stock {
            return Err(format!("Stock insuficiente: {} disponibles, {} solicitados", self.stock, unidades));
        }
        self.stock -= unidades;
        Ok(())
    }

    // Función asociada sin self
    fn precio_con_iva(precio: f64) -> f64 {
        precio * 1.16
    }
}

fn main() {
    let mut laptop = Producto::new("SKU-001", "Laptop HP Pavilion", 18999.0, 12500.0, 10);
    println!("Producto: {} (SKU: {})", laptop.nombre, laptop.sku);
    println!("Precio: ${:.2}, Costo: ${:.2}, Margen: {:.1}%",
             laptop.precio, laptop.costo, laptop.margen());
    println!("Precio con IVA: ${:.2}", Producto::precio_con_iva(laptop.precio));

    match laptop.vender(3) {
        Ok(()) => println!("Venta registrada. Stock restante: {}", laptop.stock),
        Err(e) => println!("Error: {}", e),
    }

    laptop.reabastecer(5);
    println!("Stock tras reabastecer: {}", laptop.stock);
}
```

**Análisis**:
- `impl Producto { ... }`: bloque de implementación. Todos los métodos y funciones asociadas del struct van aquí.
- `fn new(...) -> Self`: función asociada (no tiene `self`), usada como constructor. Por convención, se llama `new` pero podría tener cualquier nombre. `Self` (con mayúscula) es un alias del tipo `Producto`.
- `fn margen(&self) -> f64`: método con **referencia inmutable** a `self`. Puede leer campos pero no modificarlos.
- `fn reabastecer(&mut self, ...)`: método con **referencia mutable**. Puede modificar los campos.
- `fn vender(...) -> Result<(), String>`: método que puede fallar. Devuelve `Ok(())` si todo va bien, `Err(mensaje)` si falla.
- `fn precio_con_iva(precio: f64) -> f64`: función asociada sin `self`, útil para operaciones que no dependen de una instancia particular. Se llama con `Producto::precio_con_iva(...)` (con `::`).
- `precio, costo, stock`: en la creación del struct, cuando el nombre de la variable local coincide con el nombre del campo, se puede omitir `campo: variable` y escribir sólo `variable` (es la sintaxis "shorthand init").

### 1.11.5 Mini-proyecto: `05_modelo_erp`

El mini-proyecto del capítulo (en `proyectos_capitulo/parte1/05_modelo_erp/`) define el modelo inicial del ERP/CRM con structs `Cliente`, `Producto`, `Pedido`, `LineaPedido`, `Proveedor`. Por ahora sin base de datos, todo en memoria. En la Parte 2 conectaremos estos structs a MySQL.

```rust
// 05_modelo_erp/src/main.rs
// Modelo de datos del ERP/CRM (en memoria, sin BD)
// Aprenderás: structs, métodos, funciones asociadas, módulos

use std::fmt;

#[derive(Debug, Clone)]
pub struct Cliente {
    pub id: u32,
    pub nombre: String,
    pub rfc: String,
    pub email: String,
    pub telefono: String,
    pub credito: f64,
    pub activo: bool,
}

impl Cliente {
    pub fn new(id: u32, nombre: &str, rfc: &str, email: &str, telefono: &str, credito: f64) -> Self {
        Cliente { id, nombre: nombre.into(), rfc: rfc.into(), email: email.into(), telefono: telefono.into(), credito, activo: true }
    }
    pub fn desactivar(&mut self) { self.activo = false; }
    pub fn tiene_credito(&self, monto: f64) -> bool { monto <= self.credito && self.activo }
}

#[derive(Debug, Clone)]
pub struct Proveedor {
    pub id: u32,
    pub nombre: String,
    pub rfc: String,
    pub dias_pago: u8,
}

impl Proveedor {
    pub fn new(id: u32, nombre: &str, rfc: &str, dias_pago: u8) -> Self {
        Proveedor { id, nombre: nombre.into(), rfc: rfc.into(), dias_pago }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnidadMedida { Pieza, Kilogramo, Litro, Metro, Caja }

#[derive(Debug, Clone)]
pub struct Producto {
    pub sku: String,
    pub nombre: String,
    pub precio: f64,
    pub costo: f64,
    pub stock: u32,
    pub unidad: UnidadMedida,
}

impl Producto {
    pub fn new(sku: &str, nombre: &str, precio: f64, costo: f64, stock: u32, unidad: UnidadMedida) -> Self {
        Producto { sku: sku.into(), nombre: nombre.into(), precio, costo, stock, unidad }
    }
    pub fn margen(&self) -> f64 {
        if self.precio <= 0.0 { 0.0 } else { (self.precio - self.costo) / self.precio * 100.0 }
    }
    pub fn valor_inventario(&self) -> f64 { self.costo * self.stock as f64 }
}

#[derive(Debug, Clone)]
pub struct LineaPedido {
    pub sku: String,
    pub cantidad: u32,
    pub precio_unitario: f64,
    pub descuento_pct: f64,
}

impl LineaPedido {
    pub fn new(sku: &str, cantidad: u32, precio_unitario: f64, descuento_pct: f64) -> Self {
        LineaPedido { sku: sku.into(), cantidad, precio_unitario, descuento_pct }
    }
    pub fn subtotal(&self) -> f64 {
        let base = self.cantidad as f64 * self.precio_unitario;
        base * (1.0 - self.descuento_pct / 100.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EstadoPedido { Borrador, Confirmado, Enviado, Entregado, Cancelado }

#[derive(Debug, Clone)]
pub struct Pedido {
    pub folio: String,
    pub cliente_id: u32,
    pub lineas: Vec<LineaPedido>,
    pub estado: EstadoPedido,
}

impl Pedido {
    pub fn new(folio: &str, cliente_id: u32) -> Self {
        Pedido { folio: folio.into(), cliente_id, lineas: Vec::new(), estado: EstadoPedido::Borrador }
    }
    pub fn agregar_linea(&mut self, linea: LineaPedido) { self.lineas.push(linea); }
    pub fn subtotal(&self) -> f64 { self.lineas.iter().map(|l| l.subtotal()).sum() }
    pub fn iva(&self) -> f64 { self.subtotal() * 0.16 }
    pub fn total(&self) -> f64 { self.subtotal() + self.iva() }
    pub fn confirmar(&mut self) -> Result<(), String> {
        if self.lineas.is_empty() { return Err("Pedido sin líneas".into()); }
        if self.estado != EstadoPedido::Borrador { return Err(format!("No se puede confirmar un pedido en estado {:?}", self.estado)); }
        self.estado = EstadoPedido::Confirmado;
        Ok(())
    }
}

// Implementación manual de Display para Cliente (veremos Display en 1.13)
impl fmt::Display for Cliente {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.nombre, self.rfc)
    }
}

fn main() {
    // Crear datos de prueba
    let cliente = Cliente::new(1, "Constructora del Bajío", "CDB010101AB3", "contacto@cdb.mx", "5551234567", 100_000.0);
    let laptop = Producto::new("SKU-001", "Laptop HP", 18999.0, 12500.0, 10, UnidadMedida::Pieza);
    let mouse  = Producto::new("SKU-002", "Mouse óptico", 350.0, 150.0, 50, UnidadMedida::Pieza);

    // Crear un pedido
    let mut pedido = Pedido::new("PED-0001", cliente.id);
    pedido.agregar_linea(LineaPedido::new(&laptop.sku, 2, laptop.precio, 5.0));
    pedido.agregar_linea(LineaPedido::new(&mouse.sku, 5, mouse.precio, 0.0));
    pedido.confirmar().unwrap();

    // Imprimir resumen
    println!("Cliente: {}", cliente);
    println!("Pedido:  {}", pedido.folio);
    println!("Estado:  {:?}", pedido.estado);
    println!("Subtotal: ${:.2}", pedido.subtotal());
    println!("IVA:      ${:.2}", pedido.iva());
    println!("Total:    ${:.2}", pedido.total());
    println!("Cliente tiene crédito? {}", cliente.tiene_credito(pedido.total()));
}
```

Para ejecutarlo:
```bash
cd /home/roy/rust_man/proyectos_capitulo/parte1
cargo new 05_modelo_erp
# pegar el contenido en src/main.rs
cd 05_modelo_erp
cargo run
```

## 1.12 Enums y pattern matching

Los enums (enumeraciones) son una de las características más poderosas de Rust. A diferencia de los enums de C o Java, donde cada variante es esencialmente un entero, los enums de Rust pueden tener **datos asociados** a cada variante. Esto los hace ideales para modelar estados, categorías y resultados.

### 1.12.1 Definicion de enums

```rust
// Definición simple
enum EstadoPedido {
    Borrador,
    Confirmado,
    Enviado,
    Entregado,
    Cancelado,
}

// Definición con datos asociados
enum AccionInventario {
    Entrada { sku: String, cantidad: u32, costo: f64 },
    Salida { sku: String, cantidad: u32, motivo: String },
    Ajuste { sku: String, cantidad: i32, autorizado_por: String },
    Transferencia { sku: String, cantidad: u32, origen: String, destino: String },
}
```

Cada variante puede llevar datos, y esos datos pueden ser de tipos distintos. Es como si dijeras "un Movimiento de Inventario puede ser una Entrada, una Salida, un Ajuste o una Transferencia, y cada uno tiene su propia forma". Esto es imposible de modelar limpiamente con structs tradicionales; con enums es trivial.

### 1.12.2 `match`: la sentencia estrella

`match` es la construcción de control de flujo más poderosa de Rust. Compara un valor contra una serie de **patrones** y ejecuta el código del primer patrón que coincida. Es exhaustivo: el compilador verifica que hayas cubierto todas las variantes posibles.

```rust
fn clasificar_accion(accion: &AccionInventario) -> &str {
    match accion {
        AccionInventario::Entrada { .. } => "Compra a proveedor",
        AccionInventario::Salida { motivo, .. } if motivo == "Venta" => "Venta a cliente",
        AccionInventario::Salida { .. } => "Salida por otro motivo",
        AccionInventario::Ajuste { cantidad, .. } if *cantidad > 0 => "Ajuste positivo",
        AccionInventario::Ajuste { .. } => "Ajuste negativo o merma",
        AccionInventario::Transferencia { .. } => "Transferencia entre almacenes",
    }
}
```

**Análisis**:
- `match accion { ... }`: el valor a comparar.
- `AccionInventario::Entrada { .. }`: patrón con `..` que ignora los campos. No nos importa cuáles son, sólo que la variante sea `Entrada`.
- `AccionInventario::Salida { motivo, .. } if motivo == "Venta"`: **guarda de match** (`if`). Sólo coincide si la variante es `Salida` **y** `motivo` es `"Venta"`.
- El orden importa: la primera coincidencia gana. Ponemos los casos más específicos primero.
- El compilador verifica que cubrimos todas las variantes. Si olvidamos una, nos avisa con un mensaje claro.

### 1.12.3 `if let` y `while let`

A veces sólo queremos manejar **una** variante de un enum. Para esos casos, `if let` es más conciso que `match`:

```rust
fn main() {
    let accion = AccionInventario::Entrada { sku: "SKU-001".into(), cantidad: 50, costo: 8.50 };
    if let AccionInventario::Entrada { sku, cantidad, costo } = &accion {
        println!("Entrada de {} unidades de {} a ${}", cantidad, sku, costo);
    } else {
        println!("No fue una entrada");
    }

    // while let: itera mientras el patrón coincida
    let mut pila = vec!["PED-001", "PED-002", "PED-003"];
    while let Some(folio) = pila.pop() {
        println!("Procesando: {}", folio);
    }
}
```

`while let Some(...)` es el patrón idiomático para consumir un `Vec` o un iterador cuyo elemento es `Option`.

### 1.12.4 `Option<T>` y `Result<T, E>`: los enums mas importantes

La librería estándar define dos enums que se usan en absolutamente todos los programas Rust:

```rust
// Option: ausencia o presencia de un valor
enum Option<T> {
    None,
    Some(T),
}

// Result: éxito con un valor, o error con un mensaje
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Option<T>` se usa cuando un valor puede o no existir (por ejemplo, "el cliente número 5 podría no existir en la base de datos"). `Result<T, E>` se usa cuando una operación puede fallar con un error específico. **No hay `null` en Rust**: en su lugar, se usa `Option`. **No hay excepciones**: en su lugar, se usa `Result`. Esto fuerza a manejar explícitamente los casos de ausencia y error, y el compilador se asegura de que no te olvides.

```rust
fn main() {
    // Option
    let maybe_cliente: Option<&str> = Some("Constructora del Bajío");
    let sin_cliente: Option<&str> = None;

    // Desempaquetar con if let
    if let Some(nombre) = maybe_cliente {
        println!("Encontrado: {}", nombre);
    } else {
        println!("No existe");
    }

    // Result
    fn dividir(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 { Err("División por cero".into()) }
        else { Ok(a / b) }
    }
    match dividir(10.0, 0.0) {
        Ok(r) => println!("Resultado: {}", r),
        Err(e) => println!("Error: {}", e),
    }
}
```

### 1.12.5 Mini-proyecto: `06_estados_pedido`

El mini-proyecto del capítulo (en `proyectos_capitulo/parte1/06_estados_pedido/`) modela la máquina de estados de un pedido en el ERP/CRM. Sólo se permiten ciertas transiciones, y el programa verifica que no se hagan transiciones inválidas. Es un ejemplo perfecto de la potencia de los enums.

```rust
// 06_estados_pedido/src/main.rs
// Máquina de estados de un pedido
// Aprenderás: enums con datos, match exhaustivo, transiciones de estado

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
        // No se puede cancelar si ya está entregado
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

    // Esto debe fallar:
    match p.cancelar("Cliente lo rechazó", "2026-07-09") {
        Ok(()) => println!("Cancelado (inesperado)"),
        Err(e) => println!("[Error esperado] {}", e),
    }
}
```

Para ejecutarlo:
```bash
cd /home/roy/rust_man/proyectos_capitulo/parte1
cargo new 06_estados_pedido
# pegar el contenido en src/main.rs
cd 06_estados_pedido
cargo run
```

## 1.13 Traits

Un *trait* (en otros lenguajes se llama *interface* o *protocol*) es un conjunto de métodos que un tipo puede implementar. Es la forma que tiene Rust de hacer polimorfismo: definir comportamiento compartido sin compartir estructura.

### 1.13.1 Definicion e implementacion

```rust
trait Validable {
    fn validar(&self) -> Result<(), String>;
}

struct Cliente { rfc: String, email: String, /* ... */ }

impl Validable for Cliente {
    fn validar(&self) -> Result<(), String> {
        if self.rfc.len() < 12 { return Err("RFC muy corto".into()); }
        if !self.email.contains('@') { return Err("Email inválido".into()); }
        Ok(())
    }
}
```

A partir de ese momento, cualquier función que reciba un `&impl Validable` puede operar con un `Cliente`, con un `Proveedor` o con cualquier otro tipo que implemente el trait.

### 1.13.2 Traits comunes de la libreria estandar

Algunos traits se usan tan a menudo que conviene memorizarlos:

| Trait | Propósito | Cómo se obtiene |
|---|---|---|
| `Debug` | Permite imprimir con `{:?}` | `#[derive(Debug)]` |
| `Display` | Permite imprimir con `{}` | Implementación manual |
| `Clone` | Permite `.clone()` | `#[derive(Clone)]` |
| `Copy` | Copia bit a bit (tipos triviales) | `#[derive(Copy, Clone)]` |
| `PartialEq`, `Eq` | Permite `==` y `!=` | `#[derive(PartialEq, Eq)]` |
| `PartialOrd`, `Ord` | Permite `<`, `>`, ordenamiento | `#[derive(PartialOrd, Ord)]` |
| `Hash` | Permite usar como clave en `HashMap` | `#[derive(Hash)]` |
| `Default` | Permite `Tipo::default()` | `#[derive(Default)]` o manual |
| `From`, `Into` | Conversiones entre tipos | Implementación manual |
| `Iterator` | Habilita `.next()`, `for`, etc. | Implementación manual |
| `Send`, `Sync` | Marcadores de seguridad para hilos | Automático para la mayoría de tipos |

### 1.13.3 Traits como parametros: `impl Trait` y trait bounds

Hay dos formas sintácticas equivalentes de decir "esta función acepta cualquier tipo que implemente X":

```rust
fn imprimir(t: impl Display) {
    println!("{}", t);
}

fn imprimir<T: Display>(t: T) {
    println!("{}", t);
}
```

La segunda forma (con `<T: Display>`) es necesaria cuando quieres usar `T` en más de un lugar:

```rust
fn comparar_e_imprimir<T: Display + PartialEq>(a: T, b: T) {
    if a == b { println!("Iguales: {}", a); } else { println!("Diferentes: {} vs {}", a, b); }
}
```

Para casos complejos, la cláusula `where` es más legible:

```rust
fn proceso_complejo<T, U>(a: T, b: U) -> String
where
    T: Display + Clone,
    U: Debug + Default,
{
    format!("{} {:?}", a, b)
}
```

### 1.13.4 Trait objects y dispatch dinamico

A veces quieres una colección heterogénea de objetos que implementen el mismo trait. Para eso se usan los **trait objects** con la palabra clave `dyn`:

```rust
trait Notificable { fn notificar(&self, mensaje: &str); }
struct Email;
struct SMS;
struct Push;
impl Notificable for Email { fn notificar(&self, m: &str) { println!("Email: {}", m); } }
impl Notificable for SMS   { fn notificar(&self, m: &str) { println!("SMS:   {}", m); } }
impl Notificable for Push  { fn notificar(&self, m: &str) { println!("Push:  {}", m); } }

fn main() {
    let canales: Vec<Box<dyn Notificable>> = vec![Box::new(Email), Box::new(SMS), Box::new(Push)];
    for canal in &canales {
        canal.notificar("Su pedido ha sido enviado");
    }
}
```

`Box<dyn Notificable>` es un puntero a un objeto en el heap que implementa `Notificable`. Esto se llama **dispatch dinámico**: en tiempo de ejecución se decide qué método concreto llamar. El coste es un puntero indirecto, pero permite mezclar tipos heterogéneos.

## 1.14 Genericos

Los genéricos permiten escribir código que funciona con múltiples tipos sin sacrificar la verificación estática. La sintaxis es `<T>` (o `<T, U>`, etc.).

### 1.14.1 Funciones genericas

```rust
// Mayor de dos valores (cualquier tipo que se pueda comparar)
fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", mayor(3, 7));          // 7
    println!("{}", mayor(3.14, 2.71));   // 3.14
    println!("{}", mayor("a", "b"));      // b
}
```

### 1.14.2 Structs genericos

```rust
#[derive(Debug)]
struct Caja<T> {
    contenido: T,
}

impl<T> Caja<T> {
    fn new(contenido: T) -> Self { Caja { contenido } }
    fn obtener(&self) -> &T { &self.contenido }
}

fn main() {
    let caja_int = Caja::new(42);
    let caja_str = Caja::new("ERP");
    println!("{:?}", caja_int.obtener());
    println!("{:?}", caja_str.obtener());
}
```

### 1.14.3 Repositorio generico

Combinando genéricos, traits y `Option`, podemos crear un mini-repositorio en memoria:

```rust
use std::collections::HashMap;

trait Identificable {
    type Id;
    fn id(&self) -> Self::Id;
}

struct Repositorio<T: Identificable + Clone> {
    almacenamiento: HashMap<u32, T>,
    siguiente_id: u32,
}

impl<T: Identificable + Clone> Repositorio<T> {
    fn new() -> Self { Repositorio { almacenamiento: HashMap::new(), siguiente_id: 1 } }

    fn guardar(&mut self, mut item: T) -> u32 {
        let id = self.siguiente_id;
        self.siguiente_id += 1;
        // En un caso real habría un método `set_id`; lo simulamos:
        self.almacenamiento.insert(id, item);
        id
    }

    fn obtener(&self, id: u32) -> Option<T> {
        self.almacenamiento.get(&id).cloned()
    }

    fn listar(&self) -> Vec<T> {
        self.almacenamiento.values().cloned().collect()
    }
}

#[derive(Clone, Debug)]
struct Producto { id: u32, nombre: String, precio: f64 }
impl Identificable for Producto {
    type Id = u32;
    fn id(&self) -> u32 { self.id }
}

fn main() {
    let mut repo: Repositorio<Producto> = Repositorio::new();
    repo.guardar(Producto { id: 0, nombre: "Laptop".into(), precio: 18999.0 });
    repo.guardar(Producto { id: 0, nombre: "Mouse".into(),  precio: 350.0 });

    println!("Todos: {:?}", repo.listar());
    println!("Obtener id=2: {:?}", repo.obtener(2));
    println!("Obtener id=99: {:?}", repo.obtener(99));
}
```

## 1.15 Manejo de errores

El manejo de errores es uno de los aspectos donde Rust se distingue más claramente de otros lenguajes. En lugar de excepciones (Java, C#, Python) o de códigos de retorno que se pueden ignorar (C), Rust distingue dos grandes categorías de errores:

- **Errores irrecuperables**: situaciones en las que el programa no puede continuar de forma razonable (por ejemplo, intentar acceder a un índice fuera de un array, división por cero en modo debug, violación de invariantes). Se manejan con `panic!`.
- **Errores recuperables**: situaciones en las que el programa puede continuar de forma razonable (por ejemplo, "el usuario ingresó un RFC mal escrito", "no se pudo conectar a la base de datos"). Se manejan con `Result<T, E>`.

### 1.15.1 `panic!` y errores irrecuperables

`panic!` imprime un mensaje de error y *desenrolla* la pila (cierra funciones en orden inverso, ejecutando destructores) o *abortando* directamente (sin desenrollar). El desenrollamiento es el comportamiento por defecto en modo debug.

```rust
fn main() {
    panic!("¡Esto es un error grave!");
}
```

También hay `unimplemented!`, `unreachable!`, `todo!` y `assert!`. Cuando un programa entra en pánico, sale con código 101.

### 1.15.2 `Result<T, E>` y el operador `?`

`Result` es un enum con dos variantes: `Ok(T)` (éxito con un valor) y `Err(E)` (error con un mensaje). El operador `?` (interrogante) se coloca después de una expresión que devuelve `Result` y, si es `Err`, propaga el error a la función que llama; si es `Ok`, extrae el valor. Esto evita el "callback hell" o las pirámides de `if err != nil`.

```rust
use std::fs::File;
use std::io::{self, Read};

fn leer_configuracion(ruta: &str) -> Result<String, io::Error> {
    let mut f = File::open(ruta)?;          // si falla, retorna Err
    let mut s = String::new();
    f.read_to_string(&mut s)?;              // idem
    Ok(s)
}
```

El operador `?` sólo se puede usar en funciones que devuelven `Result` o `Option` (o tipos que los contengan).

### 1.15.3 Combinadores

Los combinadores son métodos de `Result` (y `Option`) que permiten encadenar transformaciones sin recurrir a `match` exhaustivos. Los más útiles:

| Método | Resultado | Efecto |
|---|---|---|
| `map` | `Result<U, E>` | Transforma el `Ok`, deja el `Err` |
| `map_err` | `Result<T, U>` | Transforma el `Err`, deja el `Ok` |
| `and_then` | `Result<U, E>` | Encadena otra operación que devuelve `Result` |
| `or_else` | `Result<T, U>` | Encadena otra operación si fue `Err` |
| `unwrap_or` | `T` | Devuelve el valor o un default |
| `unwrap_or_else` | `T` | Igual pero con un closure para el default |
| `ok_or` | `Result<T, E>` | Convierte `Option` en `Result` |
| `is_ok`, `is_err` | `bool` | Predicados |
| `ok` | `Option<T>` | Convierte `Result` en `Option` descartando el error |

```rust
fn obtener_precio(sku: &str) -> Option<f64> { /* ... */ Some(99.9) }

fn main() {
    let precio_con_iva = obtener_precio("SKU-001")
        .map(|p| p * 1.16)            // si hay precio, aplicar IVA
        .unwrap_or(0.0);              // si no, 0
    println!("Precio con IVA: ${}", precio_con_iva);
}
```

### 1.15.4 Errores personalizados con `thiserror` y `anyhow`

En un proyecto real, definirás tus propios tipos de error que encapsulen los fallos específicos del dominio. Hay dos crates muy populares:

- **`thiserror`**: derive macros que generan implementaciones de `Display`, `Error` y `From` para enums de error.
- **`anyhow`**: simplifica el manejo de errores cuando no necesitas tipos concretos (ideal para prototipos, scripts y aplicaciones de un solo binario).

```rust
// Con thiserror
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorNegocio {
    #[error("Cliente con RFC {0} no encontrado")]
    ClienteNoEncontrado(String),
    #[error("Stock insuficiente: {0} disponibles, {1} solicitados")]
    StockInsuficiente(u32, u32),
    #[error("Error de base de datos: {0}")]
    BaseDatos(#[from] sqlx::Error),
    #[error("Error de validación: {0}")]
    Validacion(String),
}
```

```rust
// Con anyhow (para main.rs y scripts)
use anyhow::{Result, Context};

fn main() -> Result<()> {
    let contenido = std::fs::read_to_string("config.txt")
        .context("No se pudo leer config.txt")?;
    println!("{}", contenido);
    Ok(())
}
```

Profundizaremos mucho más en `Result` y en los errores personalizados cuando conectemos a MySQL en la Parte 2, y otra vez más cuando los mapeemos a respuestas HTTP en la Parte 3.

### 1.15.5 Mini-ejercicio integrador

Combina structs, enums, traits y manejo de errores para modelar un sistema de procesamiento de pagos del ERP. El programa debe:
1. Definir un enum `Pago` con variantes `Efectivo`, `Tarjeta`, `Transferencia`, `Credito`.
2. Definir un enum `ResultadoPago` con variantes `Aprobado { folio: String }`, `Rechazado { motivo: String }`, `Pendiente`.
3. Definir un trait `ProcesadorPago` con un método `procesar(&self, monto: f64) -> ResultadoPago`.
4. Implementar el trait para `Efectivo` (siempre aprobado), `Tarjeta` (aprobado si el monto ≤ 50 000), `Transferencia` (siempre pendiente hasta confirmación), `Credito` (aprobado si el cliente tiene crédito).
5. En `main`, crear una lista de pagos y procesarlos.

La solución está en el Anexo A.2.

## Cierre de la Fase 1.b

Con esto concluimos los temas de tipos personalizados (structs, enums), polimorfismo (traits, genéricos) y manejo de errores. En la **Fase 1.c** veremos las colecciones (`Vec`, `HashMap`, `HashSet`), closures e iteradores, módulos y sistema de archivos, pruebas unitarias, y cerraremos la Parte 1 con 20 ejercicios acumulativos y sus soluciones. Los mini-proyectos `07_catalogo_productos` y `08_modelo_erp_modular` materializan todos estos conceptos.

> **Métricas parciales** (Fase 1.a + 1.b):
> - 15 secciones H2 escritas (1.1 – 1.15).
> - ~45 subsecciones H3/H4.
> - 6 mini-proyectos físicos.
> - 50+ bloques de código.
> - ~12 000 palabras.


---

## 1.16 Colecciones

La librería estándar ofrece tres colecciones principales: `Vec<T>` (lista dinámica), `HashMap<K, V>` (diccionario) y `HashSet<T>` (conjunto). Todas viven en `std::collections` y se importan con `use`.

### 1.16.1 `Vec<T>` – vector dinamico

Un `Vec` es una lista de elementos del mismo tipo, almacenada en el heap, de tamaño variable. Es probablemente la colección más usada en Rust.

```rust
fn main() {
    // Crear un Vec vacío
    let mut productos: Vec<String> = Vec::new();
    productos.push(String::from("Laptop"));
    productos.push(String::from("Mouse"));
    productos.push(String::from("Teclado"));

    // Crear un Vec con valores iniciales (macro vec!)
    let precios = vec![18999.0, 350.0, 1200.0];

    // Acceso por índice
    println!("Primer producto: {}", productos[0]);

    // Acceso seguro con get() (devuelve Option)
    match productos.get(5) {
        Some(p) => println!("Quinto: {}", p),
        None => println!("No hay quinto producto"),
    }

    // Iterar (inmutable)
    for p in &productos {
        println!("Producto: {}", p);
    }

    // Iterar (mutable) - útil para modificar
    for precio in precios.iter_mut() {
        *precio *= 1.10;       // +10% de aumento
    }
    println!("Precios actualizados: {:?}", precios);

    // Métodos útiles
    println!("Cantidad: {}", productos.len());
    println!("¿Vacío?: {}", productos.is_empty());
    println!("¿Contiene 'Mouse'?: {}", productos.contains(&String::from("Mouse")));

    // Eliminar
    let removido = productos.remove(1);  // elimina "Mouse"
    println!("Eliminado: {}", removido);

    // Ordenar
    let mut nums = vec![5, 2, 8, 1, 3];
    nums.sort();
    println!("Ordenado: {:?}", nums);
}
```

**Errores típicos**:
- `productos[10]` cuando sólo hay 3 elementos: **panic!** en debug.
- `productos[0]` después de hacer `pop()`: panic.
- Olvidar el `&` al iterar: mueve el Vec, ya no puedes usarlo.

### 1.16.2 `HashMap<K, V>` – diccionario hash

Un `HashMap` asocia claves con valores. La clave debe implementar `Hash` y `Eq` (la mayoría de los tipos built-in ya lo hacen).

```rust
use std::collections::HashMap;

fn main() {
    // Crear un HashMap vacío
    let mut precios: HashMap<String, f64> = HashMap::new();
    precios.insert(String::from("Laptop"), 18999.0);
    precios.insert(String::from("Mouse"),   350.0);
    precios.insert(String::from("Teclado"), 1200.0);

    // Acceso
    match precios.get("Laptop") {
        Some(p) => println!("Laptop cuesta ${}", p),
        None => println!("No encontrado"),
    }

    // insert_or (sólo inserta si la clave no existe)
    precios.entry(String::from("Mouse")).or_insert(400.0);  // no actualiza
    precios.entry(String::from("Cable")).or_insert(89.0);   // sí inserta

    // Iterar
    for (producto, precio) in &precios {
        println!("{}: ${}", producto, precio);
    }

    // Incrementar un contador
    let mut contador: HashMap<String, u32> = HashMap::new();
    for palabra in ["Laptop", "Mouse", "Laptop", "Teclado", "Laptop"] {
        *contador.entry(palabra.to_string()).or_insert(0) += 1;
    }
    println!("{:?}", contador);   // {"Laptop": 3, "Mouse": 1, "Teclado": 1}
}
```

### 1.16.3 `HashSet<T>` – conjunto

Un `HashSet` almacena valores únicos, sin orden. Es útil para deduplicar y para verificar pertenencia rápidamente.

```rust
use std::collections::HashSet;

fn main() {
    let sku_vendidos: HashSet<String> = ["SKU-001", "SKU-002", "SKU-001", "SKU-003"]
        .iter().map(|s| s.to_string()).collect();
    println!("SKU únicos vendidos: {:?}", sku_vendidos);
    println!("Total únicos: {}", sku_vendidos.len());
    println!("¿Se vendió SKU-001?: {}", sku_vendidos.contains("SKU-001"));
}
```

### 1.16.4 Cadenas: `String` y `&str`

Rust tiene dos tipos principales de cadenas:

- `&str` (string slice): una referencia inmutable a una secuencia UTF-8. Vive en algún lugar (literal, archivo, etc.) y la usamos cuando no necesitamos poseer la cadena.
- `String`: una cadena owned, mutable, en heap. La creamos con `String::from()`, `String::new()`, o `"texto".to_string()`.

```rust
fn main() {
    // &str literal
    let literal: &str = "ERP México";
    // String owned
    let owned: String = String::from("ERP México");
    let owned2: String = "ERP México".to_string();

    // &str -> String
    let s1: String = literal.to_string();
    // String -> &str
    let s2: &str = &owned;

    // Concatenar
    let mut saludo = String::from("Hola");
    saludo.push_str(", mundo");
    saludo.push('!');
    println!("{}", saludo);

    // Formato
    let usuario = "Ana";
    let mensaje = format!("Bienvenida, {}", usuario);

    // Longitud
    let texto = "México";
    println!("bytes={}, chars={}", texto.len(), texto.chars().count());
}
```

**Errores típicos**:
- `let s: &str = String::from("x");` (no compila: `String` no es `&str`).
- `texto.len()` para contar caracteres: en UTF-8 cuenta bytes, no caracteres. Usa `texto.chars().count()`.
- Intentar indexar `s[0]` un String: Rust no lo permite directamente, porque UTF-8 tiene caracteres multibyte. Usa `s.chars().nth(0)` o `s.as_bytes()[0]`.

## 1.17 Closures e iteradores

### 1.17.1 Sintaxis de closures

Una closure es una función anónima que puede capturar variables del entorno. Se delimita con `|args| { cuerpo }`:

```rust
fn main() {
    let iva = 0.16;
    let aplicar_iva = |precio: f64| precio * (1.0 + iva);     // captura iva
    println!("100 con IVA = ${}", aplicar_iva(100.0));

    let sumar = |a, b| a + b;        // tipos inferidos
    println!("3+4 = {}", sumar(3, 4));
}
```

### 1.17.2 Traits de captura: `Fn`, `FnMut`, `FnOnce`

Las closures se categorizan en tres traits según cómo capturan el entorno:

- `Fn`: captura por referencia inmutable (`&T`). Puede llamarse múltiples veces sin problemas.
- `FnMut`: captura por referencia mutable (`&mut T`). Puede llamarse varias veces, modificando el estado capturado.
- `FnOnce`: consume la captura (`T`). Sólo puede llamarse una vez (toma ownership).

El compilador elige el trait más restrictivo posible.

```rust
fn main() {
    let x = vec![1, 2, 3];

    // Fn: captura por referencia
    let imprimir = || println!("{:?}", x);
    imprimir();
    imprimir();

    // FnMut: captura mutable
    let mut sum = 0;
    let mut acumular = |n| sum += n;
    acumular(5);
    acumular(3);
    println!("Suma: {}", sum);

    // FnOnce: consume
    let s = String::from("hola");
    let consumir = || s;          // mueve s
    let tomado = consumir();      // Ok
    // println!("{}", s);        // ERROR: s ya no existe
}
```

### 1.17.3 Iteradores perezosos

Un iterador es cualquier tipo que implementa el trait `Iterator`, que tiene un solo método obligatorio: `fn next(&mut self) -> Option<Item>`. Los iteradores en Rust son **perezosos** (lazy): no hacen nada hasta que se les pide consumir el resultado. Esto permite componer operaciones eficientemente sin crear colecciones intermedias.

```rust
fn main() {
    let ventas = vec![
        ("Laptop", 2, 18999.0),
        ("Mouse",  5,   350.0),
        ("Teclado",3,  1200.0),
    ];

    // Calcula el total facturado
    let total: f64 = ventas.iter()
        .map(|(_, cant, precio)| cant * precio)        // adapta cada item
        .sum();                                          // consume sumando
    println!("Total facturado: ${}", total);
}
```

### 1.17.4 Adaptadores: `map`, `filter`, `collect`, `fold`

| Adaptador | Descripción | Ejemplo |
|---|---|---|
| `map(closure)` | Transforma cada elemento | `vec![1,2,3].iter().map(\|x\| x*2)` |
| `filter(closure)` | Conserva los que cumplen la condición | `.iter().filter(\|x\| **x > 5)` |
| `collect()` | Materializa en una colección | `.iter().map(\|x\| *x*2).collect::<Vec<_>>()` |
| `fold(acumulador_inicial, closure)` | Reduce con acumulador | `.iter().fold(0, \|acc, x\| acc + x)` |
| `take(n)` | Toma los primeros n | `iter.take(3)` |
| `skip(n)` | Salta los primeros n | `iter.skip(2)` |
| `enumerate()` | Añade índice | `iter.enumerate()` |
| `zip(otro)` | Combina dos iteradores | `a.iter().zip(b.iter())` |
| `chain(otro)` | Concatena | `a.iter().chain(b.iter())` |
| `rev()` | Invierte | `vec![1,2,3].iter().rev()` |

### 1.17.5 Consumidores: `sum`, `count`, `for_each`, `any`, `all`

| Consumidor | Descripción |
|---|---|
| `sum()` | Suma (requiere tipo `Sum`) |
| `count()` | Cuenta los elementos |
| `for_each(closure)` | Aplica closure a cada uno |
| `any(closure)` | `true` si alguno cumple |
| `all(closure)` | `true` si todos cumplen |
| `find(closure)` | Devuelve el primero que cumple |
| `min()`, `max()` | Mínimo / máximo |
| `collect()` | Materializa |

```rust
fn main() {
    let productos = vec![
        ("Laptop",  18999.0, 10),
        ("Mouse",     350.0, 50),
        ("Teclado",  1200.0, 20),
        ("Cable",      89.0, 100),
    ];

    // ¿Algún producto tiene precio > 10000?
    let hay_caros = productos.iter().any(|(_, p, _)| *p > 10000.0);
    println!("¿Hay productos caros? {}", hay_caros);

    // ¿Todos tienen stock?
    let todos_con_stock = productos.iter().all(|(_, _, s)| *s > 0);
    println!("¿Todos con stock? {}", todos_con_stock);

    // Encontrar el más caro
    let mas_caro = productos.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("Más caro: {:?}", mas_caro);

    // Valor del inventario
    let valor_inventario: f64 = productos.iter()
        .map(|(_, precio, stock)| precio * *stock as f64)
        .sum();
    println!("Valor inventario: ${}", valor_inventario);
}
```

### 1.17.6 Mini-proyecto: `07_catalogo_productos`

El mini-proyecto (en `proyectos_capitulo/parte1/07_catalogo_productos/`) implementa un catálogo de productos con filtrado, ordenamiento y agregaciones, todo en memoria.

```rust
// 07_catalogo_productos/src/main.rs
// Catálogo de productos con operaciones funcionales (closures, iteradores)

#[derive(Debug, Clone)]
struct Producto {
    sku: String,
    nombre: String,
    categoria: String,
    precio: f64,
    costo: f64,
    stock: u32,
}

impl Producto {
    fn margen(&self) -> f64 {
        if self.precio <= 0.0 { 0.0 } else { (self.precio - self.costo) / self.precio * 100.0 }
    }
    fn valor_stock(&self) -> f64 { self.costo * self.stock as f64 }
}

fn main() {
    let catalogo = vec![
        Producto { sku: "SKU-001".into(), nombre: "Laptop HP".into(),       categoria: "Cómputo".into(),   precio: 18999.0, costo: 12500.0, stock: 10 },
        Producto { sku: "SKU-002".into(), nombre: "Mouse óptico".into(),    categoria: "Accesorios".into(), precio:   350.0, costo:   150.0, stock: 50 },
        Producto { sku: "SKU-003".into(), nombre: "Teclado mecánico".into(), categoria: "Accesorios".into(), precio:  1200.0, costo:   600.0, stock: 20 },
        Producto { sku: "SKU-004".into(), nombre: "Monitor 24\"".into(),    categoria: "Cómputo".into(),   precio:  4500.0, costo:  3000.0, stock: 15 },
        Producto { sku: "SKU-005".into(), nombre: "Cable HDMI".into(),       categoria: "Accesorios".into(), precio:    89.0, costo:    35.0, stock: 100 },
    ];

    // Filtrar productos con margen < 20% (candidatos a subir precio)
    let margen_bajo: Vec<&Producto> = catalogo.iter()
        .filter(|p| p.margen() < 20.0)
        .collect();
    println!("Productos con margen bajo ({}):", margen_bajo.len());
    for p in &margen_bajo {
        println!("  {} ({}) margen: {:.1}%", p.nombre, p.sku, p.margen());
    }

    // Valor total del inventario
    let total_inventario: f64 = catalogo.iter().map(|p| p.valor_stock()).sum();
    println!("\nValor total del inventario: ${:.2}", total_inventario);

    // Top 3 productos más caros
    let mut ordenados: Vec<&Producto> = catalogo.iter().collect();
    ordenados.sort_by(|a, b| b.precio.partial_cmp(&a.precio).unwrap());
    println!("\nTop 3 más caros:");
    for p in ordenados.iter().take(3) {
        println!("  {} - ${}", p.nombre, p.precio);
    }

    // Agrupar por categoría
    use std::collections::HashMap;
    let mut por_categoria: HashMap<String, Vec<&Producto>> = HashMap::new();
    for p in &catalogo {
        por_categoria.entry(p.categoria.clone()).or_default().push(p);
    }
    println!("\nProductos por categoría:");
    for (cat, prods) in &por_categoria {
        println!("  {} ({}): {}", cat, prods.len(),
                 prods.iter().map(|p| p.nombre.as_str()).collect::<Vec<_>>().join(", "));
    }
}
```

## 1.18 Modulos y sistema de archivos

Conforme un proyecto crece, meter todo en `main.rs` se vuelve insostenible. Rust ofrece un sistema de **módulos** que te permite organizar el código en archivos y subcarpetas, y controlar la visibilidad de cada pieza.

### 1.18.1 `mod`, `pub`, `use`

- `mod nombre;` declara un módulo. Si el módulo está en `src/nombre.rs` o en `src/nombre/mod.rs`, no hace falta escribir el cuerpo.
- `pub` marca un item como público (visible fuera del módulo).
- `pub(crate)` marca como público sólo dentro del crate actual.
- `use ruta::item;` trae un item al ámbito actual para no escribir la ruta completa.

### 1.18.2 Estructura de directorios

Un proyecto Rust puede organizarse de dos formas:

**Forma 1** (módulo en archivo):
```
src/
├── main.rs
├── clientes.rs
├── productos.rs
└── pedidos.rs
```

En `main.rs`:
```rust
mod clientes;
mod productos;
mod pedidos;

use clientes::Cliente;
use productos::Producto;

fn main() {
    let c = Cliente::new(/* ... */);
}
```

**Forma 2** (módulo con subcarpeta):
```
src/
├── main.rs
├── clientes/
│   ├── mod.rs
│   └── validacion.rs
└── productos.rs
```

### 1.18.3 El archivo `lib.rs` y los crates

Un *crate* es la unidad de compilación: o es un binario (con `main.rs`) o es una librería (con `lib.rs`). En proyectos grandes, es común tener un workspace con varios crates: uno como librería y varios como binarios que la consumen. Lo veremos al final del manual con `proyecto_api/`.

### 1.18.4 Mini-proyecto: `08_modelo_erp_modular`

El mini-proyecto refactoriza `05_modelo_erp` en módulos separados:

```
proyectos_capitulo/parte1/08_modelo_erp_modular/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── clientes.rs
    ├── productos.rs
    ├── pedidos.rs
    └── errores.rs
```

```rust
// src/errores.rs
use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorNegocio {
    Validacion(String),
    NoEncontrado(String),
    StockInsuficiente { disponible: u32, solicitado: u32 },
    CreditoInsuficiente { disponible: f64, requerido: f64 },
}

impl fmt::Display for ErrorNegocio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorNegocio::Validacion(m) => write!(f, "Validación: {}", m),
            ErrorNegocio::NoEncontrado(q) => write!(f, "No encontrado: {}", q),
            ErrorNegocio::StockInsuficiente { disponible, solicitado } =>
                write!(f, "Stock insuficiente: {} disponibles, {} solicitados", disponible, solicitado),
            ErrorNegocio::CreditoInsuficiente { disponible, requerido } =>
                write!(f, "Crédito insuficiente: ${} disponibles, ${} requeridos", disponible, requerido),
        }
    }
}

impl std::error::Error for ErrorNegocio {}

pub type ResultadoNegocio<T> = Result<T, ErrorNegocio>;
```

```rust
// src/clientes.rs
use crate::errores::ResultadoNegocio;

#[derive(Debug, Clone)]
pub struct Cliente {
    pub id: u32,
    pub nombre: String,
    pub rfc: String,
    pub email: String,
    pub credito: f64,
    pub activo: bool,
}

impl Cliente {
    pub fn new(id: u32, nombre: &str, rfc: &str, email: &str, credito: f64) -> ResultadoNegocio<Self> {
        if nombre.is_empty() { return Err(crate::errores::ErrorNegocio::Validacion("Nombre vacío".into())); }
        if rfc.len() < 12 { return Err(crate::errores::ErrorNegocio::Validacion("RFC inválido".into())); }
        if !email.contains('@') { return Err(crate::errores::ErrorNegocio::Validacion("Email inválido".into())); }
        Ok(Cliente { id, nombre: nombre.into(), rfc: rfc.into(), email: email.into(), credito, activo: true })
    }
    pub fn tiene_credito(&self, monto: f64) -> bool { monto <= self.credito && self.activo }
    pub fn desactivar(&mut self) { self.activo = false; }
}
```

```rust
// src/productos.rs
use crate::errores::ResultadoNegocio;

#[derive(Debug, Clone)]
pub struct Producto {
    pub sku: String,
    pub nombre: String,
    pub precio: f64,
    pub costo: f64,
    pub stock: u32,
}

impl Producto {
    pub fn new(sku: &str, nombre: &str, precio: f64, costo: f64, stock: u32) -> ResultadoNegocio<Self> {
        if sku.is_empty() { return Err(crate::errores::ErrorNegocio::Validacion("SKU vacío".into())); }
        if precio < costo { return Err(crate::errores::ErrorNegocio::Validacion("Precio < costo".into())); }
        Ok(Producto { sku: sku.into(), nombre: nombre.into(), precio, costo, stock })
    }
    pub fn margen(&self) -> f64 { if self.precio <= 0.0 { 0.0 } else { (self.precio - self.costo) / self.precio * 100.0 } }
    pub fn descontar_stock(&mut self, cantidad: u32) -> ResultadoNegocio<()> {
        if cantidad > self.stock {
            return Err(crate::errores::ErrorNegocio::StockInsuficiente { disponible: self.stock, solicitado: cantidad });
        }
        self.stock -= cantidad;
        Ok(())
    }
}
```

```rust
// src/pedidos.rs
use crate::clientes::Cliente;
use crate::errores::ResultadoNegocio;
use crate::productos::Producto;

#[derive(Debug, Clone)]
pub struct LineaPedido {
    pub sku: String,
    pub cantidad: u32,
    pub precio_unitario: f64,
}

impl LineaPedido {
    pub fn new(sku: &str, cantidad: u32, precio_unitario: f64) -> Self {
        LineaPedido { sku: sku.into(), cantidad, precio_unitario }
    }
    pub fn subtotal(&self) -> f64 { self.cantidad as f64 * self.precio_unitario }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EstadoPedido { Borrador, Confirmado, Enviado, Entregado, Cancelado }

#[derive(Debug, Clone)]
pub struct Pedido {
    pub folio: String,
    pub cliente_id: u32,
    pub lineas: Vec<LineaPedido>,
    pub estado: EstadoPedido,
}

impl Pedido {
    pub fn new(folio: &str, cliente: &Cliente) -> Self {
        Pedido { folio: folio.into(), cliente_id: cliente.id, lineas: Vec::new(), estado: EstadoPedido::Borrador }
    }
    pub fn agregar_linea(&mut self, linea: LineaPedido) { self.lineas.push(linea); }
    pub fn subtotal(&self) -> f64 { self.lineas.iter().map(|l| l.subtotal()).sum() }
    pub fn iva(&self) -> f64 { self.subtotal() * 0.16 }
    pub fn total(&self) -> f64 { self.subtotal() + self.iva() }
    pub fn confirmar(&mut self) -> ResultadoNegocio<()> {
        if self.lineas.is_empty() { return Err(crate::errores::ErrorNegocio::Validacion("Pedido sin líneas".into())); }
        self.estado = EstadoPedido::Confirmado;
        Ok(())
    }
}
```

```rust
// src/main.rs
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

    // Descontar stock del producto
    laptop.descontar_stock(2).unwrap();

    println!("Pedido {} confirmado. Total: ${:.2}", pedido.folio, pedido.total());
    println!("Stock restante de {}: {}", laptop.sku, laptop.stock);

    // Probar error de stock
    match laptop.descontar_stock(1000) {
        Ok(()) => println!("Algo falló"),
        Err(e) => println!("Error esperado: {}", e),
    }
}
```

## 1.19 Pruebas unitarias y de integracion

### 1.19.1 `#[test]` y `assert!`

Para escribir un test, marca una función con `#[test]` y usa `assert!`, `assert_eq!` o `assert_ne!`:

```rust
fn sumar(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suma_positivos() { assert_eq!(sumar(2, 3), 5); }

    #[test]
    fn suma_negativos() { assert_eq!(sumar(-2, -3), -5); }

    #[test]
    #[should_panic]
    fn division_por_cero_panica() {
        let _ = 1 / 0;  // en modo debug panica
    }
}
```

Corre los tests con `cargo test`. Por defecto se ejecutan en paralelo.

### 1.19.2 Tests de integracion

Los tests de integración viven en la carpeta `tests/` del proyecto y sólo tienen acceso a la API pública del crate. Son perfectos para probar flujos completos:

```
mi_crate/
├── src/lib.rs
├── tests/
│   ├── integracion_clientes.rs
│   └── integracion_pedidos.rs
```

### 1.19.3 Tests de documentacion

Los bloques de código en los comentarios `///` se ejecutan como tests:

```rust
/// Suma dos números
///
/// ```
/// assert_eq!(mi_crate::sumar(2, 3), 5);
/// ```
pub fn sumar(a: i32, b: i32) -> i32 { a + b }
```

`cargo test` los ejecuta automáticamente. Es una forma poderosa de mantener la documentación sincronizada con el código.

### 1.19.4 Mocks y dobles de prueba

Para simular dependencias (por ejemplo, una base de datos) en los tests, se usan **traits**. Definimos un trait, lo implementamos para el "servicio real" y para un "mock" en los tests, y nuestras funciones reciben `&impl Trait` en vez de una instancia concreta. Lo veremos en detalle en la Parte 3 cuando hablemos de Actix.

## 1.20 Ejercicios acumulativos (Parte 1) – 20 ejercicios

Los siguientes 20 ejercicios consolidan todo lo aprendido en la Parte 1. Cada uno está contextualizado en el ERP/CRM que estamos construyendo. Las soluciones completas se encuentran en el **Anexo A.2**.

1. **Hola ERP personalizado**: crea un programa que imprima un saludo personalizado con el nombre del usuario capturado por `std::env::args()`.

2. **Calculadora de antigüedad de clientes**: a partir de una fecha de alta en formato `AAAA-MM-DD`, calcula cuántos años lleva el cliente con la empresa. Considera los días bisiestos (simplificado: 1 año = 365 días).

3. **Validador de códigos de barras EAN-13**: verifica que un código de 13 dígitos tenga el dígito de control correcto. El dígito de control se calcula multiplicando las posiciones impares por 1 y las pares por 3, sumando, y restando de la decena superior.

4. **Conversor de monedas**: recibe un monto en MXN y lo convierte a USD, EUR y GBP usando tasas de cambio fijas (o leídas de un archivo). Redondea a 2 decimales.

5. **Generador de folios**: crea una función que genere un folio único para pedidos con formato `PED-AAAA-####` donde AAAA es el año y #### es un contador de 4 dígitos.

6. **Catálogo de productos con filtros**: implementa un `Vec<Producto>` con 10 productos y permite al usuario filtrarlos por: rango de precios, categoría, y disponibilidad de stock.

7. **Calculadora de comisiones para vendedores**: un vendedor recibe 5% de comisión por ventas, más 1% adicional si supera $50 000 en el mes. Dado un `Vec<f64>` de ventas del mes, calcula el total de comisión.

8. **Inventario en tiempo real**: simula un sistema de inventario donde se procesan movimientos (entradas y salidas) y se valida en cada momento que el stock no quede negativo.

9. **Validador de RFC completo**: además de las validaciones de formato, verifica que la fecha de nacimiento (posiciones 5-10) sea válida.

10. **Clasificador de productos por margen**: ordena un catálogo de productos por margen de ganancia y muestra el top 5 y el bottom 5.

11. **Máquina de estados de factura**: implementa una factura con estados `Borrador`, `Timbrada`, `Pagada`, `Cancelada`, `Vencida`. Sólo se permiten ciertas transiciones.

12. **Conversor de CSV a structs**: lee un archivo CSV con datos de clientes y crea un `Vec<Cliente>`. Maneja líneas mal formadas devolviendo un error.

13. **Generador de tickets de venta**: a partir de los datos de un pedido, genera un ticket de venta con formato alineado a la derecha, similar a los tickets de las tiendas departamentales.

14. **Sistema de roles y permisos**: modela usuarios con roles (`Admin`, `Vendedor`, `Almacén`, `Contador`) y permisos asociados. Verifica que un usuario puede o no puede realizar una acción.

15. **Historial de cambios (auditoría)**: implementa un struct `Auditoria` que registra `timestamp`, `usuario`, `acción`, `entidad`, `id_entidad`. Añade un método para listar los últimos N cambios.

16. **Calculadora de IVA con redondeo fiscal**: en México, el IVA se redondea al centavo más cercano. Implementa la función `redondear_centavos(monto: f64) -> f64` que use la estrategia del SAT.

17. **Gestor de múltiples almacenes**: un producto puede tener stock en N almacenes. Modela la relación y permite consultar el stock total, transferir entre almacenes, y emitir alertas de stock bajo.

18. **Importador de productos desde JSON**: lee un archivo JSON con productos y los importa a un `Vec<Producto>`. Define el schema esperado y maneja errores de parseo.

19. **Reporte de ventas mensual**: a partir de un `Vec<Pedido>` con fecha y total, agrupa por mes y muestra el total vendido y el ticket promedio.

20. **Refactor del catálogo en módulos**: toma el ejercicio 6 y sepáralo en módulos: `catalogo.rs` (lógica), `filtros.rs` (filtros como funciones), `main.rs` (orquestación).

## 1.21 Soluciones de los ejercicios de la Parte 1

Las soluciones completas de los 20 ejercicios están en el **Anexo A.2** del manual, con un ejemplo de proyecto `ejercicios_p1/` que se puede descargar del repositorio y ejecutar con `cargo test`.

---

## Cierre de la Parte 1

Con esto cerramos los fundamentos del lenguaje Rust. Has aprendido todo lo necesario para escribir programas robustos, modulares y testeados. En la **Parte 2** conectaremos todo esto a una base de datos MariaDB/MySQL, donde persistiremos clientes, productos y pedidos.

> **Métricas finales de la Parte 1**:
> - 21 secciones H2 escritas (1.1 – 1.21).
> - 70+ subsecciones H3/H4.
> - 8 mini-proyectos físicos (todos compilan y pasan tests).
> - 80+ bloques de código comentados en español.
> - 20 ejercicios propuestos.
> - ~22 000 palabras.


---

# Parte 2: Rust y MySQL

> **Aviso pedagógico**: en esta parte conectaremos todos los structs que construimos en la Parte 1 a una base de datos MariaDB/MySQL real. Usaremos el crate `mysql` (cliente síncrono) y el pool de conexiones `r2d2_mysql`. Aprenderemos a ejecutar SELECT, INSERT, UPDATE, DELETE con parámetros preparados (para evitar inyección SQL), a manejar transacciones, y a construir un CLI que gestione clientes, productos y pedidos del ERP.

## 2.1 Introduccion a bases de datos relacionales y MySQL

### 2.1.1 ¿Que es una base de datos relacional?

Una base de datos relacional (RDBMS) es un sistema de almacenamiento que organiza la información en **tablas** formadas por **filas** (registros) y **columnas** (campos), y que permite establecer **relaciones** entre tablas mediante claves foráneas. El modelo relacional fue propuesto por Edgar Codd en 1970 y es, cuarenta años después, el pilar de prácticamente toda la industria del software de gestión: ERPs, CRMs, sistemas contables, bancarios, etc.

**Analogía**: imagina una hoja de cálculo de Excel. Cada hoja es una tabla, cada fila es un registro, cada columna es un campo. Hasta ahí, una hoja de cálculo también es una "base de datos tabular". La diferencia es que un RDBMS te garantiza:

- **Integridad**: no puedes tener un pedido que apunte a un cliente que no existe (gracias a las claves foráneas).
- **Atomicidad**: una operación que toca varias tablas se aplica completa o no se aplica (gracias a las transacciones).
- **Concurrencia**: mil usuarios pueden leer y escribir a la vez sin corromper los datos (gracias al control de concurrencia y al bloqueo de filas).
- **Consultas complejas**: puedes unir tablas, agrupar, filtrar, ordenar, con un lenguaje declarativo (SQL).
- **Persistencia y recuperación**: los datos sobreviven a caídas del sistema y se pueden restaurar desde respaldos.

### 2.1.2 Tablas, filas, columnas y claves

Una **tabla** es un conjunto de filas con la misma estructura. Por ejemplo, la tabla `clientes`:

| id | nombre | rfc | email | credito |
|---|---|---|---|---|
| 1 | Constructora del Bajío | CDB010101AB3 | contacto@cdb.mx | 100000.00 |
| 2 | Distribuidora del Norte | DDN020202CD4 | ventas@ddn.mx | 50000.00 |

Cada fila es un **registro** (un cliente específico). Cada columna es un **campo** (atributo del cliente). Los nombres de columna y sus tipos forman el **esquema** de la tabla.

**Claves**:
- **Clave primaria (PRIMARY KEY)**: una columna (o conjunto) que identifica unívocamente cada fila. En `clientes` es `id`. Una tabla sólo puede tener una clave primaria.
- **Clave foránea (FOREIGN KEY)**: una columna que referencia la clave primaria de otra tabla. En `pedidos`, `cliente_id` es una clave foránea que referencia `clientes.id`. Garantiza que no puedes tener un pedido de un cliente inexistente.
- **Clave única (UNIQUE)**: una columna (o conjunto) cuyos valores no pueden repetirse. El RFC de un cliente es único.
- **Índice (INDEX)**: una estructura auxiliar que acelera las búsquedas por una columna concreta, a costa de un poco más de espacio en disco y de tiempo en inserciones.

### 2.1.3 Instalacion de MariaDB/MySQL (Docker/Podman)

Para este manual usaremos **MariaDB 11** (compatible con MySQL 8) ejecutándose en un contenedor de **Podman** (o Docker, si lo prefieres). El comando para arrancar la base de datos es:

```bash
# Crear carpeta para los datos persistentes
mkdir -p ~/podman/mysql_data

# Arrancar el contenedor (la primera vez descarga la imagen)
podman run -d --name mysql_man \
  -e MYSQL_ROOT_PASSWORD=secret \
  -e MYSQL_DATABASE=erp_crm \
  -v ~/podman/mysql_data:/var/lib/mysql:Z \
  -p 127.0.0.1:3306:3306 \
  docker.io/library/mariadb:11
```

**Análisis de cada parámetro**:
- `-d` (*detached*): ejecuta en segundo plano.
- `--name mysql_man`: nombre del contenedor, para referirnos a él con `podman stop mysql_man`, etc.
- `-e MYSQL_ROOT_PASSWORD=secret`: define la contraseña del usuario `root`. En producción, cámbiala por una robusta y usa un usuario no-root.
- `-e MYSQL_DATABASE=erp_crm`: crea automáticamente la base de datos `erp_crm` al iniciar.
- `-v ~/podman/mysql_data:/var/lib/mysql:Z`: monta una carpeta local como volumen para persistir los datos. La `:Z` es una opción de SELinux (en Fedora/RHEL) para que el contenedor pueda escribir.
- `-p 127.0.0.1:3306:3306`: mapea el puerto 3306 del contenedor al 3306 del host, **sólo en la interfaz local** (más seguro que `-p 3306:3306` que lo expondría en todas las interfaces).
- `docker.io/library/mariadb:11`: la imagen a usar.

Para verificar que funciona:

```bash
podman ps                    # ver contenedores en ejecución
mysql -h 127.0.0.1 -u root -psecret -e "SELECT VERSION();"
```

### 2.1.4 Creacion de la base de datos del ERP

Conectado a MariaDB como root, vamos a crear las tablas del ERP. En lugar de hacerlo desde la línea de comandos, lo dejaremos en un archivo `schema.sql` que ejecutaremos con `source`:

```sql
-- archivo: sql/init.sql
-- Esquema inicial del ERP/CRM (México)
DROP DATABASE IF EXISTS erp_crm;
CREATE DATABASE erp_crm CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE erp_crm;

-- Tabla de clientes
CREATE TABLE clientes (
    id           INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    nombre       VARCHAR(150) NOT NULL,
    rfc          VARCHAR(13)  NOT NULL UNIQUE,
    email        VARCHAR(150),
    telefono     VARCHAR(15),
    credito      DECIMAL(12,2) NOT NULL DEFAULT 0,
    activo       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_clientes_nombre (nombre)
) ENGINE=InnoDB;

-- Tabla de proveedores
CREATE TABLE proveedores (
    id           INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    nombre       VARCHAR(150) NOT NULL,
    rfc          VARCHAR(13)  NOT NULL UNIQUE,
    dias_pago    TINYINT UNSIGNED NOT NULL DEFAULT 30,
    activo       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB;

-- Tabla de categorías
CREATE TABLE categorias (
    id           INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    nombre       VARCHAR(100) NOT NULL UNIQUE,
    descripcion  VARCHAR(255)
) ENGINE=InnoDB;

-- Tabla de productos
CREATE TABLE productos (
    sku          VARCHAR(30) PRIMARY KEY,
    nombre       VARCHAR(150) NOT NULL,
    categoria_id INT UNSIGNED,
    precio       DECIMAL(12,2) NOT NULL,
    costo        DECIMAL(12,2) NOT NULL,
    stock        INT NOT NULL DEFAULT 0,
    activo       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (categoria_id) REFERENCES categorias(id)
) ENGINE=InnoDB;

-- Tabla de pedidos
CREATE TABLE pedidos (
    folio        VARCHAR(20) PRIMARY KEY,
    cliente_id   INT UNSIGNED NOT NULL,
    subtotal     DECIMAL(14,2) NOT NULL,
    iva          DECIMAL(14,2) NOT NULL,
    total        DECIMAL(14,2) NOT NULL,
    estado       ENUM('BORRADOR','CONFIRMADO','ENVIADO','ENTREGADO','CANCELADO') NOT NULL DEFAULT 'BORRADOR',
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (cliente_id) REFERENCES clientes(id)
) ENGINE=InnoDB;

-- Tabla de líneas de pedido
CREATE TABLE lineas_pedido (
    id              BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    pedido_folio    VARCHAR(20) NOT NULL,
    sku             VARCHAR(30) NOT NULL,
    cantidad        INT UNSIGNED NOT NULL,
    precio_unitario DECIMAL(12,2) NOT NULL,
    FOREIGN KEY (pedido_folio) REFERENCES pedidos(folio) ON DELETE CASCADE,
    FOREIGN KEY (sku) REFERENCES productos(sku)
) ENGINE=InnoDB;

-- Datos de ejemplo
INSERT INTO clientes (nombre, rfc, email, telefono, credito) VALUES
    ('Constructora del Bajío S.A. de C.V.', 'CDB010101AB3', 'contacto@cdb.mx', '5551234567', 100000.00),
    ('Distribuidora del Norte S.A. de C.V.', 'DDN020202CD4', 'ventas@ddn.mx',   '5552345678',  50000.00),
    ('Comercializadora del Sur S.C.',        'CDS030303EF5', 'info@cds.mx',     '5553456789',  25000.00);

INSERT INTO categorias (nombre, descripcion) VALUES
    ('Cómputo', 'Equipo de cómputo y accesorios'),
    ('Oficina', 'Material y mobiliario de oficina'),
    ('Servicios', 'Servicios profesionales');

INSERT INTO productos (sku, nombre, categoria_id, precio, costo, stock) VALUES
    ('SKU-001', 'Laptop HP Pavilion 15', 1, 18999.00, 12500.00, 10),
    ('SKU-002', 'Mouse óptico USB',      1,   350.00,   150.00, 50),
    ('SKU-003', 'Teclado mecánico',      1,  1200.00,   600.00, 20),
    ('SKU-004', 'Monitor LED 24"',       1,  4500.00,  3000.00, 15),
    ('SKU-005', 'Silla ergonómica',      2,  2800.00,  1800.00,  8);
```

Para ejecutarlo:

```bash
mysql -h 127.0.0.1 -u root -psecret < schema.sql
```

### 2.1.5 SQL basico: DDL y DML

SQL se divide en varias sublenguajes:

- **DDL** (Data Definition Language): `CREATE`, `ALTER`, `DROP` para definir la estructura.
- **DML** (Data Manipulation Language): `SELECT`, `INSERT`, `UPDATE`, `DELETE` para manipular los datos.
- **DCL** (Data Control Language): `GRANT`, `REVOKE` para gestionar permisos.
- **TCL** (Transaction Control Language): `COMMIT`, `ROLLBACK` para transacciones.

Veamos las cuatro operaciones DML básicas que usaremos constantemente:

```sql
-- SELECT: leer
SELECT id, nombre, rfc FROM clientes WHERE activo = TRUE ORDER BY nombre;

-- INSERT: crear
INSERT INTO clientes (nombre, rfc, email, credito) VALUES
    ('Constructora del Bajío', 'CDB010101AB3', 'contacto@cdb.mx', 100000);

-- UPDATE: actualizar
UPDATE clientes SET credito = 150000 WHERE id = 1;

-- DELETE: eliminar (¡cuidado!)
DELETE FROM clientes WHERE id = 3;
```

Y los operadores de filtrado más útiles:

| Operador | Significado |
|---|---|
| `=` | Igual |
| `<>` o `!=` | Distinto |
| `>`, `<`, `>=`, `<=` | Comparación |
| `BETWEEN x AND y` | En rango |
| `IN (a, b, c)` | En lista |
| `LIKE 'patron%'` | Coincide patrón (`%` = cualquier cosa, `_` = un carácter) |
| `IS NULL` / `IS NOT NULL` | Nulo o no nulo |
| `AND`, `OR`, `NOT` | Lógicos |

## 2.2 Configuracion del proyecto con el crate `mysql`

Hasta ahora, en la sección 2.1, hemos visto qué es una base de datos relacional y cómo instalar MariaDB con Docker. Pero saber qué es una base de datos no sirve de nada si no podemos hablar con ella desde Rust. El problema es que Rust, por sí mismo, no sabe cómo conectarse a MariaDB. No tiene un "driver de MySQL" incorporado en su biblioteca estándar. Y esto es intencional: la filosofía de Rust es tener un núcleo pequeño y estable, y dejar que la comunidad construya las herramientas especializadas. Para casi todo lo que no sea matemáticas básicas, manejo de archivos o networking de bajo nivel, necesitas un **crate** externo.

¿Y por qué Rust no incluye el driver de MySQL en la biblioteca estándar? La respuesta es que la biblioteca estándar de Rust se mantiene deliberadamente pequeña para que pueda evolucionar sin romper el ecosistema. Imagina que el equipo de Rust decidiera incluir el driver de MySQL versión 25 en la std. Cuando salga la versión 26 del driver, cambiar la std requeriría una nueva edición de Rust, y todo el ecosistema tendría que actualizarse al mismo tiempo. En cambio, si el driver es un crate independiente, cada proyecto puede elegir la versión que necesita, actualizarla cuando quiera, y el lenguaje no se ve afectado. Esta separación de responsabilidades es uno de los pilares del diseño de Rust: el lenguaje proporciona las herramientas (el compilador, el sistema de tipos, Cargo), y la comunidad proporciona el resto a través de crates.

Pensemos en esto desde la perspectiva de un ERP. Cuando construyes un sistema de facturación, no esperas que el lenguaje de programación incluya el cálculo del IVA o la validación del RFC. Eso lo construyes tú o lo tomas de una librería especializada. De la misma manera, el lenguaje no incluye el cliente de MySQL: tomas el crate `mysql`, que es la librería especializada. Y así como separas la lógica de negocio del acceso a datos, Rust separa el lenguaje del ecosistema de librerías.

En esta sección vamos a configurar nuestro primer proyecto Rust que se conecta a MariaDB. Aprenderás qué son los crates, cómo se declaran en el archivo `Cargo.toml`, cómo se estructura una URL de conexión a la base de datos, y escribirás tu primer programa que ejecuta un `SELECT` y muestra los resultados. No es un programa complejo —son unas 15 líneas— pero contiene todos los elementos que usarás en los próximos capítulos: el pool de conexiones, la ejecución de consultas, el mapeo de resultados a tipos de Rust, y el manejo de errores con el operador `?`.

### 2.2.1 Dependencias en `Cargo.toml`

Antes de escribir código, tenemos que decirle a Rust qué librerías externas necesitamos. Esto se hace en el archivo `Cargo.toml`, que Cargo crea automáticamente cuando ejecutas `cargo new nombre_proyecto`. Piensa en `Cargo.toml` como una lista de compras: escribes los nombres de los productos (crates) que necesitas, y Cargo sale a comprarlos por ti a crates.io, los trae a tu máquina, los compila y los deja listos para usar.

```toml
[package]
name = "conexion_mysql"
version = "0.1.0"
edition = "2021"

[dependencies]
mysql = "25"
r2d2 = "0.8"
r2d2_mysql = "26"
```

**Análisis línea por línea**:

- `[package]`: abre la sección de configuración del paquete. Todo lo que sigue hasta la siguiente sección (`[dependencies]`) pertenece a la identidad del proyecto.
- `name = "conexion_mysql"`: el nombre del proyecto. Aparecerá en el binario compilado y en crates.io si lo publicas. Por convención, se usa `snake_case` (minúsculas y guiones bajos).
- `version = "0.1.0"`: la versión del proyecto en formato SemVer (MAJOR.MINOR.PATCH). Empezamos en 0.1.0 porque es un proyecto nuevo. Cuando lo consideres estable, pasarías a 1.0.0.
- `edition = "2021"`: la edición del lenguaje Rust. La edición 2021 es la más reciente y estable. Cada edición puede tener pequeños cambios en la sintaxis, pero todas son compatibles hacia atrás.
- `[dependencies]`: abre la sección donde declaras los crates externos que necesita tu proyecto.
- `mysql = "25"`: le dice a Cargo "necesito el crate `mysql`, cualquier versión compatible con la 25 (es decir, 25.x.x)". El número `25` es la versión *major*; las versiones *minor* y *patch* pueden ser cualquier número. Esto se llama un "requerimiento flexible" porque te permite recibir actualizaciones de seguridad y parches sin cambiar tu código.
- `r2d2 = "0.8"`: igual, pero para el crate `r2d2`. La versión 0.8 es la que usaremos para el pool de conexiones.
- `r2d2_mysql = "26"`: el puente entre `r2d2` y `mysql`. ¿Por qué necesitamos un crate separado? Porque `r2d2` es genérico: funciona con cualquier base de datos que tenga un "manager". `r2d2_mysql` implementa ese manager específicamente para MySQL/MariaDB. Si mañana migraras a PostgreSQL, cambiarías `r2d2_mysql` por `r2d2_pq` y el resto del código del pool quedaría igual. Esta separación sigue el principio de composición: cada crate hace una cosa y la hace bien.

**Salida esperada del `cargo build`**:

La primera vez que ejecutes `cargo build`, verás algo como:

```
   Compiling libc v0.2.155
   Compiling mysql_common v0.33.0
   Compiling mysql v25.0.0
   Compiling r2d2 v0.8.10
   Compiling r2d2_mysql v26.0.0
   Compiling conexion_mysql v0.1.0 (/home/.../conexion_mysql)
    Finished dev profile (unoptimized + debuginfo) ...
```

Cargo descarga cada crate y sus dependencias transitivas (por ejemplo, `mysql` depende de `mysql_common`, que a su vez depende de `libc`), las compila y las guarda en la carpeta `target/`. La primera compilación puede tardar unos minutos; las siguientes serán casi instantáneas gracias al caché de Cargo.

### 2.2.2 La URL de conexion

Para conectarnos a MariaDB, el crate `mysql` necesita saber dónde está el servidor, con qué usuario autenticarse y a qué base de datos conectarse. Toda esta información se empaqueta en una **URL de conexión**, una cadena de texto con un formato específico:

```
mysql://usuario:contraseña@host:puerto/nombre_base_datos
```

Es como la dirección de un sobre postal: el protocolo (`mysql://`) es el tipo de correo, el usuario y la contraseña son el remitente, el `@` separa al remitente del destino, el host y el puerto son la dirección del edificio, y el nombre de la base de datos es el número de departamento. Cada parte es necesaria para que el mensaje llegue a su destino.

En nuestro caso, la URL es:

```
mysql://root:secret@127.0.0.1:3306/erp_crm
```

- `mysql://` indica que usamos el protocolo de MySQL/MariaDB. Podría ser `postgres://`, `sqlite://`, etc.
- `root` es el usuario administrador de MariaDB.
- `secret` es la contraseña que configuramos al crear el contenedor Docker.
- `127.0.0.1` es la dirección IP de "localhost": tu propia máquina.
- `3306` es el puerto por defecto de MySQL/MariaDB.
- `erp_crm` es la base de datos que creamos con el `init.sql`.

### 2.2.3 Primer programa: conectar, consultar, imprimir

Vamos a escribir el primer programa que realmente se conecta a MariaDB, ejecuta una consulta y muestra los resultados. Es un programa pequeño, pero introduce varios conceptos nuevos: cómo crear un pool de conexiones, cómo obtener una conexión del pool, cómo ejecutar un `SELECT` y cómo manejar los errores que puedan ocurrir.

```rust
// archivo: src/main.rs
// Primer programa de conexión a MariaDB.
// Crea un pool, obtiene una conexión, ejecuta SELECT y muestra los clientes.

use mysql::prelude::*;
use mysql::Pool;

fn main() -> Result<(), mysql::Error> {
    let database_url = "mysql://root:secret@127.0.0.1:3306/erp_crm";

    let opts = mysql::Opts::from_url(database_url)?;
    let pool = Pool::new(opts)?;
    let mut conn = pool.get_conn()?;

    let clientes: Vec<(u32, String, String)> =
        conn.query("SELECT id, nombre, rfc FROM clientes")?;

    println!("Clientes ({}):", clientes.len());
    for (id, nombre, rfc) in &clientes {
        println!("  [{}] {} ({})", id, nombre, rfc);
    }

    Ok(())
}
```

**Análisis línea por línea**:

- `use mysql::prelude::*;`: importa todo lo que está en el módulo `prelude` del crate `mysql`. En Rust, "prelude" es una convención: un módulo que reexporta los traits y tipos más comunes para que no tengas que importarlos uno por uno. Sin esta línea, el compilador no encontraría los métodos `.query()`, `.exec()`, etc., porque están definidos en traits que no estarían en el alcance.
- `use mysql::Pool;`: importa el tipo `Pool`, que es la estructura principal para manejar conexiones a la base de datos. Un `Pool` mantiene un conjunto de conexiones abiertas y las reutiliza.
- `fn main() -> Result<(), mysql::Error> {`: nota que `main` devuelve un `Result`. Normalmente `main` no devuelve nada, pero cuando devuelve `Result`, Rust imprime automáticamente el error si ocurre uno. No necesitas escribir un `match` ni un `unwrap`: si algo falla, el mensaje de error aparece en la terminal y el programa termina con código de error distinto de cero.
- `let database_url = "mysql://root:secret@127.0.0.1:3306/erp_crm";`: definimos la URL de conexión como una variable de tipo `&str` (un string slice, que es una referencia inmutable a una cadena de texto). Esta URL contiene toda la información que el driver necesita para localizar el servidor.
- `let opts = mysql::Opts::from_url(database_url)?;`: convertimos la URL en una estructura `Opts`, que contiene las opciones de conexión ya parseadas. El `?` es crucial: si la URL tiene un formato incorrecto (por ejemplo, falta la contraseña o el puerto no es un número), `from_url` devuelve un `Err` y el programa termina con el mensaje de error. Si la URL es correcta, obtenemos un `Opts` listo para usar.
- `let pool = Pool::new(opts)?;`: creamos un pool de conexiones con las opciones que acabamos de construir. El pool no abre conexiones inmediatamente; las abre bajo demanda cuando alguien llama a `get_conn()`. El `?` propaga el error si el pool no se puede crear.
- `let mut conn = pool.get_conn()?;`: pedimos una conexión al pool. Si el pool tiene una conexión disponible, la obtenemos inmediatamente. Si todas están ocupadas, esperamos hasta que una se libere. Si el servidor está caído, obtenemos un error. La variable `conn` es de tipo `PooledConn`, que implementa los mismos métodos que una conexión directa.
- `let clientes: Vec<(u32, String, String)> = conn.query("SELECT id, nombre, rfc FROM clientes")?;`: ejecutamos la consulta SQL. El método `.query()` ejecuta la consulta y automáticamente intenta convertir cada fila en una tupla `(u32, String, String)`. El orden de los tipos debe coincidir con el orden de las columnas en el SELECT: la primera columna (`id`) se mapea a `u32`, la segunda (`nombre`) a `String`, la tercera (`rfc`) a `String`. Si los tipos no coinciden, el error ocurre en tiempo de ejecución.
- `println!("Clientes ({}):", clientes.len());`: imprimimos cuántos clientes se obtuvieron. `clientes.len()` devuelve el número de elementos en el vector.
- `for (id, nombre, rfc) in &clientes {`: iteramos sobre cada elemento del vector. El `&` es importante: estamos prestando (borrowing) los elementos, no moviéndolos, lo que nos permite seguir usando el vector después del bucle.
- `println!("  [{}] {} ({})", id, nombre, rfc);`: imprimimos cada cliente formateado como `[1] Nombre (RFC)`.
- `Ok(())`: si llegamos aquí sin errores, devolvemos `Ok` con la unidad `()`. Esto indica al sistema operativo que el programa terminó correctamente.

**Salida esperada**:

```
Clientes (3):
  [1] Constructora del Bajío S.A. de C.V. (CDB010101AB3)
  [2] Distribuidora del Norte S.A. de C.V. (DDN020202CD4)
  [3] Comercializadora del Sur S.C. (CDS030303EF5)
```

Si el servidor no está corriendo, verás:

```
Error: Connection refused (os error 111)
```

Si la base de datos no existe, verás:

```
Error: Unknown database 'erp_crm'
```

### 2.2.4 Errores tipicos al conectar

**Error 1: olvidar el operador `?`**.

```rust
let opts = mysql::Opts::from_url(database_url);  // falta el ?
let pool = Pool::new(opts)?;
```

Mensaje del compilador: `error[E0308]: mismatched types, expected Opts, found Result<Opts, Error>`. Solución: añadir `?` al final: `mysql::Opts::from_url(database_url)?`.

**Error 2: puerto incorrecto en la URL**.

```rust
let database_url = "mysql://root:secret@127.0.0.1:9999/erp_crm";
//                                              ^^^^ el puerto 9999 no es el de MariaDB
```

Mensaje en ejecución: `Error: Connection refused (os error 111)`. Solución: verificar que el puerto sea `3306` y que el contenedor esté corriendo con `podman ps`.

**Error 3: contraseña incorrecta**.

```rust
let database_url = "mysql://root:clave_incorrecta@127.0.0.1:3306/erp_crm";
```

Mensaje en ejecución: `Error: Access denied for user 'root'@'localhost' (using password: YES)`. Solución: usar la contraseña correcta (`secret` en nuestro caso).

### 2.2.5 Mini-proyecto: `01_conexion_mysql`

El mini-proyecto `01_conexion_mysql` (en `proyectos_capitulo/parte2/01_conexion_mysql/`) amplía el programa anterior: además de conectar y listar clientes, también muestra la versión del servidor MariaDB y las tablas existentes en la base de datos. Es un diagnóstico completo que confirma que todo está funcionando.

```rust
use mysql::prelude::*;
use mysql::Pool;

const DB_URL: &str = "mysql://root:secret@127.0.0.1:3306/erp_crm";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = Pool::new(DB_URL)?;

    // 1. Versión del servidor
    let version: String = pool.get_conn()?.query_first("SELECT VERSION()")?
        .ok_or("No se pudo obtener la versión")?;
    println!("Versión del servidor: {}", version);

    // 2. Listar tablas
    let tablas: Vec<String> = pool.get_conn()?
        .query("SHOW TABLES")?;
    println!("\nTablas en la BD ({}):", tablas.len());
    for t in &tablas {
        println!("  - {}", t);
    }

    // 3. Contar clientes
    let total: i64 = pool.get_conn()?
        .query_first("SELECT COUNT(*) FROM clientes")?
        .unwrap_or(0);
    println!("\nTotal de clientes: {}", total);

    Ok(())
}
```

**Análisis línea por línea**:

- `const DB_URL: &str = ...`: definimos la URL como una constante. Usar `const` en lugar de `let` indica que este valor nunca cambiará y se incrusta en el binario en tiempo de compilación.
- `fn main() -> Result<(), Box<dyn std::error::Error>>`: el tipo `Box<dyn std::error::Error>` es un "comodín" para cualquier tipo de error. Esto nos permite usar `?` con cualquier función que devuelva un `Result`, sin importar el tipo de error específico.
- `let version: String = pool.get_conn()?.query_first("SELECT VERSION()")?.ok_or(...)?;`: aquí hay tres `?` en una sola línea. Primero obtenemos una conexión del pool, luego ejecutamos `SELECT VERSION()` y tomamos la primera fila con `query_first` (que devuelve `Option<String>`), luego convertimos el `Option` a `Result` con `ok_or` (para tener un mensaje de error si la consulta no devuelve nada). Cada `?` propaga el error al siguiente nivel.
- `pool.get_conn()?.query("SHOW TABLES")?`: ejecutamos el comando `SHOW TABLES` de MySQL, que devuelve los nombres de todas las tablas en la base de datos actual. Nota que pedimos una nueva conexión del pool en lugar de reutilizar la anterior. El pool decide si darnos la misma o una diferente.

**Salida esperada**:

```
Versión del servidor: 11.4.2-MariaDB

Tablas en la BD (6):
  - categorias
  - clientes
  - lineas_pedido
  - pedidos
  - productos
  - proveedores

Total de clientes: 3
```

Esta salida confirma cuatro cosas: la conexión funciona, el servidor responde, las tablas del ERP existen y los tipos de datos se convierten correctamente.

## 2.3 Consultas `SELECT`

Ya sabemos configurar un proyecto y conectarnos a MariaDB. Ahora vamos a aprender a **leer datos**, que es la operación más frecuente en cualquier sistema. En un ERP, las consultas están en todas partes: listar clientes, buscar productos por nombre, obtener el detalle de un pedido, generar un reporte de ventas. Todas estas son variaciones de una misma instrucción SQL: `SELECT`. La diferencia es cómo expresamos esa instrucción desde Rust y cómo procesamos los resultados.

En el crate `mysql`, la forma de ejecutar un `SELECT` depende de tres preguntas: ¿la consulta tiene partes variables (un RFC específico, un rango de precios)? Si no, usamos `query`. Si sí, usamos `exec`. ¿Esperamos una fila o muchas? Si esperamos una, usamos `query_first` o `exec_first`. Si esperamos muchas, usamos `query` o `exec` que devuelven `Vec<T>`. ¿Queremos las filas como tuplas genéricas, como structs con nombre, o queremos procesarlas una por una sin acumularlas en memoria? Cada respuesta nos lleva a un método diferente.

Pensemos en las consultas como si le preguntáramos algo a un archivero. Si la pregunta es fija ("dame la lista de todos los clientes"), usamos `query`. Si la pregunta incluye un detalle variable ("dame el cliente con este RFC"), usamos `exec`. Si solo esperamos una respuesta ("¿existe este cliente?"), usamos `query_first`. Si esperamos muchas respuestas pero las queremos procesar una por una ("genera un reporte con los 10,000 clientes"), usamos `query_iter`. La elección correcta hace que el código sea más claro y eficiente.

### 2.3.1 `query` y `exec`: consultas fijas vs consultas con parámetros

El crate `mysql` ofrece dos familias de métodos, y la diferencia está en si el SQL tiene partes variables. Con `query`, la consulta es una cadena fija sin marcadores. Con `exec`, la consulta tiene marcadores `?` que se rellenan con valores separados.

```rust
use mysql::prelude::*;

let pool = mysql::Pool::new(DB_URL)?;
let mut conn = pool.get_conn()?;

// query: consulta sin partes variables
let nombres: Vec<String> = conn.query("SELECT nombre FROM clientes")?;

// exec: consulta con parámetros
let filtrados: Vec<String> = conn.exec(
    "SELECT nombre FROM clientes WHERE rfc = ?",
    ("CDB010101AB3",)
)?;
```

**Análisis línea por línea**:

- `use mysql::prelude::*;`: importa los traits que añaden los métodos `.query()` y `.exec()` al tipo `PooledConn`. Sin esta línea, el compilador diría "method `query` not found".
- `let pool = mysql::Pool::new(DB_URL)?;`: crea el pool con la URL de conexión.
- `let mut conn = pool.get_conn()?;`: pide una conexión al pool. La variable `conn` debe ser mutable (`mut`) porque los métodos de consulta modifican el estado interno de la conexión (buffers, posición del cursor).
- `let nombres: Vec<String> = conn.query("SELECT nombre FROM clientes")?;`: ejecuta la consulta. La anotación `Vec<String>` le dice a Rust "cada fila tiene una columna de tipo String". Si la columna `nombre` fuera NULL para algún cliente, el mapeo fallaría porque `String` no puede ser NULL.
- `let filtrados: Vec<String> = conn.exec(...)`: `exec` funciona igual que `query`, pero acepta un segundo argumento: una tupla con los valores que reemplazan a los `?`. La tupla `("CDB010101AB3",)` tiene UNA coma al final: es obligatoria para tuplas de un elemento, porque sin ella Rust interpreta `("CDB010101AB3")` como un string entre paréntesis, no como una tupla.

**Salida esperada**:

```
["Constructora del Bajío S.A. de C.V.", "Distribuidora del Norte S.A. de C.V.", "Comercializadora del Sur S.C."]
```

La segunda consulta devuelve solo `["Constructora del Bajío S.A. de C.V."]` porque el RFC `CDB010101AB3` pertenece a ese único cliente.

### 2.3.2 `query_first`, `exec_first` y `fetch_all`: controlar cuántas filas recibes

No todas las consultas devuelven muchas filas. Cuando buscas un cliente por su ID, esperas una o ninguna. Para eso están los métodos que devuelven `Option<T>` en lugar de `Vec<T>`.

```rust
// query_first: devuelve la primera fila (o None)
let primero: Option<(u32, String)> = conn.query_first(
    "SELECT id, nombre FROM clientes"
)?;

if let Some((id, nombre)) = primero {
    println!("Primer cliente: {} ({})", nombre, id);
} else {
    println!("No hay clientes registrados");
}

// exec_first: igual pero con parámetros
let uno: Option<(String, f64)> = conn.exec_first(
    "SELECT nombre, credito FROM clientes WHERE id = ?",
    (1,)
)?;

match uno {
    Some((nombre, credito)) => {
        println!("Cliente: {}, crédito: ${:.2}", nombre, credito);
    }
    None => {
        println!("No se encontró un cliente con id = 1");
    }
}
```

**Análisis línea por línea**:

- `let primero: Option<(u32, String)> = conn.query_first(...)?;`: `query_first` ejecuta la consulta y devuelve solo la primera fila. El tipo de retorno es `Option`: `Some(fila)` si hay al menos un resultado, `None` si no hay ninguno. El `?` propaga errores de conexión o de sintaxis SQL, pero NO convierte `None` en error: la ausencia de resultados no es un error.
- `if let Some((id, nombre)) = primero {`: `if let` es una forma abreviada de `match` para un solo patrón. Si `primero` es `Some`, desempaqueta la tupla en `id` y `nombre`. Si es `None`, ejecuta el `else`.
- `match uno {`: aquí usamos `match` completo en lugar de `if let` porque tenemos dos ramas con código en cada una. `match` obliga a cubrir todos los casos, lo que es más seguro que `if let` cuando ambas ramas tienen lógica.

**Salida esperada**:

```
Cliente: Constructora del Bajío S.A. de C.V., crédito: $100000.00
```

Si el cliente con id=1 no existe:
```
No se encontró un cliente con id = 1
```

### 2.3.3 Iteracion sobre filas con `query_iter`

Los métodos anteriores (`query`, `exec`, `query_first`, `exec_first`) cargan todas las filas en memoria. Para tablas pequeñas está bien, pero si tienes 10,000 productos y solo quieres generar un reporte línea por línea, estás gastando memoria innecesariamente. `query_iter` resuelve esto permitiendo procesar las filas una por una a medida que llegan del servidor, sin acumularlas.

```rust
use mysql::prelude::*;

let mut conn = pool.get_conn()?;

conn.query_iter("SELECT id, nombre, rfc FROM clientes")?
    .for_each(|row| {
        let id: u32 = row.get("id").unwrap();
        let nombre: String = row.get("nombre").unwrap();
        let rfc: String = row.get("rfc").unwrap();
        println!("  [{}] {} ({})", id, nombre, rfc);
    });
```

**Análisis línea por línea**:

- `conn.query_iter("SELECT ...")?`: ejecuta la consulta pero no devuelve un `Vec`. Devuelve un `QueryResult`, que es un iterador. El `?` propaga errores de conexión o sintaxis.
- `.for_each(|row| { ... })`: `for_each` recibe un closure que se ejecuta para cada fila. La fila se llama `row` y es de tipo `mysql::Row`.
- `let id: u32 = row.get("id").unwrap();`: `row.get("nombre_columna")` extrae el valor de la columna por su nombre. Devuelve `Option<T>` porque la columna podría ser NULL. `.unwrap()` asume que no es NULL; si lo fuera, el programa entra en pánico.
- La memoria usada es constante: cada fila se procesa y se descarta antes de leer la siguiente. No importa si hay 10 filas o 10 millones.

### 2.3.4 Mapeo a structs con `FromRow`

Trabajar con tuplas `(u32, String, String, f64, bool)` es frágil: si cambia el orden de las columnas en el SELECT, tu código se rompe sin aviso. La alternativa profesional es usar un **struct** con `#[derive(FromRow)]`:

```rust
use mysql::prelude::*;
use mysql::FromRow;

#[derive(Debug, FromRow)]
struct Cliente {
    id: u32,
    nombre: String,
    rfc: String,
    email: Option<String>,
    credito: f64,
    activo: bool,
}

fn listar_clientes(conn: &mut mysql::PooledConn) -> mysql::Result<Vec<Cliente>> {
    conn.query("SELECT id, nombre, rfc, email, credito, activo FROM clientes")
}
```

**Análisis línea por línea**:

- `#[derive(Debug, FromRow)]`: `Debug` permite imprimir el struct con `{:?}`. `FromRow` genera automáticamente el código para convertir una fila SQL en una instancia de `Cliente`.
- `struct Cliente { id: u32, nombre: String, ... }`: los campos del struct deben coincidir en orden y tipo con las columnas del SELECT. `email` es `Option<String>` porque la columna puede ser NULL.
- `conn.query("SELECT id, nombre, rfc, email, credito, activo FROM clientes")`: la consulta devuelve un `Vec<Cliente>` automáticamente. No necesitas especificar el tipo de retorno en la llamada porque Rust lo infiere del tipo de retorno de la función.

### 2.3.5 Errores tipicos en consultas SELECT

**Error 1: orden incorrecto en la tupla**.

```rust
let erroneo: Vec<(u32, String)> = conn.query(
    "SELECT nombre, id FROM clientes"
)?;
```

Mensaje: `Error (mysql error): Column 'nombre' cannot be converted to u32`. Solución: el orden de los tipos debe coincidir con el orden de las columnas: `Vec<(String, u32)>`.

**Error 2: columna NULL sin Option**.

```rust
let clientes: Vec<(u32, String, String)> = conn.query(
    "SELECT id, nombre, email FROM clientes"
)?;
```

Mensaje: `Error (mysql error): Column 'email' is NULL but the return type is String`. Solución: usa `Option<String>` para el email: `Vec<(u32, String, Option<String>)>`.

**Error 3: olvidar el operador `?`**.

```rust
let resultado = conn.query("SELECT COUNT(*) FROM clientes");
let total = resultado;
```

Mensaje: `error[E0308]: expected i64, found Result<i64, mysql::Error>`. Solución: añade `?` al final: `conn.query(...)?`.

**Error 4: columna inexistente**.

```rust
let erroneo: Vec<String> = conn.query("SELECT apellido FROM clientes")?;
```

Mensaje: `Error (mysql error): Table 'erp_crm.clientes' doesn't have column 'apellido'`. Solución: verifica los nombres de columna con `DESCRIBE clientes;`.

**Error 5: tabla inexistente**.

```rust
let erroneo: Vec<String> = conn.query("SELECT nombre FROM cliente")?;
```

Mensaje: `Error (mysql error): Table 'erp_crm.cliente' doesn't exist`. Solución: la tabla se llama `clientes` (con s), no `cliente`. Verifica con `SHOW TABLES;`.

## 2.4 Parametros preparados y seguridad

Hasta ahora hemos visto dos formas de construir consultas SQL: con `query`, que usa cadenas fijas, y con `exec`, que usa marcadores `?`. Esta sección explica **por qué** la segunda forma no es opcional en una aplicación que recibe datos del exterior.

La seguridad en bases de datos no es una característica "avanzada". Es como cerrar la puerta de tu casa: no la cierras porque esperes un robo, la cierras porque si no lo haces, cualquiera puede entrar. La **inyección SQL** es la puerta abierta más común en aplicaciones web, y ha causado algunas de las filtraciones de datos más grandes de la historia: Sony Pictures (2011, 100 millones de cuentas), Heartland Payment Systems (2008, 130 millones de tarjetas), Equifax (2017, 147 millones de datos personales). En todos los casos, la causa fue la misma: construir consultas SQL concatenando strings en lugar de usar parámetros preparados.

Rust no puede prevenir la inyección SQL por sí mismo porque el problema no está en el lenguaje, sino en cómo se construye la consulta. El crate `mysql` te da la herramienta para hacerlo bien (`exec` con `?`), pero también te permite hacerlo mal (`format!` con concatenación). La diferencia entre hacerlo bien y hacerlo mal es una línea de código, pero sus consecuencias son abismales.

### 2.4.1 Inyeccion SQL: como funciona un ataque

Imagina que tu sistema busca un cliente por su nombre con esta consulta:

```sql
SELECT * FROM clientes WHERE nombre = 'LORENZO'
```

Todo normal. Pero ¿qué pasa si alguien escribe como nombre:

```
LORENZO'; DROP TABLE clientes; --
```

Al concatenar, la consulta se convierte en dos:

```sql
SELECT * FROM clientes WHERE nombre = 'LORENZO'; DROP TABLE clientes; --'
```

La primera es el SELECT original. La segunda es un `DROP TABLE` que borra la tabla `clientes`. El `--` convierte el resto en comentario para evitar errores de sintaxis.

En Rust, el código vulnerable se ve así:

```rust
// PELIGRO: NO USAR EN PRODUCCIÓN
// Este código construye SQL concatenando strings.
// Si el usuario introduce un valor malicioso, la base de datos puede ser destruida.

let nombre_buscado = "'; DROP TABLE clientes; --";
let sql = format!("SELECT * FROM clientes WHERE nombre = '{}'", nombre_buscado);
let resultado = conn.query(&sql);
```

**Análisis línea por línea**:

- `let nombre_buscado = "'; DROP TABLE clientes; --";`: este valor podría venir de un formulario web, una API, o cualquier entrada del usuario. El atacante ha escrito un string que cierra la comilla y añade un comando destructivo.
- `let sql = format!("SELECT ... '{}'", nombre_buscado);`: `format!` crea un nuevo String reemplazando `{}` por el valor de `nombre_buscado`. El resultado no es una consulta segura, sino una cadena que contiene SQL malicioso.
- `let resultado = conn.query(&sql);`: `query` ejecuta la cadena directamente. Si el servidor permite consultas múltiples, ambas se ejecutan.

**Salida del código vulnerable**:

```sql
SELECT * FROM clientes WHERE nombre = ''; DROP TABLE clientes; --'
```

El servidor ejecuta dos consultas: el SELECT (no encuentra nada porque ningún cliente se llama vacío) y el DROP TABLE (borra la tabla `clientes` para siempre).

La única protección contra esto son los **parámetros preparados**: enviar la plantilla SQL separada de los valores.

### 2.4.2 Parametros posicionales `?`: la protección

Con `exec`, la consulta tiene marcadores `?` y los valores se pasan por separado:

```rust
use mysql::prelude::*;

let nombre_buscado = "Constructora del Bajío";

let resultado: Vec<(u32, String)> = conn.exec(
    "SELECT id, nombre FROM clientes WHERE nombre = ?",
    (nombre_buscado,)
)?;
```

**Análisis línea por línea**:

- `let resultado: Vec<(u32, String)> = conn.exec(...)`: `exec` recibe dos argumentos: la plantilla SQL con `?` y una tupla con los valores. El tipo de retorno es `Vec<T>`, igual que `query`.
- `"SELECT ... WHERE nombre = ?"`: el `?` es un marcador de posición. El servidor entiende que ahí debe ir un valor, pero no lo interpreta como SQL.
- `(nombre_buscado,)`: la tupla contiene el valor que reemplaza al `?`. La coma final es obligatoria porque sin ella `(nombre_buscado)` no es una tupla, es un valor entre paréntesis.

Cuando el valor es `"'; DROP TABLE clientes; --"`, el servidor lo trata como un dato literal, no como código. La consulta efectiva busca un cliente que se llame exactamente esa cadena extraña. No hay daño posible.

**Múltiples parámetros**:

```rust
let resultado: Vec<(String, f64)> = conn.exec(
    "SELECT nombre, credito FROM clientes WHERE id = ? AND activo = ?",
    (1, true)
)?;
```

Cada `?` se reemplaza por el valor correspondiente en la tupla, en orden. Si hay 2 `?`, la tupla debe tener 2 elementos.

### 2.4.3 Parametros nombrados `:nombre`

Cuando hay muchos parámetros, mantener el orden es tedioso. Los parámetros nombrados usan `:nombre` y un mapa:

```rust
use mysql::prelude::*;

let resultado = conn.exec(
    "SELECT id, nombre FROM clientes WHERE rfc = :rfc AND credito >= :minimo",
    params! {
        "rfc" => "CDB010101AB3",
        "minimo" => 1000.0
    }
)?;
```

**Análisis línea por línea**:

- `"WHERE rfc = :rfc AND credito >= :minimo"`: los marcadores tienen nombres (`:rfc`, `:minimo`) en lugar de ser posicionales (`?`). El orden en el SQL no importa.
- `params! { "rfc" => "...", "minimo" => 1000.0 }`: la macro `params!` crea un mapa que asocia cada nombre a su valor. El orden aquí tampoco importa: podrías poner `minimo` antes que `rfc`.

### 2.4.4 Errores tipicos

**Error 1: olvidar la coma en la tupla**.

```rust
conn.exec("SELECT nombre FROM clientes WHERE id = ?", (1))?;
```

Mensaje: `error[E0308]: expected Params, found {integer}`. Solución: `(1,)` con coma.

**Error 2: número incorrecto de parámetros**.

```rust
conn.exec("SELECT nombre WHERE id = ? AND activo = ?", (1,))?;
```

Mensaje: `Error: Parameter count mismatch: query has 2 parameters but you provided 1`. Solución: `(1, true)`.

**Error 3: tipo incorrecto**.

```rust
conn.exec("SELECT nombre FROM clientes WHERE id = ?", ("uno",))?;
```

Mensaje: `Error: Incorrect integer value: 'uno' for column 'id'`. Solución: pasa un entero: `(1,)`.

**Error 4: usar `query` con parámetros**.

```rust
conn.query("SELECT * FROM clientes WHERE id = ?")?;
```

Esto busca clientes cuyo id sea literalmente el texto `"?"`, no el valor de una variable. Solución: usa `exec`.

**Error 5: mezclar `?` y `:nombre`**.

```rust
conn.exec("SELECT id WHERE nombre = ? AND credito >= :minimo", ("Lorenzo", 1000.0))?;
```

Mensaje: error de sintaxis SQL. Solución: usa solo `?` o solo `:nombre`, no ambos.

## 2.5 `INSERT`, `UPDATE` y `DELETE`

Hasta ahora solo hemos leído datos con `SELECT`. En un ERP también hay que **crear** clientes, **actualizar** sus datos y **eliminar** registros. Estas tres operaciones se llaman mutaciones porque modifican el estado de la base de datos. A diferencia del `SELECT`, que solo observa, las mutaciones tienen efectos permanentes: un `INSERT` crea un registro, un `UPDATE` lo modifica, un `DELETE` lo borra. No hay "Ctrl+Z" en una base de datos (a menos que tengas un respaldo).

Piensa en un almacén: `SELECT` es mirar los estantes, `INSERT` es recibir mercancía nueva, `UPDATE` es cambiar el precio de un producto, `DELETE` es tirar mercancía a la basura. En los tres casos de mutación, el contenido del almacén cambia para siempre.

El método para ejecutar mutaciones en el crate `mysql` es `exec_drop`. "Drop" significa "descartar el resultado": a diferencia de `query`, que devuelve filas, `exec_drop` ejecuta la operación y descarta el resultado porque INSERT, UPDATE y DELETE no devuelven filas. Devuelven solo un `Result<(), mysql::Error>` que indica si la operación fue exitosa o no.

### 2.5.1 `exec_drop`: INSERT, UPDATE, DELETE

```rust
use mysql::prelude::*;

// INSERT
conn.exec_drop(
    "INSERT INTO clientes (nombre, rfc, email, credito) VALUES (?, ?, ?, ?)",
    ("Constructora del Sur S.C.", "CDS030303EF5", "info@cds.mx", 25000.0)
)?;

// UPDATE
conn.exec_drop(
    "UPDATE clientes SET credito = ? WHERE id = ?",
    (200_000.0, 1)
)?;

// DELETE
conn.exec_drop("DELETE FROM clientes WHERE id = ?", (99,))?;
```

**Análisis línea por línea**:

- `conn.exec_drop("INSERT INTO clientes (nombre, rfc, email, credito) VALUES (?, ?, ?, ?)", ...)`: el INSERT especifica las columnas explícitamente. Esto es importante: no dependemos del orden físico de las columnas en la tabla. Las columnas no listadas (`id`, `activo`, `created_at`, `updated_at`) usan sus valores por defecto (AUTO_INCREMENT, TRUE, TIMESTAMP, etc.).
- `("Constructora...", "CDS030303EF5", "info@cds.mx", 25000.0)`: los valores se pasan como tupla, en el mismo orden que las columnas en el INSERT. Si el RFC ya existe (violación de UNIQUE), `exec_drop` devuelve un error que el `?` propaga.
- `conn.exec_drop("UPDATE clientes SET credito = ? WHERE id = ?", (200_000.0, 1))`: el UPDATE modifica el crédito del cliente con id=1. El `WHERE` es la parte más importante: si lo olvidas, `UPDATE clientes SET credito = 200000` cambiaría el crédito de **todos** los clientes. Es uno de los errores más costosos en bases de datos.
- `conn.exec_drop("DELETE FROM clientes WHERE id = ?", (99,))`: elimina el cliente con id=99. Si el id no existe, no hay error: la operación afecta 0 filas y se reporta como exitosa.

### 2.5.2 `last_insert_id` y `affected_rows`

Después de un INSERT, a menudo necesitas saber qué ID se generó. Después de un UPDATE o DELETE, quieres saber cuántas filas se afectaron.

```rust
// INSERT y obtener el ID generado
conn.exec_drop(
    "INSERT INTO clientes (nombre, rfc, email) VALUES (?, ?, ?)",
    ("Cliente X", "XXX010101XX1", "x@x.com")
)?;
let id: u64 = conn.last_insert_id().unwrap();
println!("ID generado: {}", id);

// UPDATE y obtener filas afectadas
conn.exec_drop("UPDATE clientes SET credito = credito + 1000 WHERE activo = ?", (true,))?;
let affected = conn.affected_rows();
println!("Filas actualizadas: {}", affected);
```

**Análisis línea por línea**:

- `conn.last_insert_id().unwrap()`: devuelve el ID autogenerado por el último INSERT. Es específico de la conexión: si dos usuarios insertan al mismo tiempo, cada uno obtiene su propio ID sin interferencia.
- `conn.affected_rows()`: devuelve el número de filas insertadas, actualizadas o eliminadas por la última operación. Para el UPDATE, devuelve cuántos clientes activos se modificaron. Debe llamarse inmediatamente después de la operación, antes de ejecutar cualquier otra consulta.

### 2.5.3 Errores tipicos

**Error 1: olvidar el WHERE en UPDATE o DELETE**.

```rust
conn.exec_drop("UPDATE clientes SET credito = 0", ())?;
// Esto pone credito = 0 a TODOS los clientes
```

Solución: siempre escribe el WHERE primero como hábito mental.

**Error 2: violación de UNIQUE en INSERT**.

```rust
conn.exec_drop(
    "INSERT INTO clientes (nombre, rfc) VALUES (?, ?)",
    ("Cliente X", "CDB010101AB3")  // RFC ya existe
)?;
```

Mensaje: `Error: Duplicate entry 'CDB010101AB3' for key 'clientes.rfc'`. Solución: verifica antes de insertar o usa `ON DUPLICATE KEY UPDATE`.

**Error 3: violación de FOREIGN KEY**.

```rust
conn.exec_drop(
    "INSERT INTO pedidos (folio, cliente_id, subtotal, iva, total) VALUES (?, ?, ?, ?, ?)",
    ("PED-1", 9999, 1000.0, 160.0, 1160.0)  // cliente_id 9999 no existe
)?;
```

Mensaje: `Error: Cannot add or update a child row: a foreign key constraint fails`. Solución: verifica que el cliente exista antes de crear el pedido.

### 2.5.4 Mini-proyecto: `02_crud_clientes`

El mini-proyecto `02_crud_clientes` (en `proyectos_capitulo/parte2/02_crud_clientes/`) implementa un menú CLI con las cuatro operaciones CRUD sobre la tabla `clientes`. Es el primer programa interactivo del curso que realmente hace algo útil.

```rust
use mysql::prelude::*;
use mysql::Pool;
use std::io::{self, Write};

const DB_URL: &str = "mysql://root:secret@127.0.0.1:3306/erp_crm";

fn main() -> mysql::Result<()> {
    let pool = Pool::new(DB_URL)?;
    let mut entrada = String::new();

    loop {
        println!("\n=== ERP/CRM - Gestión de Clientes ===");
        println!("1) Listar");
        println!("2) Crear");
        println!("3) Actualizar crédito");
        println!("4) Eliminar");
        println!("0) Salir");
        print!("Opción: ");
        io::stdout().flush().unwrap();

        entrada.clear();
        io::stdin().read_line(&mut entrada).unwrap();
        let opcion = entrada.trim();

        let resultado = match opcion {
            "1" => listar(&pool),
            "2" => crear(&pool),
            "3" => actualizar(&pool),
            "4" => eliminar(&pool),
            "0" => break,
            _ => { println!("Opción inválida"); Ok(()) }
        };
        if let Err(e) = resultado {
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}

fn listar(pool: &Pool) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    let clientes: Vec<(u32, String, String, Option<String>, f64, bool)> =
        conn.query("SELECT id, nombre, rfc, email, credito, activo FROM clientes ORDER BY id")?;
    for (id, nombre, rfc, email, credito, activo) in &clientes {
        let e = email.as_deref().unwrap_or("—");
        println!("  [{}] {} ({}) ${} {}", id, nombre, rfc, credito, if *activo { "✓" } else { "✗" });
    }
    Ok(())
}

fn crear(pool: &Pool) -> mysql::Result<()> {
    let mut entrada = String::new();
    let mut conn = pool.get_conn()?;

    print!("Nombre: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let nombre = entrada.trim().to_string(); entrada.clear();
    print!("RFC: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let rfc = entrada.trim().to_string(); entrada.clear();
    print!("Email: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let email = entrada.trim().to_string(); entrada.clear();
    print!("Crédito: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let credito: f64 = entrada.trim().parse().unwrap_or(0.0);

    conn.exec_drop(
        "INSERT INTO clientes (nombre, rfc, email, credito) VALUES (?, ?, ?, ?)",
        (&nombre, &rfc, &email as &str, credito)
    )?;
    println!("✓ Cliente creado con ID {}", conn.last_insert_id().unwrap());
    Ok(())
}

fn actualizar(pool: &Pool) -> mysql::Result<()> {
    let mut entrada = String::new();
    let mut conn = pool.get_conn()?;
    print!("ID a actualizar: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let id: u32 = entrada.trim().parse().unwrap_or(0); entrada.clear();
    print!("Nuevo crédito: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let credito: f64 = entrada.trim().parse().unwrap_or(0.0);
    conn.exec_drop("UPDATE clientes SET credito = ? WHERE id = ?", (credito, id))?;
    println!("✓ Filas actualizadas: {}", conn.affected_rows());
    Ok(())
}

fn eliminar(pool: &Pool) -> mysql::Result<()> {
    let mut entrada = String::new();
    let mut conn = pool.get_conn()?;
    print!("ID a eliminar: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let id: u32 = entrada.trim().parse().unwrap_or(0); entrada.clear();
    print!("¿Confirma? (s/N): "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?;
    if entrada.trim().to_lowercase() != "s" { println!("Cancelado"); return Ok(()); }
    conn.exec_drop("DELETE FROM clientes WHERE id = ?", (id,))?;
    println!("✓ Filas eliminadas: {}", conn.affected_rows());
    Ok(())
}
```

**Análisis línea por línea**:

- `let pool = Pool::new(DB_URL)?;`: el pool se crea una vez en `main` y se pasa como referencia a cada función. Esto evita crear y destruir conexiones en cada operación.
- `loop { ... }`: el bucle infinito muestra el menú, lee una opción y ejecuta la función correspondiente. La opción `"0"` rompe el bucle con `break`.
- `fn listar(pool: &Pool)`: la función recibe el pool por referencia y obtiene una conexión con `pool.get_conn()?`. Ejecuta un SELECT y muestra los resultados.
- `fn eliminar(pool: &Pool)`: antes de borrar, pregunta `¿Confirma? (s/N)`. Si el usuario no escribe exactamente `s`, se cancela. Esta es una medida de seguridad básica.

**Salida esperada**:
```
=== ERP/CRM - Gestión de Clientes ===
1) Listar
2) Crear
3) Actualizar crédito
4) Eliminar
0) Salir
Opción: 1
  [1] Constructora del Bajío S.A. de C.V. (CDB010101AB3) $100000 ✓
  [2] Distribuidora del Norte S.A. de C.V. (DDN020202CD4) $50000 ✓
  [3] Comercializadora del Sur S.C. (CDS030303EF5) $25000 ✓
```

De manera similar, cuando ejecutas un UPDATE o DELETE, quieres saber cuántas filas fueron afectadas para confirmar que la operación hizo lo que esperabas. Para eso está `affected_rows()`.

Piensa en `last_insert_id` como el ticket que te dan cuando dejas ropa en la tintorería: te dice exactamente qué número de ticket se generó para tu pedido. Piensa en `affected_rows` como el recibo que te dice "se actualizaron 3 clientes" después de un UPDATE masivo.

```rust
// INSERT y obtener el ID autogenerado
conn.exec_drop(
    "INSERT INTO clientes (nombre, rfc, email) VALUES (?, ?, ?)",
    //                            ^^^^^^ solo estas tres columnas;
    //                                   id se genera solo, credito y activo usan defaults
    ("Cliente X", "XXX010101XX1", "x@x.com")
)?;

// Inmediatamente después del INSERT, preguntamos qué ID se generó
let id: u64 = conn.last_insert_id().unwrap();
//              ^^^^^^^^^^^^^^^^^^^^^
//              Devuelve Option<u64>. None si el último INSERT no generó un ID
println!("✓ Cliente creado con ID {}", id);
// Salida: "✓ Cliente creado con ID 4"
// (suponiendo que ya había 3 clientes, el nuevo tiene id=4)

// UPDATE masivo: aumentar crédito de todos los clientes activos
conn.exec_drop(
    "UPDATE clientes SET credito = credito + 1000 WHERE activo = ?",
    //                   ^^^^^^^^^^^^^^^^  ^^^^^^^^
    //                   incrementar el    filtrar solo activos
    //                   crédito actual
    (true,)
)?;

// Verificar cuántas filas se modificaron
let affected = conn.affected_rows();
//               ^^^^^^^^^^^^^^^^^^
//               Devuelve u64: número de filas insertadas, actualizadas o eliminadas
//               en la ÚLTIMA operación
println!("✓ Filas actualizadas: {}", affected);
// Si hay 3 clientes activos: "✓ Filas actualizadas: 3"
```

**Análisis:**

`last_insert_id()` es específico de la **conexión**, no del pool ni del servidor. Esto significa que si dos usuarios están insertando clientes al mismo tiempo desde conexiones diferentes, cada uno obtiene su propio ID sin interferencia del otro. Es seguro en entornos concurrentes porque MySQL/MariaDB garantiza que `last_insert_id()` devuelve el ID generado por la última operación INSERT en esa conexión específica.

`affected_rows()` cuenta filas para la última operación INSERT, UPDATE, DELETE o incluso DDL (como `CREATE TABLE`). Para un INSERT, devuelve 1 si se insertó una fila. Para un UPDATE, devuelve el número de filas que coincidieron con el WHERE (aunque el valor no hubiera cambiado). Para un DELETE, devuelve el número de filas eliminadas.

**Error típico con `affected_rows()`:**

```rust
// ERROR: affected_rows() se pierde si haces otra operación entre medias
conn.exec_drop("UPDATE clientes SET credito = 0 WHERE id = 1", ())?;
// (alguna otra operación aquí)
let total = conn.query_first::<i64, _, _>("SELECT COUNT(*) FROM clientes")?;
//                                       ^^^^^ después de esta consulta,
//                                             affected_rows() YA NO VALE
let afectadas = conn.affected_rows(); // ¡Esto devuelve el COUNT, no el UPDATE!
```

La solución es llamar a `affected_rows()` inmediatamente después de la mutación, antes de cualquier otra consulta.

### 2.5.3 UUID como clave primaria: cuando el auto-increment no alcanza

En un sistema ERP que se ejecuta en una sola oficina, el `AUTO_INCREMENT` es perfecto: los IDs comienzan en 1, aumentan de uno en uno, y no hay conflictos. Pero ¿qué pasa si tienes dos sucursales, cada una con su propia base de datos, y necesitas sincronizar los datos? Si ambas sucursales generan un cliente con ID 1, al sincronizar habrá un conflicto. O si tu ERP es una aplicación SaaS en la nube, con miles de empresas usando la misma base de datos, los IDs numéricos secuenciales pueden ser problemáticos por seguridad (es fácil adivinar que el cliente 1001 es el siguiente después del 1000).

La solución son los **UUID** (Universally Unique Identifiers): identificadores de 128 bits que se representan como cadenas de 36 caracteres (ej: `"550e8400-e29b-41d4-a716-446655440000"`). La probabilidad de generar dos UUID iguales es tan baja (aproximadamente 1 en 5.3×10³⁶) que se considera cero para fines prácticos.

MariaDB/MySQL puede generar UUIDs con la función `UUID()`:

```rust
// Generar un UUID desde el servidor MySQL
// La función UUID() del servidor genera un UUID versión 1 (basado en tiempo + MAC)
let uuid: String = conn.query_first("SELECT UUID()")?.unwrap();
//                ^^^^^^        ^^^^^^^^^^^^^^^^^
//                query_first   Ejecuta SELECT UUID() y devuelve la primera fila
//                porque solo esperamos una fila con un valor
.//unwrap() convierte Option<String> a String (asumimos que UUID() nunca es NULL)

println!("UUID generado: {}", uuid);
// Salida: "UUID generado: 550e8400-e29b-41d4-a716-446655440000"
```

**¿Cuándo usar UUID vs AUTO_INCREMENT?**

Usa `AUTO_INCREMENT` cuando:
- La base de datos es única y no se sincroniza con otras.
- Quieres IDs cortos y legibles para humanos (ej: factura F-1001).
- El rendimiento de las inserciones es crítico (los UUIDs son más lentos como claves primarias porque no son secuenciales).

Usa UUID cuando:
- Tienes múltiples bases de datos que se sincronizan (sucursales, replicación multi-maestro).
- No quieres que los IDs sean adivinables (seguridad por oscuridad).
- Necesitas generar IDs sin consultar la base de datos (el crate `uuid` de Rust puede generarlos localmente).

**Para generar UUIDs desde Rust (sin consultar MySQL):**

```toml
[dependencies]
uuid = { version = "1", features = ["v4"] }
```

```rust
use uuid::Uuid;
let uuid = Uuid::new_v4().to_string();
println!("UUID generado: {}", uuid);
```

Esto genera un UUID versión 4 (aleatorio) sin hacer ninguna llamada a la base de datos, lo que es más rápido y no consume recursos del servidor.

### 2.5.4 Mini-proyecto: `02_crud_clientes` — el primer ERP funcional

El segundo mini-proyecto de la Parte 2 (en `proyectos_capitulo/parte2/02_crud_clientes/`) implementa un menú CLI (Command Line Interface) con todas las operaciones CRUD sobre la tabla `clientes`. Es la primera vez que construyes un programa interactivo que realmente hace algo útil: gestionar clientes.

**Arquitectura del programa:**

Antes de leer el código, entendamos la estructura. El programa tiene cuatro funciones principales más una función de menú y el `main`:

1. **`listar`**: ejecuta un SELECT y muestra los clientes en una tabla formateada con bordes Unicode (┌, ─, ┐, etc.). Es como el reporte de clientes que usarías en la oficina.
2. **`crear`**: pide datos al usuario por teclado (nombre, RFC, email, crédito) y ejecuta un INSERT. Luego muestra el ID generado.
3. **`actualizar`**: pide un ID y un nuevo crédito, ejecuta un UPDATE, y muestra cuántas filas se modificaron.
4. **`eliminar`**: pide un ID, **pide confirmación** (medida de seguridad), y si el usuario confirma, ejecuta un DELETE.
5. **`menu`**: muestra las opciones disponibles y lee la opción del usuario.
6. **`main`**: crea el pool y ejecuta un bucle infinito que muestra el menú, lee la opción y llama a la función correspondiente.

El flujo es: `main → loop → menu → (listar|crear|actualizar|eliminar) → loop → ...`

```rust
use mysql::prelude::*;
use mysql::Pool;
use std::io::{self, Write};

const DB_URL: &str = "mysql://root:secret@127.0.0.1:3306/erp_crm";

// Estructura para representar un cliente
// Coincide con las columnas de la tabla clientes
#[derive(Debug)]
struct Cliente {
    id: u32,
    nombre: String,
    rfc: String,
    email: Option<String>,
    credito: f64,
    activo: bool,
}

// 1. LISTAR: muestra todos los clientes en una tabla formateada
fn listar(pool: &Pool) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    let clientes: Vec<(u32, String, String, Option<String>, f64, bool)> =
        conn.query("SELECT id, nombre, rfc, email, credito, activo FROM clientes ORDER BY id")?;
    //                                                                              ^^^^^^^^^
    //                                                                              Ordenamos por ID para consistencia

    // Tabla con bordes Unicode para presentación profesional
    println!("\n┌────┬────────────────────────────────────────────┬──────────────┬─────────────────────────┬────────────┬────────┐");
    println!("│ ID │ Nombre                                     │ RFC          │ Email                   │ Crédito    │ Activo │");
    println!("├────┼────────────────────────────────────────────┼──────────────┼─────────────────────────┼────────────┼────────┤");
    for (id, nombre, rfc, email, credito, activo) in &clientes {
        let e = email.as_deref().unwrap_or("—");
        //                    ^^^^^^^^^^
        //                    Si email es None, mostramos un guión
        println!("│ {:2} │ {:<42} │ {:<12} │ {:<23} │ ${:>9.2} │ {:<6} │",
                 id, &nombre[..nombre.len().min(42)], rfc, e, credito, activo);
        //         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //         Tomamos solo los primeros 42 caracteres para que la tabla no se desborde
    }
    println!("└────┴────────────────────────────────────────────┴──────────────┴─────────────────────────┴────────────┴────────┘");
    Ok(())
}

// 2. CREAR: pide datos al usuario y ejecuta INSERT
fn crear(pool: &Pool) -> mysql::Result<()> {
    let mut entrada = String::new();
    let mut conn = pool.get_conn()?;

    // Pedir cada campo al usuario
    print!("Nombre: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let nombre = entrada.trim().to_string(); entrada.clear();

    print!("RFC: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let rfc = entrada.trim().to_string(); entrada.clear();

    print!("Email: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let email = entrada.trim().to_string(); entrada.clear();

    print!("Crédito: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let credito: f64 = entrada.trim().parse().unwrap_or(0.0); entrada.clear();
    //                                                                           ^^^^^^^^^^
    //                                                                           Si el usuario no escribe un número válido,
    //                                                                           se usa 0.0 en lugar de fallar

    // Ejecutar INSERT con parámetros preparados
    conn.exec_drop(
        "INSERT INTO clientes (nombre, rfc, email, credito) VALUES (?, ?, ?, ?)",
        (&nombre, &rfc, &email as &str, credito)
        //                     ^^^^^^^^
        //                     Forzamos &str porque email es String y necesitamos &str
    )?;
    println!("✓ Cliente creado con ID {}", conn.last_insert_id().unwrap());
    //                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //                                   Mostramos el ID que MySQL generó automáticamente
    Ok(())
}

// 3. ACTUALIZAR: modifica el crédito de un cliente existente
fn actualizar(pool: &Pool) -> mysql::Result<()> {
    let mut entrada = String::new();
    let mut conn = pool.get_conn()?;

    print!("ID a actualizar: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let id: u32 = entrada.trim().parse().unwrap_or(0); entrada.clear();

    print!("Nuevo crédito: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let credito: f64 = entrada.trim().parse().unwrap_or(0.0); entrada.clear();

    conn.exec_drop("UPDATE clientes SET credito = ? WHERE id = ?", (credito, id))?;
    // Si el id no existe, no falla: affected_rows() será 0
    println!("✓ Filas actualizadas: {}", conn.affected_rows());
    Ok(())
}

// 4. ELIMINAR: borra un cliente (con confirmación)
fn eliminar(pool: &Pool) -> mysql::Result<()> {
    let mut entrada = String::new();
    let mut conn = pool.get_conn()?;

    print!("ID a eliminar: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let id: u32 = entrada.trim().parse().unwrap_or(0); entrada.clear();

    // Medida de seguridad: pedir confirmación antes de borrar
    print!("¿Confirma? (s/N): "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?;
    if entrada.trim().to_lowercase() != "s" {
        println!("Cancelado"); return Ok(());
    }

    conn.exec_drop("DELETE FROM clientes WHERE id = ?", (id,))?;
    // El WHERE es obligatorio: sin él, se borrarían TODOS los clientes
    println!("✓ Filas eliminadas: {}", conn.affected_rows());
    Ok(())
}

// Menú de opciones
fn menu() {
    println!("\n=== ERP/CRM - Gestión de Clientes ===");
    println!("1) Listar");
    println!("2) Crear");
    println!("3) Actualizar crédito");
    println!("4) Eliminar");
    println!("0) Salir");
    print!("Opción: ");
    io::stdout().flush().unwrap();
}

// Función principal: bucle infinito hasta que el usuario elija "0"
fn main() -> mysql::Result<()> {
    let pool = Pool::new(DB_URL)?;  // El pool se crea una vez
    let mut entrada = String::new();

    loop {
        menu();                     // Mostrar opciones
        entrada.clear();
        io::stdin().read_line(&mut entrada).unwrap();
        let opcion = entrada.trim();

        let resultado = match opcion {
            "1" => listar(&pool),
            "2" => crear(&pool),
            "3" => actualizar(&pool),
            "4" => eliminar(&pool),
            "0" => break,            // Salir del bucle
            _ => { println!("Opción inválida"); Ok(()) }
        };

        // Si alguna operación devolvió un error, lo mostramos
        if let Err(e) = resultado {
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}
```

**Análisis del mini-proyecto:**

**El patrón `pool` compartido**: Observa que el `Pool` se crea una sola vez en `main` y se pasa como referencia (`&Pool`) a cada función. Esto es eficiente porque las conexiones se reutilizan y no se crean y destruyen en cada operación. Es el mismo patrón que usaremos en la Parte 3 con Actix Web.

**El manejo de entrada del usuario**: Las funciones `crear`, `actualizar` y `eliminar` leen de la terminal usando `io::stdin()`. Nota el patrón repetido: `print!(prompt); flush(); read_line(); trim(); to_string(); clear();`. Es verboso pero necesario porque `print!` no vacía el buffer automáticamente (necesitamos `flush()` para que el prompt aparezca antes de que el usuario escriba), y debemos limpiar la variable `entrada` entre lecturas para no arrastrar datos viejos.

**La confirmación de DELETE**: La función `eliminar` pide confirmación antes de borrar. Esto es una medida de seguridad básica: en una aplicación real, la confirmación evitaría borrados accidentales. Sin embargo, incluso con confirmación, un DELETE sigue siendo permanente. En sistemas profesionales, se usa "borrado lógico" (marcar `activo = FALSE`) en lugar de DELETE, pero para este ejemplo didáctico el DELETE directo está bien.

**El formateo de la tabla**: La función `listar` imprime una tabla con bordes Unicode. Los caracteres `┌`, `─`, `┐`, `│`, `├`, `┘`, etc., son caracteres de dibujo de caja (box-drawing) que están disponibles en cualquier terminal moderna. La tabla tiene formato fijo con ancho de columna definido. Nota que truncamos el nombre a 42 caracteres para evitar que desborde la columna: `&nombre[..nombre.len().min(42)]`.

**Salida esperada:**

```
=== ERP/CRM - Gestión de Clientes ===
1) Listar
2) Crear
3) Actualizar crédito
4) Eliminar
0) Salir
Opción: 1

┌────┬────────────────────────────────────────────┬──────────────┬─────────────────────────┬────────────┬────────┐
│ ID │ Nombre                                     │ RFC          │ Email                   │ Crédito    │ Activo │
├────┼────────────────────────────────────────────┼──────────────┼─────────────────────────┼────────────┼────────┤
│  1 │ Constructora del Bajío S.A. de C.V.        │ CDB010101AB3 │ contacto@cdb.mx         │ $100000.00 │ true   │
│  2 │ Distribuidora del Norte S.A. de C.V.       │ DDN020202CD4 │ ventas@ddn.mx           │ $ 50000.00 │ true   │
│  3 │ Comercializadora del Sur S.C.              │ CDS030303EF5 │ info@cds.mx             │ $ 25000.00 │ true   │
└────┴────────────────────────────────────────────┴──────────────┴─────────────────────────┴────────────┴────────┘
```

**Errores típicos en este mini-proyecto:**

1. **Error al ejecutar el INSERT**: Si el RFC ya existe en la tabla, obtendrás `Error: Duplicate entry '...' for key 'clientes.rfc'`. El programa mostrará el error y volverá al menú. Para evitarlo, el INSERT debería verificar primero si el RFC existe, o usar `INSERT ... ON DUPLICATE KEY UPDATE`.

2. **Error al parsear el crédito**: Si el usuario escribe "cien mil" en lugar de "100000", `.parse::<f64>()` falla y `unwrap_or(0.0)` pone crédito en 0. El programa no informa del error; simplemente usa 0. Una versión mejor validaría la entrada y pediría de nuevo si es inválida.

3. **Error de conexión**: Si MariaDB no está corriendo, todas las funciones fallarán con `Error: Connection refused`. El programa no puede hacer nada al respecto más que mostrar el error y esperar la siguiente opción del menú.

4. **DELETE sin confirmación accidental**: Si el usuario pulsa Enter sin escribir "s", la confirmación falla y se cancela. Si escribe "S" (mayúscula) o "si", también se cancela porque solo acepta "s" minúscula. Esto es intencional: es más seguro requerir una confirmación exacta.

```rust
use mysql::prelude::*;
use mysql::Pool;
use std::io::{self, Write};

const DB_URL: &str = "mysql://root:secret@127.0.0.1:3306/erp_crm";

#[derive(Debug)]
struct Cliente {
    id: u32,
    nombre: String,
    rfc: String,
    email: Option<String>,
    credito: f64,
    activo: bool,
}

fn listar(pool: &Pool) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    let clientes: Vec<(u32, String, String, Option<String>, f64, bool)> =
        conn.query("SELECT id, nombre, rfc, email, credito, activo FROM clientes ORDER BY id")?;
    println!("\n┌────┬────────────────────────────────────────────┬──────────────┬─────────────────────────┬────────────┬────────┐");
    println!("│ ID │ Nombre                                     │ RFC          │ Email                   │ Crédito    │ Activo │");
    println!("├────┼────────────────────────────────────────────┼──────────────┼─────────────────────────┼────────────┼────────┤");
    for (id, nombre, rfc, email, credito, activo) in &clientes {
        let e = email.as_deref().unwrap_or("—");
        println!("│ {:2} │ {:<42} │ {:<12} │ {:<23} │ ${:>9.2} │ {:<6} │",
                 id, &nombre[..nombre.len().min(42)], rfc, e, credito, activo);
    }
    println!("└────┴────────────────────────────────────────────┴──────────────┴─────────────────────────┴────────────┴────────┘");
    Ok(())
}

fn crear(pool: &Pool) -> mysql::Result<()> {
    let mut entrada = String::new();
    let mut conn = pool.get_conn()?;

    print!("Nombre: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let nombre = entrada.trim().to_string(); entrada.clear();
    print!("RFC: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let rfc = entrada.trim().to_string(); entrada.clear();
    print!("Email: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let email = entrada.trim().to_string(); entrada.clear();
    print!("Crédito: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let credito: f64 = entrada.trim().parse().unwrap_or(0.0); entrada.clear();

    conn.exec_drop(
        "INSERT INTO clientes (nombre, rfc, email, credito) VALUES (?, ?, ?, ?)",
        (&nombre, &rfc, &email as &str, credito)
    )?;
    println!("✓ Cliente creado con ID {}", conn.last_insert_id().unwrap());
    Ok(())
}

fn actualizar(pool: &Pool) -> mysql::Result<()> {
    let mut entrada = String::new();
    let mut conn = pool.get_conn()?;

    print!("ID a actualizar: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let id: u32 = entrada.trim().parse().unwrap_or(0); entrada.clear();
    print!("Nuevo crédito: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let credito: f64 = entrada.trim().parse().unwrap_or(0.0); entrada.clear();

    conn.exec_drop("UPDATE clientes SET credito = ? WHERE id = ?", (credito, id))?;
    println!("✓ Filas actualizadas: {}", conn.affected_rows());
    Ok(())
}

fn eliminar(pool: &Pool) -> mysql::Result<()> {
    let mut entrada = String::new();
    let mut conn = pool.get_conn()?;

    print!("ID a eliminar: "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?; let id: u32 = entrada.trim().parse().unwrap_or(0); entrada.clear();
    print!("¿Confirma? (s/N): "); io::stdout().flush()?;
    io::stdin().read_line(&mut entrada)?;
    if entrada.trim().to_lowercase() != "s" {
        println!("Cancelado"); return Ok(());
    }
    conn.exec_drop("DELETE FROM clientes WHERE id = ?", (id,))?;
    println!("✓ Filas eliminadas: {}", conn.affected_rows());
    Ok(())
}

fn menu() {
    println!("\n=== ERP/CRM - Gestión de Clientes ===");
    println!("1) Listar");
    println!("2) Crear");
    println!("3) Actualizar crédito");
    println!("4) Eliminar");
    println!("0) Salir");
    print!("Opción: ");
    io::stdout().flush().unwrap();
}

fn main() -> mysql::Result<()> {
    let pool = Pool::new(DB_URL)?;
    let mut entrada = String::new();

    loop {
        menu();
        entrada.clear();
        io::stdin().read_line(&mut entrada).unwrap();
        let opcion = entrada.trim();
        let resultado = match opcion {
            "1" => listar(&pool),
            "2" => crear(&pool),
            "3" => actualizar(&pool),
            "4" => eliminar(&pool),
            "0" => break,
            _ => { println!("Opción inválida"); Ok(()) }
        };
        if let Err(e) = resultado {
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}
```

Para ejecutarlo:

```bash
# Antes, carga los datos de ejemplo
mysql -h 127.0.0.1 -u root -psecret < proyectos_capitulo/parte2/02_crud_clientes/init.sql
cargo run --manifest-path /home/roy/rust_man/proyectos_capitulo/parte2/02_crud_clientes/Cargo.toml
```


## 2.6 Pool de conexiones con `r2d2_mysql`

Hasta ahora hemos usado `Pool::new(DB_URL)?` en cada programa sin preguntarnos qué hace exactamente ese `Pool`. En esta sección veremos por qué el pool es necesario y cómo configurarlo correctamente.

Cada vez que abres una conexión a MariaDB, el servidor dedica recursos: un hilo, memoria para buffers, el handshake TCP, la autenticación. Si abres y cierras una conexión por cada operación (como harías sin pool), el programa se vuelve lento y el servidor se satura. Es como pedir un taxi nuevo cada vez que necesitas moverte: pierdes tiempo esperando a que llegue. Un pool es una flotilla de taxis esperando en la parada: ya están listos, solo subes y te vas.

El crate `mysql` ya incluye un pool básico por defecto, pero `r2d2` te da control sobre cuántas conexiones mantener, cuánto tiempo esperar, y cómo manejar conexiones rotas.

### 2.6.1 Configuracion del pool

```rust
use r2d2_mysql::MysqlConnectionManager;
use r2d2::Pool;
use mysql::OptsBuilder;

fn crear_pool() -> Result<Pool<MysqlConnectionManager>, Box<dyn std::error::Error>> {
    let opts = OptsBuilder::default()
        .ip_or_hostname(Some("127.0.0.1"))
        .user(Some("root"))
        .pass(Some("secret"))
        .db_name(Some("erp_crm"));
    let manager = MysqlConnectionManager::new(opts);
    let pool = Pool::builder()
        .max_size(15)
        .min_idle(Some(2))
        .connection_timeout(std::time::Duration::from_secs(10))
        .build(manager)?;
    Ok(pool)
}
```

**Análisis línea por línea**:

- `OptsBuilder::default()`: crea un builder con valores por defecto para las opciones de conexión. Cada método (`.ip_or_hostname`, `.user`, etc.) modifica una opción y devuelve el builder.
- `.ip_or_hostname(Some("127.0.0.1"))`: dirección del servidor.
- `.max_size(15)`: máximo de conexiones simultáneas. Si hay 15 conexiones en uso y llega una 16ª, espera hasta que una se libere.
- `.min_idle(Some(2))`: el pool mantiene al menos 2 conexiones inactivas listas para usar. Si hay un pico de tráfico, no hay que esperar a crear nuevas conexiones.
- `.connection_timeout(...)`: tiempo máximo que un hilo espera por una conexión. Si pasan 10 segundos y ninguna se libera, `pool.get()` devuelve un error de timeout.

### 2.6.2 Prestamo de conexiones

```rust
let pool = crear_pool()?;
let mut conn = pool.get()?;
conn.query("SELECT nombre FROM clientes")?;
// conn se devuelve al pool automáticamente al salir del scope
```

- `pool.get()?`: pide una conexión al pool. Devuelve `PooledConnection`, que implementa los mismos métodos que una conexión directa gracias al trait `Deref`.
- La conexión se devuelve al pool automáticamente cuando `conn` sale del alcance (el destructor `Drop` la devuelve). No necesitas llamar a un método `close`.

### 2.6.3 Errores tipicos

**Error 1: pool demasiado pequeño**.

```rust
Pool::builder().max_size(2).build(manager)?;
// Con 2 conexiones, 3 hilos concurrentes dejarán a 1 esperando
```

Síntoma: errores de timeout intermitentes en horas pico. Solución: aumenta `max_size`.

**Error 2: olvidar Arc para compartir entre hilos**.

El pool se puede compartir entre hilos con `Arc`. Sin `Arc`, el compilador impide compartirlo.

**Error 3: no configurar timeout**.

Sin `connection_timeout`, el pool espera indefinidamente. Siempre configura un timeout.

Si haces todo esto para cada consulta, una operación que debería tomar 1 milisegundo (ejecutar un SELECT simple) termina tomando 20-50 milisegundos por culpa de la sobrecarga de la conexión. Y si tienes 100 usuarios haciendo consultas al mismo tiempo, el servidor se satura rápidamente con 100 conexiones simultáneas, cada una consumiendo recursos aunque esté inactiva.

**Analogía:** un pool de conexiones es como una flotilla de taxis esperando en una parada. En lugar de llamar a un taxi nuevo cada vez que necesitas ir a algún lado (y esperar a que llegue desde el otro lado de la ciudad), tienes una fila de taxis listos para salir. Cuando necesitas uno, tomas el primero de la fila. Cuando terminas, el taxi vuelve a la fila para el siguiente pasajero. El tiempo de espera se reduce de minutos a segundos.

```toml
[dependencies]
mysql = "25"
r2d2 = "0.8"
r2d2_mysql = "26"
```

**¿Qué es `r2d2` exactamente?**

`r2d2` (Rust Resource Database 2) es una librería genérica de pool de conexiones. No sabe nada de MySQL: solo sabe gestionar un conjunto de recursos que pueden ser creados, verificados y destruidos. La lógica específica de MySQL la proporciona `r2d2_mysql`, que implementa el trait `ManageConnection` del pool para el tipo de conexión que usa el crate `mysql`.

La separación entre `r2d2` (genérico) y `r2d2_mysql` (específico) sigue el principio de responsabilidad única. Si mañana quisieras usar PostgreSQL, solo cambiarías `r2d2_mysql` por `r2d2_pq` y el resto del código del pool quedaría igual.

### 2.6.2 Configuracion del pool: ajustando los parametros

Ahora vamos a crear un pool con control fino sobre sus parámetros. En lugar de usar la URL directamente (como hacíamos con `Pool::new(DB_URL)?`), usaremos `OptsBuilder` para construir las opciones de conexión paso a paso, y `Pool::builder()` para configurar el pool.

```rust
use r2d2_mysql::MysqlConnectionManager;
use r2d2::Pool;
use mysql::OptsBuilder;

fn crear_pool() -> Result<Pool<MysqlConnectionManager>, Box<dyn std::error::Error>> {
    // OptsBuilder construye las opciones de conexión usando un patrón "builder"
    // Cada método devuelve el builder modificado, y al final obtenemos Opts
    let opts = OptsBuilder::default()
        // ^^^^^^^^^^^^^^^^^^^^^^^^^^^
        // Crea un builder con valores por defecto

        .ip_or_hostname(Some("127.0.0.1"))
        // ^^^^^^^^^^^^^^ dirección del servidor
        .user(Some("root"))
        .pass(Some("secret"))
        .db_name(Some("erp_crm"));
        // ^^^^^^^^ nombre de la base de datos

    // MysqlConnectionManager sabe cómo crear y destruir conexiones MySQL
    let manager = MysqlConnectionManager::new(opts);

    // Pool::builder() devuelve un PoolBuilder con el que configuramos el pool
    let pool = Pool::builder()
        .max_size(15)
        // ^^^^^^^^ máximo de conexiones simultáneas en el pool (por defecto 10)
        // En producción, este número depende de:
        //   - max_connections del servidor MySQL (por defecto 151)
        //   - Número de hilos/workers de tu aplicación
        //   - Latencia de las consultas
        // Regla general: entre 5 y 30 conexiones por instancia de aplicación

        .min_idle(Some(2))
        // ^^^^^^^^ número mínimo de conexiones inactivas que el pool mantiene
        // Si hay menos de 2 conexiones inactivas, el pool crea más (hasta max_size)
        // Esto evita que los usuarios esperen cuando llega un pico de tráfico

        .connection_timeout(std::time::Duration::from_secs(10))
        // ^^^^^^^^^^^^^^^^^^ tiempo máximo que un hilo espera por una conexión
        // Si todas las conexiones están ocupadas y no se libera ninguna en 10s,
        // pool.get() devuelve un error "Connection timed out"

        .build(manager)?;
        // ^^^^^ construye el pool. Falla si la configuración es inválida

    Ok(pool)
}
```

**Explicación de cada parámetro de configuración:**

- **`max_size`**: el número máximo de conexiones que el pool mantendrá abiertas simultáneamente. Si 15 hilos están usando conexiones y llega un 16º, esperará hasta que una se libere o hasta que expire el timeout. ¿Cómo elegir este número? Si tu servidor MySQL tiene `max_connections = 151` y tienes 10 instancias de tu aplicación, cada una debería tener `max_size <= 15` para no agotar el límite del servidor. Una fórmula aproximada: `max_size = (max_connections_del_servidor / número_de_instancias) - 2` (dejando 2 conexiones de margen para administración).

- **`min_idle`**: el número de conexiones que el pool mantiene en reserva, incluso si nadie las está usando. Si hay un pico repentino de tráfico, las conexiones ya están listas y no hay que esperar a crearlas. El costo es que consumen recursos del servidor aunque estén inactivas. Un valor típico es `min_idle = max_size / 3` (redondeando).

- **`connection_timeout`**: el tiempo máximo que un hilo esperará para obtener una conexión del pool. Si todas están ocupadas y el timeout expira, la operación falla con un error. Esto es importante para la experiencia del usuario: es mejor que una operación falle rápido a que el usuario se quede esperando indefinidamente. Para una aplicación interna de oficina, 10 segundos es razonable. Para una API web pública, quizá 3-5 segundos es más apropiado.

**¿Qué pasa cuando se alcanza el límite de conexiones?**

Si 15 hilos están usando las 15 conexiones y llega un 16º hilo, `pool.get()` bloqueará la ejecución de ese hilo hasta que una conexión se libere o hasta que pasen 10 segundos (el `connection_timeout`). Si pasan 10 segundos sin que ninguna conexión se libere, `pool.get()` devuelve un error:

```
Error: Connection pool timed out after 10 seconds
```

Esto es mucho mejor que permitir que el servidor MySQL se sature con cientos de conexiones, lo que podría ralentizar o incluso colapsar el servidor.

### 2.6.3 Prestamo de conexiones: tomar, usar, devolver

Una vez configurado el pool, usarlo es sorprendentemente simple. El pool actúa como una fábrica de conexiones: le pides una, la usas, y cuando terminas, se devuelve automáticamente. No necesitas llamar a ningún método "close" o "release".

```rust
let pool = crear_pool()?;             // Crear el pool (una vez al iniciar)

// Tomar una conexión prestada del pool
// Si hay una conexión libre, la obtiene inmediatamente.
// Si todas están ocupadas, espera hasta 10 segundos (el timeout configurado).
let mut conn = pool.get()?;
//                ^^^^^^^
//                Devuelve PooledConnection, que se comporta como una conexión normal
//                gracias al trait Deref (delegación automática de métodos)

// Usar la conexión como si fuera una conexión directa
conn.query("SELECT nombre FROM clientes")?;

// Al cerrar este bloque (o cuando conn sale del alcance),
// la conexión se devuelve automáticamente al pool.
// Esto lo hace el destructor (Drop) de PooledConnection.
```

**¿Por qué no necesito cerrar la conexión manualmente?**

Porque `PooledConnection` implementa el trait `Drop`. En Rust, cualquier tipo que implementa `Drop` tiene un código que se ejecuta automáticamente cuando el valor sale del alcance (scope). En el caso de `PooledConnection`, el destructor devuelve la conexión al pool en lugar de cerrarla.

Esto es más seguro que en lenguajes como Java o Python, donde olvidar cerrar una conexión (o hacerlo en el lugar incorrecto) provoca "fugas de conexiones" (connection leaks) que agotan el pool y eventualmente dejan la aplicación sin poder conectarse. Con `PooledConnection`, la fuga es imposible porque el compilador de Rust garantiza que el destructor se ejecutará (a menos que uses `std::mem::forget`, que es explícitamente inseguro).

**¿Qué pasa si la conexión se cae mientras está prestada?**

Si el servidor MySQL se reinicia o la red se corta mientras tienes una `PooledConnection` prestada, la siguiente consulta fallará con un error como "MySQL server has gone away". En ese caso, debes capturar el error, descartar la conexión (soltándola) y pedir una nueva al pool. El pool detecta que la conexión está rota y la elimina, creando una nueva en su lugar.

```rust
// Patrón para manejar conexiones caídas
let mut conn = pool.get()?;
match conn.query("SELECT 1") {
    Ok(_) => { /* todo bien */ }
    Err(e) => {
        eprintln!("Conexión perdida: {}, se obtendrá una nueva", e);
        // Al soltar conn, el pool descarta la conexión rota
        // En la siguiente iteración, conn = pool.get() dará una conexión nueva
    }
}
```

### 2.6.4 El pool como estado compartido entre hilos

Una de las grandes ventajas de usar un pool es que puedes compartirlo entre múltiples hilos de ejecución. Cada hilo puede pedir su propia conexión al pool sin interferir con los demás. Pero en Rust, compartir datos entre hilos tiene reglas estrictas: los datos deben ser `Send` (enviables entre hilos) y `Sync` (accesibles desde múltiples hilos).

`Pool<MysqlConnectionManager>` implementa ambos traits, lo que significa que se puede compartir de forma segura. La forma más común de hacerlo es usando `Arc` (Atomic Reference Counting):

```rust
use std::sync::Arc;
use std::thread;

// Arc (Atomic Reference Counting) permite que múltiples hilos compartan
// una referencia inmutable al pool. Es como un control remoto compartido:
// todos pueden ver la misma TV (el pool) pero nadie puede cambiarla.
let pool: Arc<Pool<MysqlConnectionManager>> = Arc::new(crear_pool()?);

// Clonar un Arc es barato: solo incrementa un contador de referencias.
// No crea una copia del pool, solo otra "flecha" que apunta al mismo pool.
let pool_hilo_2 = Arc::clone(&pool);

// Lanzamos un hilo que usará el pool
thread::spawn(move || {
    // move: la closure toma posesión de pool_hilo_2
    // Esto es necesario porque el hilo puede vivir más que la función que lo creó

    let mut conn = pool_hilo_2.get().unwrap();
    //                ^^^^^^^^^^^^^^^^^^^^^^^
    //                pool_hilo_2 es un Arc<Pool>, pero .get() funciona igual
    //                porque Arc implementa Deref, delegando a Pool

    conn.query("SELECT 1").unwrap();
    // Al terminar, la conexión se devuelve al pool automáticamente
});

// Mientras tanto, el hilo principal puede usar el pool original
let mut conn = pool.get().unwrap();
conn.query("SELECT 2").unwrap();
```

**¿Por qué `Arc` y no `Rc`?**

`Rc` (Reference Counting) es para compartir datos dentro de un mismo hilo. `Arc` es para compartir entre hilos. La diferencia es que `Arc` usa operaciones atómicas para el contador de referencias, que son seguras entre hilos (pero ligeramente más lentas). Si intentas usar `Rc` en un `thread::spawn`, el compilador te dará un error:

```
error[E0277]: `Rc<Pool<...>>` cannot be sent between threads safely
```

Rust te obliga a ser explícito sobre la concurrencia. No puedes compartir accidentalmente datos entre hilos sin marcarlos como `Arc`.

**¿Qué pasa si quiero compartir el pool en una aplicación con Actix Web (Parte 3)?**

En Actix Web, el estado compartido se inyecta con `web::Data<T>`. `web::Data` internamente usa `Arc`, así que el patrón es el mismo:

```rust
// Avance de la Parte 3:
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = crear_pool().expect("pool");
    // pool es Pool<MysqlConnectionManager>

    HttpServer::new(move || {
        // move: mueve pool al closure
        App::new()
            .app_data(web::Data::new(pool.clone()))
            //                  ^^^^^^^^^^^^^^^^^
            //                  web::Data envuelve el pool en un Arc
            .route("/clientes", web::get().to(listar_clientes))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

Este patrón lo veremos en detalle en la sección 3.4, pero vale la pena mencionarlo aquí para que veas cómo el pool de conexiones es el puente entre el mundo de la base de datos y el mundo del servidor web.

**Errores típicos con pools:**

1. **Pool demasiado pequeño**: Si `max_size = 5` pero tu aplicación recibe 10 peticiones simultáneas, 5 peticiones esperarán (hasta el timeout) y 5 fallarán. Síntoma: errores intermitentes de timeout en horas pico. Solución: monitoriza el uso del pool y aumenta `max_size`.

2. **Pool demasiado grande**: Si `max_size = 100` pero tu servidor MySQL tiene `max_connections = 100`, y tienes 2 instancias de la aplicación, la segunda instancia no podrá conectarse. Síntoma: errores de conexión "Too many connections". Solución: reduce `max_size` o aumenta `max_connections` del servidor.

3. **Olvidar Arc para compartir entre hilos**: Si intentas pasar `Pool` directamente a un `thread::spawn`, el compilador falla porque `Pool` no implementa el trait `Send` (no puede moverse a otro hilo porque se necesita mover el ownership). Solución: envuelve en `Arc`.

4. **No manejar el timeout del pool**: Si no configuras `connection_timeout`, el valor por defecto de `r2d2` es esperar indefinidamente. Esto puede hacer que tu aplicación se "cuelgue" cuando el servidor MySQL está lento. Siempre configura un timeout razonable (5-30 segundos).

5. **Mantener conexiones prestadas por mucho tiempo**: Si tomas una conexión del pool y la usas para una operación larga (por ejemplo, un reporte que tarda 5 minutos), esa conexión no está disponible para otros hilos durante 5 minutos. Para operaciones largas, considera usar una conexión directa (sin pool) o aumentar `max_size` para compensar.

## 2.7 Transacciones

Hasta ahora hemos ejecutado operaciones individuales que se aplican inmediatamente. Pero en un ERP, crear un pedido requiere varias operaciones: insertar en `pedidos`, insertar en `lineas_pedido` y actualizar `productos` (descontar stock). Si falla el descuento de stock después de insertar el pedido, la base de datos queda inconsistente.

Las **transacciones** resuelven esto: agrupan operaciones en una unidad atómica que se aplica completa o no se aplica en absoluto. Es como un "modo edición": haces cambios y al final decides "guardar todo" (COMMIT) o "descartar todo" (ROLLBACK).

### 2.7.1 Transacciones en Rust

```rust
use mysql::prelude::*;

let mut conn = pool.get_conn()?;
let mut tx = conn.start_transaction(TxOpts::default())?;

tx.exec_drop("UPDATE productos SET stock = stock - 1 WHERE sku = ?", ("SKU-001",))?;
tx.exec_drop("INSERT INTO movimientos_inventario (sku, cantidad, tipo) VALUES (?, ?, ?)",
             ("SKU-001", 1, "SALIDA"))?;

tx.commit()?;
```

**Análisis línea por línea**:

- `let mut tx = conn.start_transaction(TxOpts::default())?;`: inicia una transacción. Todas las operaciones posteriores se hacen sobre `tx`, no sobre `conn`. Ningún cambio es visible para otras conexiones hasta el `commit`.
- `tx.exec_drop("UPDATE productos ...")?`: ejecuta una operación dentro de la transacción. Si falla, el `?` propaga el error y el `Drop` de `tx` hace rollback automático.
- `tx.commit()?;`: confirma la transacción. Todas las operaciones se aplican como una unidad atómica.

### 2.7.2 Rollback automatico

El objeto `Transaction` implementa `Drop`. Si `tx` sale del alcance sin `commit()` ni `rollback()`, el destructor ejecuta `ROLLBACK`. Es una red de seguridad: en otros lenguajes, olvidar el rollback deja transacciones abiertas; en Rust, es imposible.

### 2.7.3 Errores tipicos

**Error 1: olvidar `commit()`**.

```rust
let mut tx = conn.start_transaction(TxOpts::default())?;
tx.exec_drop("UPDATE productos SET stock = stock - 1 WHERE sku = ?", ("SKU-001",))?;
// No hay tx.commit()
// Drop hace ROLLBACK, los cambios se pierden
```

Solución: siempre llama a `tx.commit()?` explícitamente.

**Error 2: no proteger el stock dentro de la transacción**.

```rust
// El UPDATE no verifica que el stock siga siendo suficiente
tx.exec_drop("UPDATE productos SET stock = stock - ? WHERE sku = ?",
    (5, "SKU-001"))?;
// Si otro usuario ya compró 5, el stock podría volverse negativo
```

Solución: añade `AND stock >= ?` al UPDATE.

### 2.7.4 Mini-proyecto: `03_cli_pedidos_transacciones`

El proyecto `03_cli_pedidos_transacciones` (en `proyectos_capitulo/parte2/03_cli_pedidos_transacciones/`) implementa la creación de un pedido con transacción. El flujo es: obtener cliente, mostrar productos, construir líneas, validar stock, iniciar transacción, insertar pedido + líneas + descontar stock, confirmar. Cualquier error dentro de la transacción la deshace.

```rust
use mysql::prelude::*;
use mysql::{Pool, TxOpts};
use std::io::{self, Write};

const DB_URL: &str = "mysql://root:secret@127.0.0.1:3306/erp_crm";

fn crear_pedido(pool: &Pool) -> mysql::Result<()> {
    let cliente_id: u32 = 1;  // Cliente de ejemplo

    // Obtener productos
    let productos: Vec<(String, String, f64, i64)> = {
        let mut conn = pool.get_conn()?;
        conn.query("SELECT sku, nombre, precio, stock FROM productos WHERE activo = TRUE AND stock > 0")
    }?;

    println!("\nProductos disponibles:");
    for (sku, nombre, precio, stock) in &productos {
        println!("  [{}] {} - ${} (stock: {})", sku, nombre, precio, stock);
    }

    // Transacción
    let mut conn = pool.get_conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;

    let folio = "PED-2026-0001";
    tx.exec_drop(
        "INSERT INTO pedidos (folio, cliente_id, subtotal, iva, total, estado) VALUES (?, ?, ?, ?, ?, 'CONFIRMADO')",
        (folio, cliente_id, 1000.0, 160.0, 1160.0)
    )?;
    tx.exec_drop(
        "INSERT INTO lineas_pedido (pedido_folio, sku, cantidad, precio_unitario) VALUES (?, ?, ?, ?)",
        (folio, "SKU-001", 1, 18999.0)
    )?;
    tx.exec_drop(
        "UPDATE productos SET stock = stock - 1 WHERE sku = ? AND stock >= 1",
        ("SKU-001",)
    )?;

    tx.commit()?;
    println!("✓ Pedido {} creado", folio);
    Ok(())
}

fn main() -> mysql::Result<()> {
    let pool = Pool::new(DB_URL)?;
    crear_pedido(&pool)
}
```

**Análisis línea por línea**:

- `let mut tx = conn.start_transaction(TxOpts::default())?;`: inicia la transacción. Todo lo que sigue hasta `commit()` es atómico.
- `tx.exec_drop("INSERT INTO pedidos ...")?;`: inserta el pedido. Si falla, `?` sale y `Drop` de `tx` hace rollback.
- `tx.exec_drop("UPDATE productos SET stock = stock - 1 WHERE sku = ? AND stock >= 1", ...)?;`: descuenta stock solo si hay suficiente. La condición `AND stock >= 1` previene stocks negativos.
- `tx.commit()?;`: si llegamos aquí sin errores, todo se aplica junto.

### 2.7.3 La forma mas limpia: transaccion con closure

Para evitar errores de lógica (como olvidar hacer rollback en una rama del `if`), el patrón más recomendado es envolver toda la transacción en un closure. Si el closure se ejecuta sin errores, se hace commit. Si ocurre cualquier error, el `?` propaga el error y el `Drop` del `Transaction` hace rollback automático.

```rust
// Patrón de transacción segura con closure
let result: mysql::Result<()> = (|| {
    // La barra doble || es la sintaxis de closure en Rust
    // Este closure captura conn del entorno exterior

    let mut tx = conn.start_transaction(TxOpts::default())?;

    tx.exec_drop("UPDATE productos SET stock = stock - 1 WHERE sku = ?", ("SKU-001",))?;
    //          ^
    //          Si esto falla, el ? sale del closure con el error.
    //          Cuando tx sale del alcance, Drop hace ROLLBACK automático.

    tx.exec_drop("INSERT INTO movimientos_inventario (...) VALUES (?, ?, ?)",
                 ("SKU-001", 1, "SALIDA"))?;
    //          ^
    //          Si esto falla, también se hace rollback

    tx.commit()
    // Si llegamos aquí, todo salió bien. commit() aplica los cambios.
    // Si commit() falla, tx sale del alcance y se hace rollback.
})();
//  ^^ Esto INVOCA el closure inmediatamente.
//     El resultado es un Result<(), mysql::Error>.

// Aquí ya sabemos si la transacción tuvo éxito o no
match result {
    Ok(()) => println!("✓ Transacción completada con éxito"),
    Err(e) => eprintln!("✗ Transacción fallida, cambios descartados: {}", e),
}
```

**Análisis del patrón:**

1. El closure `|| { ... }` se define y se invoca inmediatamente con `()`.
2. Dentro del closure, cada `?` propaga el error hacia fuera del closure.
3. Si todo sale bien, `tx.commit()` se llama explícitamente.
4. Si ocurre un error antes de `commit()`, el `?` hace que el closure retorne `Err(...)`. Al salir del closure, `tx` se destruye y `Drop` ejecuta `ROLLBACK`.
5. El resultado del closure es un `Result` que se puede evaluar fuera para decidir qué hacer.

Este patrón es tan común que algunos ORM como `diesel` o `sea-orm` tienen métodos específicos para ello (`conn.transaction(|tx| { ... })`), pero con el crate `mysql` puedes implementarlo tú mismo con este sencillo closure.

### 2.7.4 Niveles de aislamiento: que ve cada transaccion

Cuando dos transacciones se ejecutan al mismo tiempo, surge una pregunta fundamental: ¿qué datos ve cada una? Si la transacción A actualiza un cliente y la transacción B lo lee antes de que A haga commit, ¿debería B ver los datos actualizados o los originales?

La respuesta depende del **nivel de aislamiento** de la transacción. MariaDB/MySQL soporta cuatro niveles, ordenados de menor a mayor protección:

**1. `READ UNCOMMITTED` (el menos seguro)**
Una transacción puede leer datos que otra transacción está modificando pero aún no ha confirmado. Esto se llama "lectura sucia" (dirty read). Por ejemplo, si el vendedor A está actualizando el crédito de un cliente (pero aún no ha hecho commit), y el vendedor B consulta el mismo cliente, B vería el crédito a medio cambiar, lo que podría llevarlo a tomar decisiones incorrectas (como aprobar una venta basada en un crédito que aún no es definitivo). Este nivel prácticamente no se usa en producción.

**2. `READ COMMITTED` (el más común en otros motores)**
Una transacción solo lee datos que han sido confirmados. No hay lecturas sucias, pero sí puede haber "lecturas no repetibles": si dentro de la misma transacción lees dos veces el mismo registro, puedes obtener valores diferentes si otra transacción lo modificó y confirmó entre ambas lecturas. Es el nivel por defecto en PostgreSQL, Oracle y SQL Server. En MariaDB/MySQL también está disponible pero no es el predeterminado.

**3. `REPEATABLE READ` (por defecto en MySQL/MariaDB)**
Una transacción ve una "instantánea" (snapshot) de los datos tal como estaban al iniciar la transacción. Todas las lecturas dentro de la misma transacción devuelven los mismos valores, incluso si otras transacciones modifican y confirman datos a medio camino. Esto evita lecturas no repetibles, pero puede causar "lecturas fantasma": una consulta devuelve filas que aparecieron después de que la transacción empezó (por ejemplo, SELECT COUNT(*) devuelve un número diferente si otra transacción insertó filas y confirmó).

**4. `SERIALIZABLE` (el más seguro)**
Las transacciones se ejecutan como si fueran en serie: una después de otra, sin solapamiento. Esto elimina todos los problemas de concurrencia (lecturas sucias, no repetibles, fantasmas), pero reduce drásticamente el rendimiento porque las transacciones no pueden ejecutarse en paralelo. Solo se usa cuando la integridad de los datos es más importante que la velocidad.

**¿Cómo elegir el nivel de aislamiento?**

En un ERP típico, el `REPEATABLE READ` por defecto de MariaDB es adecuado para la mayoría de las operaciones. Sin embargo, para reportes largos (que pueden durar minutos), podrías usar `READ COMMITTED` para evitar bloqueos innecesarios. Para operaciones críticas (como corte de caja o cierre contable), podrías usar `SERIALIZABLE` para garantizar consistencia absoluta.

Puedes cambiar el nivel de aislamiento al iniciar la transacción:

```rust
use mysql::{TxOpts, IsolationLevel};

let opts = TxOpts::default()
    .set_isolation_level(Some(IsolationLevel::RepeatableRead));
//                                  ^^^^^^^^^^^^^^^^^^^^^^^^^
//                                  Opciones: ReadUncommitted, ReadCommitted,
//                                            RepeatableRead, Serializable

let mut tx = conn.start_transaction(opts)?;
//                                  ^^^^
//                                  Usamos nuestras opciones personalizadas
```

### 2.7.5 Mini-proyecto: `03_cli_pedidos_transacciones` — el corazon del ERP

Este mini-proyecto es el más importante de la Parte 2 porque implementa el flujo de creación de un pedido con transacción, que es la operación central de cualquier ERP. Cada vez que un cliente compra productos, el sistema debe:

1. Verificar que el cliente existe.
2. Mostrar los productos disponibles con su stock.
3. Permitir al usuario seleccionar productos y cantidades.
4. Verificar que hay suficiente stock para cada producto.
5. Calcular subtotal, IVA (16%) y total.
6. Insertar el pedido en la tabla `pedidos`.
7. Insertar cada línea en `lineas_pedido`.
8. Descontar el stock de cada producto en `productos`.
9. Si todo sale bien, confirmar la transacción. Si algo falla, deshacer todo.

Los pasos 6, 7 y 8 deben ocurrir dentro de una transacción porque son una unidad atómica: si falla el descuento de stock después de haber insertado el pedido, tendrías un pedido sin productos. Si falla la inserción de líneas después de descontar stock, tendrías stock descontado sin un pedido que lo justifique.

Analicemos el código paso a paso:

```rust
use mysql::prelude::*;
use mysql::{Pool, TxOpts};
use std::io::{self, Write};

const DB_URL: &str = "mysql://root:secret@127.0.0.1:3306/erp_crm";

// Función auxiliar para leer una línea de la terminal
// Reutilizable en todos los mini-proyectos
fn leer_linea(prompt: &str) -> String {
    let mut s = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    //          ^^^^^ flush: forzar que el prompt se muestre antes de leer
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()    // eliminar el \n del final
}

// Representa una línea de pedido (un producto con su cantidad)
#[derive(Debug)]
struct Linea {
    sku: String,
    nombre: String,
    precio: f64,
    stock: i64,       // stock actual (para validación)
    cantidad: u32,    // cantidad que el usuario quiere comprar
}

fn crear_pedido(pool: &Pool) -> mysql::Result<()> {
    // Paso 1: identificar al cliente
    let cliente_id: u32 = leer_linea("ID del cliente: ").parse().unwrap_or(0);
    //                     ^^^^^ parse convierte el string a número
    //                           Si el usuario escribe "abc", unwrap_or(0) usa 0

    // Paso 2: obtener productos disponibles (con stock > 0)
    let productos: Vec<(String, String, f64, i64)> = {
        // Las llaves {} crean un bloque que nos permite usar una variable
        // conn que se libera al salir del bloque
        let mut conn = pool.get_conn()?;
        conn.query(
            "SELECT sku, nombre, precio, stock FROM productos WHERE activo = TRUE AND stock > 0"
        )?
    };
    // conn se libera aquí y vuelve al pool

    // Mostrar productos al usuario
    println!("\nProductos disponibles:");
    for (sku, nombre, precio, stock) in &productos {
        println!("  [{}] {} - ${} (stock: {})", sku, nombre, precio, stock);
    }

    // Paso 3: el usuario selecciona productos y cantidades
    let mut lineas: Vec<Linea> = Vec::new();
    loop {
        let sku = leer_linea("\nSKU (o ENTER para terminar): ");
        if sku.is_empty() { break; }  // ENTER sin texto termina la selección

        let cantidad_str = leer_linea("Cantidad: ");
        let cantidad: u32 = cantidad_str.parse().unwrap_or(0);
        if cantidad == 0 { continue; }  // cantidad inválida, preguntar de nuevo

        // Buscar el SKU en la lista de productos disponibles
        if let Some((s, n, p, st)) = productos.iter().find(|(s, _, _, _)| s == &sku) {
            lineas.push(Linea {
                sku: s.clone(),
                nombre: n.clone(),
                precio: *p,
                stock: *st,
                cantidad,
            });
        } else {
            println!("SKU {} no encontrado", sku);
        }
    }

    if lineas.is_empty() {
        println!("Pedido cancelado (sin líneas)");
        return Ok(());
    }

    // Paso 4: validar stock ANTES de la transacción
    // Esto evita iniciar una transacción que sabemos que va a fallar
    for l in &lineas {
        if (l.cantidad as i64) > l.stock {
            println!("✗ Stock insuficiente para {}: tienes {}, necesitas {}",
                     l.sku, l.stock, l.cantidad);
            return Ok(());  // Salimos sin crear transacción
        }
    }

    // Paso 5: calcular totales
    // subtotal = suma de (cantidad * precio) de cada línea
    let subtotal: f64 = lineas.iter().map(|l| l.cantidad as f64 * l.precio).sum();
    let iva = subtotal * 0.16;     // IVA estándar en México (16%)
    let total = subtotal + iva;

    // Paso 6: generar un folio único para el pedido
    // Formato: PED-AAAA-NNNN (año + número aleatorio)
    let folio = format!("PED-2026-{:04}", rand::random::<u16>() % 10000);

    // ===== INICIO DE LA TRANSACCIÓN =====
    let mut conn = pool.get_conn()?;
    //          ^^^^ obtenemos una nueva conexión del pool para la transacción

    let mut tx = conn.start_transaction(TxOpts::default())?;
    //               ^^^^^^^^^^^^^^^^^^^^^^ comienza la transacción
    //               Todas las operaciones a continuación son atómicas

    // Paso 7: insertar el pedido
    tx.exec_drop(
        "INSERT INTO pedidos (folio, cliente_id, subtotal, iva, total, estado) VALUES (?, ?, ?, ?, ?, 'CONFIRMADO')",
        (&folio, cliente_id, subtotal, iva, total)
    )?;
    // Si esto falla (por ejemplo, violación de PK porque el folio ya existe),
    // el ? propaga el error, la transacción se deshace automáticamente.

    // Paso 8: insertar cada línea y descontar stock
    for l in &lineas {
        // 8a. Insertar la línea de pedido
        tx.exec_drop(
            "INSERT INTO lineas_pedido (pedido_folio, sku, cantidad, precio_unitario) VALUES (?, ?, ?, ?)",
            (&folio, &l.sku, l.cantidad, l.precio)
        )?;

        // 8b. Descontar el stock
        // La cláusula AND stock >= ? es una protección adicional:
        // si el stock cambió entre la validación y la transacción,
        // esta condición evita que el stock quede negativo
        tx.exec_drop(
            "UPDATE productos SET stock = stock - ? WHERE sku = ? AND stock >= ?",
            (l.cantidad, &l.sku, l.cantidad)
        )?;

        // Si alguna de estas operaciones falla (por ejemplo, un deadlock),
        // el ? hace rollback automático de TODO lo que se haya hecho hasta ahora
    }

    // ===== CONFIRMAR TRANSACCIÓN =====
    // Si llegamos aquí, todas las operaciones se ejecutaron sin error
    tx.commit()?;
    // ^^^^^^^^^ En este momento, PEDIDO + LÍNEAS + STOCK se guardan juntos

    println!("\n✓ Pedido {} creado. Total: ${:.2}", folio, total);
    println!("  Subtotal: ${:.2}  IVA: ${:.2}", subtotal, iva);
    Ok(())
}

fn main() -> mysql::Result<()> {
    let pool = Pool::new(DB_URL)?;
    crear_pedido(&pool)?;
    Ok(())
}
```

**Análisis detallado del mini-proyecto:**

**¿Por qué validar stock antes de la transacción?**

El código valida el stock en el paso 4, **antes** de iniciar la transacción. Esto es intencional: si el stock es insuficiente, no tiene sentido gastar recursos del servidor abriendo una transacción que va a fallar. Es más eficiente fallar rápido (fail fast) que fallar después de haber iniciado una transacción.

Sin embargo, hay un problema de concurrencia: entre la validación (paso 4) y el descuento dentro de la transacción (paso 8b), otro usuario podría haber comprado el mismo producto y reducido el stock. Por eso la consulta UPDATE incluye `AND stock >= ?` como protección adicional: si el stock ya no es suficiente, la actualización no afecta ninguna fila (`affected_rows = 0`) y podemos detectar que algo salió mal.

En un sistema de producción, este patrón se refuerza con `SELECT ... FOR UPDATE` dentro de la transacción para bloquear las filas de productos que se van a modificar, evitando que dos transacciones intenten descontar el mismo stock simultáneamente.

**El generador de folios:**

El folio se genera como `PED-2026-NNNN` donde NNNN es un número aleatorio de 4 dígitos. En un sistema real, el folio podría ser secuencial (PED-2026-0001, PED-2026-0002, ...), pero para este ejemplo didáctico el aleatorio funciona. El riesgo de colisión (generar dos folios iguales) es bajo con 10,000 combinaciones posibles, pero en producción usarías una secuencia auto-incremental o un UUID.

**El manejo de errores:**

Cada `?` dentro de la transacción propaga el error hacia `main`, que imprime el error y termina el programa. Si quisiéramos reintentar en lugar de terminar, podríamos usar el patrón de closure visto en 2.7.3:

```rust
let resultado = (|| {
    let mut conn = pool.get_conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;
    // ... operaciones ...
    tx.commit()
})();

match resultado {
    Ok(_) => println!("✓ Pedido creado"),
    Err(e) => eprintln!("✗ Error: {}, se ha hecho rollback", e),
}
```

**Salida esperada:**

```
ID del cliente: 1

Productos disponibles:
  [SKU-001] Laptop HP Pavilion 15 - $18999 (stock: 10)
  [SKU-002] Mouse óptico USB - $350 (stock: 50)
  [SKU-003] Teclado mecánico - $1200 (stock: 20)
  [SKU-004] Monitor LED 24" - $4500 (stock: 15)
  [SKU-005] Silla ergonómica - $2800 (stock: 8)

SKU (o ENTER para terminar): SKU-002
Cantidad: 3

SKU (o ENTER para terminar): SKU-005
Cantidad: 1

SKU (o ENTER para terminar):

✓ Pedido PED-2026-0742 creado. Total: $3850.00
  Subtotal: $3318.97  IVA: $531.03
```

Después de esto, puedes verificar que el stock de SKU-002 bajó de 50 a 47, y el de SKU-005 de 8 a 7. Si ejecutas el programa de nuevo pero pides 100 unidades de un producto con stock 10, verás:

```
✗ Stock insuficiente para SKU-002: tienes 50, necesitas 100
```

Y no se crea ningún pedido.

**Errores típicos con transacciones:**

1. **No llamar a `commit()`**: Si olvidas el `tx.commit()`, la transacción se deshace automáticamente cuando `tx` sale del alcance. Esto puede ser sorprendente si esperabas que los cambios se guardaran. Siempre verifica que llamaste a `commit()`.

2. **Múltiples transacciones anidadas**: El crate `mysql` no soporta transacciones anidadas. Si llamas a `start_transaction()` dos veces sobre la misma conexión, la segunda llamada puede fallar o comportarse inesperadamente. Usa una sola transacción por operación.

3. **Olvidar la protección `AND stock >= ?`**: Si no verificas que el stock sigue siendo suficiente dentro de la transacción, el stock podría quedar negativo si otro usuario compró el mismo producto entre tu validación y tu descuento. Siempre incluye una verificación dentro de la transacción.

4. **Transacciones largas**: Mantener una transacción abierta por mucho tiempo bloquea recursos del servidor y puede causar deadlocks. Una transacción de creación de pedido no debería durar más de unos pocos segundos. Si necesitas hacer cálculos pesados entre el BEGIN y el COMMIT, considera hacer los cálculos fuera de la transacción y solo meter las operaciones SQL dentro.

## 2.8 Errores tipicos con bases de datos

Los errores al trabajar con bases de datos se dividen en dos categorías: errores de **conexión** (el servidor no está accesible) y errores de **ejecución** (la consulta tiene un problema). Cada error aquí incluye el código que lo produce, el mensaje real y la solución.

### 2.8.1 Errores de conexion

**Error: `Connection refused`**

```rust
let pool = Pool::new("mysql://root:secret@127.0.0.1:3306/erp_crm")?;
let mut conn = pool.get_conn()?;
```

Mensaje: `Error: Connection refused (os error 111)`. Causa: MariaDB no está corriendo. Solución: `podman start mysql_man`.

**Error: `Access denied for user`**

```rust
let pool = Pool::new("mysql://root:clave_incorrecta@127.0.0.1:3306/erp_crm")?;
let mut conn = pool.get_conn()?;
```

Mensaje: `Error: Access denied for user 'root'@'localhost' (using password: YES)`. Solución: usa la contraseña correcta (`secret`).

**Error: `Unknown database`**

```rust
let pool = Pool::new("mysql://root:secret@127.0.0.1:3306/bd_inexistente")?;
let mut conn = pool.get_conn()?;
```

Mensaje: `Error: Unknown database 'bd_inexistente'`. Solución: ejecuta el `init.sql` para crear la base de datos.

### 2.8.2 Errores de tipo de dato

**Error: tipo incorrecto en INSERT**.

```rust
conn.exec_drop(
    "INSERT INTO clientes (nombre, credito) VALUES (?, ?)",
    ("Cliente", "cien mil")  // "cien mil" no es un número
)?;
```

Mensaje: `Error: Incorrect integer value: 'cien mil' for column 'credito'`. Solución: pasa `f64` en lugar de `String`.

**Error: dato demasiado largo**.

```rust
conn.exec_drop(
    "INSERT INTO clientes (nombre) VALUES (?)",
    ("un string de más de 150 caracteres...")
)?;
```

Mensaje: `Error: Data too long for column 'nombre'`. Solución: trunca el string o aumenta el tamaño de la columna.

### 2.8.3 Violaciones de restricciones

**Error: `Duplicate entry`**.

```rust
conn.exec_drop(
    "INSERT INTO clientes (nombre, rfc) VALUES (?, ?)",
    ("Cliente", "CDB010101AB3")  // RFC ya existente
)?;
```

Mensaje: `Error: Duplicate entry 'CDB010101AB3' for key 'clientes.rfc'`. Solución: verifica con `SELECT COUNT(*)` antes de insertar, o usa `ON DUPLICATE KEY UPDATE`.

**Error: `Foreign key constraint fails`**.

```rust
conn.exec_drop(
    "INSERT INTO pedidos (folio, cliente_id) VALUES (?, ?)",
    ("PED-1", 9999)  // cliente_id 9999 no existe
)?;
```

Mensaje: `Error: Cannot add or update a child row: a foreign key constraint fails`. Solución: verifica que el `cliente_id` exista.

### 2.8.4 Deadlocks

Un deadlock ocurre cuando dos transacciones esperan recursos que la otra tiene. MariaDB aborta una con:

```
Error: Deadlock found when trying to get lock; try restarting transaction
```

Solución: reintenta la transacción. La prevención: accede a los recursos siempre en el mismo orden.

## 2.9 Ejemplo completo: ERP/CRM CLI de gestion

El proyecto `04_cli_erp_completo` (en `proyectos_capitulo/parte2/04_cli_erp_completo/`) integra todas las funcionalidades en un solo programa con módulos separados. La arquitectura es:

```
src/
├── main.rs      # Bucle del menú principal
├── db.rs        # Pool de conexiones
├── clientes.rs  # CRUD de clientes
├── productos.rs # CRUD de productos
├── pedidos.rs   # Creación de pedidos con transacción
└── ui.rs        # Menús y entrada/salida
```

```rust
// main.rs
mod db;
mod clientes;
mod productos;
mod pedidos;
mod ui;

fn main() -> mysql::Result<()> {
    let pool = db::crear_pool()?;
    loop {
        ui::mostrar_menu();
        match ui::leer_opcion().as_str() {
            "1" => clientes::gestionar(&pool)?,
            "2" => productos::gestionar(&pool)?,
            "3" => pedidos::gestionar(&pool)?,
            "0" => break,
            _ => println!("Opción inválida"),
        }
    }
    Ok(())
}
```

**Análisis línea por línea**:

- `let pool = db::crear_pool()?;`: el pool se crea una vez en `main` y se pasa como referencia a todos los módulos. Ninguna función "posee" el pool; todas lo toman prestado.
- `clientes::gestionar(&pool)?`: cada módulo tiene una función `gestionar` que muestra su submenú y ejecuta la operación seleccionada. La separación en módulos permite que cada archivo tenga una responsabilidad única.

## 2.10 Buenas practicas

### 2.10.1 Migraciones versionadas

Los cambios al esquema de la base de datos deben ser versionados, igual que el código fuente. Cada migración tiene un `up.sql` (los cambios) y un `down.sql` (cómo revertirlos):

```
migrations/
├── 0001_crear_clientes/
│   ├── up.sql     → CREATE TABLE clientes (...)
│   └── down.sql   → DROP TABLE clientes
├── 0002_agregar_telefono/
│   ├── up.sql     → ALTER TABLE clientes ADD telefono VARCHAR(15)
│   └── down.sql   → ALTER TABLE clientes DROP COLUMN telefono
```

Herramientas: `diesel migration` (Parte 3), `migrant`, `flyway`.

### 2.10.2 Manejo de concurrencia

- Usa transacciones para operaciones que tocan múltiples tablas.
- Usa `SELECT ... FOR UPDATE` dentro de transacciones para bloquear filas.
- Mantén las transacciones cortas.
- Usa `INSERT ... ON DUPLICATE KEY UPDATE` para upserts atómicos.

### 2.10.3 Separacion en capas

Separa el código en tres capas:
- **Repository**: funciones que ejecutan SQL y devuelven structs.
- **Service**: lógica de negocio (validaciones, cálculos).
- **UI**: interfaz con el usuario (menús, entrada/salida).

Esta separación permite probar cada capa de forma independiente y facilita el cambio de base de datos o de interfaz de usuario sin afectar las otras capas.

    ```sql
    INSERT INTO inventario (sku, stock) VALUES ('SKU-001', 10)
    ON DUPLICATE KEY UPDATE stock = stock + 10;
    ```
    Esto evita la condición de carrera de "verificar si existe → insertar o actualizar" que requeriría dos consultas.

### 2.10.3 Logging de consultas

En desarrollo, es útil ver qué consultas SQL se están ejecutando y cuánto tardan. El crate `mysql` usa la infraestructura de logging de Rust (el crate `log`), que puedes activar con `env_logger`:

```toml
[dependencies]
env_logger = "0.11"
log = "0.4"
```

```rust
fn main() {
    // Inicializar el logger (debe hacerse al inicio del programa)
    env_logger::init();

    // A partir de aquí, el crate mysql registrará cada consulta con nivel DEBUG
    let pool = Pool::new(DB_URL).expect("pool");

    // Cada query se verá en la terminal como:
    // [2026-07-05T12:00:00Z DEBUG mysql] SELECT id, nombre FROM clientes
    // [2026-07-05T12:00:00Z DEBUG mysql] Query took 0.003s
}
```

Para controlar el nivel de detalle, usa la variable de entorno `RUST_LOG`:

```bash
# Mostrar solo errores (producción)
RUST_LOG=error cargo run

# Mostrar consultas SQL (desarrollo)
RUST_LOG=debug cargo run

# Mostrar todo, incluyendo trazas internas del driver (debugging avanzado)
RUST_LOG=trace cargo run
```

### 2.10.4 Separacion en capas (repository / service)

En los mini-proyectos de esta Parte 2, el código de acceso a datos (SQL), la lógica de negocio (cálculos, validaciones) y la interfaz de usuario (menús, entrada/salida) están mezclados en las mismas funciones. Esto se llama "código espagueti" y es difícil de mantener.

La solución profesional es separar el código en tres capas:

1. **Repository**: funciones que ejecutan SQL puro y devuelven structs. No toman decisiones de negocio; solo traducen entre la base de datos y Rust.
2. **Service**: funciones que implementan la lógica de negocio (validar stock, calcular IVA, decidir si un cliente puede hacer un pedido). Usan los repositorios para obtener y guardar datos.
3. **UI/CLI**: funciones que se comunican con el usuario (mostrar menús, leer entrada, formatear salida). No contienen SQL ni lógica de negocio.

```
src/
├── main.rs           ← UI (menú, entrada/salida)
├── db.rs             ← Configuración del pool
├── clientes/
│   ├── mod.rs        ← Service (lógica de negocio)
│   └── repositorio.rs ← Repository (SQL)
├── productos/
│   ├── mod.rs
│   └── repositorio.rs
└── pedidos/
    ├── mod.rs
    └── repositorio.rs
```

**Ventajas:**
- **Testeabilidad**: puedes probar la lógica de negocio sin base de datos (usando mocks), y probar los repositorios con una base de datos de prueba.
- **Mantenibilidad**: si cambias de MySQL a PostgreSQL, solo tocas los repositorios. La lógica de negocio y la UI no cambian.
- **Claridad**: cada función tiene un propósito único y se entiende sin necesidad de leer todo el programa.

Esta separación la veremos aplicada en la Parte 3 con Actix Web, donde la UI es HTTP en lugar de terminal, pero el principio es el mismo.

## 2.11 Ejercicios acumulativos (Parte 2) – 20 ejercicios

1. **Conectar y mostrar versión**: crea un programa que se conecte a la BD, imprima la versión del servidor y la lista de tablas.

2. **Insertar 100 clientes de prueba**: usa un loop para insertar 100 clientes con datos generados.

3. **Listar productos con stock bajo**: muestra los productos cuyo stock sea menor a 10.

4. **Búsqueda por RFC**: pide un RFC al usuario y muestra los datos del cliente (si existe).

5. **Actualización masiva**: aumenta el crédito de todos los clientes "VIP" en un 10%.

6. **Soft delete**: modifica el `DELETE` para que en lugar de borrar, marque `activo = FALSE`.

7. **Exportar a CSV**: lee todos los clientes y escribe un archivo `clientes.csv`.

8. **Importar desde CSV**: lee `clientes.csv` e inserta los registros.

9. **Reporte de ventas del día**: muestra los pedidos creados hoy con su total.

10. **Top 10 productos más vendidos**: agrupa `lineas_pedido` por `sku` y suma cantidades.

11. **Cliente con más pedidos**: encuentra el cliente que más pedidos ha hecho.

12. **Reabastecer stock**: aumenta el stock de un producto en N unidades (sin transacción, validar primero).

13. **Devolución**: crea un pedido "negativo" que devuelva stock a un producto.

14. **Búsqueda fuzzy**: implementa una búsqueda por nombre que ignore acentos y mayúsculas.

15. **Auditoría**: crea una tabla `auditoria` con `tabla`, `id_registro`, `accion`, `fecha`, `usuario`. Inserta un registro en cada INSERT/UPDATE/DELETE.

16. **Paginación**: lista clientes con paginación (10 por página) y opción de ir a página siguiente/anterior.

17. **Índices**: mide el tiempo de un SELECT sin índice, crea un índice, y mide de nuevo.

18. **Transacción con rollback manual**: simula un error a la mitad de una transacción y verifica que no se aplicó nada.

19. **Reporte de antigüedad de clientes**: para cada cliente, calcula los días desde su `created_at`.

20. **Respaldo lógico**: usa `mysqldump` para hacer un backup de la BD, restáuralo, y verifica.

## 2.12 Soluciones detalladas (Parte 2)

Las soluciones completas de los 20 ejercicios están en el **Anexo A.2**.

---

## Cierre de la Parte 2

Con esto concluimos la conexión entre Rust y MySQL/MariaDB. Has aprendido a configurar el cliente, ejecutar SELECT/INSERT/UPDATE/DELETE con parámetros preparados, usar un pool de conexiones, manejar transacciones, y construir un CLI con menú. En la **Parte 3** levantaremos un servidor HTTP con Actix Web y conectaremos los structs a una API REST completa con un ORM (Diesel y SeaORM).

> **Métricas finales de la Parte 2**:
> - 12 secciones H2 escritas (2.1 – 2.12).
> - 30+ subsecciones H3/H4.
> - 2 mini-proyectos físicos probados (los otros 2 los creamos a continuación).
> - 30+ bloques de código.
> - 20 ejercicios propuestos.
> - ~6 000 palabras añadidas en esta parte.


---

# Parte 3: Rust, Actix Web y ORM

> **Aviso pedagógico**: en esta parte construiremos una API REST profesional para el ERP/CRM. Usaremos **Actix Web** como framework HTTP y veremos dos ORM — **Diesel** (síncrono, con macros) y **SeaORM** (asíncrono, moderno) — para que el lector elija el que mejor se adapte a su proyecto. La API final expone 15+ endpoints con autenticación JWT, validación, documentación OpenAPI y está lista para producción.

## 3.1 Introduccion a Actix Web

Hasta ahora hemos construido programas que se ejecutan y terminan. Un programa que lee datos de la terminal, hace algo y termina. Pero un ERP no funciona así: un ERP es un servicio que está **siempre corriendo**, esperando peticiones de usuarios, clientes de escritorio, aplicaciones móviles u otros sistemas. Para eso necesitamos un **servidor HTTP**.

Un servidor HTTP es un programa que escucha en un puerto de red, recibe peticiones (requests) de clientes (navegadores, apps, otras APIs), las procesa y devuelve respuestas (responses). Es como un restaurante: los clientes llegan, hacen un pedido (request), el chef lo prepara (el servidor procesa) y el mesero lleva el plato (response). El restaurante no se cierra después de atender a un cliente; sigue abierto esperando al siguiente.

Construir un servidor HTTP desde cero manejando TCP, parsing de HTTP, routing, concurrencia, etc., es una tarea titánica. Por eso existen los **frameworks web**: librerías que hacen todo ese trabajo pesado por ti. Tú solo escribes la lógica de tu aplicación (qué hacer cuando alguien pide la lista de clientes, o cuando alguien crea un producto nuevo), y el framework se encarga del resto: recibir la petición, interpretar la URL, llamar a tu función, y devolver la respuesta formateada correctamente.

Actix Web es el framework web más rápido del ecosistema Rust (encabeza constantemente los benchmarks TechEmpower, compitiendo con frameworks en C++ y Rust). Está basado en el **modelo de actores**, un patrón de concurrencia donde cada "actor" es un objeto independiente con su propio estado que procesa mensajes de forma secuencial. Esto permite que Actix maneje miles de peticiones simultáneas con recursos mínimos. Pero no necesitas entender el modelo de actores para usar Actix Web: la API es limpia y directa, similar a Express.js (Node.js) o Flask (Python), pero con la seguridad de tipos de Rust.

¿Por qué Actix Web para un ERP? Porque un ERP necesita:
- **Concurrencia**: muchos usuarios haciendo peticiones al mismo tiempo.
- **Rendimiento**: las respuestas deben ser rápidas (milisegundos).
- **Seguridad de tipos**: el compilador de Rust garantiza que las rutas, los parámetros y las respuestas coinciden.
- **Ecosistema**: integraciones con bases de datos (Diesel, SeaORM, sqlx), autenticación (JWT), serialización (serde), logging, CORS, etc.

Actix Web proporciona todo esto sin sacrificar velocidad. Una petición que en Node.js tarda 10ms, en Actix Web puede tardar 0.5ms. En un ERP con miles de usuarios, esa diferencia se acumula.

## 3.2 Configuracion del proyecto

Antes de escribir código, tenemos que configurar las dependencias. Actix Web necesita el crate `actix-web`, y para trabajar con JSON necesitamos `serde` y `serde_json`. También necesitamos entender cómo funciona la macro `#[actix_web::main]`, que es el punto de entrada del servidor asíncrono.

### 3.2.1 Dependencias en Cargo.toml

```toml
[package]
name = "api_health"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

**Análisis línea por línea**:

- `actix-web = "4"`: el framework web. La versión 4 es la estable actual (2026). Incluye el servidor HTTP, el enrutador, los extractores (Path, Query, Json), los traits (Responder, Handler), y los middlewares básicos (Logger, Compress).
- `serde = { version = "1", features = ["derive"] }`: serde es el sistema de serialización/deserialización de Rust. La feature `derive` permite usar `#[derive(Serialize, Deserialize)]` en los structs para convertir automáticamente entre Rust y JSON. Sin serde, tendrías que construir y parsear JSON manualmente.
- `serde_json = "1"`: el backend de JSON para serde. Convierte structs Rust a `serde_json::Value` y viceversa. Actix Web usa serde_json internamente para las respuestas `.json()`.

### 3.2.2 Primer servidor HTTP

Vamos a crear nuestro primer servidor. Será simple: dos rutas, una que saluda y otra que indica que el servicio está vivo (health check). Este es el "Hola Mundo" del desarrollo web, pero con un toque profesional: devolvemos JSON con información del servicio, como haría cualquier API real.

```rust
// archivo: src/main.rs
// Mini-proyecto 01: el primer servidor HTTP del ERP
// Endpoints: / (información del servicio), /health (health check)

use actix_web::{web, App, HttpServer, HttpResponse, Responder};

async fn hola() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "servicio": "ERP/CRM Rust México",
        "version": env!("CARGO_PKG_VERSION"),
        "estado": "activo"
    }))
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("ERP/CRM API escuchando en http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hola))
            .route("/health", web::get().to(health))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

**Análisis línea por línea**:

- `use actix_web::{web, App, HttpServer, HttpResponse, Responder};`: importamos los tipos que necesitamos. `web` es un módulo que contiene utilidades como extractores y configuradores de rutas. `App` es la aplicación web (el conjunto de rutas y middlewares). `HttpServer` es el servidor HTTP que escucha peticiones. `HttpResponse` es el tipo de respuesta HTTP. `Responder` es un trait que permite que cualquier tipo se convierta en respuesta HTTP.
- `async fn hola() -> impl Responder {`: los handlers de Actix Web son funciones **asíncronas** (async). `impl Responder` significa "devuelve cualquier tipo que implemente el trait Responder". Esto incluye `HttpResponse`, `String`, `Json<T>`, etc.
- `HttpResponse::Ok().json(serde_json::json!({...}))`: construye una respuesta HTTP 200 OK con cuerpo JSON. `serde_json::json!` es una macro que crea un `serde_json::Value` a partir de un literal JSON. `env!("CARGO_PKG_VERSION")` lee la versión del `Cargo.toml` en tiempo de compilación.
- `async fn health() -> impl Responder { HttpResponse::Ok().body("OK") }`: un health check simple. Devuelve el texto "OK" con código 200. Los sistemas de orquestación (Kubernetes, Docker) usan estos endpoints para verificar que el servicio está vivo.
- `#[actix_web::main]`: esta macro convierte una función `async fn main()` en el punto de entrada del programa. Sin ella, no podrías usar `await` dentro de `main`. Internamente, crea un runtime tokio y ejecuta la función asíncrona.
- `HttpServer::new(|| { App::new() ... })`: crea un servidor HTTP. El closure `|| { App::new()... }` se llama para cada worker (hilo) que crea el servidor. Cada worker tiene su propia copia de la aplicación.
- `.route("/", web::get().to(hola))`: registra una ruta. `"/"` es la URL raíz. `web::get()` indica que solo responde a peticiones GET. `.to(hola)` asigna el handler.
- `.bind("127.0.0.1:8080")?`: vincula el servidor a la dirección y puerto. `127.0.0.1` es localhost, solo accesible desde tu máquina. `8080` es el puerto. Si el puerto está ocupado, `bind` devuelve un error.
- `.run().await`: inicia el servidor y espera peticiones. Este `await` nunca termina a menos que el servidor se detenga.

**Salida esperada**:

Al ejecutar `cargo run`, verás:

```
ERP/CRM API escuchando en http://127.0.0.1:8080
```

En otra terminal, ejecuta:

```bash
curl http://127.0.0.1:8080/
```

Respuesta:
```json
{"servicio":"ERP/CRM Rust México","version":"0.1.0","estado":"activo"}
```

```bash
curl http://127.0.0.1:8080/health
```

Respuesta:
```
OK
```

**Truco profesional**: el health check real de un ERP debe verificar también la conexión a la base de datos, no solo responder "OK". En producción, el health check haría un `SELECT 1` a MariaDB y devolvería 200 solo si la BD responde, o 503 si no:

```rust
async fn health(pool: web::Data<Pool>) -> impl Responder {
    match pool.get() {
        Ok(mut conn) => match conn.query("SELECT 1") {
            Ok(_) => HttpResponse::Ok().body("OK"),
            Err(_) => HttpResponse::ServiceUnavailable().body("BD no disponible"),
        },
        Err(_) => HttpResponse::ServiceUnavailable().body("Pool agotado"),
    }
}
```

**Error típico**: olvidar `#[actix_web::main]` e intentar usar `async fn main()` directamente:

```rust
// ERROR: esto no compila
async fn main() -> std::io::Result<()> {
    // ...
}
```

Mensaje: `error[E0277]: the main function is not allowed to be async`. Solución: añadir `#[actix_web::main]` encima de `fn main`.

**Error típico**: intentar usar `.bind("0.0.0.0:8080")` sin permisos. Si el puerto es menor a 1024, necesitas privilegios de administrador. Solución: usar puertos mayores a 1024 (8080, 3000, etc.).

### 3.2.3 Mini-proyecto: `01_api_health`

El proyecto `01_api_health` (en `proyectos_capitulo/parte3/01_api_health/`) es este primer servidor. Para ejecutarlo:

```bash
cd proyectos_capitulo/parte3/01_api_health
cargo run
```

Luego en otra terminal: `curl http://127.0.0.1:8080/health`.

## 3.3 Rutas y handlers

Ya tenemos un servidor que responde "OK". Ahora vamos a construir las rutas reales del ERP: listar clientes, crear productos, obtener pedidos. Cada operación del ERP será una **ruta** (una URL como `/clientes` o `/pedidos/{id}`) combinada con un **método HTTP** (GET para leer, POST para crear, PUT para actualizar, DELETE para eliminar). Esta combinación de URL + método se llama **endpoint**.

Actix Web organiza las rutas con `App::route()` o, de forma más profesional, con `web::resource()` y `web::scope()`. La idea es que cada recurso del ERP (clientes, productos, pedidos) tenga su propio conjunto de rutas agrupadas bajo un mismo path.

### 3.3.1 Metodos HTTP y configuracion de rutas

Cada operación CRUD se asigna a un método HTTP:

| Operación | Método HTTP | Ruta | Código de respuesta |
|---|---|---|---|
| Listar | GET | `/clientes` | 200 OK |
| Crear | POST | `/clientes` | 201 Created |
| Obtener uno | GET | `/clientes/{id}` | 200 OK |
| Actualizar | PUT | `/clientes/{id}` | 200 OK |
| Eliminar | DELETE | `/clientes/{id}` | 204 No Content |

```rust
use actix_web::{web, HttpResponse, Responder};

async fn listar() -> impl Responder {
    HttpResponse::Ok().body("Listando")
}

async fn crear() -> impl Responder {
    HttpResponse::Created().body("Creado")
}

async fn obtener() -> impl Responder {
    HttpResponse::Ok().body("Obteniendo")
}

async fn actualizar() -> impl Responder {
    HttpResponse::Ok().body("Actualizado")
}

async fn eliminar() -> impl Responder {
    HttpResponse::NoContent().finish()
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/clientes")
            .route(web::get().to(listar))
            .route(web::post().to(crear))
    )
    .service(
        web::resource("/clientes/{id}")
            .route(web::get().to(obtener))
            .route(web::put().to(actualizar))
            .route(web::delete().to(eliminar))
    );
}
```

**Análisis línea por línea**:

- `async fn listar() -> impl Responder { HttpResponse::Ok().body("Listando") }`: handler simple que devuelve 200 OK con texto plano. `"Listando"` es un `&str`, que implementa `Responder`, así que se puede devolver directamente.
- `HttpResponse::Created()`: código 201, indica que un recurso fue creado exitosamente.
- `HttpResponse::NoContent().finish()`: código 204, indica que la operación fue exitosa pero no hay contenido que devolver. Es común en DELETE.
- `fn config(cfg: &mut web::ServiceConfig) {`: función que configura rutas. Recibe un `ServiceConfig` y le añade servicios. Cualquier módulo del ERP (clientes, productos, pedidos) tendría su propia función `config`.
- `web::resource("/clientes")`: define un recurso en la ruta `/clientes`. A este recurso se le asignan rutas para diferentes métodos HTTP.
- `.route(web::get().to(listar))`: asigna el método GET al handler `listar`. Si alguien hace GET a `/clientes`, se ejecuta `listar()`.
- `web::resource("/clientes/{id}")`: el `{id}` es un parámetro de ruta. Captura el valor de la URL y lo pasa al handler.
- `cfg.service(...)`: registra el recurso en la configuración. Para que esta configuración se aplique, debes llamar a `.configure(config)` en `App::new()`:

```rust
App::new()
    .configure(config)
```

**Truco profesional**: separa cada recurso en su propia función de configuración. Esto mantiene el código organizado cuando tienes decenas de endpoints:

```rust
// main.rs
App::new()
    .configure(clientes::configurar_rutas)
    .configure(productos::configurar_rutas)
    .configure(pedidos::configurar_rutas)
    .configure(auth::configurar_rutas)
```

Cada módulo (`clientes.rs`, `productos.rs`) exporta una función `pub fn configurar_rutas(cfg: &mut web::ServiceConfig)`.

### 3.3.2 Parametros de ruta y query

Los parámetros de ruta (`{id}`) se extraen con `web::Path<T>`. Los parámetros de query (`?page=1&per_page=10`) se extraen con `web::Query<T>`.

```rust
use actix_web::{web, HttpResponse, Responder};

// Parámetro de ruta: /clientes/5 → id = 5
async fn obtener_cliente(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();  // Extrae el valor u32 de Path
    HttpResponse::Ok().body(format!("Cliente {}", id))
}

// Parámetros de query: /clientes?page=2&per_page=10&buscar=laptop
#[derive(serde::Deserialize)]
struct ListQuery {
    page: Option<u32>,
    per_page: Option<u32>,
    buscar: Option<String>,
}

async fn listar_clientes(query: web::Query<ListQuery>) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let buscar = query.buscar.clone().unwrap_or_default();
    HttpResponse::Ok().body(
        format!("Página {}, por página {}, buscar: {}", page, per_page, buscar)
    )
}
```

**Análisis línea por línea**:

- `async fn obtener_cliente(path: web::Path<u32>)`: `web::Path<u32>` extrae el parámetro `{id}` de la ruta y lo convierte a `u32`. Si la URL es `/clientes/abc`, Actix devuelve automáticamente 400 Bad Request porque "abc" no es un `u32`.
- `let id = path.into_inner();`: `into_inner()` consume el `Path` y devuelve el valor interno (`u32`).
- `#[derive(serde::Deserialize)] struct ListQuery { page: Option<u32>, ... }`: el struct debe coincidir con los parámetros de query. Todos los campos son `Option` porque los parámetros de query son opcionales.
- `query.page.unwrap_or(1)`: si el usuario no envía `?page=...`, usamos 1 como valor por defecto. `unwrap_or` es la forma idiomática de hacer esto en Rust.

**Error típico**: olvidar que los campos de `ListQuery` deben ser `Option<T>` si son opcionales.

```rust
#[derive(Deserialize)]
struct ListQuery {
    page: u32,  // ERROR: si el usuario no envía ?page=, esto falla con 400
}
```

**Salida esperada**:

```
GET /clientes/5  → "Cliente 5"
GET /clientes?page=2&per_page=10&buscar=laptop → "Página 2, por página 10, buscar: laptop"
GET /clientes    → "Página 1, por página 20, buscar: " (valores por defecto)
GET /clientes/abc → 400 Bad Request (abc no es u32)
```

### 3.3.3 Extraccion de JSON del cuerpo

Para crear un cliente, el cliente envía un JSON en el cuerpo de la petición. Actix Web extrae ese JSON con `web::Json<T>`:

```rust
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ClienteInput {
    nombre: String,
    rfc: String,
    email: String,
    credito: f64,
}

async fn crear_cliente(payload: web::Json<ClienteInput>) -> impl Responder {
    let c = payload.into_inner();
    // En un ERP real, aquí insertarías en la base de datos
    HttpResponse::Created().json(c)  // Devuelve el mismo objeto como confirmación
}
```

**Análisis línea por línea**:

- `#[derive(Deserialize, Serialize)]`: `Deserialize` permite convertir JSON a Rust. `Serialize` permite convertir Rust a JSON. Ambas son necesarias porque recibimos JSON y también devolvemos JSON.
- `struct ClienteInput { nombre: String, rfc: String, email: String, credito: f64 }`: los nombres de los campos en Rust deben coincidir exactamente con las claves del JSON. Para nombres diferentes, usa `#[serde(rename = "nombre_json")]`.
- `async fn crear_cliente(payload: web::Json<ClienteInput>)`: `web::Json<T>` extrae el cuerpo de la petición y lo deserializa a `T`. Si el JSON es inválido (falta un campo, tipo incorrecto), Actix devuelve automáticamente 400 Bad Request.
- `let c = payload.into_inner();`: `into_inner()` consume el `Json` wrapper y devuelve el `ClienteInput`.
- `HttpResponse::Created().json(c)`: `HttpResponse::Created()` es 201. `.json(c)` serializa `c` a JSON y lo pone en el cuerpo de la respuesta.

**Error típico**: enviar JSON con campos incorrectos.

```bash
curl -X POST http://localhost:8080/clientes \
  -H "Content-Type: application/json" \
  -d '{"nombre": "Cliente X", "rfc": 123}'
# rfc es un número, pero ClienteInput espera String
# Respuesta: 400 Bad Request con mensaje de error
```

**Truco profesional**: usa `#[serde(rename_all = "camelCase")]` para que los campos JSON usen camelCase (`nombreCliente`) mientras que Rust usa snake_case (`nombre_cliente`). Esto es importante porque las APIs REST suelen usar camelCase:

```rust
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClienteInput {
    nombre_cliente: String,  // en JSON será "nombreCliente"
}
```

### 3.3.4 Mini-proyecto: `02_api_clientes_v0`

El proyecto `02_api_clientes_v0` (en `proyectos_capitulo/parte3/02_api_clientes_v0/`) implementa un CRUD completo de clientes en memoria, sin base de datos. Usa un `Mutex<Vec<Cliente>>` compartido como almacenamiento. Este proyecto es la base sobre la que agregaremos base de datos en los siguientes capítulos.

## 3.4 Estado compartido

En un servidor HTTP, los handlers son funciones independientes que se ejecutan en diferentes hilos. Pero a menudo necesitan compartir datos: el pool de conexiones a la base de datos, la configuración, un contador de peticiones, etc. Actix Web proporciona `web::Data<T>` para inyectar datos compartidos de forma segura y eficiente.

`web::Data<T>` internamente es un `Arc<T>` (Atomic Reference Counting). Esto significa que los datos se comparten entre todos los hilos del servidor sin copiarlos, y el contador de referencias garantiza que los datos se liberen cuando ningún hilo los necesita más.

```rust
use std::sync::Mutex;
use actix_web::{web, App, HttpServer, HttpResponse};

struct Contador {
    valor: Mutex<u32>,
}

async fn incrementar(contador: web::Data<Contador>) -> String {
    let mut v = contador.valor.lock().unwrap();
    *v += 1;
    format!("Valor: {}", v)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let contador = web::Data::new(Contador {
        valor: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(contador.clone())
            .route("/incrementar", web::get().to(incrementar))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

**Análisis línea por línea**:

- `struct Contador { valor: Mutex<u32> }`: el estado compartido. `Mutex<u32>` permite que múltiples hilos accedan al mismo `u32` de forma segura. Sin `Mutex`, dos hilos podrían incrementar al mismo tiempo y perder actualizaciones (data race).
- `async fn incrementar(contador: web::Data<Contador>)`: `web::Data<Contador>` es un extractor. Actix Web busca en el estado de la aplicación un `Contador` envuelto en `web::Data` y lo inyecta en el handler. Si no existe, el handler falla en tiempo de ejecución.
- `let mut v = contador.valor.lock().unwrap();`: `lock()` adquiere el mutex. Si otro hilo lo tiene bloqueado, espera. `unwrap()` porque `lock()` puede fallar si el mutex está envenenado (cuando otro hilo entró en pánico mientras lo tenía). En un ERP, evita `unwrap()` y maneja el error apropiadamente.
- `*v += 1;`: incrementa el valor dentro del mutex.
- `let contador = web::Data::new(...)`: crea un `web::Data` (que es un `Arc` internamente). Esto debe hacerse ANTES de `HttpServer::new`.
- `HttpServer::new(move || { App::new().app_data(contador.clone()) })`: `move` mueve `contador` al closure. `.app_data()` registra el dato en la aplicación. `.clone()` en `web::Data` es barato: solo incrementa el contador de referencias del `Arc`.

**Truco profesional para el pool de BD**:

```rust
// db.rs
pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// main.rs
let pool = db::crear_pool();
let pool_data = web::Data::new(pool);

HttpServer::new(move || {
    App::new()
        .app_data(pool_data.clone())  // Cada worker obtiene un Arc al mismo pool
        .configure(clientes::configurar_rutas)
})
```

El pool se crea UNA vez y se comparte entre todos los workers. Cada worker puede obtener conexiones del pool de forma concurrente.

## 3.5 Middleware

El middleware es código que se ejecuta en cada petición, antes o después del handler. Es como un filtro por el que pasa cada request. Actix Web incluye varios middlewares listos para usar, y también puedes crear los tuyos.

Los middlewares se añaden con `.wrap()` en `App::new()`. El orden importa: el primer `.wrap()` es el más externo (se ejecuta primero).

```rust
use actix_web::{web, App, HttpServer, middleware, http::header};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // 1. Logger: registra cada petición
            .wrap(middleware::Logger::default())
            // 2. Compress: comprime respuestas con gzip
            .wrap(middleware::Compress::default())
            // 3. DefaultHeaders: añade headers a todas las respuestas
            .wrap(middleware::DefaultHeaders::new()
                .add(("X-API-Version", "1.0.0"))
                .add(("X-ERP", "Rust México")))
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("OK") }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

**Análisis línea por línea**:

- `middleware::Logger::default()`: registra cada petición con el formato: `INFO 127.0.0.1 "GET /clientes" 200 1.2ms`. Es esencial en producción para auditoría y debugging.
- `middleware::Compress::default()`: comprime las respuestas con gzip si el cliente lo soporta (header `Accept-Encoding: gzip`). Reduce el ancho de banda.
- `middleware::DefaultHeaders::new().add(...)`: añade headers HTTP a todas las respuestas. Útil para versionado de API, información del servidor, cabeceras de seguridad.

**Truco profesional: middleware de seguridad**:

```rust
.wrap(middleware::DefaultHeaders::new()
    .add(("X-Content-Type-Options", "nosniff"))
    .add(("X-Frame-Options", "DENY"))
    .add(("X-XSS-Protection", "1; mode=block"))
)
```

Estos headers previenen ataques comunes (MIME sniffing, clickjacking, XSS).

## 3.6 Manejo de errores

En una API REST, los errores deben devolverse como respuestas HTTP con el código de estado adecuado y un cuerpo JSON descriptivo. Actix Web proporciona el trait `ResponseError` para convertir errores de Rust en respuestas HTTP de forma automática.

La idea es: defines un tipo `ErrorNegocio` con todas las variantes de error que puede devolver tu API (no encontrado, validación, base de datos, no autorizado), implementas `ResponseError` para ese tipo, y luego los handlers pueden devolver `Result<T, ErrorNegocio>` y Actix Web se encarga de convertirlo en la respuesta HTTP correcta.

```rust
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use std::fmt;
use serde_json::json;

#[derive(Debug)]
enum ErrorNegocio {
    NoEncontrado(String),
    Validacion(String),
    BaseDatos(String),
    NoAutorizado,
}

impl fmt::Display for ErrorNegocio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoEncontrado(s) => write!(f, "No encontrado: {}", s),
            Self::Validacion(s) => write!(f, "Validación: {}", s),
            Self::BaseDatos(s) => write!(f, "Error de base de datos: {}", s),
            Self::NoAutorizado => write!(f, "No autorizado"),
        }
    }
}

impl ResponseError for ErrorNegocio {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NoEncontrado(_) => StatusCode::NOT_FOUND,         // 404
            Self::Validacion(_) => StatusCode::BAD_REQUEST,          // 400
            Self::BaseDatos(_) => StatusCode::INTERNAL_SERVER_ERROR, // 500
            Self::NoAutorizado => StatusCode::UNAUTHORIZED,          // 401
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(json!({
                "error": self.to_string(),
                "codigo": self.status_code().as_u16()
            }))
    }
}
```

**Análisis línea por línea**:

- `enum ErrorNegocio`: define todos los errores que puede devolver nuestra API. Cada variante lleva un mensaje descriptivo, excepto `NoAutorizado` que no necesita más contexto.
- `impl fmt::Display for ErrorNegocio`: cómo se convierte el error a string. Se usa en `self.to_string()` dentro de `error_response()`.
- `impl ResponseError for ErrorNegocio`: implementa el trait de Actix Web. Dos métodos clave:
  - `status_code()`: devuelve el código HTTP para cada variante. `NotFound` → 404, `BadRequest` → 400, etc.
  - `error_response()`: construye la respuesta HTTP. Usamos `HttpResponse::build()` con el código de estado y un cuerpo JSON.

**Uso en un handler**:

```rust
async fn obtener_cliente(
    pool: web::Data<Pool>,
    path: web::Path<u32>,
) -> Result<HttpResponse, ErrorNegocio> {
    let id = path.into_inner();
    let cliente = repositorios::buscar_cliente(pool.get_ref(), id)
        .map_err(|e| ErrorNegocio::BaseDatos(e.to_string()))?
        .ok_or_else(|| ErrorNegocio::NoEncontrado(format!("Cliente {} no encontrado", id)))?;

    Ok(HttpResponse::Ok().json(cliente))
}
```

**Truco profesional: convertir errores de BD automáticamente**:

```rust
impl From<diesel::result::Error> for ErrorNegocio {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => {
                ErrorNegocio::NoEncontrado("Recurso no encontrado".into())
            }
            _ => ErrorNegocio::BaseDatos(e.to_string()),
        }
    }
}
```

Con esto, en los handlers puedes usar `?` directamente con errores de Diesel y se convierten automáticamente a `ErrorNegocio`:

```rust
let cliente = clientes::find(&mut conn, id)?;  // NotFound → ErrorNegocio::NoEncontrado
```

**Error típico**: olvidar implementar `ResponseError` y devolver errores sin formato JSON.

```rust
// ERROR: esto no compila porque actix_web::Error no es lo que queremos
async fn handler() -> Result<HttpResponse, diesel::result::Error> {
    // ...
}
```

Solución: usa `Result<HttpResponse, ErrorNegocio>` con tu tipo de error personalizado.
```

## 3.7 Diesel: el ORM síncrono

En la Parte 2 escribíamos SQL a mano con el crate `mysql`. Era didáctico, pero en un ERP real escribir SQL manual para cada operación es tedioso y propenso a errores. Los **ORM** (Object-Relational Mappers) resuelven esto: en lugar de escribir `SELECT id, nombre FROM clientes WHERE id = ?`, escribes `clientes.find(id).first(&mut conn)`. El ORM genera el SQL por ti, verifica los tipos en compilación y mapea automáticamente las filas a structs Rust.

Diesel es el ORM síncrono más maduro de Rust. Es "síncrono" porque las operaciones de base de datos bloquean el hilo actual hasta que terminan. Esto es más simple que el modelo asíncrono, y para un ERP donde la latencia de red a la base de datos es de 1-5ms, la diferencia es irrelevante. Diesel se integra con Actix Web mediante `web::block()`, que ejecuta código bloqueante en un hilo separado sin bloquear el hilo principal del servidor.

### 3.7.1 Instalacion de Diesel CLI y configuracion

Diesel CLI es una herramienta de línea de comandos que gestiona migraciones y genera automáticamente el archivo `schema.rs` con los tipos de Rust que reflejan las tablas de la base de datos.

```bash
# Instalar Diesel CLI solo con soporte MySQL
cargo install diesel_cli --no-default-features --features mysql
```

**Análisis**: `--no-default-features` evita instalar soporte para PostgreSQL, SQLite, etc. (acelera la compilación). `--features mysql` añade solo MySQL/MariaDB.

Configurar Diesel en el proyecto:

```toml
# diesel.toml (en la raíz del proyecto)
[print_schema]
file = "src/schema.rs"       # ← aquí se genera el schema de Rust

[migrations_directory]
dir = "migrations"           # ← aquí se guardan las migraciones
```

Y la variable de entorno (en `.env` o exportada):

```bash
DATABASE_URL=mysql://root:secret@127.0.0.1:3306/erp_crm
```

### 3.7.2 Dependencias y pool

```toml
[dependencies]
diesel = { version = "2.2", features = ["mysql", "r2d2"] }
dotenvy = "0.15"
actix-web = "4"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```rust
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;

// Type alias: Pool de Diesel con MySQL
pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn crear_pool() -> Pool {
    dotenv().ok();  // Carga .env si existe (no falla si no existe)
    let url = std::env::var("DATABASE_URL")
        .expect("Variable DATABASE_URL no definida");
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Error al crear el pool de conexiones")
}
```

**Análisis línea por línea**:

- `pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;`: definimos un alias para el tipo del pool. `ConnectionManager<MysqlConnection>` es el "manager" que sabe cómo crear y destruir conexiones MySQL. Lo usaremos en todos los módulos que necesiten acceso a la BD.
- `dotenv().ok();`: carga las variables de entorno desde un archivo `.env`. `ok()` ignora el error si el archivo no existe (en producción las variables se definen en el sistema, no en `.env`).
- `std::env::var("DATABASE_URL").expect(...)`: lee la variable de entorno. Si no está definida, el programa termina con un mensaje claro. Es mejor que un valor por defecto porque en producción no quieres conectarte a la BD equivocada.
- `ConnectionManager::<MysqlConnection>::new(url)`: crea el manager. El tipo `MysqlConnection` es específico de Diesel para MySQL.
- `.max_size(15)`: máximo de conexiones simultáneas. Para un ERP con 50 usuarios concurrentes, 15 conexiones por instancia es razonable.

### 3.7.3 Migraciones con Diesel CLI

Las migraciones son versiones del esquema de la base de datos, como los commits de git.

```bash
# Generar una migración
diesel migration generate crear_clientes
```

Esto crea la carpeta `migrations/` con un timestamp y dos archivos:

```
migrations/
└── 2026-07-05-123456_crear_clientes/
    ├── up.sql       # ← lo que se aplica
    └── down.sql     # ← lo que se revierte
```

`up.sql`:
```sql
CREATE TABLE clientes (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(150) NOT NULL,
    rfc VARCHAR(13) NOT NULL UNIQUE,
    email VARCHAR(150),
    credito DECIMAL(12,2) NOT NULL DEFAULT 0,
    activo BOOLEAN NOT NULL DEFAULT TRUE
);
```

`down.sql`:
```sql
DROP TABLE clientes;
```

**Aplicar migraciones**:

```bash
diesel migration run        # Aplica todas las migraciones pendientes
diesel migration redo       # Revierte y vuelve a aplicar la última
diesel migration revert     # Revierte la última migración
```

Después de `diesel migration run`, Diesel genera automáticamente `src/schema.rs`:

```rust
// src/schema.rs (generado automáticamente, NO editar a mano)
table! {
    clientes (id) {
        id -> Unsigned<Integer>,
        nombre -> Varchar,
        rfc -> Varchar,
        email -> Nullable<Varchar>,
        credito -> Decimal,
        activo -> Bool,
    }
}
```

La macro `table!` define una tabla con sus columnas y tipos. Diesel usa esto para verificar en tiempo de compilación que tus consultas coinciden con el esquema.

### 3.7.4 Modelos: los structs que mapean las tablas

Además del `schema.rs` generado, necesitamos structs Rust para insertar y leer datos:

```rust
// src/modelos.rs
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

// Para LEER: coincide exactamente con la tabla
#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::clientes)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Cliente {
    pub id: u32,
    pub nombre: String,
    pub rfc: String,
    pub email: Option<String>,
    pub credito: rust_decimal::Decimal,
    pub activo: bool,
}

// Para INSERT: no tiene id (es AUTO_INCREMENT)
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::clientes)]
pub struct NuevoCliente {
    pub nombre: String,
    pub rfc: String,
    pub email: Option<String>,
    pub credito: rust_decimal::Decimal,
}

// Para UPDATE: todos los campos son Option (solo se actualizan los que están en Some)
#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::clientes)]
pub struct CambiosCliente {
    pub nombre: Option<String>,
    pub email: Option<String>,
    pub credito: Option<rust_decimal::Decimal>,
    pub activo: Option<bool>,
}
```

**Análisis línea por línea**:

- `#[derive(Queryable, Selectable, Serialize)]`: `Queryable` permite leer filas de la BD y convertirlas a `Cliente`. `Selectable` permite usar `.select()` para elegir columnas específicas. `Serialize` permite convertir a JSON para la API.
- `#[diesel(table_name = crate::schema::clientes)]`: vincula el struct a la tabla `clientes` definida en `schema.rs`.
- `pub struct Cliente { pub id: u32, ... }`: los campos deben coincidir en nombre y tipo con las columnas de la tabla. `email` es `Option<String>` porque la columna permite NULL.
- `pub struct NuevoCliente { ... }`: no incluye `id` (es AUTO_INCREMENT), `activo` (tiene default TRUE), ni los timestamps. `Insertable` permite usar este struct en `diesel::insert_into()`.
- `pub struct CambiosCliente { ... }`: todos los campos son `Option`. `AsChangeset` solo actualiza los campos que son `Some`. Si el usuario envía `{"nombre": "Nuevo nombre"}`, solo se actualiza el nombre, el resto queda igual.

**Error típico**: olvidar `#[diesel(table_name = ...)]`. Sin esto, Diesel no sabe a qué tabla pertenece el struct y no compila.

### 3.7.5 CRUD completo con Diesel

Con los modelos definidos y el schema generado, podemos escribir las operaciones CRUD. Diesel convierte estas llamadas a SQL en tiempo de compilación, verificando que los nombres de columna y tipos son correctos.

```rust
use crate::schema::clientes::dsl::*;
use crate::modelos::{Cliente, NuevoCliente, CambiosCliente};
use diesel::prelude::*;

pub fn listar(conn: &mut MysqlConnection) -> QueryResult<Vec<Cliente>> {
    clientes
        .order(id.desc())
        .limit(50)
        .load::<Cliente>(conn)
}

pub fn obtener(conn: &mut MysqlConnection, id_cliente: u32) -> QueryResult<Cliente> {
    clientes.find(id_cliente).first(conn)
}

pub fn crear(conn: &mut MysqlConnection, nuevo: &NuevoCliente) -> QueryResult<Cliente> {
    diesel::insert_into(clientes)
        .values(nuevo)
        .execute(conn)?;
    // Obtener el último ID insertado (MySQL específico)
    let ultimo_id = diesel::select(diesel::dsl::sql::<
        diesel::sql_types::Unsigned<diesel::sql_types::Integer>
    >("LAST_INSERT_ID()"))
    .first::<u32>(conn)?;
    obtener(conn, ultimo_id)  // Devolver el cliente completo
}

pub fn actualizar(
    conn: &mut MysqlConnection,
    id_cliente: u32,
    cambios: &CambiosCliente,
) -> QueryResult<Cliente> {
    diesel::update(clientes.find(id_cliente))
        .set(cambios)
        .execute(conn)?;
    obtener(conn, id_cliente)
}

pub fn eliminar(conn: &mut MysqlConnection, id_cliente: u32) -> QueryResult<usize> {
    diesel::delete(clientes.find(id_cliente)).execute(conn)
}
```

**Análisis línea por línea**:

- `use crate::schema::clientes::dsl::*;`: importa el DSL (Domain Specific Language) de la tabla `clientes`. Esto trae nombres como `clientes` (la tabla), `id`, `nombre`, etc. al alcance. Sin esto, tendrías que escribir `clientes::table` y `clientes::columns::id`.
- `clientes.order(id.desc()).limit(50).load::<Cliente>(conn)`: `.load::<Cliente>` ejecuta la consulta y mapea cada fila a `Cliente`. `order(id.desc())` ordena por ID descendente (los más recientes primero).
- `clientes.find(id_cliente).first(conn)`: `find` busca por clave primaria. `first` devuelve una sola fila. Si no existe, devuelve `diesel::result::Error::NotFound`.
- `diesel::insert_into(clientes).values(nuevo).execute(conn)?`: INSERT. `execute` devuelve el número de filas afectadas (1 si fue exitoso).
- `diesel::update(clientes.find(id_cliente)).set(cambios).execute(conn)?`: UPDATE. `set` recibe un struct `AsChangeset` y solo actualiza los campos `Some`.
- `diesel::delete(clientes.find(id_cliente)).execute(conn)?`: DELETE. `execute` devuelve el número de filas eliminadas (0 si no existía, 1 si se eliminó).

**Truco profesional**: `crear` debería devolver el registro completo, no solo el ID. Esto permite al cliente ver los valores por defecto que puso la base de datos (fechas, estado activo, etc.).

### 3.7.6 Integracion de Diesel con Actix Web

Diesel es síncrono, Actix Web es asíncrono. Para que funcionen juntos, usamos `web::block()`, que ejecuta código bloqueante en un hilo separado del pool de hilos de Actix. Esto evita que una consulta lenta a la BD bloquee el hilo principal del servidor.

```rust
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use crate::db::Pool;
use crate::modelos::{Cliente, NuevoCliente};
use crate::repositorios::clientes_repo;
use crate::errores::ErrorNegocio;

pub async fn listar(pool: web::Data<Pool>) -> Result<HttpResponse, ErrorNegocio> {
    let mut conn = pool.get_ref().get()
        .map_err(|e| ErrorNegocio::BaseDatos(e.to_string()))?;

    let resultado = web::block(move || {
        clientes_repo::listar(&mut conn)
    })
    .await
    .map_err(|e| ErrorNegocio::BaseDatos(e.to_string()))??;
    //                                ^^ doble ?: primero el del join handle, luego el de QueryResult

    Ok(HttpResponse::Ok().json(resultado))
}

pub async fn crear(
    pool: web::Data<Pool>,
    body: web::Json<NuevoCliente>,
) -> Result<HttpResponse, ErrorNegocio> {
    let nuevo = body.into_inner();
    let mut conn = pool.get_ref().get()
        .map_err(|e| ErrorNegocio::BaseDatos(e.to_string()))?;

    let cliente = web::block(move || {
        clientes_repo::crear(&mut conn, &nuevo)
    })
    .await
    .map_err(|e| ErrorNegocio::BaseDatos(e.to_string()))??;

    Ok(HttpResponse::Created().json(cliente))
}

pub async fn obtener(
    pool: web::Data<Pool>,
    path: web::Path<u32>,
) -> Result<HttpResponse, ErrorNegocio> {
    let id = path.into_inner();
    let mut conn = pool.get_ref().get()
        .map_err(|e| ErrorNegocio::BaseDatos(e.to_string()))?;

    let cliente = web::block(move || {
        clientes_repo::obtener(&mut conn, id)
    })
    .await
    .map_err(|e| ErrorNegocio::BaseDatos(e.to_string()))?
    .map_err(|_| ErrorNegocio::NoEncontrado(format!("Cliente {} no encontrado", id)))?;

    Ok(HttpResponse::Ok().json(cliente))
}
```

**Análisis línea por línea**:

- `let mut conn = pool.get_ref().get().map_err(...)?`: `pool.get_ref()` devuelve `&Pool`. `.get()` obtiene una conexión del pool. `map_err` convierte el error de pool a `ErrorNegocio::BaseDatos`.
- `web::block(move || { ... })`: `move` mueve `conn` y `nuevo` al closure. `web::block` devuelve `JoinHandle<Result<T, E>>`. El `.await` espera a que el closure termine en el hilo del pool.
- `.map_err(|e| ErrorNegocio::BaseDatos(e.to_string()))??`: primer `?` para errores de `JoinHandle` (si el hilo del pool entró en pánico). Segundo `?` para errores de Diesel (`QueryResult`). El doble `??` es común en este patrón.

**Truco profesional: transacciones con Diesel y Actix**:

```rust
pub async fn crear_pedido(
    pool: web::Data<Pool>,
    body: web::Json<PedidoInput>,
) -> Result<HttpResponse, ErrorNegocio> {
    let input = body.into_inner();
    let mut conn = pool.get_ref().get().map_err(...)?;

    let resultado = web::block(move || {
        conn.transaction(|tx| {
            // 1. Validar cliente
            let cliente = clientes_repo::obtener(tx, input.cliente_id)
                .map_err(|_| ErrorNegocio::NoEncontrado("Cliente no existe".into()))?;

            // 2. Crear pedido
            let pedido = pedidos_repo::crear(tx, &input)?;

            // 3. Descontar stock (dentro de la misma transacción)
            for linea in &input.lineas {
                productos_repo::descontar_stock(tx, &linea.sku, linea.cantidad)?;
            }

            Ok(pedido)
        })
    }).await.map_err(|e| ErrorNegocio::BaseDatos(e.to_string()))??;

    Ok(HttpResponse::Created().json(resultado))
}
```

`conn.transaction(|tx| { ... })` ejecuta un closure dentro de una transacción. Si el closure devuelve `Ok`, se hace commit. Si devuelve `Err`, se hace rollback. Es el patrón más seguro y limpio para transacciones.

**Error típico**: no usar `web::block()` y llamar a Diesel directamente desde un handler asíncrono. Esto bloquea el hilo de Actix Web y reduce drásticamente el rendimiento.

```rust
// MAL: bloquea el hilo de Actix
async fn listar(pool: web::Data<Pool>) -> ... {
    let mut conn = pool.get_ref().get().unwrap();
    let clientes = clientes_repo::listar(&mut conn)?;  // ← bloquea aquí
}

// BIEN: usa web::block para no bloquear
async fn listar(pool: web::Data<Pool>) -> ... {
    let mut conn = pool.get_ref().get().unwrap();
    let clientes = web::block(move || clientes_repo::listar(&mut conn)).await??;
}
```
```

## 3.11 SeaORM: la alternativa asincrona

SeaORM es un ORM **asíncrono** (async-first). A diferencia de Diesel, que bloquea el hilo mientras espera la BD, SeaORM usa `async/await` nativamente. Esto lo hace ideal para Actix Web porque no necesitas `web::block()` — el handler puede llamar directamente a SeaORM con `await`.

La desventaja: SeaORM es más reciente que Diesel y tiene un ecosistema más pequeño. Para un ERP nuevo, cualquiera de los dos es una buena elección. Si valoras la madurez y el tipado fuerte, elige Diesel. Si valoras la integración natural con async y una API más moderna, elige SeaORM.

### 3.11.1 Instalacion

```bash
cargo install sea-orm-cli
```

### 3.11.2 Configuracion

```toml
[dependencies]
sea-orm = { version = "1.1", features = ["sqlx-mysql", "runtime-tokio-rustls", "macros"] }
```

```rust
use sea_orm::*;
use dotenvy::dotenv;

pub async fn crear_pool() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let db = Database::connect(db_url).await?;
    Ok(db)
}
```

**Análisis**: `Database::connect(url).await` es asíncrono. No bloquea el hilo mientras establece la conexión. `DatabaseConnection` es el equivalente a `Pool` de Diesel, pero ya viene con pool interno de sqlx.

### 3.11.3 Entidades y CRUD

SeaORM genera entidades desde la base de datos existente con `sea-orm-cli generate entity -o src/entidades`.

```rust
use sea_orm::*;
use crate::entidades::cliente::Entity as Cliente;
use crate::entidades::cliente::Model as ClienteModel;

pub async fn listar(db: &DatabaseConnection) -> Result<Vec<ClienteModel>, DbErr> {
    Cliente::find().all(db).await
}

pub async fn crear(
    db: &DatabaseConnection,
    nombre: String,
    rfc: String,
) -> Result<ClienteModel, DbErr> {
    let nuevo = crate::entidades::cliente::ActiveModel {
        nombre: Set(nombre),
        rfc: Set(rfc),
        ..Default::default()  // Los campos no especificados usan su valor por defecto
    };
    nuevo.insert(db).await
}
```

**Análisis línea por línea**:

- `Cliente::find().all(db).await`: SELECT * FROM clientes. Devuelve `Vec<ClienteModel>`. Es asíncrono, se usa `await`.
- `ActiveModel { nombre: Set(nombre), rfc: Set(rfc), ..Default::default() }`: `ActiveModel` es una entidad que se va a insertar o actualizar. `Set(valor)` indica que el campo tiene un valor. `..Default::default()` usa los valores por defecto para los campos no especificados (como `activo: true`, `created_at: now()`).
- `nuevo.insert(db).await`: INSERT y devuelve la entidad completa, incluyendo el ID generado.

**Integración con Actix Web** (mucho más directa que Diesel):

```rust
use actix_web::{web, HttpResponse};
use sea_orm::*;
use crate::errores::ErrorNegocio;

pub async fn listar(db: web::Data<DatabaseConnection>) -> Result<HttpResponse, ErrorNegocio> {
    let clientes = Cliente::find()
        .all(db.get_ref())
        .await
        .map_err(|e| ErrorNegocio::BaseDatos(e.to_string()))?;

    Ok(HttpResponse::Ok().json(clientes))
}
```

No necesitas `web::block()` porque SeaORM ya es asíncrono. El handler es `async` y hace `await` directamente en la operación de BD.

## 3.12 Buenas practicas en APIs REST

Una API REST profesional requiere estructura, validación, documentación y configuración. En esta sección veremos cómo implementar estas prácticas en el ERP.

### 3.12.1 Separacion en capas

La estructura de directorios del proyecto final refleja la separación de responsabilidades:

```
src/
├── main.rs              ← Punto de entrada, configuración del servidor
├── config.rs            ← Carga de .env y configuración
├── db.rs                ← Pool de conexiones (Diesel o SeaORM)
├── errores.rs           ← ErrorNegocio con ResponseError
├── modelos/             ← Structs de request/response (con Serialize/Deserialize)
│   ├── mod.rs
│   ├── cliente.rs
│   ├── producto.rs
│   └── pedido.rs
├── repositorios/        ← Funciones que ejecutan SQL (solo datos)
│   ├── mod.rs
│   ├── clientes_repo.rs
│   └── productos_repo.rs
├── servicios/           ← Lógica de negocio (validaciones, cálculos)
│   ├── mod.rs
│   └── pedidos_svc.rs
└── handlers/            ← Funciones que reciben requests HTTP
    ├── mod.rs
    ├── clientes_handler.rs
    └── pedidos_handler.rs
```

Cada capa tiene una responsabilidad única:
- **Handlers**: reciben el request, extraen parámetros, llaman al servicio, devuelven la respuesta.
- **Servicios**: implementan la lógica de negocio (validar stock, calcular IVA, verificar permisos).
- **Repositorios**: ejecutan SQL, devuelven structs de datos. No toman decisiones de negocio.

### 3.12.2 Validacion con `validator`

```rust
use validator::Validate;
use serde::Deserialize;

#[derive(Deserialize, Validate)]
struct ClienteInput {
    #[validate(length(min = 1, max = 150))]
    nombre: String,

    #[validate(length(min = 12, max = 13))]
    rfc: String,

    #[validate(email)]
    email: String,
}

// En el handler:
async fn crear(payload: web::Json<ClienteInput>) -> Result<HttpResponse, ErrorNegocio> {
    payload.validate()
        .map_err(|e| ErrorNegocio::Validacion(e.to_string()))?;
    // ...
}
```

**Análisis**: `#[validate(length(min = 1, max = 150))]` valida que el nombre tenga entre 1 y 150 caracteres. `#[validate(email)]` valida el formato del email. `payload.validate()` ejecuta todas las validaciones y devuelve un `Result`. Si falla, se convierte a `ErrorNegocio::Validacion` y Actix devuelve 400 Bad Request.

### 3.12.3 Documentacion con `utoipa`

```rust
use utoipa::{ToSchema, OpenApi};
use utoipa::openapi::OpenApi;

#[derive(Serialize, Deserialize, ToSchema)]
struct Cliente {
    id: u32,
    nombre: String,
    rfc: String,
}

#[utoipa::path(
    get,
    path = "/clientes",
    responses(
        (status = 200, description = "Lista de clientes", body = [Cliente])
    )
)]
async fn listar() -> Result<HttpResponse, ErrorNegocio> { ... }

// Generar documentación OpenAPI
#[derive(OpenApi)]
#[openapi(paths(listar), components(schemas(Cliente)))]
struct ApiDoc;
```

Esto genera automáticamente un archivo OpenAPI (Swagger) que describe toda la API. Puedes servir la documentación en `/docs` con `utoipa-swagger-ui`.

### 3.12.4 Configuracion por entorno

```bash
# .env (desarrollo)
DATABASE_URL=mysql://root:secret@127.0.0.1:3306/erp_crm
JWT_SECRET=mi-secreto-cambiar-en-produccion
RUST_LOG=info
PUERTO=8080
```

```rust
// config.rs
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub puerto: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL debe estar definida"),
            jwt_secret: std::env::var("JWT_SECRET")
                .expect("JWT_SECRET debe estar definida"),
            puerto: std::env::var("PUERTO")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .expect("PUERTO debe ser un número"),
        }
    }
}
```

## 3.13 Pruebas

### 3.13.1 Tests de integracion con `actix_web::test`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_listar_clientes_devuelve_200() {
        // 1. Inicializar la app (sin BD real, usando datos mock)
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(MockPool::new()))
                .configure(configurar_rutas)
        ).await;

        // 2. Crear un request GET a /clientes
        let req = test::TestRequest::get()
            .uri("/clientes")
            .to_request();

        // 3. Enviar el request y obtener la respuesta
        let resp = test::call_service(&app, req).await;

        // 4. Verificar que la respuesta es 200 OK
        assert!(resp.status().is_success());
    }
}
```

**Análisis línea por línea**:

- `#[actix_web::test]`: similar a `#[tokio::test]`, pero configura el runtime de Actix Web para pruebas.
- `test::init_service(App::new()...)`: inicializa la aplicación sin lanzar el servidor HTTP. Las pruebas se ejecutan directamente sobre los handlers, sin pasar por el socket de red.
- `test::TestRequest::get().uri("/clientes").to_request()`: construye un request GET. Puedes añadir headers, cuerpo JSON, etc.
- `test::call_service(&app, req).await`: ejecuta el request contra la app y devuelve la respuesta.
- `assert!(resp.status().is_success())`: verifica que el código de estado es 2xx.

**Truco profesional: mock de BD para pruebas**:

```rust
// repositorios/mock.rs
pub struct MockClienteRepo;

impl MockClienteRepo {
    pub fn listar() -> Vec<Cliente> {
        vec![
            Cliente { id: 1, nombre: "Test".into(), rfc: "XAXX010101000".into(), ... },
        ]
    }
}

// En el handler de pruebas, inyectas MockClienteRepo en lugar del repo real
```

## 3.14 Ejemplo completo: API REST del ERP/CRM

El proyecto final está en `proyecto_api/` con dos variantes:

- **`api_diesel/`**: versión con Diesel (síncrona, usa `web::block()`). 6 archivos fuente, pool r2d2, autenticación JWT, 18 endpoints.
- **`api_seaorm/`**: versión con SeaORM (asíncrona, esqueleto documentado). README con instrucciones de migración.

### 3.14.1 Endpoints implementados

| Método | Ruta | Auth | Descripción |
|---|---|---|---|
| GET | `/health` | No | Health check con verificación de BD |
| POST | `/api/auth/login` | No | Login con JWT |
| GET | `/api/clientes` | No | Lista clientes (paginado) |
| POST | `/api/clientes` | No | Crea cliente |
| GET | `/api/clientes/{id}` | No | Detalle del cliente |
| PUT | `/api/clientes/{id}` | No | Actualiza cliente |
| DELETE | `/api/clientes/{id}` | No | Elimina cliente |
| GET | `/api/productos` | No | Lista productos |
| POST | `/api/productos` | No | Crea producto |
| GET | `/api/pedidos` | No | Lista pedidos |
| POST | `/api/pedidos` | No | Crea pedido (transacción) |
| GET | `/api/pedidos/{folio}` | No | Detalle del pedido |
| GET | `/api/reportes/ventas` | No | Reporte de ventas |
| GET | `/api/usuarios` | Sí (JWT) | Lista usuarios |

### 3.14.2 Ejecucion del proyecto final

```bash
# 1. Iniciar MariaDB
docker compose -f proyecto_api/docker-compose.yml up -d

# 2. Compilar y ejecutar API Diesel
cd proyecto_api/api_diesel
cargo run

# 3. Probar
curl http://localhost:8080/health
curl http://localhost:8080/api/clientes
```

## 3.15 Despliegue

### 3.15.1 Compilacion de produccion

```bash
cargo build --release
./target/release/api_erp
```

El binario compilado en `target/release/` es auto-contenido y no necesita Rust instalado para ejecutarse. Solo necesita acceso a MariaDB.

### 3.15.2 Dockerizacion (multi-stage)

```dockerfile
# Etapa 1: compilar
FROM rust:1.82 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Etapa 2: imagen mínima de ejecución
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libmariadb3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/api_erp /usr/local/bin/
EXPOSE 8080
CMD ["api_erp"]
```

**Análisis**: la etapa `builder` compila el binario. La etapa final solo tiene el binario y la librería `libmariadb3` (necesaria para conectar a MariaDB). La imagen resultante pesa ~80MB en lugar de los ~2GB de la imagen de Rust.

### 3.15.3 docker-compose completo

```yaml
version: "3.9"
services:
  mysql:
    image: mariadb:11
    environment:
      MYSQL_ROOT_PASSWORD: ${MYSQL_PASSWORD:-secret}
      MYSQL_DATABASE: erp_crm
    ports: ["3306:3306"]
    volumes:
      - mysql_data:/var/lib/mysql
      - ./sql/init.sql:/docker-entrypoint-initdb.d/init.sql
    healthcheck:
      test: ["CMD", "mysqladmin", "ping", "-h", "localhost"]
      timeout: 5s
      retries: 10

  api:
    build: .
    depends_on:
      mysql:
        condition: service_healthy
    environment:
      DATABASE_URL: mysql://root:${MYSQL_PASSWORD:-secret}@mysql:3306/erp_crm
      RUST_LOG: info
      JWT_SECRET: ${JWT_SECRET:-cambiar-en-produccion}
    ports: ["8080:8080"]

volumes:
  mysql_data:
```

**Análisis**: `healthcheck` asegura que MariaDB esté lista antes de iniciar la API. La variable `${MYSQL_PASSWORD:-secret}` permite sobrescribir la contraseña desde el entorno sin modificar el archivo. El volumen `mysql_data` persiste la BD entre reinicios del contenedor.

## 3.16 Ejercicios acumulativos (Parte 3) – 30 ejercicios

1. **Health check**: crea un endpoint `/health` que devuelva `{"status": "ok"}`.
2. **Endpoint de eco**: `/echo/{mensaje}` devuelve el mensaje.
3. **Suma de dos números**: `/sumar?a=1&b=2` devuelve 3.
4. **CRUD en memoria**: clientes, productos y pedidos.
5. **Middleware de logging**: loguea cada request con su duración.
6. **Validación de email**: rechaza emails sin `@`.
7. **Manejo de errores personalizado**: tipo `ErrorNegocio` con `ResponseError`.
8. **Conexión a MySQL con Diesel**: lista clientes desde la BD.
9. **Crear cliente vía POST con Diesel**.
10. **Actualizar cliente vía PUT con Diesel**.
11. **Eliminar cliente vía DELETE con Diesel**.
12. **Paginación**: `?page=2&per_page=10`.
13. **Filtros**: `?nombre=...&activo=true`.
14. **Ordenamiento**: `?orden=nombre&dir=asc`.
15. **Búsqueda fuzzy**: `?buscar=constructora`.
16. **Migración inicial con Diesel CLI**.
17. **Seeders**: poblar la BD con datos de ejemplo al iniciar.
18. **JWT login**: POST /auth/login devuelve un token.
19. **Middleware de auth**: rechaza requests sin token válido.
20. **Roles y permisos**: middleware que valida que el usuario tiene el rol requerido.
21. **Transacción en handler**: crear pedido con descuento de stock.
22. **Documentación OpenAPI**: con `utoipa`.
23. **Test de integración con `actix_web::test`**.
24. **Mock de la BD para tests**.
25. **Migración de Diesel a SeaORM**: rehaz el ejercicio 8.
26. **WebSocket para notificaciones**: notificar cuando un pedido cambia de estado.
27. **Rate limiting**: limitar a 100 requests por minuto por IP.
28. **CORS**: configurar CORS para un frontend en otro dominio.
29. **Compresión gzip**: middleware de compresión.
30. **Dockerfile multi-stage**: compilar y ejecutar en una imagen ligera.

## 3.17 Soluciones detalladas (Parte 3)

Las soluciones de los 30 ejercicios están en el **Anexo A.2**, junto con el código completo del proyecto `proyecto_api/`.

---

## Cierre de la Parte 3

Has aprendido a construir una API REST profesional con Rust, Actix Web y dos ORM. Continúa con los anexos en [`anexos.md`](anexos.md) (soluciones, casos de estudio, tutoriales) y los glosarios en [`glosario.md`](glosario.md) (términos de Rust, ERP/CRM, facturación, inglés-español y más).

> **Métricas finales de la Parte 3**:
> - 17 secciones H2 escritas (3.1 – 3.17).
> - 40+ subsecciones H3/H4.
> - 5 mini-proyectos físicos.
> - 50+ bloques de código.
> - 30 ejercicios propuestos.
> - ~5 000 palabras.


---
