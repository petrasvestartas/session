/** Rhino-like Trim and Extend through the real UI: curves on an empty scene, BRep, mesh and surface on the interaction fixture, a phone tap flow. */
const path = require('node:path');
const {chromium, devices} = require('playwright');
const base = process.env.VIEWER_URL || 'http://127.0.0.1:8770/';
const fixture = process.env.FIXTURE || '/tmp/interaction.pb';
const out = process.env.OUT || '/tmp/viewer-trim-extend';
const only = process.env.ONLY ? process.env.ONLY.split(',') : null;
const results = [];
let section = '';
const check = (ok, text) => { results.push(`${ok ? 'PASS' : 'FAIL'} [${section}] ${text}`); console.log(results.at(-1)); return ok; };
const state = page => page.evaluate(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')));
const ui = page => page.evaluate(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')));
const near = (a, b, eps = 1e-6) => a.length === b.length && a.every((x, i) => Math.abs(x - b[i]) <= eps);
async function settle(page, ms = 250) { await page.waitForTimeout(ms); await page.waitForFunction(() => !JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).pick_busy); }
function project(s, p) { const v = p.map((n, i) => n - s.origin[i]); const c = [0, 1, 2, 3].map(r => s.mvp[r] * v[0] + s.mvp[r + 4] * v[1] + s.mvp[r + 8] * v[2] + s.mvp[r + 12]); return [(c[0] / c[3] * .5 + .5) * s.logical_canvas[0], (.5 - c[1] / c[3] * .5) * s.logical_canvas[1]]; }
async function control(page, key, button = 'left') { const c = (await ui(page)).controls.filter(c => c.key === key).at(-1); if (!c) return false; const [a, b, x, y] = c.rect; await page.mouse.click((a + x) / 2, (b + y) / 2, {button}); await settle(page); return true; }
async function enter(page, text) { await control(page, 'command/input'); await page.keyboard.press('Control+a'); if (text) await page.keyboard.type(text, {delay: 25}); await page.keyboard.press('Enter'); await settle(page); }
async function hover(page, p) { const at = project(await state(page), p); await page.mouse.move(at[0], at[1], {steps: 3}); await settle(page, 150); return at; }
async function click(page, p) { const at = await hover(page, p); await page.mouse.click(at[0], at[1]); await settle(page); }
async function blur(page) { const c = (await ui(page)).controls.find(c => c.key === 'command/input'); await page.mouse.click(c.rect[0] - 30, (c.rect[1] + c.rect[3]) / 2); await settle(page, 120); }
async function key(page, name) { await blur(page); await page.keyboard.press(name); await settle(page); }
const said = async page => { const u = await ui(page); const status = await page.evaluate(() => document.getElementById('viewer-status')?.textContent || ''); return `${status} | ${u.history.slice(-1).join(' / ')}`; };
const bez = (a, b, c, t) => a.map((v, i) => (1 - t) * (1 - t) * v + 2 * t * (1 - t) * b[i] + t * t * c[i]);
async function view(page) { await clear(page); await key(page, '5'); await enter(page, 'Fit'); await clear(page); }
const shot = (page, name) => page.screenshot({path: path.join(out, `${name}.png`)});
const tool = async page => (await state(page)).tool;
async function clear(page) { if (await tool(page)) await key(page, 'Escape'); await key(page, 'Escape'); await key(page, 'Escape'); }
async function controls(page, p) { await clear(page); await click(page, p); await key(page, 'F10'); const s = await state(page); await key(page, 'Escape'); await key(page, 'Escape'); return s.controls.map(c => c.position); }

async function open(browser, options, yaml) {
  const context = await browser.newContext(options);
  const page = await context.newPage();
  const errors = [];
  page.on('pageerror', e => errors.push(e.message));
  page.on('console', m => { if (m.type() === 'error' && !/favicon/.test(m.text())) errors.push(m.text()); });
  await page.route('**/view_local.yaml', r => r.fulfill({contentType: 'application/yaml', body: yaml}));
  await page.route('**/interaction.pb', r => r.fulfill({path: fixture}));
  await page.route('**/favicon.ico', r => r.fulfill({status: 204}));
  await page.goto(new URL('?data=off&inspect=1', base).href);
  await page.waitForFunction(() => document.querySelector('canvas')?.getAttribute('data-viewer-ui'), {}, {timeout: 90000});
  await settle(page, 1500);
  return {context, page, errors};
}

const sections = {};
const run = (name, fn) => { sections[name] = fn; };
const EMPTY = 'name: Empty\nitems: []\n';
const FIXTURE = 'name: Trim\nitems:\n  - file: interaction.pb\n';

let rows = {};
run('setup', async page => {
  const gpu = await page.evaluate(async () => { const a = await navigator.gpu?.requestAdapter(); return a ? (a.info?.vendor || 'adapter') + ' ' + (a.info?.architecture || '') : null; });
  check(!!gpu, `real WebGPU adapter (${gpu})`);
  await control(page, 'command/collapse');
  await enter(page, 'Line 0,0,0 100,0,0');
  rows.h = (await state(page)).selected_rows[0];
  await enter(page, 'Line 50,-50,0 50,50,0');
  rows.v = (await state(page)).selected_rows[0];
  await key(page, '5');
  await enter(page, 'Fit');
  await clear(page);
  check((await state(page)).objects === 2, `two lines (${(await state(page)).objects})`);
});

run('trim-line', async page => {
  await clear(page);
  await enter(page, 'Trim');
  let t = await tool(page);
  check(t?.phase === 'targets', `Trim starts in targets (${JSON.stringify(t)})`);
  const u = await ui(page);
  check(u.controls.some(c => c.key === 'command/option/Next') && u.controls.some(c => c.key === 'command/option/Cancel'), 'Next and Cancel buttons');
  await shot(page, 'trim-targets-prompt');
  await click(page, [25, 0, 0]);
  t = await tool(page);
  check(near(t?.targets || [], [rows.h]), `target is the horizontal line (${JSON.stringify(t?.targets)})`);
  await enter(page, '');
  check((await tool(page))?.phase === 'cutters', `Enter: cutters (${(await tool(page))?.phase})`);
  await click(page, [50, 30, 0]);
  check(near((await tool(page))?.cutters || [], [rows.v]), `cutter is the vertical line (${JSON.stringify((await tool(page))?.cutters)})`);
  await enter(page, '');
  t = await tool(page);
  check(t?.phase === 'parts' && t.parts[0]?.count === 2, `Enter: parts, 2 of them (${JSON.stringify(t)})`);
  await shot(page, 'trim-parts');
  await hover(page, [75, 0, 0]);
  t = await tool(page);
  check(near(t?.hover || [], [0, 1]), `hover over the right part (${JSON.stringify(t?.hover)})`);
  await shot(page, 'trim-hover');
  await click(page, [75, 0, 0]);
  const s = await state(page);
  check(s.tool === null && s.objects === 2 && /Trimmed/.test(await said(page)), `a click trims (${JSON.stringify(s.tool)}, ${s.objects} objects, ${await said(page)})`);
  await shot(page, 'trim-done');
  let points = await controls(page, [25, 0, 0]);
  check(near(points.flat(), [0, 0, 0, 50, 0, 0], 1e-6), `line is 0..50 (${JSON.stringify(points)})`);
  await enter(page, 'Undo');
  points = await controls(page, [25, 0, 0]);
  check(near(points.flat(), [0, 0, 0, 100, 0, 0], 1e-6), `Undo restores 0..100 (${JSON.stringify(points)})`);
  await enter(page, 'Redo');
  points = await controls(page, [25, 0, 0]);
  check(near(points.flat(), [0, 0, 0, 50, 0, 0], 1e-6), `Redo trims again (${JSON.stringify(points)})`);
  await enter(page, 'Undo');
});

run('trim-esc', async page => {
  await clear(page);
  const before = (await state(page)).objects;
  const undo = (await state(page)).undo_depth;
  await enter(page, 'Trim'); await click(page, [25, 0, 0]); await enter(page, ''); await click(page, [50, 30, 0]); await enter(page, '');
  check((await tool(page))?.phase === 'parts', 'reached parts');
  await key(page, 'Escape');
  let s = await state(page);
  check(s.tool === null && s.objects === before && s.undo_depth === undo, `Esc: no tool, objects ${s.objects}, undo depth ${s.undo_depth} == ${undo}`);
  await shot(page, 'trim-esc');
  const points = await controls(page, [75, 0, 0]);
  check(near(points.flat(), [0, 0, 0, 100, 0, 0], 1e-6), `full line visible and pickable at 75 (${JSON.stringify(points)})`);
  for (const [name, finish] of [['targets', async () => key(page, 'Escape')], ['cutters', async () => enter(page, 'Escape')], ['cutters-button', async () => control(page, 'command/option/Cancel')]]) {
    await clear(page);
    await enter(page, 'Trim'); await click(page, [25, 0, 0]);
    if (name !== 'targets') { await enter(page, ''); await click(page, [50, 30, 0]); }
    await finish();
    s = await state(page);
    check(s.tool === null && s.selected_rows.length === 0, `Esc in ${name}: no tool, nothing highlighted (${JSON.stringify(s.tool)}, ${s.selected_rows})`);
  }
});

run('trim-three', async page => {
  await clear(page);
  await enter(page, 'Line 0,20,0 100,20,0'); const line = (await state(page)).selected_rows[0];
  await enter(page, 'Line 30,10,0 30,40,0'); await enter(page, 'Line 70,10,0 70,40,0');
  await view(page);
  const objects = (await state(page)).objects;
  await enter(page, 'Trim'); await click(page, [50, 20, 0]); await enter(page, ''); await click(page, [30, 35, 0]); await click(page, [70, 35, 0]); await enter(page, '');
  let t = await tool(page);
  check(t?.parts?.[0]?.count === 3, `three parts (${JSON.stringify(t?.parts)})`);
  await click(page, [50, 20, 0]);
  t = await tool(page);
  check(t?.phase === 'parts' && near(t.parts[0].removed, [1]), `middle removed, still in parts (${JSON.stringify(t)})`);
  await shot(page, 'trim-three-middle');
  await enter(page, '');
  check((await state(page)).objects === objects + 1, `Enter adds one object (${(await state(page)).objects})`);
  const left = await controls(page, [15, 20, 0]);
  const right = await controls(page, [85, 20, 0]);
  check(near(left.flat(), [0, 20, 0, 30, 20, 0], 1e-6) && near(right.flat(), [70, 20, 0, 100, 20, 0], 1e-6), `0..30 and 70..100 (${JSON.stringify(left)} ${JSON.stringify(right)})`);
  await enter(page, 'Undo');
  check((await state(page)).objects === objects, `one Undo gives one line back (${(await state(page)).objects})`);
  const whole = await controls(page, [50, 20, 0]);
  check(near(whole.flat(), [0, 20, 0, 100, 20, 0], 1e-6), `0..100 again (${JSON.stringify(whole)})`);
});

run('trim-polyline-curve', async page => {
  await clear(page);
  await enter(page, 'Polyline 0,60,0 40,60,0 40,100,0'); await enter(page, 'Line 20,50,0 20,70,0');
  await view(page);
  await enter(page, 'Trim'); await click(page, [40, 80, 0]); await enter(page, ''); await click(page, [20, 52, 0]); await enter(page, '');
  await click(page, [10, 60, 0]);
  const poly = await controls(page, [40, 80, 0]);
  check(near(poly.flat(), [20, 60, 0, 40, 60, 0, 40, 100, 0], 1e-6), `polyline keeps (20,60),(40,60),(40,100) (${JSON.stringify(poly)})`);
  await enter(page, 'Curve 0,-60,0 50,0,0 100,-60,0'); await enter(page, 'Line 50,-80,0 50,10,0');
  await view(page);
  const arc = t => bez([0, -60, 0], [50, 0, 0], [100, -60, 0], t);
  await enter(page, 'Trim'); await click(page, arc(0.2));
  let t = await tool(page);
  await enter(page, '');
  await click(page, [50, 5, 0]);
  await enter(page, '');
  t = await tool(page);
  check(t?.phase === 'parts', `curve reached parts (${JSON.stringify(t)})`);
  await click(page, arc(0.8));
  const curve = await controls(page, arc(0.2));
  check(curve.length > 0 && Math.abs(curve.at(-1)[0] - 50) < 1e-3, `curve ends at x = 50 (${JSON.stringify(curve)})`);
  await shot(page, 'trim-polyline-curve');
});

run('trim-preselect-refuse', async page => {
  await clear(page);
  await click(page, [50, 20, 0]);
  await enter(page, 'Trim');
  let t = await tool(page);
  check(t?.phase === 'cutters' && t.targets.length === 1, `preselected line starts in cutters (${JSON.stringify(t)})`);
  await click(page, [20, 60, 0]);
  await enter(page, '');
  t = await tool(page);
  check(t?.phase === 'cutters' && /No crossings/.test(await said(page)), `a cutter that misses stays in cutters (${t?.phase}; ${await said(page)})`);
  await key(page, 'Escape');
});

run('extend-boundary', async page => {
  await clear(page);
  await enter(page, 'Line 0,-100,0 50,-100,0'); await enter(page, 'Line 100,-150,0 100,-50,0');
  await view(page);
  const undo = (await state(page)).undo_depth;
  await enter(page, 'Extend');
  check((await tool(page))?.phase === 'boundaries', `Extend asks for boundaries (${JSON.stringify(await tool(page))})`);
  await click(page, [100, -60, 0]);
  await enter(page, '');
  check((await tool(page))?.phase === 'curves', `Enter: curves (${(await tool(page))?.phase})`);
  await hover(page, [45, -100, 0]);
  const t = await tool(page);
  check(t?.hover && near(t.hover.to, [100, -100, 0], 1e-6), `hover shows the end reaching (100,-100) (${JSON.stringify(t?.hover)})`);
  await shot(page, 'extend-hover');
  await click(page, [45, -100, 0]);
  check((await tool(page))?.phase === 'curves', `tool stays after an extension (${await said(page)})`);
  await key(page, 'Escape');
  const points = await controls(page, [30, -100, 0]);
  check(near(points.flat(), [0, -100, 0, 100, -100, 0], 1e-6), `line reaches x = 100 (${JSON.stringify(points)})`);
  check((await state(page)).undo_depth === undo + 1, `one undo step (${(await state(page)).undo_depth} vs ${undo})`);
  await enter(page, 'Undo');
  const back = await controls(page, [30, -100, 0]);
  check(near(back.flat(), [0, -100, 0, 50, -100, 0], 1e-6), `Undo gives 0..50 back (${JSON.stringify(back)})`);
});

run('extend-distance', async page => {
  await clear(page);
  await enter(page, 'Extend Distance 25');
  let t = await tool(page);
  check(t?.phase === 'curves' && t.distance === 25, `Extend Distance 25 (${JSON.stringify(t)})`);
  await click(page, [2, -100, 0]);
  await key(page, 'Escape');
  const points = await controls(page, [30, -100, 0]);
  check(near(points.flat(), [-25, -100, 0, 50, -100, 0], 1e-6), `start moved to -25 (${JSON.stringify(points)})`);
  await enter(page, 'Undo');
  await clear(page);
  await enter(page, 'Extend');
  await control(page, 'command/option/Distance');
  await enter(page, '10');
  t = await tool(page);
  check(t?.distance === 10 && t.phase === 'curves', `Distance button then 10 (${JSON.stringify(t)})`);
  await key(page, 'Escape');
  await enter(page, 'Polyline Rectangle 200,0,0 240,40,0');
  await view(page);
  await enter(page, 'Extend 5');
  await click(page, [220, 0, 0]);
  check(/no free end/.test(await said(page)) || /Click near/.test(await said(page)), `closed rectangle refused (${await said(page)})`);
  await key(page, 'Escape');
});

run('extend-curve', async page => {
  await clear(page);
  await enter(page, 'Curve 0,-200,0 50,-150,0 100,-200,0'); await enter(page, 'Line 130,-300,0 130,-100,0');
  await view(page);
  await enter(page, 'Extend'); await click(page, [130, -120, 0]); await enter(page, '');
  await hover(page, bez([0, -200, 0], [50, -150, 0], [100, -200, 0], 0.96));
  const t = await tool(page);
  await shot(page, 'extend-curve-hover');
  await click(page, bez([0, -200, 0], [50, -150, 0], [100, -200, 0], 0.96));
  await key(page, 'Escape');
  const points = await controls(page, bez([0, -200, 0], [50, -150, 0], [100, -200, 0], 0.5));
  check(points.length > 0 && Math.abs(points.at(-1)[0] - 130) < 1e-3, `curve ends at x = 130 (${JSON.stringify(points.at(-1))}; hover ${JSON.stringify(t?.hover)})`);
});

// the interaction fixture: brep box at (-3,-3), mesh box at (-9,3), flat surface -10..-8 x -4..-2
run('fixture', async page => {
  await control(page, 'command/collapse');
  await key(page, '5');
  await enter(page, 'Fit');
  await clear(page);
  const box = await controls(page, [-3, -3, 0.2]);
  const top = Math.max(...box.map(p => p[2]));
  const bottom = Math.min(...box.map(p => p[2]));
  check(box.length >= 8, `brep box vertices z ${bottom}..${top}`);
  rows.top = top; rows.bottom = bottom;
});

run('brep-fence', async page => {
  await clear(page);
  await enter(page, 'Line -3,-5,0 -3,-4.5,0');
  await clear(page);
  const objects = (await state(page)).objects;
  await enter(page, 'Trim'); await click(page, [-3.5, -3, rows.top]); await enter(page, ''); await click(page, [-3, -4.8, 0]); await enter(page, '');
  let t = await tool(page);
  check(t?.phase === 'parts' && t.parts[0]?.count === 2, `fence cut: parts 2 (${JSON.stringify(t)}; ${await said(page)})`);
  await shot(page, 'brep-fence-parts');
  await click(page, [-3.5, -3, rows.top]);
  let s = await state(page);
  check(s.tool === null && s.objects === objects, `trimmed (${JSON.stringify(s.tool)}; ${await said(page)})`);
  await clear(page); await click(page, [-2.5, -3, rows.top]);
  s = await state(page);
  check(s.source_faces === 5, `5 faces left (${s.source_faces})`);
  await key(page, '7'); await shot(page, 'brep-fence-iso'); await key(page, '5');
  await enter(page, 'Undo');
  await clear(page); await click(page, [-3.5, -3, rows.top]);
  check((await state(page)).source_faces === 6, `Undo: 6 faces (${(await state(page)).source_faces})`);
});

run('brep-on-face', async page => {
  await clear(page);
  await enter(page, `Line -3,-4.5,${rows.top} -3,-1.5,${rows.top}`);
  await clear(page);
  await enter(page, 'Trim'); await click(page, [-3.5, -3.5, rows.top]); await enter(page, ''); await click(page, [-3, -1.7, rows.top]); await enter(page, '');
  const t = await tool(page);
  check(t?.phase === 'parts' && t.parts[0]?.count === 2, `on-face cut: 2 regions (${JSON.stringify(t)}; ${await said(page)})`);
  await click(page, [-3.5, -3.5, rows.top]);
  await clear(page); await click(page, [-2.5, -3, rows.top]);
  check((await state(page)).source_faces === 6, `6 faces, top-left half open (${(await state(page)).source_faces})`);
  await key(page, '7'); await shot(page, 'brep-on-face-iso'); await key(page, '5');
  await enter(page, 'Undo');
});

run('mesh-fence', async page => {
  await clear(page);
  await enter(page, 'Line -9,1.2,0 -9,1.6,0');
  await clear(page);
  const vertices = (await state(page)).vertices;
  const objects = (await state(page)).objects;
  await enter(page, 'Trim'); await click(page, [-9.5, 3, 0.2]); await enter(page, ''); await click(page, [-9, 1.4, 0]); await enter(page, '');
  let t = await tool(page);
  check(t?.phase === 'parts' && t.parts[0]?.count === 2, `mesh cut by a fence (${JSON.stringify(t)}; ${await said(page)})`);
  await shot(page, 'mesh-parts');
  await key(page, 'Escape');
  check((await state(page)).vertices === vertices && (await state(page)).tool === null, `Esc keeps the mesh (${(await state(page)).vertices} vs ${vertices})`);
  // two targets, one removal previewed, then Esc
  await enter(page, 'Line -3,-5,0 -3,-4.5,0');
  await clear(page);
  const undo = (await state(page)).undo_depth;
  await enter(page, 'Trim'); await click(page, [-9.5, 3, 0.2]); await click(page, [-3.5, -3, rows.top]); await enter(page, '');
  await click(page, [-9, 1.4, 0]); await click(page, [-3, -4.8, 0]); await enter(page, '');
  t = await tool(page);
  check(t?.parts?.length === 2, `two targets cut (${JSON.stringify(t?.parts)}; ${await said(page)})`);
  await click(page, [-9.5, 3, 0.2]);
  t = await tool(page);
  check(t?.phase === 'parts', `one removal previews, the tool continues (${JSON.stringify(t?.parts)})`);
  await shot(page, 'mesh-brep-preview');
  await key(page, 'Escape');
  let s = await state(page);
  check(s.tool === null && s.vertices === vertices + 0 * objects && s.undo_depth === undo, `Esc restores both (vertices ${s.vertices} vs ${vertices}, undo ${s.undo_depth} vs ${undo})`);
  await clear(page); await click(page, [-3.5, -3, rows.top]);
  check((await state(page)).source_faces === 6, `box still has 6 faces (${(await state(page)).source_faces})`);
  // remove a mesh side for real
  await clear(page);
  await enter(page, 'Trim'); await click(page, [-9.5, 3, 0.2]); await enter(page, ''); await click(page, [-9, 1.4, 0]); await enter(page, '');
  await click(page, [-9.5, 3, 0.2]);
  s = await state(page);
  const said1 = await said(page);
  const half = await controls(page, [-8.5, 3, 0.2]);
  const low = Math.min(...half.map(p => p[0]));
  check(s.tool === null && Math.abs(low) < 1e-6 && s.objects === objects + 1, `mesh trimmed: its local controls start at x = 0, world -9 (${low}; ${said1})`);
  await key(page, '7'); await shot(page, 'mesh-trimmed-iso'); await key(page, '5');
  await enter(page, 'Undo');
  const whole = await controls(page, [-8.5, 3, 0.2]);
  check(Math.abs(Math.min(...whole.map(p => p[0])) + 1) < 1e-6, `Undo restores the mesh, local x from -1 (${Math.min(...whole.map(p => p[0]))})`);
});

run('surface-line', async page => {
  await clear(page);
  await enter(page, 'Line -9,-4.5,0 -9,-1.5,0');
  await clear(page);
  const objects = (await state(page)).objects;
  await enter(page, 'Trim'); await click(page, [-9.5, -3, 0]); await enter(page, ''); await click(page, [-9, -1.7, 0]); await enter(page, '');
  const t = await tool(page);
  check(t?.phase === 'parts' && t.parts[0]?.count === 2, `surface split by an on-surface line (${JSON.stringify(t)}; ${await said(page)})`);
  await click(page, [-9.5, -3, 0]);
  const s = await state(page);
  check(s.tool === null && s.objects === objects, `surface swapped for a BRep, object count kept (${s.objects} vs ${objects}; ${await said(page)})`);
  await clear(page); await click(page, [-8.5, -3, 0]);
  check((await state(page)).source_faces === 1, `the kept region is one face (${(await state(page)).source_faces})`);
  await shot(page, 'surface-trimmed');
  await enter(page, 'Undo');
});

(async () => {
  require('node:fs').mkdirSync(out, {recursive: true});
  const browser = await chromium.launch({executablePath: '/usr/bin/google-chrome', headless: false, args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--ozone-platform=x11']});
  const empty = ['setup', 'trim-line', 'trim-esc', 'trim-three', 'trim-polyline-curve', 'trim-preselect-refuse', 'extend-boundary', 'extend-distance', 'extend-curve'];
  const later = ['fixture', 'brep-fence', 'brep-on-face', 'mesh-fence', 'surface-line'];
  for (const [yaml, names] of [[EMPTY, empty], [FIXTURE, later]]) {
    if (only && !names.some(n => only.includes(n))) continue;
    const {page, errors, context} = await open(browser, {viewport: {width: 1280, height: 860}}, yaml);
    for (const name of names) {
      if (only && !['setup', 'fixture'].includes(name) && !only.includes(name)) continue;
      section = name;
      try { await fn(page, name); } catch (e) { check(false, `threw: ${e.stack.split('\n').slice(0, 3).join(' ')}`); await shot(page, `fail-${name}`); await clear(page).catch(() => {}); }
    }
    section = 'errors';
    check(errors.length === 0, `no page/console errors (${errors.slice(0, 3).join(' | ')})`);
    await context.close();
  }

  async function fn(page, name) { await sections[name](page); }

  if (!only || only.includes('phone')) {
    section = 'phone';
    const phone = await open(browser, {...devices['Pixel 7']}, EMPTY);
    try {
      const p = phone.page;
      await enter(p, 'Line 0,0,0 100,0,0'); await enter(p, 'Line 50,-50,0 50,50,0'); await enter(p, 'Fit');
      await control(p, 'command/option/Cancel').catch(() => {});
      await enter(p, 'Escape'); await enter(p, 'Escape');
      await enter(p, 'Trim');
      const tap = async q => { const at = project(await state(p), q); await p.touchscreen.tap(at[0], at[1]); await settle(p); };
      await tap([25, 0, 0]);
      check((await tool(p))?.targets?.length === 1, `tap picks a target (${JSON.stringify(await tool(p))})`);
      await control(p, 'command/option/Next');
      await p.waitForTimeout(400);
      await tap([50, 30, 0]);
      check((await tool(p))?.cutters?.length === 1, `tap picks a cutter (${JSON.stringify(await tool(p))})`);
      await control(p, 'command/option/Next');
      check((await tool(p))?.phase === 'parts', `Next previews (${JSON.stringify(await tool(p))})`);
      await p.waitForTimeout(400);
      await tap([75, 0, 0]);
      check((await state(p)).tool === null, `a tap removes the part (${JSON.stringify(await tool(p))})`);
      await shot(p, 'phone-trim');
      await enter(p, 'Extend Distance');
      await p.keyboard.type('10', {delay: 80}); await p.keyboard.press('Enter'); await settle(p);
      check((await tool(p))?.distance === 10, `phone keyboard types the distance (${JSON.stringify(await tool(p))})`);
      await control(p, 'command/option/Cancel');
    } catch (e) { check(false, `phone threw: ${e.stack.split('\n').slice(0, 3).join(' ')}`); }
    check(phone.errors.length === 0, `no phone errors (${phone.errors.slice(0, 3).join(' | ')})`);
  }
  await browser.close();
  const failed = results.filter(r => r.startsWith('FAIL')).length;
  console.log(`${results.length - failed}/${results.length} passed`);
  process.exit(failed ? 1 : 0);
})();
