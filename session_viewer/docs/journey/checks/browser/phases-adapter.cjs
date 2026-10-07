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
            const scenario = new URL(location.href).searchParams.get('identity') || 'native';
            const gpu = navigator.gpu, request = GPU.prototype.requestAdapter;
            const probe = window.__adapter = {calls: 0, original: null, replacement: null, expected: null, native: null};
            const strings = info => Object.fromEntries(['vendor', 'architecture', 'device', 'description'].map(name => [name, info[name]]));
            const observed = async function (...args) {
                probe.calls++;
                const adapter = await Reflect.apply(request, this, args);
                if (!adapter) return adapter;
                const native = adapter.info; probe.native = strings(native); probe.expected = {...probe.native};
                if (scenario === 'long') {
                    const info = {vendor: '😀'.repeat(10000), architecture: 'é'.repeat(300), device: 'model', description: 'GPU'};
                    Object.defineProperty(adapter, 'info', {value: info});
                    probe.expected = {vendor: '😀'.repeat(256), architecture: 'é'.repeat(256), device: 'model', description: 'GPU'};
                } else if (scenario === 'empty') {
                    const info = {vendor: '', architecture: '', device: '', description: ''};
                    Object.defineProperty(adapter, 'info', {value: info}); probe.expected = info;
                } else if (scenario === 'unavailable') {
                    let first = true;
                    Object.defineProperty(adapter, 'info', {get() { if (first) { first = false; throw new Error('Identity denied'); } return native; }});
                    probe.expected = null;
                } else if (scenario === 'replacement') {
                    const captured = gpu.requestAdapter;
                    probe.replacement = function (...args) { return Reflect.apply(captured, this, args); };
                    Object.defineProperty(gpu, 'requestAdapter', {value: probe.replacement, configurable: true, writable: true});
                }
                return adapter;
            };
            GPU.prototype.requestAdapter = observed;
            if (scenario === 'own') Object.defineProperty(gpu, 'requestAdapter', {value: observed, enumerable: true, writable: true, configurable: true});
            if (scenario === 'denied') Object.defineProperty(gpu, 'requestAdapter', {value: observed, enumerable: true, writable: false, configurable: false});
            probe.original = Object.getOwnPropertyDescriptor(gpu, 'requestAdapter');
            probe.restored = () => {
                const current = Object.getOwnPropertyDescriptor(gpu, 'requestAdapter');
                if (!probe.original) return current === undefined;
                return current?.value === probe.original.value && current?.enumerable === probe.original.enumerable
                    && current?.writable === probe.original.writable && current?.configurable === probe.original.configurable;
            };
        });
        const ready = () => page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && document.querySelector('canvas')?.hasAttribute('data-command-ui') && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
        const snapshot = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
        const download = async command => {
            const file = page.waitForEvent('download'); await helpers.command(page, command);
            return JSON.parse(await fs.readFile(await (await file).path(), 'utf8'));
        };
        for (const scenario of ['native', 'own', 'long', 'empty', 'unavailable', 'denied', 'replacement']) {
            const url = new URL(original.url()); url.searchParams.set('identity', scenario);
            await page.goto(url.href); await ready();
            const expected = await page.evaluate(() => window.__adapter.expected), report = await snapshot();
            assert.equal(await page.evaluate(() => window.__adapter.calls), 1, 'The viewer requests one actual adapter');
            assert.equal(report.outcome, 'ready'); assert.equal(report.failure, null);
            if (scenario === 'unavailable' || scenario === 'denied') {
                assert.equal(report.adapter, undefined);
                assert(report.events.some(event => event.message === 'Adapter identity unavailable'));
            } else assert.deepEqual(report.adapter, expected);
            if (scenario === 'replacement') {
                assert(await page.evaluate(() => navigator.gpu.requestAdapter === window.__adapter.replacement));
            } else assert(await page.evaluate(() => window.__adapter.restored()), 'Restore the exact original property descriptor');
            if (scenario === 'native') {
                const actualDevice = await page.evaluate(() => {
                    const info = window.__life.devices[0].adapterInfo;
                    return Object.fromEntries(['vendor', 'architecture', 'device', 'description'].map(name => [name, info[name]]));
                });
                assert.deepEqual(report.adapter, actualDevice, 'Identity belongs to the actual drawing device');
            }
            const typed = await download('Report');
            assert.deepEqual(typed.adapter, report.adapter);
            const camera = await page.locator('canvas').getAttribute('data-camera-matrix');
            await helpers.command(page, 'Orbit Right');
            assert.notEqual(await page.locator('canvas').getAttribute('data-camera-matrix'), camera);
            for (let i = 0; i < 30; i++) await page.evaluate(i => window.wasmBindings.observe('phase', 'activity ' + i), i);
            assert.deepEqual((await snapshot()).adapter, report.adapter);
            assert.equal((await snapshot()).events.length, 24);
            if (scenario === 'replacement') {
                await page.evaluate(async () => { await navigator.gpu.requestAdapter(); });
                assert.equal(await page.evaluate(() => window.__adapter.calls), 2);
                assert.deepEqual((await snapshot()).adapter, report.adapter, 'A released observer cannot replace the report identity');
            }
            if (scenario === 'native') {
                const automatic = page.waitForEvent('download'); await page.evaluate(() => window.__life.devices[0].destroy());
                const failed = JSON.parse(await fs.readFile(await (await automatic).path(), 'utf8'));
                await page.waitForFunction(() => !window.wasmBindings.runtime_running());
                assert.deepEqual(failed.adapter, report.adapter);
                assert.deepEqual((await snapshot()).adapter, report.adapter);
                await page.reload(); await ready();
                assert.deepEqual((await download('Report Previous')).adapter, report.adapter, 'Previous-run decoding retains identity');
            }
            console.log(`${helpers.step.id}: adapter identity ${scenario} passed`);
        }
        assert.deepEqual(errors, []);
    } finally { await context.close(); await original.bringToFront(); }
};
