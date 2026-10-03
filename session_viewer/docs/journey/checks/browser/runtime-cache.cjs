const assert = require('node:assert/strict');
const path = require('node:path');
module.exports = async (page, helpers) => {
    await require('./runtime-owner.cjs')(page, helpers);
    const {command, drawing, step} = helpers;
    const data = async key => JSON.parse(await page.locator('canvas').getAttribute(key));
    const state = async () => ({placement: await data('data-row-placements'), selected: await page.locator('canvas').getAttribute('data-selected-id'), camera: await data('data-camera-matrix')});
    const before = await state(), image = await drawing(page);
    await page.mouse.move(100, 100); await page.mouse.down({button:'right'});
    await page.evaluate(() => window.dispatchEvent(new PageTransitionEvent('pagehide', {persisted:true})));
    await page.waitForTimeout(100); assert(await page.evaluate(() => window.wasmBindings.runtime_running()));
    await page.evaluate(() => window.dispatchEvent(new PageTransitionEvent('pageshow', {persisted:true})));
    await page.mouse.move(180, 150); await page.mouse.up({button:'right'}); await page.waitForTimeout(120);
    assert.deepEqual(await state(), before, 'Cached hide cancels the old camera gesture'); assert.equal(await drawing(page), image);
    await command(page, 'Background'); assert.notEqual(await drawing(page), image);
    await command(page, 'Background'); assert.equal(await drawing(page), image);
    await command(page, 'Move 0.1,0,0'); await command(page, 'Fit');
    const navigatedState = await state(), navigatedImage = await drawing(page);
    const token = await page.evaluate(() => {
        window.__cacheToken = crypto.randomUUID(); window.__cachedReturn = false;
        window.addEventListener('pageshow', event => { window.__cachedReturn = event.persisted; });
        return window.__cacheToken;
    });
    await page.goto('about:blank'); await page.goBack({waitUntil:'domcontentloaded'});
    await page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && document.querySelector('canvas')?.hasAttribute('data-command-ui'));
    const restored = await page.evaluate(token => ({cached: window.__cacheToken === token, persisted:window.__cachedReturn === true}), token);
    if (restored.cached) {
        assert(restored.persisted); assert.deepEqual(await state(), navigatedState); assert.equal(await drawing(page), navigatedImage);
    }
    console.log(`${step.id}: actual Back navigation ${restored.cached ? 'reused cached document' : 'loaded a fresh document'}`);
    // The proof image always uses the same selected specimen, independently of browser cache eligibility.
    const picker = page.waitForEvent('filechooser'); await command(page, 'Open Replace');
    await (await picker).setFiles(path.resolve('target', `course-${step.id}`, 'sample.pb'));
    await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Document replaced. Undo restores the previous document.');
    await command(page, 'Select Next'); await command(page, 'Move 0.35,0,0.25'); await command(page, 'Move 0.25,0,0.15');
    await command(page, 'Move 0,-0.5,0'); await command(page, 'View Isometric'); await command(page, 'Orbit Right');
    await command(page, 'Move 0.1,0,0'); await command(page, 'Fit');
};
