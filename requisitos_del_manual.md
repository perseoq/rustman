# Prompt para generar el manual

Genera un archivo en formato Markdown (.md) que contenga un manual completo, progresivo y extremadamente detallado, con una extensión tal que, si se imprimiera, equivaldría a más de 300 páginas (mínimo 100,000 palabras de contenido explicativo, sin contar código ni tablas). El manual debe cubrir tres áreas en orden progresivo:

1. **Lenguaje Rust** (desde cero absoluto hasta nivel intermedio).
2. **Rust con MySQL** (conexión, consultas, pool, transacciones).
3. **Rust + Actix Web + un ORM** (elegir entre Diesel o SeaORM), incluyendo un servidor HTTP con CRUD completo.

El público objetivo son personas que **nunca han programado** en Rust ni han usado bases de datos o servidores web. Cada concepto debe explicarse como si fuera un libro de texto universitario, con ej innumerables ejemplos, analogías cotidianas, código comentado línea por línea y secciones dedicadas a errores típicos.

## Requisitos estructurales y de contenido

### 1. Extensión y profundidad
- El manual debe contener **al menos 50 secciones principales** (H2) y **más de 300 subsecciones** (H3 o H4).
- Cada subsección debe tener un mínimo de **10 párrafos explicativos** (cada párrafo de 5 a 10 oraciones) antes de cualquier bloque de código.
- Incluir **al menos 100 bloques de código completos** (cada uno con más de 15 líneas de código, todos comentados en español).
- Dedicar **al menos 20 páginas equivalentes** a los siguientes temas (repartidos en las partes correspondientes):
  - Ownership, borrowing y lifetimes.
  - Manejo de errores con `Result` y `Option`.
  - Pool de conexiones y transacciones.
  - Middleware, estado compartido y configuración de Actix.
  - Migraciones y modelos en el ORM.
- Incluir **más de 80 ejercicios prácticos** distribuidos uniformemente (con enunciados detallados, pistas y soluciones completas en un anexo al final).
- Agregar un **glosario extenso** con definiciones sencillas de todos los términos técnicos (mínimo 200 entradas).

### 2. Estilo pedagógico obligatorio
- **Explicación larga antes de cada bloque de código** (mínimo 15 oraciones) donde se describa:
  - El problema a resolver.
  - La sintaxis y semántica de cada elemento.
  - Analogías del mundo real (por ejemplo: "una variable `let` es como una caja etiquetada que solo puede contener un tipo de objeto").
  - Posibles errores y cómo evitarlos.
- **Código siempre comentado**: cada línea o grupo de líneas debe tener un comentario en español explicando su propósito.
- **Después del código**: análisis de la salida esperada, explicación de por qué funciona y variantes.
- **Preguntas retóricas** para fomentar la reflexión.
- **Secciones de errores típicos** (al menos una por cada tema principal) con diagnosis de mensajes de error y soluciones.

### 3. Estructura exacta a seguir (puedes expandir cada punto con múltiples subsecciones)

# Manual completo de Rust, bases de datos y backend web

## Tabla de contenidos

### Parte 1: Fundamentos de Rust (desde cero)
1.1 Introducción a Rust  
1.2 Instalación del entorno  
1.3 Tu primer programa: Hola Mundo  
1.4 Variables y mutabilidad  
1.5 Tipos de datos primitivos  
1.6 Operadores  
1.7 Control de flujo  
1.8 Funciones  
1.9 Ownership, borrowing y lifetimes  
 1.9.1 Reglas del ownership  
 1.9.2 Movimiento de propiedad  
 1.9.3 Referencias inmutables y mutables  
 1.9.4 Reglas del borrowing  
 1.9.5 Lifetimes: anotaciones y elisión  
1.10 Tipos compuestos: tuplas, arrays, slices  
1.11 Structs  
 1.11.1 Definición y creación  
 1.11.2 Métodos y funciones asociadas  
 1.11.3 Structs con campos públicos/privados  
1.12 Enums y pattern matching  
 1.12.1 Definición de enums  
 1.12.2 match, if let, while let  
 1.12.3 Option y Result  
1.13 Traits  
 1.13.1 Definición e implementación  
 1.13.2 Traits comunes (Display, Debug, Clone, Copy, PartialEq)  
 1.13.3 Traits como parámetros (impl Trait, trait bounds)  
1.14 Genéricos  
 1.14.1 Funciones y structs genéricas  
 1.14.2 Restricciones con traits  
1.15 Manejo de errores  
 1.15.1 panic!  
 1.15.2 Result<T, E> y ?  
 1.15.3 Combinadores (map, and_then, unwrap_or)  
 1.15.4 Errores personalizados (thiserror, anyhow)  
1.16 Colecciones  
 1.16.1 Vec<T>  
 1.16.2 HashMap<K, V>  
 1.16.3 HashSet<T>  
1.17 Cierres (closures) e iteradores  
 1.17.1 Sintaxis de closures  
 1.17.2 Captura de entorno (Fn, FnMut, FnOnce)  
 1.17.3 Iteradores: map, filter, collect, for_each  
 1.17.4 Iteradores perezosos y consumo  
1.18 Módulos y sistema de archivos  
 1.18.1 mod, pub, use  
 1.18.2 Estructura de directorios  
1.19 Pruebas unitarias y de integración  
 1.19.1 #[test] y assert!  
 1.19.2 Pruebas de integración  
 1.19.3 Test de documentación  
1.20 Ejercicios acumulativos (Parte 1) – 20 ejercicios  
1.21 Soluciones detalladas (Parte 1)

### Parte 2: Rust y MySQL
2.1 Introducción a bases de datos relacionales y MySQL  
 2.1.1 ¿Qué es una base de datos?  
 2.1.2 Tablas, filas, columnas  
 2.1.3 Instalación de MySQL (local y Docker)  
 2.1.4 Creación de base de datos y tablas (SQL básico)  
