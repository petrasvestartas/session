const assert = require('node:assert/strict');
const {PNG} = require('pngjs');

module.exports = async (original, helpers) => {
    await require('./recovery-browser.cjs')(original, helpers);
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message));
            await page.goto(original.url());
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
                && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const value = async name => JSON.parse(await page.locator('canvas').getAttribute(name));
            const usage = () => value('data-stroke-usage');
            const quiet = async () => { await page.keyboard.press('Escape'); await page.waitForTimeout(120); };
            const width = async () => {
                await quiet();
                const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
                const pixels = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
                const x = pixels.width / 2, mid = pixels.height / 2;
                let coverage = 0;
                for (let y = mid - 24; y < mid + 24; y++) {
                    const v = pixels.data[(y * pixels.width + x) * 4] / 255;
                    const linear = v <= 0.04045 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4;
                    coverage += 1 - linear;
                }
                assert(Math.abs(coverage - 5 * density) < 0.12, `Actual line width: ${coverage}, density ${density}`);
            };
            await helpers.command(page, 'Close'); assert.deepEqual((await usage()).slice(0, 2), [0, 0]);
            const empty = await helpers.drawing(page);
            await helpers.command(page, 'Example Line'); await quiet();
            assert.deepEqual((await usage()).slice(0, 2), [1, 44]);
            const visible = await helpers.drawing(page); assert.notEqual(visible, empty);
            const uploaded = await usage();
            for (const projection of ['View Perspective', 'View Orthographic']) {
                await helpers.command(page, 'View Reset'); await helpers.command(page, projection);
                await width(); await helpers.command(page, 'Zoom In'); await width();
                assert.deepEqual(await usage(), uploaded, 'Camera updates reuse all stroke vertex data');
            }
            await helpers.command(page, 'View Reset'); await quiet();
            assert.equal(await helpers.drawing(page), visible);
            await helpers.command(page, 'Undo'); await quiet();
            assert.equal(await helpers.drawing(page), empty);
            assert.deepEqual((await usage()).slice(0, 2), [0, 0]);
            await helpers.command(page, 'Redo'); await quiet();
            assert.equal(await helpers.drawing(page), visible);
            await helpers.command(page, 'Save');
            assert.match((await value('data-command-ui')).history.at(-1), /cannot save line objects yet/);
            await helpers.command(page, 'Undo'); assert.deepEqual((await usage()).slice(0, 2), [0, 0]);
            await helpers.command(page, 'Redo');
            const beforeClose = await usage(); await helpers.command(page, 'Close');
            assert.deepEqual(await value('data-line-usage'), [0, 0, 0]);
            assert.deepEqual(await usage(), [0, 0, beforeClose[2], beforeClose[3]]);
            await helpers.command(page, 'Undo'); assert.deepEqual((await usage()).slice(0, 2), [0, 0]);
            await helpers.command(page, 'Example Line'); await width();
            assert.deepEqual(errors, []);
        } finally { await context.close(); await original.bringToFront(); }
    }
    console.log(`${helpers.step.id}: actual stroke pixels, CSS density, view reuse, history and Close passed`);
};
