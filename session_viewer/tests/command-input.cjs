/** Typing belongs to the command dock; right click repeats a tool with fresh input. */
const assert = require('node:assert/strict');
const path = require('node:path');
const {chromium} = require('playwright');
const root = path.resolve(__dirname, '..');
const state = page => page.locator('canvas').getAttribute('data-viewer-inspection').then(JSON.parse);
const ui = page => page.locator('canvas').getAttribute('data-viewer-ui').then(JSON.parse);
async function settle(page) {
  await page.waitForTimeout(180);
  await page.waitForFunction(() => !JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).pick_busy);
}
async function enter(page, text) {
  const {rect} = (await ui(page)).controls.find(c => c.key === 'command/input');
  await page.mouse.click((rect[0] + rect[2]) / 2, (rect[1] + rect[3]) / 2);
  await page.keyboard.press('Control+a');
  await page.keyboard.type(text);
  await page.keyboard.press('Enter');
  await settle(page);
}
async function blur(page) {
  const {rect} = (await ui(page)).controls.find(c => c.key === 'command/input');
  await page.mouse.click(rect[0] - 15, (rect[1] + rect[3]) / 2);
  await settle(page);
}
async function rightClick(page) {
  await page.mouse.click(640, 350, {button: 'right'});
  await settle(page);
}
function world(s) {
  const m = [...s.selected_models[0]];
  for (let i = 0; i < 3; i++) m[12 + i] += s.origin[i];
  return m;
}
(async () => {
  const browser = await chromium.launch({executablePath: '/usr/bin/google-chrome', headless: process.env.HEADLESS === '1', args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--ozone-platform=x11']});
  const page = await browser.newPage({viewport: {width: 1200, height: 800}});
  const errors = [];
  page.on('pageerror', e => errors.push(e.message));
  page.on('console', m => {if (m.type() === 'error') errors.push(m.text());});
  try {
    await page.route('**/view_local.yaml', r => r.fulfill({contentType: 'application/yaml', body: 'name: Commands\nitems:\n  - file: nested.pb\n'}));
    await page.route('**/nested.pb', r => r.fulfill({path: path.join(root, 'docs/extensions/nested.pb')}));
    await page.route('**/favicon.ico', r => r.fulfill({status: 204}));
    await page.goto(new URL('?data=off&inspect=1', process.env.VIEWER_URL || 'http://127.0.0.1:8770/').href);
    await page.waitForFunction(() => {const s = document.querySelector('canvas')?.getAttribute('data-viewer-inspection'); return s && JSON.parse(s).objects === 3;}, {}, {timeout: 90000});
    await settle(page);
    await rightClick(page);
    assert.equal((await state(page)).drawing, null, 'no previous command is a no-op');
    const initial = await state(page);
    assert(Math.abs(initial.opacity - 0.95) < 1e-6, 'faces start at the requested opacity');
    assert.equal(initial.element_interactions, false, 'element interactions start off');
    assert.equal(initial.element_attributes, false, 'element attributes start off');
    for (const letter of ['g', 'o', '5', 'f']) {
      await blur(page);
      await page.keyboard.type(letter);
      await settle(page);
      assert((await ui(page)).command.toLowerCase().startsWith(letter), `first character ${letter} reaches the command dock`);
      assert.deepEqual((await state(page)).mvp, initial.mvp, 'typing does not move the camera');
      assert.equal((await state(page)).ssao, initial.ssao, 'typing does not toggle shading');
      assert.equal((await state(page)).outlines, initial.outlines, 'typing does not toggle outlines');
      await page.keyboard.press('Escape');
      await settle(page);
    }
    await blur(page);
    await page.keyboard.type('View Top');
    await page.keyboard.press('Enter');
    await settle(page);
    assert.match((await ui(page)).history.at(-1), /^> View Top\n/);
    assert.notDeepEqual((await state(page)).mvp, initial.mvp, 'the complete View command changes the camera');
    await enter(page, 'Polyline');
    assert(!(await ui(page)).controls.some(c => c.key.startsWith('command/option/') || c.key.startsWith('snap/')), 'drawing and snap options use typed commands');
    await page.keyboard.press('Escape');
    await settle(page);
    await enter(page, 'Line 0,0,0 10,0,0');
    const before = world(await state(page));
    await enter(page, 'Move 5,0,0');
    const moved = world(await state(page));
    assert(Math.abs(moved[12] - before[12] - 5) < 1e-6, 'first Move uses its supplied offset');
    await rightClick(page);
    assert.equal((await state(page)).drawing.command, 'Move');
    assert.deepEqual(world(await state(page)), moved, 'repeat asks again instead of reusing the old offset');
    await enter(page, '0,0,0');
    await enter(page, '0,7,0');
    const twice = world(await state(page));
    assert(Math.abs(twice[13] - moved[13] - 7) < 1e-6, 'repeat accepts a new offset');
    assert.equal((await state(page)).drawing, null);
    await rightClick(page);
    assert.equal((await state(page)).drawing.command, 'Move', 'point responses preserve the last command');
    await page.keyboard.press('Escape');
    await settle(page);
    const camera = (await state(page)).mvp;
    await page.mouse.move(640, 350);
    await page.mouse.down({button: 'right'});
    await page.mouse.move(700, 385, {steps: 6});
    await page.mouse.move(640, 350, {steps: 6});
    await page.mouse.up({button: 'right'});
    await settle(page);
    assert.equal((await state(page)).drawing, null, 'an orbit returning to its start does not repeat');
    await page.mouse.move(640, 350);
    await page.mouse.down({button: 'right'});
    await page.mouse.move(700, 385, {steps: 6});
    await page.mouse.up({button: 'right'});
    await settle(page);
    assert.notDeepEqual((await state(page)).mvp, camera, 'right drag still orbits');
    assert.equal((await state(page)).drawing, null);
    const jitterCamera = (await state(page)).mvp;
    await page.mouse.move(640, 350);
    await page.mouse.down({button: 'right'});
    await page.mouse.move(642, 351);
    await page.mouse.up({button: 'right'});
    await settle(page);
    assert.equal((await state(page)).drawing.command, 'Move', 'small click jitter still repeats');
    assert.deepEqual((await state(page)).mvp, jitterCamera, 'click jitter does not orbit');
    await page.keyboard.press('Escape');
    await settle(page);
    await enter(page, 'nonsense');
    await rightClick(page);
    assert.equal((await state(page)).drawing.command, 'Move', 'an invalid command does not replace the last successful command');
    assert.deepEqual(errors, []);
    console.log('PASS typing ownership, named View command, fresh Move repeat, orbit and click jitter');
    const phone = await browser.newPage({viewport: {width: 390, height: 844}, isMobile: true, hasTouch: true});
    phone.on('pageerror', e => errors.push(e.message));
    await phone.route('**/view_local.yaml', r => r.fulfill({contentType: 'application/yaml', body: 'name: Commands\nitems:\n  - file: nested.pb\n'}));
    await phone.route('**/nested.pb', r => r.fulfill({path: path.join(root, 'docs/extensions/nested.pb')}));
    await phone.route('**/favicon.ico', r => r.fulfill({status: 204}));
    await phone.goto(new URL('?data=off&inspect=1', process.env.VIEWER_URL || 'http://127.0.0.1:8770/').href);
    await phone.waitForFunction(() => {const s = document.querySelector('canvas')?.getAttribute('data-viewer-inspection'); return s && JSON.parse(s).objects === 3;}, {}, {timeout: 90000});
    const cdp = await phone.context().newCDPSession(phone);
    async function touch(type, points) {
      await cdp.send('Input.dispatchTouchEvent', {type, touchPoints: points.map(([x, y], id) => ({x, y, id}))});
      await settle(phone);
    }
    let phoneCamera = (await state(phone)).mvp;
    await touch('touchStart', [[80, 100]]);
    await touch('touchMove', [[120, 125]]);
    await touch('touchEnd', []);
    assert.notDeepEqual((await state(phone)).mvp, phoneCamera, 'one finger still orbits on a phone');
    phoneCamera = (await state(phone)).mvp;
    await touch('touchStart', [[80, 100], [150, 100]]);
    await touch('touchMove', [[100, 125], [170, 125]]);
    await touch('touchEnd', []);
    assert.notDeepEqual((await state(phone)).mvp, phoneCamera, 'two fingers still pan on a phone');
    phoneCamera = (await state(phone)).mvp;
    await touch('touchStart', [[80, 100], [150, 100]]);
    await touch('touchMove', [[55, 100], [175, 100]]);
    await touch('touchEnd', []);
    assert.notDeepEqual((await state(phone)).mvp, phoneCamera, 'a pinch still zooms on a phone');
    phoneCamera = (await state(phone)).mvp;
    await touch('touchStart', [[80, 100]]);
    await touch('touchCancel', []);
    await phone.mouse.move(150, 150);
    await settle(phone);
    assert.deepEqual((await state(phone)).mvp, phoneCamera, 'cancelled touch cannot keep moving the camera');
    assert.deepEqual(errors, []);
    console.log('PASS phone orbit, two-finger pan, pinch zoom and touch cancellation');
  } finally {
    await browser.close();
  }
})().catch(e => {console.error(e); process.exitCode = 1;});
