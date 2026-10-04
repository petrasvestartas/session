const assert = require('node:assert/strict');
const {PNG} = require('pngjs');

module.exports = async (page, {command, drawing, step}) => {
    const centre = async () => {
        const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
        const image = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
        const offset = (Math.floor(image.height / 2) * image.width + Math.floor(image.width / 2)) * 4;
        return [...image.data.subarray(offset, offset + 3)];
    };
    const colour = async expected => {
        const pixel = await centre();
        assert(pixel.every((value, i) => Math.abs(value - expected[i]) <= 2),
            `Perspective centre: expected ${expected}, received ${pixel}`);
    };
    const clickCentre = async () => {
        const canvas = page.locator('canvas');
        const box = await canvas.boundingBox();
        await canvas.click({position: {x: box.width / 2, y: box.height / 2}});
        await page.waitForTimeout(180);
    };
    const initial = await drawing(page);
    await colour([63, 218, 218]);
    await command(page, 'Zoom In');
    assert.notEqual(await drawing(page), initial, 'Perspective zoom must change the drawing');
    await colour([63, 218, 218]);
    if (step.id === '15-perspective') {
        await clickCentre();
        const selected = await centre();
        assert(selected[0] > 200 && selected[1] > 200 && selected[2] < 130,
            'The nearest turquoise surface must become yellow after the actual canvas click');
        await command(page, 'Delete');
        await colour([243, 137, 179]);
        await command(page, 'Undo');
        await colour([63, 218, 218]);
        await command(page, 'Redo');
        await colour([243, 137, 179]);
        await command(page, 'Undo');
        await colour([63, 218, 218]);
    } else {
        const before = await drawing(page);
        await clickCentre();
        assert.equal(await drawing(page), before, 'Canvas selection must remain paused before the ray query');
    }
    await command(page, 'View Reset');
    assert.equal(await drawing(page), initial, 'Reset must restore the original perspective drawing and aspect');
    if (step.id === '15-perspective') {
        await clickCentre();
        const selected = await centre();
        assert(selected[0] > 200 && selected[1] > 200 && selected[2] < 130,
            'Restored default-camera picking must still select turquoise');
    }
};
