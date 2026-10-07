const assert = require('node:assert/strict');

module.exports = async (page, {command, drawing, step}) => {
    const ui = async () => JSON.parse(await page.locator('canvas').getAttribute('data-command-ui'));
    const canvas = async () => {
        await page.keyboard.press('Escape'); await page.waitForTimeout(120);
        assert.equal((await ui()).focused, false, 'Escape releases command text focus');
    };
    const focused = async () => {
        const control = (await ui()).controls.find(row => row.key === 'command/input');
        const box = await page.locator('canvas').boundingBox();
        await page.mouse.click(box.x + (control.rect[0] + control.rect[2]) / 2,
            box.y + (control.rect[1] + control.rect[3]) / 2);
    };
    const selected = await drawing(page);
    await command(page, 'Pan Right'); await canvas();
    assert.notEqual(await drawing(page), selected);
    await page.keyboard.press('f'); await page.waitForTimeout(150);
    assert.equal(await drawing(page), selected, 'Canvas F fits the selected object');
    assert.match((await ui()).history.at(-1), /^> Fit\n/);

    await command(page, 'Example Triangle'); const changed = await drawing(page);
    assert.notEqual(changed, selected); await canvas();
    await page.keyboard.press('Control+z'); await page.waitForTimeout(150);
    assert.equal(await drawing(page), selected, 'Ctrl+Z uses existing document history');
    await page.keyboard.press('Control+y'); await page.waitForTimeout(150);
    assert.equal(await drawing(page), changed, 'Ctrl+Y restores the edit');
    await page.keyboard.press('Control+z'); await page.waitForTimeout(150);
    await page.keyboard.press('Control+Shift+z'); await page.waitForTimeout(150);
    assert.equal(await drawing(page), changed, 'Ctrl+Shift+Z also redoes');
    await page.keyboard.press('Meta+z'); await page.waitForTimeout(150);
    assert.equal(await drawing(page), selected, 'Cmd uses the same history policy');
    await page.keyboard.press('Meta+Shift+z'); await page.waitForTimeout(150);
    assert.equal(await drawing(page), changed);

    await focused(); await page.keyboard.type('fhs'); await page.waitForTimeout(120);
    assert.equal((await ui()).command.toLowerCase(), 'fhs', 'Shortcut letters remain focused text through completion');
    const before = await drawing(page), history = (await ui()).history;
    await page.keyboard.press('Control+z'); await page.waitForTimeout(150);
    assert.equal(await drawing(page), before, 'A text-owned undo leaves the document unchanged');
    assert.deepEqual((await ui()).history, history, 'Text undo does not echo a document command');

    await canvas(); await command(page, 'Pan Right'); await canvas();
    const panned = await drawing(page), echo = (await ui()).history;
    for (const options of [{altKey: true}, {repeat: true}, {isComposing: true}]) {
        await page.locator('canvas').evaluate((canvas, options) => canvas.dispatchEvent(
            new KeyboardEvent('keydown', {key: 'f', bubbles: true, ...options})), options);
        await page.waitForTimeout(80);
        await canvas();
        assert.equal(await drawing(page), panned, 'Modified, repeated and composing F cannot request Fit');
        assert.deepEqual((await ui()).history, echo);
    }
    await command(page, 'Select Next'); await command(page, 'Select Next');
    // Clearing selection with an actual empty click makes F use whole-scene Fit.
    const box = await page.locator('canvas').boundingBox();
    await page.mouse.click(box.x + 4, box.y + 4); await page.waitForTimeout(120);
    await command(page, 'Fit'); const whole = await drawing(page);
    await command(page, 'Pan Right'); await canvas();
    await page.keyboard.press('Shift+f'); await page.waitForTimeout(150);
    assert.equal(await drawing(page), whole, 'Uppercase canvas F fits all geometry without a selection');
    console.log(`${step.id}: canvas Fit, platform history and text ownership passed`);
};
