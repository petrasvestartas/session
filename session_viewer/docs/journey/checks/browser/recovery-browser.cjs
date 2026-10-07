const assert = require('node:assert/strict');
const fs = require('node:fs/promises');

module.exports = async (original, helpers) => {
    await require('./document-phases.cjs')(original, helpers);
    const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: 2, acceptDownloads: true});
    try {
        const page = await context.newPage(), errors = [];
        let navigations = 0;
        page.on('pageerror', error => errors.push(error.message));
        page.on('request', request => {
            if (request.isNavigationRequest() && request.frame() === page.mainFrame()
                && new URL(request.url()).pathname === new URL(original.url()).pathname) navigations++;
        });
        await require('./lifecycle-probe.cjs')(page);
        await page.addInitScript(() => window.addEventListener('pagehide', () => {
            if (window.__life?.lostCalls) sessionStorage.setItem('__oldLostCalls', JSON.stringify(window.__life.lostCalls));
        }));
        const url = new URL(original.url()); url.searchParams.set('keep', 'alpha beta'); url.hash = 'retained';
        const ready = () => page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
            && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
        const snapshot = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
        const size = () => page.locator('canvas').evaluate(canvas => [canvas.width, canvas.height]);
        const saved = async action => {
            const download = page.waitForEvent('download'); await action();
            return JSON.parse(await fs.readFile(await (await download).path(), 'utf8'));
        };
        await page.goto(url.href); await ready();
        assert.deepEqual(await size(), [1920, 1280]);
        const first = await snapshot(), before = navigations;
        const failed = await saved(() => page.evaluate(() => window.__life.devices[0].destroy()));
        assert.equal(failed.started, first.started); assert.equal(failed.outcome, 'failed');
        assert.match(failed.failure.message, /device lost/);
        await ready(); await page.waitForFunction(() => document.getElementById('status').textContent.includes('device scale 1'));
        assert.equal(navigations, before + 1, 'Device loss performs one actual page reload');
        assert.deepEqual(await size(), [960, 640]);
        const clean = new URL(page.url());
        assert.equal(clean.searchParams.get('keep'), 'alpha beta'); assert.equal(clean.hash, '#retained');
        assert(!clean.searchParams.has('recovered'));
        const recovered = await snapshot(); assert.notEqual(recovered.started, first.started);
        const previous = await saved(() => helpers.command(page, 'Report Previous'));
        assert.equal(previous.started, first.started); assert.equal(previous.failure.message, failed.failure.message);
        assert.deepEqual(await page.evaluate(() => JSON.parse(sessionStorage.getItem('__oldLostCalls'))), [], 'No GPU work follows old device loss');
        const secondBefore = navigations;
        const second = await saved(() => page.evaluate(() => window.__life.devices[0].destroy()));
        await page.waitForFunction(() => !window.wasmBindings.runtime_running()); await page.waitForTimeout(500);
        assert.equal(navigations, secondBefore, 'A reduced run never starts a repeated recovery reload');
        assert.equal(second.outcome, 'failed'); assert.equal(second.started, recovered.started);
        assert.deepEqual(await page.evaluate(() => window.__life.lostCalls), []);
        await page.reload(); await ready();
        assert.deepEqual(await size(), [1920, 1280], 'An ordinary reload restores normal high-DPI drawing');
        assert(!new URL(page.url()).searchParams.has('recovered'));
        const again = await saved(() => helpers.command(page, 'Report Previous'));
        assert.equal(again.started, second.started); assert.equal(again.failure.message, second.failure.message);
        assert.deepEqual(errors, []);
        console.log(`${helpers.step.id}: actual one-reload recovery, saved failure and normal quality passed`);
    } finally { await context.close(); await original.bringToFront(); }
};
