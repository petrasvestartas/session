const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');

module.exports = async (original, helpers) => {
    const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, acceptDownloads: true});
    try {
        const page = await context.newPage(), errors = [];
        page.on('pageerror', error => errors.push(error.message));
        await page.addInitScript(() => {
            const read = File.prototype.arrayBuffer;
            window.__reads = [];
            File.prototype.arrayBuffer = function (...args) {
                const entry = {started: performance.now(), name: this.name, size: this.size};
                window.__reads.push(entry);
                return Reflect.apply(read, this, args).then(buffer => new Promise((resolve, reject) => {
                    entry.ready = true;
                    entry.release = () => { entry.finished = performance.now(); resolve(buffer); };
                    entry.fail = () => { entry.finished = performance.now(); reject(new Error('read denied')); };
                }));
            };
        });
        const ready = () => page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
            && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
        const snapshot = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
        const phases = async () => (await snapshot()).phases.filter(phase => phase.name.startsWith('file read'));
        const state = async () => Object.fromEntries(await Promise.all([
            'data-object-count', 'data-row-placements', 'data-selected-id', 'data-camera-matrix', 'data-gpu-stats'
        ].map(async name => [name, await page.locator('canvas').getAttribute(name)])));
        const specimen = await fs.readFile(path.resolve('target', `course-${helpers.step.id}`, 'sample.pb'));
        const choose = async () => {
            const index = await page.evaluate(() => window.__reads.length);
            const picker = page.waitForEvent('filechooser'); await helpers.command(page, 'Open Replace');
            await (await picker).setFiles({name: 'sample.pb?private=secret#fragment', mimeType: 'application/octet-stream', buffer: specimen});
            await page.waitForFunction(index => window.__reads[index]?.ready, index);
            return index;
        };
        const settle = (index, fail = false) => page.evaluate(({index, fail}) =>
            window.__reads[index][fail ? 'fail' : 'release'](), {index, fail});
        for (const scenario of ['success', 'failure', 'cancel', 'superseded', 'exit']) {
            await page.goto(original.url()); await ready();
            await helpers.command(page, 'Help'); await page.keyboard.press('Escape');
            const before = await state(), image = await helpers.drawing(page);
            const first = await choose(); await page.waitForTimeout(180);
            assert.deepEqual(await phases(), [], 'An incomplete read has no completed measurement');
            if (scenario === 'cancel') await helpers.command(page, 'Cancel Open');
            if (scenario === 'exit') {
                await page.evaluate(() => window.dispatchEvent(new PageTransitionEvent('pagehide', {persisted: false})));
                await page.waitForFunction(() => !window.wasmBindings.runtime_running());
            }
            let second;
            if (scenario === 'superseded') second = await choose();
            await settle(first, scenario === 'failure');
            if (scenario === 'success' || scenario === 'failure') {
                await page.waitForFunction(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()).phases
                    .some(phase => phase.name.startsWith('file read')));
                const [phase] = await phases(), native = await page.evaluate(() => window.__reads[0]);
                assert.equal(phase.name, scenario === 'success' ? 'file read' : 'file read failed');
                assert.equal(phase.source, 'sample.pb');
                assert.equal(phase.bytes, scenario === 'success' ? specimen.length : 0);
                assert(phase.durationMs >= 180); assert(phase.elapsedMs >= native.finished);
                assert(phase.durationMs <= phase.elapsedMs - native.started + 2);
                if (scenario === 'failure') {
                    await page.waitForFunction(() => document.getElementById('status').textContent.includes('Cannot read file'));
                    assert.deepEqual(await state(), before); assert.equal(await helpers.drawing(page), image);
                    assert.equal((await snapshot()).outcome, 'ready', 'A rejected file read does not fail the GPU run');
                } else {
                    await page.waitForFunction(() => document.getElementById('status').textContent.startsWith('Document replaced'));
                    const download = page.waitForEvent('download'); await helpers.command(page, 'Report');
                    const saved = JSON.parse(await fs.readFile(await (await download).path(), 'utf8'));
                    assert.deepEqual(saved.phases.filter(phase => phase.name.startsWith('file read')), [phase]);
                    assert(!JSON.stringify(saved).includes('private=secret'));
                    await helpers.command(page, 'Undo'); assert.equal(await helpers.drawing(page), image);
                    await helpers.command(page, 'Redo');
                }
            } else {
                await page.waitForTimeout(120);
                assert.deepEqual(await phases(), [], 'A revoked ticket cannot add diagnostics to the current report');
                assert.deepEqual(await state(), before); assert.equal(await helpers.drawing(page), image);
                if (scenario === 'superseded') {
                    await settle(second); await page.waitForFunction(() => document.getElementById('status').textContent.startsWith('Document replaced'));
                    assert.equal((await phases()).length, 1, 'Only the replacement read owns a measurement');
                }
            }
            console.log(`${helpers.step.id}: measured file read ${scenario} passed`);
        }
        assert.deepEqual(errors, []);
    } finally { await context.close(); await original.bringToFront(); }
};
