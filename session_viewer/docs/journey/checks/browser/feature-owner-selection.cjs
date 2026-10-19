const assert = require('node:assert/strict');
module.exports = async (page, helpers) => {
    await require('./exact-completion.cjs')(page, helpers);
    const value = name => page.locator('canvas').getAttribute(name).then(JSON.parse);
    const control = async key => {
        await page.waitForFunction(key => JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).controls.some(c => c.key === key), key);
        const item = (await value('data-command-ui')).controls.find(c => c.key === key);
        await page.mouse.move((item.rect[0] + item.rect[2]) / 2, (item.rect[1] + item.rect[3]) / 2);
        await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
        await page.mouse.down(); await page.mouse.up();
    };
    const selected = async id => page.waitForFunction(id => JSON.parse(document.querySelector('canvas').getAttribute('data-selected-ids')).join() === String(id), id);
    await helpers.command(page, 'Element Attributes On');
    await helpers.command(page, 'Layers All');
    let rows = await value('data-tree');
    const host = rows.find(row => row.label === 'host');
    const child = rows.find(row => row.label === 'child');
    const interaction = rows.find(row => row.label === 'InteractionContactFace seam');
    const piece = rows.slice(rows.indexOf(interaction) + 1, interaction.end).find(row => row.label.startsWith('piece '));
    assert(host && child && interaction && piece);
    for (const row of [interaction, piece]) {
        await control(`tree/select/${row.key}`);
        await selected(host.ids[0]);
        assert.equal((await value('data-selected-ids')).length, 1, 'Generated paths never enter editing selection');
        assert(Number(await page.locator('canvas').getAttribute('data-highlight-bytes')) > 0);
    }
    await control(`tree/select/${child.key}`);
    await selected(child.ids[0]);
    await control(`tree/select/${piece.key}`);
    await selected(host.ids[0]);
    await helpers.command(page, 'Delete');
    await page.waitForFunction(() => document.querySelector('canvas').getAttribute('data-object-count') === '1');
    assert.equal((await value('data-tree')).filter(row => row.label.startsWith('piece ')).length, 0);
    await helpers.command(page, 'Undo');
    await page.waitForFunction(() => document.querySelector('canvas').getAttribute('data-object-count') === '3');
    rows = await value('data-tree');
    assert.equal(rows.find(row => row.label === 'host').ids[0], host.ids[0]);
    assert.equal(rows.find(row => row.label === 'child').ids[0], child.ids[0]);
    assert.equal(rows.filter(row => row.label.startsWith('piece ')).length, 6);
    await control(`tree/select/${piece.key}`);
    await selected(host.ids[0]);
    await helpers.command(page, 'Fit');
};
