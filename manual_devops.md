# Manual de DevOps para los proyectos Rust

> **Despliegue, orquestación, balanceo de carga y automatización de los sistemas construidos en los manuales del repositorio: ERP/CRM (Actix Web), inventarios (Axum) y tickets de soporte (Rocket).**

---

## 1. Introduccion

Cada manual de este repositorio construye una API REST funcional. Pero una API no sirve de nada si solo se ejecuta en tu máquina local. Para que los clientes, los agentes de soporte y los gerentes puedan usar el sistema, la API debe estar desplegada en un servidor, disponible 24/7, escalable ante picos de tráfico, y actualizable sin tiempos de inactividad.

Este manual cubre el ciclo completo de DevOps para los tres proyectos del repositorio:

- **ERP/CRM** (`index.md`) — Actix Web + Diesel
- **Inventarios** (`manual_axum.md`) — Axum + sqlx
- **Tickets** (`manual_rocket.md`) — Rocket + Diesel

Cada proyecto tiene su propio `Dockerfile` y `docker-compose.yml`. En este manual aprenderás a llevarlos a producción.

### 1.1 Arquitectura de despliegue

```
                    ┌──────────────┐
                    │  Load Balancer│
                    │   (nginx)     │
                    └──────┬───────┘
                           │
            ┌──────────────┼──────────────┐
            │              │              │
            ▼              ▼              ▼
    ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
    │  API ERP     │ │  API Axum    │ │  API Rocket  │
    │  (instancia) │ │  (instancia) │ │  (instancia) │
    └──────┬───────┘ └──────┬───────┘ └──────┬───────┘
           │                │                │
           └────────────────┼────────────────┘
                            │
                            ▼
                    ┌──────────────┐
                    │   MariaDB    │
                    │  (replicado) │
                    └──────────────┘
```

### 1.2 Stack DevOps

| Herramienta | Propósito |
|---|---|
| Docker / Podman | Contenerización de las APIs |
| Docker Compose | Orquestación local multi-servicio |
| nginx | Balanceo de carga y proxy inverso |
| Kubernetes | Orquestación en producción |
| Ansible | Automatización de configuración |
| Terraform | Infraestructura como código |

## 2. Contenedores con Docker

Cuando terminas de desarrollar una API en Rust, tienes un binario compilado en `target/release/`. Si solo quieres ejecutarlo en tu máquina, basta con copiar el binario a cualquier carpeta y ejecutarlo. Pero cuando quieres ejecutarlo en un servidor de producción, las cosas se complican: el servidor puede tener otra versión de las librerías del sistema, puede no tener instalado `libmariadb3`, puede tener otro sistema operativo, o puede no tener instalado nada de Rust. Cada vez que despliegas una nueva versión, tienes que asegurarte de que el entorno del servidor sea exactamente el mismo que usaste para compilar.

Los contenedores (Docker) resuelven este problema de raíz: empaquetan el binario junto con **todas sus dependencias** en una imagen. Cuando ejecutas la imagen, el binario se ejecuta exactamente en el mismo entorno en el que fue compilado, sin importar qué sistema operativo tenga el servidor. No importa si el servidor es Debian, Ubuntu, Fedora o Alpine: el contenedor lleva su propio sistema de archivos con las librerías necesarias.

Pero hay un detalle importante: las imágenes de Docker pueden ser muy grandes. La imagen oficial de Rust pesa ~2GB porque incluye el compilador, Cargo, y todo el toolchain. Si usas esa imagen para ejecutar tu API en producción, estás desperdiciando 2GB de espacio en disco y tu servidor tarda más en descargar la imagen cada vez que actualizas.

La solución es la **compilación multi-stage** (multi-stage build). Usas DOS imágenes en el mismo Dockerfile:

1. **Primera etapa (builder)**: usas la imagen grande de Rust (`rust:1.82`) para compilar el binario. Esta etapa solo existe durante la compilación; su resultado es el binario compilado.
2. **Segunda etapa (runtime)**: usas una imagen mínima (`debian:bookworm-slim`, ~80MB) y solo copias el binario compilado de la primera etapa. También instalas las librerías necesarias para ejecutar el binario (`libmariadb3` para conectar a MariaDB).

El resultado: la imagen final pesa ~80MB en lugar de ~2GB, y solo contiene lo necesario para ejecutar la API, no para compilarla.

### 2.1 Dockerfile multi-stage para API Rust

El siguiente Dockerfile sigue el patrón multi-stage. Lo usaremos para la API del ERP (Actix Web), pero es prácticamente idéntico para las APIs de Axum y Rocket, cambiando solo el nombre del binario.

```dockerfile
# ============================================
# ETAPA 1: COMPILACIÓN (builder)
# Usa la imagen completa de Rust con el compilador
# ============================================
FROM rust:1.82 AS builder

# Directorio de trabajo dentro del contenedor
WORKDIR /app

# Copiar todo el proyecto al contenedor
# (Cargo.toml, Cargo.lock, src/, migrations/, .env, etc.)
COPY . .

# Compilar en modo release
# --release activa optimizaciones del compilador (el binario es más rápido)
# --bin api_erp especifica qué binario compilar (definido en Cargo.toml)
RUN cargo build --release --bin api_erp

# ============================================
# ETAPA 2: EJECUCIÓN (runtime)
# Usa una imagen mínima de Debian
# ============================================
FROM debian:bookworm-slim

# Instalar las librerías necesarias en tiempo de ejecución
# libmariadb3: necesaria para conectar a MariaDB desde Diesel/sqlx
# ca-certificates: necesaria para conexiones HTTPS
RUN apt-get update && \
    apt-get install -y libmariadb3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copiar el binario compilado desde la etapa builder
# --from=builder indica que el archivo viene de la primera etapa
COPY --from=builder /app/target/release/api_erp /usr/local/bin/api_erp

# Puerto que expone la API (debe coincidir con el que usa el código)
EXPOSE 8080

# Comando que se ejecuta al iniciar el contenedor
CMD ["api_erp"]
```

**Análisis línea por línea**:

- `FROM rust:1.82 AS builder`: inicia la primera etapa. La imagen `rust:1.82` incluye el compilador `rustc`, Cargo, y todas las librerías estándar. `AS builder` le da un nombre a esta etapa para referenciarla desde la segunda etapa.

- `WORKDIR /app`: establece el directorio de trabajo dentro del contenedor. Todas las instrucciones siguientes (COPY, RUN, CMD) se ejecutan en este directorio. Si el directorio no existe, Docker lo crea automáticamente.

- `COPY . .`: copia todo el contenido del directorio actual (donde está el Dockerfile) al directorio `/app` dentro del contenedor. Esto incluye `Cargo.toml`, `Cargo.lock`, `src/`, `migrations/`, etc. En producción, añadirías un archivo `.dockerignore` para evitar copiar la carpeta `target/` (que puede pesar gigabytes).

- `RUN cargo build --release --bin api_erp`: ejecuta la compilación dentro del contenedor. `--release` activa las optimizaciones del compilador (código más rápido, compilación más lenta). `--bin api_erp` compila solo el binario especificado, no todos los binarios del proyecto. La primera compilación descarga todas las dependencias y puede tardar 5-10 minutos; las siguientes usan la caché.

