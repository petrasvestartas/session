const assert = require('node:assert/strict');
const {PNG} = require('pngjs');

module.exports = async (original, helpers) => {
    await require('./path-input.cjs')(original, helpers);
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message));
            await page.goto(original.url());
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
                && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const value = async name => JSON.parse(await page.locator('canvas').getAttribute(name));
            const usage = () => value('data-point-gpu');
            const quiet = async () => { await page.keyboard.press('Escape'); await page.waitForTimeout(120); };
            const disc = async () => {
                await quiet(); const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
                const p = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
                const linear = v => { v /= 255; return v <= 0.04045 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4; };
                const x = Math.floor(p.width / 2), y = Math.floor(p.height / 2);
                const interior = linear(p.data[(y * p.width + x) * 4]);
                assert(Math.abs(interior - 0.5) < 0.025, `Point must blend once: ${interior}`);
                let area = 0, width = 0, height = 0;
                for (let dy = -16 * density; dy <= 16 * density; dy++) {
                    for (let dx = -16 * density; dx <= 16 * density; dx++) {
                        const red = p.data[((y + dy) * p.width + x + dx) * 4];
                        area += (1 - linear(red)) / (1 - interior);
                        if (dy === 0 && red < 254) width++;
                        if (dx === 0 && red < 254) height++;
                    }
                }
                assert(Math.abs(area - Math.PI * (6 * density) ** 2) < 1.5, `Actual marker area: ${area}`);
                assert.equal(width, 12 * density); assert.equal(height, width);
                return {area, width, height};
            };
            await helpers.command(page, 'Close'); const empty = await helpers.drawing(page);
            await helpers.command(page, 'Example Point'); const expected = await disc();
            assert.deepEqual((await usage()).slice(0, 2), [1, 32]);
            assert.deepEqual((await value('data-point-cpu')).slice(0, 2), [1, 1]);
            const uploaded = await usage(), drawn = await helpers.drawing(page); assert.notEqual(drawn, empty);
            for (const projection of ['View Perspective', 'View Orthographic']) {
                await helpers.command(page, 'View Reset'); await helpers.command(page, projection);
                assert.deepEqual(await disc(), expected);
                await helpers.command(page, 'Zoom In'); assert.deepEqual(await disc(), expected);
                assert.deepEqual(await usage(), uploaded);
            }
            await helpers.command(page, 'View Reset'); await quiet(); assert.equal(await helpers.drawing(page), drawn);
            await helpers.command(page, 'Undo'); await quiet(); assert.equal(await helpers.drawing(page), empty);
            assert.deepEqual((await usage()).slice(0, 2), [0, 0]);
            await helpers.command(page, 'Redo'); assert.deepEqual(await disc(), expected);
            await helpers.command(page, 'Save');
            assert.match((await value('data-command-ui')).history.at(-1), /cannot save point objects yet/);
            const before = await usage(); await helpers.command(page, 'Close');
            assert.deepEqual(await value('data-point-cpu'), [0, 0, 0]);
            assert.deepEqual(await usage(), [0, 0, before[2], before[3]]);
            await helpers.command(page, 'Undo'); assert.deepEqual((await usage()).slice(0, 2), [0, 0]);
            await helpers.command(page, 'Example Point'); await disc(); assert.deepEqual(errors, []);
        } finally { await context.close(); await original.bringToFront(); }
    }
    console.log(`${helpers.step.id}: actual point diameter, circular coverage, DPR, camera reuse, history and Close passed`);
};
