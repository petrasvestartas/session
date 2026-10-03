const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const {PNG} = require('pngjs');
module.exports = async (page, {step}) => {
    const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
    const image = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
    const scene = PNG.sync.read(fs.readFileSync(path.resolve('docs/screenshots/journey/03-triangle-browser.png')));
    assert.deepEqual([image.width, image.height], [scene.width, scene.height]);
    const above = image.width * (image.height - 40) * 4;
    assert.deepEqual(image.data.subarray(0, above), scene.data.subarray(0, above), 'Painting the label must preserve every scene pixel above the dock');
    const pixel = (x, y) => [...image.data.subarray((y * image.width + x) * 4, (y * image.width + x) * 4 + 4)];
    assert.deepEqual(pixel(image.width / 2, image.height - 15), [255, 255, 255, 255], 'The command panel must be opaque white');
    let ink = 0;
    for (let y = image.height - 27; y < image.height - 3; y++) for (let x = 6; x < 100; x++) {
        if (pixel(x, y).slice(0, 3).every(channel => channel < 100)) ink++;
    }
    assert(ink > 50, 'The embedded Noto font must produce visible Command label ink');
    console.log(`${step.id}: actual Noto label ink, white panel and exact scene preservation passed`);
};
