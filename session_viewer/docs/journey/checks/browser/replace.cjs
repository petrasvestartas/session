const assert = require('node:assert/strict');
const path = require('node:path');

module.exports = async (page, {command, drawing, step}) => {
    await require('./load-feedback.cjs')(page, {command, drawing, step});
    const specimen = path.resolve('target', `course-${step.id}`, 'sample.pb');
    const count = async () => Number(await page.locator('canvas').getAttribute('data-object-count'));
    const history = async () => JSON.parse(await page.locator('canvas').getAttribute('data-command-ui')).history;
    const choose = async (verb, file) => {
        const picker = page.waitForEvent('filechooser');
        await command(page, verb);
        const chooser = await picker;
        if (file) await chooser.setFiles(file);
    };
    const baseline = await drawing(page), initial = await count();
    assert.equal(initial, 5);
    const projection = await page.locator('canvas').getAttribute('data-projection');
    await page.evaluate(() => {
        const original = File.prototype.arrayBuffer;
        window.__originalRead = original;
        File.prototype.arrayBuffer = function () {
            const bytes = original.call(this);
            return new Promise((resolve, reject) => { window.__releaseRead = () => bytes.then(resolve, reject); });
        };
    });
    try {
        await choose('Open', specimen);
        await page.waitForFunction(() => typeof window.__releaseRead === 'function');
        await choose('Open Replace');
        // Exercise the input's native cancel-event handler; Playwright intercepts the OS picker.
        await page.locator('#open').evaluate(input => input.dispatchEvent(new Event('cancel', {bubbles: true})));
        assert.equal(await page.locator('#status').textContent(), 'Open cancelled.');
        const cancelled = await history();
        await page.evaluate(() => window.__releaseRead());
        await page.waitForTimeout(150);
        assert.equal(await count(), initial, 'Opening a newer picker revokes old delivery before selection');
        assert.equal(await drawing(page), baseline);
        assert.deepEqual(await history(), cancelled, 'A cancelled newer picker cannot resurrect the old read');
    } finally {
        await page.evaluate(() => { File.prototype.arrayBuffer = window.__originalRead; delete window.__originalRead; delete window.__releaseRead; });
    }
    await choose('Open Replace', {name: 'bad.pb', mimeType: 'application/octet-stream', buffer: Buffer.from([255])});
    await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Invalid session protobuf');
    assert.equal(await count(), initial, 'Malformed replacement preserves every old row');
    assert.equal(await drawing(page), baseline);
    await command(page, 'Redo');
    assert.equal(await count(), initial + 3, 'Failed replacement preserves the earlier Redo');
    await command(page, 'Undo');
    assert.equal(await drawing(page), baseline);

    await choose('Open Replace', specimen);
    await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Document replaced. Undo restores the previous document.');
    assert.equal(await count(), 3, 'Replacement does not append to old rows');
    const replaced = await drawing(page);
    assert.notEqual(replaced, baseline);
    assert.equal(await page.locator('canvas').getAttribute('data-projection'), projection);
    await command(page, 'Undo');
    assert.equal(await count(), initial);
    assert.equal(await drawing(page), baseline, 'Undo restores all old scene pixels');
    await command(page, 'Redo');
    assert.equal(await count(), 3);
    assert.equal(await drawing(page), replaced, 'Redo restores the replacement without changing the camera');
    await command(page, 'Select Next');
    await command(page, 'Select Next');
    await command(page, 'Move 0.35,0,0.25');
    await command(page, 'View Isometric');
    await command(page, 'Fit');
};
