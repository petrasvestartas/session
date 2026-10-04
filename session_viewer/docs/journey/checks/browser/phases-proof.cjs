const assert = require('node:assert/strict');
const fs = require('node:fs/promises');

module.exports = async (original, helpers) => {
    const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, acceptDownloads: true});
    try {
        const page = await context.newPage(), errors = [];
        page.on('pageerror', error => errors.push(error.message));
        await require('./lifecycle-probe.cjs')(page);
        await page.addInitScript(() => {
            if (typeof GPU === 'undefined') return;
            const scenario = new URL(location.href).searchParams.get('frameTiming') || 'native';
            const probe = window.__phase = {scenario, adapters: 0, devices: 0, queues: 0, held: false,
                released: false, nativeDone: null, releaseAt: null};
            const request = GPU.prototype.requestAdapter;
            GPU.prototype.requestAdapter = function (...args) { probe.adapters++; return Reflect.apply(request, this, args); };
            const device = GPUAdapter.prototype.requestDevice;
            GPUAdapter.prototype.requestDevice = function (...args) { probe.devices++; return Reflect.apply(device, this, args); };
            const done = GPUQueue.prototype.onSubmittedWorkDone;
            GPUQueue.prototype.onSubmittedWorkDone = function (...args) {
                const first = ++probe.queues === 1;
                return Reflect.apply(done, this, args).then(value => {
                    if (!first) return value;
                    probe.nativeDone = performance.now();
                    if (scenario === 'native') return value;
                    probe.held = true;
                    return new Promise(resolve => { probe.release = () => {
                        probe.released = true; probe.releaseAt = performance.now(); resolve(value);
                    }; });
                });
            };
        });
        const snapshot = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
        const ready = () => page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
            && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
        const typed = async () => {
            const file = page.waitForEvent('download'); await helpers.command(page, 'Diagnostic Report');
            return JSON.parse(await fs.readFile(await (await file).path(), 'utf8'));
        };
        const names = ['adapter request', 'device request', 'renderer setup', 'first frame complete'];
        for (const scenario of ['native', 'hold', 'exit', 'loss', 'replacement']) {
            const url = new URL(original.url()); url.searchParams.set('frameTiming', scenario);
            await page.goto(url.href);
            if (scenario === 'native') {
                await ready(); const report = await typed();
                assert.deepEqual(report.phases.map(phase => phase.name), names);
                for (const phase of report.phases) {
                    assert(Number.isFinite(phase.durationMs) && phase.durationMs >= 0);
                    assert(phase.elapsedMs >= phase.durationMs); assert.equal(phase.bytes, 0); assert.equal(phase.source, 'GPU');
                }
                const probe = await page.evaluate(() => window.__phase);
                assert.equal(probe.adapters, 1); assert.equal(probe.devices, 1); assert.equal(probe.queues, 1);
                assert(report.phases.at(-1).elapsedMs >= probe.nativeDone, 'The first-frame measurement follows actual queue completion');
                const before = report.phases;
                for (let i = 0; i < 30; i++) await page.evaluate(i => window.wasmBindings.observe('activity', 'camera ' + i), i);
                await helpers.command(page, 'Orbit Right');
                assert.deepEqual((await typed()).phases, before, 'Phases outlive their event messages and are measured once');
                assert.equal((await snapshot()).events.length, 24);
            } else {
                await page.waitForFunction(() => window.__phase?.held && window.wasmBindings?.runtime_running?.());
                const waiting = await snapshot(); assert.equal(waiting.outcome, 'running');
                assert.deepEqual(waiting.phases.map(phase => phase.name), names.slice(0, 3));
                await page.waitForTimeout(220);
                assert.equal((await snapshot()).outcome, 'running', 'Submitted presentation does not claim completed GPU work');
                if (scenario === 'hold') {
                    const pending = await typed(); assert.equal(pending.outcome, 'running');
                    await page.evaluate(() => window.__phase.release()); await ready();
                    const complete = await typed(), probe = await page.evaluate(() => window.__phase);
                    assert.deepEqual(complete.phases.map(phase => phase.name), names);
                    assert(complete.phases.at(-1).durationMs >= 200);
                    assert(complete.phases.at(-1).elapsedMs >= probe.releaseAt);
                } else if (scenario === 'replacement') {
                    const previous = waiting.started;
                    await page.evaluate(() => window.wasmBindings.start());
                    await page.waitForFunction(started => JSON.parse(window.wasmBindings.diagnostic_snapshot()).started !== started
                        && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready', previous);
                    const replacement = await snapshot();
                    assert.deepEqual(replacement.phases.map(phase => phase.name), names);
                    await page.evaluate(() => window.__phase.release()); await page.waitForTimeout(150);
                    assert.deepEqual((await snapshot()).phases, replacement.phases, 'An older device cannot append a completion to its replacement');
                    assert.equal((await snapshot()).started, replacement.started);
                } else {
                    if (scenario === 'exit') {
                        await page.evaluate(() => window.dispatchEvent(new PageTransitionEvent('pagehide', {persisted: false})));
                        await page.waitForFunction(() => !window.wasmBindings.runtime_running() && !window.wasmBindings.report_lifecycle_running());
                    } else {
                        const file = page.waitForEvent('download'); await page.evaluate(() => window.__life.devices[0].destroy());
                        const failed = JSON.parse(await fs.readFile(await (await file).path(), 'utf8'));
                        assert.deepEqual(failed.phases.map(phase => phase.name), names.slice(0, 3));
                        await page.waitForFunction(() => !window.wasmBindings.runtime_running());
                    }
                    const settled = await snapshot(); assert.equal(settled.outcome, scenario === 'exit' ? 'closed' : 'failed');
                    await page.evaluate(() => window.__phase.release()); await page.waitForTimeout(150);
                    assert.deepEqual((await snapshot()).phases, settled.phases, 'Stale completion cannot add a phase');
                    assert.equal((await snapshot()).outcome, settled.outcome); assert.deepEqual((await snapshot()).failure, settled.failure);
                }
            }
            console.log(`${helpers.step.id}: actual GPU phase timing ${scenario} passed`);
        }
        assert.deepEqual(errors, []);
    } finally { await context.close(); await original.bringToFront(); }
};
