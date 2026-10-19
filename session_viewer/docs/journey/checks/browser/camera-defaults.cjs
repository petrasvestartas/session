const assert = require('node:assert/strict');
module.exports = async (original, helpers) => {
    await require('./reversed-depth.cjs')(original, helpers);
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message));
            const url = new URL(original.url()); url.searchParams.set('nogrid', '1'); url.searchParams.set('opacity', '1');
            await page.goto(url.href);
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const canvas = page.locator('canvas'), value = name => canvas.getAttribute(name).then(JSON.parse);
            const command = line => helpers.command(page, line);
            for (const line of ['Layers Off', 'Select All', 'Delete', 'CPlane XY', 'Box w0,0,0 1000 1000 1000', 'Select None', 'Arctic Off', 'View Outline Off', 'View Grid Off', 'View Reset']) await command(line);
            const pose = async () => {
                assert.equal(await canvas.getAttribute('data-projection'), 'Perspective');
                assert.deepEqual(await value('data-camera-target'), [0, 0, 0]);
                const matrix = await value('data-camera-matrix');
                const expected = [Math.sqrt(3) / 4, 0.75, -0.5, 3000];
                [3, 7, 11, 15].forEach((at, i) => assert(Math.abs(matrix[at] - expected[i]) < 1e-9, 'Reset restores the physical three-metre isometric camera'));
                return matrix;
            };
            const initial = await pose(), pixels = await helpers.drawing(page), sources = await value('data-cads'), uploads = (await value('data-gpu-stats'))[0];
            assert.equal(sources.length, 1);
            await command('Zoom In');
            assert(Math.abs((await value('data-camera-matrix'))[15] - 1500) < 1e-9, 'Zoom halves the physical camera distance within matrix rounding');
            assert.notEqual(await helpers.drawing(page), pixels);
            for (const line of ['View Top', 'Fit', 'View Reset']) await command(line);
            assert.deepEqual(await pose(), initial);
            assert.equal(await helpers.drawing(page), pixels);
            assert.deepEqual(await value('data-cads'), sources);
            assert.equal((await value('data-gpu-stats'))[0], uploads);
            assert.deepEqual(errors, []);
            console.log(`Physical default camera: three-metre isometric Reset, actual original drawing, Zoom, named view/Fit/Reset, exact pixels, source preservation and geometry reuse passed at DPR ${density}`);
        } finally { await context.close(); await original.bringToFront(); }
    }
};
