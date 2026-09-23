/** Large-scene command startup and hover must not eagerly collect every scene snap. */
const assert = require('node:assert/strict');
const {chromium} = require('playwright');

(async () => {
  const browser = await chromium.launch({
    executablePath: '/usr/bin/google-chrome',
    headless: process.env.HEADLESS === '1',
    args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--ozone-platform=x11'],
  });
  const page = await browser.newPage({viewport: {width: 1100, height: 800}});
  page.setDefaultTimeout(5000);
  const errors = [];
  let scenePosted = false;
  page.on('pageerror', e => errors.push(e.message));
  page.on('console', m => {
    if (m.text().includes('scene posted')) scenePosted = true;
    if (m.type() === 'error') errors.push(m.text());
  });
  const state = () => page.locator('canvas').getAttribute('data-viewer-inspection').then(JSON.parse);
  const ui = () => page.locator('canvas').getAttribute('data-viewer-ui').then(JSON.parse);
  async function command(text) {
    const rect = (await ui()).controls.find(c => c.key === 'command/input').rect;
    await page.mouse.click((rect[0] + rect[2]) / 2, (rect[1] + rect[3]) / 2);
    await page.keyboard.press('Control+a');
    await page.keyboard.type(text);
    const start = Date.now();
    await page.keyboard.press('Enter');
    await page.waitForFunction(text => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).history.at(-1)?.startsWith(`> ${text}\n`), text);
    console.log(`${text}: ${Date.now() - start} ms`);
  }
  try {
    await page.route('**/favicon.ico', r => r.fulfill({status: 204}));
    const url = new URL(process.env.VIEWER_URL || 'http://127.0.0.1:8784/');
    url.searchParams.set('scene', 'scenes/view_mixed.yaml');
    url.searchParams.set('inspect', '1');
    await page.goto(url.href);
    const started = Date.now();
    while (!scenePosted && Date.now() - started < 120000) await page.waitForTimeout(100);
    assert(scenePosted, 'mixed scene finished loading');
    await page.waitForFunction(() => JSON.parse(document.querySelector('canvas')?.getAttribute('data-viewer-inspection') || '{}').objects > 107000, null, {timeout: 120000});
    await page.waitForTimeout(500);
    assert.equal((await state()).clipping.fill, 'Solid');
    // Exercise the shared lazy snap collection while requesting the first point.
    for (const verb of ['clipping_plane', 'Line']) {
      await command(verb);
      for (const [x, y] of [[250, 250], [550, 350], [800, 450], [551, 351]]) {
        await page.mouse.move(x, y);
        await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
        assert((await state()).objects > 107000);
      }
      await command('Escape');
    }
    await command('Arctic On');
    const box = (await state()).clipping.scene_box;
    await command(`clipping_plane XY ${box.slice(0, 3).join(',')}`);
    await command('Escape');
    assert.equal((await state()).clipping.count, 1);
    await command('clipping_plane Fill Hatch');
    assert.equal((await state()).clipping.fill, 'Hatch');
    await command('clipping_plane Fill Solid');
    assert.equal((await state()).clipping.fill, 'Solid');
    await command('Outline Off');
    assert.equal((await state()).outlines, false);
    assert.equal((await state()).ssao, true);
    await command('clipping_plane Off');
    assert.equal((await state()).clipping.enabled, false);
    assert.deepEqual(errors, []);
    console.log('PASS mixed-scene clipping startup, drawing hover, fill options and independent outlines');
  } finally {
    await browser.close();
  }
})().catch(e => { console.error(e); process.exitCode = 1; });
