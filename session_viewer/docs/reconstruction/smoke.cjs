/** Real-browser checkpoint smoke; a WASM build alone cannot satisfy this check. */
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const {chromium} = require('playwright');

/** Read either the early teaching shell or the final production observation contract. */
function inspection() {
  const canvas = document.getElementById('canvas');
  const raw = canvas?.getAttribute('data-tutorial-inspection') ||
              canvas?.getAttribute('data-viewer-inspection');
  return raw ? JSON.parse(raw) : null;
}

/** Require an actual submitted frame rather than a nonempty canvas element. */
function ready() {
  const canvas = document.getElementById('canvas');
  const early = canvas?.getAttribute('data-tutorial-inspection');
  if (early) {
    const state = JSON.parse(early);
    return state.drawn === true && state.objects > 0;
  }
  const late = canvas?.getAttribute('data-viewer-inspection');
  if (!late) return false;
  const state = JSON.parse(late);
  return state.objects > 0 && state.frames > 0;
}

/** Decode the captured PNG, then count actual colored pixels and luminance variation. */
async function rasterStatistics(base64) {
  const image = new Image();
  const loaded = new Promise(function register(resolve, reject) {
    image.onload = resolve;
    image.onerror = reject;
  });
  image.src = 'data:image/png;base64,' + base64;
  await loaded;
  const canvas = document.createElement('canvas');
  canvas.width = image.width;
  canvas.height = image.height;
  const context = canvas.getContext('2d');
  context.drawImage(image, 0, 0);
  const pixels = context.getImageData(0, 0, image.width, image.height).data;
  let colored = 0, shaded = 0, minimum = 255, maximum = 0;
  for (let index = 0; index < pixels.length; index += 4) {
    const red = pixels[index], green = pixels[index + 1], blue = pixels[index + 2];
    const low = Math.min(red, green, blue), high = Math.max(red, green, blue);
    if (high - low > 25 && high > 60) colored++;
    if (high - low <= 25 && low > 40 && high < 230) shaded++;
    minimum = Math.min(minimum, (red + green + blue) / 3);
    maximum = Math.max(maximum, (red + green + blue) / 3);
  }
  return {width: image.width, height: image.height, colored, shaded, minimum, maximum};
}

/** Build evidence for one source-hashed checkpoint at both required device scales. */
async function main() {
  const id = process.env.VIEWER_TUTORIAL_STEP;
  assert.match(id || '', /^\d\d[a-z]?$/);
  const series = JSON.parse(await fs.readFile(path.join(__dirname, 'series.json'), 'utf8'));
  const step = series.steps.find(function sameStep(value) { return value.id === id; });
  assert.ok(step, 'checkpoint must be in the maintained series');
  const output = process.env.VIEWER_TEST_OUTPUT;
  await fs.mkdir(output, {recursive: true});
  const browser = await chromium.launch({
    executablePath: process.env.CHROME_BIN || '/usr/bin/google-chrome',
    headless: process.env.VIEWER_HEADLESS === '1',
    args: JSON.parse(process.env.VIEWER_CHROME_ARGS || '[]'),
  });
  const reports = [];
  try {
    for (const dpr of id === '00' ? [1] : [1, 2]) {
      const context = await browser.newContext({viewport: {width: 1000, height: 700}, deviceScaleFactor: dpr});
      const page = await context.newPage();
      const errors = [], logs = [];
      page.on('pageerror', function capture(error) { errors.push(String(error)); });
      page.on('console', function capture(message) {
        logs.push({type: message.type(), text: message.text()});
        if (message.type() === 'error' && !message.text().includes('{{__trunk_address__}}')) {
          errors.push(message.text());
        }
      });
      // A static Trunk build has no favicon asset. This response is test-server plumbing.
      await page.route('**/favicon.ico', function favicon(route) { return route.fulfill({status: 204}); });
      const route = step.browser?.route || '?data=off&inspect=1';
      await page.goto(new URL(route, process.env.VIEWER_URL).href);
      await page.bringToFront();
      if (id === '00') {
        await page.waitForFunction(function wasmReady() {
          return document.querySelector('[data-checkpoint="00"]')?.textContent.includes('Rust/WASM ready');
        }, null, {timeout: 60000});
        assert.deepEqual(errors, []);
        reports.push({id, dpr, wasm: true, errors, logs});
        await context.close();
        continue;
      }
      await page.waitForFunction(ready, null, {timeout: 60000});
      const state = await page.evaluate(inspection);
      if (step.browser?.objects !== undefined) assert.equal(state.objects, step.browser.objects);
      const dimensions = await page.locator('#canvas').evaluate(function dimensions(canvas) {
        const box = canvas.getBoundingClientRect();
        return {physical: [canvas.width, canvas.height], logical: [box.width, box.height]};
      });
      assert.equal(dimensions.physical[0], Math.round(dimensions.logical[0] * dpr));
      assert.equal(dimensions.physical[1], Math.round(dimensions.logical[1] * dpr));
      const png = await page.locator('#canvas').screenshot({path: path.join(output, `checkpoint-${id}-dpr${dpr}.png`)});
      const raster = await page.evaluate(rasterStatistics, png.toString('base64'));
      assert.ok(raster.maximum - raster.minimum > 15, 'submitted scene must produce visible pixel variation');
      if (step.browser?.colored !== false) assert.ok(raster.colored > 100, 'fixture must show actual colored geometry');
      else assert.ok(raster.shaded > 10000 * dpr * dpr, 'grey CAD fixture must show a substantial shaded surface');
      assert.deepEqual(errors, []);
      reports.push({id, dpr, state, dimensions, raster, errors, logs});
      if (id === '10') {
        await page.goto(new URL('text-layout.html', process.env.VIEWER_URL).href);
        await page.waitForFunction(function layoutReady() {
          return window.textLayout?.passed === true;
        }, null, {timeout: 60000});
        const layout = await page.evaluate(function layoutReport() { return window.textLayout; });
        assert.equal(layout.shapeCount, 5);
        assert.equal(layout.metrics.length, 5);
        for (const row of layout.metrics) assert.ok(Math.abs(row.difference) <= 0.2);
        assert.deepEqual(errors, []);
        reports.push({id, dpr, layout});
      }
      await context.close();
    }
  } finally {
    await browser.close();
  }
  await fs.writeFile(path.join(output, 'browser.json'), JSON.stringify(reports, null, 2) + '\n');
  console.log(`PASS checkpoint ${id}: actual browser WASM/frame, required DPR, visible pixels, no page/GPU errors`);
}

main().catch(function failed(error) { console.error(error); process.exitCode = 1; });
