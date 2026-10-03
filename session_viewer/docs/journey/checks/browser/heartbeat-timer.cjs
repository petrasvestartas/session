const assert = require('node:assert/strict');
const fs = require('node:fs/promises');

module.exports = async (page, helpers) => {
    // Observe the actual requested period; accelerate delivery only in this proof.
    // Gating inherited checks preserves their exact snapshot comparisons.
    await page.addInitScript(() => {
        const probe = window.__heartbeat = {enabled: false, calls: 0, gpu: 0, entries: [], clears: [], deltas: []};
        // The inherited Back test visits a page without the WebGPU interface.
        if (typeof GPUQueue === 'undefined') return;
        for (const method of ['writeBuffer', 'writeTexture', 'submit']) {
            const original = GPUQueue.prototype[method];
            GPUQueue.prototype[method] = function (...args) { probe.gpu++; return original.apply(this, args); };
        }
        const interval = window.setInterval.bind(window), clear = window.clearInterval.bind(window);
        const deny = sessionStorage.getItem('__denyHeartbeatScheduler') === '1';
        sessionStorage.removeItem('__denyHeartbeatScheduler');
        window.setInterval = function (callback, period, ...args) {
            if (period !== 15000) return interval(callback, period, ...args);
            if (deny) throw new DOMException('Denied heartbeat scheduling', 'SecurityError');
            const id = interval(() => {
                if (!probe.enabled) return;
                const before = probe.gpu;
                probe.calls++;
                callback(...args);
                probe.deltas.push(probe.gpu - before);
            }, 20);
            probe.entries.push({id, period}); return id;
        };
        window.clearInterval = function (id) { probe.clears.push(id); return clear(id); };
    });
    await require('./heartbeat-value.cjs')(page, helpers);
    const snapshot = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
    const probe = () => page.evaluate(() => window.__heartbeat);
    const ready = () => page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && document.querySelector('canvas')?.hasAttribute('data-command-ui'));
    const sameExceptHeartbeat = (before, after) => assert.deepEqual(after, {...before, lastSeen: after.lastSeen});
    let observed = await probe(); assert.equal(observed.entries.length, 1); assert.equal(observed.entries[0].period, 15000);
    const original = await snapshot();
    await page.evaluate(() => { window.__heartbeat.enabled = true; });
    await page.waitForFunction(() => window.__heartbeat.calls >= 3);
    const refreshed = await snapshot();
    assert(Date.parse(refreshed.lastSeen) > Date.parse(original.lastSeen)); sameExceptHeartbeat(original, refreshed);
    const stored = await page.evaluate(started => Object.keys(localStorage).filter(key => key.startsWith('viewer-journey-report:'))
        .map(key => JSON.parse(localStorage.getItem(key))).find(report => report.started === started), refreshed.started);
    sameExceptHeartbeat(refreshed, stored); assert(Date.parse(stored.lastSeen) >= Date.parse(refreshed.lastSeen));
    assert((await probe()).deltas.every(value => value === 0));

    assert.equal(await page.evaluate(() => window.wasmBindings.stop_periodic()), true);
    assert.equal(await page.evaluate(() => window.wasmBindings.stop_periodic()), false);
    observed = await probe(); const stoppedCount = observed.calls;
    assert.equal(observed.clears.filter(id => id === observed.entries[0].id).length, 1);
    const stopped = await snapshot(); await page.waitForTimeout(70);
    assert.equal((await probe()).calls, stoppedCount); assert.deepEqual(await snapshot(), stopped);
    await page.evaluate(() => window.wasmBindings.start_periodic());
    await page.evaluate(() => window.wasmBindings.start_periodic());
    observed = await probe(); assert.equal(observed.entries.length, 3);
    assert.equal(new Set(observed.entries.map(entry => entry.id)).size, 3);
    assert.equal(observed.clears.filter(id => id === observed.entries[1].id).length, 1);
    assert.equal(observed.clears.filter(id => id === observed.entries[2].id).length, 0);
    await page.waitForFunction(count => window.__heartbeat.calls >= count + 3, stoppedCount);

    const file = page.waitForEvent('download', {predicate: file => file.suggestedFilename() === 'viewer-diagnostic.json'});
    await page.evaluate(() => window.__lossProbe.devices[0].destroy());
    const failed = JSON.parse(await fs.readFile(await (await file).path(), 'utf8'));
    await page.waitForFunction(() => !window.wasmBindings.runtime_running());
    const lostCount = (await probe()).calls;
    await page.waitForFunction(count => window.__heartbeat.calls >= count + 3, lostCount);
    const after = await snapshot(); assert(Date.parse(after.lastSeen) > Date.parse(failed.lastSeen));
    sameExceptHeartbeat(failed, after); assert.equal(after.outcome, 'failed');
    assert.deepEqual(await page.evaluate(() => window.__lossProbe.calls), []);
    assert((await probe()).deltas.every(value => value === 0));
    assert.equal(await page.evaluate(() => window.wasmBindings.stop_periodic()), true);
    observed = await probe(); assert.equal(observed.clears.filter(id => id === observed.entries[2].id).length, 1);

    await page.evaluate(() => sessionStorage.setItem('__denyHeartbeatScheduler', '1'));
    await page.reload(); await ready();
    const denied = await snapshot(); assert.equal(denied.outcome, 'ready');
    assert(denied.events.some(event => event.message.includes('Heartbeat unavailable')));
    assert.equal((await probe()).entries.length, 0);
    const download = page.waitForEvent('download', {predicate: file => file.suggestedFilename() === 'viewer-diagnostic.json'});
    await helpers.command(page, 'Diagnostic Report');
    const usable = JSON.parse(await fs.readFile(await (await download).path(), 'utf8'));
    assert.equal(usable.outcome, 'ready'); assert.equal(usable.failure, null);
    await page.reload(); await ready();
    await require('./diagnostic-proof.cjs')(page, helpers, ['Move 0.05,0,0', 'Move 0,-0.05,0']);
    console.log(`${helpers.step.id}: actual 15-second registration, accelerated metadata delivery, zero callback GPU work, exact cancellation/replacement, preserved post-loss evidence and scheduler-denied current download passed`);
};
