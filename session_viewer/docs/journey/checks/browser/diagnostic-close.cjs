const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
module.exports = async (page, helpers) => {
    try { await require('./heartbeat-timer.cjs')(page, helpers); }
    catch (error) {
        console.error('Close checkpoint inherited dock failure:', await page.locator('canvas').getAttribute('data-command-ui'));
        throw error;
    }
    const snapshot = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
    const state = async () => Object.fromEntries(await Promise.all(['data-row-placements', 'data-selected-id', 'data-camera-matrix', 'data-gpu-stats', 'data-command-ui']
        .map(async name => [name, await page.locator('canvas').getAttribute(name)])));
    const original = await snapshot(), scene = await helpers.drawing(page), editor = await state();
    await page.waitForTimeout(20); await page.evaluate(() => window.wasmBindings.close_report());
    const closed = await snapshot(); assert.equal(closed.outcome, 'closed');
    assert(Date.parse(closed.lastSeen) > Date.parse(original.lastSeen));
    assert.deepEqual(closed, {...original, outcome: 'closed', lastSeen: closed.lastSeen});
    assert.equal(await helpers.drawing(page), scene); assert.deepEqual(await state(), editor);
    const stored = await page.evaluate(started => Object.keys(localStorage).filter(key => key.startsWith('viewer-journey-report:'))
        .map(key => JSON.parse(localStorage.getItem(key))).find(report => report.started === started), closed.started);
    assert.deepEqual(stored, closed);
    await page.evaluate(() => window.wasmBindings.observe('milestone', 'geometry on screen'));
    const late = await snapshot(); assert.equal(late.outcome, 'closed'); assert.equal(late.failure, null);
    assert.equal(late.events.at(-1).message, 'geometry on screen');
    assert.equal(await helpers.drawing(page), scene); assert.deepEqual(await state(), editor);
    const file = page.waitForEvent('download', {predicate: file => file.suggestedFilename() === 'viewer-diagnostic.json'});
    await page.evaluate(() => window.__lossProbe.devices[0].destroy());
    const failed = JSON.parse(await fs.readFile(await (await file).path(), 'utf8'));
    await page.waitForFunction(() => !window.wasmBindings.runtime_running());
    await page.waitForTimeout(20); await page.evaluate(() => window.wasmBindings.close_report());
    const after = await snapshot(); assert.equal(after.outcome, 'failed');
    assert.deepEqual(after, {...failed, lastSeen: after.lastSeen});
    assert.deepEqual(await page.evaluate(() => window.__lossProbe.calls), []);
    await page.reload(); await page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && document.querySelector('canvas')?.hasAttribute('data-command-ui'));
    await require('./diagnostic-proof.cjs')(page, helpers, ['Move 0.05,0,0', 'Move 0,-0.05,0', 'Move 0.05,0,0']);
    console.log(`${helpers.step.id}: real final-close persistence preserves scene/history and keeps Failed/first failure after actual GPU loss`);
};
