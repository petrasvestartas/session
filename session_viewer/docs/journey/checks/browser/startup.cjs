const assert = require('node:assert/strict');
const path = require('node:path');

module.exports = async (page, helpers) => {
    const normal = page.url().split('#')[0];
    await page.addInitScript(() => {
        if (location.hash === '#check-adapter-unavailable') {
            navigator.gpu.requestAdapter = () => Promise.resolve(null);
        }
    });
    await page.evaluate(url => history.replaceState(null, '', url), normal + '#check-adapter-unavailable');
    await page.reload();
    await page.waitForFunction(() => {
        const status = document.getElementById('status');
        return status?.textContent?.startsWith('Cannot draw:') && !status.hidden;
    });
    assert.equal(await page.locator('button, form').count(), 0, 'The failure fallback adds no feature controls');
    await page.evaluate(url => history.replaceState(null, '', url), normal);
    await page.reload();
    await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.');
    const picker = page.waitForEvent('filechooser');
    await helpers.command(page, 'Open');
    await (await picker).setFiles(path.resolve('target', `course-${helpers.step.id}`, 'sample.pb'));
    await page.waitForFunction(() => Number(document.querySelector('canvas').getAttribute('data-object-count')) === 5);
    await require('./replace.cjs')(page, helpers);
};
