const assert = require('node:assert/strict');

module.exports = async (original, helpers) => {
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message)); await page.goto(original.url());
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const canvas = page.locator('canvas'); const value = async name => JSON.parse(await canvas.getAttribute(name));
            const selected = () => value('data-selected-ids');
            const click = async (p, additive = false) => {
                await page.keyboard.press('Escape'); const m = await value('data-camera-matrix'), box = await canvas.boundingBox();
                const c = [0, 1, 2, 3].map(row => p.reduce((sum, n, axis) => sum + m[axis * 4 + row] * n, m[12 + row]));
                assert(Math.abs(c[0] / c[3]) < 1 && Math.abs(c[1] / c[3]) < 1, `Selection target must stay on canvas: ${JSON.stringify(p)}`);
                const before = (await value('data-pick-gpu'))[3]; if (additive) await page.keyboard.down('Shift');
                try { await page.mouse.click(box.x + (c[0] / c[3] + 1) / 2 * box.width, box.y + (1 - c[1] / c[3]) / 2 * box.height); }
                finally { if (additive) await page.keyboard.up('Shift'); }
                await page.waitForFunction(before => { const p = JSON.parse(document.querySelector('canvas').getAttribute('data-pick-gpu')); return p[3] > before && p[0] === 0 && p[1] === 0; }, before).catch(async error => { throw new Error(`Multiple pick density ${density}, point ${JSON.stringify(p)}, clip ${JSON.stringify(c)}, GPU ${JSON.stringify(await value('data-pick-gpu'))}, selected ${JSON.stringify(await selected())}, placements ${JSON.stringify(await value('data-row-placements'))}, gizmo ${await canvas.getAttribute('data-gizmo')}`, {cause:error}); });
            };
            for (const projection of ['View Perspective', 'View Orthographic']) {
                await helpers.command(page, 'Close'); await helpers.command(page, 'Select Object');
                await helpers.command(page, 'View Grid Off'); await helpers.command(page, 'View Reset'); await helpers.command(page, projection);
                await helpers.command(page, 'Example Box'); await helpers.command(page, 'Select Next'); await helpers.command(page, 'Move -2,0,0');
                await helpers.command(page, 'Example Box'); await helpers.command(page, 'Select None'); await helpers.command(page, 'Fit');
                await click([-1.1, 0, .8]); const first = await selected(); assert.equal(first.length, 1);
                await click([.9, 0, .8], true); const pair = await selected(); assert.equal(pair.length, 2); assert.equal(pair[0], first[0]);
                await click([-1.1, 0, .8], true); assert.deepEqual(await selected(), pair, 'Repeated additive picks preserve unique pick order');
                await helpers.command(page, 'Selection'); assert.match((await value('data-command-ui')).status, /^Objects \[\d+, \d+\]$/);
                const geometry = await value('data-gpu-stats'); await helpers.command(page, 'Zoom In');
                assert.deepEqual((await value('data-gpu-stats')).slice(0, 2), geometry.slice(0, 2));
                const positions = await value('data-row-placements'); await helpers.command(page, 'Move 0,0,1');
                const moved = await value('data-row-placements'); for (let i = 0; i < 2; i++) assert.equal(moved[i][1][14], positions[i][1][14] + 1);
                await helpers.command(page, 'Undo'); assert.deepEqual(await value('data-row-placements'), positions);
                await helpers.command(page, 'Redo'); assert.deepEqual(await value('data-row-placements'), moved);
                await helpers.command(page, 'Delete'); assert.equal(await canvas.getAttribute('data-object-count'), '0'); assert.deepEqual(await selected(), []);
                await helpers.command(page, 'Undo'); assert.equal(await canvas.getAttribute('data-object-count'), '2');
                await helpers.command(page, 'Select All'); assert.equal((await selected()).length, 2);
                await helpers.command(page, 'Select Face'); assert.deepEqual(await selected(), [], 'Part selection does not retain a multiple-object set');
                await helpers.command(page, 'Select Object'); await helpers.command(page, 'Select All'); await helpers.command(page, 'Select None'); assert.deepEqual(await selected(), []);
                await helpers.command(page, 'Close'); assert.equal(await canvas.getAttribute('data-highlight-bytes'), '0');
            }
            assert.deepEqual(errors, []);
        } finally { await context.close(); }
    }
};
