const assert = require('node:assert/strict');

module.exports = async (original, helpers) => {
    await require('./record-properties.cjs')(original, helpers);
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message));
            const url = new URL(original.url()); url.searchParams.set('nogrid', '1'); url.searchParams.set('opacity', '1');
            await page.goto(url.href);
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
                && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const value = async name => JSON.parse(await page.locator('canvas').getAttribute(name));
            const command = line => helpers.command(page, line);
            for (const line of ['Layers Off', 'Select All', 'Delete', 'Example Box', 'Select None', 'Select Next', 'Move -2,0,0', 'Select None', 'Example Box', 'Fit']) await command(line);
            const placements = await value('data-row-placements'), usage = await value('data-gpu-usage');
            assert.equal(placements.length, 2); assert.equal(usage[0], 2);
            const uploads = (await value('data-gpu-stats'))[0], picture = await helpers.drawing(page);
            await command('Select Next');
            const removed = await value('data-selected-ids'); assert.equal(removed.length, 1);
            await command('Delete');
            assert.equal((await value('data-gpu-usage'))[0], 1);
            const survivor = (await value('data-row-placements'))[0];
            assert.notEqual(survivor[0], placements[0][0]);
            await command('Example Box');
            assert.equal((await value('data-gpu-stats'))[0], uploads + 1, 'A new precise source uploads once');
            assert.equal((await value('data-gpu-usage'))[1], usage[1], 'A retired first span is reused while its survivor keeps the arena alive');
            assert.deepEqual((await value('data-row-placements'))[0], survivor, 'Survivor ownership and placement stay fixed');
            await command('Undo'); await command('Undo'); await command('Select None');
            assert.deepEqual(await value('data-row-placements'), placements, 'Undo restores the original identities and placements');
            assert.equal(await helpers.drawing(page), picture, 'Undo restores exact geometry pixels without refitting');
            assert.equal((await value('data-gpu-stats'))[0], uploads + 2, 'History restores the retired original with one new upload');
            await command('Select All'); await command('Delete');
            assert.deepEqual(await value('data-gpu-usage'), [0, 0, 0, 0], 'Empty drawing releases GPU geometry while Undo retains the source');
            await command('Undo'); await command('Select None');
            assert.deepEqual(await value('data-row-placements'), placements);
            assert.equal(await helpers.drawing(page), picture);
            assert.deepEqual(errors, []);
            console.log(`Arena reclamation: independent sources, retained survivor, bounded allocation, exact Undo pixels/owners and empty release passed at DPR ${density}`);
        } finally { await context.close(); await original.bringToFront(); }
    }
};
