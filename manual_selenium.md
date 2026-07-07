# Manual de Selenium con JavaScript para testing de APIs

> **Automatización de pruebas end-to-end con Selenium WebDriver y JavaScript sobre las APIs del repositorio: ERP/CRM (Actix Web), inventarios (Axum) y tickets (Rocket).**

---

**Stack tecnológico**: Node.js 22 · selenium-webdriver 4 · Mocha 10 · Chai 5 · Chrome Driver · Firefox (GeckoDriver)

---

## 1. Introduccion a Selenium

Cuando construyes una API, escribes tests unitarios para verificar que cada función devuelve el resultado correcto. Escribes tests de integración para verificar que la API y la base de datos funcionan juntas. Pero hay algo que estos tests no verifican: **¿la API funciona cuando un usuario real la usa desde un navegador?**. Los tests unitarios no detectan problemas de CORS, no detectan errores en el formato JSON, no detectan que un endpoint devuelve 500 cuando debería devolver 404, no detectan que la página de login tarda 30 segundos en cargar.

Selenium resuelve este problema: automatiza un navegador real (Chrome, Firefox) y ejecuta las mismas acciones que haría un usuario. Selenium puede abrir una página web, hacer clic en botones, llenar formularios, leer textos, y verificar que todo funciona como se espera. No importa si el backend está escrito en Rust, Python o Java: Selenium prueba el sistema completo desde la perspectiva del usuario.

En este manual vamos a usar Selenium WebDriver con JavaScript (Node.js) para probar las APIs del repositorio. Pero como nuestras APIs son REST (devuelven JSON, no HTML), vamos a hacer dos cosas:

1. **Probar las APIs directamente**: Selenium puede ejecutar JavaScript en el navegador, lo que nos permite hacer peticiones HTTP a los endpoints y verificar las respuestas.
2. **Construir un mini frontend**: crearemos un dashboard HTML/JS que consuma las APIs, y probaremos ese dashboard con Selenium.

### 1.1 ¿Que es Selenium WebDriver?

Selenium WebDriver es una librería que se comunica con el navegador a través de un protocolo estándar (WebDriver Protocol o DevTools Protocol). Le dices "navega a esta URL", "encuentra este botón", "haz clic", y el navegador lo ejecuta como si lo hiciera un usuario real.

La arquitectura es:

```
Código JS (Node.js)
      │
      ▼
 selenium-webdriver (cliente)
      │
      ▼
 chromedriver (puente)
      │
      ▼
 Chrome (navegador real)
```

Tu código JavaScript usa la librería `selenium-webdriver` para enviar comandos. La librería se comunica con `chromedriver` (un ejecutable que abre Chrome y lo controla). Chrome ejecuta las acciones y devuelve los resultados.

### 1.2 ¿Que probaremos?

A lo largo del manual probaremos estos escenarios reales de las APIs del repositorio:

| Escenario | API | Operaciones Selenium |
|---|---|---|
| Login de usuario | ERP (Actix) | Llamar POST /api/auth/login, verificar token JWT |
| CRUD de clientes | ERP (Actix) | Crear, listar, actualizar y eliminar clientes vía API |
| Consulta de stock | Inventarios (Axum) | Listar productos, consultar stock por SKU |
| Creación de ticket | Tickets (Rocket) | Crear ticket, verificar estado "Abierto", cambiar estado |
| Dashboard visual | Frontend HTML | Navegar, hacer clic, leer datos en pantalla |

## 2. Configuracion del entorno

Antes de escribir tests, necesitamos instalar las herramientas necesarias: Node.js, Chrome, chromedriver, y las dependencias npm.

### 2.1 Instalacion de Node.js y dependencias

Selenium WebDriver soporta múltiples navegadores. Los dos más usados son **Chrome** (con chromedriver) y **Firefox** (con geckodriver). Puedes instalar ambos o solo el que prefieras. Los tests se pueden configurar para usar uno u otro mediante una variable de entorno.

```bash
# Verificar que Node.js está instalado
node --version   # debe ser 18+ o 22+

# Crear el proyecto
mkdir proyectos_selenium
cd proyectos_selenium
npm init -y

# Instalar dependencias de Selenium
npm install selenium-webdriver@4

# Instalar dependencias de testing
npm install mocha chai --save-dev

# Instalar drivers de navegadores (elige uno o ambos)
npm install chromedriver --save-dev    # Para Chrome/Chromium
npm install geckodriver --save-dev     # Para Firefox
```

**Análisis línea por línea**:

- `node --version`: verifica que Node.js está instalado. Selenium WebDriver 4 requiere Node.js 18+. Si no tienes Node.js, descárgalo desde nodejs.org o usa `nvm install 22`.

- `npm init -y`: crea un `package.json` con valores por defecto.

- `npm install selenium-webdriver@4`: instala la librería principal de Selenium para JavaScript. La versión 4 es la más reciente estable e incluye el protocolo W3C WebDriver estándar que funciona tanto con Chrome como con Firefox.

