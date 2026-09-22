/** Browser regression for centered white-on-black GPU annotations and resource reuse. */
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const {chromium} = require('playwright');

/** Measure the opaque plate and the glyph interiors inside its bounds from actual pixels. */
async function measure(base64) {
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
  const pixels = context.getImageData(0, 0, canvas.width, canvas.height).data;
  let left = canvas.width, right = 0, top = canvas.height, bottom = 0, black = 0, white = 0;
  for (let index = 0; index < pixels.length; index += 4) {
    if (pixels[index] < 5 && pixels[index + 1] < 5 && pixels[index + 2] < 5) {
      const x = (index / 4) % canvas.width, y = Math.floor(index / 4 / canvas.width);
      left = Math.min(left, x); right = Math.max(right, x);
      top = Math.min(top, y); bottom = Math.max(bottom, y); black++;
    }
  }
  for (let y = top + 1; y < bottom; y++) {
    for (let x = left + 1; x < right; x++) {
      const index = (y * canvas.width + x) * 4;
      if (pixels[index] > 240 && pixels[index + 1] > 240 && pixels[index + 2] > 240) white++;
    }
  }
  // Pill corners reveal the white scene while the middle of the cap remains black.
  const corner=(top*canvas.width+left)*4;
  const cap=(Math.floor((top+bottom)/2)*canvas.width+left+2)*4;
  return {width:canvas.width, height:canvas.height, left, right, top, bottom, black, white,
    corner:Array.from(pixels.slice(corner,corner+3)),cap:Array.from(pixels.slice(cap,cap+3))};
}

/** Exercise the production TextLane through the maintained fixture at DPR 1 and 2. */
async function main() {
  const output = process.env.VIEWER_TEST_OUTPUT || '/tmp/session-viewer-nameplate';
  await fs.mkdir(output, {recursive:true});
  const browser = await chromium.launch({
    executablePath:process.env.CHROME_BIN || '/usr/bin/google-chrome',
    headless:process.env.VIEWER_HEADLESS === '1',
    args:JSON.parse(process.env.VIEWER_CHROME_ARGS || '[]'),
  });
  const reports = [];
  try {
    for (const dpr of [1, 2]) {
      const context = await browser.newContext({viewport:{width:1400,height:900},deviceScaleFactor:dpr});
      const page = await context.newPage();
      const errors = [];
      page.on('pageerror', function failed(error) { errors.push(String(error)); });
      page.on('console', function failed(message) {
        if (message.type() === 'error' && !message.text().includes('{{__trunk_address__}}')) errors.push(message.text());
      });
      await page.route('**/favicon.ico', function favicon(route) { return route.fulfill({status:204}); });
      await page.goto(new URL('text-quality.html',process.env.VIEWER_URL || 'http://127.0.0.1:8770/').href);
      await page.bringToFront();
      await page.waitForFunction(function ready() { return Boolean(window.textQuality?.report); }, null, {timeout:60000});
      await page.evaluate(function fixedSpecimenWidth() {
        document.getElementById('text-quality-canvas').style.width = '640px';
      });
      await page.evaluate(async function settleInitialResize() {
        await new Promise(requestAnimationFrame);
        await new Promise(requestAnimationFrame);
      });
      await page.selectOption('#specimen', 'nameplate');
      await page.evaluate(async function settleSpecimenResize() {
        await new Promise(requestAnimationFrame);
        await new Promise(requestAnimationFrame);
      });
      const first = await page.evaluate(function renderPlate() { return JSON.parse(window.textQuality.fixture.render_nameplate(0)); });
      const second = await page.evaluate(function renderPlate() { return JSON.parse(window.textQuality.fixture.render_nameplate(0)); });
      assert.equal(second.stats.shape_count, first.stats.shape_count, 'unchanged annotation must retain shaped text');
      assert.ok(second.stats.skipped_preparations > first.stats.skipped_preparations);
      assert.ok(second.stats.nameplate_capacity_bytes >= 48);
      const png = await page.locator('#text-quality-canvas').screenshot({path:path.join(output,`nameplate-dpr${dpr}.png`)});
      const pixels = await page.evaluate(measure, png.toString('base64'));
      const diagnostics=await page.evaluate(function shapedLine(){return JSON.parse(window.textQuality.fixture.diagnostics());});
      const lineWidth=diagnostics.glyphs[0].line_width;
      const straightWidth=(pixels.right-pixels.left+1)/dpr-25.5;
      assert.ok(straightWidth>=lineWidth-2/dpr,'the entire shaped line fits between the maximum-radius caps');
      const metrics=await page.evaluate(function referenceMetrics(){return window.textQuality.report.lineMetrics[0];});
      assert.ok(Math.abs(metrics.browser-metrics.shaper)<0.2,'same-font reference agrees after excluding the full cap padding');
      assert.ok(pixels.black > 800 * dpr * dpr, 'opaque black background must be actual GPU output');
      assert.deepEqual(pixels.corner,[255,255,255], 'maximum rounded corners expose the white scene');
      assert.deepEqual(pixels.cap,[0,0,0], 'maximum rounded cap retains a black middle');
      assert.ok(pixels.white > 50 * dpr * dpr, 'normal-size white glyph interiors must be visible on that background');
      assert.ok(Math.abs((pixels.left + pixels.right) / 2 - pixels.width / 2) <= 1);
      assert.ok(Math.abs((pixels.top + pixels.bottom) / 2 - pixels.height / 2) <= 1);
      assert.ok(Math.abs((pixels.bottom - pixels.top + 1) / dpr - 25.5) <= 2, '19.5px line box plus 3px vertical padding');
      const reset = await page.evaluate(function releasePlate() {
        window.textQuality.fixture.reset();
        return JSON.parse(window.textQuality.fixture.render_nameplate(0));
      });
      assert.equal(reset.stats.shape_count, first.stats.shape_count + 1, 'released source label reshapes exactly once');
      assert.deepEqual(errors, []);
      reports.push({dpr, first, second, reset, pixels});
      await context.close();
    }
  } finally { await browser.close(); }
  await fs.writeFile(path.join(output,'nameplate.json'),JSON.stringify(reports,null,2) + '\n');
  console.log('PASS centered13.5px rounded nameplate: black/white GPU pixels, DPR1/2, cached preparation and release');
}

main().catch(function failed(error) { console.error(error); process.exitCode = 1; });
