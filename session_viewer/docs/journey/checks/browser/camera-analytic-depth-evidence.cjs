const assert = require('node:assert/strict');
const {PNG} = require('pngjs');
async function scene(page, address, density, helpers) {
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    const url = new URL(address); url.searchParams.set('nogrid', '1'); url.searchParams.set('opacity', '1');
    await page.goto(url.href);
    await page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
    const canvas = page.locator('canvas'), value = name => canvas.getAttribute(name).then(JSON.parse), command = line => helpers.command(page, line);
    for (const line of ['Layers Off', 'Select All', 'Delete', 'Select None', 'CPlane XY', 'Arctic Off', 'View Outline Off', 'Background', 'Box w0,0,-2000 1000 1000 1000', 'View Reset', 'View Top', 'Zoom Out', 'Zoom Out', 'Select None', 'View Grid Off']) await command(line);
    const cads = await value('data-cads'), placement = await value('data-row-placements'), matrix = await value('data-render-matrix'), target = await value('data-camera-target'), uploads = (await value('data-gpu-stats'))[0];
    assert.equal(cads.length, 1); assert.deepEqual(target, [0, 0, 0]);
    const owner = cads[0].id, bounds = await canvas.boundingBox(), origin = await value('data-render-origin');
    const point = [0, 250, -1000].map((v, i) => v - origin[i]);
    const clip = [0, 1, 2, 3].map(row => point.reduce((sum, v, axis) => sum + matrix[axis * 4 + row] * v, matrix[12 + row]));
    assert(clip[2] / clip[3] > 0 && clip[2] / clip[3] < 1, 'The below-ground original remains inside the hidden-grid depth range');
    const physical = await canvas.evaluate(c => [c.width, c.height]);
    const location = [(clip[0] / clip[3] + 1) / 2, (1 - clip[1] / clip[3]) / 2];
    const png = async () => PNG.sync.read(Buffer.from((await require('./canvas-image.cjs')(canvas)).split(',')[1], 'base64'));
    const pixel = (image, x, y) => [...image.data.subarray((y * image.width + x) * 4, (y * image.width + x) * 4 + 3)];
    const x = Math.floor(location[0] * physical[0]), y = Math.floor(location[1] * physical[1]);
    const off = await helpers.drawing(page), image = await png(), solid = pixel(image, x, y);
    assert.notDeepEqual(solid, pixel(image, 8, 8), 'The below-ground original actually draws against the contrasting background');
    await command('View Grid On');
    assert.deepEqual(await value('data-render-matrix'), matrix, 'Grid drawing visibility cannot change camera depth');
    assert.notEqual(await helpers.drawing(page), off, 'Real grid lines appear');
    assert.deepEqual(pixel(await png(), x, y), solid, 'The original remains opaque above the background grid pass');
    await command('View Grid Off');
    assert.deepEqual(await value('data-render-matrix'), matrix);
    assert.equal(await helpers.drawing(page), off, 'Hiding the grid restores exact original pixels');
    const submitted = (await value('data-pick-gpu'))[3];
    await page.keyboard.press('Escape'); await page.mouse.click(bounds.x + location[0] * bounds.width, bounds.y + location[1] * bounds.height);
    await page.waitForFunction(({owner, submitted}) => {const c = document.querySelector('canvas'), picks = JSON.parse(c.getAttribute('data-pick-gpu')); return JSON.parse(c.getAttribute('data-selected-ids')).includes(owner) && picks[3] > submitted && picks[0] === 0 && picks[1] === 0;}, {owner, submitted});
    await command('Select None');
    assert.equal(await helpers.drawing(page), off);
    assert.deepEqual(await value('data-cads'), cads);
    assert.deepEqual(await value('data-row-placements'), placement);
    assert.equal((await value('data-gpu-stats'))[0], uploads, 'Grid visibility reuses original geometry');
    assert.deepEqual(errors, []);
    console.log(`Current construction-plane depth: below-ground original after Reset, identical hidden/shown grid matrices, real grid pixels, exact restoration, original GPU selection/source/placement and geometry reuse passed at DPR ${density}`);

    return {density, owner, target, matrix, original: cads, placement, uploads, solid, physical};
}
module.exports = async (original, helpers) => {
    await require('./camera-analytic-ray-evidence.cjs')(original, helpers);
    const address = original.url();
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try { await scene(await context.newPage(), address, density, helpers); }
        finally { await context.close(); await original.bringToFront(); }
    }
    await original.setViewportSize({width: 960, height: 640});
    return scene(original, address, await original.evaluate(() => devicePixelRatio), helpers);
};
module.exports.feature = scene;