- `npm install mocha chai --save-dev`: instala Mocha (framework de tests) y Chai (librería de assertions).

- `npm install chromedriver --save-dev`: instala el driver para Chrome/Chromium. chromedriver es un ejecutable que actúa como puente entre Selenium y Chrome. Debe coincidir con la versión de Chrome instalada.

- `npm install geckodriver --save-dev`: instala el driver para Firefox. geckodriver es el equivalente a chromedriver pero para Firefox. Mozilla lo mantiene oficialmente.

### 2.2 Verificacion de los drivers

```bash
# Verificar que chromedriver está instalado y su versión
npx chromedriver --version
# Salida esperada: ChromeDriver 130.0.xxxx (xx) (yyyy-mm-dd)

# Verificar que geckodriver está instalado
npx geckodriver --version
# Salida esperada: geckodriver 0.35.0

# Ejecutar tests con Chrome (por defecto)
npm test

# Ejecutar tests con Firefox
BROWSER=firefox npm test
```

**Análisis**: Siempre verifica que el driver coincide con la versión de tu navegador. Si Chrome se actualiza y chromedriver no, Selenium falla con `session not created: This version of ChromeDriver only supports Chrome version X`. Para Firefox, la compatibilidad es más estable porque geckodriver usa el protocolo estándar W3C.

### 2.3 Configuracion especifica de Firefox

Firefox tiene opciones específicas que Chrome no tiene, y viceversa. Selenium permite configurarlas con clases separadas:

```javascript
// Opciones específicas de Firefox
const firefox = require('selenium-webdriver/firefox');
const opcionesFF = new firefox.Options();

// Modo headless
opcionesFF.addArguments('--headless');

// Configurar preferencias de Firefox
opcionesFF.setPreference('dom.ipc.processCount', 8);
opcionesFF.setPreference('dom.webdriver.enabled', false);

// Crear driver con Firefox
const driverFF = await new Builder()
  .forBrowser('firefox')
  .setFirefoxOptions(opcionesFF)
  .build();

// Opciones específicas de Chrome
const chrome = require('selenium-webdriver/chrome');
const opcionesChrome = new chrome.Options();
opcionesChrome.addArguments('--headless', '--disable-gpu', '--no-sandbox');

const driverChrome = await new Builder()
  .forBrowser('chrome')
  .setChromeOptions(opcionesChrome)
  .build();
```

**Análisis línea por línea**:

- `const firefox = require('selenium-webdriver/firefox');`: importa el módulo específico de Firefox. Contiene `Options` para configurar Firefox y `Driver` para crear instancias.

- `new firefox.Options()`: crea un objeto de opciones específico de Firefox. Las opciones de Chrome (`chrome.Options`) tienen métodos diferentes.

- `.setPreference('dom.ipc.processCount', 8)`: las preferencias de Firefox son similares a las de `about:config`. Esta preferencia aumenta el número de procesos de contenido para mejorar el rendimiento.

- `.setPreference('dom.webdriver.enabled', false)`: deshabilita la detección de automatización. Algunos sitios web bloquean a Selenium si detectan la variable `navigator.webdriver`. Con esta preferencia, Firefox no la expone.

- `forBrowser('firefox')`: indica a Selenium que use Firefox en lugar de Chrome. Selenium busca `geckodriver` en el PATH automáticamente.

### 2.4 package.json base

El `package.json` incluye scripts para ejecutar tests tanto con Chrome como con Firefox. La variable de entorno `BROWSER` permite elegir el navegador sin modificar el código:

```json
{
  "name": "proyectos_selenium",
  "version": "1.0.0",
  "description": "Tests Selenium para las APIs del repositorio",
  "scripts": {
    "test": "mocha --timeout 30000 'tests/**/*.test.js'",
    "test:erp": "mocha --timeout 30000 'tests/erp/**/*.test.js'",
    "test:axum": "mocha --timeout 30000 'tests/axum/**/*.test.js'",
    "test:chrome": "BROWSER=chrome mocha --timeout 30000 'tests/**/*.test.js'",
    "test:firefox": "BROWSER=firefox mocha --timeout 30000 'tests/**/*.test.js'",
    "test:headless": "HEADLESS=true mocha --timeout 30000 'tests/**/*.test.js'"
  },
  "devDependencies": {
    "chai": "^5.0.0",
    "chromedriver": "^latest",
    "geckodriver": "^latest",
    "mocha": "^10.0.0",
    "selenium-webdriver": "^4.0.0"
  }
}
```

**Análisis**:

- `"test": "mocha --timeout 30000 'tests/**/*.test.js'"`: ejecuta todos los tests con el navegador definido en `BROWSER` (chrome por defecto). `--timeout 30000` da hasta 30 segundos a cada test antes de fallar.

- `"test:chrome"`: fuerza el uso de Chrome independientemente de la variable `BROWSER`.

- `"test:firefox"`: fuerza el uso de Firefox. Requiere `geckodriver` instalado.

- `"test:headless"`: ejecuta en modo headless (sin ventana gráfica). Útil para servidores CI/CD sin pantalla.