- `FROM debian:bookworm-slim`: inicia la segunda etapa. Esta imagen pesa ~80MB, comparado con los ~2GB de `rust:1.82`. Contiene solo lo básico para ejecutar un binario de Linux.

- `RUN apt-get install -y libmariadb3`: instala el cliente de MariaDB. El binario compilado enlaza dinámicamente con `libmariadb.so.3`. Si esta librería no está presente, el binario falla al iniciar con el error `error while loading shared libraries: libmariadb.so.3`. También instalamos `ca-certificates` para que la API pueda hacer conexiones HTTPS a servicios externos.

- `rm -rf /var/lib/apt/lists/*`: elimina la caché de paquetes de apt. Esto reduce el tamaño de la imagen final en ~20MB. Es una buena práctica en Dockerfiles: cada instrucción `RUN` crea una capa; si no limpias la caché, esta se guarda en la imagen y la hace más pesada.

- `COPY --from=builder /app/target/release/api_erp /usr/local/bin/api_erp`: copia el binario compilado de la primera etapa a la segunda. `--from=builder` indica que el archivo viene de la etapa llamada `builder`. La ruta de origen es dentro de la etapa builder (`/app/target/release/api_erp`). La ruta de destino es `/usr/local/bin/api_erp`, que está en el PATH del sistema.

- `EXPOSE 8080`: documenta que el contenedor escucha en el puerto 8080. Esto no publica el puerto (para eso necesitas `-p` en `docker run` o `ports:` en docker-compose), pero sirve como documentación y es necesario si usas redes de Docker.

- `CMD ["api_erp"]`: especifica el comando que se ejecuta al iniciar el contenedor. `CMD` se escribe como un array JSON para evitar problemas con el shell. Si escribieras `CMD api_erp` (sin array), Docker lo ejecutaría con un shell intermedio, lo que puede causar problemas con señales (Ctrl+C).

### 2.2 Errores tipicos

**Error 1: olvidar instalar libmariadb3**.

```dockerfile
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/api_erp /usr/local/bin/api_erp
CMD ["api_erp"]
```

La imagen compila pero al ejecutar el contenedor:
```
api_erp: error while loading shared libraries: libmariadb.so.3:
  cannot open shared object file: No such file or directory
```

Solución: instalar `libmariadb3` en la etapa de ejecución.

**Error 2: compilar en debug en lugar de release**.

```dockerfile
RUN cargo build  # falta --release
```

El binario compilado pesa más y es más lento. Además, en un entorno de producción, las optimizaciones de release mejoran el rendimiento notablemente. Solución: usar `cargo build --release`.

**Error 3: copiar target/ del host al contenedor**.

Si el directorio `target/` existe en el host y no está en `.dockerignore`, Docker lo copia al contenedor en el paso `COPY . .`. Esto hace que la compilación falle porque los artefactos del host no son compatibles con el contenedor. Solución: añadir `target/` al `.dockerignore`.

### 2.3 .dockerignore

```dockerignore
target/
.git/
*.md
Dockerfile
docker-compose.yml
```

Este archivo evita que Docker copie archivos innecesarios al contenedor. La compilación es más rápida y la imagen final es más pequeña.

### 2.2 Dockerfile para Axum (sqlx)

```dockerfile
FROM rust:1.82 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin api_axum_inventarios

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libmariadb3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/api_axum_inventarios /usr/local/bin/api
EXPOSE 8080
CMD ["api"]
```

**Diferencia**: sqlx también necesita `libmariadb3` (es el mismo cliente que usa Diesel).

### 2.3 Dockerfile para Rocket

```dockerfile
FROM rust:1.82 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin api_rocket_tickets

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libmariadb3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/api_rocket_tickets /usr/local/bin/api
EXPOSE 8000
CMD ["api"]
```

### 2.4 .dockerignore

```dockerignore
target/
.git/
*.md
Dockerfile
docker-compose.yml
```

### 2.5 Errores típicos

**Error 1: olvidar libmariadb3**.

La imagen compila pero al ejecutar falla con:
```
error: error while loading shared libraries: libmariadb.so.3: cannot open shared object file
```

Solución: instalar `libmariadb3` en la etapa final.

**Error 2: compilar en debug en lugar de release**.

```dockerfile
RUN cargo build  # falta --release
```

El binario en debug es más grande y más lento. Solución: `cargo build --release`.

**Error 3: copiar target/ desde el host**.

```dockerfile
COPY . .
# target/ del host puede tener compilaciones previas incompatibles
```

Solución: añadir `target/` al `.dockerignore`.

## 3. Docker Compose multi-proyecto

Cuando tienes varias APIs que dependen del mismo servicio (MariaDB), el enfoque más simple es usar Docker Compose. Cada API se define como un **servicio** dentro del mismo archivo `docker-compose.yml`. Docker Compose se encarga de crear una red interna donde los servicios pueden comunicarse por nombre (no por IP), iniciarlos en el orden correcto, y gestionar los volúmenes de datos.

El archivo que vamos a construir unifica los tres proyectos del repositorio: el ERP (Actix Web), el sistema de inventarios (Axum) y el sistema de tickets (Rocket). Cada uno tiene su propia base de datos dentro de la misma instancia de MariaDB, y cada uno expone su API en un puerto diferente (8080, 8081, 8000). Un balanceador nginx recibe las peticiones en el puerto 80 y las distribuye según el dominio.

La clave para entender Docker Compose son las **dependencias entre servicios**. MariaDB debe iniciarse primero porque las APIs necesitan la base de datos para funcionar. Pero "iniciarse" no significa "estar lista": MariaDB puede estar aceptando conexiones TCP antes de haber terminado de inicializar sus tablas. Por eso usamos `healthcheck`, que es una prueba que Docker ejecuta periódicamente para determinar si el servicio está realmente listo. Las APIs esperan a que el healthcheck de MariaDB pase antes de iniciarse.

### 3.1 docker-compose.yml unificado

```yaml
version: "3.9"

services:
  # ====== MariaDB compartida ======
  # Una sola instancia con 3 bases de datos: erp_crm, inventarios, tickets_soporte
  mysql:
    image: mariadb:11
    environment:
      # ${VAR:-default}: usa la variable VAR, o "default" si no está definida
      MYSQL_ROOT_PASSWORD: ${MYSQL_ROOT_PASSWORD:-secret}
    ports:
      - "127.0.0.1:3306:3306"
    volumes:
      - mysql_data:/var/lib/mysql
      - ./sql/init_all.sql:/docker-entrypoint-initdb.d/init.sql
    healthcheck:
      test: ["CMD", "mysqladmin", "ping", "-h", "localhost"]
      interval: 10s
      timeout: 5s
      retries: 5

  # ====== API ERP (Actix Web) ======
  api-erp:
    build: ./proyecto_api/api_diesel
    depends_on:
      mysql:
        condition: service_healthy
    environment:
      DATABASE_URL: mysql://root:${MYSQL_ROOT_PASSWORD:-secret}@mysql:3306/erp_crm
      JWT_SECRET: ${JWT_SECRET:-cambiar-en-produccion}
      RUST_LOG: info
    ports: ["8080:8080"]

  # ====== API Inventarios (Axum) ======
  api-axum:
    build: ./proyecto_api_axum_inventarios
    depends_on:
      mysql:
        condition: service_healthy
    environment:
      DATABASE_URL: mysql://root:${MYSQL_ROOT_PASSWORD:-secret}@mysql:3306/inventarios
      JWT_SECRET: ${JWT_SECRET:-cambiar-en-produccion}
      RUST_LOG: info
    ports: ["8081:8080"]

  # ====== API Tickets (Rocket) ======
  api-rocket:
    build: ./proyecto_api_rocket_tickets
    depends_on:
      mysql:
        condition: service_healthy
    environment:
      DATABASE_URL: mysql://root:${MYSQL_ROOT_PASSWORD:-secret}@mysql:3306/tickets_soporte
      RUST_LOG: info
    ports: ["8000:8000"]

  # ====== Nginx Load Balancer ======
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/conf.d/default.conf
    depends_on:
      - api-erp
      - api-axum
      - api-rocket

volumes:
  mysql_data:
```

