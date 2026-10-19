const assert = require('node:assert/strict');

module.exports = async (page, ids) => {
    await page.keyboard.press('Escape');
    await page.waitForFunction(() => !JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).focused);
    const selected = async () => JSON.parse(await page.locator('canvas').getAttribute('data-selected-ids'));
    assert.deepEqual((await selected()).sort((a,b) => a-b), [...ids].sort((a,b) => a-b));
    await page.keyboard.press('Delete');
    await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-selected-ids')).length === 0);
};
