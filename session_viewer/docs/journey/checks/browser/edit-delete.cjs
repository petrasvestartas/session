const assert = require('node:assert/strict');
module.exports = async (page, helpers) => {
    await require('./edit-move.cjs')(page, helpers);
    const before = await helpers.drawing(page);
    await helpers.command(page, 'Delete');
    const removed = await helpers.drawing(page); assert.notEqual(removed, before);
    await helpers.command(page, 'Undo');
    await helpers.command(page, 'Select Next'); await helpers.command(page, 'Select Next');
    assert.equal(await helpers.drawing(page), before);
    await helpers.command(page, 'Redo'); assert.equal(await helpers.drawing(page), removed);
    await helpers.command(page, 'Select Next'); await helpers.command(page, 'Fit');
};
