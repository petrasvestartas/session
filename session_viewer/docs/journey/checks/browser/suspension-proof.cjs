const assert = require('node:assert/strict');

module.exports = async (original, helpers) => {
    const context = await require('./raw-lifecycle.cjs')(original);
    try {
        const page = await context.page(), cover = await context.page();
        await require('./lifecycle-probe.cjs')(page);
        const wait = async fn => {
            const until = Date.now() + 20000;
            while (Date.now() < until) {
                if (await page.evaluate(fn)) return;
                await new Promise(resolve => setTimeout(resolve, 50));
            }
            throw Error(`Real lifecycle condition timed out: ${fn}`);
        };
        const summary = () => page.evaluate(() => window.__life.summary());
        const report = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
        const active = async () => (await summary()).intervals.filter(row => !row.cleared);
        const state = () => page.evaluate(() => {
            const canvas = document.querySelector('canvas');
            return {attributes: Object.fromEntries(['data-row-placements', 'data-selected-id', 'data-camera-matrix', 'data-gpu-stats']
                .map(name => [name, canvas.getAttribute(name)])), history: JSON.parse(canvas.getAttribute('data-command-ui')).history};
        });
        const drawing = () => page.evaluate(() => document.querySelector('canvas').toDataURL());
        await page.activate(); await page.send('Page.navigate', {url: original.url()});
        await wait(() => window.wasmBindings?.runtime_running?.() && document.querySelector('canvas')?.hasAttribute('data-command-ui'));
        await wait(() => !document.hidden);
        assert.equal((await summary()).activeMetadata, 5); assert.equal((await active()).length, 1);
        const image = await drawing(), editor = await state();

        // Browser tab activation produces real visibility, with no hidden-property override.
        await cover.activate(); await wait(() => document.hidden);
        await wait(() => window.__life.summary().intervals.every(row => row.cleared === 1));
        assert.equal((await report()).events.at(-1).message, 'hidden');
        assert.deepEqual(await state(), editor);
        await page.activate(); await wait(() => !document.hidden);
        await wait(() => window.__life.summary().intervals.filter(row => !row.cleared).length === 1);
        assert.equal((await report()).events.at(-1).message, 'visible');
        assert.equal(await drawing(), image); assert.deepEqual(await state(), editor);

        // Chrome freezes the actual page; DevTools can inspect it while ordinary tasks are suspended.
        const before = (await active())[0].id;
        try {
            await page.send('Page.setWebLifecycleState', {state: 'frozen'});
            assert.equal((await report()).events.at(-1).message, 'freeze');
            assert.equal((await active()).length, 0);
        } finally { await page.send('Page.setWebLifecycleState', {state: 'active'}); }
        await wait(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()).events.some(event => event.message === 'resume'));
        assert.equal((await summary()).intervals.find(row => row.id === before).cleared, 1);
        const hiddenAfterResume = await page.evaluate(() => document.hidden);
        assert.equal((await active()).length, hiddenAfterResume ? 0 : 1, 'Resume preserves actual visibility');
        await cover.activate(); await wait(() => document.hidden);
        await page.activate(); await wait(() => !document.hidden);
        await wait(() => window.__life.summary().intervals.filter(row => !row.cleared).length === 1);
        const messages = (await report()).events.map(event => event.message);
        assert(messages.indexOf('freeze') < messages.indexOf('resume'));
        assert.equal((await report()).outcome, 'ready'); assert.equal((await report()).failure, null);
        assert.equal(await drawing(), image); assert.deepEqual(await state(), editor);

        // Inject order combinations separately from the browser-driven events above.
        await cover.activate(); await wait(() => document.hidden);
        await page.evaluate(() => {
            document.dispatchEvent(new Event('freeze'));
            window.dispatchEvent(new PageTransitionEvent('pagehide', {persisted: true}));
        });
        await page.activate(); await wait(() => !document.hidden);
        assert.equal((await active()).length, 0, 'Visible still has frozen and cached reasons');
        await page.evaluate(() => document.dispatchEvent(new Event('resume')));
        assert.equal((await active()).length, 0, 'Resume leaves the cached reason intact');
        await page.evaluate(() => document.dispatchEvent(new Event('visibilitychange')));
        assert.equal((await active()).length, 0);
        await page.evaluate(() => window.dispatchEvent(new PageTransitionEvent('pageshow', {persisted: true})));
        assert.equal((await active()).length, 1);
        const resumed = await summary();
        await page.evaluate(() => {
            window.dispatchEvent(new PageTransitionEvent('pageshow', {persisted: true}));
            document.dispatchEvent(new Event('resume')); document.dispatchEvent(new Event('resume'));
        });
        assert.deepEqual((await summary()).intervals, resumed.intervals, 'Duplicate resumes retain the same native interval');
        assert.equal(await drawing(), image); assert.deepEqual(await state(), editor);

        // Load the actual viewer while its tab is genuinely hidden.
        await cover.activate(); await wait(() => document.hidden);
        await page.send('Page.reload');
        await wait(() => window.wasmBindings?.runtime_running?.() && window.wasmBindings.report_lifecycle_running()
            && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
        assert(await page.evaluate(() => document.hidden));
        assert.equal((await summary()).activeMetadata, 5);
        assert.equal((await summary()).intervals.length, 0, 'Initially hidden startup must not install heartbeat');
        await page.activate(); await wait(() => !document.hidden);
        await wait(() => window.__life.summary().intervals.filter(row => !row.cleared).length === 1);
        assert.equal((await summary()).intervals.length, 1);
        await page.evaluate(() => { window.__life.enabled = true; });
        await wait(() => window.__life.ticks >= 3);
        await page.evaluate(() => { window.__life.enabled = false; });
        assert((await summary()).deltas.every(delta => delta === 0));
        assert.deepEqual((await summary()).lostCalls, []); assert.deepEqual(context.errors, []);
    } finally { await context.close(); }
    console.log(`${helpers.step.id}: genuine tab visibility, Chrome freeze/resume, overlap ordering, duplicate resumption and hidden startup passed`);
};
