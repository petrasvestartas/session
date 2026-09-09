/** Preserve the existing 32-patch teapot as editable source CAD through the real browser loader. */
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const crypto = require('node:crypto');
const {chromium} = require('playwright');

/** Read submitted state without adding a test-only mutation interface. */
function snapshot() {
  return JSON.parse(document.querySelector('#canvas')?.getAttribute('data-viewer-inspection') || '{}');
}

/** Count shaded body pixels independently of black labels, grid and white background. */
async function shadedPixels(base64) {
  const image = await createImageBitmap(await (await fetch('data:image/png;base64,' + base64)).blob());
  const canvas = document.createElement('canvas');
  canvas.width = image.width; canvas.height = image.height;
  const context = canvas.getContext('2d'); context.drawImage(image, 0, 0); image.close();
  const data = context.getImageData(0, 0, canvas.width, canvas.height).data;
  let count = 0;
  for (let i = 0; i < data.length; i += 4) {
    if (data[i] > 40 && data[i] < 220 && data[i + 1] > 40 && data[i + 1] < 220) count++;
  }
  return count;
}

/** Sample the actual cubic foot boundary, independently of its display triangulation. */
async function footBoundaryPixels({base64, state}) {
  const image = await createImageBitmap(await (await fetch('data:image/png;base64,' + base64)).blob());
  const canvas = document.createElement('canvas');
  canvas.width = image.width; canvas.height = image.height;
  const context = canvas.getContext('2d'); context.drawImage(image, 0, 0); image.close();
  const pixels = context.getImageData(0, 0, canvas.width, canvas.height).data;
  const radius = Math.ceil(canvas.width / state.logical_canvas[0]);
  let visible = 0, total = 0;
  for (let quadrant = 0; quadrant < 4; quadrant++) {
    for (let i = 0; i < 160; i++) {
      const t = (i + .5) / 160, s = 1 - t;
      // Original teapot patch controls at z=15, in source millimetres.
      let x = 150*s*s*s + 450*s*s*t + 252*s*t*t;
      let y = -252*s*s*t - 450*s*t*t - 150*t*t*t;
      for (let q = 0; q < quadrant; q++) [x, y] = [-y, x];
      const point = [x-state.origin[0], y-state.origin[1], 15-state.origin[2], 1];
      const clip = [0, 0, 0, 0];
      for (let r = 0; r < 4; r++) for (let c = 0; c < 4; c++) clip[r] += state.mvp[c*4+r]*point[c];
      const px = (clip[0]/clip[3]*.5+.5)*canvas.width;
      const py = (.5-clip[1]/clip[3]*.5)*canvas.height;
      let found = false;
      for (let dy = -radius; dy <= radius; dy++) for (let dx = -radius; dx <= radius; dx++) {
        const ix = Math.floor(px)+dx, iy = Math.floor(py)+dy;
        if (ix < 0 || iy < 0 || ix >= canvas.width || iy >= canvas.height) continue;
        const offset = (iy*canvas.width+ix)*4;
        found ||= Math.max(pixels[offset], pixels[offset+1], pixels[offset+2]) < 180;
      }
      visible += found; total++;
    }
  }
  return {visible, total};
}

