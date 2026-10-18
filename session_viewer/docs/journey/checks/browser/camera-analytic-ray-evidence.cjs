const assert = require('node:assert/strict');
module.exports = async (original, helpers) => {
    await require('./hardware-grid-range.cjs')(original, helpers);
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message));
            const url = new URL(original.url()); url.searchParams.set('nogrid', '1');
            await page.goto(url.href);
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const canvas = page.locator('canvas'), value = name => canvas.getAttribute(name).then(JSON.parse), command = line => helpers.command(page, line);
            const project = async point => {
                const m = await value('data-render-matrix'), origin = await value('data-render-origin'), local = point.map((v, i) => v - origin[i]);
                const clip = [0, 1, 2, 3].map(row => local.reduce((sum, v, axis) => sum + m[axis * 4 + row] * v, m[12 + row]));
                const box = await canvas.boundingBox(); return [box.x + (clip[0] / clip[3] + 1) * box.width / 2, box.y + (1 - clip[1] / clip[3]) * box.height / 2];
            };
            // A real offscreen original holds depth fixed while this case isolates ray placement.
            for (const projection of ['View Orthographic', 'View Perspective']) {
                for (const line of ['Close', 'Layers Off', 'Select All', 'Delete', 'Select None', 'Snap Off', 'Grid Snap Off', 'CPlane XY', 'View Grid On', 'Box w100000,0,0 1000 10000 1000', 'View Reset', 'View Top', projection, 'Zoom Out', 'Zoom Out', 'Zoom Out']) await command(line);
                const matrix = await value('data-camera-matrix');
                assert.equal(await canvas.getAttribute('data-projection'), projection.slice(5));
                const before = await helpers.drawing(page), target = await project([1000, 500, 0]);
                await command('Point'); await page.mouse.move(target[0], target[1]);
                await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-tool'))?.cursor);
                const cursor = (await value('data-tool')).cursor, actual = await project(cursor);
                assert(Math.abs(cursor[2]) < 1e-7, 'Large-view ray reaches the declared XY plane');
                assert(Math.hypot(actual[0] - target[0], actual[1] - target[1]) < 1, 'The constructed cursor remains under the actual pointer');
                await page.mouse.click(...target); await page.keyboard.press('Escape'); await command('Select None');
                const points = (await value('data-drawing-originals')).points;
                assert.equal(points.length, 1);
                points[0].forEach((v, i) => assert(Math.abs(v - cursor[i]) < 1e-6));
                assert.deepEqual(await value('data-camera-matrix'), matrix);
                const placed = await helpers.drawing(page); assert.notEqual(placed, before);
                await command('Undo'); assert.deepEqual((await value('data-drawing-originals')).points, []);
                await command('Redo'); assert.deepEqual((await value('data-drawing-originals')).points, points);
                assert.equal(await helpers.drawing(page), placed, 'Original point pixels restore exactly');
            }
            assert.deepEqual(errors, []);
            console.log(`Analytic camera rays: actual large orthographic/perspective Point hover, XY cursor, original coordinates, stable pose, Undo/Redo and exact restoration passed at DPR ${density}`);
        } finally { await context.close(); await original.bringToFront(); }
    }
};
