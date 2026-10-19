const assert = require('node:assert/strict');
const {PNG} = require('pngjs');

module.exports = async (original, helpers) => {
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message));
            await page.goto(original.url());
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
                && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const value = async name => JSON.parse(await page.locator('canvas').getAttribute(name));
            const quiet = async () => { await page.keyboard.press('Escape'); await page.waitForTimeout(120); };
            const pixels = async diagnostics => {
                await quiet();
                assert.equal(await value('data-normal-view'), diagnostics);
                const url = await require('./canvas-image.cjs')(page.locator('canvas'));
                const image = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
                const m = await value('data-camera-matrix');
                for (const x of [-0.35, 0.35]) {
                    const w = m[3] * x + m[15], sx = (m[0] * x + m[12]) / w, sy = (m[1] * x + m[13]) / w;
                    const col = Math.floor((sx + 1) * image.width / 2), row = Math.floor((1 - sy) * image.height / 2);
                    const actual = [...image.data.subarray((row * image.width + col) * 4, (row * image.width + col) * 4 + 3)];
                    if (diagnostics) actual.forEach((v, axis) => assert(Math.abs(v - [188, 188, 255][axis]) <= 2,
                        `Positive-Z diagnostic at DPR${density}: ${actual}`));
                    else if (x < 0) assert(actual[0] > 180 && actual[1] < 30 && actual[2] < 30);
                    else assert(actual[2] > 180 && actual[0] < 30 && actual[1] < 30);
                }
            };
            await helpers.command(page, 'Close');
            await helpers.command(page, 'View Reset');
            await helpers.command(page, 'View Normals Off');
            await helpers.command(page, 'Opacity 1');
            await quiet();
            const empty = await helpers.drawing(page);
            await helpers.command(page, 'Example Face Colors');
            const uploads = await value('data-gpu-stats');
            for (const projection of ['View Perspective', 'View Orthographic']) {
                await helpers.command(page, 'View Reset');
                await helpers.command(page, projection);
                await pixels(false);
                const surface = await helpers.drawing(page);
                await helpers.command(page, 'View Normals On');
                await pixels(true);
                await helpers.command(page, 'Zoom In');
                await pixels(true);
                await helpers.command(page, 'View Reset');
                await helpers.command(page, projection);
                await helpers.command(page, 'View Normals Off');
                await pixels(false);
                assert.equal(await helpers.drawing(page), surface, 'View commands must restore every original surface pixel');
                assert.deepEqual(await value('data-gpu-stats'), uploads, 'View commands and zoom must reuse source geometry');
            }
            await helpers.command(page, 'View Reset');
            await helpers.command(page, 'View Normals On');
            await pixels(true);
            const expected = await helpers.drawing(page);
            await helpers.command(page, 'Undo');
            await quiet();
            assert.equal(await value('data-normal-view'), true);
            assert.equal(await helpers.drawing(page), empty, 'One Undo must remove the source, ignoring view toggles');
            await helpers.command(page, 'Redo');
            await pixels(true);
            assert.equal(await helpers.drawing(page), expected);
            await helpers.command(page, 'Close');
            assert.deepEqual(await value('data-gpu-usage'), [0, 0, 0, 0]);
            assert.deepEqual(errors, []);
        } finally { await context.close(); await original.bringToFront(); }
    }
    console.log(`${helpers.step.id}: focused normal-view commands, direction pixels, exact surface restoration, buffer reuse and history passed at DPR1/2`);
};
