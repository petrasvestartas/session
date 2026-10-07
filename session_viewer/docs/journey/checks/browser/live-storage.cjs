const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
module.exports = async (page, helpers) => {
    await require('./storage-writer.cjs')(page, {...helpers, stableTab: true});
    const {command, step} = helpers, prefix = 'viewer-journey-report:';
    const ready = () => page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && document.querySelector('canvas')?.hasAttribute('data-command-ui'));
    const snapshot = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
    const previous = () => page.evaluate(() => { try { return JSON.parse(window.wasmBindings.previous_diagnostic_snapshot()); } catch { return null; } });
    const ui = async () => JSON.parse(await page.locator('canvas').getAttribute('data-command-ui'));
    const parse = async file => JSON.parse(await fs.readFile(await file.path(), 'utf8'));
    const rows = () => page.evaluate(prefix => Object.keys(localStorage).filter(key => key.startsWith(prefix)).map(key =>
        ({key, report: JSON.parse(localStorage.getItem(key))})), prefix);
    await page.reload(); await ready();
    assert.equal(await previous(), null);
    let downloads = 0; const count = () => downloads++; page.on('download', count);
    await command(page, 'Report Previous');
    assert.match((await ui()).status, /No previous report/); assert.equal(downloads, 0); page.off('download', count);
    for (const outcome of ['ready', 'running']) {
        await page.evaluate(({prefix, outcome}) => {
            const source = JSON.parse(window.wasmBindings.diagnostic_snapshot());
            for (const key of Object.keys(localStorage)) if (key.startsWith(prefix)) localStorage.removeItem(key);
            localStorage.setItem(prefix + 'quiet', JSON.stringify({...source, tab: 'active-other-tab', outcome,
                started: new Date(Date.now() - 300000).toISOString(), lastSeen: new Date(Date.now() - 30000).toISOString()}));
        }, {prefix, outcome});
        await page.reload(); await ready(); assert.equal(await previous(), null);
        assert(!/Previous viewer/.test((await ui()).status));
    }
    const current = await snapshot();
    const lossDownload = page.waitForEvent('download', {predicate:file => file.suggestedFilename() === 'viewer-diagnostic.json'});
    await page.evaluate(() => window.__lossProbe.devices[0].destroy());
    const failed = await parse(await lossDownload);
    await page.waitForFunction(() => window.wasmBindings.runtime_running() === false);
    assert.deepEqual(await page.evaluate(() => window.__lossProbe.calls), []);
    const lostRows = await rows(), storedFailure = lostRows.find(row => row.report.started === failed.started);
    assert(storedFailure); assert.deepEqual(storedFailure.report, failed); assert(lostRows.length <= 3);
    await page.reload(); await ready();
    const healthy = await snapshot(); assert.equal(healthy.tab, current.tab); assert.notEqual(healthy.started, failed.started);
    assert.equal(healthy.outcome, 'ready'); assert.equal(healthy.failure, null);
    assert.deepEqual(await previous(), failed); assert.match((await ui()).status, /Previous viewer run failed/);
    const freshRows = await rows(); assert(freshRows.length <= 3);
    const storedCurrent = freshRows.find(row => row.report.started === healthy.started);
    assert(storedCurrent); assert.notEqual(storedCurrent.key, storedFailure.key);
    const choose = async () => {
        const picker = page.waitForEvent('filechooser'); await command(page, 'Open Replace');
        await (await picker).setFiles(path.resolve('target', `course-${step.id}`, 'sample.pb'));
        await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Document replaced. Undo restores the previous document.');
    };
    await choose(); await command(page, 'Select Next');
    const state = async () => Object.fromEntries(await Promise.all(['data-row-placements', 'data-selected-id', 'data-camera-matrix', 'data-gpu-stats']
        .map(async name => [name, await page.locator('canvas').getAttribute(name)])));
    const original = await state(); await command(page, 'Move 0.12,0,0'); const moved = await state();
    await page.evaluate(() => {
        window.__previousURLs = []; window.__previousRevoked = [];
        const create = URL.createObjectURL, revoke = URL.revokeObjectURL;
        URL.createObjectURL = function (blob) { const url = create.call(URL, blob); window.__previousURLs.push(url); return url; };
        URL.revokeObjectURL = function (url) { window.__previousRevoked.push(url); return revoke.call(URL, url); };
    });
    const file = page.waitForEvent('download', {predicate:file => file.suggestedFilename() === 'viewer-diagnostic-previous.json'});
    await command(page, 'Report Previous'); assert.deepEqual(await parse(await file), failed);
    assert.deepEqual(await state(), moved); await command(page, 'Undo');
    assert.equal((await state())['data-row-placements'], original['data-row-placements']);
    await command(page, 'Redo'); assert.equal((await state())['data-row-placements'], moved['data-row-placements']);
    const url = await page.evaluate(() => window.__previousURLs[0]); assert(url);
    await page.waitForFunction(url => window.__previousRevoked.filter(value => value === url).length === 1, url, {timeout:13000});
    await page.addInitScript(() => {
        if (sessionStorage.getItem('__denyDiagnosticStorage') === '1') {
            sessionStorage.removeItem('__denyDiagnosticStorage');
            Object.defineProperty(window, 'localStorage', {configurable: true, get: () => { throw new DOMException('Denied', 'SecurityError'); }});
        }
    });
    await page.evaluate(() => sessionStorage.setItem('__denyDiagnosticStorage', '1'));
    await page.reload(); await ready(); assert.equal(await previous(), null);
    const deniedFile = page.waitForEvent('download', {predicate:file => file.suggestedFilename() === 'viewer-diagnostic.json'});
    await command(page, 'Report'); const denied = await parse(await deniedFile);
    assert.equal(denied.outcome, 'ready'); assert.equal(denied.failure, null);
    await page.reload(); await ready(); await choose();
    for (const line of ['Select Next', 'Move 0.35,0,0.25', 'Move 0.25,0,0.15', 'Move 0,-0.5,0', 'View Isometric',
        'Orbit Right', 'Move 0.1,0,0', 'Orbit Up', 'Orbit Right', 'Orbit Up', 'Orbit Right', 'Move 0.05,0,0',
        'Move 0,-0.05,0', 'Move 0.05,0,0', 'Move 0,-0.05,0', 'Move 0.05,0,0', 'Move 0,-0.05,0',
        'Move 0.05,0,0', 'Move 0,-0.05,0', 'Move 0.05,0,0', 'Fit']) await command(page, line);
    console.log(`${step.id}: real loss persisted, same-page recovery and typed previous download; bounded metadata, scene/history, URL release and denied-storage current download passed`);
};
