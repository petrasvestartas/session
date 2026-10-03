const assert = require('node:assert/strict');
module.exports = async (page, helpers) => {
    await require('./storage-reader.cjs')(page, helpers);
    const before = await page.locator('canvas').getAttribute('data-gpu-stats');
    const result = await page.evaluate(() => {
        const prefix = 'viewer-journey-report:';
        const source = {...JSON.parse(window.wasmBindings.diagnostic_snapshot()), tab: 'known-tab'};
        const keys = () => Object.keys(localStorage).filter(key => key.startsWith(prefix));
        const clean = () => { for (const key of keys()) localStorage.removeItem(key); };
        const put = (key, age) => localStorage.setItem(prefix + key, JSON.stringify({...source,
            lastSeen: new Date(Date.now() - age).toISOString()}));
        const probe = value => JSON.parse(window.wasmBindings.stored_report_probe(JSON.stringify(value)));
        clean(); localStorage.setItem('unrelated-proof', 'retained');
        put('oldest', 600000); put('middle', 400000); put('newest', 200000);
        localStorage.setItem(prefix + 'malformed', '{');
        const written = probe(source), retained = keys();
        const saved = JSON.parse(localStorage.getItem(written.key));
        const unrelated = localStorage.getItem('unrelated-proof');
        const snapshot = JSON.stringify(Object.entries(localStorage));
        const wrongTab = probe({...source, tab: 'wrong-tab'});
        const wrongUnchanged = snapshot === JSON.stringify(Object.entries(localStorage));
        const originalSet = Storage.prototype.setItem, originalRemove = Storage.prototype.removeItem;
        let quota, quotaUnchanged, deletion, deletionSaved;
        try {
            Storage.prototype.setItem = () => { throw new DOMException('Full', 'QuotaExceededError'); };
            quota = probe(source); quotaUnchanged = snapshot === JSON.stringify(Object.entries(localStorage));
        } finally { Storage.prototype.setItem = originalSet; }
        clean(); for (let i = 0; i < 4; i++) put(`prior-${i}`, (i + 1) * 10000);
        try {
            Storage.prototype.removeItem = () => { throw new DOMException('Denied', 'SecurityError'); };
            deletion = probe(source); deletionSaved = JSON.parse(localStorage.getItem(deletion.key));
        } finally { Storage.prototype.removeItem = originalRemove; }
        clean(); for (let i = 0; i < 33; i++) put(`excess-${i}`, i * 1000);
        const excessiveBefore = JSON.stringify(Object.entries(localStorage));
        const excessive = probe(source), excessiveUnchanged = excessiveBefore === JSON.stringify(Object.entries(localStorage));
        clean(); localStorage.removeItem('unrelated-proof');
        return {written, retained, saved, source, unrelated, wrongTab, wrongUnchanged, quota, quotaUnchanged,
            deletion, deletionSaved, excessive, excessiveUnchanged};
    });
    assert(result.written.first && result.written.second); assert.equal(result.retained.length, 3);
    assert(result.retained.includes(result.written.key));
    assert(result.retained.includes('viewer-journey-report:newest')); assert(result.retained.includes('viewer-journey-report:middle'));
    assert.deepEqual(result.saved, result.source); assert.equal(result.unrelated, 'retained');
    for (const name of ['wrongTab', 'quota', 'deletion', 'excessive']) {
        assert.equal(result[name].first, false, name); assert.equal(result[name].second, false, name);
    }
    assert(result.wrongUnchanged); assert(result.quotaUnchanged); assert(result.excessiveUnchanged);
    assert.deepEqual(result.deletionSaved, result.source);
    assert.equal(await page.locator('canvas').getAttribute('data-gpu-stats'), before);
    await helpers.command(page, 'Move 0,-0.05,0'); await helpers.command(page, 'Fit');
    console.log(`${helpers.step.id}: actual three-run retention, stable write key, quota and deletion failure behavior passed`);
};
