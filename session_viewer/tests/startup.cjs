const assert = require('node:assert/strict');
const fs = require('node:fs');
const http = require('node:http');
const {chromium} = require('playwright');
(async () => {
    const bytes = fs.readFileSync('tests/fixtures/timber-floor.pb');
    let base;
    const server = http.createServer((req, res) => {
        res.setHeader('Access-Control-Allow-Origin', '*');
        if (req.url === '/scenes/startup.yaml') res.end(`name: startup floor\nitems:\n  - file: ${base}floor.pb\n`);
        else if (req.url === '/floor.pb') setTimeout(() => res.end(bytes), 700);
        else { res.writeHead(404); res.end(); }
    });
    await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
    base = `http://127.0.0.1:${server.address().port}/`;
    const browser = await chromium.launch({channel: 'chrome', headless: false,
        args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--disable-vulkan-surface', '--ozone-platform=x11']});
    try {
        const page = await browser.newPage({viewport: {width: 412, height: 915}, deviceScaleFactor: 2.625, isMobile: true, hasTouch: true});
        const logs = [], errors = [];
        page.on('console', message => { logs.push(message.text()); if (message.type() === 'error') errors.push(message.text()); });
        page.on('pageerror', e => errors.push(e.message));
        await page.goto(`${process.env.VIEWER_URL || 'http://127.0.0.1:8770/'}?live=${encodeURIComponent(base + 'scenes/startup.yaml')}&notify=off&inspect=1`);
        await page.waitForFunction(() => window.viewerDiagnostics?.read().outcome === 'ready');
        const report = await page.evaluate(() => viewerDiagnostics.read());
        const warm = report.phases.find(p => p.name === 'first frame prewarm');
        const decode = report.phases.find(p => p.name === 'decode');
        assert(warm && decode && warm.elapsedMs < decode.elapsedMs);
        const physical = report.phases.filter(p => p.name === 'pipeline creation' && p.source === 'physical triangle opaque');
        assert.equal(physical.length, 1, 'compile the correct capped-DPR target once');
        assert(physical[0].elapsedMs <= warm.elapsedMs, 'visible faces compile before download finishes');
        await page.waitForTimeout(1200);
        assert(!logs.some(line => /Arctic .*pipelines ready/.test(line)), 'inactive Arctic does not prewarm during startup');
        const state = await page.locator('canvas').getAttribute('data-viewer-inspection').then(JSON.parse);
        assert.equal(state.samples, 1);
        assert.equal(state.opacity, 1);
        const ui = await page.locator('canvas').getAttribute('data-viewer-ui').then(JSON.parse);
        const {rect} = ui.controls.find(c => c.key === 'command/input');
        await page.mouse.click((rect[0] + rect[2]) / 2, (rect[1] + rect[3]) / 2);
        await page.keyboard.type('Arctic On');
        await page.keyboard.press('Enter');
        for (let i = 0; i < 100 && !logs.some(line => /Arctic .*pipelines ready/.test(line)); i++) await page.waitForTimeout(100);
        assert(logs.some(line => /Arctic .*pipelines ready/.test(line)), 'explicit Arctic still becomes available');
        assert.deepEqual(errors, []);
        console.log(`first-frame prewarm ${warm.durationMs.toFixed(1)} ms, before decode; one solid-phone face compilation; no startup Arctic; explicit Arctic passes`);
    } finally { await browser.close(); await new Promise(resolve => server.close(resolve)); }
})().catch(e => { console.error(e); process.exitCode = 1; });
