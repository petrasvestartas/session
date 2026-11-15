const assert = require('node:assert/strict');
const http = require('node:http');
const fs = require('node:fs');
const path = require('node:path');
const {chromium} = require('playwright');
(async () => {
    const bytes = fs.readFileSync('tests/fixtures/timber-floor.pb');
    let polls = 0, files = 0, base, changed = false;
    const server = http.createServer((req, res) => {
        res.setHeader('Access-Control-Allow-Origin', '*');
        res.setHeader('Access-Control-Expose-Headers', 'ETag');
        res.setHeader('Access-Control-Allow-Headers', 'If-None-Match');
        if (req.method === 'OPTIONS') { res.writeHead(204); res.end(); return; }
        if (req.url === '/scenes/live.yaml') {
            polls++;
            const tag = ['"same"', 'W/"same"', null][(polls - 1) % 3];
            if (tag) res.setHeader('ETag', tag);
            res.end(`name: retained floor\nitems:\n  - file: ${base}pb/revisions/0123456789abcdef.pb\n`
                + (changed ? 'texts:\n  - text: revision-two\n    at: [0,0,10]\n    height: 32\n' : ''));
        } else if (req.url.startsWith('/pb/revisions/')) {
            files++;
            res.end(bytes);
        } else { res.writeHead(404); res.end(); }
    });
    await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
    base = `http://127.0.0.1:${server.address().port}/`;
    const browser = await chromium.launch({
        ...(process.env.CHROME_BIN ? {executablePath: process.env.CHROME_BIN} : {channel: 'chrome'}),
        headless: process.env.VIEWER_HEADLESS === '1',
        args: JSON.parse(process.env.VIEWER_CHROME_ARGS || '["--enable-unsafe-webgpu","--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE","--disable-vulkan-surface","--ozone-platform=x11"]')});
    try {
        const page = await browser.newPage();
        await page.goto(`${process.env.VIEWER_URL || 'http://127.0.0.1:8770/'}?live=${encodeURIComponent(base + 'scenes/live.yaml')}&poll=1&notify=off&inspect=1`);
        await page.waitForFunction(() => window.viewerDiagnostics?.read().outcome === 'ready');
        while (polls < 4) await page.waitForTimeout(200);
        await page.waitForTimeout(400);
        const report = await page.evaluate(() => viewerDiagnostics.read());
        console.log(JSON.stringify({polls, files, replacements: report.liveReloads}));
        assert(report.phases.filter(p => p.name === 'manifest').length >= 4, 'all four polls must complete successfully');
        assert.equal(files, 1, 'immutable geometry downloads once');
        assert.equal(report.liveReloads, 1, 'unchanged manifest bytes cannot replace the scene when ETag representation changes');
        const read = () => JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection'));
        const before = await page.evaluate(read);
        changed = true;
        await page.waitForFunction(() => {
            const state = JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection'));
            return viewerDiagnostics.read().liveReloads === 2 && state.text_labels.some(label => label.text === 'revision-two');
        });
        const after = await page.evaluate(read);
        assert.deepEqual(after.camera_pose, before.camera_pose, 'changed live content retains the camera');
        assert.equal(files, 1, 'changed manifest reuses the immutable geometry');
        const output = process.env.VIEWER_TEST_OUTPUT;
        if (output) {
            fs.mkdirSync(output, {recursive: true});
            await page.locator('#canvas').screenshot({path: path.join(output, 'changed-manifest.png')});
            fs.writeFileSync(path.join(output, 'live-revision.json'), JSON.stringify({polls, files, before, after, report: await page.evaluate(() => viewerDiagnostics.read())}, null, 2));
        }
    } finally { await browser.close(); await new Promise(resolve => server.close(resolve)); }
})().catch(e => { console.error(e); process.exitCode = 1; });
