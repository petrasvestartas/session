const assert = require('node:assert/strict');
const path = require('node:path');

module.exports = async (page, {command, drawing, step}) => {
    const specimen = path.resolve('target', `course-${step.id}`, 'sample.pb');
    const count = async () => Number(await page.locator('canvas').getAttribute('data-object-count'));
    const history = async () => JSON.parse(await page.locator('canvas').getAttribute('data-command-ui')).history;
    const upload = async file => {
        const picker = page.waitForEvent('filechooser');
        await command(page, 'Open');
        await (await picker).setFiles(file);
    };
    const settle = async (index, reject = false) => {
        await page.evaluate(({index, reject}) => window.__heldReads[index][reject ? 'reject' : 'resolve'](), {index, reject});
        await page.waitForTimeout(150);
    };
    const baseline = await drawing(page), initial = await count();
    assert.equal(initial, 6);
    await page.evaluate(() => {
        const original = File.prototype.arrayBuffer;
        window.__originalRead = original;
        window.__heldReads = [];
        File.prototype.arrayBuffer = function () {
            const bytes = original.call(this);
            return new Promise((resolve, reject) => window.__heldReads.push({
                resolve: () => bytes.then(resolve, reject),
                reject: () => reject(new Error('held read refused')),
            }));
        };
    });
    try {
        await upload(specimen);
        await page.waitForFunction(() => window.__heldReads.length === 1);
        await command(page, 'Cancel Open');
        const cancelled = await history();
        await settle(0);
        assert.equal(await count(), initial, 'Cancelled completion cannot append overlapping objects');
        assert.equal(await drawing(page), baseline);
        assert.deepEqual(await history(), cancelled, 'Cancelled completion cannot change feedback');
        assert.equal(await page.locator('#status').textContent(), 'Open cancelled.');

        await upload(specimen);
        await upload(specimen);
        await page.waitForFunction(() => window.__heldReads.length === 3);
        await command(page, 'View Isometric');
        const beforeOld = await history();
        await settle(1, true);
        assert.deepEqual(await history(), beforeOld, 'An old read error cannot overwrite a newer command');
        assert.equal(await count(), initial);
        assert.equal(await page.locator('#status').textContent(), 'Reading the selected file…');
        await settle(2);
        assert.equal(await count(), initial + 3, 'Only the latest chosen file commits');
        const completed = await history();
        assert.equal(completed.at(-2), beforeOld.at(-1), 'File completion retains the later typed command result');
        assert.match(completed.at(-1), /^> Open\nFile imported/);
        await command(page, 'Undo');
        assert.equal(await count(), initial);
        assert.equal(await drawing(page), baseline);

        await upload(specimen);
        await upload({name: 'bad.pb', mimeType: 'application/octet-stream', buffer: Buffer.from([255])});
        await page.waitForFunction(() => window.__heldReads.length === 5);
        await settle(4);
        assert.equal(await page.locator('#status').textContent(), 'Invalid session protobuf');
        const refused = await history();
        await settle(3);
        assert.equal(await count(), initial, 'Late valid bytes cannot replace a newer failed choice');
        assert.deepEqual(await history(), refused);
        assert.equal(await drawing(page), baseline);
        await command(page, 'Redo');
        assert.equal(await count(), initial + 3, 'Failure and stale completion preserve Redo');
        await command(page, 'Undo');
    } finally {
        await page.evaluate(() => { File.prototype.arrayBuffer = window.__originalRead; delete window.__originalRead; delete window.__heldReads; });
    }
    await upload(specimen);
    await page.waitForFunction(expected => Number(document.querySelector('canvas').getAttribute('data-object-count')) === expected, initial + 3);
    await command(page, 'Undo');
    assert.equal(await count(), initial);
    assert.equal(await drawing(page), baseline, 'A valid read still recovers after cancellation and failures');
};
