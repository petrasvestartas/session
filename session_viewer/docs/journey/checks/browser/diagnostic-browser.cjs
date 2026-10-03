const assert = require('node:assert/strict');
const path = require('node:path');
module.exports = async (page, helpers) => {
    await require('./diagnostic-events.cjs')(page, helpers);
    const {command, step} = helpers;
    const ready = () => page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && document.querySelector('canvas')?.hasAttribute('data-command-ui'));
    const snapshot = () => page.evaluate(() => JSON.parse(window.wasmBindings.diagnostic_snapshot()));
    const before = await page.locator('canvas').getAttribute('data-gpu-stats');
    const first = await snapshot(); assert.equal(first.version, 1); assert.equal(first.outcome, helpers.diagnosticOutcome || 'running');
    assert.equal(await page.locator('canvas').getAttribute('data-gpu-stats'), before);
    const actual = () => page.evaluate(() => ({page: location.origin + location.pathname, browser: navigator.userAgent,
        viewport: [innerWidth, innerHeight], canvas: [document.querySelector('canvas').width, document.querySelector('canvas').height],
        devicePixelRatio, secureContext: isSecureContext, webgpu: !!navigator.gpu}));
    for (const [key, value] of Object.entries(await actual())) assert.deepEqual(first[key], value);
    assert(Number.isFinite(Date.parse(first.started)));
    if (helpers.diagnosticOutcome) {
        assert(Date.parse(first.lastSeen) >= Date.parse(first.started));
        assert(first.events.some(event => event.kind === 'milestone' && event.message === 'geometry on screen'));
    } else { assert.equal(first.started, first.lastSeen); assert.deepEqual(first.events, []); }
    assert.equal(first.failure, null);
    await page.setViewportSize({width: 618, height: 800}); await page.waitForTimeout(120);
    const resized = await snapshot();
    for (const [key, value] of Object.entries(await actual())) assert.deepEqual(resized[key], value);
    const url = new URL(page.url()); url.searchParams.set('private', 'not-in-report'); url.hash = 'private-fragment';
    await page.goto(url.href); await ready();
    const fresh = await snapshot();
    if (helpers.stableTab) assert.equal(fresh.tab, first.tab); else assert.notEqual(fresh.tab, first.tab);
    assert.notEqual(fresh.started, first.started);
    assert.equal(fresh.page, url.origin + url.pathname); assert(!JSON.stringify(fresh).includes('not-in-report'));
    assert(!JSON.stringify(fresh).includes('private-fragment'));
    await page.setViewportSize({width: 900, height: 760}); await page.waitForTimeout(120);
    const picker = page.waitForEvent('filechooser'); await command(page, 'Open Replace');
    await (await picker).setFiles(path.resolve('target', `course-${step.id}`, 'sample.pb'));
    await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Document replaced. Undo restores the previous document.');
    for (const line of ['Select Next', 'Move 0.35,0,0.25', 'Move 0.25,0,0.15', 'Move 0,-0.5,0', 'View Isometric',
        'Orbit Right', 'Move 0.1,0,0', 'Orbit Up', 'Orbit Right', 'Orbit Up', 'Orbit Right', 'Move 0.05,0,0', 'Fit']) await command(page, line);
};