- `"chromedriver"` y `"geckodriver"`: ambos drivers están incluidos como dependencias opcionales. Puedes eliminar el que no necesites.

## 3. Primer test: abrir navegador y verificar una URL

Vamos a escribir nuestro primer test Selenium: abrir Chrome, navegar a una URL, verificar el título de la página, y cerrar el navegador. Este test verifica que todo el stack (Node.js + Selenium + Chrome) funciona correctamente.

### 3.1 Codigo del primer test

```javascript
// tests/01_primer_test.test.js
const { Builder, By, until } = require('selenium-webdriver');
const { expect } = require('chai');

// Seleccionar navegador según variable de entorno (chrome por defecto)
const NAVEGADOR = process.env.BROWSER || 'chrome';
const HEADLESS = process.env.HEADLESS === 'true';

describe('Primer test Selenium', function () {
  let driver;

  // before: se ejecuta UNA VEZ antes de todos los tests
  before(async function () {
    const opciones = new (require('selenium-webdriver').Options)();
    if (HEADLESS) {
      opciones.addArguments('--headless');
    }

    if (NAVEGADOR === 'firefox') {
      driver = await new Builder()
        .forBrowser('firefox')
        .setFirefoxOptions(opciones)
        .build();
    } else {
      driver = await new Builder()
        .forBrowser('chrome')
        .setChromeOptions(opciones)
        .build();
    }
  });

  // after: se ejecuta UNA VEZ después de todos los tests
  after(async function () {
    await driver.quit();
  });

  it('debería navegar a Google y verificar el título', async function () {
    await driver.get('https://www.google.com');
    const titulo = await driver.getTitle();
    expect(titulo).to.include('Google');
  });
});
```

**Análisis línea por línea**:

- `const { Builder, By, until } = require('selenium-webdriver');`: importa las clases principales de Selenium. `Builder` construye el driver del navegador. `By` define estrategias de localización (por ID, CSS, XPath). `until` define condiciones de espera.

- `describe('Primer test Selenium', function () { ... })`: bloque de Mocha que agrupa tests relacionados. El primer argumento es el nombre del grupo.

- `before(async function () { ... })`: hook de Mocha que se ejecuta UNA VEZ antes de todos los tests del bloque `describe`. Aquí creamos el driver del navegador.

- `driver = await new Builder().forBrowser('chrome').build();`: crea una instancia del navegador Chrome. `Builder().forBrowser('chrome')` configura Chrome. `.build()` inicia el navegador. `await` espera a que Chrome esté listo.

- `after(async function () { await driver.quit(); })`: hook que se ejecuta después de todos los tests. `driver.quit()` cierra el navegador y libera recursos. Si olvidas llamar a `quit()`, el proceso de Chrome queda abierto.

- `it('debería navegar a Google...', async function () { ... })`: un test individual. El primer argumento es la descripción del test.

- `await driver.get('https://www.google.com');`: navega a la URL especificada. `get()` espera a que la página cargue completamente antes de continuar.

- `const titulo = await driver.getTitle();`: obtiene el título de la página (la etiqueta `<title>` del HTML).

- `expect(titulo).to.include('Google');`: assertion de Chai. Verifica que el título contenga la palabra "Google". Si no, el test falla con un mensaje descriptivo.

### 3.2 Ejecucion del test

```bash
cd proyectos_selenium
npm test
```

**Salida esperada**:

```
  Primer test Selenium
    ✔ debería navegar a Google y verificar el título

  1 passing (3s)
```

Si el test falla, el mensaje indica qué salió mal. Por ejemplo, si Chrome no está instalado:

```
WebDriverError: Chrome failed to start: exited abnormally
```

### 3.3 Errores tipicos

**Error 1: Chrome no está instalado**.

```
WebDriverError: Chrome failed to start: exited abnormally
  (unknown error: DevToolsActivePort file doesn't exist)
```

Solución: instalar Chrome o usar `chromium-browser`. En Ubuntu: `sudo apt install google-chrome-stable`.

**Error 2: chromedriver no coincide con la versión de Chrome**.

```
SessionNotCreatedError: session not created: This version of ChromeDriver only supports Chrome version 114
Current browser version is 120.0.6099.71
```

Solución: actualizar chromedriver: `npm install chromedriver@latest`.

**Error 3: olvidar `await` en `driver.quit()`**.

```javascript
after(async function () {
  driver.quit();  // falta await
});
```

Sin `await`, el navegador puede cerrarse antes de que el test termine. Solución: `await driver.quit()`.

## 4. Localizadores: encontrar elementos en la pagina

Para interactuar con una página web (hacer clic, escribir, leer), primero necesitas encontrar el elemento. Selenium ofrece varias estrategias de localización.

### 4.1 Estrategias de localizacion