**Análisis línea por línea**:

- `version: "3.9"`: versión del formato de Docker Compose. La versión 3.9 es la más reciente estable. Determina qué características están disponibles (healthcheck, deploy, secrets, etc.).

- `services:`: cada bloque dentro de services define un contenedor. Docker Compose crea una red interna donde los servicios se resuelven por nombre (mysql, api-erp, api-axum, etc.).

- `image: mariadb:11`: usa la imagen oficial de MariaDB versión 11. Docker descarga la imagen si no está en caché.

- `${MYSQL_ROOT_PASSWORD:-secret}`: sintaxis de variable de entorno con valor por defecto. Si la variable `MYSQL_ROOT_PASSWORD` no está definida en el shell, se usa `secret`. Esto permite cambiar la contraseña sin modificar el archivo YAML.

- `ports: ["127.0.0.1:3306:3306"]`: expone el puerto 3306 del contenedor en el puerto 3306 del host, pero solo en la interfaz de loopback (127.0.0.1). Esto significa que MariaDB solo es accesible desde el propio servidor, no desde otros equipos de la red. Es una medida de seguridad.

- `volumes: - mysql_data:/var/lib/mysql`: monta un volumen persistente en `/var/lib/mysql`, que es donde MariaDB guarda sus datos. Sin este volumen, los datos se pierden al reiniciar el contenedor. `mysql_data` es un volumen nombrado que Docker gestiona.

- `volumes: - ./sql/init_all.sql:/docker-entrypoint-initdb.d/init.sql`: monta un archivo SQL en la carpeta de inicialización de MariaDB. MariaDB ejecuta automáticamente cualquier archivo `.sql` en `/docker-entrypoint-initdb.d/` la primera vez que se inicia. Esto crea las bases de datos y tablas automáticamente.

- `healthcheck:`: define una prueba que Docker ejecuta cada 10 segundos (`interval`) para verificar que MariaDB está lista. `mysqladmin ping` conecta a MariaDB y verifica que responde. Docker espera hasta 5 intentos (`retries`) antes de marcar el servicio como no saludable.

- `depends_on: mysql: condition: service_healthy`: las APIs esperan a que MariaDB esté saludable antes de iniciarse. Sin esto, las APIs intentarían conectarse antes de que MariaDB esté lista y fallarían con "Connection refused".

- `build: ./proyecto_api/api_diesel`: le dice a Docker Compose que construya la imagen usando el Dockerfile en ese directorio. Alternativamente, podrías usar `image:` si la imagen ya está publicada en un registro.

- `environment: DATABASE_URL: mysql://...@mysql:3306/erp_crm`: nota que el host es `mysql` (el nombre del servicio), no `127.0.0.1` ni `localhost`. Docker Compose crea una red interna donde los servicios se resuelven por nombre.

- `ports: ["8081:8080"]`: el puerto 8081 del host se mapea al puerto 8080 del contenedor de Axum. Como los tres contenedores usan el puerto 8080 internamente, se mapean a puertos diferentes en el host (8080, 8081, 8000).

### 3.2 Errores tipicos con Docker Compose

**Error 1: olvidar el healthcheck**.

Sin healthcheck, las APIs intentan conectarse a MariaDB antes de que esté lista:
```
Error: Connection refused (os error 111)
```

Solución: añadir `healthcheck` a MariaDB y `condition: service_healthy` en las APIs.

**Error 2: puerto duplicado en el host**.

```yaml
api-erp:
  ports: ["8080:8080"]
api-axum:
  ports: ["8080:8080"]  # error: puerto 8080 ya está en uso
```

Mensaje: `Error: address already in use`. Solución: usar puertos diferentes en el host (8080, 8081, 8082).

**Error 3: usar localhost en DATABASE_URL**.

```yaml
environment:
  DATABASE_URL: mysql://root:secret@localhost:3306/erp_crm
  #                                      ^^^^^^^^^
  #                                      Esto buscaría MariaDB en el host, no en el contenedor
```

Dentro de un contenedor, `localhost` es el propio contenedor, no el host ni otros contenedores. Solución: usar el nombre del servicio (`mysql`) como host.

### 3.2 Script SQL unificado (init_all.sql)

```sql
-- =============================================
-- Inicialización de las 3 bases de datos
-- =============================================

-- ERP/CRM
CREATE DATABASE IF NOT EXISTS erp_crm CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE erp_crm;
CREATE TABLE IF NOT EXISTS clientes (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(150) NOT NULL,
    rfc VARCHAR(13) NOT NULL UNIQUE,
    email VARCHAR(150),
    credito DECIMAL(12,2) NOT NULL DEFAULT 0,
    activo BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- ... (más tablas del ERP)

-- Inventarios
CREATE DATABASE IF NOT EXISTS inventarios CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE inventarios;
CREATE TABLE IF NOT EXISTS productos (
    sku VARCHAR(30) PRIMARY KEY,
    nombre VARCHAR(150) NOT NULL,
    precio DECIMAL(12,2) NOT NULL DEFAULT 0,
    stock_minimo DECIMAL(12,2) NOT NULL DEFAULT 5,
    activo BOOLEAN NOT NULL DEFAULT TRUE
);
-- ... (más tablas de inventarios)

-- Tickets de soporte
CREATE DATABASE IF NOT EXISTS tickets_soporte CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE tickets_soporte;
CREATE TABLE IF NOT EXISTS clientes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(150) NOT NULL,
    email VARCHAR(200) NOT NULL
);
-- ... (más tablas de tickets)
```

## 4. Balanceo de carga con nginx

Cuando tienes una sola instancia de la API ejecutándose y llegan 1,000 peticiones por segundo, esa instancia se satura. El CPU llega al 100%, las respuestas tardan segundos en lugar de milisegundos, y eventualmente algunas peticiones fallan con timeout. La solución no es hacer la API más rápida (eso siempre ayuda, pero tiene un límite). La solución es ejecutar **múltiples instancias** de la misma API y distribuir las peticiones entre ellas. Esto se llama **escalamiento horizontal**.

