const assert = require('node:assert/strict');
const {PNG} = require('pngjs');

module.exports = async (original, helpers) => {
    await require('./marker-input.cjs')(original, helpers);
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message));
            await page.goto(original.url());
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
                && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const value = async name => JSON.parse(await page.locator('canvas').getAttribute(name));
            const paths = () => value('data-path-gpu'), markers = () => value('data-point-gpu');
            const quiet = async () => { await page.keyboard.press('Escape'); await page.waitForTimeout(120); };
            const pixels = async () => {
                await quiet(); const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
                return PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
            };
            const project = async (point, image) => {
                const m = await value('data-camera-matrix'), p = [...point, 1];
                const clip = [0, 1, 2, 3].map(row => p.reduce((sum, v, col) => sum + m[col * 4 + row] * v, 0));
                return [(clip[0] / clip[3] + 1) * image.width / 2, (1 - clip[1] / clip[3]) * image.height / 2].map(Math.floor);
            };
            const sample = async point => {
                const image = await pixels(), [x, y] = await project(point, image);
                return image.data[(y * image.width + x) * 4];
            };
            const shape = async enabled => {
                assert(await sample([0, 0.1, 0]) < 215, 'Actual quadratic arch must draw at its kernel midpoint');
                const middle = await sample([0, 0.6, 0]);
                assert(enabled ? middle < 20 : middle >= 254, 'The original middle control is a marker only when enabled');
                assert(await sample([0, 0.35, 0]) >= 254, 'The control polygon must not replace the actual curve');
                if (enabled) {
                    assert(await sample([-0.6, -0.4, 0]) < 20); assert(await sample([0.6, -0.4, 0]) < 20);
                }
            };
            await helpers.command(page, 'Close'); const empty = await helpers.drawing(page);
            await helpers.command(page, 'Example Curve'); await shape(false); const curve = await helpers.drawing(page);
            const uploaded = await paths(); assert(uploaded[0] > 20); assert.equal(uploaded[1], uploaded[0] * 72);
            assert.deepEqual((await value('data-curve-cpu')).slice(0, 3), [1, 1, 1]);
            await helpers.command(page, 'Controls On'); await shape(true);
            assert.equal(await page.locator('canvas').getAttribute('data-control-count'), '3');
            assert.deepEqual((await markers()).slice(0, 2), [3, 96]); assert.deepEqual(await paths(), uploaded);
            const controlUploaded = await markers(), controlled = await helpers.drawing(page);
            for (const projection of ['View Perspective', 'View Orthographic']) {
                await helpers.command(page, 'View Reset'); await helpers.command(page, projection); await shape(true);
                await helpers.command(page, 'Zoom In'); await shape(true);
                assert.deepEqual(await paths(), uploaded); assert.deepEqual(await markers(), controlUploaded);
            }
            await helpers.command(page, 'View Reset'); await quiet(); assert.equal(await helpers.drawing(page), controlled);
            await helpers.command(page, 'Controls Off'); await shape(false); assert.equal(await helpers.drawing(page), curve);
            assert.deepEqual((await markers()).slice(0, 2), [0, 0]);
            await helpers.command(page, 'Controls On'); await helpers.command(page, 'Undo'); await quiet();
            assert.equal(await helpers.drawing(page), empty); assert.deepEqual((await paths()).slice(0, 2), [0, 0]);
            await helpers.command(page, 'Redo'); await shape(true); assert.equal(await helpers.drawing(page), controlled);
            await helpers.command(page, 'Save'); assert.match((await value('data-command-ui')).history.at(-1), /cannot save line objects yet/);
            const beforePaths = await paths(), beforeMarkers = await markers(); await helpers.command(page, 'Close');
            assert.deepEqual(await value('data-curve-cpu'), [0, 0, 0, 0]); assert.equal(await page.locator('canvas').getAttribute('data-control-count'), '0');
            assert.deepEqual(await paths(), [0, 0, beforePaths[2], beforePaths[3]]); assert.deepEqual(await markers(), [0, 0, beforeMarkers[2], beforeMarkers[3]]);
            await helpers.command(page, 'Undo'); assert.deepEqual((await paths()).slice(0, 2), [0, 0]); assert.deepEqual(errors, []);
        } finally { await context.close(); await original.bringToFront(); }
    }
    console.log(`${helpers.step.id}: actual quadratic arch and original control pixels, DPR, camera reuse, display history and Close passed`);
};
