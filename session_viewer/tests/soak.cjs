/** Sustained orbit soak: samples WASM/GPU memory capacities and frame-time percentiles every 10 s. */
const assert = require('node:assert/strict');
const path = require('node:path');
const {chromium} = require('playwright');

const DURATION_S = Number(process.env.SOAK_SECONDS || 600);
const SAMPLE_MS = Number(process.env.SOAK_INTERVAL_MS || 10000);
const ARGS = JSON.parse(process.env.VIEWER_CHROME_ARGS || JSON.stringify([
    '--enable-unsafe-webgpu',
    '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE',
    '--disable-vulkan-surface',
    '--ozone-platform=x11',
]));

(async () => {
    const browser = await chromium.launch({
        executablePath: process.env.CHROME_BIN,
        channel: process.env.CHROME_BIN ? undefined : 'chrome',
        headless: process.env.VIEWER_HEADLESS === '1',
        args: ARGS,
    });
    try {
        const page = await browser.newPage({viewport: {width: 1200, height: 800}});
        const errors = [];
        page.on('pageerror', e => errors.push(e.message));
        if (!process.env.VIEWER_BENCH_SCENE) {
            await page.route('**/view_live.yaml', r => r.fulfill({
                contentType: 'application/yaml',
                body: 'name: Soak benchmark\nitems:\n  - file: floor.pb\n',
            }));
            await page.route('**/floor.pb', r => r.fulfill({path: path.join(__dirname, 'fixtures/timber-floor.pb')}));
        }
        const scene = process.env.VIEWER_BENCH_SCENE || 'view_live';
        const base = process.env.VIEWER_URL || 'http://127.0.0.1:8770/';
        await page.goto(`${base}?live=off&notify=off&inspect=1&spin=1&scene=${scene}`);
        await page.waitForFunction(() => window.viewerDiagnostics?.read().outcome === 'ready', null, {timeout: 90000});

        const samples = [];
        const started = Date.now();
        while (Date.now() - started < DURATION_S * 1000) {
            const s = JSON.parse(await page.locator('canvas').getAttribute('data-viewer-inspection'));
            samples.push({
                elapsedS: Math.round((Date.now() - started) / 1000),
                frames: s.frames,
                wasmCapacityBytes: s.wasm_capacity_bytes,
                gpuBufferCapacityBytes: s.gpu_buffer_capacity_bytes,
                gpuTextureEstimateBytes: s.gpu_texture_estimate_bytes,
                gapMs: s.frame_stats?.gap_ms || null,
                completionMs: s.frame_stats?.completion_ms || null,
            });
            await page.waitForTimeout(SAMPLE_MS);
        }
        assert.deepEqual(errors, []);
        const first = samples[0], last = samples.at(-1);
        console.log(JSON.stringify({
            soakSeconds: DURATION_S,
            sampleCount: samples.length,
            gpuBufferGrowthBytes: last.gpuBufferCapacityBytes - first.gpuBufferCapacityBytes,
            gpuTextureGrowthBytes: last.gpuTextureEstimateBytes - first.gpuTextureEstimateBytes,
            wasmGrowthBytes: last.wasmCapacityBytes - first.wasmCapacityBytes,
            samples,
        }, null, 2));
    } finally {
        await browser.close();
    }
})().catch(e => {
    console.error(e);
    process.exitCode = 1;
});
