const assert = require('node:assert/strict');
module.exports = async (page, helpers) => {
    await require('./saved-exclusions.cjs')(page, helpers);
    const before = await page.locator('canvas').getAttribute('data-gpu-stats');
    const result = await page.evaluate(() => {
        const prefix = 'viewer-journey-report:';
        const source = JSON.parse(window.wasmBindings.diagnostic_snapshot());
        const probe = () => JSON.parse(window.wasmBindings.previous_storage_probe());
        const clean = () => { for (const key of Object.keys(localStorage)) if (key.startsWith(prefix)) localStorage.removeItem(key); };
        clean(); sessionStorage.setItem(prefix, 'known-tab');
        const first = probe(), second = probe();
        const time = age => new Date(Date.now() - age).toISOString();
        const failed = (message, failureAge, seenAge = 5000) => ({...source, tab: 'other', started: time(4 * 86400000),
            lastSeen: time(seenAge), outcome: 'failed', events: [],
            failure: {time: time(failureAge), kind: 'fatal', message, elapsedMs: 1}});
        const put = (name, report) => localStorage.setItem(prefix + name, JSON.stringify(report));
        put('old-fresh-beat', failed('old failure', 600000));
        put('new-stale-beat', failed('new failure', 120000, 60000));
        const entries = Object.entries(localStorage); const ranked = probe();
        const unchanged = JSON.stringify(Object.entries(localStorage)) === JSON.stringify(entries);
        clean(); put('stale', failed('days ago', 3 * 86400000)); const stale = probe();
        clean(); put('ready', source); const healthy = probe();
        clean(); put('active', {...source, tab: 'other', started: time(300000), lastSeen: time(30000), outcome: 'running'});
        const active = probe();
        put('active', {...source, tab: 'other', started: time(300000), lastSeen: time(180000), outcome: 'running'});
        const interrupted = probe();
        put('active', {...source, tab: 'known-tab', started: time(300000), lastSeen: time(5000), outcome: 'running'});
        const sameTab = probe();
        clean(); put('future', failed('future', -60000, -5000));
        localStorage.setItem(prefix + 'malformed', '{'); put('unsupported', {...source, unknown: true});
        localStorage.setItem(prefix + 'too-large', ' '.repeat(1024 * 1024 + 1)); const invalid = probe();
        const local = Object.getOwnPropertyDescriptor(window, 'localStorage');
        const session = Object.getOwnPropertyDescriptor(window, 'sessionStorage');
        let denied;
        try {
            const reject = () => { throw new DOMException('Storage denied', 'SecurityError'); };
            Object.defineProperty(window, 'localStorage', {configurable: true, get: reject});
            Object.defineProperty(window, 'sessionStorage', {configurable: true, get: reject});
            denied = probe();
        } finally {
            if (local) Object.defineProperty(window, 'localStorage', local); else delete window.localStorage;
            if (session) Object.defineProperty(window, 'sessionStorage', session); else delete window.sessionStorage;
            clean();
        }
        return {first, second, ranked, unchanged, stale, healthy, active, interrupted, sameTab, invalid, denied,
            after: JSON.parse(window.wasmBindings.diagnostic_snapshot())};
    });
    assert.equal(result.first.tab, 'known-tab'); assert.equal(result.second.tab, result.first.tab);
    assert.notEqual(result.first.key, result.second.key); assert.equal(result.first.previous, null);
    assert.equal(result.ranked.previous.failure.message, 'new failure'); assert(result.unchanged);
    for (const name of ['stale', 'healthy', 'active', 'invalid']) assert.equal(result[name].previous, null, name);
    assert.equal(result.interrupted.previous.outcome, 'running'); assert.equal(result.sameTab.previous.tab, 'known-tab');
    assert.equal(result.denied.previous, null); assert(result.denied.tab.length > 0); assert.notEqual(result.denied.tab, 'known-tab');
    assert.equal(result.after.outcome, 'ready'); assert.equal(result.after.failure, null);
    assert.equal(await page.locator('canvas').getAttribute('data-gpu-stats'), before);
    await helpers.command(page, 'Move 0.05,0,0'); await helpers.command(page, 'Fit');
    console.log(`${helpers.step.id}: real Web Storage admission, failure-time ranking, stable tab and denied-storage reader passed`);
};