El componente que distribuye las peticiones se llama **balanceador de carga** (load balancer). Su trabajo es simple: recibe una petición, elige un servidor backend del grupo, y reenvía la petición a ese servidor. Si un servidor falla, el balanceador deja de enviarle peticiones y las distribuye entre los que siguen funcionando. Si necesitas más capacidad, agregas más servidores al grupo sin modificar el balanceador.

nginx es el balanceador de carga más usado en el mundo. Se configura con bloques `upstream` (que definen los grupos de servidores) y bloques `server` (que definen cómo se reciben las peticiones). Es rápido, estable y consume pocos recursos.

### 4.1 Configuración de nginx

Vamos a configurar nginx para balancear las peticiones entre los tres proyectos del repositorio. Cada proyecto tiene su propio dominio (`erp.mi-empresa.mx`, `inventarios.mi-empresa.mx`, `tickets.mi-empresa.mx`), y nginx dirige cada dominio al grupo de servidores correspondiente.

```nginx
# nginx/nginx.conf

# Grupo de servidores para el ERP (3 instancias)
upstream erp_backend {
    # balanceo round-robin: cada petición va a una instancia diferente
    server api-erp:8080;
    server api-erp-2:8080;
    server api-erp-3:8080;
}

# Grupo de servidores para inventarios (2 instancias)
upstream axum_backend {
    server api-axum:8080;
    server api-axum-2:8080;
}

# Grupo de servidores para tickets (1 instancia, sin balanceo)
upstream rocket_backend {
    server api-rocket:8000;
}

# Servidor virtual para erp.mi-empresa.mx
server {
    listen 80;
    server_name erp.mi-empresa.mx;

    location / {
        proxy_pass http://erp_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}

# Servidor virtual para inventarios.mi-empresa.mx
server {
    listen 80;
    server_name inventarios.mi-empresa.mx;

    location / {
        proxy_pass http://axum_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}

# Servidor virtual para tickets.mi-empresa.mx
server {
    listen 80;
    server_name tickets.mi-empresa.mx;

    location / {
        proxy_pass http://rocket_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```

**Análisis línea por línea**:

- `upstream erp_backend {`: define un grupo de servidores backends. El nombre `erp_backend` se usa en `proxy_pass` para referenciar el grupo. Puedes tener tantos grupos como necesites (uno por proyecto).

- `server api-erp:8080;`: cada `server` dentro del bloque `upstream` es una dirección de backend. `api-erp` es el nombre del servicio en Docker Compose; Docker resuelve este nombre a la IP del contenedor automáticamente. nginx distribuye las peticiones entre los servidores del grupo usando round-robin (primero api-erp, luego api-erp-2, luego api-erp-3, y vuelve a empezar).

- `server api-erp-2:8080;`: segunda instancia del ERP. En Docker Compose, cuando escalas con `--scale api-erp=3`, Docker crea contenedores con sufijos `api-erp-1`, `api-erp-2`, `api-erp-3`. nginx necesita saber las direcciones de cada uno.

- `server { listen 80; server_name erp.mi-empresa.mx; }`: define un servidor virtual que escucha en el puerto 80 y solo acepta peticiones para el dominio `erp.mi-empresa.mx`. Si alguien visita `inventarios.mi-empresa.mx`, este bloque no lo procesa (lo procesará el bloque correspondiente).

- `location / { proxy_pass http://erp_backend; }`: todas las peticiones que lleguen a este servidor virtual se reenvían al grupo `erp_backend`. nginx elige qué servidor del grupo usar según el algoritmo de balanceo.

- `proxy_set_header Host $host;`: preserva el encabezado Host original de la petición. Sin esto, la API recibe `localhost` como Host y puede generar enlaces incorrectos.

- `proxy_set_header X-Real-IP $remote_addr;`: pasa la IP real del cliente a la API. Sin esto, la API ve la IP de nginx (`127.0.0.1`) como dirección de origen.

- `proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;`: añade la IP del cliente a la cadena de IPs por las que ha pasado la petición. Útil para logging y auditoría.

### 4.2 Escalamiento con Docker Compose

```bash
# Escalar el ERP a 3 instancias
docker compose up -d --scale api-erp=3

# Escalar Axum a 2 instancias
docker compose up -d --scale api-axum=2
```

Cuando ejecutas `--scale api-erp=3`, Docker Compose crea 3 contenedores con nombres `api-erp-1`, `api-erp-2`, `api-erp-3`. nginx distribuye las peticiones entre ellos automáticamente.

### 4.3 Errores típicos

**Error 1: olvidar `proxy_set_header Host`**.

```nginx
location / {
    proxy_pass http://erp_backend;
    # falta: proxy_set_header Host $host;
}
```

Sin este header, la API recibe `Host: localhost`. Si la API genera enlaces absolutos (por ejemplo, en un email de confirmación), los enlaces apuntarán a `localhost` en lugar del dominio real. Solución: añadir `proxy_set_header Host $host;`.

**Error 2: puerto incorrecto en upstream**.

```nginx
upstream erp_backend {
    server api-erp:80;  # error: la API escucha en 8080, no en 80
}
```

La API dentro del contenedor escucha en el puerto 8080 (definido con `EXPOSE 8080` en el Dockerfile). nginx debe conectar a ese puerto, no al 80. Solución: el puerto en `upstream` debe coincidir con el `EXPOSE` del Dockerfile.

**Error 3: nombre de servidor incorrecto en nginx**.

Si nginx recibe una petición para un dominio que no coincide con ningún `server_name`, devuelve el primer bloque `server` (el que está primero en el archivo). Esto puede causar que peticiones a `tickets.mi-empresa.mx` terminen en el ERP. Solución: verificar que los dominios DNS apunten correctamente a la IP del servidor.

## 5. Orquestación con Kubernetes

Docker Compose es excelente para desarrollo y entornos pequeños. Pero cuando tienes múltiples servidores, necesitas actualizaciones sin caída del servicio, escalamiento automático según la carga, balanceo de carga integrado, y gestión centralizada de secretos. Para eso existe Kubernetes.

Kubernetes (K8s) es un orquestador de contenedores. Su trabajo es asegurar que el estado deseado de tu aplicación coincida con el estado real. Si declaras "3 instancias de la API", Kubernetes se asegura de que siempre haya 3 instancias ejecutándose. Si una instancia falla, Kubernetes crea una nueva. Si el servidor donde se ejecutan se cae, Kubernetes las migra a otro servidor. Si la carga aumenta, Kubernetes puede crear más instancias automáticamente.

Kubernetes introduce varios conceptos nuevos que debes entender:

- **Pod**: la unidad más pequeña en Kubernetes. Un pod contiene uno o más contenedores. Normalmente, un pod contiene un solo contenedor de la API.
- **Deployment**: gestiona el ciclo de vida de los pods. Define cuántos pods deben ejecutarse, qué imagen usan, qué puertos exponen, y cómo actualizarlos.
- **Service**: expone los pods como un servicio de red. Proporciona una IP estable y balanceo de carga entre los pods.
- **ConfigMap**: almacena configuración no sensible (variables de entorno, archivos de configuración).
- **Secret**: almacena información sensible (contraseñas, tokens JWT, claves API).
- **Ingress**: expone servicios HTTP al exterior, con enrutamiento por dominio y ruta.

### 5.1 Namespace y ConfigMap

