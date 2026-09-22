/** Real WebGPU validation, contact lighting, projection changes and bounded resources. */
const assert = require('node:assert/strict');
const path = require('node:path');
const fs = require('node:fs/promises');
const {chromium} = require('playwright');
const {PNG} = require('pngjs');
const root = path.resolve(__dirname, '..');
async function snapshot(page) {
  return JSON.parse(await page.locator('canvas').getAttribute('data-viewer-inspection'));
}
async function cadence(page) {
  const start = await snapshot(page);
  await page.waitForTimeout(2000);
  const end = await snapshot(page);
  return Math.round(1000 * (end.frames - start.frames) / (end.submitted_at_ms - start.submitted_at_ms));
}
(async () => {
  const browser = await chromium.launch({executablePath: '/usr/bin/google-chrome',
    headless: process.env.HEADLESS === '1', args: ['--enable-unsafe-webgpu',
      '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--ozone-platform=x11']});
  const page = await browser.newPage({viewport: {width: 1440, height: 1000}});
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  try {
    await page.route('**/view_local.yaml', route => route.fulfill({contentType: 'application/yaml',
      body: 'name: Ambient regression\nitems:\n  - file: ambient.pb\n'}));
    await page.route('**/ambient.pb', route => route.fulfill({path:
      process.env.AMBIENT_FIXTURE || path.join(root, 'assets/pb/view_mixed_teapot.pb')}));
    await page.route('**/favicon.ico', route => route.fulfill({status: 204}));
    await page.goto(new URL('?inspect=1&data=off&nogrid=1&perf=1' +
      (process.env.AMBIENT_SPIN ? '&spin=1' : ''), process.env.VIEWER_URL || 'http://127.0.0.1:8770/').href);
    await page.waitForFunction(() => {
      const text = document.querySelector('canvas')?.getAttribute('data-viewer-inspection');
      return text && JSON.parse(text).vertices > 0;
    }, {}, {timeout: 90000});
    await page.keyboard.press('Escape');
    await page.waitForTimeout(1000);
    const off = await snapshot(page);
    const white = PNG.sync.read(await page.screenshot());
    const corner = (100*white.width+30)*4;
    assert.deepEqual([...white.data.subarray(corner,corner+3)],[255,255,255],"Arctic off has a white background");
    const fpsOff = await cadence(page);
    const ui = JSON.parse(await page.locator('canvas').getAttribute('data-viewer-ui'));
    const [x, y, right, bottom] = ui.controls.find(control => control.key === 'command/input').rect;
    // No artificial wait after clicking: the first letter must survive the focus change.
    await page.mouse.click((x + right) / 2, (y + bottom) / 2);
    await page.keyboard.type('SSAO On');
    await page.keyboard.press('Enter');
    await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).ssao);
    await page.waitForTimeout(1000);
    const on = await snapshot(page);
    const grey = PNG.sync.read(await page.screenshot());
    assert(grey.data[corner]<255 && grey.data[corner]>230,"Arctic on has a light grey background");
    assert(JSON.parse(await page.locator('canvas').getAttribute('data-viewer-ui')).command_open, 'Enter keeps the command field focused');
    const extraTextures = on.gpu_texture_estimate_bytes - off.gpu_texture_estimate_bytes;
    assert(extraTextures > 0 && extraTextures <= 2 * 960 * 960, 'two capped R8 textures');
    assert.equal(on.gpu_buffer_capacity_bytes - off.gpu_buffer_capacity_bytes, 144);
    const caret = PNG.sync.read(await page.screenshot());
    let darkestColumn = 0;
    for (let px = Math.floor(x)-1; px <= Math.ceil(x)+1; px++) {
      let dark = 0;
      for (let py = Math.floor((y+bottom)/2)-7; py <= Math.ceil((y+bottom)/2)+7; py++) {
        const at = (py*caret.width+px)*4;
        if(caret.data[at]<100 && caret.data[at+1]<100 && caret.data[at+2]<100) dark++;
      }
      darkestColumn = Math.max(darkestColumn,dark);
    }
    assert(darkestColumn>=10,'the empty command field paints a visible dark caret');
    const fpsOn = await cadence(page);
    // Enter leaves the caret ready: run another command without clicking again.
    await page.keyboard.type('SSAO On');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(500);
    assert(JSON.parse(await page.locator('canvas').getAttribute('data-viewer-ui')).history.at(-1).startsWith('> SSAO On\n'));
    if (process.env.SCREENSHOTS) {
      await fs.mkdir(path.join(root, 'target/review'), {recursive: true});
      await page.screenshot({path: path.join(root, 'target/review/ambient-regression.png')});
    }
    await page.keyboard.press('Escape');
    await page.mouse.move(800, 350);
    await page.mouse.down({button: 'right'});
    await page.mouse.move(940, 460, {steps: 12});
    await page.mouse.up({button: 'right'});
    await page.keyboard.press('Space');
    await page.waitForTimeout(750);
    await page.keyboard.press('g');
    await page.waitForTimeout(500);
    const released = await snapshot(page);
    assert.equal(released.ssao, false);
    assert.equal(released.gpu_texture_estimate_bytes, off.gpu_texture_estimate_bytes);
    assert.equal(released.gpu_buffer_capacity_bytes, off.gpu_buffer_capacity_bytes);
    assert.deepEqual(errors, []);
    console.log(JSON.stringify({result: 'PASS', vertices: on.vertices, extraTextures,
      additionalUniformBytes: 144, frameCadenceFps: {off: fpsOff, on: fpsOn}, rotating: !!process.env.AMBIENT_SPIN}));
  } catch (error) {
    console.error(await page.locator('canvas').getAttribute('data-viewer-ui'));
    await page.screenshot({path: path.join(root, 'target/review/ambient-failure.png')});
    throw error;
  } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
