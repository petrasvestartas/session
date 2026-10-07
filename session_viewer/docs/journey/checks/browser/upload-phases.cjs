const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');

module.exports = async (original, helpers) => {
    await require('./document-phases.cjs')(original, helpers);
    const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, acceptDownloads: true});
    try {
        const page = await context.newPage(), errors = [];
        page.on('pageerror', error => errors.push(error.message));
        await require('./lifecycle-probe.cjs')(page);
        await page.addInitScript(() => {
            const probe = window.__upload = {hold: false, pending: [], calls: 0, fences: 0};
            const submit = GPUQueue.prototype.submit, done = GPUQueue.prototype.onSubmittedWorkDone;
            GPUQueue.prototype.submit = function (commands) {
                if (commands.length === 0) probe.fences++;
                return Reflect.apply(submit, this, [commands]);
            };
            GPUQueue.prototype.onSubmittedWorkDone = function (...args) {
                probe.calls++;
                const held = probe.hold;
                return Reflect.apply(done, this, args).then(value => {
                    if (!held) return value;
                    const entry = {nativeDone: performance.now()}; probe.pending.push(entry);
                    return new Promise(resolve => { entry.release = () => {
                        entry.releaseAt = performance.now(); resolve(value);
                    }; });
                });
            };
        });
        const snapshot = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
        const ready = () => page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
            && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
        const phases = async () => (await snapshot()).phases.filter(phase => ['upload preparation', 'GPU upload ready'].includes(phase.name));
        const specimen = await fs.readFile(path.resolve('target', `course-${helpers.step.id}`, 'sample.pb'));
        for (const scenario of ['native', 'hold', 'exit', 'loss', 'replacement']) {
            await page.goto(original.url()); await ready();
            await helpers.command(page, 'Help'); await page.keyboard.press('Escape');
            if (scenario !== 'native') await page.evaluate(() => { window.__upload.hold = true; });
            const picker = page.waitForEvent('filechooser'); await helpers.command(page, 'Open Replace');
            await (await picker).setFiles({name: 'sample.pb', mimeType: 'application/octet-stream', buffer: specimen});
            await page.waitForFunction(() => document.getElementById('status').textContent.startsWith('Document replaced'));
            if (scenario !== 'native') await page.waitForFunction(() => window.__upload.pending.length === 1);
            else await page.waitForFunction(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()).phases
                .some(phase => phase.name === 'GPU upload ready'));
            const measured = await phases();
            assert.deepEqual(measured.map(phase => phase.name), scenario === 'native'
                ? ['upload preparation', 'GPU upload ready'] : ['upload preparation']);
            assert.equal(measured[0].bytes, 3 * (8 * 24 + 36 * 2 + 80));
            const probe = await page.evaluate(() => ({calls: window.__upload.calls, fences: window.__upload.fences}));
            assert.equal(probe.calls, 2); assert.equal(probe.fences, 1);
            if (scenario === 'native') assert.equal(measured[1].bytes, measured[0].bytes);
            else {
                await page.waitForTimeout(200);
                assert.deepEqual(await phases(), measured, 'CPU preparation does not claim GPU completion');
                if (scenario === 'exit') {
                    await page.evaluate(() => window.dispatchEvent(new PageTransitionEvent('pagehide', {persisted: false})));
                    await page.waitForFunction(() => !window.wasmBindings.runtime_running());
                } else if (scenario === 'loss') {
                    const download = page.waitForEvent('download'); await page.evaluate(() => window.__life.devices[0].destroy());
                    await download; await page.waitForFunction(() => !window.wasmBindings.runtime_running());
                } else if (scenario === 'replacement') {
                    const started = (await snapshot()).started;
                    await page.evaluate(() => { window.__upload.hold = false; window.wasmBindings.start(); });
                    await page.waitForFunction(started => JSON.parse(window.wasmBindings.diagnostic_snapshot()).started !== started
                        && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready', started);
                }
                const before = await snapshot();
                await page.evaluate(() => window.__upload.pending[0].release()); await page.waitForTimeout(150);
                if (scenario === 'hold') {
                    const completed = await phases(), released = await page.evaluate(() => window.__upload.pending[0].releaseAt);
                    assert.deepEqual(completed.map(phase => phase.name), ['upload preparation', 'GPU upload ready']);
                    assert(completed[1].durationMs >= 200 && completed[1].elapsedMs >= released);
                    assert.equal(completed[1].bytes, measured[0].bytes);
                } else {
                    assert.deepEqual((await snapshot()).phases, before.phases, 'A retired queue cannot append completion');
                    assert.equal((await snapshot()).outcome, before.outcome);
                }
            }
            const current = await snapshot();
            if (scenario === 'native' || scenario === 'hold' || scenario === 'replacement') {
                const download = page.waitForEvent('download'); await helpers.command(page, 'Report');
                const saved = JSON.parse(await fs.readFile(await (await download).path(), 'utf8'));
                assert.deepEqual(saved.phases, current.phases);
            }
            console.log(`${helpers.step.id}: actual upload completion ${scenario} passed`);
        }
        assert.deepEqual(errors, []);
    } finally { await context.close(); await original.bringToFront(); }
};
