const assert = require('node:assert/strict');
const {chromium} = require('playwright');
const state = page => page.locator('canvas').getAttribute('data-viewer-inspection').then(JSON.parse);
async function command(page, text) {
    const ui = await page.locator('canvas').getAttribute('data-viewer-ui').then(JSON.parse);
    const {rect} = ui.controls.find(c => c.key === 'command/input');
    await page.mouse.click((rect[0] + rect[2]) / 2, (rect[1] + rect[3]) / 2);
    await page.keyboard.press('Control+a');
    await page.keyboard.type(text);
    await page.keyboard.press('Enter');
    await page.waitForFunction(text => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).history.at(-1)?.startsWith(`> ${text}\n`), text);
}
async function idle(page, label) {
    await page.mouse.click(10, 10); // dismiss command-field caret/hover repaint before measuring rest
    await page.waitForTimeout(1800);
    const before = await state(page);
    await page.waitForTimeout(500);
    const after = await state(page);
    assert.equal(after.frames, before.frames, `${label}: a settled view stops drawing`);
    return after.frames;
}
(async () => {
    const browser = await chromium.launch({channel: 'chrome', headless: false,
        args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--disable-vulkan-surface', '--ozone-platform=x11']});
    try {
        const page = await browser.newPage({viewport: {width: 412, height: 915}, deviceScaleFactor: 2.625, isMobile: true, hasTouch: true});
        const errors = [];
        page.on('pageerror', e => errors.push(e.message));
        page.on('console', m => { if (m.type() === 'error') errors.push(m.text()); });
        await page.goto((process.env.VIEWER_URL || 'http://127.0.0.1:8770/') + '?live=off&scene=view_live&notify=off&inspect=1');
        await page.waitForFunction(() => window.viewerDiagnostics?.read().outcome === 'ready');
        const frames = [await idle(page, 'initial floor')];
        await page.mouse.move(180, 350);
        await page.mouse.wheel(0, -1200);
        frames.push(await idle(page, 'zoomed floor'));
        assert(frames[1] > frames[0], 'zoom redraws geometry');
        await command(page, 'View Hide Edges');
        await command(page, 'View Hide Lines');
        frames.push(await idle(page, 'released visibility tables'));
        await command(page, 'View Show Edges');
        await command(page, 'View Show Lines');
        await command(page, 'Fit');
        frames.push(await idle(page, 'restored visibility tables'));
        assert.deepEqual(errors, []);
        console.log(JSON.stringify({frames, result: 'phone idle, zoom, release and restore stop drawing; no GPU errors'}));
    } finally { await browser.close(); }
})().catch(e => { console.error(e); process.exitCode = 1; });
