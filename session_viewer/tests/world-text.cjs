/** Fixed-plane text is authored in a manifest and projected by the real browser renderer. */
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const {chromium} = require('playwright');

/** Read the last submitted source and text resource measurements. */
function snapshot() {
  return JSON.parse(document.querySelector('#canvas')?.getAttribute('data-viewer-inspection') || '{}');
}

/** Locate the fixed source item independently of annotation ID allocation. */
function plane(state) {
  for (const item of state.text_labels) if (item.placement === 'world_plane') return item;
  throw Error('missing authored world-plane text');
}

/** Count actual glyph and backing pixels, excluding white scene pixels outside the backing. */
async function coverage(base64) {
  const bitmap = await createImageBitmap(await (await fetch('data:image/png;base64,' + base64)).blob());
  const canvas = document.createElement('canvas'); canvas.width = bitmap.width; canvas.height = bitmap.height;
  const context = canvas.getContext('2d'); context.drawImage(bitmap, 0, 0); bitmap.close();
  const data = context.getImageData(0, 0, canvas.width, canvas.height).data;
  let black = 0, whiteInBlack = 0;
  for (let y = 4; y < canvas.height - 4; y++) {
    for (let x = 4; x < canvas.width - 4; x++) {
      const i = (y * canvas.width + x) * 4;
      if (data[i] < 8 && data[i + 1] < 8 && data[i + 2] < 8) black++;
      if (data[i] < 240 || data[i + 1] < 240 || data[i + 2] < 240) continue;
      let dark = 0;
      for (const [dx,dy] of [[-4,0],[4,0],[0,-4],[0,4]]) {
        const n = ((y + dy) * canvas.width + x + dx) * 4;
        if (data[n] < 8 && data[n + 1] < 8 && data[n + 2] < 8) dark++;
      }
      if (dark >= 2) whiteInBlack++;
    }
  }
  return {black,whiteInBlack};
}

/** Project one world coordinate through the submitted rebased camera. */
function project(state, point) {
  const p = [point[0] - state.origin[0],point[1] - state.origin[1],point[2] - state.origin[2],1];
  const clip = [0,0,0,0];
  for (let r = 0; r < 4; r++) for (let c = 0; c < 4; c++) clip[r] += state.mvp[c * 4 + r] * p[c];
  return [clip[0]/clip[3],clip[1]/clip[3]];
}

/** Verify authored axes, real perspective foreshortening, white/black pixels and T independence. */
async function main() {
  const output = process.env.VIEWER_TEST_OUTPUT || '/tmp/session-viewer-world-text';
  await fs.mkdir(output,{recursive:true});
  const browser = await chromium.launch({executablePath:process.env.CHROME_BIN || '/usr/bin/google-chrome',
    headless:process.env.VIEWER_HEADLESS === '1',args:JSON.parse(process.env.VIEWER_CHROME_ARGS || '[]')});
  const reports = [];
  try {
    for (const dpr of [1,2]) {
      const context = await browser.newContext({viewport:{width:1000,height:700},deviceScaleFactor:dpr});
      const page = await context.newPage(), errors = [];
      page.on('pageerror',function failed(error) { errors.push(String(error)); });
      await page.route('**/view_local.yaml',function manifest(route) {
        return route.fulfill({status:200,contentType:'application/yaml',body:
          'name: Fixed text\nitems: []\ntexts:\n  - text: text_not_oriented_to_camera\n    at: [0,0,10]\n    height: 32\n'});
      });
      await page.goto(new URL('?data=off&inspect=1',process.env.VIEWER_URL || 'http://127.0.0.1:8771/').href);
      await page.bringToFront();
      await page.waitForFunction(function ready() {
        const s = JSON.parse(document.querySelector('#canvas')?.getAttribute('data-viewer-inspection') || '{}');
        return s.text?.world_plane_texture_bytes > 0 && s.frames > 0;
      },null,{timeout:60000});
      await page.locator('#canvas').focus();
      const first = await page.evaluate(snapshot), label = plane(first);
      assert.equal(label.text,'text_not_oriented_to_camera');
      assert.deepEqual(label.plane,[[1,0,0],[0,1,0],32]);
      const png = await page.locator('#canvas').screenshot({path:path.join(output,`plane-iso-dpr${dpr}.png`)});
      const pixels = await page.evaluate(coverage,png.toString('base64'));
      assert(pixels.black > 1000 && pixels.whiteInBlack > 30);
      const a = project(first,label.world), b = project(first,[label.world[0]+100,label.world[1],label.world[2]]);
      assert(Math.abs(a[1]-b[1]) > .01,'fixed baseline tilts in the isometric camera');
      await page.keyboard.press('5'); await page.waitForTimeout(120);
      const top = await page.evaluate(snapshot);
      assert.deepEqual(plane(top).world,label.world); assert.deepEqual(plane(top).plane,label.plane);
      const c = project(top,label.world), d = project(top,[label.world[0]+100,label.world[1],label.world[2]]);
      assert(Math.abs(c[1]-d[1]) < .0001,'world-X baseline becomes horizontal from above');
      assert.equal(top.text.shape_count,first.text.shape_count);
      await page.keyboard.press('t'); await page.waitForTimeout(100);
      assert.equal(plane(await page.evaluate(snapshot)).text,label.text,'T only toggles selected-item annotations');
      await page.locator('#canvas').screenshot({path:path.join(output,`plane-top-dpr${dpr}.png`)});
      const unit = label.plane[2] / label.font_size;
      const center = label.world.map((value, axis) => value + label.plane[0][axis]*label.line_box[0]*unit/2 - label.plane[1][axis]*label.line_box[1]*unit/2);
      const ndc = project(top, center), at = [(ndc[0]*.5+.5)*1000,(.5-ndc[1]*.5)*700];
      await page.mouse.click(...at);
      await page.waitForFunction(() => JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection')).selected !== null);
      const selected = await page.evaluate(snapshot);
      assert.equal(selected.selection, 'Object');
      assert.deepEqual(plane(selected).color,[0,0,0,255], 'selection changes glyphs to black');
      await page.keyboard.press('h');
      await page.waitForFunction(() => !JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection')).text_labels.some(label => label.placement === 'world_plane'));
      await page.mouse.click(...at); await page.waitForTimeout(100);
      assert.equal((await page.evaluate(snapshot)).selected,null,'hidden text cannot be picked');
      await page.keyboard.press('s');
      await page.waitForFunction(() => JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection')).text_labels.some(label => label.placement === 'world_plane'));
      await page.mouse.click(...at);
      await page.waitForFunction(row => JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection')).selected === row, selected.selected);
      await page.keyboard.press('f'); await page.waitForTimeout(100);
      await page.locator('#canvas').screenshot({path:path.join(output,`plane-selected-dpr${dpr}.png`)});
      assert.deepEqual(errors,[]); reports.push({dpr,first,top,pixels,selected,errors}); await context.close();
    }
  } finally { await browser.close(); }
  await fs.writeFile(path.join(output,'world-text.json'),JSON.stringify(reports,null,2) + '\n');
  console.log('PASS fixed world text: authored plane, orbit projection, white/black pixels,DPR1/2,T independence');
}
main().catch(function failed(error) { console.error(error); process.exitCode = 1; });
