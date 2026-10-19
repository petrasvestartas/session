const assert = require('node:assert/strict');
module.exports = async (page, helpers) => {
    await require('./imported-interactions.cjs')(page, helpers);
    for (const line of ['View Isometric', 'View Isometric Back', 'Select All', 'Select Object', 'Select None', 'View Isometric']) {
        await helpers.command(page, line);
        const ui = JSON.parse(await page.locator('canvas').getAttribute('data-command-ui'));
        assert.equal(ui.history.at(-1).split('\n')[0], '> ' + line);
    }
    await helpers.command(page, 'Fit');
};
