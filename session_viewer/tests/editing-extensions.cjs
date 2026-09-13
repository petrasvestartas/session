const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const { chromium } = require('playwright');
const browserUi = require('../docs/extensions/browser.cjs');
const root = path.resolve(__dirname, '..');

async function state(page) {
  return page.evaluate(() => JSON.parse(document.querySelector('canvas')?.getAttribute('data-viewer-inspection') || 'null'));
}
async function wait(page, predicate, argument) {
  await page.waitForFunction(({ source, argument }) => {
    const raw = document.querySelector('canvas')?.getAttribute('data-viewer-inspection');
    return raw && Function('s', 'a', `return (${source})(s,a)`)(JSON.parse(raw), argument);
  }, { source: predicate.toString(), argument });
}
async function key(page, value) {
  const before = (await state(page)).submitted_at_ms;
  await page.keyboard.press(value);
  await wait(page, (s, a) => s.submitted_at_ms > a && !s.pick_busy, before);
  await page.waitForTimeout(150);
}
async function command(page, text) {
  await browserUi.command(page, text);
}

async function action(page, prefix, label) {
  await browserUi.button(page, prefix + '/', label);
}

async function selectCreated(page) {
  await key(page, 'l');
  await action(page, 'select', 'Select Created');
  await key(page, 'l');
  await page.locator('canvas').focus();
  await wait(page, s => s.selected !== null);
}
function project(s, p) {
  const v = p.map((n,i) => n - s.origin[i]);
  const c = [0,1,2,3].map(r => s.mvp[r]*v[0]+s.mvp[4+r]*v[1]+s.mvp[8+r]*v[2]+s.mvp[12+r]);
  return [(c[0]/c[3]*.5+.5)*s.logical_canvas[0],(.5-c[1]/c[3]*.5)*s.logical_canvas[1]];
}
async function drag(page, from, to, cancel = false) {
  await page.mouse.move(...from);
  await page.waitForTimeout(60);
  await page.mouse.down();
  await page.mouse.move(...to, { steps: 8 });
  if (cancel) await key(page, 'Escape');
  await page.mouse.up();
  await page.waitForTimeout(150);
}
async function round(browser, config) {
  const context = await browser.newContext({ viewport: config.viewport, deviceScaleFactor: config.dpr || 1 });
  const page = await context.newPage();
  const errors = [];
  page.on('pageerror', e => errors.push(String(e)));
  page.on('console', m => { if (m.type() === 'error') errors.push(m.text()); });
  try {
    await page.route('**/view_local.yaml', r => r.fulfill({ contentType:'application/yaml', body:'name: Editing lessons\nitems:\n  - file: extension-nested.pb\n' }));
    await page.route('**/extension-nested.pb', r => r.fulfill({ path:path.join(root,'docs/extensions/nested.pb') }));
    await page.route('**/favicon.ico', r => r.fulfill({status:204}));
    await page.goto(new URL('?data=off&inspect=1', process.env.VIEWER_URL || 'http://localhost:8791/').href);
    await wait(page, s => s.objects === 3);
    await page.locator('canvas').focus();
    await key(page, '5');
    if (config.perspective) await key(page, 'Space');
    await key(page, 'l');
    for (const label of ['Editing lessons','Editing lessons','Assembly','Nested']) await action(page,'open',label);
    await action(page, 'select', 'Select Nested');
    await wait(page,s=>s.selected_group_count===2);
    await action(page, 'hide', 'Hide Nested');
    await wait(page,s=>s.hidden_count===2);
    await action(page, 'hide', 'Show Nested');
    await wait(page,s=>s.hidden_count===0);
    await action(page, 'select', 'edge: joint');
    await wait(page,s=>s.selected_group_count===2);
    await key(page, 'l');
    await key(page, 'Escape');
    await command(page,'point 180,180,0');
    await wait(page,s=>s.objects===4);
    await command(page,'undo');
    await wait(page,s=>s.objects===3);
    await command(page,'line 0,130,0 100,130,0');
    await selectCreated(page);
    await command(page,'trim 0.2 0.8');
    await wait(page,s=>s.objects===4);
    await selectCreated(page);
    await command(page,'extend -0.2 1.2');
    await command(page,'undo');
    await command(page,'undo');
    await command(page,'undo');
    await wait(page,s=>s.objects===3);
    await command(page,'polyline 0,130,0 100,130,0 100,230,0');
    await selectCreated(page);
    await key(page, 'f');
    const initial = await state(page);
    const [origin, scale] = initial.widget;
    const from = project(initial, [origin[0]+82*scale, origin[1], origin[2]]);
    await drag(page,from,[from[0]+24,from[1]]);
    assert.notDeepEqual((await state(page)).model, initial.model, 'gumball translation commits');
    await key(page, 'Control+z');
    await selectCreated(page);
    for (const handle of ['axis scale', 'rotation', 'uniform scale']) {
      const prior = await state(page);
      const [center, unit] = prior.widget;
      const position = handle === 'axis scale'
        ? [center[0] + 48 * unit, center[1], center[2]]
        : handle === 'rotation'
          ? [center[0] - 96 * Math.SQRT1_2 * unit, center[1] - 96 * Math.SQRT1_2 * unit, center[2]]
          : center;
      const start = project(prior, position);
      const end = handle === 'rotation'
        ? project(prior, [center[0] - 48 * unit, center[1] - 96 * Math.sqrt(.75) * unit, center[2]])
        : [start[0] + 18, start[1]];
      await drag(page, start, end);
      assert.notDeepEqual((await state(page)).model, prior.model, `${handle} commits`);
      await key(page, 'Control+z');
      await selectCreated(page);
    }
    const zoom = await state(page);
    const [center, unit] = zoom.widget;
    const measure = s => {
      const [center, unit] = s.widget;
      const a = project(s, center);
      const b = project(s, [center[0] + 96 * unit, center[1], center[2]]);
      return Math.hypot(b[0] - a[0], b[1] - a[1]);
    };
    await page.mouse.move(...project(zoom, center));
    await page.mouse.wheel(0, 160);
    await page.waitForTimeout(150);
    assert(Math.abs(measure(await state(page)) - measure(zoom)) < 1, 'zoom preserves screen size');
    await key(page, 'F10');
    const controls = (await state(page)).controls;
    const control = controls[2].position;
    const at = project(await state(page),control);
    await page.mouse.click(...at);
    await page.waitForTimeout(100);
    await drag(page,at,[at[0]-25,at[1]+20]);
    assert.notDeepEqual((await state(page)).controls[2].position,control,'control drag commits');
    await key(page, 'Control+z');
    await selectCreated(page);
    await key(page,'F10');
    const restored = await state(page);
    assert.deepEqual(restored.controls,controls,'undo restores controls');
    const grab = project(restored,control);
    await page.mouse.click(...grab);
    await page.waitForTimeout(100);
    await drag(page,grab,[grab[0]-20,grab[1]+15],true);
    await key(page,'F10');
    assert.deepEqual((await state(page)).controls,controls,'cancel restores controls');
    await key(page,'Escape');
    await key(page,'Escape');
    await selectCreated(page);
    await command(page,'explode');
    await wait(page,s=>s.objects===5);
    await key(page,'l');
    await action(page,'select','Select Created');
    await wait(page,s=>s.selected_group_count===2);
    await key(page,'h');
    await wait(page,s=>s.hidden_count===2);
    await command(page,'show');
    await wait(page,s=>s.hidden_count===0);
    await key(page,'l');
    await command(page,'undo');
    await wait(page,s=>s.objects===4);
    await selectCreated(page);
    if(config.resize) await page.setViewportSize(config.resize);
    await key(page,'7');
    await key(page,'t');
    const capacity=(await state(page)).widget_bytes[0];
    for(let i=0;i<5;i++) {
      await key(page,'Escape');
      assert.equal((await state(page)).widget_bytes[1],0,'deselect releases tile');
      await selectCreated(page);
      assert.equal((await state(page)).widget_bytes[0],capacity,'mesh does not grow');
    }
    const snapshot=await state(page);
    await page.mouse.move(30,30);
    await page.screenshot({path:path.join(root,`docs/screenshots/extensions-round-${config.id}.png`)});
    assert.deepEqual(errors,[]);
    console.log(`PASS round ${config.id}: nested/graph visibility, point/line/trim/extend/explode/undo, gumball drag, control commit/cancel, bounded resources`);
    return {config, objects:snapshot.objects, widgetBytes:snapshot.widget_bytes, errors};
  } finally { await context.close(); }
}
async function main() {
  const browser=await chromium.launch({executablePath:process.env.CHROME_BIN||'/usr/bin/google-chrome',headless:false,args:JSON.parse(process.env.VIEWER_CHROME_ARGS||'[]')});
  try {
    const rounds=[];
    for(const config of [
      {id:1,viewport:{width:1200,height:800}},
      {id:2,viewport:{width:960,height:720}},
      {id:3,viewport:{width:1200,height:800},perspective:true},
      {id:4,viewport:{width:1000,height:750},dpr:2},
      {id:5,viewport:{width:1200,height:800},resize:{width:900,height:700}},
    ]) rounds.push(await round(browser,config));
    await fs.writeFile(path.join(root,'docs/extensions/rounds.json'),JSON.stringify({browser:await browser.version(),rounds},null,2)+'\n');
  } finally { await browser.close(); }
}
main().catch(e=>{console.error(e);process.exitCode=1;});
