const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
module.exports = async (page, helpers) => {
    await require('./phases-browser-errors.cjs')(page, helpers);
    await require('./errors-startup.cjs')(page, helpers);
    await require('./lifecycle-probe.cjs')(page);
    await page.addInitScript(() => {
        const deny = sessionStorage.getItem('__denySuspensionBinding') === '1';
        const denyErrors = sessionStorage.getItem('__denyErrorBinding') === '1';
        sessionStorage.removeItem('__denyErrorBinding');
        sessionStorage.removeItem('__denySuspensionBinding');
        const add = EventTarget.prototype.addEventListener;
        EventTarget.prototype.addEventListener = function (name, callback, options) {
            if (denyErrors && this === window && name === 'unhandledrejection') throw new DOMException('Denied error binding', 'SecurityError');
            if (deny && this === document && name === 'resume') throw new DOMException('Denied resume binding', 'SecurityError');
            return add.call(this, name, callback, options);
        };
    });
    const ready = () => page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && document.querySelector('canvas')?.hasAttribute('data-command-ui') && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
    const snapshot = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
    const probe = () => page.evaluate(() => window.__life.summary());
    const state = async () => Object.fromEntries(await Promise.all(['data-row-placements', 'data-selected-id', 'data-camera-matrix', 'data-gpu-stats', 'data-command-ui']
        .map(async name => [name, await page.locator('canvas').getAttribute(name)])));
    const stored = started => page.evaluate(started => Object.keys(localStorage).filter(key => key.startsWith('viewer-journey-report:'))
        .map(key => JSON.parse(localStorage.getItem(key))).find(report => report.started === started), started);
    const transition = (name, persisted) => page.evaluate(({name, persisted}) => window.dispatchEvent(new PageTransitionEvent(name, {persisted})), {name, persisted});
    const proof = () => require('./diagnostic-proof.cjs')(page, helpers, ['Move 0.05,0,0', 'Move 0,-0.05,0', 'Move 0.05,0,0', 'Move 0,-0.05,0']);
    await page.reload(); await ready(); await proof();
    await require('./errors-suspension.cjs')(page, helpers);
    await page.reload(); await ready(); await proof();
    assert.equal((await probe()).activeMetadata, 7);
    const report = await snapshot(), image = await helpers.drawing(page), editor = await state();
    await transition('pagehide', true);
    assert(await page.evaluate(() => window.wasmBindings.runtime_running() && window.wasmBindings.report_lifecycle_running()));
    let hidden = await snapshot(); assert.equal(hidden.outcome, 'ready'); assert.equal(hidden.failure, null);
    assert.equal(hidden.events.at(-1).message, 'pagehide cached'); assert.deepEqual(await stored(hidden.started), hidden);
    assert.equal((await probe()).removedMetadata, 0); assert.equal((await probe()).removedRuntime, 1);
    assert((await probe()).intervals.every(row => row.cleared === 1));
    await page.waitForTimeout(50); assert.deepEqual(await snapshot(), hidden);
    await transition('pageshow', true);
    assert.equal((await snapshot()).outcome, report.outcome); assert.equal((await snapshot()).events.at(-1).message, 'pageshow');
    assert.equal((await probe()).intervals.filter(row => !row.cleared).length, 1);
    await page.evaluate(() => { window.__life.enabled = true; }); await page.waitForFunction(() => window.__life.ticks >= 3);
    await page.evaluate(() => { window.__life.enabled = false; });
    assert((await probe()).deltas.every(delta => delta === 0));
    assert.equal(await helpers.drawing(page), image); assert.deepEqual(await state(), editor);

    // Metadata stays owned after actual device loss; only the GPU runtime stops.
    const file = page.waitForEvent('download', {predicate: file => file.suggestedFilename() === 'viewer-diagnostic.json'});
    await page.evaluate(() => window.__life.devices[0].destroy());
    const failed = JSON.parse(await fs.readFile(await (await file).path(), 'utf8'));
    await page.waitForFunction(() => !window.wasmBindings.runtime_running());
    assert.equal((await probe()).removedRuntime, 19); assert.equal((await probe()).activeMetadata, 7);
    await transition('pagehide', true); await transition('pageshow', true);
    let after = await snapshot(); assert.equal(after.outcome, 'failed'); assert.deepEqual(after.failure, failed.failure);
    assert.deepEqual(after.events.slice(0, failed.events.length), failed.events);
    await page.evaluate(() => document.dispatchEvent(new Event('freeze')));
    assert((await probe()).intervals.every(row => row.cleared === 1));
    await transition('pageshow', true);
    assert((await probe()).intervals.every(row => row.cleared === 1), 'Page return must not clear freeze after device loss');
    await page.evaluate(() => document.dispatchEvent(new Event('resume')));
    assert.equal((await probe()).intervals.filter(row => !row.cleared).length, 1);
    assert.deepEqual((await snapshot()).failure, failed.failure);
    const ticks = (await probe()).ticks;
    await page.evaluate(() => { window.__life.enabled = true; });
    await page.waitForFunction(count => window.__life.ticks >= count + 3, ticks);
    await transition('pagehide', false);
    await page.waitForFunction(() => !window.wasmBindings.report_lifecycle_running());
    after = await snapshot(); assert.equal(after.outcome, 'failed'); assert.deepEqual(after.failure, failed.failure);
    assert.deepEqual(await stored(after.started), after);
    let observed = await probe(); assert.equal(observed.removedMetadata, 7); assert.equal(observed.removedRuntime, 19);
    assert.equal(observed.activeMetadata, 0); assert(observed.intervals.every(row => row.cleared === 1));
    await page.evaluate(() => window.wasmBindings.start_periodic());
    assert.deepEqual((await probe()).intervals, observed.intervals, 'A stopped metadata owner cannot be resumed by a stray timer request');
    await transition('pageshow', true); await transition('pagehide', false); await page.waitForTimeout(60);
    assert.deepEqual(await snapshot(), after); assert.deepEqual((await probe()).lostCalls, []);

    // A healthy final exit is Closed and detaches each lifetime exactly once.
    await page.reload(); await ready(); const healthy = await snapshot();
    await transition('pagehide', false);
    await page.waitForFunction(() => !window.wasmBindings.runtime_running() && !window.wasmBindings.report_lifecycle_running());
    const closed = await snapshot(); assert.equal(closed.outcome, 'closed'); assert.equal(closed.failure, null);
    assert.equal(closed.started, healthy.started); assert.deepEqual(await stored(closed.started), closed);
    observed = await probe(); assert.equal(observed.removedMetadata, 7); assert.equal(observed.removedRuntime, 19);
    assert(observed.intervals.every(row => row.cleared === 1));
    await transition('pagehide', false); await transition('pageshow', true); assert.deepEqual(await snapshot(), closed);

    // Replacing metadata before an older deferred cleanup cannot stop the new owner.
    await page.reload(); await ready();
    await page.evaluate(() => {
        window.dispatchEvent(new PageTransitionEvent('pagehide', {persisted: false}));
        window.wasmBindings.install_report_lifecycle(); window.wasmBindings.start_periodic();
    });
    await page.waitForFunction(() => !window.wasmBindings.runtime_running()); await page.waitForTimeout(40);
    assert(await page.evaluate(() => window.wasmBindings.report_lifecycle_running()));
    observed = await probe(); assert.equal(observed.activeMetadata, 7); assert.equal(observed.removedMetadata, 7);
    assert.equal(observed.intervals.filter(row => !row.cleared).length, 1);
    await transition('pagehide', false); await page.waitForFunction(() => !window.wasmBindings.report_lifecycle_running());
    assert.equal((await probe()).removedMetadata, 14); assert((await probe()).intervals.every(row => row.cleared === 1));

    await page.evaluate(() => sessionStorage.setItem('__denyReportLifecycle', '1'));
    await page.reload(); await ready();
    const denied = await snapshot(); assert.equal(denied.outcome, 'ready');
    assert(denied.events.some(event => event.message.includes('Lifecycle unavailable')));
    assert.equal(await page.evaluate(() => window.wasmBindings.report_lifecycle_running()), false);
    assert.equal((await probe()).activeMetadata, 0); assert.equal((await probe()).removedMetadata, 1);
    assert.equal((await probe()).intervals.length, 0);
    const download = page.waitForEvent('download', {predicate: file => file.suggestedFilename() === 'viewer-diagnostic.json'});
    await helpers.command(page, 'Report');
    const usable = JSON.parse(await fs.readFile(await (await download).path(), 'utf8'));
    assert.equal(usable.outcome, 'ready'); assert.equal(usable.failure, null);
    await page.evaluate(() => sessionStorage.setItem('__denySuspensionBinding', '1'));
    await page.reload(); await ready();
    const partial = await snapshot(); assert.equal(partial.outcome, 'ready');
    assert(partial.events.some(event => event.message.includes('Lifecycle unavailable')));
    assert.equal(await page.evaluate(() => window.wasmBindings.report_lifecycle_running()), false);
    assert.equal((await probe()).activeMetadata, 0); assert.equal((await probe()).removedMetadata, 4);
    assert.equal((await probe()).intervals.length, 0);
    const current = page.waitForEvent('download', {predicate: file => file.suggestedFilename() === 'viewer-diagnostic.json'});
    await helpers.command(page, 'Report');
    assert.equal(JSON.parse(await fs.readFile(await (await current).path(), 'utf8')).outcome, 'ready');
    await page.evaluate(() => sessionStorage.setItem('__denyErrorBinding', '1'));
    await page.reload(); await ready();
    assert.equal((await probe()).activeMetadata, 0); assert.equal((await probe()).removedMetadata, 6);
    assert.equal((await probe()).intervals.length, 0);
    assert((await snapshot()).events.some(event => event.message.includes('Lifecycle unavailable')));
    const retained = page.waitForEvent('download'); await helpers.command(page, 'Report');
    assert.equal(JSON.parse(await fs.readFile(await (await retained).path(), 'utf8')).outcome, 'ready');
    await page.reload(); await ready(); await proof();
    console.log(`${helpers.step.id}: cached pause/resume, seven metadata/eighteen GPU bindings plus one released startup binding, real post-loss observations, healthy/failed final cleanup, matching-owner guard and registration-denied download passed`);
};
