const assert = require('node:assert/strict');
module.exports = async (page, helpers) => {
    await require('./live-storage.cjs')(page, helpers);
    const before = await page.locator('canvas').getAttribute('data-gpu-stats');
    const result = await page.evaluate(() => {
        const prefix = 'viewer-journey-report:';
        const source = JSON.parse(window.wasmBindings.diagnostic_snapshot());
        const clean = () => { for (const key of Object.keys(localStorage)) if (key.startsWith(prefix)) localStorage.removeItem(key); };
        clean();
        const put = (key, age) => localStorage.setItem(prefix + key, JSON.stringify({...source,
            lastSeen: new Date(Date.now() - age).toISOString()}));
        put('older', 20000); put('newer', 10000); localStorage.setItem(prefix + 'malformed', '{');
        const future = JSON.stringify({...source, version: 2, lastSeen: new Date(Date.now() - 5000).toISOString(),
            futureTelemetry: {resources: ['keep original bytes']}}, null, 2);
        localStorage.setItem(prefix + 'future', future);
        const output = JSON.parse(window.wasmBindings.stored_report_probe(JSON.stringify(source)));
        const retained = Object.keys(localStorage).filter(key => key.startsWith(prefix));
        const actual = localStorage.getItem(prefix + 'future');
        const adopted = JSON.parse(window.wasmBindings.previous_storage_probe()).previous;
        const entries = JSON.stringify(Object.entries(localStorage));
        const get = Storage.prototype.getItem; let denied;
        try {
            Storage.prototype.getItem = function (key) {
                if (key.startsWith(prefix) && key !== prefix) throw new DOMException('Read denied', 'SecurityError');
                return get.call(this, key);
            };
            denied = JSON.parse(window.wasmBindings.stored_report_probe(JSON.stringify(source)));
        } finally { Storage.prototype.getItem = get; }
        const unchanged = entries === JSON.stringify(Object.entries(localStorage));
        clean();
        return {output, retained, future, actual, adopted, denied, unchanged};
    });
    assert(result.output.first && result.output.second); assert.equal(result.retained.length, 3);
    assert(result.retained.includes(result.output.key)); assert(result.retained.includes('viewer-journey-report:future'));
    assert(result.retained.includes('viewer-journey-report:newer')); assert.equal(result.actual, result.future);
    assert.equal(result.adopted, null); assert.equal(result.denied.first, false); assert.equal(result.denied.second, false);
    assert(result.unchanged); assert.equal(await page.locator('canvas').getAttribute('data-gpu-stats'), before);
    await helpers.command(page, 'Move 0,-0.05,0'); await helpers.command(page, 'Fit');
    console.log(`${helpers.step.id}: unsupported raw JSON retained exactly, not adopted; denied reads preserve all evidence before mutation`);
};