Los recursos de Kubernetes se organizan en **namespaces** (espacios de nombres), que son como carpetas que agrupan recursos relacionados.

```yaml
# k8s/namespace.yaml
# Define un namespace para el entorno de producción
apiVersion: v1
kind: Namespace
metadata:
  name: erp-produccion
```

```yaml
# k8s/configmap.yaml
# Almacena configuración no sensible de la API
apiVersion: v1
kind: ConfigMap
metadata:
  name: erp-config
  namespace: erp-produccion
data:
  RUST_LOG: "info"                             # Nivel de logging
  DATABASE_URL: "mysql://root:${MYSQL_PASSWORD}@mysql-svc:3306/erp_crm"
```

**Análisis línea por línea**:

- `apiVersion: v1`: versión de la API de Kubernetes. Cada tipo de recurso tiene su propia versión (`v1` para ConfigMap, `apps/v1` para Deployment, `networking.k8s.io/v1` para Ingress).

- `kind: Namespace`: el tipo de recurso que estamos creando. Kubernetes tiene decenas de tipos: Pod, Service, Deployment, ConfigMap, Secret, Ingress, etc.

- `metadata.name: erp-produccion`: nombre del namespace. Todos los recursos que creemos con `namespace: erp-produccion` estarán aislados de otros namespaces.

- `data`: contiene los pares clave-valor de configuración. Puedes tener tantas entradas como necesites. Cada entrada se convierte en una variable de entorno en el pod.

### 5.2 Deployment de la API

El Deployment es el recurso más importante. Define el estado deseado de la aplicación.

```yaml
# k8s/deployment-api.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-erp
  namespace: erp-produccion
spec:
  # Número de réplicas (instancias) que queremos ejecutar
  replicas: 3
  # El selector identifica qué pods gestiona este deployment
  selector:
    matchLabels:
      app: api-erp
  # Plantilla del pod
  template:
    metadata:
      labels:
        app: api-erp
    spec:
      containers:
      - name: api-erp
        image: registry.mi-empresa.mx/api-erp:latest
        ports:
        - containerPort: 8080
        # Variables de entorno desde ConfigMap y Secrets
        env:
        - name: DATABASE_URL
          valueFrom:
            configMapKeyRef:
              name: erp-config
              key: DATABASE_URL
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: erp-secrets
              key: jwt_secret
        # Límites de recursos
        resources:
          requests:
            memory: "128Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        # Health check
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 10
```

**Análisis línea por línea**:

- `apiVersion: apps/v1`: los Deployments usan la API `apps/v1`. Las versiones antiguas usaban `extensions/v1beta1`, pero están obsoletas.

- `replicas: 3`: el número de instancias (pods) que Kubernetes debe mantener ejecutándose. Si un pod falla, Kubernetes crea uno nuevo. Si escalas a 5, Kubernetes crea 2 pods adicionales.

- `selector.matchLabels.app: api-erp`: Kubernetes etiqueta los pods con `app: api-erp` y usa esta etiqueta para saber qué pods pertenecen a este deployment. Sin esto, Kubernetes no sabría cuántos pods está gestionando.

- `image: registry.mi-empresa.mx/api-erp:latest`: la imagen del contenedor. Debe estar en un registro accesible para Kubernetes. Cuando actualizas la imagen, Kubernetes hace un rolling update: reemplaza los pods de a uno, asegurando que siempre haya instancias disponibles.

- `resources.requests.memory: "128Mi"`: la cantidad mínima de memoria que Kubernetes debe garantizar para este pod. Si un nodo no tiene al menos 128MB libres, Kubernetes no programa el pod allí. Esto evita que los pods compartan recursos insuficientes.

- `resources.limits.memory: "512Mi"`: la cantidad máxima de memoria que el pod puede usar. Si el pod supera este límite, Kubernetes lo reinicia. Esto evita que un pod consuma toda la memoria del servidor.

- `livenessProbe`: Kubernetes ejecuta esta prueba periódicamente para verificar que el pod está vivo. Si la prueba falla 3 veces seguidas, Kubernetes reinicia el pod. El `initialDelaySeconds: 5` evita que la prueba comience antes de que la API haya terminado de iniciarse.

### 5.3 Errores típicos

**Error 1: imagen no accesible**.

```yaml
image: api-erp:latest  # falta el registro (registry.mi-empresa.mx/)
```

Kubernetes intenta descargar la imagen y falla con:
```
Failed to pull image "api-erp:latest": image not found
```

Solución: incluir la URL completa del registro de imágenes.

**Error 2: memoria insuficiente**.

```yaml
resources:
  limits:
    memory: "32Mi"  # demasiado bajo para una API Rust con Diesel
```

La API consume más de 32MB y Kubernetes la reinicia constantemente. Síntoma: el pod entra en estado `CrashLoopBackOff`. Solución: monitorear el consumo real con `kubectl top pod` y ajustar los límites.

**Error 3: olvidar el ConfigMap**.

Si el Deployment referencia un ConfigMap que no existe:
```yaml
env:
- name: DATABASE_URL
  valueFrom:
    configMapKeyRef:
      name: config-inexistente  # este ConfigMap no existe
```

El pod se crea pero no puede iniciarse porque la variable `DATABASE_URL` no está definida. Solución: crear el ConfigMap antes del Deployment, o verificar con `kubectl get configmap`.

### 5.4 Service y balanceo interno

```yaml
# k8s/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: api-erp-svc
  namespace: erp-produccion
spec:
  selector:
    app: api-erp
  ports:
  - port: 80
    targetPort: 8080
  type: ClusterIP
```

**Análisis**: el Service expone los pods del Deployment como un servicio de red estable. Los pods pueden morir y renacer con IPs diferentes, pero la IP del Service no cambia. `selector.app: api-erp` indica qué pods incluir en el servicio. `targetPort: 8080` redirige del puerto 80 del Service al puerto 8080 del pod. `type: ClusterIP` hace que el Service solo sea accesible dentro del cluster de Kubernetes.

### 5.3 Service (balanceo interno)

```yaml
# k8s/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: api-erp-svc
  namespace: erp-produccion
spec:
  selector:
    app: api-erp
  ports:
  - port: 80
    targetPort: 8080
  type: ClusterIP
```

### 5.4 Ingress (balanceo externo)

```yaml
# k8s/ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: erp-ingress
  namespace: erp-produccion
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
spec:
  rules:
  - host: erp.mi-empresa.mx
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: api-erp-svc
            port:
              number: 80
```

## 6. Ansible para automatización

Docker Compose y Kubernetes gestionan los contenedores una vez que están en el servidor. Pero alguien tiene que configurar el servidor primero: instalar Docker, instalar nginx, copiar los archivos de configuración, gestionar los certificados SSL, y asegurarse de que todo esté funcionando. Hacer esto a mano en cada servidor es tedioso y propenso a errores. Un servidor puede tener un paquete desactualizado, otro puede tener una configuración diferente.

Ansible resuelve esto permitiéndote describir la configuración del servidor como **código**. Escribes un "playbook" que dice "instala Docker, configura nginx, copia estos archivos", y Ansible lo ejecuta en todos los servidores automáticamente. Si un servidor ya tiene Docker instalado, Ansible lo detecta y no lo reinstala. Si un servidor tiene una versión incorrecta, Ansible la actualiza.

