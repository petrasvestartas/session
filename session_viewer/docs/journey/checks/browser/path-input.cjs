const assert = require('node:assert/strict');
const {PNG} = require('pngjs');

module.exports = async (original, helpers) => {
    await require('./stroke-input.cjs')(original, helpers);
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message));
            await page.goto(original.url());
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
                && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const value = async name => JSON.parse(await page.locator('canvas').getAttribute(name));
            const usage = () => value('data-path-gpu');
            const quiet = async () => { await page.keyboard.press('Escape'); await page.waitForTimeout(120); };
            const pixels = async () => {
                await quiet(); const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
                return PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
            };
            const joint = async () => {
                const p = await pixels(), x = p.width / 2, y = p.height / 2;
                const sample = p.data[(y * p.width + x) * 4] / 255;
                const linear = sample <= 0.04045 ? sample / 12.92 : ((sample + 0.055) / 1.055) ** 2.4;
                assert(Math.abs(linear - 0.5) < 0.025, `Join must remain continuous and singly blended: ${linear}`);
            };
            await helpers.command(page, 'Close'); const empty = await helpers.drawing(page);
            await helpers.command(page, 'Example Polyline'); await joint();
            assert.deepEqual((await usage()).slice(0, 2), [2, 144]);
            assert.deepEqual((await value('data-path-cpu')).slice(0, 3), [1, 1, 1]);
            const uploaded = await usage(); const joined = await helpers.drawing(page); assert.notEqual(joined, empty);
            for (const projection of ['View Perspective', 'View Orthographic']) {
                await helpers.command(page, 'View Reset'); await helpers.command(page, projection);
                await joint(); await helpers.command(page, 'Zoom In'); await joint();
                assert.deepEqual(await usage(), uploaded, 'View-only changes reuse connected instance data');
            }
            await helpers.command(page, 'View Reset'); await quiet(); assert.equal(await helpers.drawing(page), joined);
            await helpers.command(page, 'Undo'); await quiet(); assert.equal(await helpers.drawing(page), empty);
            assert.deepEqual((await usage()).slice(0, 2), [0, 0]);
            await helpers.command(page, 'Redo'); await quiet(); assert.equal(await helpers.drawing(page), joined);
            await helpers.command(page, 'Example Arrow'); await quiet();
            const withArrow = await helpers.drawing(page); assert.notEqual(withArrow, joined);
            assert.deepEqual((await usage()).slice(0, 2), [3, 216]);
            const arrow = await pixels(); let black = 0;
            for (let y = 0; y < arrow.height - 34 * density; y++) {
                for (let x = 0; x < arrow.width; x++) if (arrow.data[(y * arrow.width + x) * 4] < 100) black++;
            }
            assert(black > 700 * density * density, 'The opaque arrow, including headed area, must actually draw');
            await helpers.command(page, 'Undo'); await quiet(); assert.equal(await helpers.drawing(page), joined);
            await helpers.command(page, 'Redo'); await quiet(); assert.equal(await helpers.drawing(page), withArrow);
            await helpers.command(page, 'Save');
            assert.match((await value('data-command-ui')).history.at(-1), /cannot save line objects yet/);
            const before = await usage(); await helpers.command(page, 'Close');
            assert.deepEqual(await value('data-path-cpu'), [0, 0, 0, 0, 0]);
            assert.deepEqual(await usage(), [0, 0, before[2], before[3]]);
            await helpers.command(page, 'Undo'); assert.deepEqual((await usage()).slice(0, 2), [0, 0]);
            await helpers.command(page, 'Example Polyline'); await joint(); assert.deepEqual(errors, []);
        } finally { await context.close(); await original.bringToFront(); }
    }
    console.log(`${helpers.step.id}: actual joined alpha, arrow pixels, DPR, camera reuse, history and Close passed`);
};
