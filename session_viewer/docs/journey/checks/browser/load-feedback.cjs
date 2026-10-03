const assert = require('node:assert/strict');
const path = require('node:path');

module.exports = async (page, {command, drawing, step}) => {
    const specimen = path.resolve('target', `course-${step.id}`, 'sample.pb');
    const latest = async () => JSON.parse(await page.locator('canvas').getAttribute('data-command-ui')).history.at(-1);
    const upload = async file => {
        const picker = page.waitForEvent('filechooser');
        await command(page, 'Open');
        await (await picker).setFiles(file);
    };
    const failure = async message => {
        await page.waitForFunction(text => document.getElementById('status')?.textContent?.includes(text), message);
        assert.match(await latest(), new RegExp(message));
        assert.equal(await page.locator('#status').getAttribute('hidden'), '', 'Errors stay in the actual command dock');
    };
    const baseline = await drawing(page);
    await command(page, 'Undo');
    const undone = await drawing(page);
    assert.notEqual(undone, baseline);
    await upload({name: 'malformed.pb', mimeType: 'application/octet-stream', buffer: Buffer.from([255])});
    await failure('Invalid session protobuf');
    assert.equal(await drawing(page), undone, 'Malformed bytes leave scene and selection unchanged');
    assert.notEqual(await page.locator('#status').textContent(), 'File imported. Undo removes the entire import.');
    await command(page, 'Redo');
    assert.equal(await drawing(page), baseline, 'Failed decoding preserves Redo');

    await upload({name: 'oversize.pb', mimeType: 'application/octet-stream', buffer: Buffer.alloc(4 * 1024 * 1024 + 1)});
    await failure('up to 4 MiB');
    assert.equal(await drawing(page), baseline, 'An oversized file is refused before changing the scene');
    await page.evaluate(() => {
        window.__originalRead = File.prototype.arrayBuffer;
        File.prototype.arrayBuffer = () => Promise.reject(new Error('read refused by test'));
    });
    try {
        await upload(specimen);
        await failure('Cannot read file');
        assert.equal(await drawing(page), baseline, 'Rejected reads preserve the drawing');
    } finally {
        await page.evaluate(() => { File.prototype.arrayBuffer = window.__originalRead; delete window.__originalRead; });
    }
    await upload(specimen);
    await page.waitForFunction(() => document.getElementById('status')?.textContent === 'File imported. Undo removes the entire import.');
    await page.waitForTimeout(150);
    await command(page, 'Undo');
    assert.equal(await drawing(page), baseline, 'A valid read after failures commits as one import');
};