Ansible funciona sin necesidad de instalar agentes en los servidores: se conecta por SSH y ejecuta los comandos directamente. Esto lo hace más simple que herramientas como Puppet o Chef, que requieren un agente instalado en cada servidor.

### 6.1 Estructura del proyecto Ansible

```
ansible/
├── inventory.yml           ← Lista de servidores (producción, staging)
├── ansible.cfg             ← Configuración global de Ansible
├── playbooks/
│   ├── deploy.yml          ← Despliega la API en los servidores
│   └── docker.yml          ← Instala Docker desde cero
└── roles/
    ├── docker/
    │   └── tasks/
    │       └── main.yml
    ├── api-erp/
    │   └── tasks/
    │       └── main.yml
    └── nginx/
        └── tasks/
            └── main.yml
```

**Análisis de la estructura**:

- `inventory.yml`: define qué servidores existen y cómo conectarse a ellos. Cada servidor tiene una IP, un usuario SSH, y puede pertenecer a uno o más grupos (production, staging).

- `ansible.cfg`: configuración global como la ruta al inventario, el usuario SSH por defecto, y si Ansible debe verificar la clave del host.

- `playbooks/`: contiene los playbooks, que son archivos YAML que describen los pasos a ejecutar.

- `roles/`: organiza las tareas por función. Cada rol tiene su propia carpeta con `tasks/main.yml`.

### 6.2 Inventory

El inventory define los servidores donde Ansible ejecutará los playbooks.

```yaml
# inventory.yml
all:
  children:
    production:
      hosts:
        servidor-1:
          ansible_host: 192.168.1.100
          ansible_user: deploy
    staging:
      hosts:
        servidor-staging:
          ansible_host: 192.168.1.200
          ansible_user: deploy
```

**Análisis**:

- `production` y `staging`: grupos de servidores. Puedes ejecutar un playbook solo en staging con `ansible-playbook -i inventory.yml deploy.yml --limit staging`.

- `ansible_host`: IP o dominio del servidor.

- `ansible_user`: usuario SSH. Ansible se conecta como este usuario y luego usa `become: yes` para ejecutar comandos como root.

### 6.3 Playbook de despliegue

Este playbook despliega la API del ERP en el servidor de producción.

```yaml
# playbooks/deploy.yml
---
- name: Desplegar API ERP en producción
  hosts: production
  become: yes
  tasks:
    - name: Crear directorio de la aplicación
      file:
        path: /opt/erp
        state: directory
        mode: '0755'

    - name: Copiar docker-compose.yml al servidor
      copy:
        src: ../docker-compose.yml
        dest: /opt/erp/docker-compose.yml

    - name: Iniciar contenedores con Docker Compose
      docker_compose:
        project_src: /opt/erp
        state: present
        restarted: yes

    - name: Esperar a que la API responda
      uri:
        url: http://localhost:8080/health
        status_code: 200
      register: health_check
      retries: 10
      delay: 5

    - name: Configurar nginx como proxy inverso
      template:
        src: ../nginx/nginx.conf.j2
        dest: /etc/nginx/sites-enabled/erp.conf
      notify: reiniciar nginx
```

**Análisis línea por línea**:

- `hosts: production`: ejecuta estas tareas solo en los servidores del grupo `production`.

- `become: yes`: ejecuta los comandos como root (a través de sudo). Necesario para instalar paquetes y modificar archivos del sistema.

- `file: path: /opt/erp state: directory`: crea el directorio `/opt/erp` si no existe. Es como `mkdir -p /opt/erp`.

- `copy: src: ../docker-compose.yml dest: /opt/erp/docker-compose.yml`: copia el archivo docker-compose.yml del repositorio local al servidor.

- `docker_compose: project_src: /opt/erp state: present restarted: yes`: ejecuta `docker compose up -d` en el directorio. Si los contenedores ya están corriendo, los reinicia.

- `uri: url: http://localhost:8080/health status_code: 200 retries: 10 delay: 5`: espera a que la API responda 200 OK. Lo intenta hasta 10 veces con 5 segundos de espera entre intentos.

### 6.4 Errores típicos

**Error 1: SSH no configurado**.

Ansible necesita acceso SSH al servidor. Si el usuario `deploy` no tiene las claves SSH configuradas:
```
fatal: [servidor-1]: UNREACHABLE! => {"msg": "Permission denied (publickey)"}
```

Solución: configurar `ssh-copy-id deploy@servidor-1` antes de ejecutar Ansible.

**Error 2: olvidar `become: yes`**.

Si intentas instalar paquetes sin permisos de root:
```
fatal: [servidor-1]: FAILED! => {"msg": "This command requires root privileges"}
```

Solución: añadir `become: yes` al playbook o a la tarea.

## 7. Terraform para infraestructura

Ansible configura el software dentro del servidor (Docker, nginx). Pero alguien tiene que crear el servidor primero: contratar la instancia EC2 en AWS, configurar la red, abrir los puertos en el firewall. Hacer esto a mano desde la consola web de AWS es fácil la primera vez, pero imposible de repetir consistentemente.

Terraform resuelve esto permitiéndote describir tu infraestructura como **código**. Escribes "quiero una instancia EC2 t3.medium en us-west-2 con un security group que abra los puertos 80 y 8080", y Terraform lo crea. Si necesitas cambiar algo, modificas el código y Terraform aplica solo los cambios necesarios. Si alguien borra un recurso manualmente, Terraform lo detecta y lo recrea.

### 7.1 Estructura del proyecto Terraform

```
terraform/
├── main.tf              ← Proveedores y configuración general
├── variables.tf         ← Variables de entrada (región, tipo de instancia, etc.)
├── outputs.tf           ← Valores de salida (IPs, DNS)
├── modules/
│   ├── compute/         ← Módulo de servidores EC2
│   └── database/        ← Módulo de base de datos RDS
└── environments/
    ├── dev/
    │   └── terraform.tfvars
    └── prod/
        └── terraform.tfvars
```

### 7.2 Proveedor AWS y módulo de servidores

```hcl
# main.tf
terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region = var.aws_region
}
```

```hcl
# modules/compute/main.tf
resource "aws_instance" "api_server" {
  count         = var.instance_count
  ami           = data.aws_ami.debian.id
  instance_type = var.instance_type

  vpc_security_group_ids = [aws_security_group.api_sg.id]

  user_data = templatefile("${path.module}/user_data.sh", {
    database_url = var.database_url
    environment  = var.environment
  })

  tags = {
    Name        = "api-${var.environment}-${count.index + 1}"
    Environment = var.environment
  }
}

resource "aws_security_group" "api_sg" {
  name        = "api-${var.environment}-sg"
  description = "Puertos para la API"

  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 80
    to_port     = 8080
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
```

**Análisis línea por línea**:

- `resource "aws_instance" "api_server" {`: define una instancia EC2. Terraform llama a la API de AWS para crearla.

- `count = var.instance_count`: crea `N` instancias idénticas. Si `var.instance_count = 3`, Terraform crea 3 servidores.

- `ami = data.aws_ami.debian.id`: busca la AMI (imagen de máquina) más reciente de Debian. La fuente de datos `aws_ami` consulta a AWS por las AMI disponibles.

