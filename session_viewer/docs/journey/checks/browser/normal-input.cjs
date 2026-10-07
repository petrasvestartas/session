const assert = require('node:assert/strict');
const path = require('node:path');
const {PNG} = require('pngjs');

module.exports = async (original, helpers) => {
    await require('./colour-input.cjs')(original, helpers);
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
            const pixels = async (mode, diagnostics) => {
                await quiet(); const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
                const image = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64')), m = await value('data-camera-matrix');
                assert.equal(await value('data-normal-view'), diagnostics);
                for (const x of [-0.35, 0.35]) {
                    const w = m[3] * x + m[15], sx = (m[0] * x + m[12]) / w, sy = (m[1] * x + m[13]) / w;
                    const col = Math.floor((sx + 1) * image.width / 2), row = Math.floor((1 - sy) * image.height / 2);
                    const actual = [...image.data.subarray((row * image.width + col) * 4, (row * image.width + col) * 4 + 3)];
                    const t = (x + 0.7) / 1.4;
                    const n = mode === 'Smooth' ? [t, 0, 1 - t] : [0, 0, mode === 'Winding' && x > 0 ? -1 : 1];
                    const length = Math.hypot(...n), normal = n.map(v => v / length);
                    const brightness = 0.3 + 0.7 * Math.abs((normal[0] * 0.4 - normal[1] * 0.6 + normal[2]) / Math.sqrt(1.52));
                    const expected = diagnostics ? normal.map(v => encode(v * 0.5 + 0.5)) : [0, 1, 2].map(() => encode(brightness * 0.5));
                    actual.forEach((v, axis) => assert(Math.abs(v - expected[axis]) <= 3,
                        `${mode} normal pixel ${x}, ${axis}, ${diagnostics}: ${actual}, expected ${expected}`));
                }
            };
            for (const mode of ['Sharp', 'Smooth', 'Winding']) {
                await helpers.command(page, 'Close'); await helpers.command(page, 'View Reset'); await helpers.command(page, 'View Normals Off'); await quiet();
                const empty = await helpers.drawing(page); await helpers.command(page, `Example ${mode}`); await pixels(mode, false);
                const uploads = await value('data-gpu-stats'); await helpers.command(page, 'View Normals On'); await pixels(mode, true);
                const expected = await helpers.drawing(page);
                for (const projection of ['View Perspective', 'View Orthographic']) {
                    await helpers.command(page, 'View Reset'); await helpers.command(page, projection); await pixels(mode, true);
                    await helpers.command(page, 'Zoom In'); await pixels(mode, true); assert.deepEqual(await value('data-gpu-stats'), uploads);
                    await helpers.command(page, 'View Normals Off'); await pixels(mode, false); await helpers.command(page, 'View Normals On');
                }
                await helpers.command(page, 'View Reset'); await helpers.command(page, 'Undo'); await quiet(); assert.equal(await helpers.drawing(page), empty);
                await helpers.command(page, 'Redo'); await pixels(mode, true); assert.equal(await helpers.drawing(page), expected);
                const pending = page.waitForEvent('download'); await helpers.command(page, 'Save'); const download = await pending;
                const file = path.resolve('target', `course-${helpers.step.id}`, `normal-${density}-${mode}.session`); await download.saveAs(file);
                await helpers.command(page, 'Close'); const chooser = page.waitForEvent('filechooser'); await helpers.command(page, 'Open'); await (await chooser).setFiles(file);
                await page.waitForFunction(() => document.getElementById('status')?.textContent === 'File imported. Undo removes the entire import.');
                await pixels(mode, true); assert.equal(await helpers.drawing(page), expected, 'Downloaded source attributes and face order must reconstruct identical diagnostic pixels');
                await helpers.command(page, 'Close'); assert.deepEqual(await value('data-gpu-usage'), [0, 0, 0, 0]);
            }
            assert.deepEqual(errors, []);
        } finally { await context.close(); await original.bringToFront(); }
    }
    console.log(`${helpers.step.id}: actual sharp/smooth/winding pixels, diagnostic view, geometry reuse, history and normal-attribute download/reopen passed at DPR1/2`);
};
