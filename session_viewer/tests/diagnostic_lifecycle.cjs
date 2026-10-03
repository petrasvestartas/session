const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const {chromium} = require('playwright');
async function main() {
    const source = await fs.readFile('assets/diagnostics.js', 'utf8');
    const browser = await chromium.launch({channel: 'chrome', headless: false, args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--disable-vulkan-surface', '--ozone-platform=x11']});
    try {
        for (const outcome of ['running', 'ready', 'failed']) {
            const context = await browser.newContext(), page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message));
            await page.route('http://127.0.0.1:8789/lifecycle-test', route => route.fulfill({contentType: 'text/html', body: '<canvas id="canvas"></canvas><div id="viewer-diagnostics" hidden><p id="viewer-diagnostics-message"></p></div><script src="/diagnostics.js"></script>'}));
            await page.route('**/diagnostics.js', route => route.fulfill({contentType: 'text/javascript', body: source}));
            await page.goto('http://127.0.0.1:8789/lifecycle-test'); await page.waitForFunction(() => !!window.viewerDiagnostics);
            await page.evaluate(outcome => {
                if (outcome === 'ready') window.viewerDiagnostic('milestone', 'geometry on screen');
                if (outcome === 'failed') window.viewerDiagnostic('fatal', 'original failure');
            }, outcome);
            const read = () => page.evaluate(() => window.viewerDiagnostics.read());
            const before = await read();
            for (const name of ['pagehide', 'pageshow']) {
                await page.evaluate(name => window.dispatchEvent(new PageTransitionEvent(name, {persisted: true})), name);
                const current = await read(); assert.equal(current.outcome, outcome); assert.deepEqual(current.failure, before.failure);
                assert.deepEqual(current.events, before.events);
            }
            if (outcome === 'ready') {
                await page.reload(); await page.waitForFunction(() => !!window.viewerDiagnostics);
                assert.equal(await page.locator('#viewer-diagnostics').isVisible(), false, 'A cached ready run must not create an interrupted notice');
            } else {
                await page.evaluate(() => window.dispatchEvent(new PageTransitionEvent('pagehide', {persisted: false})));
                await page.evaluate(() => window.viewerDiagnostic('milestone', 'geometry on screen'));
                const current = await read(); assert.equal(current.outcome, outcome === 'failed' ? 'failed' : 'closed');
                assert.deepEqual(current.failure, before.failure);
                const stored = await page.evaluate(started => Object.keys(localStorage).filter(key => key.startsWith('session-viewer-report:'))
                    .map(key => JSON.parse(localStorage.getItem(key))).find(report => report.started === started), current.started);
                assert.deepEqual(stored, current);
            }
            assert.deepEqual(errors, []); await context.close();
            console.log(`Production diagnostics: ${outcome} cached transitions and final/late outcome checks passed`);
        }
    } finally { await browser.close(); }
}
main().catch(error => { console.error(error); process.exitCode = 1; });
