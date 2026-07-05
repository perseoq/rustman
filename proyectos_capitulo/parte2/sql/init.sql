-- Esquema inicial del ERP/CRM (México) - MariaDB/MySQL
DROP DATABASE IF EXISTS erp_crm;
CREATE DATABASE erp_crm CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE erp_crm;

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

CREATE TABLE proveedores (
    id           INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    nombre       VARCHAR(150) NOT NULL,
    rfc          VARCHAR(13)  NOT NULL UNIQUE,
    dias_pago    TINYINT UNSIGNED NOT NULL DEFAULT 30,
    activo       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB;

CREATE TABLE categorias (
    id           INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    nombre       VARCHAR(100) NOT NULL UNIQUE,
    descripcion  VARCHAR(255)
) ENGINE=InnoDB;

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

CREATE TABLE pedidos (
    folio        VARCHAR(20) PRIMARY KEY,
    cliente_id   INT UNSIGNED NOT NULL,
    subtotal     DECIMAL(14,2) NOT NULL,
    iva          DECIMAL(14,2) NOT NULL,
    total        DECIMAL(14,2) NOT NULL,
    estado       VARCHAR(20) NOT NULL DEFAULT 'BORRADOR',
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (cliente_id) REFERENCES clientes(id)
) ENGINE=InnoDB;

CREATE TABLE lineas_pedido (
    id              BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    pedido_folio    VARCHAR(20) NOT NULL,
    sku             VARCHAR(30) NOT NULL,
    cantidad        INT UNSIGNED NOT NULL,
    precio_unitario DECIMAL(12,2) NOT NULL,
    FOREIGN KEY (pedido_folio) REFERENCES pedidos(folio) ON DELETE CASCADE,
    FOREIGN KEY (sku) REFERENCES productos(sku)
) ENGINE=InnoDB;

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
