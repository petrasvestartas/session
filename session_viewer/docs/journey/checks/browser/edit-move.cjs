const assert = require('node:assert/strict');
module.exports = async (page, helpers) => {
    await require('./edit-scope.cjs')(page, helpers);
    const before = await helpers.drawing(page);
    await helpers.command(page, 'Move 0.25,0,0.15');
    const moved = await helpers.drawing(page);
    assert.notEqual(moved, before);
    await helpers.command(page, 'Undo'); assert.equal(await helpers.drawing(page), before);
    await helpers.command(page, 'Redo'); assert.equal(await helpers.drawing(page), moved);
    await helpers.command(page, 'Fit');
};
