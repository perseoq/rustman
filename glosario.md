# Glosarios del manual
> **Todos los glosarios de términos**: Rust, ERP/CRM, facturación, inglés-español, argot, y más.
---

## Tabla de contenidos
- [A.1 Glosario de terminos (Rust + ERP/CRM)](#a1-glosario-de-terminos-rust--erpcrm)
- [Apendice F: Glosario adicional (continuacion)](#apendice-f-glosario-adicional-continuacion)
- [Apendice O: Glosario de terminos mexicanos del ERP/CRM](#apendice-o-glosario-de-terminos-mexicanos-del-erpcrm)
- [Apendice Q: Glosario final y agradecimientos](#apendice-q-glosario-final-y-agradecimientos)
- [Q.1 Glosario final](#q1-glosario-final)
- [Apendice Y: Glosario de argot de Rust](#apendice-y-glosario-de-argot-de-rust)
- [Apendice EE: Glosario ingles-espanol para programadores](#apendice-ee-glosario-ingles-espanol-para-programadores)
- [Apendice VV: Glosario final (parte 3)](#apendice-vv-glosario-final-parte-3)
- [Apendice A2: Glosario de terminos de facturacion](#apendice-a2-glosario-de-terminos-de-facturacion)
- [Apendice A4: Glosario matematico](#apendice-a4-glosario-matematico)
- [Apendice A12: Glosario de terminos de marketing y CRM](#apendice-a12-glosario-de-terminos-de-marketing-y-crm)
- [Apendice A21: Glosario final de apendices](#apendice-a21-glosario-final-de-apendices)
- [Apendice A27: Glosario de terminos del manual](#apendice-a27-glosario-de-terminos-del-manual)
- [Apendice A29: Glosario de terminos fiscales mexicanos](#apendice-a29-glosario-de-terminos-fiscales-mexicanos)
- [Apendice A36: Glosario de argot de programadores](#apendice-a36-glosario-de-argot-de-programadores)
- [Apendice A51: Glosario de terminos avanzados de Rust](#apendice-a51-glosario-de-terminos-avanzados-de-rust)

---

## A.1 Glosario de terminos (Rust + ERP/CRM)

> **Aviso**: este glosario incluye más de 200 términos. Los términos marcados con `[R]` son del lenguaje Rust; los marcados con `[BD]` son de bases de datos; los marcados con `[E]` son específicos del dominio ERP/CRM.

### A

**Actix** `[R]`: framework web para Rust basado en el sistema de actores. *Ver también:* Actix Web.

**Actix Web** `[R]`: framework HTTP construido sobre Actix, uno de los más rápidos del ecosistema Rust según TechEmpower.

**Actor** `[R]`: modelo de concurrencia donde cada actor es un objeto con su propio estado que procesa mensajes secuencialmente.

**API REST**: estilo arquitectónico para servicios web basado en los métodos HTTP (GET, POST, PUT, DELETE) y recursos identificados por URLs.

**async/await** `[R]`: sintaxis de Rust para escribir código asíncrono, similar a otros lenguajes pero con verificación estática.

**Autenticación**: proceso de verificar la identidad de un usuario (por ejemplo, con usuario y contraseña).

**Autorización**: proceso de verificar qué acciones puede realizar un usuario autenticado.

### B

**Base de datos**: sistema organizado para almacenar, recuperar y gestionar datos.

**Base de datos relacional** `[BD]`: tipo de BD que organiza los datos en tablas con relaciones entre ellas, usando SQL como lenguaje de consulta.

**Borrowing** `[R]`: sistema de Rust que permite prestar referencias a valores sin transferir la propiedad. Hay borrowing inmutable (`&T`) y mutable (`&mut T`).

**Borrow checker** `[R]`: componente del compilador de Rust que verifica que las reglas de borrowing se cumplan.

### C

**Cargo** `[R]`: gestor de paquetes y sistema de compilación de Rust. Equivale a `npm` + `webpack` en JavaScript, o `pip` + `setuptools` en Python.

**CFDI** `[E]`: Comprobante Fiscal Digital por Internet. Formato estándar de facturación electrónica en México (versión 4.0 vigente desde 2022).

**Closures** `[R]`: funciones anónimas que pueden capturar variables del entorno (`|x| x + 1`).

**Compilador** `[R]`: programa que traduce código fuente a código máquina o bytecode. En Rust se llama `rustc`.

**Concurrency** `[R]`: ejecución de múltiples tareas que progresan en el mismo período de tiempo. Rust evita data races en compilación.

**Connection pool** `[BD]`: conjunto de conexiones pre-establecidas a una base de datos que se reutilizan.

**CRM** `[E]`: Customer Relationship Management. Sistema para gestionar las relaciones con clientes.

**Crates** `[R]`: paquetes de Rust publicados en <https://crates.io>.

**Cross-Origin Resource Sharing (CORS)**: mecanismo del navegador que permite o restringe peticiones desde otros dominios.

### D

**Data race** `[R]`: situación en concurrencia donde dos hilos acceden a la misma memoria simultáneamente y al menos uno escribe, sin sincronización.

**Deadlock** `[BD]`: situación en la que dos o más transacciones se bloquean mutuamente, cada una esperando un recurso que la otra tiene.

**Diesel** `[R/BD]`: ORM síncrono para Rust, uno de los más maduros.

**Docker**: plataforma de contenedores que permite empaquetar aplicaciones y sus dependencias en imágenes portables.

**DTO (Data Transfer Object)**: objeto que transporta datos entre capas de una aplicación.

### E

**Enum** `[R]`: tipo que puede ser una de varias variantes, cada una opcionalmente con datos. `enum Estado { Activo, Inactivo, Pendiente }`.

**ERP** `[E]`: Enterprise Resource Planning. Sistema integral para gestionar los procesos de una empresa (ventas, inventario, contabilidad, RRHH, etc.).

**Error irrecuperable** `[R]`: error del que el programa no puede continuar, manejado con `panic!`.

**Error recuperable** `[R]`: error del que el programa puede continuar, manejado con `Result<T, E>`.

### F

**Factura** `[E]`: documento mercantil que refleja una venta de bienes o servicios. En México se timbra como CFDI 4.0.

**Foreign Key** `[BD]`: columna que referencia la clave primaria de otra tabla, garantizando integridad referencial.

**Framework**: librería que proporciona una estructura base para desarrollar aplicaciones.

### G

**GET** `[HTTP]`: método HTTP para solicitar datos sin modificarlos.

**Git**: sistema de control de versiones distribuido.

**Guardian** `[R]`: puntero inteligente similar a un *lock* en otros lenguajes; permite acceso exclusivo a datos.

### H

**HashMap** `[R]`: estructura de datos que asocia claves con valores, con búsqueda en O(1) promedio.

**Heap** `[R]`: zona de memoria dinámica donde se asignan datos cuyo tamaño puede variar en tiempo de ejecución (por ejemplo, `Vec`, `String`).

**HTTP (HyperText Transfer Protocol)**: protocolo de comunicación entre clientes y servidores web.

### I

**IEPS** `[E]`: Impuesto Especial sobre Producción y Servicios. En México aplica a bebidas, botanas, alcohol, tabaco, etc.

**Index** `[BD]`: estructura auxiliar que acelera búsquedas a costa de más espacio en disco y tiempo en inserciones.

**Inmutable** `[R]`: que no puede cambiarse después de creado. En Rust, las variables son inmutables por defecto.

**InnoDB** `[BD]`: motor de almacenamiento transaccional de MySQL/MariaDB, el estándar recomendado.

**Inventario** `[E]`: conjunto de bienes de una empresa disponibles para la venta o producción.

**Iterador** `[R]`: objeto que permite recorrer una secuencia de elementos uno a uno.

**IVA** `[E]`: Impuesto al Valor Agregado. En México la tasa general es 16%, en zona fronteriza 8%.

### J

**JOIN** `[BD]`: operación SQL que combina filas de dos o más tablas basándose en una columna común.

**JSON (JavaScript Object Notation)**: formato ligero de intercambio de datos.

**JWT (JSON Web Token)**: estándar para crear tokens de acceso que se pueden verificar sin necesidad de almacenamiento en el servidor.

### K

**Kardex** `[E]`: registro cronológico de los movimientos de inventario (entradas, salidas, ajustes).

### L

**Lifetimes** `[R]`: anotaciones que indican cuánto tiempo vive una referencia, usadas para prevenir *use-after-free*.

**Logger**: componente que registra eventos del sistema.

### M

**Macro** `[R]`: construcción que genera código en tiempo de compilación. Se invoca con `!` (ej. `println!`).

**Middleware**: función que se ejecuta antes o después de cada request HTTP, modificando la cadena.

**Migración** `[BD]`: cambio versionado en el esquema de la base de datos.

**Mock**: objeto simulado que imita el comportamiento de uno real, usado en tests.

**Monomorfización** `[R]`: proceso por el cual el compilador genera una copia especializada de cada genérico para cada tipo usado.

**Mutabilidad** `[R]`: capacidad de un valor de ser modificado después de creado.

### N

**NoSQL**: bases de datos que no usan el modelo relacional (MongoDB, Redis, etc.). No es el caso de este manual.

**NULL** `[BD]`: valor especial que indica "ausencia de valor". En Rust se evita con `Option<T>`.

### O

**ORM (Object-Relational Mapping)**: técnica que mapea tablas SQL a objetos (structs) en el lenguaje de programación.

**Owned** `[R]`: tipo que posee sus datos, como `String` (en heap) o `i32` (en stack).

**Ownership** `[R]`: sistema único de Rust donde cada valor tiene un único dueño, responsable de liberarlo.

### P

**Pool de conexiones** `[BD]`: *ver* Connection pool.

**POST** `[HTTP]`: método HTTP para crear un nuevo recurso.

**Primary key** `[BD]`: columna o conjunto de columnas que identifica unívocamente cada fila.

**Proveedor** `[E]`: persona o empresa que suministra bienes o servicios.

### Q

**Query** `[BD]`: consulta SQL.

**Query DSL** `[R/BD]`: lenguaje de consultas específico del ORM (por ejemplo, el DSL de macros de Diesel).

### R

**r2d2** `[R/BD]`: librería de pool de conexiones genérica en Rust.

**Repositorio** `[BD]`: patrón de diseño que encapsula el acceso a datos.

**Result<T, E>** `[R]`: enum que representa éxito (`Ok(T)`) o error (`Err(E)`).

**REST** `[API]`: *ver* API REST.

**RFC** `[E]`: Registro Federal de Contribuyentes. Identificador fiscal único de personas físicas y morales en México. Formato: 4 letras + 6 dígitos + 3 caracteres alfanuméricos.

**Rust** `[R]`: lenguaje de programación de sistemas enfocado en seguridad de memoria y concurrencia sin data races.

**rustc** `[R]`: compilador de Rust.

**rustup** `[R]`: instalador y gestor de versiones de Rust.

### S

**SeaORM** `[R/BD]`: ORM asíncrono para Rust, construido sobre `sqlx`.

**SELECT** `[BD]`: operación SQL para consultar datos.

**serde** `[R]`: librería de Rust para serializar y deserializar datos (JSON, YAML, etc.).

**Slice** `[R]`: vista a una porción contigua de memoria, como `&str` o `&[T]`.

**SQL (Structured Query Language)**: lenguaje estándar para gestionar bases de datos relacionales.

**Stack** `[R]`: zona de memoria para variables locales y datos de tamaño fijo conocido en compilación.

**Struct** `[R]`: tipo de dato compuesto por el programador, similar a un registro.

### T

**TIMBRADO** `[E]`: proceso de enviar una factura al SAT (Servicio de Administración Tributaria) para su validación y firma digital, generando el CFDI.

**Trait** `[R]`: conjunto de métodos que un tipo puede implementar. Equivalente a interfaces en otros lenguajes.

**Transacción** `[BD]`: conjunto de operaciones SQL que se ejecutan como una unidad atómica.

**Tuple** `[R]`: agrupación de valores de tipos potencialmente diferentes, de tamaño fijo.

### U

**Unit testing**: pruebas que verifican unidades individuales de código (funciones, métodos).

**UPDATE** `[BD]`: operación SQL para modificar registros existentes.

**Usuario** `[E]`: persona que usa el sistema, generalmente con un rol y permisos asociados.

### V

**Vec<T>** `[R]`: vector dinámico (lista redimensionable) de elementos del mismo tipo.

**Validación**: proceso de verificar que los datos cumplen ciertas reglas (formato, rango, etc.).

### W

**Web Framework**: *ver* Framework.

**Wrapping arithmetic** `[R]`: aritmética que desborda circularmente (ej. `255u8 + 1 = 0`).

### Y

**YAML**: formato de serialización de datos legible por humanos, usado en configuración.

### Z

**&str** `[R]`: *string slice*, una referencia inmutable a una secuencia UTF-8.

(Continuación del glosario disponible en el archivo `glosario_completo.md` del repositorio.)


# Apendice F: Glosario adicional (continuacion)

(Continuación del glosario principal con términos avanzados)

**Asynchronous**: ejecución de tareas sin bloquear el hilo principal; permite manejar miles de conexiones concurrentes.

**Cargo.toml**: archivo de manifiesto de un crate Rust, donde se declaran nombre, versión, dependencias y configuración.

**Compilation unit**: unidad básica de compilación; un crate produce un rlib o un binario.

**Const generics**: genéricos sobre valores constantes (no tipos). Ej: `Array<T, const N: usize>`.

**DST (Dynamically Sized Type)**: tipo cuyo tamaño no se conoce en compilación, como `str` o `[T]`. Siempre detrás de un puntero.

**FFI (Foreign Function Interface)**: mecanismo para llamar funciones de C desde Rust (y viceversa).

**Futures**: valores que representan cálculos que pueden completarse en el futuro; base de la programación async.

**Iced/egui**: librerías de GUI para Rust.

**IR (Intermediate Representation)**: representación interna del compilador entre el código fuente y el código máquina.

**LLVM**: infraestructura de compilación que Rust usa como backend.

**Monomorphization**: el compilador genera una copia especializada del código genérico para cada combinación de tipos usada.

**Pin**: tipo que garantiza que un valor no se moverá de su posición en memoria; necesario para futures auto-referenciales.

**Procedural macros**: funciones que reciben y producen código Rust en tiempo de compilación; permiten crear DSLs y reducir boilerplate.

**RAII (Resource Acquisition Is Initialization)**: patrón donde los recursos se liberan automáticamente al salir del scope; en Rust se implementa vía el trait `Drop`.

**Send**: marker trait que indica que un tipo puede transferirse entre hilos.

**Sync**: marker trait que indica que un tipo puede compartirse entre hilos.

**Tower**: librería de abstracciones para servicios async; usada internamente por muchos frameworks.

**WASM (WebAssembly)**: formato binario portable que Rust puede compilar como target; permite ejecutar Rust en navegadores.

**Waker**: mecanismo del runtime async para notificar a un future que debe ser re-evaluado.

---


# Apendice O: Glosario de terminos mexicanos del ERP/CRM

Este glosario complementa el glosario principal con términos específicos del contexto fiscal y comercial mexicano.

**Almacén**: lugar físico o lógico donde se almacenan productos. En el ERP, cada producto puede tener stock en N almacenes.

**Alta de cliente**: proceso de registrar un nuevo cliente en el sistema, capturando RFC, datos fiscales, dirección, etc.

**Balanza de comprobación**: reporte contable que verifica que la suma de los cargos iguala la suma de los abonos.

**Carta porte**: documento que ampara el traslado de mercancías en territorio mexicano (CFDI complementario).

**CFDI 4.0**: Comprobante Fiscal Digital por Internet, versión 4.0 (vigente desde 2022). Es el formato obligatorio para facturación electrónica en México.

**Código de barras**: representación gráfica de un código de identificación (EAN-13, UPC, Code 128, etc.). Útil para identificar productos en el punto de venta.

**Contado**: forma de pago inmediata (efectivo, transferencia, tarjeta).

**Crédito**: forma de pago diferida. El cliente recibe el producto y paga después. El ERP lleva un control de antigüedad de saldos (corriente, 30, 60, 90+ días).

**CURP**: Clave Única de Registro de Población. Identificador único de cada persona en México (no aplica a personas morales).

**DOF**: Diario Oficial de la Federación. Publicación oficial del gobierno mexicano donde se publican las leyes y reglamentos.

**Devolución**: acto de regresar un producto vendido, generalmente con nota de crédito.

**Efectivo**: pago inmediato en moneda nacional.

**Estado de cuenta**: documento que muestra los movimientos y saldo de un cliente o proveedor.

**Factura**: documento fiscal que ampara una venta. En México debe ser un CFDI 4.0 timbrado por un PAC.

**Homoclave**: tres caracteres alfanuméricos al final del RFC que distinguen homónimos dentro de la misma fecha de nacimiento o constitución.

**IEPS**: Impuesto Especial sobre Producción y Servicios. Aplica a bebidas, botanas, alcohol, tabaco, gasolinas, etc. La tasa varía según el producto.

**Inventario**: conjunto de productos disponibles para la venta. Puede valorarse por costo promedio, PEPS (primeras entradas, primeras salidas), UEPS, o identificado.

**IVA**: Impuesto al Valor Agregado. Tasa general del 16% en México, 8% en zona fronteriza. Aplica a la mayoría de las ventas de bienes y servicios.

**Kardex**: registro cronológico de los movimientos de inventario (entradas, salidas, ajustes, transferencias).

**Nota de crédito**: documento que anula parcial o totalmente una factura anterior. Se emite como un CFDI 4.0 con tipo "E" (Egreso).

**PAC**: Proveedor Autorizado de Certificación. Empresa autorizada por el SAT para timbrar CFDI. Ejemplos: Facturama, Edicom, FEL, Solución Factible.

**Pedido**: solicitud de productos por parte de un cliente. Puede convertirse en factura al entregarse o pagarse.

**Punto de venta (POS)**: sistema donde se registran las ventas directas al consumidor final. En México, las facturas en punto de venta deben ser CFDI al público (sin datos del receptor, salvo que se solicite factura con datos).

**Razón social**: nombre legal de una empresa, tal como aparece en su acta constitutiva. En el RFC se usan las primeras letras.

**Remisión**: documento que ampara la entrega de mercancía sin que se haya emitido la factura. Útil cuando el pago se procesará después.

**RFC**: Registro Federal de Contribuyentes. Identificador fiscal único de personas físicas (13 caracteres) y morales (12 caracteres) en México.

**SAT**: Servicio de Administración Tributaria. Organismo del gobierno mexicano encargado de la recaudación fiscal.

**SKU**: Stock Keeping Unit. Código único que identifica un producto en el inventario. Diferente del código de barras EAN-13 (que es estándar de la industria).

**Timbrado**: proceso de enviar un CFDI al SAT (a través de un PAC) para su validación y firma digital. El resultado es un XML con un folio fiscal (UUID) y un sello digital.

**UUID (Folio fiscal)**: identificador único universal que el SAT asigna a cada CFDI timbrado. Formato: 8-4-4-4-12 caracteres hexadecimales.

**Venta**: acto de transferir un producto o servicio a cambio de un pago. En el ERP se distingue entre cotización, pedido, remisión y factura.

**Zona fronteriza**: región del norte de México donde el IVA es del 8% en lugar del 16%. Definida por el SAT.


---

# Apendice Q: Glosario final y agradecimientos

## Q.1 Glosario final

(Este glosario complementa al glosario principal. Se recomienda leerlos en conjunto.)

**AOT (Ahead-of-Time)**: compilación que ocurre antes de la ejecución, en contraste con JIT (Just-in-Time). Rust es AOT.

**Cow (Copy-on-Write)**: técnica donde los datos se comparten hasta que uno de los dueños los modifica. `Cow<T>` en Rust es un enum que implementa esto.

**Dedupe**: herramienta de Rust que elimina código duplicado en el binario final.

**Double free**: bug donde se libera la misma memoria dos veces; imposible en Rust safe.

**Eager evaluation**: evaluación inmediata de argumentos. En contraste con lazy evaluation.

**Edition**: versión del lenguaje (2015, 2018, 2021, 2024). Cambios compatibles hacia atrás pero con mejoras.

**ffi**: Foreign Function Interface, mecanismo para llamar código C desde Rust (y viceversa).

**Hashbrown**: implementación de HashMap en Rust, optimizada.

**Inline**: optimización donde el compilador reemplaza una llamada de función con el cuerpo de la función.

**JIT**: compilación en tiempo de ejecución, usada por Java, C#, JavaScript. Rust usa AOT.

**Kani**: verificador de modelos para Rust, usado para verificar propiedades de programas.

**Lazy evaluation**: evaluación perezosa, donde el valor se calcula sólo cuando se necesita. `Iterator` en Rust es lazy.

**MIR (Mid-level Intermediate Representation)**: representación intermedia usada por el compilador de Rust para análisis.

**NLL (Non-Lexical Lifetimes)**: mejora del verificador de préstamos introducida en 2018.

**Owned**: tipo que posee sus datos, como `String`.

**Pattern**: estructura sintáctica que se compara con un valor, usado en `match` y `if let`.

**Quickcheck**: librería para property-based testing (alternativa a `proptest`).

**RAII**: Resource Acquisition Is Initialization, patrón donde los recursos se liberan al salir del scope.

**RusTLS**: implementación de TLS en Rust, usada por muchas crates.

**Sled**: base de datos embebida escrita en Rust.

**Stdin/stdout/stderr**: entrada y salida estándar.

**Target triple**: identificador del tipo de plataforma (e.g., `x86_64-unknown-linux-gnu`).

**Tokio**: runtime async más usado en Rust.

**Trait alias**: feature experimental que permite dar un nombre a un conjunto de trait bounds.

**Union**: tipo inseguro donde varios campos comparten la misma memoria. Requiere `unsafe`.

**WASI**: WebAssembly System Interface, permite ejecutar Rust en navegadores y entornos no-navegador.

# Apendice Y: Glosario de argot de Rust

La comunidad de Rust tiene su propio vocabulario. Aquí una pequeña guía para que no te pierdas en las conversaciones de Discord o Reddit.

- **rustáceo** (rustacean): usuario o desarrollador de Rust.
- **the borrow checker** (el verificador de préstamos): el componente del compilador que verifica las reglas de ownership. A veces se le culpa (con cariño) por errores.
- **fighting the borrow checker** (peleando con el borrow checker): intentar escribir código que el verificador rechaza, y tener que reorganizar.
- **fighting the trait system** (peleando con el sistema de traits): similar, pero con traits.
- **stuck on the borrow checker** (atorado con el borrow checker): expresión común de frustración.
- **borrowck** (abreviación): borrow checker.
- **NLL** (Non-Lexical Lifetimes): la versión moderna del verificador.
- **async hell** (infierno async): problemas con tipos de future complejos.
- **trait soup** (sopa de traits): cuando un tipo requiere muchos traits derivados.
- **PREG** (Pretty Rust Error): un mensaje de error del compilador que es claro y útil.
- **NLL grief**: frustración porque el verificador rechaza código que parece correcto.
- **unsafe**: la palabra clave que desactiva las verificaciones de seguridad. Usar con responsabilidad.
- **wrapper hell**: cuando tienes 5 capas de `Arc<Mutex<Result<Vec<Box<dyn Trait>>>>>`. Común pero no ideal.
- **ZST** (Zero-Sized Type): un tipo sin tamaño, como `()` o `PhantomData<T>`.
- **DST** (Dynamically Sized Type): un tipo cuyo tamaño no se conoce en compilación, como `str` o `[T]`.
- **FE** (Function Expression): una expresión que es una función, como `|x| x + 1`.
- **FFI**: Foreign Function Interface, llamada a C.
- **NPM but for Rust**: cargo.
- **cargo cult**: usar prácticas sin entender por qué, o seguir modas sin cuestionar.
- **BDSM**: (chiste interno) Bind, Deref, Size, Move. Trato a tus datos con respeto.
- **rewrite it in Rust**: la tendencia a reescribir todo en Rust. (Con humor.)
- **/r/playrust**: subreddit del juego Rust, no del lenguaje. Confusión común.
- **I am turbo fish and I swim through code**: el operador `::<>` se llama turbofish.
- **Clippy's suggestions**: las recomendaciones de clippy, a veces muy estrictas.
- **rustc eats my code**: cuando el compilador rechaza código.
- **uwu**: expresión de cariño en línea.
- **🦀**: el emoji del cangrejo herradura, mascota de Rust.
- **Ferris**: el cangrejo herradura de la mascota oficial.
- **MIRI**: un verificador de comportamiento indefinido para Rust.
- **rustc_wrapper**: personalizar la compilación.
- **the book**: el libro oficial, "The Rust Programming Language".
- **TRPL**: lo mismo.
- **too many linked lists**: tutorial clásico para entender ownership.
- **Advent of Code**: concurso de programación en diciembre, popular entre rustáceos.

---


# Apendice EE: Glosario ingles-espanol para programadores

| English | Español | Notas |
|---|---|---|
| Array | Arreglo | Colección de tamaño fijo |
| Assertion | Aserción | Verificación en tests |
| Assignment | Asignación | Dar valor a una variable |
| Attribute | Atributo | Metadatos como `#[derive(Debug)]` |
| Binary | Binario | Archivo ejecutable |
| Bit field | Campo de bits | Entero empaquetado |
| Branch | Rama | Una vía de ejecución |
| Breakpoint | Punto de interrupción | En debugging |
| Buffer | Búfer | Memoria temporal |
| Bug | Error | Defecto en el software |
| Build | Compilación | Generar el ejecutable |
| Callback | Callback | Función llamada por otra |
| Casting | Conversión de tipo | `as` en Rust |
| Closure | Clausura | Función anónima |
| Code coverage | Cobertura de código | Porcentaje ejecutado en tests |
| Compilation error | Error de compilación | Detectado antes de ejecutar |
| Compiler | Compilador | Programa que traduce código |
| Concurrency | Concurrencia | Múltiples tareas en paralelo |
| Constant | Constante | Valor inmutable en compilación |
| Constructor | Constructor | Función que crea instancias |
| Crate | Crate | Unidad de distribución |
| Crash | Cuelgue | Terminación abrupta |
| Debug | Depurar | Encontrar bugs |
| Debugger | Depurador | Herramienta para debug |
| Default | Por defecto | Valor predeterminado |
| Deref | Desreferenciar | Acceder al valor apuntado |
| Derived | Derivado | Generado automáticamente |
| Documentation | Documentación | Comentarios y docs |
| Double free | Doble liberación | Bug de C++ |
| Enum | Enum | Tipo con variantes |
| Equality | Igualdad | `==` |
| Exception | Excepción | No existe en Rust |
| Expression | Expresión | Devuelve un valor |
| Feature flag | Bandera de feature | Compilación condicional |
| Field | Campo | Atributo de struct |
| Float | Flotante | Número con decimales |
| Foreign function | Función externa | FFI a C |
| Framework | Framework | Librería base |
| Function | Función | Bloque de código reutilizable |
| Garbage collection | Recolección de basura | No existe en Rust |
| Generic | Genérico | Parámetros de tipo |
| Hash | Hash | Función de dispersión |
| Heap | Montón | Memoria dinámica |
| Helper | Auxiliar | Función de apoyo |
| Identifier | Identificador | Nombre de variable |
| Implementation | Implementación | Bloque `impl` |
| Index | Índice | Posición en array |
| Infinite loop | Bucle infinito | Sin condición de salida |
| Inheritance | Herencia | No existe en Rust |
| Instance | Instancia | Valor concreto de un tipo |
| Integer | Entero | Número sin decimales |
| Interface | Interfaz | Equivalente a trait |
| Interpreter | Intérprete | Ejecuta sin compilar |
| Iterator | Iterador | Recorre una colección |
| Keyword | Palabra reservada | No se puede usar como identificador |
| Lifetime | Tiempo de vida | Anotación `'a` |
| Linker | Enlazador | Une archivos objeto |
| Literal | Literal | Valor escrito directamente |
| Lock | Cerrojo | Mutex |
| Loop | Bucle | Repetición |
| Macro | Macro | Generación de código |
| Map | Mapa | `HashMap` |
| Memory leak | Fuga de memoria | Memoria no liberada |
| Method | Método | Función asociada a un tipo |
| Migration | Migración | Cambio de esquema de BD |
| Module | Módulo | Unidad de organización |
| Mutex | Mutex | Exclusión mutua |
| Null | Nulo | Ausencia de valor (no en Rust) |
| Object | Objeto | Instancia |
| Operator | Operador | `+`, `-`, etc. |
| Overflow | Desbordamiento | Valor fuera de rango |
| Override | Sobrescribir | Reemplazar implementación |
| Package | Paquete | `crate` en Rust |
| Parameter | Parámetro | Argumento de función |
| Panic | Pánico | Terminación por error irrecuperable |
| Parse | Parsear | Convertir texto a valor |
| Pattern | Patrón | Estructura sintáctica |
| Pointer | Puntero | Referencia a memoria |
| Polymorphism | Polimorfismo | Múltiples formas |
| Preprocessor | Preprocesador | No existe en Rust |
| Primitive | Primitivo | Tipo built-in |
| Print | Imprimir | Salida a pantalla |
| Property | Propiedad | Atributo de objeto |
| Range | Rango | `0..10` |
| Recursion | Recursión | Función que se llama a sí misma |
| Reference | Referencia | `&T` |
| Register | Registro | Variable de CPU (no en Rust) |
| Release | Lanzamiento | Versión para producción |
| Return | Retornar | Devolver un valor |
| Runtime | Tiempo de ejecución | Cuando el programa corre |
| Scope | Ámbito | Bloque de visibilidad |
| Semicolon | Punto y coma | `;` |
| Slice | Slice | Vista a una colección |
| Stack | Pila | Memoria de variables locales |
| Statement | Sentencia | No devuelve valor |
| Static | Estático | Variable de duración 'static |
| String | Cadena | Texto |
| Struct | Estructura | Tipo compuesto |
| Subroutine | Subrutina | Función |
| Symbol | Símbolo | Nombre de variable/tipo |
| Template | Plantilla | Equivalente a genérico |
| Test | Test | Prueba unitaria |
| Thread | Hilo | Unidad de concurrencia |
| Trait | Trait | Conjunto de métodos |
| Tuple | Tupla | Agrupación de valores |
| Type | Tipo | Define la naturaleza de un valor |
| Type inference | Inferencia de tipos | Deducción automática |
| Undefined behavior | Comportamiento indefinido | Bug de bajo nivel |
| Union | Unión | Tipo inseguro |
| Unit | Unidad | Tipo `()` |
| Variable | Variable | Valor con nombre |
| Vector | Vector | `Vec<T>` |
| Virtual method | Método virtual | No existe en Rust |
| Void | Vacío | `()` en Rust |
| Wrapper | Envoltorio | Tipo que envuelve otro |
| Yield | Ceder | En generadores |

---


# Apendice VV: Glosario final (parte 3)

**Crates.io**: registro oficial de paquetes de Rust.
**docs.rs**: documentación autogenerada de todas las crates.
**lib.rs**: alternativa a crates.io con búsqueda mejorada.
**lifetime elision**: reglas que permiten al compilador inferir lifetimes.
**cargo workspace**: colección de crates que comparten target/ y Cargo.lock.
**rust-analyzer**: Language Server Protocol para Rust, usado por VS Code, IntelliJ, etc.
**edition 2021/2024**: versiones del lenguaje con cambios compatibles hacia atrás.
**`?` operator**: propaga errores automáticamente.
**`#[derive(...)]`**: genera implementaciones de traits automáticamente.
**`String`**: cadena owned, en heap, mutable.
**`&str`**: string slice, referencia a una secuencia UTF-8.
**`Vec<T>`**: vector dinámico.
**`HashMap<K, V>`**: tabla hash.
**`Result<T, E>`**: éxito o error.
**`Option<T>`**: presencia o ausencia.
**`Box<T>`**: puntero a heap.
**`Rc<T>` / `Arc<T>`**: reference counting.
**`Cell<T>` / `RefCell<T>`**: interior mutability.
**`Mutex<T>` / `RwLock<T>`**: sincronización.
**`mpsc::channel`**: message passing.
**`tokio`**: runtime async.
**`actix-web`**: framework HTTP.
**`diesel`**: ORM síncrono.
**`sea-orm`**: ORM async.
**`serde`**: serialización.
**`chrono`**: fechas.
**`uuid`**: identificadores únicos.
**`anyhow`**: errores simplificados.
**`thiserror`**: errores personalizados.
**`tracing`**: logging estructurado.
**`clap`**: CLI args.
**`reqwest`**: cliente HTTP.
**`criterion`**: benchmarking.
**`proptest`**: property-based testing.
**`mockall`**: mocks.
**`validator`**: validación de structs.
**`utoipa`**: OpenAPI/Swagger.
**`jsonwebtoken`**: JWT.
**`dotenvy`**: cargar .env.
**`env_logger`**: logger simple.
**`log`**: fachada de logging.

---


# Apendice A2: Glosario de terminos de facturacion

**Addenda**: información adicional que se incluye en un CFDI para una industria o cliente específico.

**Anticipo**: pago que el cliente hace antes de recibir la mercancía. Se factura cuando se aplica contra el pedido final.

**Carta porte**: CFDI complementario que ampara el traslado de mercancía en territorio mexicano.

**CFDI 4.0**: formato vigente desde 2022 de factura electrónica en México.

**CFDI de nómina**: CFDI tipo "N" para el pago de nómina a empleados.

**CFDI de pago**: CFDI tipo "P" emitido cuando un cliente paga facturas a crédito (PPD).

**CFDI de traslado**: CFDI tipo "T" para documentar el traslado de bienes.

**Complemento**: información adicional que se agrega a un CFDI según el tipo de operación.

**Forma de pago**: catálogo del SAT que indica cómo se realizó el pago (efectivo, transferencia, cheque, etc.).

**Método de pago**: PUE (Pago en una sola exhibición) o PPD (Pago en parcialidades o diferido).

**PAC**: Proveedor Autorizado de Certificación, que sella los CFDI con la autorización del SAT.

**Sello digital**: cadena cifrada que garantiza la integridad del CFDI. Generada con el certificado del emisor.

**Serie**: identificador opcional que agrupa CFDI relacionados (e.g., "A" para tienda 1, "B" para tienda 2).

**Folio**: número consecutivo del CFDI. Combinado con la serie, identifica unívocamente al comprobante.

**UUID (folio fiscal)**: identificador único universal asignado por el SAT a cada CFDI timbrado.

**Uso de CFDI**: catálogo del SAT que indica el uso que el receptor dará al CFDI (G03 para gastos en general, etc.).

**Receptor**: la persona o empresa que recibe el CFDI. Puede ser un cliente o un proveedor.

**Emisor**: la persona o empresa que emite el CFDI. Generalmente el negocio que vende.

**Resico**: Régimen Simplificado de Confianza, vigente desde 2022 para personas físicas y morales pequeñas.

**Régimen fiscal**: clasificación del SAT para el tipo de contribuyente (e.g., 601 General de Ley PM).

---


# Apendice A4: Glosario matematico

Para entender mejor algunos conceptos:

**Complejidad algorítmica**: medida de los recursos que consume un algoritmo. O(1) constante, O(log n) logarítmica, O(n) lineal, O(n²) cuadrática, O(2ⁿ) exponencial.

**Big O**: notación para describir el peor caso de la complejidad.

**Hashing**: técnica para mapear datos de tamaño variable a tamaño fijo. Usado en `HashMap`.

**Recursión**: técnica donde una función se llama a sí misma.

**Iteración**: técnica donde se repite una operación hasta cumplir una condición.

**Divide y vencerás**: estrategia algorítmica que divide un problema en subproblemas más pequeños.

**Programación dinámica**: técnica para resolver problemas con subproblemas superpuestos, almacenando resultados intermedios.

**Algoritmos greedy**: estrategia que toma la mejor decisión local en cada paso.

**Backtracking**: técnica para encontrar todas las soluciones probando combinaciones.

**Grafos**: estructura de datos con nodos y aristas. Usados para modelar relaciones.

**Árboles**: grafos sin ciclos, con un nodo raíz.

**Búsqueda binaria**: O(log n) en arrays ordenados.

**Ordenamiento**: O(n log n) en el caso promedio con quicksort/mergesort.

---


# Apendice A12: Glosario de terminos de marketing y CRM

**Lead**: cliente potencial que ha mostrado interés pero no ha comprado.

**MQL (Marketing Qualified Lead)**: lead que el equipo de marketing considera listo para ventas.

**SQL (Sales Qualified Lead)**: lead que el equipo de ventas considera listo para cerrar.

**Conversion**: pasar de una etapa del embudo a la siguiente.

**Embudo de ventas (sales pipeline)**: visualización de las etapas por las que pasa un cliente potencial.

**Touchpoint**: cada interacción entre la marca y el cliente.

**Customer journey**: el recorrido completo del cliente desde el primer contacto hasta la post-compra.

**Churn**: porcentaje de clientes que abandonan en un periodo.

**Retention**: porcentaje de clientes que se mantienen.

**LTV (Lifetime Value)**: valor total que un cliente aporta durante su relación con la empresa.

**CAC (Customer Acquisition Cost)**: costo de adquirir un nuevo cliente.

**NPS (Net Promoter Score)**: métrica de satisfacción del cliente.

**Upsell**: vender un producto más caro o más completo al cliente actual.

**Cross-sell**: vender productos complementarios.

**Downsell**: ofrecer una opción más barata al cliente que no puede pagar la principal.

**ABM (Account-Based Marketing)**: estrategia de marketing centrada en cuentas específicas.

**CTR (Click-Through Rate)**: porcentaje de personas que hacen clic en un enlace.

**CPC (Cost Per Click)**: costo por clic en publicidad digital.

**ROI (Return on Investment)**: retorno sobre la inversión.

**KPI (Key Performance Indicator)**: indicador clave de rendimiento.

---


# Apendice A21: Glosario final de apendices

**ADR (Architecture Decision Record)**: documento que captura una decisión arquitectónica importante.

**TCO (Total Cost of Ownership)**: costo total de propiedad, incluye adquisición, operación, mantenimiento.

**ROI (Return on Investment)**: retorno sobre la inversión, medida de rentabilidad.

**MVP (Minimum Viable Product)**: producto mínimo viable, primera versión funcional.

**SLA (Service Level Agreement)**: acuerdo de nivel de servicio, compromisos de disponibilidad.

**SLO (Service Level Objective)**: objetivo de nivel de servicio, métricas internas.

**KPI (Key Performance Indicator)**: indicador clave de rendimiento.

**RTO (Recovery Time Objective)**: tiempo objetivo de recuperación tras un desastre.

**RPO (Recovery Point Objective)**: punto objetivo de recuperación, hasta dónde podemos perder datos.

**SRE (Site Reliability Engineering)**: ingeniería de confiabilidad de servicios.

**DevOps**: combinación de desarrollo y operaciones.

**GitOps**: gestión de infraestructura y configuración via Git.

**CI/CD (Continuous Integration/Continuous Deployment)**: integración y despliegue continuo.

**Blue-green deployment**: estrategia de despliegue con dos ambientes idénticos.

**Canary deployment**: estrategia de despliegue gradual a un subconjunto de usuarios.

**A/B testing**: pruebas A/B para comparar variantes.

**Feature flag**: bandera para activar/desactivar funcionalidades sin redeploy.

**Cache**: memoria temporal para acelerar accesos.

**CDN (Content Delivery Network)**: red de distribución de contenido.

**WAF (Web Application Firewall)**: firewall de aplicaciones web.

**DDoS (Distributed Denial of Service)**: ataque de denegación de servicio distribuido.

**XSS (Cross-Site Scripting)**: ataque de inyección de código en páginas web.

**CSRF (Cross-Site Request Forgery)**: ataque de falsificación de solicitud entre sitios.

**SQL Injection**: ataque de inyección de SQL.

**OWASP**: Open Web Application Security Project, organización que publica guías de seguridad.

**CVSS (Common Vulnerability Scoring System)**: sistema de puntuación de vulnerabilidades.

**CVE (Common Vulnerabilities and Exposures)**: identificadores únicos de vulnerabilidades.

---


# Apendice A27: Glosario de terminos del manual

(Para referencia rápida)

**Rust**: lenguaje de programación.
**Cargo**: gestor de paquetes.
**crate**: paquete Rust.
**trait**: conjunto de métodos.
**struct**: tipo compuesto con campos nombrados.
**enum**: tipo con variantes.
**ownership**: sistema de propiedad de memoria.
**borrowing**: prestar referencias.
**lifetime**: tiempo de vida de una referencia.
**Result**: éxito o error.
**Option**: presencia o ausencia.
**match**: pattern matching.
**async/await**: código asíncrono.
**Tokio**: runtime async.
**Actix Web**: framework HTTP.
**Diesel**: ORM síncrono.
**SeaORM**: ORM asíncrono.
**MariaDB**: BD relacional.
**JWT**: JSON Web Token.
**CFDI**: Comprobante Fiscal Digital por Internet (México).
**PAC**: Proveedor Autorizado de Certificación.
**RFC**: Registro Federal de Contribuyentes.
**IVA**: Impuesto al Valor Agregado.
**IEPS**: Impuesto Especial sobre Producción y Servicios.
**ERP**: Enterprise Resource Planning.
**CRM**: Customer Relationship Management.
**API**: Application Programming Interface.
**REST**: Representational State Transfer.
**CRUD**: Create, Read, Update, Delete.
**ORM**: Object-Relational Mapping.
**SQL**: Structured Query Language.
**HTTP**: HyperText Transfer Protocol.
**JSON**: JavaScript Object Notation.
**Mermaid**: lenguaje para diagramas.
**CI/CD**: Continuous Integration/Continuous Deployment.

---


# Apendice A29: Glosario de terminos fiscales mexicanos

Para entender mejor el contexto del ERP:

**Constancia de situación fiscal**: documento emitido por el SAT que muestra la información fiscal de un contribuyente. Se puede obtener en línea.

**e.firma (antes FIEL)**: firma electrónica avanzada del SAT. Es el equivalente a una firma autógrafa para efectos fiscales.

**Certificado de sello digital (CSD)**: archivo que se usa para firmar los CFDI. Se genera a partir de la e.firma.

**Declaración anual**: declaración de impuestos que se presenta cada año (generalmente en abril).

**Declaración mensual**: declaración de impuestos que se presenta cada mes (generalmente el día 17 del mes siguiente).

**Dictamen fiscal**: revisión opcional por contador público autorizado.

**Contabilidad electrónica**: conjunto de archivos XML que se envían al SAT con la información contable mensual.

**Nómina**: lista de empleados y sus salarios. El ERP debe generar recibos de nómina (CFDI tipo N).

**Salario mínimo**: monto mínimo legal que un trabajador debe recibir. Varía por zona.

**UMA (Unidad de Medida y Actualización)**: unidad de referencia económica en México, usada para multas y otras obligaciones.

**Subsidio al empleo**: beneficio fiscal que reduce el ISR a pagar por trabajadores de bajos ingresos.

**PTU (Participación de los Trabajadores en las Utilidades)**: derecho de los empleados a recibir una parte de las ganancias de la empresa.

**Aguinaldo**: prestación obligatoria equivalente a 15 días de salario mínimo, pagadera antes del 20 de diciembre.

**Vacaciones**: derecho de los empleados, mínimo 12 días después del primer año.

**IMSS (Instituto Mexicano del Seguro Social)**: institución que brinda seguridad social a los trabajadores.

**INFONAVIT (Instituto del Fondo Nacional de la Vivienda para los Trabajadores)**: instituto que otorga créditos para vivienda.

**SAR (Sistema de Ahorro para el Retiro)**: sistema de pensiones para el retiro.

**FONACOT (Fondo Nacional para el Consumo de los Trabajadores)**: institución que otorga créditos de consumo a los trabajadores.

---


# Apendice A36: Glosario de argot de programadores

Acrónimos y jerga comunes en el mundo del software:

- **PR**: Pull Request (solicitud de cambios en Git).
- **MR**: Merge Request (equivalente en GitLab).
- **CI**: Continuous Integration.
- **CD**: Continuous Deployment / Continuous Delivery.
- **YOLO**: You Only Live Once (usado para "lo voy a hacer sin tests").
- **TODO**: pendiente por hacer.
- **FIXME**: esto necesita arreglo.
- **XXX**: marcador de algo importante.
- **HACK**: solución rápida y no elegante.
- **WIP**: Work In Progress.
- **TL;DR**: Too Long; Didn't Read (resumen).
- **ASAP**: As Soon As Possible.
- **FYI**: For Your Information.
- **IMO/IMHO**: In My (Humble) Opinion.
- **IIRC**: If I Remember Correctly.
- **RTFM**: Read The Fucking Manual.
- **LGTM**: Looks Good To Me.
- **WTF**: What The Fuck (generalmente humorístico).
- **LOL**: Laugh Out Loud.
- **FTW**: For The Win.
- **YMMV**: Your Mileage May Vary.

---


# Apendice A51: Glosario de terminos avanzados de Rust

Para los que ya dominan Rust:

**Async fn in trait**: trait methods que son async. Estable en Rust 1.75+.

**RPIT (Return Position Impl Trait)**: usar `impl Trait` en posición de retorno. Captura closures automáticamente.

**ATCP (Auto Trait Coercion Pass)**: coerción automática de traits como Send/Sync.

**NLL (Non-Lexical Lifetimes)**: el algoritmo de análisis de lifetimes moderno.

**Polonius**: el sucesor de NLL, más preciso, en desarrollo.

**GAT (Generic Associated Types)**: tipos asociados genéricos. Útiles para iteradores que prestan del self.

**Coherence**: regla que dice que para un tipo T y un trait T1, sólo puede haber una implementación de T1 para T en todo el crate.

**Orphan rule**: para implementar un trait de un crate A para un tipo de un crate B, al menos uno de los dos debe ser tuyo.

**Variance**: cómo los lifetimes se relacionan. Invariance, covariance, contravariance.

**HRTB (Higher-Ranked Trait Bounds)**: `for<'a> T: Trait<'a>`. El trait se aplica a todos los lifetimes posibles.

**Specialization**: característica inestable que permite implementaciones más específicas.

---


