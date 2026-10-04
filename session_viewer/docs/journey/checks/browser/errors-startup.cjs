const assert = require('node:assert/strict');
module.exports = async (original, helpers) => {
    const context = await require('./raw-lifecycle.cjs')(original);
    try {
        const page = await context.page();
        await page.addInitScript(() => {
            if (typeof GPU === 'undefined') return;
            const params = new URL(location.href).searchParams;
            if (params.get('denyMetadata') === '1') sessionStorage.setItem('__denyReportLifecycle', '1');
            const stage = params.get('hold'), reject = params.get('reject') === '1';
            const probe = window.__pending = {held: false, released: false, adapterCalls: 0, deviceCalls: 0,
                created: [], destroyed: [], resources: [], bindings: [], devices: []};
            let pagehideRegistrations = 0;
            const add = EventTarget.prototype.addEventListener, remove = EventTarget.prototype.removeEventListener;
            EventTarget.prototype.addEventListener = function (name, callback, options) {
                if (this === window && name === 'pagehide' && ++pagehideRegistrations === 2
                    && params.get('denyStartup') === '1') throw new DOMException('Denied startup binding', 'SecurityError');
                const result = add.call(this, name, callback, options);
                if (this === window && name === 'pagehide') probe.bindings.push({callback, removed: 0});
                return result;
            };
            EventTarget.prototype.removeEventListener = function (name, callback, options) {
                const row = probe.bindings.find(row => row.callback === callback && !row.removed);
                if (this === window && name === 'pagehide' && row) row.removed++;
                return remove.call(this, name, callback, options);
            };
            const hold = async (name, request) => {
                let value, error;
                try { value = await request(); } catch (caught) { error = caught; }
                if (stage === name) {
                    probe.held = true;
                    await new Promise(resolve => { probe.release = () => { probe.released = true; resolve(); }; });
                }
                if (error) throw error;
                return value;
            };
            const adapter = GPU.prototype.requestAdapter;
            GPU.prototype.requestAdapter = function (...args) {
                probe.adapterCalls++;
                const options = stage === 'adapter' && reject ? {...args[0], powerPreference: 'invalid'} : args[0];
                return hold('adapter', () => adapter.call(this, options));
            };
            const device = GPUAdapter.prototype.requestDevice;
            GPUAdapter.prototype.requestDevice = function (...args) {
                probe.deviceCalls++;
                const options = stage === 'device' && reject ? {...args[0], requiredLimits: {maxBindGroups: 4294967295}} : args[0];
                return hold('device', async () => {
                    const result = await device.call(this, options); probe.devices.push(result); probe.created.push(result.label); return result;
                });
            };
            const destroy = GPUDevice.prototype.destroy;
            GPUDevice.prototype.destroy = function (...args) { probe.destroyed.push(this.label); return destroy.apply(this, args); };
            for (const [prototype, names] of [
                [GPUDevice.prototype, ['createBuffer', 'createTexture', 'createShaderModule', 'createRenderPipeline', 'createBindGroup', 'createCommandEncoder']],
                [GPUCanvasContext.prototype, ['configure', 'getCurrentTexture']],
                [GPUQueue.prototype, ['writeBuffer', 'writeTexture', 'submit']],
            ]) for (const name of names) {
                const method = prototype[name];
                prototype[name] = function (...args) { probe.resources.push(name); return method.apply(this, args); };
            }
        });
        await require('./lifecycle-probe.cjs')(page);
        const wait = async fn => {
            const until = Date.now() + 20000;
            while (Date.now() < until) {
                if (await page.evaluate(fn)) return;
                await new Promise(resolve => setTimeout(resolve, 50));
            }
            throw Error(`Startup condition timed out: ${fn}`);
        };
        const snapshot = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
        const proof = () => page.evaluate(() => ({held: window.__pending.held, released: window.__pending.released,
            adapterCalls: window.__pending.adapterCalls, deviceCalls: window.__pending.deviceCalls,
            created: [...window.__pending.created], destroyed: [...window.__pending.destroyed], resources: [...window.__pending.resources],
            pagehide: window.__pending.bindings.map(row => row.removed), lifetime: window.__life.summary(),
            runtime: window.wasmBindings.runtime_running(), reportOwner: window.wasmBindings.report_lifecycle_running()}));
        const cases = [
            ['adapter', 'final', false, false], ['device', 'final', false, false],
            ['adapter', 'final', true, false], ['device', 'final', true, false],
            ['adapter', 'cached', false, false], ['device', 'cached', false, false],
            ['adapter', 'final', false, true], ['device', 'final', false, true],
            ['adapter', 'active', true, false], ['device', 'active', true, false],
        ];
        for (const [stage, transition, reject, deny] of cases) {
            const url = new URL(original.url()); url.searchParams.set('hold', stage);
            url.searchParams.set('reject', reject ? '1' : '0'); url.searchParams.set('denyMetadata', deny ? '1' : '0');
            await page.activate(); await page.send('Page.navigate', {url: url.href});
            await wait(() => window.wasmBindings?.diagnostic_snapshot && window.__pending?.held);
            const before = await snapshot(); assert.equal(before.outcome, 'running');
            let observed = await proof(); assert.equal(observed.runtime, false); assert.deepEqual(observed.resources, []);
            assert.equal(observed.adapterCalls, 1); assert.equal(observed.deviceCalls, stage === 'device' ? 1 : 0);
            assert.equal(observed.reportOwner, !deny);
            // Use the scenario's explicit persisted value, independently of cache eligibility.
            if (transition !== 'active') await page.send('Runtime.evaluate', {expression:
                `window.dispatchEvent(new PageTransitionEvent('pagehide',{persisted:${transition === 'cached'}}))`});
            if (transition === 'final') {
                await wait(() => !window.wasmBindings.report_lifecycle_running());
                const closed = await snapshot(); assert.equal(closed.outcome, 'closed'); assert.equal(closed.failure, null);
                await page.evaluate(() => window.__pending.release());
                await wait(() => window.__pending.bindings.every(row => row.removed === 1)
                    || window.wasmBindings.runtime_running() || window.__pending.resources.length > 0);
                observed = await proof(); assert.equal(observed.runtime, false); assert.equal(observed.reportOwner, false);
                assert.deepEqual(observed.resources, []); assert.deepEqual(observed.lifetime.lostCalls, []);
                assert(observed.lifetime.intervals.every(row => row.cleared === 1));
                assert.equal(observed.deviceCalls, stage === 'device' ? 1 : 0);
                assert.equal(observed.created.length, stage === 'device' && !reject ? 1 : 0);
                assert.equal(observed.destroyed.length, stage === 'device' && !reject ? 1 : 0);
                assert.deepEqual(await snapshot(), closed, 'Late success or rejection cannot overwrite final closure');
                await page.evaluate(() => window.dispatchEvent(new PageTransitionEvent('pageshow', {persisted: true})));
                assert.deepEqual(await snapshot(), closed); assert.equal((await proof()).runtime, false);
            } else if (transition === 'cached') {
                assert.equal((await snapshot()).outcome, 'running');
                await page.evaluate(() => {
                    window.dispatchEvent(new PageTransitionEvent('pageshow', {persisted: true}));
                    window.__pending.release();
                });
                await wait(() => window.wasmBindings.runtime_running() && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
                observed = await proof(); assert.equal(observed.created.length, 1); assert.equal(observed.destroyed.length, 0);
                assert.equal(observed.pagehide.filter(count => count === 1).length, 1, 'Temporary guard detaches after successful startup');
                assert.equal(observed.lifetime.activeMetadata, 7); assert.equal(observed.lifetime.removedRuntime, 1);
                assert(observed.resources.includes('configure')); assert(observed.resources.includes('submit'));
                assert.equal((await snapshot()).failure, null);
            } else {
                await page.evaluate(() => window.__pending.release());
                await wait(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'failed');
                observed = await proof(); assert.equal(observed.runtime, false); assert.deepEqual(observed.resources, []);
                assert.equal(observed.created.length, 0); assert.equal(observed.destroyed.length, 0);
                assert((await snapshot()).failure, 'An active request rejection remains a genuine startup failure');
            }
            console.log(`${helpers.step.id}: delayed ${stage} ${transition}${reject ? ' rejection' : ''}${deny ? ' with unavailable metadata' : ''} passed`);
            await page.send('Page.navigate', {url: 'about:blank'});
        }
        const denied = new URL(original.url()); denied.searchParams.set('denyStartup', '1');
        await page.activate(); await page.send('Page.navigate', {url: denied.href});
        await wait(() => window.wasmBindings?.diagnostic_snapshot
            && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'failed');
        const failed = await proof(); assert.equal(failed.adapterCalls, 0); assert.equal(failed.deviceCalls, 0);
        assert.equal(failed.runtime, false); assert.deepEqual(failed.resources, []);
        assert.equal(failed.lifetime.activeMetadata, 7);
        assert((await snapshot()).failure.message.includes('Denied startup binding'));
        assert.deepEqual(context.errors, []);
        console.log(`${helpers.step.id}: denied mandatory startup binding refuses GPU requests and preserves failure diagnostics`);
    } finally { await context.close(); }
};
