const assert = require('node:assert/strict');
const {PNG} = require('pngjs');
const canvasImage = require('./canvas-image.cjs');

module.exports = async (page, {command, drawing}) => {
    const canvas = page.locator('canvas');
    async function centre(expected) {
        const url = await canvasImage(canvas);
        const image = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
        const offset = (Math.floor(image.height / 2) * image.width + Math.floor(image.width / 2)) * 4;
        const actual = [...image.data.subarray(offset, offset + 3)];
        assert(actual.every((value, i) => Math.abs(value - expected[i]) <= 2),
            `Centre pixel ${actual} must match visible surface ${expected}`);
    }
    await command(page, 'View Reset');
    await centre([63, 218, 218]);
    const box = await canvas.boundingBox();
    await canvas.click({position: {x:box.width / 2,y:box.height / 2}});
    await centre([255, 225, 63]);
    await command(page, 'Delete');
    await centre([243, 137, 179]);
    await command(page, 'Undo');
    await centre([63, 218, 218]);
    await command(page, 'Redo');
    await centre([243, 137, 179]);
    await command(page, 'Undo');
    const restored = await drawing(page);
    await command(page, 'Example Triangle');
    const branch = await drawing(page);
    assert.notEqual(branch, restored, 'A new document edit must be visible');
    await command(page, 'Redo');
    assert.equal(await drawing(page), branch, 'A new edit must discard the redo branch');
    await command(page, 'Orbit Right');
    assert.notEqual(await drawing(page), branch, 'Orbit must change the displayed camera');
    await command(page, 'View Reset');
    assert.equal(await drawing(page), branch, 'Camera reset must preserve document contents');
    await command(page, 'Undo');
    assert.equal(await drawing(page), restored, 'Camera commands must not enter document history');
    await command(page, 'View Isometric');
};
