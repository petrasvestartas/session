const assert = require('node:assert/strict');
module.exports = async (page, {command, drawing, step}) => {
    const value = name => page.locator('canvas').getAttribute(name).then(JSON.parse);
    const control = async key => {
        await page.waitForFunction(key => JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).controls.some(c => c.key === key), key);
        const item = (await value('data-command-ui')).controls.find(c => c.key === key);
        await page.mouse.move((item.rect[0] + item.rect[2]) / 2, (item.rect[1] + item.rect[3]) / 2);
        await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
        await page.mouse.down(); await page.mouse.up();
    };
    const open = async (file, archive = false) => {
        const choosing = page.waitForEvent('filechooser');
        await command(page, 'Open');
        await (await choosing).setFiles(file);
        await page.waitForFunction(expected => document.querySelector('#status').textContent === expected, archive
            ? 'Document replaced. Undo restores the previous document.'
            : 'File imported. Undo removes the entire import.');
        await page.waitForFunction(() => document.querySelector('canvas').getAttribute('data-object-count') === '3');
    };
    const path = `target/course-${step.id}/sample.pb`;
    await command(page, 'Close');
    const empty = await drawing(page);
    await open(path);
    await command(page, 'Fit');
    await command(page, 'Layers All');
    assert.notEqual(await drawing(page), empty, 'Imported original geometry must render');
    let rows = await value('data-tree');
    const child = rows.find(r => r.label === 'child');
    const host = rows.find(r => r.label === 'host');
    assert(child && host);
    assert.equal(child.readonly, false, 'A child Element under attributes remains editable');
    assert(rows.some(r => r.label === 'InteractionContactFace seam'));
    assert(rows.some(r => r.label === 'InteractionFeatureSolid connector'));
    assert(rows.some(r => r.label === 'cut unclaimed · face 5'));
    const pieces = rows.filter(r => r.label.startsWith('piece '));
    assert.equal(pieces.length, 6);
    assert(pieces.every(r => r.readonly));
    const flags = async () => (await value('data-view-flags')).rows;
    const visible = (state, id) => state.find(row => row.id === id).visible;
    const interactions = rows.find(r => r.label === 'interactions');
    assert.equal(interactions.ids.length, 4);
    const initial = await flags();
    assert(interactions.ids.every(id => visible(initial, id)));
    assert.equal(pieces.filter(r => !visible(initial, r.ids[0])).length, 2);
    await command(page, 'Element Attributes On');
    await command(page, 'Element Interactions Off');
    let state = await flags();
    assert(interactions.ids.every(id => !visible(state, id)));
    assert(visible(state, child.ids[0]));
    await command(page, 'Element Interactions On');
    state = await flags();
    assert(interactions.ids.every(id => visible(state, id)));
    await control(`tree/select/${child.key}`);
    await page.waitForFunction(id => JSON.parse(document.querySelector('canvas').getAttribute('data-selected-ids')).join() === String(id), child.ids[0]);
    await command(page, 'Delete');
    await page.waitForFunction(() => document.querySelector('canvas').getAttribute('data-object-count') === '2');
    await command(page, 'Undo');
    await page.waitForFunction(() => document.querySelector('canvas').getAttribute('data-object-count') === '3');
    assert.equal((await value('data-tree')).filter(r => r.label.startsWith('piece ')).length, 6);
    const treeLabels = (await value('data-tree')).map(r => [r.label, r.depth, r.readonly]);
    for (let i = 0; i < 2; i++) {
        const downloading = page.waitForEvent('download');
        await command(page, 'Save');
        const saved = await (await downloading).path();
        await command(page, 'Close');
        await open(saved, true);
        assert.deepEqual((await value('data-tree')).map(r => [r.label, r.depth, r.readonly]), treeLabels);
    }
    await command(page, 'Layers All');
    await command(page, 'View Isometric Back');
    await command(page, 'Fit');
};
