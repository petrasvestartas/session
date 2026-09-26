/** Edits beside streamed sheets and clouds: draw, delete, undo, redo and layer edits in every scene. */
const assert = require('node:assert/strict');
const { chromium, devices } = require('playwright');

const base = process.env.VIEWER_URL || 'http://127.0.0.1:8770/';
const scenes = (process.env.VIEWER_SCENES || 'scenes/view_mixed.yaml,scenes/view_sheets.yaml,scenes/view_pointclouds.yaml,local').split(',');
const ui = page => page.evaluate(() => JSON.parse(document.querySelector('canvas')?.getAttribute('data-viewer-ui') || 'null'));
const inspect = page => page.evaluate(() => JSON.parse(document.querySelector('canvas')?.getAttribute('data-viewer-inspection') || 'null'));

async function until(page, test, what, timeout = 10000) {
  const end = Date.now() + timeout;
  for (;;) {
    const view = await ui(page).catch(() => null);
    const state = await inspect(page).catch(() => null);
    if (view && state && test(view, state)) return { view, state };
    if (Date.now() > end) throw new Error(`timed out: ${what}`);
    await page.waitForTimeout(60);
  }
}

async function click(page, key, button = 'left') {
  const { view } = await until(page, v => v.controls.some(c => c.key === key), `control ${key}`);
  const [x0, y0, x1, y1] = view.controls.filter(c => c.key === key).at(-1).rect;
  await page.mouse.click((x0 + x1) / 2, (y0 + y1) / 2, { button });
  await page.waitForTimeout(200);
}

// type one command; returns its answer and how long Enter took to the next frame
async function command(page, text) {
  await click(page, 'command/input');
  await page.keyboard.press('Control+a');
  await page.keyboard.type(text, { delay: 5 });
  const before = (await ui(page)).history.length;
  const t0 = await page.evaluate(() => performance.now());
  await page.keyboard.press('Enter');
  const t1 = await page.evaluate(() => new Promise(r => requestAnimationFrame(() => r(performance.now()))));
  const { view, state } = await until(page, v => v.history.length > before, `answer to ${text}`, 30000);
  return { answer: view.history.at(-1), ms: Math.round(t1 - t0), state };
}

async function row(page, label) {
  const { view } = await until(page, v => v.rows.some(r => r.label === label), `row ${label}`);
  const found = view.rows.find(r => r.label === label);
  return { ...found, index: found.key.split('/')[1] };
}

// the layer menu item `action` on the row labelled `label`
async function menu(page, label, action) {
  const target = await row(page, label);
  await click(page, `select/${target.index}`, 'right');
  await click(page, `${action}/${target.index}`);
  return target;
}

async function name(page, text) {
  await until(page, v => v.controls.some(c => c.key.startsWith('rename/')), 'name field');
  await page.keyboard.press('Control+a');
  await page.keyboard.type(text, { delay: 20 });
  await page.keyboard.press('Enter');
  await until(page, v => v.rows.some(r => r.label === text), `layer ${text}`);
}

async function settle(page) {
  await until(page, (v, s) => s.objects > 0, 'loaded', 240000);
  let last = '';
  for (let still = 0; still < 10;) {
    const s = await inspect(page);
    const key = `${s.objects}/${s.ribbons}/${s.cloud_points}`;
    still = key === last ? still + 1 : 0;
    last = key;
    await page.waitForTimeout(500);
  }
}

