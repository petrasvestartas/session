const assert = require('node:assert/strict');
const {PNG} = require('pngjs');

module.exports = async (original, helpers) => {
    await require('./normal-input.cjs')(original, helpers);
    const colours = [[0.910, 0.278, 0.545], [0.604, 0.804, 0.196], [0.129, 0.588, 0.918]];
    const encode = v => Math.round((v <= 0.0031308 ? v * 12.92 : 1.055 * v ** (1 / 2.4) - 0.055) * 255);
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message)); await page.goto(original.url());
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
                && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const value = async name => JSON.parse(await page.locator('canvas').getAttribute(name));
            const quiet = async () => { await page.keyboard.press('Escape'); await page.waitForTimeout(120); };
            const image = async () => { await quiet(); const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
                return PNG.sync.read(Buffer.from(url.split(',')[1], 'base64')); };
            const at = (image, matrix, p) => {
                const clip = [0, 1, 2, 3].map(row => p.reduce((v, x, axis) => v + matrix[axis * 4 + row] * x, matrix[12 + row]));
                return [Math.floor((clip[0] / clip[3] + 1) * image.width / 2), Math.floor((1 - clip[1] / clip[3]) * image.height / 2)];
            };
            const axes = async () => {
                const pixels = await image(), m = await value('data-camera-matrix');
                for (const [axis, p] of [[0, [0.6, 0, 0]], [1, [0, 0.6, 0]], [2, [0, 0, 0.5]]]) {
                    const [col, row] = at(pixels, m, p), expected = colours[axis].map(encode); let best = Infinity;
                    assert(col > 3 && col < pixels.width - 3 && row > 3 && row < pixels.height - 3);
                    for (let y = row - 2; y <= row + 2; y++) for (let x = col - 2; x <= col + 2; x++) {
                        const offset = (y * pixels.width + x) * 4;
                        best = Math.min(best, Math.max(...expected.map((v, channel) => Math.abs(pixels.data[offset + channel] - v))));
                    }
                    assert(best <= 4, `Actual BRG axis ${axis} colour differs: ${best}, DPR${density}`);
                }
            };
            await helpers.command(page, 'Close'); await helpers.command(page, 'View Normals Off'); await helpers.command(page, 'View Grid On');
            assert.equal(await value('data-grid-enabled'), true); assert.deepEqual(await value('data-grid-gpu'), [25, 1100, 1, 1100]);
            const uploads = await value('data-grid-gpu'), stats = await value('data-gpu-stats');
            for (const projection of ['View Perspective', 'View Orthographic']) {
                await helpers.command(page, 'View Reset'); await helpers.command(page, projection); await helpers.command(page, 'View Isometric'); await axes();
                await helpers.command(page, 'Zoom In'); await axes(); assert.deepEqual(await value('data-grid-gpu'), uploads); assert.deepEqual(await value('data-gpu-stats'), stats);
            }
            await helpers.command(page, 'View Reset'); await helpers.command(page, 'Example Sharp'); const withGrid = await image(), m = await value('data-camera-matrix');
            const [col, row] = at(withGrid, m, [0, 0.35, 0]); const offset = (row * withGrid.width + col) * 4;
            await helpers.command(page, 'View Grid Off'); const withoutGrid = await image();
            assert.deepEqual([...withGrid.data.subarray(offset, offset + 4)], [...withoutGrid.data.subarray(offset, offset + 4)], 'Coplanar source surface must hide the grid');
            assert.deepEqual(await value('data-grid-gpu'), [0, 0, 1, 1100]);
            await helpers.command(page, 'View Grid On'); await helpers.command(page, 'View Reset');
            assert.equal(await value('data-grid-enabled'), true, 'Reset pose must preserve the grid view setting');
            await helpers.command(page, 'Undo'); assert.equal(await page.locator('canvas').getAttribute('data-object-count'), '0', 'Grid view changes must not enter document history');
            await helpers.command(page, 'Redo'); assert.equal(await page.locator('canvas').getAttribute('data-object-count'), '1');
            await helpers.command(page, 'Close'); assert.deepEqual(await value('data-gpu-usage'), [0, 0, 0, 0]); assert.equal((await value('data-grid-gpu'))[0], 25, 'Grid belongs to viewport state');
            await helpers.command(page, 'View Grid Off'); assert.deepEqual(await value('data-grid-gpu'), [0, 0, 2, 2200]); assert.deepEqual(errors, []);
        } finally { await context.close(); await original.bringToFront(); }
    }
    console.log(`${helpers.step.id}: actual BRG grid pixels, projections/DPR, coplanar occlusion, view reuse, history independence and release passed`);
};
