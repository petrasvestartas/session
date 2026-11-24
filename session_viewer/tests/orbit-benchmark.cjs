/** Real-clock browser benchmark: orbit, bare hover and wheel across desktop and phone viewports. */
const assert = require('node:assert/strict');
const path = require('node:path');
const {chromium} = require('playwright');

const ARGS = JSON.parse(process.env.VIEWER_CHROME_ARGS || JSON.stringify([
    '--enable-unsafe-webgpu',
    '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE',
    '--disable-vulkan-surface',
    '--ozone-platform=x11',
]));

async function phaseFrames(page, ms, step) {
    const startFrames = await page.evaluate(() => window.viewerFrames || 0);
    const t0 = Date.now();
    let i = 0;
    while (Date.now() - t0 < ms) {
        await step(i++);
        await page.waitForTimeout(16);
    }
    const elapsedMs = Date.now() - t0;
    const endFrames = await page.evaluate(() => window.viewerFrames || 0);
    return {steps: i, elapsedMs, renderedFrames: endFrames - startFrames};
}

async function runProfile(browser, label, viewport, dpr, isMobile) {
    const context = await browser.newContext({viewport, deviceScaleFactor: dpr, isMobile, hasTouch: isMobile});
    const page = await context.newPage();
    const errors = [];
    page.on('pageerror', e => errors.push(e.message));
    try {
        if (!process.env.VIEWER_BENCH_SCENE) {
            await page.route('**/view_live.yaml', r => r.fulfill({
                contentType: 'application/yaml',
                body: 'name: Orbit benchmark\nitems:\n  - file: floor.pb\n',
            }));
            await page.route('**/floor.pb', r => r.fulfill({path: path.join(__dirname, 'fixtures/timber-floor.pb')}));
        }
        const scene = process.env.VIEWER_BENCH_SCENE || 'view_live';
        const extra = process.env.VIEWER_BENCH_QUERY ? `&${process.env.VIEWER_BENCH_QUERY}` : '';
        const base = process.env.VIEWER_URL || 'http://127.0.0.1:8770/';
        await page.goto(`${base}?live=off&notify=off&inspect=perf&scene=${scene}${extra}`);
        await page.waitForFunction(() => window.viewerDiagnostics?.read().outcome === 'ready', null, {timeout: 90000});
        await page.mouse.click(12, 12);
        await page.waitForTimeout(800);

        const cx = Math.round(viewport.width * 0.5), cy = Math.round(viewport.height * 0.45);
        const beforeHover = await snapshot(page);
        const hover = await phaseFrames(page, 5000, async i => {
            await page.mouse.move(cx + ((i % 40) - 20) * 3, cy + ((i % 20) - 10) * 2);
        });
        await page.waitForTimeout(300);
        const afterHover = await snapshot(page);

        await page.mouse.move(cx, cy);
        await page.mouse.down({button: 'right'});
        const orbit = await phaseFrames(page, 5000, async i => {
            await page.mouse.move(cx + Math.round(Math.sin(i * 0.08) * 120), cy + Math.round(Math.cos(i * 0.06) * 40));
        });
        await page.mouse.up({button: 'right'});
        await page.waitForTimeout(400);
        const afterOrbit = await snapshot(page);

        await page.mouse.move(cx, cy);
        const wheel = await phaseFrames(page, 5000, async i => {
            await page.mouse.wheel(0, i % 2 === 0 ? -40 : 40);
        });
        await page.waitForTimeout(500);
        const afterWheel = await snapshot(page);
        const diag = await page.evaluate(() => window.viewerDiagnostics.read());
        assert.deepEqual(errors, []);

        return {
            label,
            viewport,
            dpr,
            hover: {
                ...hover,
                hoverRedrawDelta: (afterHover.redraw_causes?.hover || 0) - (beforeHover.redraw_causes?.hover || 0),
            },
            orbit: {
                ...orbit,
                frameStats: afterOrbit.frame_stats,
                cpuMs: afterOrbit.cpu_ms,
                tier: afterOrbit.navigation_tier,
            },
            wheel: {
                ...wheel,
                frameStats: afterWheel.frame_stats,
                cpuMs: afterWheel.cpu_ms,
            },
            redrawCauses: afterWheel.redraw_causes,
            drawCalls: afterWheel.draw_calls,
            longFramesCount: (diag.longFrames || []).length,
            longFramesBlockingMs: Math.round((diag.longFrames || []).reduce((s, e) => s + (e.blockingMs || 0), 0) * 10) / 10,
        };
    } finally {
        await context.close();
    }
}

async function snapshot(page) {
    return JSON.parse(await page.locator('canvas').getAttribute('data-viewer-inspection'));
}

(async () => {
    const browser = await chromium.launch({
        executablePath: process.env.CHROME_BIN,
        channel: process.env.CHROME_BIN ? undefined : 'chrome',
        headless: process.env.VIEWER_HEADLESS === '1',
        args: ARGS,
    });
    try {
        const results = [];
        results.push(await runProfile(browser, 'desktop-1200x800', {width: 1200, height: 800}, 1, false));
        results.push(await runProfile(browser, 'phone-412x915', {width: 412, height: 915}, 2.625, true));
        console.log(JSON.stringify({benchmark: 'orbit-benchmark', results}, null, 2));
    } finally {
        await browser.close();
    }
})().catch(e => {
    console.error(e);
    process.exitCode = 1;
});
