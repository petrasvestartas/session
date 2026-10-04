const assert = require('node:assert/strict');

module.exports = async (page, {command, drawing}) => {
    const canvas = page.locator('canvas');
    const initial = await drawing(page);
    const ui = async () => JSON.parse(await canvas.getAttribute('data-command-ui'));
    await page.mouse.click(8, 8);
    const untouched = await drawing(page);
    for (const letter of 'abcdefghijklmnopqrstuvwxyz') {
        await page.keyboard.type(letter);
        await page.waitForFunction(letter => JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).command.toLowerCase().startsWith(letter), letter);
        await page.keyboard.press('Escape');
        await page.waitForTimeout(100);
        assert.equal(await drawing(page), untouched, `The letter ${letter} must only edit the command field`);
        await page.mouse.click(8, 8);
    }
    for (const key of ['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown', 'Delete', 'Backspace', 'ControlOrMeta+z', 'ControlOrMeta+y']) {
        await page.keyboard.press(key);
        await page.waitForTimeout(100);
        assert.equal(await drawing(page), untouched, `${key} must not run a feature shortcut`);
        await page.keyboard.press('Escape');
        await page.mouse.click(8, 8);
    }
    const history = (await ui()).history.length;
    await page.keyboard.type('P');
    await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).command.toLowerCase().startsWith('p'));
    await page.keyboard.type('an Right');
    await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).command === 'Pan Right');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(140);
    assert.equal((await ui()).history.length, history + 1, 'Immediate typing must submit exactly one command');
    assert.equal((await ui()).command, '');
    assert.notEqual(await drawing(page), untouched, 'The typed Pan Right command must move the view');
    await command(page, 'Pan Left');
    assert.equal(await drawing(page), untouched, 'The opposite typed pan restores the view');

    let equivalent;
    for (const mode of [0, 1, 2]) {
        await command(page, 'View Reset');
        await command(page, 'View Isometric');
        const before = await drawing(page);
        const prevented = await canvas.evaluate((c, mode) => {
            const r = c.getBoundingClientRect();
            const deltaY = mode === 0 ? 48 : mode === 1 ? 3 : 48 / r.height;
            return !c.dispatchEvent(new WheelEvent('wheel', {deltaY, deltaMode: mode, clientX: r.width / 2, clientY: r.height / 2, bubbles: true, cancelable: true}));
        }, mode);
        assert(prevented, 'Handled wheel input must prevent page scrolling');
        await page.waitForTimeout(140);
        const after = await drawing(page);
        assert.notEqual(after, before, 'Each normalized wheel mode must zoom');
        if (equivalent) assert.equal(after, equivalent, 'Equivalent pixel, line and page deltas must draw the same zoom');
        equivalent = after;
        assert.equal(await page.evaluate(() => window.scrollY), 0);
    }
    const beforeWheel = await drawing(page);
    await page.mouse.move(450, 300);
    await page.mouse.wheel(0, -120);
    await page.waitForTimeout(180);
    assert.notEqual(await drawing(page), beforeWheel, 'Actual mouse wheel input must zoom');
    assert.equal(await page.evaluate(() => window.scrollY), 0);

    await page.reload();
    await page.waitForFunction(() => document.querySelector('#status')?.textContent === 'Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.');
    await command(page, 'Example Box');
    await command(page, 'View Isometric');
    const box = await canvas.boundingBox();
    await page.mouse.move(box.width / 2, box.height / 2);
    await page.mouse.wheel(0, -120);
    await page.waitForTimeout(200);
    await command(page, 'Pan Right');
    assert.equal(await drawing(page), initial, 'Restore the accepted wheel and command view for capture');
};
