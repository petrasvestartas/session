const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
module.exports = async (page, helpers) => {
    await require('./compatible-retention.cjs')(page, helpers);
    const snapshot = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
    const before = await snapshot(); const image = await helpers.drawing(page);
    const state = async () => Object.fromEntries(await Promise.all(['data-row-placements', 'data-selected-id', 'data-camera-matrix', 'data-gpu-stats']
        .map(async name => [name, await page.locator('canvas').getAttribute(name)])));
    const original = await state(); await page.waitForTimeout(20);
    await page.evaluate(() => window.wasmBindings.heartbeat());
    const beat = await snapshot(); const expected = {...before, lastSeen: beat.lastSeen};
    assert(Date.parse(beat.lastSeen) > Date.parse(before.lastSeen)); assert.deepEqual(beat, expected);
    assert.equal(await helpers.drawing(page), image); assert.deepEqual(await state(), original);
    const stored = await page.evaluate(started => Object.keys(localStorage).filter(key => key.startsWith('viewer-journey-report:'))
        .map(key => JSON.parse(localStorage.getItem(key))).find(report => report.started === started), beat.started);
    assert.deepEqual(stored, beat);
    const file = page.waitForEvent('download', {predicate:file => file.suggestedFilename() === 'viewer-diagnostic.json'});
    await page.evaluate(() => window.__lossProbe.devices[0].destroy());
    const failed = JSON.parse(await fs.readFile(await (await file).path(), 'utf8'));
    await page.waitForFunction(() => !window.wasmBindings.runtime_running()); await page.waitForTimeout(20);
    await page.evaluate(() => window.wasmBindings.heartbeat()); const after = await snapshot();
    assert(Date.parse(after.lastSeen) > Date.parse(failed.lastSeen));
    assert.deepEqual(after, {...failed, lastSeen: after.lastSeen});
    assert.deepEqual(await page.evaluate(() => window.__lossProbe.calls), []);
    await page.reload(); await page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && document.querySelector('canvas')?.hasAttribute('data-command-ui'));
    await require('./diagnostic-proof.cjs')(page, helpers, ['Move 0.05,0,0']);
    console.log(`${helpers.step.id}: actual stored heartbeat leaves scene/events/outcome unchanged; first failure survives heartbeat after real GPU loss`);
};