| Estrategia | Método | Ejemplo | Cuándo usarlo |
|---|---|---|---|
| ID | `By.id('nombre')` | `By.id('btn-login')` | Cuando el elemento tiene un ID único |
| CSS | `By.css('.clase')` | `By.css('button.primary')` | Cuando no hay ID pero hay clase o atributo |
| XPath | `By.xpath('//button')` | `By.xpath("//input[@type='email']")` | Cuando CSS no es suficiente |
| Name | `By.name('email')` | `By.name('email')` | En formularios con atributo `name` |
| Text | `By.xpath("//*[text()='Enviar']")` | — | Cuando necesitas encontrar por el texto visible |

```javascript
const { By } = require('selenium-webdriver');

// Por ID (más rápido y específico)
const boton = await driver.findElement(By.id('btn-enviar'));

// Por CSS (flexible y legible)
const campoEmail = await driver.findElement(By.css('input[type="email"]'));
const botonPrimary = await driver.findElement(By.css('button.btn-primary'));

// Por XPath (más potente, más lento)
const primeraFila = await driver.findElement(By.xpath('//table/tbody/tr[1]'));
const porTexto = await driver.findElement(By.xpath("//button[text()='Guardar']"));
```

**Análisis línea por línea**:

- `driver.findElement(By.id('btn-enviar'))`: busca un elemento con `id="btn-enviar"`. Es la estrategia más rápida y específica. Si el elemento no existe, Selenium lanza `NoSuchElementError`.

- `driver.findElement(By.css('input[type="email"]'))`: busca el primer elemento que coincida con el selector CSS `input[type="email"]`. Si hay múltiples coincidencias, devuelve la primera.

- `driver.findElement(By.xpath('//table/tbody/tr[1]'))`: busca la primera fila de la primera tabla usando XPath. XPath es más potente que CSS pero más lento.

### 4.2 findElements (buscar multiples elementos)

```javascript
// Buscar TODOS los elementos que coinciden
const filas = await driver.findElements(By.css('table tbody tr'));
console.log(`Número de filas: ${filas.length}`);

// Iterar sobre ellos
for (const fila of filas) {
  const texto = await fila.getText();
  console.log(`Fila: ${texto}`);
}
```

### 4.3 Errores tipicos

**Error 1: elemento no encontrado**.

```javascript
const elemento = await driver.findElement(By.id('id-inexistente'));
```

Mensaje: `NoSuchElementError: no such element: Unable to locate element`. Solución: verificar que el ID existe en el HTML. Usar `findElements` (en plural) para obtener un array vacío en lugar de error.

**Error 2: elemento no visible**.

```javascript
const boton = await driver.findElement(By.id('btn-oculto'));
await boton.click();  // el botón existe pero está oculto
```

Mensaje: `ElementNotInteractableError: element not interactable`. Solución: esperar a que el elemento sea visible con `driver.wait(until.elementIsVisible(elemento), 5000)`.

## 5. Interacciones: clic, escribir, leer

Una vez que encuentras un elemento, puedes interactuar con él.

### 5.1 Click y escritura

```javascript
const { By } = require('selenium-webdriver');

// Hacer clic en un botón
const boton = await driver.findElement(By.id('btn-enviar'));
await boton.click();

// Escribir en un campo de texto
const campoEmail = await driver.findElement(By.name('email'));
await campoEmail.sendKeys('usuario@ejemplo.com');

// Limpiar un campo antes de escribir
await campoEmail.clear();
await campoEmail.sendKeys('nuevo@ejemplo.com');

// Leer el texto de un elemento
const mensaje = await driver.findElement(By.id('mensaje-exito'));
const texto = await mensaje.getText();
console.log(`Mensaje: ${texto}`);
```

**Análisis línea por línea**:

- `await boton.click()`: hace clic en el elemento. Si el elemento está oculto o deshabilitado, lanza `ElementNotInteractableError`.

- `await campoEmail.sendKeys('...')`: escribe el texto en el campo. Simula la pulsación de teclas una por una.

- `await campoEmail.clear()`: limpia el campo (selecciona todo y borra). Sin `clear()`, `sendKeys()` añade al texto existente.

- `mensaje.getText()`: obtiene el texto visible del elemento. Incluye el texto de los elementos hijos. Para obtener el valor de un input, usa `getAttribute('value')`.

### 5.2 Obtencion de atributos

```javascript
// Obtener el valor de un input
const valor = await campoEmail.getAttribute('value');

// Obtener cualquier atributo HTML
const href = await enlace.getAttribute('href');
const clase = await elemento.getAttribute('class');
const placeholder = await input.getAttribute('placeholder');
```

## 6. Esperas: el secreto de los tests estables

El error más común en tests Selenium es `NoSuchElementException`. Ocurre porque el navegador es más lento que tu código: el test intenta encontrar un elemento que todavía no se ha cargado. La solución son las **esperas** (waits).

### 6.1 Espera implicita (no recomendada)

```javascript
// Configurar espera implícita (al inicio, una vez)
await driver.manage().setTimeouts({ implicit: 5000 });

// Ahora cada findElement espera hasta 5 segundos antes de fallar
const elemento = await driver.findElement(By.id('tabla-clientes'));
```

**Problema**: la espera implícita se aplica a TODAS las búsquedas, incluso cuando el elemento sí existe. Ralentiza los tests.

