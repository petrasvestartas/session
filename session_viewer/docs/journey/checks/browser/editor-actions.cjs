const assert = require('node:assert/strict');
const {PNG} = require('pngjs');

module.exports = async (page, {command, drawing}) => {
    const initial = await drawing(page);
    await command(page, 'Zoom In');
    assert.notEqual(await drawing(page), initial, 'An editor view action must change the picture');
    await command(page, 'View Reset');
    await command(page, 'View Isometric');
    assert.equal(await drawing(page), initial, 'Editor reset must retain the window aspect');
    await command(page, 'Undo');
    const removed = await drawing(page);
    assert.notEqual(removed, initial, 'Undo must remove the added box and repair selection');
    await command(page, 'Delete');
    assert.equal(await drawing(page), removed, 'Empty Delete must retain the document');
    await command(page, 'Redo');
    assert.notEqual(await drawing(page), removed, 'Empty Delete must preserve Redo');
    for (let i = 0; i < 3; i++) await command(page, 'Select Next');
    assert.equal(await drawing(page), initial, 'Redo and reselect must restore the same display');
    await command(page, 'Pan Right');
    await command(page, 'Select Next');
    const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
    const image = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
    const grey = [];
    for (let y = 0; y < image.height - 34; y++) for (let x = 0; x < image.width; x++) {
        const i = (y * image.width + x) * 4;
        const [r, g, b] = image.data.subarray(i, i + 3);
        if (r > 80 && r < 245 && Math.abs(r - g) < 2 && Math.abs(g - b) < 2) grey.push([x, y]);
    }
    assert(grey.length > 1000, 'The moved unselected box must remain visible');
    const [x, y] = grey[Math.floor(grey.length / 2)];
    const canvas = page.locator('canvas');
    const box = await canvas.boundingBox();
    await canvas.click({position: {x: (x + .5) * box.width / image.width, y: (y + .5) * box.height / image.height}});
    await page.waitForTimeout(180);
    const selected = PNG.sync.read(Buffer.from((await canvas.evaluate(c => c.toDataURL())).split(',')[1], 'base64'));
    let yellow = 0;
    for (let i = 0; i < selected.width * (selected.height - 34) * 4; i += 4) {
        const [r, g, b] = selected.data.subarray(i, i + 3);
        if (r > 160 && g > 150 && b < 120) yellow++;
    }
    assert(yellow > 1000, 'Picking through the moved editor camera must highlight the box');
    await command(page, 'Pan Left');
    assert.equal(await drawing(page), initial, 'Picking and view actions must preserve the restored document');
};
