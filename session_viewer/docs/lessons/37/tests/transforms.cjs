/** Rhino-like Move, Copy, Rotate, Scale, Orient 3 Points and Explode on the interaction fixture, mouse, keys and touch. */
const fs = require('node:fs/promises');
const path = require('node:path');
const {chromium, devices} = require('playwright');
const base = process.env.VIEWER_URL || 'http://127.0.0.1:8770/';
const fixture = process.env.FIXTURE || '/tmp/interaction.pb';
const out = process.env.OUT || '/tmp/viewer-transforms';
const results = [];
const check = (ok, text) => { results.push(`${ok ? 'PASS' : 'FAIL'} ${text}`); console.log(results.at(-1)); return ok; };
const state = page => page.evaluate(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')));
const ui = page => page.evaluate(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')));
const near = (a, b, eps = 1e-6) => a.length === b.length && a.every((x, i) => Math.abs(x - b[i]) <= eps);
async function settle(page, ms = 250) { await page.waitForTimeout(ms); await page.waitForFunction(() => !JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).pick_busy); }
function project(s, p) { const v = p.map((n, i) => n - s.origin[i]); const c = [0, 1, 2, 3].map(r => s.mvp[r] * v[0] + s.mvp[r + 4] * v[1] + s.mvp[r + 8] * v[2] + s.mvp[r + 12]); return [(c[0] / c[3] * .5 + .5) * s.logical_canvas[0], (.5 - c[1] / c[3] * .5) * s.logical_canvas[1]]; }
// the world matrix of a selected row: the published one is relative to the render origin
function world(s, index = 0) { const m = [...s.selected_models[index]]; for (let i = 0; i < 3; i++) m[12 + i] += s.origin[i]; return m; }
const apply = (m, p) => [0, 1, 2].map(r => m[r] * p[0] + m[4 + r] * p[1] + m[8 + r] * p[2] + m[12 + r]);
async function control(page, key) { const c = (await ui(page)).controls.filter(c => c.key === key).at(-1); if (!c) return false; const [a, b, x, y] = c.rect; await page.mouse.click((a + x) / 2, (b + y) / 2); await settle(page); return true; }
// type a line into the command field and press Enter; the keys stay with the field, nothing clicks the scene
async function enter(page, text) { await control(page, 'command/input'); await page.keyboard.press('Control+a'); if (text) await page.keyboard.type(text); await page.keyboard.press('Enter'); await settle(page); }
async function hover(page, p) { const at = project(await state(page), p); await page.mouse.move(at[0], at[1], {steps: 3}); await settle(page, 120); return at; }
async function click(page, p) { const at = await hover(page, p); await page.mouse.click(at[0], at[1]); await settle(page); }
// a press on the "Command:" label closes the field, so the next keys reach the viewer and nothing in the scene is clicked
async function blur(page) { const c = (await ui(page)).controls.find(c => c.key === 'command/input'); await page.mouse.click(c.rect[0] - 30, (c.rect[1] + c.rect[3]) / 2); await settle(page, 120); }
async function key(page, name) { await blur(page); await page.keyboard.press(name); await settle(page); }
// zoom out until the box from `lo` to `hi` on z = 0 is on screen
async function frame(page, lo, hi) {
  let delta = 240;
  const inside = async () => { const s = await state(page); const r = (await ui(page)).scene_rect; return [[lo[0], lo[1], 0], [hi[0], hi[1], 0], [lo[0], hi[1], 0], [hi[0], lo[1], 0]].map(p => project(s, p)).every(([x, y]) => x > r[0] + 20 && x < r[2] - 20 && y > r[1] + 20 && y < r[3] - 20); };
  const spread = async () => { const s = await state(page); const [a, b] = [project(s, [lo[0], lo[1], 0]), project(s, [hi[0], hi[1], 0])]; return Math.hypot(a[0] - b[0], a[1] - b[1]); };
  await page.mouse.move(640, 380);
  for (let i = 0; i < 30 && !(await inside()); i++) {
    const before = await spread();
    await page.mouse.wheel(0, delta); await settle(page, 80);
    if (await spread() > before) delta = -delta;
  }
}
// zoom at world point `p` until `lo`..`hi` spans `px` CSS pixels on screen: the title plate keeps its size, geometry grows past it
async function zoomTo(page, p, lo, hi, px) {
  let delta = -240;
  const spread = async () => { const s = await state(page); const [a, b] = [project(s, lo), project(s, hi)]; return Math.hypot(a[0] - b[0], a[1] - b[1]); };
  for (let i = 0; i < 30 && (await spread()) < px; i++) {
    const at = project(await state(page), p);
    await page.mouse.move(at[0], at[1]);
    const before = await spread();
    await page.mouse.wheel(0, delta); await settle(page, 80);
    if (await spread() < before) delta = -delta;
  }
}
// Undo drops the selection: click the object again to read its placement
async function undo(page, at, redo = false) { await key(page, redo ? 'Control+y' : 'Control+z'); if (at) await click(page, at); }
const last = async page => (await ui(page)).history.at(-1) || '';
const hint = async page => (await ui(page)).controls.find(c => c.key === 'command/hint')?.label || '';

async function open(browser, options) {
  const context = await browser.newContext(options);
  const page = await context.newPage();
  const errors = [];
  page.on('pageerror', e => errors.push(e.message));
  page.on('console', m => { if (m.type() === 'error' && !/favicon/.test(m.text())) errors.push(m.text()); });
  await page.route('**/view_local.yaml', r => r.fulfill({contentType: 'application/yaml', body: 'name: Transforms\nitems:\n  - file: interaction.pb\n'}));
  await page.route('**/interaction.pb', r => r.fulfill({path: fixture}));
  await page.route('**/favicon.ico', r => r.fulfill({status: 204}));
  await page.goto(new URL('?data=off&inspect=1', base).href);
  await page.waitForFunction(() => { const s = document.querySelector('canvas')?.getAttribute('data-viewer-inspection'); return s && JSON.parse(s).objects >= 7; }, {}, {timeout: 90000});
  await settle(page, 600);
  return {context, page, errors};
}

(async () => {
  await fs.mkdir(out, {recursive: true});
  const browser = await chromium.launch({executablePath: '/usr/bin/google-chrome', headless: process.env.HEADLESS === '1', args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--ozone-platform=x11']});
  const {page, errors} = await open(browser, {viewport: {width: 1280, height: 860}});
  try {
    const objects = (await state(page)).objects;
    await control(page, 'command/collapse'); // the history shows the prompt row
    await page.mouse.click(640, 300); await settle(page);
    await key(page, 'Escape'); await key(page, '5'); await key(page, 'f');

    // Move with clicks, a live preview and a typed relative point
    await enter(page, 'Line 20,0,0 30,0,0');
    await key(page, 'f');
    await frame(page, [-10, -6], [32, 16]);
    let s = await state(page);
    check(s.selected_rows.length === 1 && s.objects === objects + 1, 'a new line is selected');
    const line = world(s);
    await enter(page, 'Move');
    s = await state(page);
    check(s.drawing?.command === 'Move' && s.drawing.tool, `Move asks for points (${s.drawing?.command})`);
    check((await hint(page)).startsWith('Move: Point to move from'), `hint: ${await hint(page)}`);
    check(s.widget === null, 'the gumball hides while Move runs');
    const start = project(s, [20, 0, 0]);
    await page.mouse.move(start[0] + 4, start[1] + 3, {steps: 3}); await settle(page, 120);
    check((await state(page)).drawing?.snap === 'End', `End snap at the line start (${(await state(page)).drawing?.snap})`);
    await page.mouse.click(start[0] + 4, start[1] + 3); await settle(page);
    check(near((await state(page)).drawing.points[0], [20, 0, 0], 1e-9), 'the base point snapped to (20,0,0)');
    await hover(page, [23, 2, 0]);
    s = await state(page);
    const shifted = world(s);
    check(Math.abs(shifted[12] - line[12] - 3) < 0.2 && Math.abs(shifted[13] - line[13] - 2) < 0.2, `the line follows the cursor (${(shifted[12] - line[12]).toFixed(2)}, ${(shifted[13] - line[13]).toFixed(2)})`);
    await page.screenshot({path: path.join(out, 'move-preview.png')});
    await enter(page, '@0,10,0');
    s = await state(page);
    const moved = world(s);
    check(s.drawing === null && near([moved[12] - line[12], moved[13] - line[13], moved[14] - line[14]], [0, 10, 0], 1e-6), 'a typed @0,10,0 moves by exactly (0,10,0)');
    check(s.selected_rows.length === 1 && s.widget !== null, 'the selection stays and the gumball is back');
    await undo(page, [25, 0, 0]);
    check(near(world(await state(page)), line, 1e-6), 'Undo restores the line');
    await undo(page, [25, 10, 0], true);
    check(near(world(await state(page)), moved, 1e-6), 'Redo moves it again');

    // Move by a typed distance along the rubber band
    await enter(page, 'Move'); await enter(page, '20,10,0');
    await hover(page, [24, 10, 0]);
    const before = world(await state(page), 0);
    await enter(page, '5');
    s = await state(page);
    const after = world(s);
    const step = [after[12] - moved[12], after[13] - moved[13], after[14] - moved[14]];
    check(Math.abs(Math.hypot(...step) - 5) < 1e-9 && Math.abs(step[1]) < 1e-6, `a typed 5 moves 5 along the band (${step.map(v => v.toFixed(6))})`);
    check(before !== null, 'the preview ran before the typed distance');

    // Esc in the middle: everything goes back, the selection stays, no phantom undo step
    const kept = world(await state(page));
    await enter(page, 'Move'); await click(page, [22, 10, 0]); await hover(page, [26, 13, 0]);
    check(!near(world(await state(page)), kept, 1e-3), 'the preview moved the line');
    await key(page, 'Escape');
    s = await state(page);
    check(s.drawing === null || s.drawing.command !== 'Move', 'Esc ends Move');
    if (s.drawing === null) {
      check(near(world(s), kept, 1e-9) && s.selected_rows.length === 1 && s.objects === objects + 1, 'Esc puts the line back and keeps it selected');
    }
    await undo(page, [25, 10, 0]);
    check(near(world(await state(page)), moved, 1e-6), 'the next Undo undoes the typed-distance move');

    // nothing selected: pick objects first, Enter runs the command
    await key(page, 'Escape'); await key(page, 'Escape');
    check((await state(page)).selected === null, 'two Esc clear the selection');
    await enter(page, 'Move');
    s = await state(page);
    check(s.drawing?.command === 'select' && (await hint(page)).startsWith('Select objects for Move'), `Move with nothing selected asks for objects (${await hint(page)})`);
    await click(page, [-3, 3, 0]);
    s = await state(page);
    check(s.selected_rows.length === 1 && s.drawing?.command === 'select', 'a click adds the fixture line and keeps asking');
    await enter(page, '');
    check((await state(page)).drawing?.command === 'Move', 'Enter starts Move on it');
    await key(page, 'Escape');

    // Copy: one base point, several targets, Enter finishes
    const lineRow = (await state(page)).selected;
    await enter(page, 'Copy'); await enter(page, '-3,3,0'); await enter(page, '@0,-1,0');
    check((await state(page)).objects === objects + 2, 'the first target makes one copy');
    await enter(page, '@0,-2,0');
    s = await state(page);
    check(s.objects === objects + 3 && s.drawing?.command === 'Copy', 'the second target makes another and Copy keeps asking');
    check(s.selected === lineRow, 'the original stays selected');
    await page.screenshot({path: path.join(out, 'copy.png')});
    await enter(page, '');
    check((await state(page)).drawing === null, 'Enter finishes Copy');
    await enter(page, 'Layers On');
    const doc = (await ui(page)).controls.find(c => c.key.startsWith('open/') && c.label === 'interaction regression');
    if (doc) { await page.mouse.click((doc.rect[0] + doc.rect[2]) / 2, (doc.rect[1] + doc.rect[3]) / 2); await settle(page); }
    const rows = (await ui(page)).controls.filter(c => c.label === 'interaction line');
    check(rows.length === 3, `three layer rows are named interaction line (${rows.length}: ${(await ui(page)).controls.map(c => c.label).filter(l => /interaction|Created|Transforms/.test(l)).join(', ')})`);
    await enter(page, 'Layers Off');
    await key(page, 'Control+z');
    check((await state(page)).objects === objects + 2, 'one Undo removes one copy');
    await key(page, 'Control+z');
    check((await state(page)).objects === objects + 1, 'a second Undo removes the other');

    // Rotate in the Top plane: typed angle, then reference points
    await enter(page, 'Line 0,0,0 10,0,0');
    const flat = world(await state(page));
    await enter(page, 'Rotate');
    check((await state(page)).drawing?.command === 'Rotate', 'bare Rotate + Enter starts rotating');
    await enter(page, '0,0,0'); await enter(page, '90');
    let m = world(await state(page));
    check(Math.abs(m[0]) < 1e-9 && Math.abs(m[1] - 1) < 1e-9, 'Rotate 90 in Top turns +x to +y');
    await undo(page, [5, 0, 0]);
    check(near(world(await state(page)), flat, 1e-9), 'Undo turns it back');
    await enter(page, 'Rotate'); await enter(page, '0,0,0'); await enter(page, '10,0,0');
    await hover(page, [0, 10, 0]);
    m = world(await state(page));
    check(Math.abs(m[1] - 1) < 0.05, `the line turns live toward the cursor (${m[1].toFixed(3)})`);
    await page.screenshot({path: path.join(out, 'rotate-preview.png')});
    await enter(page, '0,10,0');
    m = world(await state(page));
    check(Math.abs(m[0]) < 1e-9 && Math.abs(m[1] - 1) < 1e-9, 'a second reference point gives the same turn');
    await undo(page, [5, 0, 0]);
    await key(page, '1');
    await enter(page, 'Rotate'); await enter(page, '0,0,0'); await enter(page, '90');
    m = world(await state(page));
    check(Math.abs(m[2] - 1) < 1e-9, 'Rotate 90 in Front turns +x to +z');
    await key(page, 'Control+z'); await key(page, '5'); await key(page, 'f'); await frame(page, [-10, -6], [32, 16]); await click(page, [5, 0, 0]);

    // Scale: typed factor, reference points, 1D and 2D
    await enter(page, 'Scale'); await enter(page, '0,0,0'); await enter(page, '2');
    m = world(await state(page));
    check([m[0], m[5], m[10]].every(v => Math.abs(v - 2) < 1e-9), 'Scale 2 about the origin');
    await undo(page, [5, 0, 0]);
    await enter(page, 'Scale'); await enter(page, '0,0,0'); await enter(page, '10,0,0'); await enter(page, '5,0,0');
    m = world(await state(page));
    check(Math.abs(m[0] - 0.5) < 1e-9, 'two reference points give ×0.5');
    await undo(page, [5, 0, 0]);
    await enter(page, 'Scale');
    check(await control(page, 'command/option/1D'), 'the 1D option button is offered');
    check((await hint(page)).includes('1D'), 'the prompt shows 1D');
    await enter(page, '0,0,0'); await enter(page, '10,0,0'); await enter(page, '30,0,0');
    m = world(await state(page));
    check(Math.abs(m[0] - 3) < 1e-9 && Math.abs(m[5] - 1) < 1e-9 && Math.abs(m[10] - 1) < 1e-9, '1D scales along x only');
    await undo(page, [5, 0, 0]);
    await enter(page, 'Scale'); await enter(page, '2D'); await enter(page, '0,0,0'); await enter(page, '3');
    m = world(await state(page));
    check(Math.abs(m[0] - 3) < 1e-9 && Math.abs(m[5] - 3) < 1e-9 && Math.abs(m[10] - 1) < 1e-9, '2D keeps the height');
    await key(page, 'Control+z');

    // Orient 3 Points, with a refused collinear third point
    await enter(page, 'Polyline 0,0,0 10,0,0 10,5,0');
    const polyline = await state(page);
    check(polyline.objects === objects + 3, 'the polyline is drawn');
    await enter(page, 'orient3pt'); await enter(page, '0,0,0'); await enter(page, '10,0,0'); await enter(page, '20,0,0');
    s = await state(page);
    check(s.drawing?.points?.length === 2 && /line/.test(await last(page)), `a collinear third point is refused (${JSON.stringify((await ui(page)).history.slice(-6))})`);
    await enter(page, '10,5,0'); await enter(page, '50,50,0'); await enter(page, '50,60,0'); await enter(page, '45,60,0');
    m = world(await state(page));
    check(near(apply(m, [10, 0, 0]), [50, 60, 0], 1e-6) && near(apply(m, [10, 5, 0]), [45, 60, 0], 1e-6), 'Orient 3 Points lands the references on the targets');
    await key(page, 'Control+z');

    // Explode: polyline, BRep, cloud, a line refuses, nothing selected asks
    await key(page, 'Escape'); await key(page, 'Escape');
    const count = (await state(page)).objects;
    await click(page, [2.5, 3.5, 0]);
    await enter(page, 'Explode');
    s = await state(page);
    check(s.objects === count + 1 && s.selected_rows.length === 2, `a polyline explodes into two selected lines (${s.objects - count}, ${s.selected_rows.length})`);
    await key(page, 'Control+z');
    check((await state(page)).objects === count, 'Undo brings the polyline back');
    await key(page, 'Escape'); await key(page, 'Escape');
    await click(page, [2.5, 3.5, 0]);
    check((await state(page)).source_faces === null && (await state(page)).selected !== null, 'the restored polyline is selectable');
    await key(page, 'Escape'); await key(page, 'Escape');
    await zoomTo(page, [-2.3, -2.3, 0.4], [-4, -4, 0], [-2, -2, 0], 300);
    await click(page, [-2.3, -2.3, 0.4]);
    check((await state(page)).source_faces === 6, 'the box BRep is selected');
    await enter(page, 'Explode');
    s = await state(page);
    check(s.objects === count + 5 && s.vertices > 0, `the BRep explodes into six faces (${s.objects - count})`);
    await page.screenshot({path: path.join(out, 'explode-brep.png')});
    await key(page, 'Escape'); await key(page, 'Escape');
    await click(page, [-2.3, -2.3, 0.4]);
    check((await state(page)).source_faces === 1, 'a clicked piece is one face');
    await key(page, 'Control+z');
    await key(page, 'Escape'); await key(page, 'Escape');
    await click(page, [-2.3, -2.3, 0.4]);
    check((await state(page)).source_faces === 6 && (await state(page)).objects === count, 'Undo gives the one BRep back');
    await key(page, 'Escape'); await key(page, 'Escape'); await key(page, 'f'); await frame(page, [-10, -6], [32, 16]);
    await key(page, 'Escape'); await key(page, 'Escape');
    await click(page, [3, -3, 0]);
    await enter(page, 'Explode');
    s = await state(page);
    check(s.objects === count + 24, `the cloud explodes into 25 points (${s.objects - count})`);
    await key(page, 'Control+z');
    await key(page, 'Escape'); await key(page, 'Escape');
    await click(page, [-3, 3, 0]);
    await enter(page, 'Explode');
    check((await state(page)).objects === count && /Line cannot be exploded/.test(await last(page)), `a line refuses (${(await last(page)).split('\n').at(-1)})`);
    await key(page, 'Escape'); await key(page, 'Escape');
    await enter(page, 'Explode');
    check((await hint(page)).startsWith('Select objects for Explode'), 'Explode with nothing selected asks for objects');
    await click(page, [2.5, 3.5, 0]); await enter(page, '');
    check((await state(page)).objects === count + 1, 'Enter explodes the picked polyline');
    await key(page, 'Control+z');

    // typed shortcuts act at once
    await key(page, 'Escape'); await key(page, 'Escape');
    await click(page, [-3, 3, 0]);
    const typed = world(await state(page));
    await enter(page, 'Move 10,0,0');
    check(Math.abs(world(await state(page))[12] - typed[12] - 10) < 1e-9, 'Move 10,0,0 moves at once');
    await undo(page, [-3, 3, 0]);
    await enter(page, 'Rotate z 45');
    check(Math.abs(world(await state(page))[1] - Math.SQRT1_2) < 1e-6, 'Rotate z 45 turns at once');
    await undo(page, [-3, 3, 0]);
    await enter(page, 'Scale 2');
    check(Math.abs(world(await state(page))[0] - 2) < 1e-9, 'Scale 2 scales at once');
    await undo(page, [-3, 3, 0]);
    check(near(world(await state(page)), typed, 1e-9), 'each shortcut is one Undo');
  } catch (e) {
    check(false, `threw: ${e.stack}`);
    await page.screenshot({path: path.join(out, 'failure.png')});
  }
  check(errors.length === 0, `no page errors: ${errors.slice(0, 3).join(' | ')}`);

  // a phone: taps place the points; snapping off so the second tap lands where it is
  const phone = await open(browser, {...devices['Pixel 7']});
  try {
    const p = phone.page;
    await p.touchscreen.tap(...project(await state(p), [-3, 3, 0])); await settle(p);
    check((await state(p)).selected !== null, 'a tap selects the line on a phone');
    const start = world(await state(p));
    await enter(p, 'Snap Off');
    await enter(p, 'Move');
    const base = project(await state(p), [-3, 3, 0]);
    await p.touchscreen.tap(...base); await settle(p);
    check((await state(p)).drawing?.points?.length === 1, 'a tap places the base point');
    await p.waitForTimeout(500); // two quick taps close together are a double tap: fit
    await p.touchscreen.tap(base[0] + 120, base[1] - 60); await settle(p);
    const s = await state(p);
    const shift = Math.hypot(world(s)[12] - start[12], world(s)[13] - start[13], world(s)[14] - start[14]);
    check(s.drawing === null && shift > 0.5, `a second tap moves the line on a phone (${shift.toFixed(3)})`);
    await p.screenshot({path: path.join(out, 'phone-move.png')});
    // two quick taps close together place two points, not a double-tap fit; Cancel is the phone's Esc
    const copies = (await state(p)).objects;
    await enter(p, 'Copy');
    const from = project(await state(p), [0, 0, 0]);
    await p.touchscreen.tap(from[0], from[1]); await p.waitForTimeout(60);
    await p.touchscreen.tap(from[0] + 15, from[1]); await settle(p);
    const copied = await state(p);
    check(copied.objects === copies + 1 && copied.drawing?.command === 'Copy', `two quick taps make one copy (${copied.objects - copies}, ${copied.drawing?.command})`);
    check(await control(p, 'command/option/Cancel'), 'a Cancel button is offered on a phone');
    check((await state(p)).drawing === null && (await state(p)).selected !== null, 'Cancel ends Copy and keeps the selection');
  } catch (e) {
    check(false, `phone threw: ${e.stack}`);
  }
  check(phone.errors.length === 0, `no phone errors: ${phone.errors.slice(0, 3).join(' | ')}`);
  await browser.close();
  const failed = results.filter(r => r.startsWith('FAIL')).length;
  console.log(`${results.length - failed}/${results.length} passed`);
  process.exit(failed ? 1 : 0);
})();
