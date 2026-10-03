const assert = require('node:assert/strict');
const http = require('node:http');
const fs = require('node:fs');
const zlib = require('node:zlib');
const {chromium} = require('playwright');
(async () => {
    const bytes = fs.readFileSync('tests/fixtures/timber-floor.pb'), packed = zlib.gzipSync(bytes);
    const requests = [];
    let header = true;
    const server = http.createServer((req, res) => {
        res.setHeader('Access-Control-Allow-Origin', '*');
        if (req.url === '/scenes/gzip.yaml') {
            res.end(`name: gzip floor\nitems:\n  - file: floor.pb\n    encoding: gzip\n    size: ${bytes.length}\n`);
        } else if (req.url === '/floor.pb') {
            requests.push(req.headers.range);
            res.setHeader('Content-Length', packed.length);
            if (header) res.setHeader('Content-Encoding', 'gzip');
            res.end(packed);
        } else { res.writeHead(404); res.end(); }
    });
    await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
    const base = `http://127.0.0.1:${server.address().port}/`;
    const browser = await chromium.launch({channel: 'chrome', headless: false,
        args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--disable-vulkan-surface', '--ozone-platform=x11']});
    try {
        for (header of [true, false]) {
            const page = await browser.newPage(), errors = [];
            page.on('pageerror', e => errors.push(e.message));
            await page.goto(`${process.env.VIEWER_URL || 'http://127.0.0.1:8770/'}?live=off&scene=gzip&data=${encodeURIComponent(base)}&inspect=1`);
            await page.waitForFunction(() => window.viewerDiagnostics?.read().outcome === 'ready', null, {timeout: 30000});
            const report = await page.evaluate(() => viewerDiagnostics.read());
            assert(report.phases.some(p => p.name === 'decode' && p.bytes === bytes.length));
            assert.deepEqual(errors, []);
            await page.close();
        }
        assert(requests.length >= 2);
        assert(requests.every(range => range === undefined), 'gzip payloads must never receive an 8KB range probe');
        console.log(`gzip ${bytes.length} -> ${packed.length} bytes; HTTP and explicit decompression pass; no range probes`);
    } finally { await browser.close(); await new Promise(resolve => server.close(resolve)); }
})().catch(e => { console.error(e); process.exitCode = 1; });
