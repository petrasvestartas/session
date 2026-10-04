const assert = require('node:assert/strict');
const {PNG} = require('pngjs');

module.exports = async (page, {command, drawing}) => {
    const canvas = page.locator('canvas');
    const initial = await drawing(page);
    await canvas.evaluate(c => c.addEventListener('pointerdown', e => { window.__courseGesturePointer = e.pointerId; }));
    const box = await canvas.boundingBox();
    const start = {x: box.width * .6, y: box.height * .4};
    const pressed = async (button = 'right') => {
        await page.mouse.move(start.x, start.y);
        await page.mouse.down({button});
        await page.waitForTimeout(80);
        assert(await canvas.evaluate(c => c.hasPointerCapture(window.__courseGesturePointer)), 'The accepted pointer must be captured');
    };
    const stopped = async () => {
        const before = await drawing(page);
        await page.mouse.move(start.x + 80, start.y + 30, {steps: 4});
        await page.waitForTimeout(130);
        assert.equal(await drawing(page), before, 'Cancelled or finished input must stop camera motion');
        await page.mouse.up({button: 'right'});
    };
    await pressed();
    const beforeOutside = await drawing(page);
    await page.mouse.move(-25, start.y + 25, {steps: 6});
    await page.waitForTimeout(130);
    assert.notEqual(await drawing(page), beforeOutside, 'Capture must deliver orbit outside the canvas');
    await page.mouse.up({button: 'right'});
    await page.waitForTimeout(100);
    assert(!await canvas.evaluate(c => c.hasPointerCapture(window.__courseGesturePointer)), 'Release must end capture');
    await stopped();

    await pressed();
    await page.mouse.move(start.x + 25, start.y + 10);
    await canvas.evaluate(c => c.releasePointerCapture(window.__courseGesturePointer));
    await page.waitForTimeout(120);
    await stopped();

    await pressed();
    await page.mouse.move(start.x + 25, start.y + 10);
    await canvas.evaluate(c => c.dispatchEvent(new PointerEvent('pointercancel', {pointerId: window.__courseGesturePointer, bubbles: true})));
    await page.waitForTimeout(120);
    await stopped();

    await pressed();
    await page.mouse.move(start.x + 25, start.y + 10);
    await canvas.evaluate(c => c.blur());
    await page.waitForTimeout(120);
    await stopped();

    await pressed();
    await page.mouse.move(start.x + 25, start.y + 10);
    await page.setViewportSize({width: 960, height: 800});
    await page.waitForTimeout(180);
    await stopped();
    await page.setViewportSize({width: 900, height: 760});
    await command(page, 'View Reset');
    await command(page, 'View Isometric');

    const unmoved = await drawing(page);
    await pressed('left');
    await page.mouse.move(start.x + 35, start.y + 15);
    await page.mouse.move(start.x, start.y);
    await page.mouse.up();
    await page.waitForTimeout(130);
    assert.equal(await drawing(page), unmoved, 'Returning a left drag to its start must not select');
    const url = await canvas.evaluate(c => c.toDataURL());
    const image = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
    const grey = [];
    for (let y = 0; y < image.height - 34; y++) for (let x = 0; x < image.width; x++) {
        const i = (y * image.width + x) * 4;
        const [r, g, b] = image.data.subarray(i, i + 3);
        if (r > 80 && r < 245 && Math.abs(r - g) < 2 && Math.abs(g - b) < 2) grey.push([x, y]);
    }
    assert(grey.length > 1000, 'The box must remain visible');
    const [x, y] = grey[Math.floor(grey.length / 2)];
    await canvas.click({position: {x: x + .5, y: y + .5}});
    await page.waitForTimeout(130);
    const selected = PNG.sync.read(Buffer.from((await canvas.evaluate(c => c.toDataURL())).split(',')[1], 'base64'));
    let yellow = 0;
    for (let i = 0; i < selected.width * (selected.height - 34) * 4; i += 4) {
        const [r, g, b] = selected.data.subarray(i, i + 3);
        if (r > 160 && g > 150 && b < 120) yellow++;
    }
    assert(yellow > 1000, 'An actual left click must select the box');
    await command(page, 'Select Next');
    const ui = JSON.parse(await canvas.getAttribute('data-command-ui'));
    const [left, top, right, bottom] = ui.controls.find(c => c.key === 'command/input').rect;
    const beforeDock = await drawing(page);
    await page.mouse.move((left + right) / 2, (top + bottom) / 2);
    await page.mouse.down({button: 'right'});
    await page.mouse.move((left + right) / 2 + 20, (top + bottom) / 2);
    await page.mouse.up({button: 'right'});
    await page.waitForTimeout(130);
    assert.equal(await drawing(page), beforeDock, 'Dock-owned pointer input must not orbit');
    await page.reload();
    await page.waitForFunction(() => document.querySelector('#status')?.textContent === 'Right-drag to orbit. Left-click to select. A cancelled drag stops immediately.');
    await command(page, 'Example Box');
    await command(page, 'View Isometric');
    await page.mouse.move(box.width * .5, box.height * .5);
    await page.mouse.down({button: 'right'});
    await page.mouse.move(box.width * .65, box.height * .6, {steps: 5});
    await page.mouse.up({button: 'right'});
    await page.waitForTimeout(130);
    assert.equal(await drawing(page), initial, 'Restore the accepted orbit for the final capture');
};
