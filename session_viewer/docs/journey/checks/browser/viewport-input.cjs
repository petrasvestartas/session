const assert = require('node:assert/strict');

module.exports = async (page, {command, drawing}) => {
    const canvas = page.locator('canvas'), viewport = page.viewportSize();
    const settle = () => page.waitForTimeout(160);
    const reset = async () => {
        await command(page, 'View Reset');
        await command(page, 'View Isometric');
        await page.keyboard.press('Escape');
        await settle();
    };
    await reset();
    const initial = await drawing(page);
    await canvas.evaluate(c => c.addEventListener('pointerdown', e => { window.__coursePointer = e.pointerId; }));
    const box = await canvas.boundingBox();
    const start = [box.x + box.width * .6, box.y + box.height * .35];
    const press = async (button = 'right') => {
        await page.mouse.move(...start);
        await page.mouse.down({button});
        await settle();
        assert(await canvas.evaluate(c => c.hasPointerCapture(window.__coursePointer)), 'Accepted pointer is captured');
    };
    const stopped = async () => {
        const before = await drawing(page);
        await page.mouse.move(start[0] + 30, start[1] + 25, {steps: 4});
        await settle();
        assert.equal(await drawing(page), before, 'Cancelled or released pointer cannot keep orbiting');
        await page.mouse.up({button: 'right'});
    };
    await press();
    await page.mouse.move(-20, start[1] + 20, {steps: 5});
    await settle();
    assert.notEqual(await drawing(page), initial, 'Captured drag orbits outside the canvas');
    await page.mouse.up({button: 'right'});
    await settle();
    assert(!await canvas.evaluate(c => c.hasPointerCapture(window.__coursePointer)));
    await stopped();
    for (const cancel of ['capture', 'pointercancel', 'blur', 'escape', 'resize']) {
        await press();
        await page.mouse.move(start[0] + 15, start[1] + 10);
        if (cancel === 'capture') await canvas.evaluate(c => c.releasePointerCapture(window.__coursePointer));
        if (cancel === 'pointercancel') await canvas.evaluate(c => c.dispatchEvent(new PointerEvent('pointercancel', {pointerId: window.__coursePointer, bubbles: true})));
        if (cancel === 'blur') await canvas.evaluate(c => c.blur());
        if (cancel === 'escape') await page.keyboard.press('Escape');
        if (cancel === 'resize') await page.setViewportSize({width: viewport.width + 20, height: viewport.height + 20});
        await settle();
        await stopped();
    }
    await page.setViewportSize(viewport);
    await reset();
    const beforeLeft = await drawing(page);
    await press('left');
    await page.mouse.move(start[0] + 25, start[1] + 10);
    await page.mouse.move(...start);
    await page.mouse.up();
    await settle();
    assert.equal(await drawing(page), beforeLeft, 'Returning a drag to its start does not pick');
    await page.mouse.move(...start);
    await page.mouse.wheel(0, -120);
    await settle();
    const zoomed = await drawing(page);
    assert.notEqual(zoomed, beforeLeft, 'Actual wheel input changes the view');
    await command(page, 'Undo');
    const removed = await drawing(page);
    assert.notEqual(removed, zoomed, 'Undo removes the box');
    await command(page, 'Redo');
    assert.equal(await drawing(page), zoomed, 'Redo restores geometry without rewinding zoom');
    const unitFrames = [];
    for (const mode of [0, 1, 2]) {
        await reset();
        await canvas.evaluate((c, mode) => c.dispatchEvent(new WheelEvent('wheel', {
            deltaY: mode === 0 ? -48 : mode === 1 ? -3 : -48 / c.getBoundingClientRect().height,
            deltaMode: mode, bubbles: true, cancelable: true,
        })), mode);
        await settle();
        unitFrames.push(await drawing(page));
    }
    assert.equal(unitFrames[0], unitFrames[1], 'Pixel and line wheel units agree');
    assert.equal(unitFrames[0], unitFrames[2], 'Pixel and page wheel units agree');
    const ui = JSON.parse(await canvas.getAttribute('data-command-ui'));
    const [left, top, right, bottom] = ui.controls.find(c => c.key === 'command/input').rect;
    const beforeDock = await drawing(page);
    await page.mouse.move(box.x + (left + right) / 2, box.y + (top + bottom) / 2);
    await page.mouse.down({button: 'right'});
    await page.mouse.move(box.x + (left + right) / 2 + 15, box.y + (top + bottom) / 2);
    await page.mouse.up({button: 'right'});
    await settle();
    assert.equal(await drawing(page), beforeDock, 'Dock-owned pointer events do not orbit');
    await reset();
    await page.mouse.move(...start);
    await page.mouse.wheel(0, -120);
    await command(page, 'Pan Right');
};