- `instance_type = var.instance_type`: tipo de instancia. `t3.medium` tiene 2 CPUs y 4GB de RAM, suficiente para una API Rust.

- `user_data`: script que se ejecuta al iniciar la instancia por primera vez. Aquí se instala Docker y se despliega la aplicación.

- `tags`: etiquetas que se asignan al recurso en AWS. Útiles para identificar recursos en la consola de AWS.

- `aws_security_group`: define las reglas de firewall. `ingress` son puertos de entrada (22 para SSH, 80-8080 para HTTP). `egress` es tráfico de salida (todo permitido).

### 7.3 Variables y outputs

```hcl
# variables.tf
variable "aws_region" {
  description = "Región de AWS"
  type        = string
  default     = "us-west-2"
}

variable "instance_count" {
  description = "Número de instancias de la API"
  type        = number
  default     = 2
}

variable "instance_type" {
  description = "Tipo de instancia EC2"
  type        = string
  default     = "t3.medium"
}

variable "environment" {
  description = "Entorno (dev, staging, prod)"
  type        = string
}

variable "database_url" {
  description = "URL de conexión a la base de datos"
  type        = string
  sensitive   = true  # No se muestra en los logs
}
```

```hcl
# outputs.tf
output "api_ips" {
  description = "IPs públicas de las instancias API"
  value       = aws_instance.api_server[*].public_ip
}

output "load_balancer_dns" {
  description = "DNS del balanceador de carga"
  value       = aws_lb.api_lb.dns_name
}
```

### 7.4 Uso

```bash
cd terraform/environments/prod
terraform init              # Descarga los proveedores
terraform plan              # Muestra los cambios a aplicar
terraform apply             # Aplica los cambios (crea los recursos)
terraform destroy           # Destruye todos los recursos
```

### 7.5 Errores típicos

**Error 1: credenciales de AWS no configuradas**.

```
Error: error configuring Terraform AWS Provider: no valid credential sources found
```

Solución: configurar las credenciales con `aws configure` o las variables `AWS_ACCESS_KEY_ID` y `AWS_SECRET_ACCESS_KEY`.

**Error 2: superar límites de AWS**.

Si pides 10 instancias EC2 pero tu cuenta solo tiene límite para 5:
```
Error: Error launching instance: LimitExceeded: Max number of instances exceeded
```

Solución: solicitar un aumento de límite en la consola de AWS o reducir `instance_count`.

**Error 3: eliminar recursos sin querer**.

`terraform destroy` elimina **todos** los recursos gestionados por Terraform. Si tienes una base de datos RDS con datos reales, la borra sin posibilidad de recuperación. Solución: usar `terraform plan` siempre antes de `apply` o `destroy`, y proteger los recursos críticos con `prevent_destroy = true`.

```hcl
resource "aws_db_instance" "erp_db" {
  # ...
  lifecycle {
    prevent_destroy = true  # Terraform no permitirá eliminar este recurso
  }
}
```

## 8. CI/CD con GitHub Actions

Hasta ahora hemos visto cómo construir imágenes Docker, cómo orquestar contenedores con Kubernetes, y cómo configurar servidores con Ansible. Pero todo esto lo hemos hecho **manualmente**: ejecutar `docker build`, después `docker push`, después conectarse por SSH al servidor y ejecutar `docker compose up -d`. Esto funciona, pero es lento y propenso a errores. Un desarrollador puede olvidar un paso, o compilar sin las últimas actualizaciones.

La solución es **CI/CD** (Continuous Integration / Continuous Deployment). Cada vez que haces push al repositorio, un pipeline automatizado se encarga de:

1. **Compilar** el código y verificar que no haya errores de compilación.
2. **Ejecutar tests** para verificar que la lógica de negocio funciona.
3. **Construir la imagen Docker** y publicarla en un registro.
4. **Desplegar** la nueva imagen en el servidor de producción.

GitHub Actions es la herramienta de CI/CD integrada en GitHub. Se configura con archivos YAML dentro de la carpeta `.github/workflows/`. Cada archivo define un **workflow** (pipeline) que se ejecuta cuando ocurre un evento (push, pull request, etc.).

### 8.1 Pipeline para API Axum

Este workflow se ejecuta cuando haces push a `main` y los cambios afectan al proyecto Axum.

```yaml
# .github/workflows/deploy-axum.yml
name: Deploy Axum API

# Evento que dispara el workflow: push a main
on:
  push:
    branches: [main]
    paths:           # Solo si los cambios afectan a estos archivos
      - 'manual_axum.md'
      - 'proyecto_api_axum_inventarios/**'

jobs:
  # Job 1: Compilar y ejecutar tests
  test:
    runs-on: ubuntu-latest
    steps:
      - name: Clonar repositorio
        uses: actions/checkout@v4

      - name: Instalar Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Compilar en modo release
        run: cargo build --release
        working-directory: ./proyecto_api_axum_inventarios

      - name: Ejecutar tests unitarios
        run: cargo test
        working-directory: ./proyecto_api_axum_inventarios

  # Job 2: Construir imagen Docker y publicarla
  build-and-push:
    needs: test        # Espera a que 'test' termine con éxito
    runs-on: ubuntu-latest
    steps:
      - name: Clonar repositorio
        uses: actions/checkout@v4

      - name: Construir imagen Docker
        run: docker build -t ghcr.io/${{ github.repository }}/api-axum:latest .

      - name: Publicar en GitHub Container Registry
        run: |
          echo "${{ secrets.GITHUB_TOKEN }}" | docker login ghcr.io -u ${{ github.actor }} --password-stdin
          docker push ghcr.io/${{ github.repository }}/api-axum:latest

  # Job 3: Desplegar en producción
  deploy:
    needs: build-and-push
    runs-on: ubuntu-latest
    steps:
      - name: Desplegar via SSH
        uses: appleboy/ssh-action@v1.0.0
        with:
          host: ${{ secrets.DEPLOY_HOST }}
          username: ${{ secrets.DEPLOY_USER }}
          key: ${{ secrets.DEPLOY_KEY }}
          script: |
            cd /opt/erp
            docker compose pull api-axum
            docker compose up -d api-axum
```

**Análisis línea por línea**:

- `name: Deploy Axum API`: nombre del workflow. Aparece en la pestaña "Actions" de GitHub.

- `on: push: branches: [main]`: el workflow se ejecuta automáticamente cuando haces push a la rama `main`. También puedes configurarlo para pull requests, tags, o eventos programados.

- `paths:`: el workflow solo se ejecuta si los cambios afectan a estos archivos. Si haces push con cambios solo en `manual_rocket.md`, el workflow de Axum no se ejecuta.

- `jobs:`: el workflow se compone de jobs. Cada job se ejecuta en un runner (máquina virtual) independiente. Los jobs se ejecutan en paralelo a menos que uses `needs:`.

- `test:` (primer job): compila y ejecuta tests. Si la compilación falla, los jobs siguientes no se ejecutan.

- `uses: actions/checkout@v4`: acción que clona el repositorio en el runner. Sin esto, el runner no tiene acceso al código.

- `uses: actions-rust-lang/setup-rust-toolchain@v1`: instala el compilador de Rust en el runner. Sin esto, `cargo build` falla.