2.2 Configuración del proyecto con `mysql` crate  
 2.2.1 Dependencias en Cargo.toml  
 2.2.2 Conexión básica: Opciones de conexión  
 2.2.3 Manejo de errores de conexión  
2.3 Consultas SELECT  
 2.3.1 fetch y fetch_all  
 2.3.2 Iteración sobre filas  
 2.3.3 Mapeo a structs (con FromRow)  
2.4 Parámetros preparados y seguridad  
 2.4.1 Inyección SQL y cómo evitarla  
 2.4.2 Uso de parámetros (? y :named)  
2.5 INSERT, UPDATE, DELETE  
 2.5.1 Ejecución de consultas con parámetros  
 2.5.2 Obtención del último ID insertado  
2.6 Pool de conexiones con `r2d2_mysql`  
 2.6.1 ¿Por qué un pool?  
 2.6.2 Configuración y uso  
 2.6.3 Pool como estado compartido  
2.7 Transacciones  
 2.7.1 BEGIN, COMMIT, ROLLBACK  
 2.7.2 Transacciones en Rust  
 2.7.3 Manejo de errores y rollback automático  
2.8 Errores típicos en base de datos  
 2.8.1 Error de conexión  
 2.8.2 Error de tipo  
 2.8.3 Violación de restricciones  
2.9 Ejemplo completo: Gestor de tareas en consola  
 2.9.1 Modelo de datos  
 2.9.2 CRUD desde línea de comandos  
 2.9.3 Casos de uso y pruebas manuales  
2.10 Buenas prácticas  
 2.10.1 Migraciones manuales  
 2.10.2 Manejo de concurrencia  
 2.10.3 Logging de consultas  
2.11 Ejercicios acumulativos (Parte 2) – 20 ejercicios  
2.12 Soluciones detalladas (Parte 2)

### Parte 3: Rust, Actix Web y ORM
3.1 Introducción a Actix Web  
 3.1.1 ¿Qué es un framework web?  
 3.1.2 Actores y sistema de actores  
 3.1.3 Ventajas sobre otros frameworks  
3.2 Configuración del proyecto  
 3.2.1 Dependencias: actix-web, serde, ORM (Diesel o SeaORM)  
 3.2.2 Estructura del proyecto  
 3.2.3 Primer servidor: Hola Mundo HTTP  
3.3 Rutas y handlers  
 3.3.1 Métodos GET, POST, PUT, DELETE  
 3.3.2 Parámetros de ruta y query  
 3.3.3 Extracción de datos de la solicitud (Json, Form, Path)  
 3.3.4 Respuestas (HttpResponse, Json)  
3.4 Estado compartido  
 3.4.1 AppState y Data  
 3.4.2 Inyección de pool de conexiones  
 3.4.3 Mutex y Arc  
3.5 Middleware  
 3.5.1 Logger  
 3.5.2 CORS  
 3.5.3 Manejo global de errores (ErrorHandler)  
 3.5.4 Middleware personalizado  
3.6 Manejo de errores en Actix  
 3.6.1 Error trait y HttpResponse  
 3.6.2 Mapeo de errores de BD a HTTP  
3.7 Introducción al ORM (Diesel como ejemplo)  
 3.7.1 ¿Qué es un ORM?  
 3.7.2 Instalación de Diesel CLI  
 3.7.3 Configuración de la conexión (diesel.toml)  
3.8 Modelos y migraciones  
 3.8.1 Definición de tablas con macros de Diesel  
 3.8.2 Generación de migraciones  
 3.8.3 Relaciones entre tablas  
3.9 CRUD completo con Diesel  
 3.9.1 Crear (INSERT)  
 3.9.2 Leer (SELECT)  
 3.9.3 Actualizar (UPDATE)  
 3.9.4 Eliminar (DELETE)  
 3.9.5 Consultas con filtros y joins  
3.10 Integración de Diesel con Actix  
 3.10.1 Pool de conexiones con r2d2_diesel  
 3.10.2 Handlers que usan el ORM  
 3.10.3 Transacciones en handlers  
3.11 Alternativa: SeaORM  
 3.11.1 Diferencias con Diesel  
 3.11.2 Configuración y migraciones  
 3.11.3 Ejemplo CRUD con SeaORM  
3.12 Buenas prácticas en APIs REST  
 3.12.1 Separación en capas (router, controller, service, repository)  
 3.12.2 Validación de entrada (validator crate)  
 3.12.3 Documentación automática (utoipa)  
 3.12.4 Configuración por entorno (.env)  
3.13 Pruebas  
 3.13.1 Pruebas con curl/Postman  
 3.13.2 Tests de integración con actix-test  
 3.13.3 Mock de base de datos  
3.14 Ejemplo completo: API de gestión de usuarios con tareas  
 3.14.1 Modelos: Usuario, Tarea  
 3.14.2 Rutas: /usuarios, /tareas  
 3.14.3 CRUD completo con relaciones  
 3.14.4 Autenticación básica (JWT)  
 3.14.5 Documentación de la API  
3.15 Despliegue  
 3.15.1 Compilación para producción  
 3.15.2 Dockerización  
 3.15.3 Despliegue en servidor (VPS, Railway, Fly.io)  
3.16 Ejercicios acumulativos (Parte 3) – 30 ejercicios  
3.17 Soluciones detalladas (Parte 3)

### Anexos
A.1 Glosario de términos (más de 200 términos con definiciones sencillas)  
A.2 Soluciones completas de todos los ejercicios (Partes 1, 2 y 3)  
A.3 Recursos y lecturas adicionales  
A.4 Índice alfabético