### 6.2 Espera explicita (recomendada)

```javascript
const { By, until } = require('selenium-webdriver');

// Esperar hasta que un elemento específico sea visible
const boton = await driver.wait(
  until.elementLocated(By.id('btn-guardar')),
  10000,      // timeout: 10 segundos
  'El botón guardar no apareció después de 10 segundos'
);

// Esperar hasta que el texto esté presente
await driver.wait(
  until.elementTextContains(By.id('mensaje'), 'Creado exitosamente'),
  5000
);

// Esperar hasta que un elemento desaparezca
await driver.wait(
  until.stalenessOf(elementoAnterior),
  5000
);
```

**Análisis línea por línea**:

- `driver.wait(condicion, timeout, mensaje)`: espera hasta que la condición se cumpla o hasta que pasen `timeout` milisegundos. Si el timeout se alcanza, el test falla con el mensaje personalizado.

- `until.elementLocated(By.id('btn-guardar'))`: condición que espera a que el elemento exista en el DOM (aunque esté oculto).

- `until.elementTextContains(By.id('mensaje'), 'Creado')`: espera a que el elemento contenga el texto especificado.

### 6.3 Espera fluida (custom)

```javascript
async function esperarElemento(driver, localizador, timeout = 10000, intervalo = 500) {
  const fin = Date.now() + timeout;
  while (Date.now() < fin) {
    try {
      const elemento = await driver.findElement(localizador);
      if (await elemento.isDisplayed()) {
        return elemento;
      }
    } catch (e) {
      // elemento no encontrado, esperar y reintentar
    }
    await driver.sleep(intervalo);
  }
  throw new Error(`Elemento no encontrado después de ${timeout}ms: ${localizador}`);
}
```

### 6.4 Errores tipicos

**Error 1: Timeout en wait**.

```javascript
await driver.wait(until.elementLocated(By.id('tabla')), 2000);
// Si la tabla tarda 3 segundos en cargar, esto falla
```

Mensaje: `TimeoutError: Waiting for element to be located`. Solución: aumentar el timeout o usar una espera más larga.

**Error 2: StaleElementReferenceError**.

```javascript
const elemento = await driver.findElement(By.id('lista'));
// (la página se actualiza aquí)
await elemento.getText();  // el elemento ya no está en el DOM
```

Mensaje: `StaleElementReferenceError: element is not attached to the page document`. Solución: volver a buscar el elemento después de la actualización de la página.

## 7. Page Object Model (POM)

A medida que los tests crecen, el código se vuelve repetitivo y difícil de mantener. El **Page Object Model** resuelve esto encapsulando la lógica de cada página en una clase.

### 7.1 Clase Page Object para Login

```javascript
// pages/LoginPage.js
const { By, until } = require('selenium-webdriver');

class LoginPage {
  constructor(driver) {
    this.driver = driver;
    this.url = 'http://localhost:8080/login';
  }

  // Localizadores (definidos como métodos)
  emailInput() { return this.driver.findElement(By.name('email')); }
  passwordInput() { return this.driver.findElement(By.name('password')); }
  submitButton() { return this.driver.findElement(By.id('btn-ingresar')); }
  errorMessage() { return this.driver.findElement(By.id('error-login')); }

  // Acciones
  async abrir() {
    await this.driver.get(this.url);
  }

  async login(email, password) {
    await this.emailInput().sendKeys(email);
    await this.passwordInput().sendKeys(password);
    await this.submitButton().click();
  }

  async obtenerMensajeError() {
    await this.driver.wait(
      until.elementLocated(By.id('error-login')),
      5000
    );
    return await this.errorMessage().getText();
  }
}

module.exports = LoginPage;
```

**Análisis línea por línea**:

- `class LoginPage {`: encapsula todo lo relacionado con la página de login. Los tests no necesitan saber los IDs de los elementos.

- `this.emailInput() { return this.driver.findElement(By.name('email')); }`: método que devuelve el elemento. Si el localizador cambia, solo modificas este método, no todos los tests.

- `async login(email, password) {`: método de alto nivel. El test solo llama a `loginPage.login('admin', '1234')` sin preocuparse de los detalles de implementación.

- `async obtenerMensajeError() {`: encapsula la espera y la lectura del mensaje. El test no necesita saber que hay un `wait` ni qué ID tiene el mensaje.

### 7.2 Test usando la Page Object

```javascript
// tests/erp/login.test.js
const { Builder } = require('selenium-webdriver');
const { expect } = require('chai');
const LoginPage = require('../../pages/LoginPage');

describe('Login del ERP', function () {
  let driver;
  let loginPage;

  before(async function () {
    driver = await new Builder().forBrowser('chrome').build();
    loginPage = new LoginPage(driver);
  });

  after(async function () {
    await driver.quit();
  });

  it('debería mostrar error con credenciales inválidas', async function () {
    await loginPage.abrir();
    await loginPage.login('invalido@test.com', '1234');
    const error = await loginPage.obtenerMensajeError();
    expect(error).to.include('Credenciales inválidas');
  });
});
```

