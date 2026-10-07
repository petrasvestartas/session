const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');

module.exports = async (original, helpers) => {
    await require('./file-read-phases.cjs')(original, helpers);
    const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, acceptDownloads: true});
    try {
        const page = await context.newPage(), errors = [];
        page.on('pageerror', error => errors.push(error.message));
        await page.goto(original.url());
        await page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
            && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
        await helpers.command(page, 'Help'); await page.keyboard.press('Escape');
        const snapshot = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
        const names = ['decode', 'validation', 'kernel', 'display walk'];
        const phases = async () => (await snapshot()).phases.filter(phase => names.some(name => phase.name.startsWith(name)));
        const state = async () => Object.fromEntries(await Promise.all([
            'data-object-count', 'data-row-placements', 'data-selected-id', 'data-camera-matrix', 'data-gpu-stats'
        ].map(async name => [name, await page.locator('canvas').getAttribute(name)])));
        const specimen = await fs.readFile(path.resolve('target', `course-${helpers.step.id}`, 'sample.pb'));
        const open = async buffer => {
            const picker = page.waitForEvent('filechooser'); await helpers.command(page, 'Open Replace');
            await (await picker).setFiles({name: 'sample.pb', mimeType: 'application/octet-stream', buffer});
        };
        const before = await state(), image = await helpers.drawing(page);
        assert.deepEqual(await phases(), []);
        await open(specimen);
        await page.waitForFunction(() => document.getElementById('status').textContent.startsWith('Document replaced'));
        const actual = await phases();
        assert.deepEqual(actual.map(phase => phase.name), names);
        let end = 0;
        for (const phase of actual) {
            assert(phase.durationMs >= 0 && Number.isFinite(phase.durationMs));
            assert(phase.elapsedMs >= end + phase.durationMs - 0.001);
            assert.equal(phase.bytes, specimen.length); assert.equal(phase.source, 'document');
            end = phase.elapsedMs;
        }
        const download = page.waitForEvent('download'); await helpers.command(page, 'Report');
        const saved = JSON.parse(await fs.readFile(await (await download).path(), 'utf8'));
        assert.deepEqual(saved.phases.filter(phase => names.includes(phase.name)), actual);
        await helpers.command(page, 'Undo'); assert.equal(await helpers.drawing(page), image);
        await helpers.command(page, 'Redo');
        const acceptedState = await state(), acceptedImage = await helpers.drawing(page);
        await open(Buffer.from([0xff]));
        await page.waitForFunction(() => document.getElementById('status').textContent.includes('Invalid session protobuf'));
        const failed = await phases();
        assert.deepEqual(failed.map(phase => phase.name), [...names, 'decode failed']);
        assert.equal(failed.at(-1).bytes, 1);
        assert.deepEqual(await state(), acceptedState); assert.equal(await helpers.drawing(page), acceptedImage);
        assert.equal((await snapshot()).outcome, 'ready');
        await helpers.command(page, 'Undo');
        const restored = await state();
        for (const key of Object.keys(before).filter(key => key !== 'data-gpu-stats')) assert.equal(restored[key], before[key]);
        assert.equal(await helpers.drawing(page), image);
        assert.deepEqual(errors, []);
        console.log(`${helpers.step.id}: measured document phases and atomic failed import passed`);
    } finally { await context.close(); await original.bringToFront(); }
};
