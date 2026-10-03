const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const {PNG} = require('pngjs');
module.exports = async (page) => {
    const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
    const image = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
    const previous = PNG.sync.read(fs.readFileSync(path.resolve('docs/screenshots/journey/03b-memory-browser.png')));
    assert.deepEqual([image.width, image.height], [previous.width, previous.height]);
    assert.deepEqual(image.data, previous.data, 'Preparing dock helpers must leave the connected field and scene unchanged');
    console.log('Complete connected field and scene remain unchanged.');
};