**Ventaja del POM**: si la página de login cambia (el ID del mensaje de error cambia de `error-login` a `msg-error`), solo modificas `LoginPage.js`. Los tests no cambian.

## 8. Tests contra las APIs del repositorio

Vamos a escribir tests Selenium que consuman directamente las APIs REST del repositorio. Selenium puede ejecutar JavaScript en el navegador, lo que nos permite hacer peticiones HTTP.

### 8.1 Test contra el ERP (Actix Web)

Este test verifica que la API del ERP responde correctamente. Primero inicia sesión con JWT, luego crea un cliente, y finalmente verifica que el cliente fue creado.

```javascript
// tests/erp/api_clientes.test.js
const { Builder } = require('selenium-webdriver');
const { expect } = require('chai');

const API_BASE = 'http://localhost:8080/api';

describe('API de Clientes del ERP', function () {
  let driver;
  let token;

  before(async function () {
    driver = await new Builder().forBrowser('chrome').build();
  });

  after(async function () {
    await driver.quit();
  });

  it('POST /api/auth/login - debería obtener token JWT', async function () {
    const script = `
      return fetch('${API_BASE}/auth/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username: 'admin', password: 'admin123' })
      }).then(r => r.json())
    `;

    const respuesta = await driver.executeScript(script);
    expect(respuesta).to.have.property('token');
    token = respuesta.token;
  });

  it('POST /api/clientes - debería crear un cliente', async function () {
    const script = `
      return fetch('${API_BASE}/clientes', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': 'Bearer ${token}'
        },
        body: JSON.stringify({
          nombre: 'Cliente Test Selenium',
          rfc: 'SEL010101XXX',
          email: 'selenium@test.com',
          credito: 50000
        })
      }).then(r => r.json())
    `;

    const respuesta = await driver.executeScript(script);
    expect(respuesta).to.have.property('id');
    expect(respuesta.nombre).to.equal('Cliente Test Selenium');
  });

  it('GET /api/clientes - debería listar clientes', async function () {
    const script = `
      return fetch('${API_BASE}/clientes', {
        method: 'GET',
        headers: { 'Authorization': 'Bearer ${token}' }
      }).then(r => r.json())
    `;

    const clientes = await driver.executeScript(script);
    expect(clientes).to.be.an('array');
    expect(clientes.length).to.be.greaterThan(0);
  });
});
```

**Análisis línea por línea**:

- `driver.executeScript(script)`: ejecuta código JavaScript en el navegador. El script se ejecuta como si estuviera en la consola del navegador. Puede hacer peticiones `fetch`, manipular el DOM, etc.

- `fetch(...).then(r => r.json())`: hace una petición HTTP a la API y parsea la respuesta como JSON. `fetch` es el API nativa del navegador para hacer peticiones HTTP.

- `headers: { 'Authorization': 'Bearer ${token}' }`: autenticación JWT. El token se obtuvo en el test anterior y se pasa como variable.

- `expect(respuesta).to.have.property('id')`: verifica que la respuesta contenga un campo `id`. Si la API devuelve un error, el test falla con un mensaje claro.

### 8.2 Test contra la API de inventarios (Axum)

```javascript
// tests/axum/productos.test.js
const { Builder } = require('selenium-webdriver');
const { expect } = require('chai');

const AXUM_API = 'http://localhost:8081/api';

describe('API de Inventarios (Axum)', function () {
  let driver;

  before(async function () {
    driver = await new Builder().forBrowser('chrome').build();
  });

  after(async function () {
    await driver.quit();
  });

  it('GET /api/productos - debería listar productos', async function () {
    const script = `
      return fetch('${AXUM_API}/productos')
        .then(r => r.json())
    `;

    const productos = await driver.executeScript(script);
    expect(productos).to.be.an('array');
    if (productos.length > 0) {
      expect(productos[0]).to.have.property('sku');
      expect(productos[0]).to.have.property('nombre');
    }
  });

  it('GET /api/stock?producto=LAP-001 - debería mostrar stock', async function () {
    const script = `
      return fetch('${AXUM_API}/stock?producto=LAP-001')
        .then(r => r.json())
    `;

    const stock = await driver.executeScript(script);
    expect(stock).to.be.an('array');
    // Cada elemento debería tener producto_sku, almacen_id, cantidad
    if (stock.length > 0) {
      expect(stock[0]).to.have.property('cantidad');
    }
  });
});
```

### 8.3 Test contra la API de tickets (Rocket)

```javascript
// tests/rocket/tickets.test.js
const { Builder } = require('selenium-webdriver');
const { expect } = require('chai');

const ROCKET_API = 'http://localhost:8000/api';

describe('API de Tickets (Rocket)', function () {
  let driver;

  before(async function () {
    driver = await new Builder().forBrowser('chrome').build();
  });

  after(async function () {
    await driver.quit();
  });

  it('POST /api/tickets - debería crear un ticket', async function () {
    const script = `
      return fetch('${ROCKET_API}/tickets', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          titulo: 'Problema con Selenium',
          descripcion: 'Test automatizado desde Selenium',
          cliente_id: 1,
          prioridad: 3
        })
      }).then(r => r.json())
    `;

    const respuesta = await driver.executeScript(script);
    expect(respuesta).to.have.property('id');
    expect(respuesta.estado).to.equal('Abierto');
  });
});
```

