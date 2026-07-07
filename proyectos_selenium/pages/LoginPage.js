const { By, until } = require('selenium-webdriver');

class LoginPage {
  constructor(driver) {
    this.driver = driver;
    this.url = 'http://localhost:8080/login';
  }

  emailInput() { return this.driver.findElement(By.name('email')); }
  passwordInput() { return this.driver.findElement(By.name('password')); }
  submitButton() { return this.driver.findElement(By.id('btn-ingresar')); }
  errorMessage() { return this.driver.findElement(By.id('error-login')); }

  async abrir() {
    await this.driver.get(this.url);
  }

  async login(email, password) {
    await this.emailInput().sendKeys(email);
    await this.passwordInput().sendKeys(password);
    await this.submitButton().click();
  }

  async obtenerMensajeError() {
    await this.driver.wait(until.elementLocated(By.id('error-login')), 5000);
    return await this.errorMessage().getText();
  }
}

module.exports = LoginPage;
