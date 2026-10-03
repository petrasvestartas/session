const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

module.exports = async (page, {command, drawing, step}) => {
    await page.evaluate(() => {
        window.__saveURLs = {created: [], revoked: []};
        const create = URL.createObjectURL.bind(URL), revoke = URL.revokeObjectURL.bind(URL);
        URL.createObjectURL = blob => {
            const url = create(blob); window.__saveURLs.created.push(url); return url;
        };
        URL.revokeObjectURL = url => { window.__saveURLs.revoked.push(url); revoke(url); };
    });
    const baseline = await drawing(page);
    assert.equal(await page.locator('#open').getAttribute('accept'), '.pb,.session');
    const pending = page.waitForEvent('download');
    await command(page, 'Save');
    const download = await pending;
    assert.equal(download.suggestedFilename(), 'viewer.session');
    const file = path.resolve('target', `course-${step.id}`, 'downloaded.session');
    await download.saveAs(file);
    assert(fs.statSync(file).size > 100, 'A nonempty editable file was downloaded');
    const latest = async () => JSON.parse(await page.locator('canvas').getAttribute('data-command-ui')).history.at(-1);
    assert.match(await latest(), /Saved editable document \(\d+ bytes\)/);
    assert.equal(await drawing(page), baseline, 'Save does not change scene pixels');
    assert.equal(await page.locator('a[download]').count(), 0, 'The temporary anchor is removed');
    await command(page, 'Undo');
    assert.notEqual(await drawing(page), baseline, 'Undo still reaches the preceding Move');
    await command(page, 'Redo');
    assert.equal(await drawing(page), baseline, 'Redo restores the saved scene');

    for (let i = 0; i < 6; i++) {
        await command(page, 'Select Next');
        await command(page, 'Delete');
    }
    const empty = await drawing(page);
    assert.notEqual(empty, baseline, 'Rows were removed before reopening');
    const downloads = [];
    const record = download => downloads.push(download);
    page.on('download', record);
    try {
        await command(page, 'Save');
        assert.match(await latest(), /1–64 meshes/);
        assert.equal(await drawing(page), empty, 'A refused Save leaves the empty scene unchanged');
        assert.equal(downloads.length, 0, 'A refused Save starts no download');
    } finally {
        page.off('download', record);
    }

    const picker = page.waitForEvent('filechooser');
    await command(page, 'Open');
    await (await picker).setFiles(file);
    await page.waitForFunction(() => document.getElementById('status')?.textContent === 'File imported. Undo removes the entire import.');
    await page.waitForTimeout(200);
    for (let i = 0; i < 6; i++) await command(page, 'Select Next');
    assert.equal(await drawing(page), baseline, 'Opening the actual download restores local geometry, placement and selection pixels');
    await page.waitForFunction(() => window.__saveURLs.revoked.length === 1, undefined, {timeout: 20_000});
    const urls = await page.evaluate(() => window.__saveURLs);
    assert.equal(urls.created.length, 1, 'Only the successful Save creates an object URL');
    assert.deepEqual(urls.revoked, urls.created, 'The delayed cleanup releases the downloaded Blob URL');
};
