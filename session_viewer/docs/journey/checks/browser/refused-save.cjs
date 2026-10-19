const assert = require('node:assert/strict');

module.exports = async (page, helpers, message) => {
    const downloads = [];
    const record = download => downloads.push(download);
    page.on('download', record);
    try {
        await helpers.command(page, 'Save');
        const ui = JSON.parse(await page.locator('canvas').getAttribute('data-command-ui'));
        assert.equal(ui.history.at(-1), '> Save\n' + message);
        assert.equal(downloads.length, 0, 'An incomplete source must not produce an editable archive');
    } finally { page.off('download', record); }
};
