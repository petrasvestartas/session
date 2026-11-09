const assert = require('node:assert/strict');
const fs = require('fs');
const http = require('http');
const path = require('path');
const { chromium } = require('playwright');

const root = path.resolve(__dirname, '..');
const deckDirectory = process.env.DECK_DIR || path.join(root, 'examples', 'linkwitz', 'generated-one-slide');
const viewerDirectory = process.env.VIEWER_DIST || path.join(root, 'dist');
const deck = JSON.parse(fs.readFileSync(path.join(deckDirectory, 'presentation.yaml'), 'utf8'));
assert.equal(deck.topics.length, 1);
assert.equal(deck.topics[0].steps.length, 1);
assert.deepEqual(deck.topics[0].steps[0].items.map(item => item.file), ['steps/00_00.pb']);
assert.deepEqual(fs.readdirSync(path.join(deckDirectory, 'steps')).filter(name => name.endsWith('.pb')).sort(), ['00_00.pb']);
const types = { '.html': 'text/html', '.js': 'text/javascript', '.wasm': 'application/wasm', '.yaml': 'application/yaml', '.pb': 'application/octet-stream', '.ttf': 'font/ttf', '.css': 'text/css' };
const server = http.createServer((request, response) => {
  const pathname = decodeURIComponent(new URL(request.url, 'http://localhost').pathname);
  const base = path.resolve(pathname.startsWith('/deck/') ? deckDirectory : viewerDirectory);
  const relative = pathname === '/' ? 'index.html' : pathname.startsWith('/deck/') ? pathname.slice(6) : pathname.slice(1);
  const filename = path.resolve(base, relative);
  if (filename !== base && !filename.startsWith(base + path.sep)) {
    response.writeHead(403).end();
    return;
  }
  fs.readFile(filename, (error, data) => {
    if (error) {
      response.writeHead(404).end();
      return;
    }
    response.writeHead(200, { 'Content-Type': types[path.extname(filename)] || 'application/octet-stream', 'Cache-Control': 'no-store' });
    response.end(data);
  });
});

(async () => {
  let browser;
  try {
    await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
    browser = await chromium.launch({
      executablePath: process.env.CHROME_BIN || 'C:/Program Files/Google/Chrome/Application/chrome.exe',
      headless: process.env.HEADLESS !== '0',
      args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan'],
    });
    const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    await page.goto(`http://127.0.0.1:${server.address().port}/?inspect=1&presentation=deck/presentation.yaml`, { waitUntil: 'domcontentloaded' });
    await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationReady === 'true', null, { timeout: 60000 });
    await page.waitForFunction(() => {
      const state = JSON.parse(document.querySelector('canvas')?.dataset.viewerInspection || '{}');
      return state.text_labels?.some(label => label.name === 'B-0-0-calculation');
    }, null, { timeout: 10000 });
    await page.waitForFunction(() => document.querySelector('canvas')?.dataset.presentationAnimating === 'false', null, { timeout: 10000 });
    const state = JSON.parse(await page.locator('canvas').getAttribute('data-viewer-inspection'));
    const formula = state.text_labels.filter(label => label.name === 'B-0-0-calculation').map(label => label.text).join('').normalize('NFKC').replace(/\s/g, '');
    assert.equal(formula, '1×5+2×6=17');
    assert.equal(state.presentation_effects.active.length, 0);
    assert.equal(await page.locator('canvas').getAttribute('data-presentation-overlaps'), '0');
    await page.locator('canvas').focus();
    await page.keyboard.press('ArrowRight');
    await page.waitForTimeout(200);
    assert.equal(await page.locator('canvas').getAttribute('data-presentation-step'), '0');
    assert.deepEqual(errors, []);
    const out = path.join(root, 'target', 'presentation');
    fs.mkdirSync(out, { recursive: true });
    const screenshot = await page.screenshot({ path: path.join(out, 'one-slide.png') });
    const pinkPixels = await page.evaluate(async encoded => {
      const image = new Image();
      image.src = `data:image/png;base64,${encoded}`;
      await image.decode();
      const canvas = document.createElement('canvas');
      canvas.width = image.width;
      canvas.height = image.height;
      const context = canvas.getContext('2d');
      context.drawImage(image, 0, 0);
      const pixels = context.getImageData(760, 230, 370, 180).data;
      let count = 0;
      for (let index = 0; index < pixels.length; index += 4) {
        if (Math.abs(pixels[index] - 220) <= 3 && Math.abs(pixels[index + 1] - 75) <= 3 && Math.abs(pixels[index + 2] - 141) <= 3) count++;
      }
      return count;
    }, screenshot.toString('base64'));
    assert.ok(pinkPixels >= 8, `presentation matrix positives are not #dc4b8d: ${pinkPixels} matching pixels`);
    console.log(JSON.stringify({ ok: true, slides: 1, formula, animating: false, pinkPixels }));
  } finally {
    if (browser) await browser.close();
    await new Promise(resolve => server.close(resolve));
  }
})().catch(error => {
  console.error(error);
  process.exitCode = 1;
});
