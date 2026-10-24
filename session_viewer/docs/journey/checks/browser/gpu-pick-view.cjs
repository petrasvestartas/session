const assert = require('node:assert/strict');

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
            const selected = async () => page.locator('canvas').getAttribute('data-selected-id');
            const click = async p => {
                await helpers.command(page, 'Select None');
                await page.keyboard.press('Escape');
                const matrix = await value('data-camera-matrix'), box = await page.locator('canvas').boundingBox();
                const clip = [0, 1, 2, 3].map(row => p.reduce((sum, n, axis) => sum + matrix[axis * 4 + row] * n, matrix[12 + row]));
                const x = (clip[0] / clip[3] + 1) / 2, y = (1 - clip[1] / clip[3]) / 2;
                assert(x > 0 && x < 1 && y > 0 && y < .85, 'The actual source must be inside the drawing above the dock');
                const before = (await value('data-pick-gpu'))[3];
                await page.mouse.click(box.x + x * box.width, box.y + y * box.height);
                await page.waitForFunction(before => {
                    const pick = JSON.parse(document.querySelector('canvas').getAttribute('data-pick-gpu'));
                    return pick[3] > before && pick[0] === 0 && pick[1] === 0;
                }, before);
            };
            for (const projection of ['View Perspective', 'View Orthographic']) {
                for (const [command, p, controls] of [
                    ['Example Point', [0, 0, 0], false], ['Example Line', [0, 0, 0], false],
                    ['Example Polyline', [-.3, -.2, 0], false], ['Example Arrow', [.58, .3, 0], false],
                    ['Example Curve', [-.6, -.4, 0], true],
                ]) {
                    await helpers.command(page, 'Close'); await helpers.command(page, 'View Grid Off');
                    await helpers.command(page, 'View Reset'); await helpers.command(page, projection);
                    await helpers.command(page, command);
                    if (controls) await helpers.command(page, 'Controls On');
                    const usageName = command === 'Example Point' ? 'data-point-gpu'
                        : command === 'Example Line' ? 'data-stroke-usage' : 'data-path-gpu';
                    const usage = await value(usageName), geometry = await value('data-gpu-stats');
                    await click(p); assert.match(await selected(), /^ObjectId\(\d+\)$/, command + ' must resolve its original row');
                    const id = await selected();
                    assert.deepEqual(await value(usageName), usage, 'Selection must reuse the visible instance buffer');
                    await helpers.command(page, 'Zoom In'); await click(p); assert.equal(await selected(), id);
                    assert.deepEqual(await value('data-gpu-stats'), geometry, 'Camera and picking must reuse geometry');
                    await helpers.command(page, 'Undo'); assert.equal(await selected(), '', 'Picking is outside document history');
                    await helpers.command(page, 'Redo'); await click(p); assert.equal(await selected(), id);
                    await helpers.command(page, 'Delete'); assert.equal(await selected(), '');
                    await helpers.command(page, 'Undo'); await click(p); assert.equal(await selected(), id);
                    const submissions = (await value('data-pick-gpu'))[3];
                    await helpers.command(page, 'Help'); assert.equal((await value('data-pick-gpu'))[3], submissions, 'Dock clicks must not request a scene pick');
                    await helpers.command(page, 'Close'); assert.equal(await selected(), '');
                    assert.deepEqual((await value('data-pick-gpu')).slice(0, 3), [0, 0, helpers.pickUniformBytes || 144], 'Close releases targets, readback and owner buffers');
                }
            }
            await helpers.command(page, 'Close'); await helpers.command(page, 'View Grid On');
            await click([0, 0, 0]); assert.equal(await selected(), '', 'The viewport grid has no source owner');
            assert.deepEqual(errors, []);
        } finally { await context.close(); }
    }
};
