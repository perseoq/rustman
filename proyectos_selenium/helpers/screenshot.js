const fs = require('fs');
const path = require('path');

async function tomarScreenshot(driver, nombre) {
  const dir = path.join(__dirname, '..', 'screenshots');
  if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });

  const imagen = await driver.takeScreenshot();
  const timestamp = Date.now();
  const ruta = path.join(dir, `${timestamp}-${nombre}.png`);
  fs.writeFileSync(ruta, imagen, 'base64');
  console.log(`Captura guardada: ${ruta}`);
  return ruta;
}

module.exports = { tomarScreenshot };
