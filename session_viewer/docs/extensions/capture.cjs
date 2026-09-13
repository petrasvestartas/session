const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const { chromium } = require('playwright');
const browserUi = require('./browser.cjs');
const output = path.resolve(__dirname, '../screenshots');
const args = JSON.parse(process.env.VIEWER_CHROME_ARGS || '[]');

async function state(page) {
  return page.evaluate(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')));
}
async function settle(page, before) {
  await page.waitForFunction(before => {
    const raw = document.querySelector('canvas')?.getAttribute('data-viewer-inspection');
    if (!raw) return false;
    const s = JSON.parse(raw);
    return s.submitted_at_ms > before && !s.pick_busy;
  }, before);
}
async function key(page, key) {
  const before = (await state(page)).submitted_at_ms;
  await page.keyboard.press(key);
  await settle(page, before);
  await page.waitForTimeout(150);
}
async function command(page, text) {
  await browserUi.command(page, text);
}

async function button(page, prefix, label) {
  await browserUi.button(page, prefix + '/', label);
}

function project(s, p) {
  const v = [p[0] - s.origin[0], p[1] - s.origin[1], p[2] - s.origin[2], 1];
  const c = [0, 1, 2, 3].map(r => s.mvp[r]*v[0]+s.mvp[4+r]*v[1]+s.mvp[8+r]*v[2]+s.mvp[12+r]);
  return [(c[0]/c[3]*.5+.5)*1200,(.5-c[1]/c[3]*.5)*800];
}
async function click(page, point) {
  const before = (await state(page)).submitted_at_ms;
  await page.mouse.move(...point);
  await page.waitForTimeout(60);
  await page.mouse.click(...point);
  await page.waitForTimeout(150);
  await settle(page, before);
}
async function main() {
  await fs.mkdir(output, { recursive: true });
  const browser = await chromium.launch({ executablePath: process.env.CHROME_BIN || '/usr/bin/google-chrome', headless: false, args });
  const report = { browser: await browser.version(), headless: false, args, gpuEnvironment: Object.fromEntries(['VK_DRIVER_FILES', '__NV_PRIME_RENDER_OFFLOAD', '__GLX_VENDOR_LIBRARY_NAME'].map(key => [key, process.env[key] || ''])), captures: [] };
  try {
    const page = await browser.newPage({ viewport: { width: 1200, height: 800 } });
    const errors = [];
    page.on('pageerror', e => errors.push(String(e)));
    page.on('console', m => { if (m.type() === 'error') { errors.push(m.text()); console.error(m.text()); } });
    const fixture = await fs.readFile(path.join(__dirname, 'nested.pb'));
    await page.route('**/view_local.yaml', r => r.fulfill({ contentType: 'application/yaml', body: 'name: Editing lessons\nitems:\n  - file: extension-nested.pb\n' }));
    await page.route('**/extension-nested.pb', r => r.fulfill({ contentType: 'application/octet-stream', body: fixture }));
    await page.route('**/favicon.ico', r => r.fulfill({ status: 204 }));
    await page.goto(process.env.VIEWER_URL || 'http://localhost:8791/?data=off&inspect=1');

    await page.waitForFunction(() => {
      const raw = document.querySelector('canvas')?.getAttribute('data-viewer-inspection');
      return raw && JSON.parse(raw).objects >= 3;
    });
    await page.locator('canvas').focus();
    await key(page, '5');
    await key(page, 'f');
    await page.mouse.move(600,400);
    await page.mouse.wheel(0,450);
    await page.waitForTimeout(200);
    await key(page, 'l');
    await button(page, 'open', 'Editing lessons');
    await button(page, 'open', 'Editing lessons');
    await button(page, 'open', 'Assembly');
    await button(page, 'open', 'Nested');
    await button(page, 'select', 'Select Nested');
    assert.equal((await state(page)).selected_group_count, 2);
    await page.screenshot({ path: path.join(output, 'extensions-panels.png') });
    report.captures.push({ file: 'extensions-panels.png', operation: 'Nested selected', state: await state(page) });
    await key(page, 'l');
    await key(page, 'Escape');
    await click(page, project(await state(page), [65, 0, 0]));
    assert.notEqual((await state(page)).selected, null);
    await key(page, 'F10');
    await click(page, project(await state(page), [110, 90, 0]));
    const at = project(await state(page), [110, 90, 0]);
    await page.mouse.move(...at);
    await page.mouse.down();
    await page.mouse.move(at[0]-70, at[1]-50, {steps:8});
    await page.mouse.up();
    await page.waitForTimeout(300);
    await page.screenshot({ path: path.join(output, 'extensions-controls.png') });
    report.captures.push({ file: 'extensions-controls.png', operation: 'Placed polyline control dragged', state: await state(page) });
    await key(page, 'Escape');
    await command(page, 'line -100,130,0 110,130,0');
    await key(page, 'Escape');
    await key(page, 'f');
    await page.mouse.move(600,400);
    await page.mouse.wheel(0,300);
    await page.waitForTimeout(200);
    await click(page, project(await state(page), [0, 130, 0]));
    await command(page, 'trim 0.2 0.8');
    await click(page, project(await state(page), [0, 130, 0]));
    await page.screenshot({ path: path.join(output, 'extensions-modeling.png') });
    report.captures.push({ file: 'extensions-modeling.png', operation: 'Created line trimmed to normalized interval 0.2–0.8', state: await state(page) });
    await key(page, '7');
    await key(page, 't');
    await page.mouse.move(1000,700);
    await page.waitForTimeout(150);
    await page.screenshot({ path: path.join(output, 'extensions-gumball-overview.png') });
    const center = project(await state(page), (await state(page)).widget[0]);
    await page.screenshot({ path: path.join(output, 'extensions-gumball.png'), clip: { x: Math.round(center[0]-140), y: Math.round(center[1]-140), width: 280, height: 280 } });
    report.captures.push({ file: 'extensions-gumball.png', operation: 'Cylindrical gumball and cone tips in isometric view', state: await state(page) });
    await page.reload();
    await page.waitForFunction(() => {
      const raw = document.querySelector('canvas')?.getAttribute('data-viewer-inspection');
      return raw && JSON.parse(raw).objects === 3;
    });
    await page.locator('canvas').focus();
    await key(page, '5');
    async function shot(file, operation) {
      await page.waitForTimeout(150);
      await page.screenshot({path:path.join(output,file)});
      report.captures.push({file,operation,state:await state(page),ui:await browserUi.ui(page)});
    }
    await browserUi.typeCommand(page,'line 0,130,0 100,130,0');
    await shot('extensions-command-interface.png','Type a complete line command in egui');
    await browserUi.command(page,'line 0,130,0 100,130,0',false);
    await shot('extensions-command-create.png','Enter creates the line and displays feedback');
    await browserUi.button(page,'command/close','Close');
    await key(page,'f');
    await click(page,project(await state(page),[50,130,0]));
    await browserUi.typeCommand(page,'trim 0.2 0.8');
    await shot('extensions-command-trim.png','Select the line before entering trim');
    await browserUi.command(page,'trim 0.2 0.8');
    await click(page,project(await state(page),[50,130,0]));
    await browserUi.command(page,'extend -0.2 1.2',false);
    await shot('extensions-command-extend.png','Extend the selected trimmed line');
    await browserUi.button(page,'command/close','Close');
    await browserUi.command(page,'polyline 0,180,0 100,180,0 100,230,0');
    await key(page,'f');
    await click(page,project(await state(page),[50,180,0]));
    await browserUi.command(page,'explode',false);
    await shot('extensions-command-explode.png','Explode the selected polyline into two lines');
    await browserUi.command(page,'undo',false);
    await shot('extensions-command-undo.png','Undo restores the polyline');
    assert.deepEqual(errors, []);
    await fs.writeFile(path.join(__dirname, 'screenshots.json'), JSON.stringify(report, null, 2)+'\n');
    console.log('PASS actual viewer captures: nested selection, placed control drag, unlit gumball and command-line create/trim/extend/explode/undo');
  } finally { await browser.close(); }
}
main().catch(e => { console.error(e); process.exitCode = 1; });
