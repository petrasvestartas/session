/** B6: Select By Name, Select Small and Select Lasso through the real UI. */
const path = require('node:path');
const {chromium, devices} = require('playwright');
const root = path.resolve(__dirname, '..');
const base = process.env.VIEWER_URL || 'http://127.0.0.1:8770/';
const out = process.env.OUT || '/tmp/viewer-selection';
require('node:fs').mkdirSync(out, {recursive: true});
const only = process.env.ONLY ? process.env.ONLY.split(',') : null;
const results = [];
let section = '';
const check = (ok, text) => { results.push(`${ok ? 'PASS' : 'FAIL'} [${section}] ${text}`); console.log(results.at(-1)); return ok; };
const state = page => page.evaluate(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')));
const ui = page => page.evaluate(() => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')));
async function settle(page, ms = 250) { await page.waitForTimeout(ms); await page.evaluate(() => new Promise(r => requestAnimationFrame(() => requestAnimationFrame(r)))); await page.waitForFunction(() => !JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).pick_busy); }
function project(s, p) { const v = p.map((n, i) => n - s.origin[i]); const c = [0, 1, 2, 3].map(r => s.mvp[r] * v[0] + s.mvp[r + 4] * v[1] + s.mvp[r + 8] * v[2] + s.mvp[r + 12]); return [(c[0] / c[3] * .5 + .5) * s.logical_canvas[0], (.5 - c[1] / c[3] * .5) * s.logical_canvas[1]]; }
async function control(page, key) { const c = (await ui(page)).controls.filter(c => c.key === key).at(-1); if (!c) return false; const [a, b, x, y] = c.rect; await page.mouse.click((a + x) / 2, (b + y) / 2); await settle(page); return true; }
async function type(page, text) { await control(page, 'command/input'); await page.keyboard.press('Control+a'); await page.keyboard.type(text, {delay: 10}); }
async function enter(page, text) { await type(page, text); await page.keyboard.press('Enter'); await settle(page); }
async function blur(page) { const c = (await ui(page)).controls.find(c => c.key === 'command/input'); await page.mouse.click(c.rect[0] - 30, (c.rect[1] + c.rect[3]) / 2); await settle(page, 120); }
async function key(page, name) { await blur(page); await page.keyboard.press(name); await settle(page); }
async function expand(page) { for (let i = 0; i < 12; i++) { const u = await ui(page); const row = u.rows.find(r => !r.expanded && u.controls.some(c => c.key === 'open/' + r.key.split('/')[1])); if (!row) return u.rows; await control(page, 'open/' + row.key.split('/')[1]); } return (await ui(page)).rows; }
const status = page => page.evaluate(() => document.getElementById('viewer-status')?.textContent || '');
const last = async page => (await ui(page)).history.at(-1) || '';
const rows = async page => (await state(page)).selected_rows;
const count = page => page.evaluate(() => { const s = JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')); return s.selected_rows.length; });
const small = page => page.evaluate(() => { const s = JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')); return {objects: s.objects, canvas: s.logical_canvas, heap: s.wasm_capacity_bytes, buffers: s.gpu_buffer_capacity_bytes, textures: s.gpu_texture_estimate_bytes}; });
const shot = (page, name) => page.screenshot({path: path.join(out, `${name}.png`)});

/** A loop through world points, closed on screen, `steps` mouse moves per side; `mid` runs halfway. */
async function lasso(page, world, {mods = [], mid = null, steps = 6} = {}) {
  const s = await state(page);
  const at = world.map(p => project(s, p));
  for (const m of mods) await page.keyboard.down(m);
  await page.mouse.move(at[0][0], at[0][1]);
  await page.mouse.down();
  for (let i = 1; i <= at.length; i++) {
    const p = at[i % at.length];
    await page.mouse.move(p[0], p[1], {steps});
    if (mid && i === Math.floor(at.length / 2)) await mid();
  }
  await page.mouse.up();
  for (const m of mods) await page.keyboard.up(m);
  await settle(page);
}
const box = (x0, y0, x1, y1) => [[x0, y0, 0], [x1, y0, 0], [x1, y1, 0], [x0, y1, 0]];
/** A 24-point circle of radius r CSS px around screen point c, as a pixel loop. */
async function circle(page, c, r, mods = []) {
  for (const m of mods) await page.keyboard.down(m);
  const pts = Array.from({length: 24}, (_, i) => [c[0] + r * Math.cos(i * Math.PI / 12), c[1] + r * Math.sin(i * Math.PI / 12)]);
  await page.mouse.move(pts[0][0], pts[0][1]); await page.mouse.down();
  for (let i = 1; i <= 24; i++) await page.mouse.move(pts[i % 24][0], pts[i % 24][1], {steps: 2});
  await page.mouse.up();
  for (const m of mods) await page.keyboard.up(m);
  await settle(page);
}

async function open(browser, options, yaml, query = '?data=off&inspect=1', count = null) {
  const context = await browser.newContext(options);
  const page = await context.newPage();
  const errors = [];
  page.on('pageerror', e => errors.push(e.message));
  page.on('console', m => { if ((m.type() === 'error' || /validation/i.test(m.text())) && !/cancelable=false/.test(m.text())) errors.push(m.text()); });
  await page.route('**/favicon.ico', r => r.fulfill({status: 204}));
  if (yaml === 'nested') {
    await page.route('**/view_local.yaml', r => r.fulfill({contentType: 'application/yaml', body: 'name: Editing lessons\nitems:\n  - file: extension-nested.pb\n'}));
    await page.route('**/extension-nested.pb', r => r.fulfill({path: path.join(root, 'docs/extensions/nested.pb')}));
  } else if (yaml) {
    await page.route('**/view_local.yaml', r => r.fulfill({contentType: 'application/yaml', body: yaml}));
  }
  await page.goto(new URL(query, base).href);
  await page.waitForFunction(n => { const s = document.querySelector('canvas')?.getAttribute('data-viewer-inspection'); return s && (n === null ? JSON.parse(s).objects > 0 : JSON.parse(s).objects === n); }, count, {timeout: 120000});
  await settle(page, 1500);
  return {context, page, errors};
}

/** The sizes scene: a 1-unit line, a point, a 30-unit line and a polyline, seen from the top. */
async function sizes(browser, options = {viewport: {width: 1200, height: 800}}) {
  const o = await open(browser, options, 'name: B6\nitems: []\n', '?data=off&inspect=1', 0).catch(async () => null);
  const p = o.page;
  for (const line of ['Line 0,0,0 1,0,0', 'Point 3,3,0', 'Line 50,0,0 80,0,0', 'Polyline 0,40,0 100,40,0 100,60,0']) await enter(p, line);
  await key(p, 'Escape'); await key(p, '5'); await key(p, 'f');
  await p.mouse.move(600, 350); await p.mouse.wheel(0, 300); await settle(p);
  return o;
}

async function c1(p, label) {
  const before = (await state(p)).objects;
  await enter(p, 'Select Lasso');
  const t = (await state(p)).tool;
  check(t?.command === 'Select Lasso' && t.outline === 0, `${label}: armed, no outline (${JSON.stringify(t)})`);
  let midOutline = 0;
  await lasso(p, box(-3, -3, 5, 5), {steps: 8, mid: async () => { await settle(p, 100); midOutline = (await state(p)).tool?.outline || 0; if (label === 'C1') await shot(p, 'lasso-drawing'); }});
  check(midOutline > 10, `${label}: outline grows mid-drag (${midOutline})`);
  const s = await state(p);
  check(s.selected_rows.length === 2 && s.tool === null && s.objects === before, `${label}: short line and point selected, tool ended (${JSON.stringify(s.selected_rows)})`);
  check((await status(p)) === '2 selected inside the lasso', `${label}: status '${await status(p)}'`);
  return s.selected_rows;
}

(async () => {
  const browser = await chromium.launch({executablePath: '/usr/bin/google-chrome', headless: process.env.HEADLESS === '1', args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--ozone-platform=x11']});
  const all = [];
  try {
    if (!only || only.includes('A')) {
      section = 'A names';
      const o = await open(browser, {viewport: {width: 1200, height: 800}}, 'nested', '?data=off&inspect=1', 3);
      const p = o.page;
      await type(p, 'Select By Name'); await p.keyboard.press('Enter'); await settle(p);
      let u = await ui(p);
      check(u.command === 'Select By Name ', `bare verb waits: '${u.command}'`);
      check(u.hint.startsWith('Select By Name text'), `hint '${u.hint}'`);
      check((await rows(p)).length === 0, 'nothing selected yet');
      await p.keyboard.type('beam'); await p.keyboard.press('Enter'); await settle(p);
      check((await rows(p)).length === 2, `beam selects 2 (${await last(p)})`);
      check((await last(p)) === '> Select By Name beam\n2 selected: names containing `beam`', `history '${await last(p)}'`);
      check((await state(p)).widget !== null, 'one gumball around the group');
      await shot(p, 'by-name');
      await enter(p, 'Select By Name BEAM a');
      check((await rows(p)).length === 1, `BEAM a selects 1 (${await last(p)})`);
      await enter(p, 'Select By Name placed POLYLINE');
      const placed = await rows(p);
      check(placed.length === 1, `placed POLYLINE selects 1 (${await last(p)})`);
      const placedModel = (await state(p)).selected_models[0];
      const placedCenter = (await state(p)).widget?.[0];
      await enter(p, 'Select By Name zzz');
      check((await rows(p)).length === 1 && (await last(p)).includes('the selection is unchanged'), `zzz keeps selection (${await last(p)})`);
      // hide
      await enter(p, 'Select By Name beam'); await key(p, 'h');
      check((await state(p)).hidden_count === 2, `H hides both (${(await state(p)).hidden_count})`);
      await enter(p, 'Select By Name beam');
      check((await rows(p)).length === 0 && (await last(p)).includes('no visible object'), `hidden beams not matched (${await last(p)})`);
      await key(p, 's'); await enter(p, 'Select By Name beam');
      check((await rows(p)).length === 2, 'after S both again');
      // layers: lock and eye
      await enter(p, 'Layers On'); await key(p, 'Escape'); await key(p, 'Escape');
      u = {rows: await expand(p)};
      console.log('rows', JSON.stringify(u.rows.map(r => [r.key, r.label, r.count])));
      const beamA = u.rows.find(r => /^beam a$/i.test(r.label || ''));
      if (check(!!beamA, 'layer row for Beam A')) {
        const index = beamA.key.split('/')[1];
        await control(p, 'lock/' + index);
        await enter(p, 'Select By Name beam');
        check((await rows(p)).length === 1, `locked Beam A skipped (${await last(p)})`);
        await control(p, 'lock/' + index);
        const group = (await ui(p)).rows.find(r => r.count >= 2 && r.key.startsWith('select/'));
        if (check(!!group, 'group row')) {
          const gi = group.key.split('/')[1];
          await control(p, 'hide/' + gi);
          const hidden = (await state(p)).hidden_count;
          await enter(p, 'Select By Name beam');
          check((await rows(p)).length === 0 && (await state(p)).hidden_count === hidden && hidden >= 2, `eye-hidden beams not matched, stay hidden (${hidden})`);
          await control(p, 'hide/' + gi);
        }
        await enter(p, 'Select By Name placed');
        check((await ui(p)).rows.some(r => r.selected), 'layer row of the placed polyline highlighted');
      }
      await enter(p, 'Layers Off');
      // placement: a tight circle around the placed polyline's gumball selects it
      await key(p, 'Escape'); await key(p, 'Escape');
      check(placedModel && (Math.abs(placedModel[12]) + Math.abs(placedModel[13]) + Math.abs(placedModel[14]) > 1e-6 || placedModel.some((v, i) => Math.abs(v - [1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1][i]) > 1e-9)), `placed polyline has a placement (${JSON.stringify(placedModel)})`);
      if (placedCenter) {
        const c = project(await state(p), placedCenter);
        let got = null;
        for (let r = 20; r < 700 && !got; r *= 1.4) { await enter(p, 'Select Lasso'); await circle(p, c, r); const sel = await rows(p); if (sel.length) got = {r, sel}; }
        check(got && got.sel.length === 1 && got.sel[0] === placed[0], `a loop around where the placed polyline is drawn selects it (${JSON.stringify(got)})`);
      }
      check(o.errors.length === 0, `no console errors (${o.errors.slice(0, 3).join(' | ')})`);
      await o.context.close();
    }

    if (!only || only.includes('B')) {
      section = 'B small';
      const o = await sizes(browser); const p = o.page;
      check((await state(p)).objects === 4, `four objects (${(await state(p)).objects})`);
      await type(p, 'Select Small'); await p.keyboard.press('Enter'); await settle(p);
      check((await ui(p)).command === 'Select Small ', `waits: '${(await ui(p)).command}'`);
      await p.keyboard.type('10'); await p.keyboard.press('Enter'); await settle(p);
      check((await rows(p)).length === 2 && (await last(p)).endsWith('2 selected: bounding-box diagonal below 10'), `10 selects 2 (${await last(p)})`);
      await shot(p, 'small');
      await enter(p, 'Select Small 1'); check((await rows(p)).length === 1, `1 selects only the point (${await last(p)})`);
      await enter(p, 'Select Small 50'); check((await rows(p)).length === 3, `50 selects 3 (${await last(p)})`);
      await enter(p, 'Select Small 1000'); check((await rows(p)).length === 4, `1000 selects 4 (${await last(p)})`);
      for (const bad of ['Select Small 0', 'Select Small -5', 'Select Small abc']) {
        await enter(p, bad); check((await rows(p)).length === 4, `${bad} errs, selection kept (${await last(p)})`);
      }
      await enter(p, 'Select Small 1'); await key(p, 'h');
      await enter(p, 'Select Small 10'); check((await rows(p)).length === 1, `hidden point skipped (${await last(p)})`);
      await key(p, 's'); await key(p, 'Escape'); await key(p, 'Escape');

      section = 'C lasso';
      const two = await c1(p, 'C1');
      await enter(p, 'Select Lasso');
      await lasso(p, box(40, 35, 110, 65));
      check((await status(p)) === 'nothing visible lies entirely inside the lasso; the selection is unchanged' && JSON.stringify(await rows(p)) === JSON.stringify(two), `C2 half polyline: '${await status(p)}'`);
      await key(p, 'Escape'); await key(p, 'Escape');
      check((await state(p)).tool === null, 'Esc ends the lasso');
      // C3 concave
      await enter(p, 'Select Lasso');
      await lasso(p, [[-2, -2, 0], [6, -2, 0], [6, 1.5, 0], [2, 1.5, 0], [2, 6, 0], [-2, 6, 0]], {mid: () => shot(p, 'lasso-concave')});
      check((await rows(p)).length === 1, `C3 C-shape selects only the short line (${JSON.stringify(await rows(p))} '${await status(p)}')`);
      // C4 modifiers
      await enter(p, 'Select Lasso'); await lasso(p, box(-3, -3, 5, 5));
      await enter(p, 'Select Lasso'); await lasso(p, box(45, -3, 85, 3), {mods: ['Shift']});
      check((await rows(p)).length === 3 && (await status(p)) === '1 inside the lasso · 3 selected', `C4 Shift adds: '${await status(p)}'`);
      await enter(p, 'Select Lasso'); await lasso(p, box(2, 2, 4, 4), {mods: ['Control']});
      check((await rows(p)).length === 2 && (await status(p)) === '1 inside the lasso · 2 selected', `C4 Ctrl removes: '${await status(p)}'`);
      await shot(p, 'lasso-selected');
      // C5 click, then drag; Esc
      await key(p, 'Escape'); await key(p, 'Escape');
      await enter(p, 'Select Lasso');
      const empty = project(await state(p), [30, 20, 0]);
      await p.mouse.click(empty[0], empty[1]); await settle(p);
      check((await state(p)).tool !== null, 'C5 a click keeps the lasso armed');
      await lasso(p, box(-3, -3, 5, 5)); check((await rows(p)).length === 2, 'C5 the next drag lassoes');
      await enter(p, 'Select Lasso'); await key(p, 'Escape');
      check((await state(p)).tool === null && (await rows(p)).length === 2, 'C5 Esc on the canvas ends the lasso, keeps the selection');
      await key(p, 'Escape');
      const mvp = (await state(p)).mvp;
      await lasso(p, box(20, 10, 45, 30));
      check((await rows(p)).length === 0 && (await state(p)).tool === null, 'C5 a later left drag on empty space selects nothing');
      await p.mouse.move(600, 400); await p.mouse.down({button: 'right'}); await p.mouse.move(640, 420, {steps: 5}); await p.mouse.up({button: 'right'}); await settle(p);
      check(JSON.stringify((await state(p)).mvp) !== JSON.stringify(mvp), 'C5 right drag still orbits');
      await key(p, '5'); await key(p, 'f'); await p.mouse.move(600, 350); await p.mouse.wheel(0, 300); await settle(p);
      // C6 hidden, projection
      await enter(p, 'Select By Name point'); await key(p, 'h'); await key(p, 'Escape');
      await enter(p, 'Select Lasso'); await lasso(p, box(2, 2, 4, 4));
      check((await rows(p)).length === 0, `C6 hidden point not lassoed ('${await status(p)}')`);
      await key(p, 'Escape'); await key(p, 's'); await key(p, 'Escape'); await key(p, 'Escape');
      await key(p, ' '); await p.mouse.move(600, 350); await p.mouse.wheel(0, 300); await settle(p);
      await shot(p, 'perspective');
      await c1(p, 'C6 perspective/ortho toggled');
      check(o.errors.length === 0, `no console errors (${o.errors.slice(0, 3).join(' | ')})`);
      await o.context.close();

      section = 'C6 DPR 2';
      const d = await sizes(browser, {viewport: {width: 1200, height: 800}, deviceScaleFactor: 2});
      await c1(d.page, 'DPR2');
      // a loop longer than the 4096-point store thins out instead of closing early
      await key(d.page, 'Escape');
      check((await rows(d.page)).length === 0, 'Esc clears the selection before the long loop');
      await enter(d.page, 'Select Lasso');
      const view = await state(d.page), r = box(-3, -3, 5, 5).map(q => project(view, q));
      const teeth = [];
      for (let side = 0; side < 4; side++) {
        const [a, b] = [r[side], r[(side + 1) % 4]], len = Math.hypot(b[0] - a[0], b[1] - a[1]);
        const out = [(b[1] - a[1]) / len, -(b[0] - a[0]) / len], n = Math.floor(len / 4);
        const sign = (out[0] * ((a[0] + b[0]) / 2 - (r[0][0] + r[2][0]) / 2) + out[1] * ((a[1] + b[1]) / 2 - (r[0][1] + r[2][1]) / 2)) > 0 ? 1 : -1;
        for (let i = 0; i < n; i++) {
          const t = i / n, x = a[0] + (b[0] - a[0]) * t, y = a[1] + (b[1] - a[1]) * t;
          teeth.push([x, y], [x + sign * out[0] * 80, y + sign * out[1] * 80]);
        }
      }
      await d.page.mouse.move(...teeth[0]); await d.page.mouse.down();
      for (const q of [...teeth.slice(1), teeth[0]]) await d.page.mouse.move(...q, {steps: 50});
      const long = (await state(d.page)).tool?.outline || 0;
      await d.page.mouse.up(); await settle(d.page);
      check(teeth.length * 50 > 4096 && long > 1000 && long <= 4096, `a ${teeth.length * 50}-move zigzag loop keeps ${long} points`);
      check((await rows(d.page)).length === 2 && (await status(d.page)) === '2 selected inside the lasso', `the thinned loop still selects both: '${await status(d.page)}'`);
      check(d.errors.length === 0, `no console errors (${d.errors.slice(0, 3).join(' | ')})`);
      await d.context.close();
    }

    if (!only || only.includes('T')) {
      section = 'touch';
      const o = await sizes(browser, {...devices['Pixel 7']});
      const p = o.page;
      await enter(p, 'Select Lasso');
      const cdp = await p.context().newCDPSession(p);
      const s = await state(p);
      const at = box(45, -3, 85, 3).map(q => project(s, q));
      const path_ = [];
      for (let i = 0; i <= 4; i++) { const a = at[i % 4], b = at[(i + 1) % 4]; for (let k = 0; k < 4; k++) path_.push([a[0] + (b[0] - a[0]) * k / 4, a[1] + (b[1] - a[1]) * k / 4]); }
      const touch = (type, q) => cdp.send('Input.dispatchTouchEvent', {type, touchPoints: q ? [{x: q[0], y: q[1], id: 1}] : []});
      await touch('touchStart', path_[0]);
      console.log('touch path', JSON.stringify(path_.map(q => q.map(Math.round))), JSON.stringify((await ui(p)).scene_rect), JSON.stringify(s.logical_canvas));
      for (const q of path_.slice(1)) { await touch('touchMove', q); await p.waitForTimeout(16); }
      await shot(p, 'touch-mid');
      await touch('touchEnd'); await settle(p);
      check((await rows(p)).length === 1 && (await state(p)).tool === null && (await status(p)) === '1 selected inside the lasso', `touch lasso selects the 30-unit line (${JSON.stringify(await rows(p))} '${await status(p)}')`);
      // phone keyboard: Select By Name typed through the command line
      await enter(p, 'Select By Name point');
      check((await rows(p)).length === 1, `phone Select By Name (${await last(p)})`);
      check(o.errors.length === 0, `no console errors (${o.errors.slice(0, 3).join(' | ')})`);
      await o.context.close();
    }

    if (!only || only.includes('D')) {
      section = 'D local';
      const o = await open(browser, {viewport: {width: 1200, height: 800}}, null, '?data=off&inspect=1');
      const p = o.page;
      await p.waitForTimeout(4000); await settle(p);
      const s0 = await state(p);
      await enter(p, 'Select Small 1e30');
      const visible = (await rows(p)).length;
      await key(p, 'Escape'); await key(p, 'Escape');
      await enter(p, 'Select Lasso');
      await p.evaluate(() => { window.__lasso = {}; window.addEventListener('mouseup', () => { window.__lasso.up = performance.now(); const c = document.querySelector('canvas'); new MutationObserver((_, obs) => { if (JSON.parse(c.getAttribute('data-viewer-inspection')).selected_rows.length) { window.__lasso.done = performance.now(); obs.disconnect(); } }).observe(c, {attributes: true}); }, {once: true, capture: true}); });
      const [w, h] = [s0.logical_canvas[0], (await ui(p)).scene_rect[3] + 30];
      await p.mouse.move(3, 3); await p.mouse.down();
      for (const q of [[w - 3, 3], [w - 3, h - 40], [3, h - 40], [3, 3]]) await p.mouse.move(q[0], q[1], {steps: 20});
      await p.mouse.up(); await settle(p, 1000);
      const t = await p.evaluate(() => window.__lasso);
      const got = (await rows(p)).length;
      check(got === visible && got > 0, `lasso around everything selects every visible unlocked row (${got} of ${visible}, objects ${s0.objects}; '${await status(p)}')`);
      
      await enter(p, 'Select By Name my_mesh');
      check((await rows(p)).length >= 3, `Select By Name my_mesh selects the boxes (${await last(p)})`);
      await key(p, 'Escape'); await key(p, 'Escape');
      // the bunny: its name from a click-free search, then a loop around its gumball
      for (const name of []) { await enter(p, 'Select By Name ' + name); if ((await rows(p)).length === 1) break; }
      const bunny = await rows(p); const center = (await state(p)).widget?.[0];
      console.log('bunny', JSON.stringify(bunny), await last(p));
      if (bunny.length === 1 && center) {
        await key(p, 'Escape'); await key(p, 'Escape');
        const c = project(await state(p), center);
        let found = null;
        for (let r = 20; r < 600 && !found; r *= 1.3) { await enter(p, 'Select Lasso'); await circle(p, c, r); const sel = await rows(p); if (sel.length) found = {r, sel}; }
        check(found && found.sel.length === 1 && found.sel[0] === bunny[0], `a loop around the bunny selects only it (${JSON.stringify(found)})`);
      }
      check(o.errors.length === 0, `no console errors (${o.errors.slice(0, 3).join(' | ')})`);
      await o.context.close();
    }

    for (const scene of (process.env.SCENES || '').split(',').filter(Boolean)) {
      section = 'scene ' + scene;
      const o = await open(browser, {viewport: {width: 1200, height: 800}}, null, `?scene=${scene}&inspect=1`).catch(e => { check(false, 'load: ' + e.message.split('\n')[0]); return null; });
      if (!o) continue;
      const p = o.page;
      await p.waitForTimeout(8000); await settle(p);
      let info = await small(p); for (let i = 0; i < 20; i++) { await p.waitForTimeout(3000); const next = await small(p); if (next.objects === info.objects) break; info = next; } console.log('scene', scene, JSON.stringify(info));
      await enter(p, 'Select Small 1e30'); const visible = await count(p);
      await key(p, 'Escape'); await key(p, 'Escape');
      if (process.env.ZOOM) { await p.mouse.move(600, 380); await p.mouse.wheel(0, Number(process.env.ZOOM)); await settle(p); }
      await enter(p, 'Select Lasso');
      const [w, h] = [info.canvas[0], (await ui(p)).scene_rect[3] + 30]; // the loop stays above the dock
      const t0 = Date.now();
      await p.mouse.move(3, 3); await p.mouse.down();
      for (const q of [[w - 3, 3], [w - 3, h - 40], [3, h - 40], [3, 3]]) await p.mouse.move(q[0], q[1], {steps: 20});
      await p.mouse.up(); await settle(p, 500);
      const got = await count(p);
      await shot(p, 'lasso-' + scene.replace(/[\/.]/g, '_'));
      check(got > 0 && got <= visible, `full-screen lasso selects ${got} (visible ${visible}; '${await status(p)}')`);
      await enter(p, 'Select By Name a');
      check(/selected|no visible/.test(await last(p)), `Select By Name a: ${(await last(p)).split('\n')[1]}`);
      await shot(p, 'scene-' + scene.replace(/[\/.]/g, '_'));
      check(o.errors.length === 0, `no console errors (${o.errors.slice(0, 3).join(' | ')})`);
      await o.context.close();
    }
  } catch (e) {
    check(false, 'exception: ' + e.stack);
  } finally {
    await browser.close();
    const failed = results.filter(r => r.startsWith('FAIL'));
    console.log(`\n${results.length - failed.length} passed, ${failed.length} failed`);
    process.exit(failed.length ? 1 : 0);
  }
})();
