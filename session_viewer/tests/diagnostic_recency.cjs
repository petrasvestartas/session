const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const {chromium} = require('playwright');
async function main() {
    const source = await fs.readFile('assets/diagnostics.js', 'utf8'), fixed = Date.now();
    const iso = delta => new Date(fixed + delta).toISOString();
    const base = {version:1, tab:'other', started:iso(-3*86400000), lastSeen:iso(-5000), outcome:'failed', failure:{time:iso(-10000)}};
    const cases = [
        ['old failure with fresh heartbeat', {...base, failure:{time:iso(-3*86400000)}}, false],
        ['recent failure', base, true],
        ['future failure', {...base, failure:{time:iso(1000)}}, false],
        ['missing failure time', {...base, failure:{}}, false],
        ['future heartbeat', {...base, lastSeen:iso(1000)}, false],
        ['failure after heartbeat', {...base, lastSeen:iso(-20000)}, false],
        ['active other tab', {...base, outcome:'running'}, false],
        ['interrupted other tab', {...base, outcome:'running', lastSeen:iso(-120001)}, true],
        ['interrupted same tab', {...base, outcome:'running', tab:'current'}, true],
        ['stale interrupted tab', {...base, outcome:'running', lastSeen:iso(-7200000)}, false],
    ];
    const browser = await chromium.launch({channel:'chrome', headless:false, args:['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--disable-vulkan-surface', '--ozone-platform=x11']});
    try {
        for (const [name, report, expected] of cases) {
            const context = await browser.newContext(); const page = await context.newPage();
            await page.addInitScript(({fixed, report}) => {
                Date.now = () => fixed;
                sessionStorage.setItem('session-viewer-report:', 'current');
                localStorage.setItem('session-viewer-report:fixture', JSON.stringify(report));
            }, {fixed, report});
            await page.route('http://127.0.0.1:8789/recency-test', route => route.fulfill({contentType:'text/html', body:'<canvas id="canvas"></canvas><script src="/diagnostics.js"></script>'}));
            await page.route('**/diagnostics.js', route => route.fulfill({contentType:'text/javascript', body:source}));
            await page.goto('http://127.0.0.1:8789/recency-test');
            await page.waitForFunction(() => !!window.viewerDiagnostics);
            assert.equal(await page.evaluate(() => new Promise(done => { const make = URL.createObjectURL; URL.createObjectURL = blob => { blob.text().then(text => done(!!JSON.parse(text).previous)); return make.call(URL, blob); }; window.viewerDiagnostics.download(); })), expected, name);
            const current = await page.evaluate(() => window.viewerDiagnostics.read());
            assert.equal(current.outcome, 'running'); assert.equal(current.webgpu, true);
            await context.close(); console.log(`Report recency: ${name} passed`);
        }
    } finally { await browser.close(); }
}
main().catch(error => { console.error(error); process.exitCode=1; });
