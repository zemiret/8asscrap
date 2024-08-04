import { parseArgs } from 'node:util';
import puppeteer from 'puppeteer-extra';
import StealthPlugin from 'puppeteer-extra-plugin-stealth';

puppeteer.use(StealthPlugin()); // very important to avoid being detected and make headless work

const options = {
  username: {
    type: 'string',
    short: 'u'
  },
  password: {
    type: 'string',
    short: 'p'
  },
};

const { values, _ } = parseArgs({ options });
const { username, password } = values;

(async () => {
  if (username == null || password == null) {
    console.log("username or password not provied");
    process.exitCode = 1
    return
  }

  const browser = await puppeteer.launch();
//  const browser = await puppeteer.launch({ headless: false });
  const page = await browser.newPage();

  await page.goto('https://www.8a.nu/');

  await page.setViewport({ width: 1080, height: 1024 });
  await page.waitForNetworkIdle();

  const logInButtonSelector = '.loggedout > .user-links > a:nth-child(1)';
  await page.waitForSelector(logInButtonSelector, { timeout: 10000 });

  await Promise.all([
    page.click(logInButtonSelector),
    page.waitForNavigation(),
  ]);

  const userInputSelector = '#username';
  const passwordInputSelector = '#password';
  await page.type(userInputSelector, username);
  await page.type(passwordInputSelector, password);

  const signInButtonSelector = '#kc-login';
  await page.waitForSelector(signInButtonSelector, { timeout: 5000 });
  await Promise.all([
    page.click(signInButtonSelector),
    page.waitForNavigation(),
  ]);

  const cookies = await page.cookies('https://www.8a.nu/');
  const sidCookie = cookies.find(x => x.name === "connect.sid");

  console.log(sidCookie.value);
  await browser.close();
})();