## 9. Dashboard frontend (HTML/JS)

Para demostrar pruebas Selenium con interfaz visual, vamos a construir un dashboard HTML que consuma la API del ERP. Los tests Selenium abrirán este dashboard en el navegador y verificarán que los datos se muestran correctamente.

### 9.1 Codigo del dashboard

```html
<!-- dashboard/index.html -->
<!DOCTYPE html>
<html lang="es">
<head>
  <meta charset="UTF-8">
  <title>Dashboard ERP - Selenium</title>
  <style>
    body { font-family: Arial; margin: 20px; }
    table { border-collapse: collapse; width: 100%; }
    th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
    th { background-color: #4CAF50; color: white; }
    .error { color: red; }
    .cargando { color: gray; font-style: italic; }
  </style>
</head>
<body>
  <h1>Dashboard del ERP</h1>
  <button id="btn-cargar" onclick="cargarClientes()">Cargar Clientes</button>
  <div id="mensaje" class="cargando">Presiona el botón para cargar clientes...</div>
  <table id="tabla-clientes">
    <thead>
      <tr><th>ID</th><th>Nombre</th><th>RFC</th><th>Crédito</th></tr>
    </thead>
    <tbody id="tbody-clientes"></tbody>
  </table>

  <script>
    const API_URL = 'http://localhost:8080/api';

    async function cargarClientes() {
      const mensaje = document.getElementById('mensaje');
      const tbody = document.getElementById('tbody-clientes');

      mensaje.textContent = 'Cargando...';
      mensaje.className = 'cargando';
      tbody.innerHTML = '';

      try {
        // Login primero
        const loginRes = await fetch(`${API_URL}/auth/login`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ username: 'admin', password: 'admin123' })
        });
        const loginData = await loginRes.json();

        // Obtener clientes
        const clientesRes = await fetch(`${API_URL}/clientes`, {
          headers: { 'Authorization': `Bearer ${loginData.token}` }
        });
        const clientes = await clientesRes.json();

        // Mostrar clientes en la tabla
        clientes.forEach(c => {
          const fila = tbody.insertRow();
          fila.insertCell().textContent = c.id;
          fila.insertCell().textContent = c.nombre;
          fila.insertCell().textContent = c.rfc;
          fila.insertCell().textContent = `$${c.credito?.toFixed(2) || '0.00'}`;
        });

        mensaje.textContent = `${clientes.length} clientes cargados`;
        mensaje.className = '';
      } catch (error) {
        mensaje.textContent = 'Error al cargar clientes: ' + error.message;
        mensaje.className = 'error';
      }
    }
  </script>
</body>
</html>
```

### 9.2 Test Selenium del dashboard

```javascript
// tests/dashboard/dashboard.test.js
const { Builder, By, until } = require('selenium-webdriver');
const { expect } = require('chai');

describe('Dashboard ERP', function () {
  let driver;

  before(async function () {
    driver = await new Builder().forBrowser('chrome').build();
  });

  after(async function () {
    await driver.quit();
  });

  it('debería mostrar la tabla de clientes al hacer clic en "Cargar Clientes"', async function () {
    // 1. Navegar al dashboard
    await driver.get('http://localhost:3000');  // servido con http-server

    // 2. Verificar que el botón existe
    const boton = await driver.findElement(By.id('btn-cargar'));
    expect(boton).to.exist;

    // 3. Hacer clic en el botón
    await boton.click();

    // 4. Esperar a que los clientes se carguen (el mensaje cambia)
    await driver.wait(
      until.elementTextContains(By.id('mensaje'), 'clientes cargados'),
      10000
    );

    // 5. Verificar que la tabla tiene filas
    const filas = await driver.findElements(By.css('#tabla-clientes tbody tr'));
    expect(filas.length).to.be.greaterThan(0);

    // 6. Verificar que la primera fila tiene datos
    const primeraFila = filas[0];
    const celdas = await primeraFila.findElements(By.css('td'));
    expect(celdas.length).to.equal(4);  // ID, Nombre, RFC, Crédito
  });
});
```

**Análisis línea por línea**:

- `await driver.get('http://localhost:3000')`: navega al dashboard. El dashboard debe servirse con un servidor HTTP estático (`npx http-server dashboard/ -p 3000`).

- `const boton = await driver.findElement(By.id('btn-cargar'))`: busca el botón por su ID.

- `await boton.click()`: hace clic en el botón, lo que dispara la función `cargarClientes()`.

- `driver.wait(until.elementTextContains(By.id('mensaje'), 'clientes cargados'), 10000)`: espera hasta que el mensaje cambie, lo que indica que los datos se cargaron.

- `driver.findElements(By.css('#tabla-clientes tbody tr'))`: espera a que la tabla tenga filas y las cuenta.

