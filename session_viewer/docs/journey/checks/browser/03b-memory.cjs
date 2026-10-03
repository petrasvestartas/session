const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const {PNG} = require('pngjs');
module.exports = async (page) => {
    const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
    const image = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
    const previous = PNG.sync.read(fs.readFileSync(path.resolve('docs/screenshots/journey/03a-panel-browser.png')));
    assert.deepEqual([image.width, image.height], [previous.width, previous.height]);
    const above = image.width * (image.height - 40) * 4;
    assert.deepEqual(image.data.subarray(0, above), previous.data.subarray(0, above), 'Model ownership must preserve every scene pixel');
    let changed = 0;
    for (let i = above; i < image.data.length; i += 4) {
        if (!image.data.subarray(i, i + 4).equals(previous.data.subarray(i, i + 4))) changed++;
    }
    assert(changed > 100, 'The connected model status must change the visible field hint');
    console.log('Connected model changes the field hint and preserves the scene.');
};
