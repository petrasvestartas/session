const assert = require('node:assert/strict');
module.exports = async (page, helpers) => {
    await require('./reload-failures.cjs')(page, helpers);
    const before = await helpers.drawing(page);
    assert.deepEqual(await page.evaluate(() => Array.from(window.wasmBindings.listener_probe())), [2, 2, 0]);
    assert.equal(await helpers.drawing(page), before, 'The ownership probe does not mutate the viewer');
    await helpers.command(page, 'Orbit Right'); await helpers.command(page, 'Fit');
};
