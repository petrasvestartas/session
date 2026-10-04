const assert = require('node:assert/strict');
const fs = require('node:fs/promises');

module.exports = async (original, helpers) => {
    // Intentional failures belong to this isolated page; the normal capture keeps its no-error assertion.
    const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, acceptDownloads: true});
    try {
        const page = await context.newPage(), errors = [], downloads = [];
        page.on('pageerror', error => errors.push(error.message));
        page.on('download', file => downloads.push(file));
        await require('./lifecycle-probe.cjs')(page);
        await page.addInitScript(() => {
            const add = EventTarget.prototype.addEventListener;
            window.__errorDelta = [];
            EventTarget.prototype.addEventListener = function (name, callback, options) {
                if (this === window && (name === 'error' || name === 'unhandledrejection')) {
                    const wrapped = event => {
                        const before = window.__life.queueCalls;
                        callback(event);
                        window.__errorDelta.push(window.__life.queueCalls - before);
                    };
                    // Preserve callback identity for owned removal through the probe.
                    window.__wrappedErrors ??= new Map(); window.__wrappedErrors.set(callback, window.__wrappedErrors.get(callback) || new Map());
                    window.__wrappedErrors.get(callback).set(name, wrapped);
                    return add.call(this, name, wrapped, options);
                }
                return add.call(this, name, callback, options);
            };
            const remove = EventTarget.prototype.removeEventListener;
            EventTarget.prototype.removeEventListener = function (name, callback, options) {
                return remove.call(this, name, window.__wrappedErrors?.get(callback)?.get(name) || callback, options);
            };
        });
        const ready = () => page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && document.querySelector('canvas')?.hasAttribute('data-command-ui') && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
        const snapshot = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
        const state = () => page.evaluate(() => {
            const canvas = document.querySelector('canvas');
            return {attributes: Object.fromEntries(['data-row-placements', 'data-selected-id', 'data-camera-matrix', 'data-gpu-stats']
                .map(name => [name, canvas.getAttribute(name)])), history: JSON.parse(canvas.getAttribute('data-command-ui')).history};
        });
        const readDownload = async file => JSON.parse(await fs.readFile(await file.path(), 'utf8'));
        for (const first of ['error', 'rejection']) {
            await page.goto(original.url()); await ready(); await helpers.command(page, 'View Isometric');
            const before = await state(), automatic = page.waitForEvent('download');
            if (first === 'error') await page.evaluate(() => { setTimeout(() => { throw new Error('actual uncaught lesson failure'); }, 0); });
            else await page.evaluate(() => { setTimeout(() => { Promise.reject(new Error('actual rejected lesson failure')); }, 0); });
            const report = await readDownload(await automatic);
            assert.equal(report.outcome, 'failed');
            assert(report.failure.message.startsWith(first === 'error' ? 'Browser error: ' : 'Unhandled rejection: '), JSON.stringify(report.failure));
            assert(report.failure.message.includes(first === 'error' ? 'actual uncaught lesson failure' : 'actual rejected lesson failure'), JSON.stringify(report.failure));
            assert.deepEqual(await state(), before);
            assert(await page.evaluate(() => window.wasmBindings.runtime_running()));
            // This wrapper observes GPU work only; it does not cancel either browser event's default.
            assert((await page.evaluate(() => window.__errorDelta)).every(delta => delta === 0));
            const count = downloads.length;
            await page.evaluate(() => {
                window.__coercions = 0;
                const cyclic = {}; cyclic.self = cyclic;
                Object.defineProperty(cyclic, 'toString', {get() { window.__coercions++; throw new Error('must not coerce'); }});
                const broken = new Error('unreadable'); Object.defineProperty(broken, 'message', {get() { throw new Error('getter denied'); }});
                for (const reason of ['  readable  ', null, undefined, true, 17, cyclic, broken]) {
                    window.dispatchEvent(new PromiseRejectionEvent('unhandledrejection', {promise: Promise.resolve(), reason}));
                }
                window.dispatchEvent(new ErrorEvent('error', {message: '😀'.repeat(5000), filename: 'https://example.org/' + 'é'.repeat(2000) + '?token=private#fragment', lineno: 42, colno: 7}));
            });
            let current = await snapshot();
            for (const detail of ['readable', 'null', 'undefined', 'true', '17', 'Non-text rejection reason', 'Error message unavailable']) {
                assert(current.events.some(event => event.message === 'Unhandled rejection: ' + detail), detail);
            }
            const bounded = current.events.at(-1).message;
            assert([...bounded].length < 4096); assert(bounded.endsWith(':42:7')); assert(!bounded.includes('token=private'));
            assert.equal(await page.evaluate(() => window.__coercions), 0);
            await page.evaluate(() => {
                for (let i = 0; i < 35; i++) window.dispatchEvent(new ErrorEvent('error', {message: 'later failure ' + i}));
                window.dispatchEvent(new Event('error'));
            });
            current = await snapshot(); assert.equal(current.events.length, 24); assert.deepEqual(current.failure, report.failure);
            assert.equal(current.events.at(-1).message, 'Browser error: later failure 34');
            assert.equal(downloads.length, count, 'Later failures do not trigger another automatic download');
            assert.deepEqual(await state(), before);
            const manual = page.waitForEvent('download'); await helpers.command(page, 'Diagnostic Report');
            assert.deepEqual((await readDownload(await manual)).failure, report.failure);
            await helpers.command(page, 'Orbit Right');
            assert.notDeepEqual((await state()).attributes['data-camera-matrix'], before.attributes['data-camera-matrix']);
            const downloadsBeforeLoss = downloads.length;
            await page.evaluate(() => window.__life.devices[0].destroy());
            await page.waitForFunction(() => !window.wasmBindings.runtime_running());
            await page.evaluate(() => window.dispatchEvent(new ErrorEvent('error', {message: 'after GPU loss'})));
            current = await snapshot(); assert.equal(current.events.at(-1).message, 'Browser error: after GPU loss');
            assert.deepEqual(current.failure, report.failure); assert.equal(downloads.length, downloadsBeforeLoss);
            assert(await page.evaluate(() => window.wasmBindings.report_lifecycle_running()));
            await page.evaluate(() => window.dispatchEvent(new PageTransitionEvent('pagehide', {persisted: false})));
            await page.waitForFunction(() => !window.wasmBindings.report_lifecycle_running());
            const stopped = await snapshot();
            await page.evaluate(() => window.dispatchEvent(new ErrorEvent('error', {message: 'after cleanup'})));
            assert.deepEqual(await snapshot(), stopped);
        }
        assert.deepEqual(errors, ['actual uncaught lesson failure', 'actual rejected lesson failure']);
        console.log(`${helpers.step.id}: real browser errors/rejections, automatic/manual downloads, bounded safe reasons, first failure, healthy camera and post-loss ownership passed`);
    } finally { await context.close(); await original.bringToFront(); }
};