/** Check original bytes, shaded surface, source identity, 512 surface controls and label restoration. */
async function main() {
  const asset = path.join(__dirname, '../assets/pb/view_mixed_teapot.pb');
  const bytes = await fs.readFile(asset);
  assert.equal(crypto.createHash('sha256').update(bytes).digest('hex'),
    '6e3866e40ee3b669cfeebd9180ff4a4c938c980244f6b405924a8024c9fc4b1f');
  const output = process.env.VIEWER_TEST_OUTPUT || '/tmp/session-viewer-teapot';
  await fs.mkdir(output, {recursive:true});
  const browser = await chromium.launch({
    executablePath:process.env.CHROME_BIN || '/usr/bin/google-chrome',
    headless:process.env.VIEWER_HEADLESS === '1',
    args:JSON.parse(process.env.VIEWER_CHROME_ARGS || '[]'),
  });
  const reports = [];
  try {
    for (const dpr of [1, 2]) {
      const context = await browser.newContext({viewport:{width:1000,height:700},deviceScaleFactor:dpr});
      const page = await context.newPage(), errors = [];
      page.on('pageerror', function capture(error) { errors.push(String(error)); });
      await page.route('**/view_local.yaml', function manifest(route) {
        return route.fulfill({status:200,contentType:'application/yaml',
          body:'name: Teapot inspection\nitems:\n  - file: "pb/view_mixed_teapot.pb"\n    name: "Utah teapot (32 NURBS patches)"\n'});
      });
      await page.route('**/pb/view_mixed_teapot.pb', function source(route) {
        return route.fulfill({status:200,contentType:'application/octet-stream',body:bytes});
      });
      await page.goto(new URL('?data=off&inspect=1', process.env.VIEWER_URL || 'http://127.0.0.1:8771/').href);
      await page.bringToFront();
      await page.waitForFunction(function loaded() {
        const s = JSON.parse(document.querySelector('#canvas')?.getAttribute('data-viewer-inspection') || '{}');
        // Two source rows: the teapot and its document-title text object (a source row since
        // authored text became selectable), so the count is 2.
        return s.objects === 2 && s.frames > 0;
      }, null, {timeout:60000});
      await page.waitForTimeout(150);
      const initial = await page.evaluate(snapshot);
      assert(initial.vertices > 1000 && initial.pipes > 100, 'source patches and boundaries must tessellate');
      const png = await page.locator('#canvas').screenshot({path:path.join(output,`teapot-dpr${dpr}.png`)});
      assert(await page.evaluate(shadedPixels, png.toString('base64')) > 10000 * dpr * dpr);
      await page.keyboard.press('6');
      await page.waitForFunction(function bottomFrame(previous) {
        return JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection')).frames > previous;
      }, initial.frames);
      const bottom = await page.evaluate(snapshot);
      const footPng = await page.locator('#canvas').screenshot({path:path.join(output,`teapot-foot-dpr${dpr}.png`)});
      const foot = await page.evaluate(footBoundaryPixels, {base64:footPng.toString('base64'),state:bottom});
      assert(foot.visible >= .99*foot.total, `concave foot boundary has missing ink: ${JSON.stringify(foot)}`);
      await page.keyboard.press('7'); await page.waitForTimeout(150);
      await page.mouse.click(500, 350);
      await page.waitForFunction(function selected() {
        return JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection')).selected === 0;
      });
      const selected = await page.evaluate(snapshot);
      assert.equal(selected.identity[1], 'ab614062-c23b-4207-9512-a77df706fe9f');
      await page.keyboard.press('F10'); await page.waitForTimeout(150);
      const controls = await page.evaluate(snapshot);
      let surfaces = 0;
      for (const control of controls.controls) if (control.id.Surface) surfaces++;
      assert.equal(surfaces, 32 * 4 * 4, 'all original NURBS control IDs must survive');
      assert.equal(controls.selection.Controls.parent, 0);
      await page.screenshot({path:path.join(output,`teapot-controls-dpr${dpr}.png`)});
      await page.keyboard.press('Escape'); await page.waitForTimeout(150);
      const restored = await page.evaluate(snapshot);
      assert.equal(restored.markers, 0); assert.equal(restored.selected, 0);
      assert.deepEqual(errors, []);
      reports.push({dpr,initial,selected,surface_controls:surfaces,foot,restored,errors});
      await context.close();
    }
  } finally { await browser.close(); }
  await fs.writeFile(path.join(output,'teapot.json'),JSON.stringify(reports,null,2) + '\n');
  console.log('PASS existing NURBS teapot: shaded32-patch source, identity,512 surface controls,DPR1/2');
}

main().catch(function failed(error) { console.error(error); process.exitCode = 1; });
