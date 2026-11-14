const assert = require('node:assert/strict');
const fs = require('node:fs');
const {PNG} = require('pngjs');

module.exports = async (page, helpers) => {
    const {command, drawing, step} = helpers;
    await require('./shortcuts.cjs')(page, helpers);

    await page.reload();
    await page.waitForFunction(status => document.querySelector('#status')?.textContent === status, step.browser_status);
    await page.waitForTimeout(200);
    const empty = await drawing(page);
    const choosing = page.waitForEvent('filechooser');
    await command(page, 'Open');
    await (await choosing).setFiles({name: 'sample.pb', mimeType: 'application/octet-stream',
        buffer: fs.readFileSync(`target/course-${step.id}/sample.pb`)});
    await page.waitForFunction(() => document.querySelector('#status')?.textContent === 'File imported. Undo removes the entire import.');
    await page.waitForTimeout(200);
    const imported = await drawing(page);
    assert.notEqual(imported, empty, 'Import must display the frame');
    await command(page, 'Undo');
    assert.equal(await drawing(page), empty, 'One undo removes the complete imported frame');
    await command(page, 'Redo');
    assert.equal(await drawing(page), imported, 'Redo restores the complete imported frame');
    await command(page, 'View Isometric');

    await require('./import.cjs')(page, helpers);
    const projections = [];
    for (const projection of ['Perspective', 'Orthographic']) {
        await command(page, 'View ' + projection);
        await command(page, 'Fit');
        const fitted = await drawing(page);
        await command(page, 'Fit');
        assert.equal(await drawing(page), fitted, projection + ' Fit is stable');
        await command(page, 'Pan Right');
        assert.notEqual(await drawing(page), fitted, projection + ' pan moves the picture');
        await command(page, 'Fit');
        assert.equal(await drawing(page), fitted, projection + ' Fit recovers the picture');
        await command(page, 'Undo');
        assert.notEqual(await drawing(page), fitted, projection + ' Fit leaves import undoable');
        await command(page, 'Redo');
        assert.equal(await drawing(page), fitted, projection + ' redo retains the fitted view');

        const canvas = page.locator('canvas');
        const url = await require('./canvas-image.cjs')(canvas);
        const {data, width, height} = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
        const box = await canvas.boundingBox();
        const sceneHeight = height - Math.ceil(34 * width / box.width);
        let pixels = 0;
        for (let y = 0; y < sceneHeight; y++) for (let x = 0; x < width; x++) {
            const at = (y * width + x) * 4;
            if (data[at] === data[0] && data[at + 1] === data[1] && data[at + 2] === data[2]) continue;
            assert(x > width * 0.04 && x < width * 0.96 && y > height * 0.04 && y < height * 0.96,
                projection + ' fitting leaves space around the geometry');
            pixels++;
        }
        assert(pixels > 2000, projection + ' fitting retains visible geometry');
        projections.push(fitted);
    }
    assert.notEqual(projections[0], projections[1], 'Perspective and orthographic projection differ visibly');
    console.log(`${step.id}: file races, transactional import, both projections, fitting and keyboard ownership passed`);
};