## 10. Data-driven testing

Los tests con datos fijos (hardcoded) solo prueban un escenario. Para probar múltiples combinaciones, usamos **data-driven testing**: los datos vienen de un archivo externo (JSON, CSV) y el test se ejecuta una vez por cada conjunto de datos.

### 10.1 Datos desde archivo JSON

```json
// data/clientes.json
[
  {
    "nombre": "Cliente A",
    "rfc": "AAA010101AAA",
    "email": "a@test.com",
    "credito": 10000,
    "esperado": 201
  },
  {
    "nombre": "Cliente B",
    "rfc": "BBB010101BBB",
    "email": "b@test.com",
    "credito": 50000,
    "esperado": 201
  },
  {
    "nombre": "",
    "rfc": "INV123",
    "email": "",
    "credito": -100,
    "esperado": 400
  }
]
```

### 10.2 Test parametrizado

```javascript
// tests/erp/clientes_data_driven.test.js
const { Builder } = require('selenium-webdriver');
const { expect } = require('chai');
const fs = require('fs');

const clientes = JSON.parse(fs.readFileSync('data/clientes.json', 'utf-8'));

describe('Creación de clientes (data-driven)', function () {
  let driver;
  let token;

  before(async function () {
    driver = await new Builder().forBrowser('chrome').build();
    // Login para obtener token
    const script = `return fetch('http://localhost:8080/api/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username: 'admin', password: 'admin123' })
    }).then(r => r.json())`;
    const resp = await driver.executeScript(script);
    token = resp.token;
  });

  after(async function () {
    await driver.quit();
  });

  clientes.forEach((cliente, index) => {
    it(`debería crear cliente ${index + 1}: ${cliente.nombre || 'vacío'}`, async function () {
      const script = `
        return fetch('http://localhost:8080/api/clientes', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': 'Bearer ${token}'
          },
          body: JSON.stringify(${JSON.stringify(cliente)})
        }).then(r => ({ status: r.status, body: r.json() }))
      `;

      const respuesta = await driver.executeScript(script);
      expect(respuesta.status).to.equal(cliente.esperado);
    });
  });
});
```

## 11. Screenshots y reportes

Cuando un test falla en CI/CD, no tienes acceso al navegador para ver qué pasó. Las capturas de pantalla automáticas te salvan: cuando un test falla, Selenium toma una foto del navegador en ese momento.

### 11.1 Captura de pantalla en fallos

```javascript
// helpers/screenshot.js
const fs = require('fs');

async function tomarScreenshot(driver, nombre) {
  const imagen = await driver.takeScreenshot();
  const timestamp = Date.now();
  const ruta = `screenshots/${timestamp}-${nombre}.png`;
  fs.writeFileSync(ruta, imagen, 'base64');
  console.log(`📸 Screenshot guardado: ${ruta}`);
  return ruta;
}

module.exports = { tomarScreenshot };
```

### 11.2 Uso en tests

```javascript
const { tomarScreenshot } = require('../../helpers/screenshot');

afterEach(async function () {
  if (this.currentTest.state === 'failed') {
    await tomarScreenshot(driver, this.currentTest.title.replace(/\s+/g, '_'));
  }
});
```

## 12. CI/CD con GitHub Actions

Para ejecutar tests Selenium en CI/CD, necesitas un navegador en el servidor de CI. GitHub Actions proporciona Chrome preinstalado en sus runners.

### 12.1 Workflow de GitHub Actions

```yaml
# .github/workflows/selenium-tests.yml
name: Selenium Tests

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  selenium:
    runs-on: ubuntu-latest
    services:
      mysql:
        image: mariadb:11
        env:
          MYSQL_ROOT_PASSWORD: secret
          MYSQL_DATABASE: erp_crm
        ports: ["3306:3306"]
        options: >-
          --health-cmd "mysqladmin ping -h localhost"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 22

      - name: Instalar dependencias
        run: npm ci
        working-directory: proyectos_selenium

      - name: Iniciar API del ERP
        run: |
          cd proyecto_api/api_diesel
          cargo run &
          sleep 10  # Esperar a que la API inicie

      - name: Ejecutar tests Selenium
        run: npm test
        working-directory: proyectos_selenium
        env:
          HEADLESS: true
```

## 13. Ejercicios

1. Escribir un test que verifique que `GET /api/productos` devuelve un array con al menos un producto (Axum).
2. Crear una Page Object para la página de creación de tickets (Rocket).
3. Implementar un test data-driven que pruebe la creación de 10 clientes con datos desde un CSV.
4. Agregar captura de pantalla automática cuando un test falle.
5. Escribir un test que verifique el flujo completo: login → crear cliente → verificar que aparece en el listado.
6. Configurar un workflow de GitHub Actions que ejecute los tests Selenium contra la API del ERP.
7. Escribir un test que verifique que el dashboard HTML carga correctamente y muestra clientes.

## 14. Soluciones

Las soluciones detalladas están en `proyectos_selenium/` con los tests completos y funcionales.
