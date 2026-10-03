const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const {chromium} = require('playwright');
(async () => {
    const out = process.env.VIEWER_TEST_OUTPUT || '/home/petras/viewer_review_work';
    const browser = await chromium.launch({channel: 'chrome', headless: false,
        args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--disable-vulkan-surface', '--ozone-platform=x11']});
    try {
        for (const [name, viewport, scale, mobile, throttle] of [
            ['desktop', {width: 1200, height: 800}, 1, false, 1],
            ['phone', {width: 412, height: 915}, 2.625, true, 1],
            ['phone-cpu6', {width: 412, height: 915}, 2.625, true, 6],
        ]) {
            const context = await browser.newContext({viewport, deviceScaleFactor: scale, isMobile: mobile, hasTouch: mobile, acceptDownloads: true});
            const page = await context.newPage(), errors = [];
            page.on('pageerror', e => errors.push(e.message));
            const cdp = await context.newCDPSession(page);
            await cdp.send('Emulation.setCPUThrottlingRate', {rate: throttle});
            const started = Date.now();
            await page.goto((process.env.VIEWER_URL || 'http://127.0.0.1:8770/') + '?scene=view_live&notify=off&inspect=1');
            await page.waitForFunction(() => window.viewerDiagnostics?.read().outcome === 'ready', null, {timeout: 120000});
            const visibleMs = Date.now() - started;
            await page.waitForTimeout(6500);
            const report = await page.evaluate(() => viewerDiagnostics.read());
            assert.deepEqual(errors, []);
            for (const phase of ['manifest', 'download', 'decode', 'walk', 'upload', 'pipeline creation', 'first frame encode'])
                assert(report.phases.some(p => p.name === phase), `missing ${phase}`);
            assert(report.gpuAdapterInfo, 'actual WebGPU adapter info');
            const inspection = await page.locator('canvas').getAttribute('data-viewer-inspection').then(JSON.parse);
            assert(Math.abs(inspection.opacity - (mobile ? 1 : 0.95)) < 1e-6, 'phone/desktop default opacity');
            const download = page.waitForEvent('download');
            await page.evaluate(() => viewerDiagnostics.download());
            await (await download).saveAs(`${out}/${name}-report.json`);
            await page.screenshot({path: `${out}/${name}.png`});
            console.log(JSON.stringify({name, visibleMs, reloads: report.liveReloads, phases: report.phases.filter(p => ['decode', 'walk', 'upload'].includes(p.name))}));
            // A full-solid URL must survive element adoption; arbitrary explicit values win too.
            for (const opacity of [1, 0.6]) {
                await page.goto((process.env.VIEWER_URL || 'http://127.0.0.1:8770/') + `?scene=view_live&notify=off&inspect=1&opacity=${opacity}`);
                await page.waitForFunction(() => window.viewerDiagnostics?.read().outcome === 'ready');
                const actual = await page.locator('canvas').getAttribute('data-viewer-inspection').then(JSON.parse);
                assert(Math.abs(actual.opacity - opacity) < 1e-6, 'explicit opacity survives element load');
            }
            await context.close();
        }
    } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
