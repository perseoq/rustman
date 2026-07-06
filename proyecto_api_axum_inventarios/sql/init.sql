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
