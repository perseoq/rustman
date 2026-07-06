CREATE DATABASE IF NOT EXISTS tickets_soporte CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE tickets_soporte;

CREATE TABLE clientes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(150) NOT NULL,
    email VARCHAR(200) NOT NULL,
    empresa VARCHAR(200),
    activo BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE agentes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(150) NOT NULL,
    email VARCHAR(200) NOT NULL UNIQUE,
    especialidad VARCHAR(100),
    disponible BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tickets (
    id INT AUTO_INCREMENT PRIMARY KEY,
    titulo VARCHAR(200) NOT NULL,
    descripcion TEXT NOT NULL,
    estado ENUM('Abierto','EnProgreso','Resuelto','Cerrado','Cancelado','Escalado') NOT NULL DEFAULT 'Abierto',
    prioridad TINYINT NOT NULL DEFAULT 3,
    cliente_id INT NOT NULL,
    agente_id INT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (cliente_id) REFERENCES clientes(id),
    FOREIGN KEY (agente_id) REFERENCES agentes(id)
);

CREATE TABLE slas (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    prioridad TINYINT NOT NULL UNIQUE,
    tiempo_respuesta_horas INT NOT NULL DEFAULT 4,
    tiempo_resolucion_horas INT NOT NULL DEFAULT 24
);

INSERT INTO clientes (nombre, email, empresa) VALUES
    ('Constructora del Bajío', 'contacto@cdb.mx', 'Constructora del Bajío S.A.'),
    ('Distribuidora del Norte', 'ventas@ddn.mx', 'Distribuidora del Norte S.A.');

INSERT INTO agentes (nombre, email, especialidad) VALUES
    ('Ana López', 'ana@soporte.mx', 'Redes'),
    ('Carlos Ruiz', 'carlos@soporte.mx', 'Software');

INSERT INTO slas (nombre, prioridad, tiempo_respuesta_horas, tiempo_resolucion_horas) VALUES
    ('Baja', 1, 8, 72),
    ('Normal', 2, 4, 48),
    ('Alta', 3, 2, 24),
    ('Crítica', 4, 1, 8);
