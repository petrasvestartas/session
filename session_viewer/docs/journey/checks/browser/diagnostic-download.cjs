const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
module.exports = async (page, helpers) => {
    const auto = [], onDownload = file => { if (file.suggestedFilename() === 'viewer-diagnostic.json') auto.push(file); };
    page.on('download', onDownload);
    await require('./diagnostic-browser.cjs')(page, {...helpers, diagnosticOutcome:'ready'});
    const parse = async file => JSON.parse(await fs.readFile(await file.path(), 'utf8'));
    assert.equal(auto.length, 1, 'Inherited real GPU loss attempts exactly one report download');
    assert.equal((await parse(auto[0])).outcome, 'failed');
    const {command, step} = helpers;
    const data = async key => await page.locator('canvas').getAttribute(key);
    const state = async () => ({placement:await data('data-row-placements'), camera:await data('data-camera-matrix'), selected:await data('data-selected-id'), stats:await data('data-gpu-stats')});
    const original = await state(); await command(page, 'Move 0.12,0,0'); const moved = await state();
    await page.evaluate(() => {
        window.__reportURLs = []; window.__reportRevoked = [];
        const create = URL.createObjectURL, revoke = URL.revokeObjectURL;
        URL.createObjectURL = function (blob) { const url = create.call(URL, blob); window.__reportURLs.push(url); return url; };
        URL.revokeObjectURL = function (url) { window.__reportRevoked.push(url); return revoke.call(URL, url); };
    });
    const downloaded = page.waitForEvent('download', {predicate:file => file.suggestedFilename() === 'viewer-diagnostic.json'});
    await command(page, 'Diagnostic Report'); const readyReport = await parse(await downloaded);
    assert.equal(readyReport.outcome, 'ready'); assert.equal(readyReport.failure, null);
    assert(readyReport.events.some(event => event.kind === 'milestone' && event.message === 'geometry on screen'));
    assert.deepEqual(await state(), moved, 'Diagnostic command leaves placement, selection, camera and geometry GPU counters unchanged');
    await command(page, 'Undo'); assert.equal((await state()).placement, original.placement, 'Diagnostics do not consume scene Undo');
    await command(page, 'Redo'); assert.equal((await state()).placement, moved.placement);
    const reportURL = await page.evaluate(() => window.__reportURLs[0]); assert(reportURL);
    await page.waitForFunction(url => window.__reportRevoked.filter(value => value === url).length === 1, reportURL, {timeout:13000});
    const failedDownload = page.waitForEvent('download', {predicate:file => file.suggestedFilename() === 'viewer-diagnostic.json'});
    const previousCount = auto.length;
    await page.evaluate(() => window.__lossProbe.devices[0].destroy());
    const failedReport = await parse(await failedDownload);
    await page.waitForFunction(() => window.wasmBindings.runtime_running() === false && document.getElementById('status')?.textContent.includes('device lost'));
    assert.equal(failedReport.outcome, 'failed'); assert.match(failedReport.failure.message, /WebGPU device lost/);
    assert.equal(failedReport.page, readyReport.page); assert.equal(failedReport.started, readyReport.started);
    await page.keyboard.type('Diagnostic Report'); await page.keyboard.press('Enter'); await page.mouse.wheel(0, 200);
    await page.evaluate(() => { window.dispatchEvent(new Event('resize')); window.dispatchEvent(new Event('viewer-file')); });
    await page.waitForTimeout(150); assert.equal(auto.length, previousCount + 1);
    assert.deepEqual(await page.evaluate(() => window.__lossProbe.calls), []);
    const snapshot = await page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
    assert.deepEqual(snapshot.failure, failedReport.failure, 'Failure evidence remains after runtime disposal and late input');
    // Reject one startup adapter request, then permit a normal same-tab restart.
    await page.addInitScript(() => {
        if (sessionStorage.getItem('__reportRejectAdapter') === '1') {
            sessionStorage.removeItem('__reportRejectAdapter'); navigator.gpu.requestAdapter = async () => null;
        }
    });
    await page.evaluate(() => sessionStorage.setItem('__reportRejectAdapter', '1'));
    const startupDownload = page.waitForEvent('download', {predicate:file => file.suggestedFilename() === 'viewer-diagnostic.json'});
    await page.reload(); const startup = await parse(await startupDownload);
    assert.equal(startup.outcome, 'failed'); assert.match(startup.failure.message, /Cannot draw:/);
    assert.equal(await page.evaluate(() => window.wasmBindings.runtime_running()), false);
    assert.equal(await page.evaluate(() => window.__lossProbe.devices.length), 0);
    console.log(`${step.id}: actual ready, GPU-loss and startup-failure report downloads; unchanged scene/history and released download URL`);
    await page.reload(); await page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && document.querySelector('canvas')?.hasAttribute('data-command-ui'));
    const picker = page.waitForEvent('filechooser'); await command(page, 'Open Replace');
    await (await picker).setFiles(path.resolve('target', `course-${step.id}`, 'sample.pb'));
    await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Document replaced. Undo restores the previous document.');
    for (const line of ['Select Next', 'Move 0.35,0,0.25', 'Move 0.25,0,0.15', 'Move 0,-0.5,0', 'View Isometric',
        'Orbit Right', 'Move 0.1,0,0', 'Orbit Up', 'Orbit Right', 'Orbit Up', 'Orbit Right', 'Move 0.05,0,0', 'Move 0,-0.05,0', 'Fit']) await command(page, line);
    page.off('download', onDownload);
};