- `working-directory: ./proyecto_api_axum_inventarios`: ejecuta el comando en ese directorio. Sin esto, los comandos se ejecutan en la raíz del repositorio.

- `build-and-push:` (segundo job): construye la imagen Docker y la publica en GitHub Container Registry (ghcr.io).

- `needs: test`: este job espera a que `test` termine con éxito. Si `test` falla, este job no se ejecuta.

- `${{ secrets.GITHUB_TOKEN }}`: token de autenticación generado automáticamente por GitHub. Permite publicar en el registro de contenedores sin configurar credenciales manualmente.

- `deploy:` (tercer job): se conecta al servidor por SSH y actualiza los contenedores.

- `secrets.DEPLOY_HOST`: variables secretas configuradas en GitHub (Settings > Secrets). Contienen la IP del servidor, el usuario SSH y la clave privada.

### 8.2 Errores típicos

**Error 1: tests que fallan pero el workflow continúa**.

```yaml
- name: Ejecutar tests
  run: cargo test -- --nocapture  # --nocapture NO debe estar en CI
```

Si los tests fallan pero el workflow no se detiene, es porque el paso no maneja correctamente el código de salida. Solución: GitHub Actions detecta automáticamente fallos por código de salida distinto de cero. No necesitas hacer nada especial.

**Error 2: credenciales de Docker no configuradas**.

```
Error: denied: access forbidden
```

El push a GitHub Container Registry falla porque el token no tiene permisos. Solución: el `GITHUB_TOKEN` generado automáticamente necesita permisos de escritura en `Settings > Actions > General > Workflow permissions`.

**Error 3: secrets no definidos**.

Si el workflow usa `secrets.DEPLOY_HOST` pero no está definido en GitHub:
```
Error: Key validation error: DEPLOY_HOST is not a valid secret
```

Solución: definir los secretos en `Settings > Secrets and variables > Actions`.

## 9. Monitoreo y logging

Tener la API desplegada en producción no es el final del camino. Una vez que los usuarios empiezan a usar el sistema, necesitas saber: ¿está funcionando? ¿Está lenta? ¿Hay errores? ¿Cuántas peticiones recibe por segundo? ¿Cuánto tarda en responder? ¿Hay tickets sin asignar desde hace horas?

Para responder estas preguntas necesitas dos cosas: **métricas** (números agregados como peticiones/segundo, latencia, uso de CPU) y **logs** (registros detallados de cada petición y cada error).

### 9.1 Prometheus + Grafana para métricas

Prometheus recolecta métricas de la API (peticiones, latencia, errores) y las almacena en una base de datos de series temporales. Grafana visualiza esas métricas en dashboards.

```yaml
# docker-compose.monitoring.yml
services:
  prometheus:
    image: prom/prometheus
    volumes:
      - ./prometheus/prometheus.yml:/etc/prometheus/prometheus.yml
    ports: ["9090:9090"]

  grafana:
    image: grafana/grafana
    ports: ["3000:3000"]
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - grafana_data:/var/lib/grafana

volumes:
  grafana_data:
```

**Análisis**:

- `prometheus`: servidor de métricas. Escucha en el puerto 9090 y recolecta datos de las APIs. Se configura con `prometheus.yml` donde se definen los "targets" (las APIs a monitorear).

- `grafana`: visualización de métricas. Se conecta a Prometheus como fuente de datos y muestra dashboards. Escucha en el puerto 3000.

**Configuración de Prometheus**:

```yaml
# prometheus/prometheus.yml
global:
  scrape_interval: 15s  # Recolectar métricas cada 15 segundos

scrape_configs:
  - job_name: 'api-erp'
    static_configs:
      - targets: ['api-erp:8080']

  - job_name: 'api-axum'
    static_configs:
      - targets: ['api-axum:8080']

  - job_name: 'api-rocket'
    static_configs:
      - targets: ['api-rocket:8000']
```

### 9.2 Vector para logs

Vector recolecta los logs de los contenedores Docker, los procesa (parsea JSON, filtra errores) y los envía a un destino de almacenamiento (Elasticsearch, archivos, S3).

```toml
# vector.toml
# Fuente: logs de los contenedores Docker
[sources.api_logs]
type = "docker_logs"
include_images = ["api-*"]

# Transformación: parsear cada línea como JSON
[transforms.parse_json]
type = "remap"
inputs = ["api_logs"]
source = '''
  . = parse_json!(.message) ?? .
'''

# Destino: Elasticsearch para búsqueda y análisis
[sinks.elasticsearch]
type = "elasticsearch"
inputs = ["parse_json"]
endpoints = ["http://elasticsearch:9200"]
index = "api-logs-%Y-%m-%d"
```

**Análisis línea por línea**:

- `[sources.api_logs]`: define la fuente de datos. `docker_logs` lee los logs de los contenedores Docker en tiempo real.

- `include_images = ["api-*"]`: solo captura logs de contenedores cuyo nombre de imagen empiece por `api-`. Excluye logs de MariaDB y nginx.

- `[transforms.parse_json]`: procesa los logs. `remap` es el lenguaje de transformación de Vector.

- `. = parse_json!(.message) ?? .`: intenta parsear cada línea de log como JSON. Si tiene éxito, el log se convierte en un objeto JSON con campos estructurados. Si falla, se deja como texto plano.

- `[sinks.elasticsearch]`: define el destino. `elasticsearch` envía los logs procesados a Elasticsearch para búsqueda y análisis.

### 9.3 Errores típicos

**Error 1: Prometheus no puede alcanzar las APIs**.

Si las APIs no exponen métricas en el formato de Prometheus, los targets aparecen como `DOWN` en Prometheus. Solución: las APIs deben exponer un endpoint `/metrics` con métricas en formato Prometheus. Puedes usar `metrics-exporter-prometheus` en Rust.

**Error 2: Elasticsearch sin memoria**.

Elasticsearch puede consumir mucha RAM si no se limita. Síntoma: el contenedor de Elasticsearch se reinicia constantemente. Solución: limitar la memoria en docker-compose:

```yaml
elasticsearch:
  image: docker.elastic.co/elasticsearch/elasticsearch:8.x
  environment:
    - ES_JAVA_OPTS=-Xms512m -Xmx512m
  mem_limit: 1g
```

## 10. Ejercicios

1. Crear un Dockerfile para la API de Rocket del manual_rocket.md y compilar la imagen.
2. Escalar la API del ERP a 5 instancias con Docker Compose y verificar el balanceo con nginx.
3. Escribir un playbook de Ansible que instale Docker y despliegue los 3 proyectos en un servidor.
4. Crear un módulo de Terraform para una base de datos RDS MySQL administrada.
5. Configurar Prometheus para recolectar métricas de las 3 APIs.
6. Implementar un pipeline de CI/CD en GitHub Actions que compile, testee y despliegue automáticamente.
7. Agregar un healthcheck que verifique no solo el endpoint /health sino también la conexión a la BD.

## 11. Soluciones

Las soluciones detalladas de los ejercicios están en los directorios `ansible/`, `terraform/`, `k8s/` y `.github/workflows/` del proyecto, con ejemplos funcionales para cada herramienta.
