const assert = require('node:assert/strict');
const {chromium} = require('playwright');

async function snapshot(page, name) {
  return JSON.parse(await page.locator('canvas').getAttribute('data-viewer-' + name));
}
async function settle(page) {
  await page.waitForTimeout(750);
  await page.waitForFunction(() => !JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).pick_busy);
}
async function command(page, text) {
  await page.bringToFront();
  await page.locator('canvas').focus();
  const input = (await snapshot(page, 'ui')).controls.find(c => c.key === 'command/input').rect;
  await page.mouse.click((input[0] + input[2]) / 2, (input[1] + input[3]) / 2);
  await settle(page);
  await page.keyboard.press('Control+a');
  await page.keyboard.type(text);
  await page.keyboard.press('Enter');
  await settle(page);
}

(async () => {
  const browser = await chromium.launch({
    executablePath: process.env.CHROME_BIN || '/usr/bin/google-chrome',
    headless: process.env.HEADLESS === '1',
    args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--ozone-platform=x11'],
  });
  const timeout = setTimeout(() => { console.error('Drawing stalled in the large scene'); process.exitCode = 1; browser.close(); }, 45000);
  try {
    const page = await browser.newPage({viewport: {width: 1200, height: 800}});
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    await page.goto(new URL('?data=off&inspect=1', process.env.VIEWER_URL || 'http://127.0.0.1:8770/').href);
    await page.waitForFunction(() => {
      const data = document.querySelector('canvas')?.getAttribute('data-viewer-inspection');
      return data && JSON.parse(data).objects > 100000;
    });
    await page.waitForTimeout(3000);
    for (const [name, points] of [['Point', [[600, 350]]], ['Line', [[550, 320], [620, 370]]], ['Polyline', [[500, 300], [580, 340], [630, 300]]]]) {
      const started = Date.now();
      await command(page, name);
      const state = await snapshot(page, 'inspection');
      assert.equal(state.drawing?.command, name.toLowerCase(), JSON.stringify(await snapshot(page, 'ui')));
      assert(Date.now() - started < 10000, name + ' must start without scanning the scene tree for every object');
      for (const [index, point] of points.entries()) {
        await page.mouse.click(...point);
        await settle(page);
        await page.waitForFunction(({name, count}) => {
          const drawing = JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).drawing;
          return (name === 'Point' || (name === 'Line' && count === 2)) ? drawing === null : drawing?.points.length === count;
        }, {name, count: index + 1});
      }
      if (name === 'Polyline') { await page.keyboard.press('Enter'); await settle(page); }
      const after = await snapshot(page, 'inspection');
      assert.equal(after.objects, state.objects + 1, name + ' creates one object: ' + JSON.stringify({drawing: after.drawing, history: (await snapshot(page, 'ui')).history.slice(-4), command: (await snapshot(page, 'ui')).command}));
      assert.equal(after.drawing, null);
      console.log(`PASS ${name}: ${state.objects} existing objects, ${Date.now() - started} ms including input waits`);
    }
    assert.deepEqual(errors, []);
  } finally {
    clearTimeout(timeout);
    await browser.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
