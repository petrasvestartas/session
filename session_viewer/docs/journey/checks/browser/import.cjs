const assert = require('node:assert/strict');
const fs = require('node:fs');

module.exports = async (page, {command, drawing, step}) => {
    const sample = fs.readFileSync(`target/course-${step.id}/sample.pb`);
    const initial = await drawing(page);
    const ready = async () => page.waitForFunction(() => document.querySelector('#status')?.textContent === 'Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.');
    const choose = async (name, buffer) => {
        const selecting = page.waitForEvent('filechooser');
        await command(page, 'Open');
        await (await selecting).setFiles({name, mimeType: 'application/octet-stream', buffer});
    };
    for (const newer of ['malformed', 'oversized']) {
        await page.reload();
        await ready();
        const unchanged = await drawing(page);
        await page.evaluate(() => {
            const original = Blob.prototype.arrayBuffer;
            window.__courseFileEvents = 0;
            window.__courseFileReads = [];
            window.addEventListener('viewer-file', () => window.__courseFileEvents++);
            File.prototype.arrayBuffer = function () {
                window.__courseFileReads.push(this.name);
                if (this.name !== 'held-old.pb') return original.call(this);
                window.__courseHeldFile = this;
                window.__courseOriginalRead = original;
                return new Promise(resolve => { window.__courseReleaseFile = resolve; });
            };
        });
        await choose('held-old.pb', sample);
        await page.waitForFunction(() => typeof window.__courseReleaseFile === 'function');
        await choose(newer + '.pb', newer === 'malformed' ? Buffer.from([255]) : Buffer.alloc(4 * 1024 * 1024 + 1));
        const expected = newer === 'malformed' ? 'Invalid session protobuf' : 'This checkpoint accepts files up to 4 MiB';
        await page.waitForFunction(expected => document.querySelector('#status')?.textContent === expected, expected);
        assert.equal(await drawing(page), unchanged, 'A failed newer selection must preserve the scene');
        const events = await page.evaluate(() => window.__courseFileEvents);
        if (newer === 'oversized') assert(!await page.evaluate(() => window.__courseFileReads.includes('oversized.pb')), 'Oversized files must be rejected before byte reading');
        await page.evaluate(async () => {
            const buffer = await window.__courseOriginalRead.call(window.__courseHeldFile);
            window.__courseReleaseFile(buffer);
        });
        await page.waitForTimeout(220);
        assert.equal(await page.evaluate(() => window.__courseFileEvents), events, 'An old pending read must not deliver a completion event');
        assert.equal(await drawing(page), unchanged, 'An old pending read must not import after a newer selection');
        assert.equal(await page.locator('#status').textContent(), expected, 'Stale success must not overwrite current failure feedback');
    }
    await page.reload();
    await ready();
    await choose('sample.pb', sample);
    await page.waitForFunction(() => document.querySelector('#status')?.textContent === 'File imported. Undo removes the entire import.');
    await command(page, 'View Isometric');
    assert.equal(await drawing(page), initial, 'Restore the accepted imported fixture for capture');
};
