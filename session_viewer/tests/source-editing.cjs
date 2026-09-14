/** Verify original mesh/NURBS/BRep subobjects, undo, and a touch control-gumball edit. */
const { chromium } = require('playwright');
const assert = require('node:assert/strict');
const ui = require('../docs/extensions/browser.cjs');
const state = async page => JSON.parse(await page.locator('canvas').getAttribute('data-viewer-inspection'));
async function settle(page) {
  await page.waitForTimeout(200);
  await page.waitForFunction(() => !JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).pick_busy);
}
function project(s, point) {
  const v = point.map((x, i) => x - s.origin[i]);
  const c = [0, 1, 2, 3].map(r => s.mvp[r]*v[0] + s.mvp[4+r]*v[1] + s.mvp[8+r]*v[2] + s.mvp[12+r]);
  return [(c[0]/c[3]*.5+.5)*s.logical_canvas[0], (.5-c[1]/c[3]*.5)*s.logical_canvas[1]];
}
async function pick(page, tool, point) {
  await ui.button(page, `toolbar/${tool}`, '');
  await page.mouse.click(...project(await state(page), point));
  await settle(page);
}
(async () => {
  const browser = await chromium.launch({ executablePath: process.env.CHROME_BIN || '/usr/bin/google-chrome', headless: false, args: JSON.parse(process.env.VIEWER_CHROME_ARGS || '[]') });
  try {
    const page = await browser.newPage({ viewport: { width: 1400, height: 900 }, hasTouch: true });
    const errors = [];
    page.on('pageerror', e => errors.push(e.message));
    await page.route('**/view_local.yaml', r => r.fulfill({ contentType: 'application/yaml', body: 'name: Source editing\nitems:\n  - file: fixture.pb\n' }));
    await page.route('**/fixture.pb', r => r.fulfill({ path: process.env.VIEWER_INTERACTION_FIXTURE || '/tmp/viewer-interaction.pb' }));
    await page.goto(new URL('?inspect=1', process.env.VIEWER_URL || 'http://127.0.0.1:8770/').href);
    await page.waitForFunction(() => {
      const s = document.querySelector('canvas')?.getAttribute('data-viewer-inspection');
      return s && JSON.parse(s).objects >= 7;
    });
    await ui.button(page, 'layers/close', 'Close layers');
    await page.locator('canvas').focus();
    await page.keyboard.press('5');
    await page.keyboard.press('f');
    await settle(page);
    for (const [kind, face, edge] of [
      ['mesh', [-9,3,.2], [-9,4,.2]],
      ['surface', [-9,-3,0], [-9,-2,0]],
      ['brep', [-3,-3,.2], [-3,-2,.2]],
    ]) {
      for (const [tool, point] of [['Face', face], ['Edge', edge]]) {
        await pick(page, tool, point);
        const before = await state(page);
        assert(before.selection[tool], `${kind} ${tool} selected`);
        await ui.command(page, 'move 0 0 1');
        assert.equal((await ui.ui(page)).history.at(-1), '> move 0 0 1\nmove');
        assert.deepEqual((await state(page)).model, before.model, 'parent placement stays unchanged');
        await ui.button(page, 'toolbar/Vtx', 'source');
        await settle(page);
        assert((await state(page)).controls.some(c => c.position[2] > .5), `${kind} original controls moved`);
        await ui.command(page, 'undo');
        await ui.button(page, 'toolbar/Obj', 'objects');
      }
    }
    await pick(page, 'Obj', [-9,3,.2]);
    await ui.button(page, 'toolbar/Vtx', 'source');
    let s = await state(page);
    const initial = s.controls;
    const control = initial.find(c => c.position[0] > 0 && c.position[2] > 0);
    assert(control, 'visible top mesh vertex');
    const world = [0,1,2].map(r => s.model[r]*control.position[0] + s.model[4+r]*control.position[1] + s.model[8+r]*control.position[2] + s.model[12+r] + s.origin[r]);
    await page.mouse.click(...project(s, world));
    await settle(page);
    s = await state(page);
    assert(s.selection.Controls.selected, 'original vertex selected');
    const [origin, scale] = s.widget;
    const start = project(s, [origin[0] + 86*scale, origin[1], origin[2]]);
    const cdp = await page.context().newCDPSession(page);
    await cdp.send('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: [{ x: start[0], y: start[1], id: 0 }] });
    await cdp.send('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: [{ x: start[0]+30, y: start[1], id: 0 }] });
    await settle(page);
    await cdp.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
    await settle(page);
    assert.deepEqual((await state(page)).model, s.model, 'touch vertex edit preserves parent placement');
    assert.notDeepEqual((await state(page)).controls, initial, 'touch gumball changes original mesh vertex');
    await ui.command(page, 'undo');
    await pick(page, 'Obj', [-9,3,.2]);
    await ui.button(page, 'toolbar/Vtx', 'source');
    assert.deepEqual((await state(page)).controls, initial, 'undo restores original mesh controls');
    assert.deepEqual(errors, []);
    console.log('PASS source mesh/NURBS/BRep face and edge edits, touch mesh vertex, parent placement and undo');
  } finally { await browser.close(); }
})().catch(e => { console.error(e); process.exitCode = 1; });
