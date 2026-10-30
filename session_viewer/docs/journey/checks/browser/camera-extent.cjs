const assert = require('node:assert/strict');
const adaptive = require('./adaptive-gpu-reuse.cjs');
const fs = require('node:fs'), path = require('node:path'), {PNG} = require('pngjs');
const output = path.resolve(__dirname, '../../../screenshots/journey/review');
async function scene(page, address, density, helpers) {
    const errors = []; page.on('pageerror', error => errors.push(error.message));
    const capture = !helpers.step || helpers.step.browser_check === 'journey/checks/browser/camera-extent.cjs';
    const url = new URL(address); url.searchParams.set('nogrid', '1'); url.searchParams.set('opacity', '1');
    await page.goto(url.href);
    await page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
    const canvas = page.locator('canvas'), value = name => canvas.getAttribute(name).then(JSON.parse), command = line => helpers.command(page, line);
    for (const line of ['Layers Off', 'Select All', 'Delete', 'Select None', 'CPlane XY', 'Arctic Off', 'View Outline Off', 'Box w0,0,-20000 1000 1000 1000', 'View Reset']) await command(line);
    assert(Math.abs((await value('data-camera-matrix'))[15] - 3000) < 1e-9, 'Reset keeps the physical three-metre distance');
    for (const line of ['View Top', 'View Orthographic', 'View Grid Off', 'Select None']) await command(line);
    const pose = async () => [await value('data-camera-target'), await value('data-camera-frame'), await canvas.getAttribute('data-projection'), (await value('data-camera-matrix')).filter((_, i) => i % 4 !== 2)];
    await adaptive.snapshot(page, {lane: 'mesh', source: 'cad', expectedRows: 1, expectedQualities: 1});
    const before = await pose(), originals = await value('data-cads'), placement = await value('data-row-placements'), uploads = (await value('data-gpu-stats'))[0];
    assert.deepEqual(before[0], [0, 0, 0]); assert.equal(before[2], 'Orthographic'); assert.equal(originals.length, 1);
    const owner = originals[0].id;
    const matrix = await value('data-render-matrix'), origin = await value('data-render-origin'), world = [0, 250, -19000];
    const local = world.map((v, i) => v - origin[i]), clip = [0, 1, 2, 3].map(row => local.reduce((sum, v, axis) => sum + matrix[axis * 4 + row] * v, matrix[12 + row]));
    assert(clip[2] / clip[3] > 0 && clip[2] / clip[3] < 1, 'Late original must be inside the grown depth range without Fit');
    const location = [(clip[0] / clip[3] + 1) / 2, (1 - clip[1] / clip[3]) / 2], bounds = await canvas.boundingBox();
    const png = PNG.sync.read(Buffer.from((await require('./canvas-image.cjs')(canvas)).split(',')[1], 'base64'));
    const pixel = (x, y) => [...png.data.subarray((y * png.width + x) * 4, (y * png.width + x) * 4 + 3)];
    const solid = pixel(Math.floor(location[0] * png.width), Math.floor(location[1] * png.height)), background = pixel(8, 8);
    assert(Math.max(...solid.map((v, i) => Math.abs(v - background[i]))) >= 16, 'The deep original has real contrasting face pixels');
    const picture = await helpers.drawing(page), submitted = (await value('data-pick-gpu'))[3];
    await page.keyboard.press('Escape'); await page.mouse.click(bounds.x + location[0] * bounds.width, bounds.y + location[1] * bounds.height);
    await page.waitForFunction(({owner, submitted}) => {const c = document.querySelector('canvas'), picks = JSON.parse(c.getAttribute('data-pick-gpu')); return JSON.parse(c.getAttribute('data-selected-ids')).includes(owner) && picks[3] > submitted && picks[0] === 0 && picks[1] === 0;}, {owner, submitted});
    await command('Select None'); assert.equal(await helpers.drawing(page), picture);
    for (const line of ['Zoom In', 'Zoom Out', 'Select None']) await command(line);
    assert.deepEqual(await pose(), before); assert.equal(await helpers.drawing(page), picture, 'Camera-only zoom restores exact original pixels');
    for (const line of ['Select All', 'Move 1000,0,0', 'Select None']) await command(line);
    const moved = await value('data-row-placements'), movedPicture = await helpers.drawing(page);
    assert.notDeepEqual(moved, placement); assert.notEqual(movedPicture, picture); assert.deepEqual(await pose(), before);
    fs.mkdirSync(output, {recursive: true});
    if (capture) await page.screenshot({path: path.join(output, `${helpers.step?.id || 'camera-extent'}-moved-dpr${density}.png`)});
    await command('Undo'); await command('Select None');
    assert.deepEqual(await value('data-row-placements'), placement); assert.equal(await helpers.drawing(page), picture, 'Undo preserves the original view while extent stays grown');
    await command('Redo'); await command('Select None'); assert.deepEqual(await value('data-row-placements'), moved); assert.equal(await helpers.drawing(page), movedPicture);
    await command('Undo'); await command('Select None'); assert.equal(await helpers.drawing(page), picture);
    assert.deepEqual(await pose(), before); assert.deepEqual(await value('data-cads'), originals); if (!(await adaptive.snapshot(page, {lane: 'mesh', source: 'cad', expectedRows: 1, expectedQualities: 1})).adaptive) assert.equal((await value('data-gpu-stats'))[0], uploads);
    assert.deepEqual(errors, []);
    if (capture) await page.screenshot({path: path.join(output, `${helpers.step?.id || 'camera-extent'}-dpr${density}.png`)});
    console.log(`Cached camera extent: real below-previous-range original pixels and GPU pick without Fit, unchanged pose, camera-only zoom restoration, Move/Undo/Redo, original sources and geometry reuse passed at DPR ${density}`);
    return {density, pose: before, originals, placement, uploads, depth: clip[2] / clip[3], solid, background};
}
module.exports = async (original, helpers) => {
    await require('./camera-analytic-depth-evidence.cjs')(original, helpers);
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
