const assert = require('node:assert/strict');
module.exports = async (page, helpers) => {
    await require('./edit-delete.cjs')(page, helpers);
    const before = await helpers.drawing(page);
    await helpers.command(page, 'Move 0.15,0,0');
    const moved = await helpers.drawing(page); assert.notEqual(moved, before);
    const download = page.waitForEvent('download'); await helpers.command(page, 'Save');
    const file = await download; assert.equal(await file.failure(), null);
    await helpers.command(page, 'Undo'); assert.equal(await helpers.drawing(page), before);
    await helpers.command(page, 'Redo'); assert.equal(await helpers.drawing(page), moved);
    await helpers.command(page, 'Fit');
};
