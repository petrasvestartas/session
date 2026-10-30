const assert = require('node:assert/strict');
const adaptive = require('./adaptive-gpu-reuse.cjs');
const fs = require('node:fs'), path = require('node:path'), {PNG} = require('pngjs');
const output = path.resolve(__dirname, '../../../screenshots/journey/review');
const capture = helpers => !helpers.step || helpers.step.browser_check === 'journey/checks/browser/camera-navigation.cjs';
const name = helpers => helpers.step?.id || 'camera-navigation';
async function ready(page, address) {
    const url = new URL(address); url.searchParams.set('nogrid', '1'); url.searchParams.set('opacity', '1'); url.searchParams.delete('dpr');
    await page.goto(url.href);
    await page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
}
function data(page) {
    const canvas = page.locator('canvas');
    return {canvas, value: key => canvas.getAttribute(key).then(JSON.parse)};
}
async function project(page, point) {
    const {canvas, value} = data(page), m = await value('data-camera-matrix');
    const clip = [0, 1, 2, 3].map(row => point.reduce((sum, v, axis) => sum + m[axis * 4 + row] * v, m[12 + row]));
    const box = await canvas.boundingBox();
    return [box.x + (clip[0] / clip[3] + 1) * box.width / 2, box.y + (1 - clip[1] / clip[3]) * box.height / 2];
}
async function wheel(page, address, density, helpers) {
    await ready(page, address);
    const {canvas, value} = data(page), command = line => helpers.command(page, line);
    for (const line of ['Layers Off', 'Select All', 'Delete', 'Select None', 'Arctic Off', 'View Outline Off', 'Box w0,0,-500 1000 1000 1000', 'View Reset', 'View Grid Off']) await command(line);
    assert.equal(await canvas.getAttribute('data-projection'), 'Perspective', 'Wheel distance is observed in the declared perspective pose');
    assert.equal(await page.evaluate(() => devicePixelRatio), density);
    const box = await canvas.boundingBox(), physical = await canvas.evaluate(c => [c.width, c.height]);
    assert.equal(physical[0], Math.round(box.width * Math.min(density, 2)), 'Render density is capped independently of actual window DPR');
    await adaptive.snapshot(page, {lane: 'mesh', source: 'cad', expectedRows: 1, expectedQualities: 1});
    const originals = await value('data-cads'), uploads = (await value('data-gpu-stats'))[0];
    const anchor = await project(page, [0, 0, 0]); await page.mouse.move(...anchor);
    await page.evaluate(() => window.addEventListener('wheel', e => window.navigationWheel = {delta: e.deltaY, mode: e.deltaMode, trusted: e.isTrusted}, {capture: true}));
    const before = (await value('data-camera-matrix'))[15], picture = await helpers.drawing(page);
    await page.mouse.wheel(0, 100);
    await page.waitForFunction(before => JSON.parse(document.querySelector('canvas').getAttribute('data-camera-matrix'))[15] !== before, before);
    const observed = await page.evaluate(() => window.navigationWheel), after = (await value('data-camera-matrix'))[15];
    assert.equal(observed.mode, 0); assert.equal(observed.trusted, true);
    assert(Math.abs(observed.delta * density - 100) < 1e-4, 'Record the actual logical delta delivered for the requested physical scroll');
    const physicalAmount = Math.fround(Math.fround(-observed.delta * density) / 100);
    assert(Math.abs(after / before - Math.pow(0.9, physicalAmount)) < 1e-10, 'Real physical pixel wheel uses actual DPR before the precise zoom factor');
    const fixed = await project(page, [0, 0, 0]); assert(Math.hypot(fixed[0] - anchor[0], fixed[1] - anchor[1]) < 1, 'The target-plane original anchor stays under the actual pointer');
    assert.notEqual(await helpers.drawing(page), picture);
    fs.mkdirSync(output, {recursive: true});
    if (capture(helpers)) await page.screenshot({path: path.join(output, `${name(helpers)}-wheel-dpr${density}.png`)});
    for (const line of ['View Reset']) await command(line);
    const lineAnchor = await project(page, [0, 0, 0]), lineBefore = (await value('data-camera-matrix'))[15];
    const dispatch = (delta, mode) => canvas.evaluate((c, input) => c.dispatchEvent(new WheelEvent('wheel', {deltaY: input.delta, deltaMode: input.mode, clientX: input.x, clientY: input.y, bubbles: true, cancelable: true})), {delta, mode, x: lineAnchor[0], y: lineAnchor[1]});
    await dispatch(100, 0);
    await page.waitForFunction(before => JSON.parse(document.querySelector('canvas').getAttribute('data-camera-matrix'))[15] !== before, lineBefore);
    const logicalPixelRatio = (await value('data-camera-matrix'))[15] / lineBefore;
    assert(Math.abs(logicalPixelRatio - Math.pow(0.9, -density)) < 1e-10, 'Synthetic100-logical-pixel protocol uses actual DPR, including3 with render cap2');
    await command('View Reset');
    await dispatch(1, 1);
    await page.waitForFunction(before => JSON.parse(document.querySelector('canvas').getAttribute('data-camera-matrix'))[15] !== before, lineBefore);
    assert(Math.abs((await value('data-camera-matrix'))[15] / lineBefore - 1 / 0.9) < 1e-10, 'Synthetic line protocol keeps its step unit independently of DPR');
    const unchanged = await value('data-camera-matrix');
    for (const [delta, mode] of [[1, 2], [1e100, 0]]) {await dispatch(delta, mode); await page.waitForTimeout(120); assert.deepEqual(await value('data-camera-matrix'), unchanged, 'Page events and amounts overflowing f32 cannot move the camera');}
    assert.deepEqual(await value('data-cads'), originals); if (!(await adaptive.snapshot(page, {lane: 'mesh', source: 'cad', expectedRows: 1, expectedQualities: 1})).adaptive) assert.equal((await value('data-gpu-stats'))[0], uploads);
    console.log(`Current wheel: real pixel/DPR ${density}, render cap ${Math.min(density, 2)}, synthetic logical-pixel/line/page/overflow protocol, cursor anchor, originals and geometry reuse passed`);
    return {density, physical, pixelRatio: after / before, logicalPixelRatio, lineRatio: (await value('data-camera-matrix'))[15] / lineBefore, observed};
}
async function framing(page, address, density, helpers) {
    await ready(page, address);
    const {canvas, value} = data(page), command = line => helpers.command(page, line);
    for (const line of ['Layers Off', 'Select All', 'Delete', 'Select None', 'CPlane XY', 'Arctic Off', 'View Outline Off', 'View Grid Off', 'Box w0,0,-1000 6000 2000 2000', 'Line w-2400,0,1000 w2400,0,1000', 'Select None', 'View Reset', 'View Top', 'View Orthographic']) await command(line);
    await adaptive.snapshot(page, {lane: 'mesh', source: 'cad', expectedRows: 1, expectedQualities: 1});
    const originals = await value('data-cads'), drawing = await value('data-drawing-originals'), placement = await value('data-row-placements'), uploads = (await value('data-gpu-stats'))[0];
    assert.equal(originals.length, 1); assert.equal(drawing.lines.length, 1);
    const landmarks = [[-2400, 0, 1000], [2400, 0, 1000]], bounds = await canvas.boundingBox();
    const visible = async () => {
        const positions = await Promise.all(landmarks.map(p => project(page, p)));
        for (const [x, y] of positions) assert(x > bounds.x && x < bounds.x + bounds.width && y > bounds.y && y < bounds.y + bounds.height, 'Visible orthographic landmarks must remain visible in perspective');
        const png = PNG.sync.read(Buffer.from((await require('./canvas-image.cjs')(canvas)).split(',')[1], 'base64')), scale = png.width / bounds.width;
        for (const [x, y] of positions) {
            let dark = false; const cx = Math.round((x - bounds.x) * scale), cy = Math.round((y - bounds.y) * scale), r = Math.ceil(3 * scale);
            for (let py = Math.max(0, cy - r); py <= Math.min(png.height - 1, cy + r); py++) for (let px = Math.max(0, cx - r); px <= Math.min(png.width - 1, cx + r); px++) {const i = (py * png.width + px) * 4; if (Math.max(png.data[i], png.data[i + 1], png.data[i + 2]) < 180) dark = true;}
            assert(dark, 'Actual original line endpoint pixels accompany the matrix landmark control');
        }
        return positions;
    };
    const initial = await helpers.drawing(page), before = await visible();
    fs.mkdirSync(output, {recursive: true});
    if (capture(helpers)) await page.screenshot({path: path.join(output, `${name(helpers)}-orthographic-dpr${density}.png`)});
    await command('View Perspective'); assert.equal(await canvas.getAttribute('data-projection'), 'Perspective');
    const perspective = await value('data-camera-matrix'), after = await visible(), perspectivePicture = await helpers.drawing(page);
    assert.notEqual(perspectivePicture, initial); assert(perspective[15] > 3000, 'Perspective refits the clipped visible box instead of keeping the too-close orthographic distance');
    await command('View Perspective'); assert.deepEqual(await value('data-camera-matrix'), perspective, 'Requesting the active projection is a framing no-op');
    const plane = await Promise.all([[0, 0, 0], [1000, 0, 0], [0, 500, 0]].map(p => project(page, p)));
    await command('View Orthographic'); assert.equal(await canvas.getAttribute('data-projection'), 'Orthographic');
    const orthographic = await Promise.all([[0, 0, 0], [1000, 0, 0], [0, 500, 0]].map(p => project(page, p)));
    plane.forEach((p, i) => assert(Math.hypot(p[0] - orthographic[i][0], p[1] - orthographic[i][1]) < 1e-6, 'Reverse transition preserves the target-plane framing'));
    for (const line of ['View Reset', 'View Top', 'View Orthographic']) await command(line);
    assert.equal(await helpers.drawing(page), initial, 'Reset restores the exact cropped orthographic original scene');
    await command('View Perspective'); assert.equal(await helpers.drawing(page), perspectivePicture, 'The current framed perspective result repeats exactly');
    assert.deepEqual(await value('data-cads'), originals); assert.deepEqual(await value('data-drawing-originals'), drawing); assert.deepEqual(await value('data-row-placements'), placement); if (!(await adaptive.snapshot(page, {lane: 'mesh', source: 'cad', expectedRows: 1, expectedQualities: 1})).adaptive) assert.equal((await value('data-gpu-stats'))[0], uploads);
    if (capture(helpers)) await page.screenshot({path: path.join(output, `${name(helpers)}-perspective-dpr${density}.png`)});
    console.log(`Current framed projection: real cropped Box/original line endpoints, preserved visible landmarks, current refit, no-op/reverse guards, exact repeated pixels, original placements and geometry reuse passed at DPR ${density}`);
    return {density, before, after, distance: perspective[15], originals, drawing, placement, uploads};
}
async function feature(page, address, density, helpers) {
    const errors = []; page.on('pageerror', error => errors.push(error.message));
    const result = {wheel: await wheel(page, address, density, helpers), framing: await framing(page, address, density, helpers)};
    assert.deepEqual(errors, []); return result;
}
module.exports = async (original, helpers) => {
    await require('./camera-extent.cjs')(original, helpers);
    const address = original.url();
    for (const density of [1, 2, 3]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {await feature(await context.newPage(), address, density, helpers);}
        finally {await context.close(); await original.bringToFront();}
    }
    await original.setViewportSize({width: 960, height: 640});
    return feature(original, address, await original.evaluate(() => devicePixelRatio), helpers);
};
module.exports.feature = feature;
module.exports.wheel = wheel;
module.exports.framing = framing;
