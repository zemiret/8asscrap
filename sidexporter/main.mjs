import puppeteer from 'puppeteer';

(async () => {
  const userName = process.env.USERNAME_8A;
  const password = process.env.PASSWORD_8A;

  if (userName == null || password == null) {
    console.log("username or password not provied");
    process.exitCode = 1
    return
  }

  // const browser = await puppeteer.launch();
  const browser = await puppeteer.launch({ headless: false });
  const page = await browser.newPage();

  // Navigate the page to a URL
  await page.goto('https://www.8a.nu/');

  // Set screen size
  await page.setViewport({ width: 1080, height: 1024 });
  await page.waitForNetworkIdle();

  const logInButtonSelector = '.loggedout > .user-links > a:nth-child(1)';
  await page.waitForSelector(logInButtonSelector, { timeout: 5000 });

  await Promise.all([
    page.click(logInButtonSelector),
    page.waitForNavigation(),
  ]);

  await page.type('#username', userName);
  await page.type('#password', password);
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
