const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const coordinates = require('./source-coordinates.cjs');
module.exports = async (page, helpers) => {
    await require('./reload-auto.cjs')(page, helpers);
    const {command, drawing, step} = helpers;
    const root = path.resolve('target', `course-${step.id}`), expected = coordinates(fs.readFileSync(path.join(root, 'sample.pb')));
    assert.equal(expected.length, 3); assert(expected.every(mesh => mesh.vertices.length > 0));
    const rounded = expected.map(mesh => ({...mesh, vertices: mesh.vertices.map(([key, xyz]) => [key, xyz.map(Math.fround)])}));
    assert.notDeepEqual(rounded, expected, 'Source precision must be distinguishable from the GPU floats');
    const data = async key => JSON.parse(await page.locator('canvas').getAttribute(key));
    const before = await drawing(page), placement = await data('data-row-placements');
    await page.evaluate(() => {
        window.__precisionURLs = {created: [], revoked: []}; window.__precisionCreate = URL.createObjectURL; window.__precisionRevoke = URL.revokeObjectURL;
        URL.createObjectURL = function (blob) { const url = window.__precisionCreate.call(URL, blob); window.__precisionURLs.created.push(url); return url; };
        URL.revokeObjectURL = function (url) { window.__precisionURLs.revoked.push(url); return window.__precisionRevoke.call(URL, url); };
    });
    const save = async name => {
        const pending = page.waitForEvent('download');
        // Real fetch may complete before the shared one-result helper returns; type directly into the same egui input.
        const ui = await data('data-command-ui'), field = ui.controls.find(row => row.key === 'command/input');
        const box = await page.locator('canvas').boundingBox(), r = field.rect;
        await page.mouse.click(box.x + (r[0] + r[2]) / 2, box.y + (r[1] + r[3]) / 2);
        await page.keyboard.press('ControlOrMeta+A'); await page.keyboard.type('Save'); await page.keyboard.press('Enter');
        const download = await pending; assert.equal(await download.failure(), null); const file = path.join(root, name); await download.saveAs(file);
        await page.waitForTimeout(120); assert.deepEqual(coordinates(fs.readFileSync(file)), expected);
        assert.deepEqual(await data('data-row-placements'), placement); assert.equal(await drawing(page), before);
        assert.equal(await page.locator('a[download]').count(), 0);
    };
    try {
        await save('warm-exact.session'); await command(page, 'Unload Sources');
        assert((await data('data-source-residency')).every(row => !row[1]));
        await save('restored-exact.session'); assert((await data('data-source-residency')).every(row => row[1]));
        await page.waitForFunction(() => window.__precisionURLs.created.every(url => window.__precisionURLs.revoked.filter(value => value === url).length === 1), undefined, {timeout: 20_000});
        assert.equal(await page.evaluate(() => window.__precisionURLs.created.length), 2);
        const sources = (await data('data-source-origins')).map(row => row[3]); await command(page, 'Close');
        const revoked = await page.evaluate(() => window.__precisionURLs.revoked); assert(sources.every(url => revoked.includes(url)));
        assert.equal((await data('data-row-placements')).length, 0);
    } finally {
        await page.evaluate(() => { URL.createObjectURL = window.__precisionCreate; URL.revokeObjectURL = window.__precisionRevoke; delete window.__precisionURLs; delete window.__precisionCreate; delete window.__precisionRevoke; });
    }
    // Restore the displayed proof scene, retaining this same page.
    const picker = page.waitForEvent('filechooser'); await command(page, 'Open Replace'); await (await picker).setFiles(path.join(root, 'sample.pb'));
    await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Document replaced. Undo restores the previous document.');
    await command(page, 'Select Next'); await command(page, 'Move 0.35,0,0.25'); await command(page, 'View Isometric'); await command(page, 'Fit');
    await command(page, 'Move 0.25,0,0.15'); await command(page, 'Move 0,-0.5,0'); await command(page, 'Orbit Up'); await command(page, 'Fit');
};
