const assert = require('node:assert/strict');

module.exports = async (page, ids) => {
    const ui = JSON.parse(await page.locator('canvas').getAttribute('data-command-ui'));
    if (ui.focused) {
        const input = ui.controls.find(control => control.key === 'command/input'); assert(input);
        const box = await page.locator('canvas').boundingBox();
        const local = [input.rect[0]-20,(input.rect[1]+input.rect[3])/2];
        assert(local[0]>0 && local[0]<input.rect[0] && local[1]>ui.top && local[1]<box.height,'Command label is inside owned dock, outside input');
        const at = [box.x+local[0],box.y+local[1]];
        await page.mouse.move(...at);
        await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
        await page.mouse.down();
        await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
        await page.mouse.up();
    }
    await page.waitForFunction(() => !JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).focused);
    const selected = async () => JSON.parse(await page.locator('canvas').getAttribute('data-selected-ids'));
    assert.deepEqual((await selected()).sort((a,b) => a-b), [...ids].sort((a,b) => a-b));
    await page.keyboard.press('Delete');
    await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-selected-ids')).length === 0);
};