async function run(browser, scene) {
  // VIEWER_DEVICE=Pixel 7 runs as a phone, with touch
  const device = devices[process.env.VIEWER_DEVICE || ''] || { viewport: { width: 1400, height: 900 } };
  const page = await (await browser.newContext(device)).newPage();
  const errors = [];
  page.on('pageerror', e => errors.push(e.message));
  page.on('console', m => {
    // the test server has no favicon
    if (/favicon/.test(m.location()?.url || '')) return;
    if (m.type() === 'error' || /validation|panicked/i.test(m.text())) errors.push(`${m.text()} ${m.location()?.url || ''}`);
  });
  const extra = process.env.VIEWER_QUERY || ''; // e.g. &msaa=4
  const query = (scene === 'local' ? '?inspect=1' : `?scene=${scene}&inspect=1`) + extra;
  await page.goto(new URL(query, base).href);
  await settle(page);
  const start = await inspect(page);
  const times = {};
  const step = async (text, change) => {
    const before = (await inspect(page)).objects;
    const { answer, ms, state } = await command(page, text);
    times[text.split(' ')[0]] = ms;
    if (process.env.VIEWER_VERBOSE) console.log(`${text} -> ${answer.replace(/\n/g, ' | ')} (${state.objects})`);
    assert.equal(state.objects, before + change, `${scene} ${text}: ${answer}`);
    return state;
  };

  await step('Point 1,2,3', 1);
  const line = await step('Line 0,0,0 500,500,0', 1);
  assert.notEqual(line.selected, null, 'the new line is selected');

  // layers: a new layer under Created, the line moved onto it, then renamed; all undone
  await command(page, 'Layers On');
  await menu(page, 'Created', 'new_layer');
  await name(page, 'Edits');
  await menu(page, 'Edits', 'change_layer');
  await until(page, v => v.rows.find(r => r.label === 'Edits')?.count === 1, 'the line on Edits').catch(async error => {
    const view = await ui(page);
    const state = await inspect(page);
    console.log(JSON.stringify({ status: await page.evaluate(() => document.getElementById('viewer-status')?.textContent), history: view.history.slice(-3), rows: view.rows.map(r => [r.label, r.count, r.key]), selected: state.selected, selected_rows: state.selected_rows }));
    await page.screenshot({ path: `${process.env.VIEWER_TEST_OUTPUT || '/tmp'}/streamed-editing-fail.png` });
    throw error;
  });
  await menu(page, 'Edits', 'menu-rename');
  await name(page, 'Edits 2');
  assert.equal((await inspect(page)).objects, start.objects + 2);

  // rename, change layer, the naming rename, then the new layer itself
  for (let i = 0; i < 4; i++) await command(page, 'Undo');

  await until(page, v => !v.rows.some(r => ['Edits', 'Edits 2', 'Layer 01'].includes(r.label)), 'layer edits undone');
  assert.equal((await inspect(page)).objects, start.objects + 2);

  // the control points of an object whose layer is deleted go with it, and so does the selection
  await step('Line 0,0,0 300,-300,0', 1);
  await menu(page, 'Created', 'new_layer');
  await name(page, 'Doomed');
  await menu(page, 'Doomed', 'change_layer');
  await until(page, v => v.rows.find(r => r.label === 'Doomed')?.count === 1, 'the line on Doomed');
  await command(page, 'Controls');
  const shown = await inspect(page);
  assert(shown.selection.Controls && shown.markers > 0, `F10 shows the line controls: ${JSON.stringify(shown.selection)}`);
  const doomed = await menu(page, 'Doomed', 'menu-delete');
  await click(page, `delete_layer/${doomed.index}`);
  const { state: gone } = await until(page, v => !v.rows.some(r => r.label === 'Doomed'), 'Doomed deleted');
  assert.equal(gone.objects, start.objects + 2, 'the line went with its layer');
  assert.equal(gone.selection, 'Object', `no controls of a deleted object: ${JSON.stringify(gone.selection)}`);
  assert.equal(gone.markers, 0, 'no control dots of a deleted object');
  await command(page, 'Layers Off');

  // delete, undo, redo on the line
  const selected = await command(page, 'Line 0,0,0 400,-400,0');
  assert.equal(selected.state.objects, start.objects + 3);
  await step('Delete', -1);
  await step('Undo', 1);
  await step('Redo', -1);
  await step('Undo', 1);

  // move and trim a new curve, then undo both and the curve
  const drawn = await step('Curve 0,0,0 100,50,0 200,0,0 300,80,0', 1);
  const moved = await step('Move 0,0,50', 0);
  assert.notDeepEqual(moved.model, drawn.model, 'the curve moved');
  await step('Trim 0.2 0.8', 0);
  await step('Undo', 0);
  await step('Undo', 0);
  await step('Undo', -1);

  // repeated draw and delete: GPU capacity stays bounded, freed ids come back
  const cycles = Number(process.env.VIEWER_CYCLES || (scene === 'local' ? 50 : 0));
  const before = await inspect(page);

  for (let i = 0; i < cycles; i++) {
    await step(`Polyline 0,0,${i} 100,0,${i} 100,100,${i} 0,100,${i}`, 1);
    await step('Delete', -1);
  }

  if (cycles) {
    const after = await inspect(page);
    assert(after.gpu_buffer_capacity_bytes <= before.gpu_buffer_capacity_bytes * 1.05 + 4 * 1024 * 1024, `capacity ${before.gpu_buffer_capacity_bytes} -> ${after.gpu_buffer_capacity_bytes}`);
    console.log(`${cycles} draw/delete cycles: capacity ${before.gpu_buffer_capacity_bytes} -> ${after.gpu_buffer_capacity_bytes}, dead bytes ${after.dead_bytes}, free ids ${after.free_rows}, graves ${after.graves}`);
  }

  // the streamed sources are untouched
  const end = await inspect(page);
  assert.equal(end.cloud_points, start.cloud_points, 'streamed cloud points unchanged');
  assert(end.ribbons >= start.ribbons, 'sheet segments kept');

  assert.deepEqual(errors, []);
  await page.screenshot({ path: `${process.env.VIEWER_TEST_OUTPUT || '/tmp'}/streamed-editing-${scene.replace(/\W+/g, '_')}.png` });
  await page.close();
  console.log(`PASS ${scene}: ${start.objects} objects, ${start.cloud_points} cloud points; command ms ${JSON.stringify(times)}`);
}

(async () => {
  const browser = await chromium.launch({
    executablePath: process.env.CHROME_BIN || '/usr/bin/google-chrome',
    headless: false,
    args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--ozone-platform=x11'],
  });

  try {
    for (const scene of scenes) await run(browser, scene);
  } finally {
    await browser.close();
  }
})().catch(e => { console.error(e); process.exitCode = 1; });
