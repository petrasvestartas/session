const assert = require('node:assert/strict');
const adaptive = require('./adaptive-gpu-reuse.cjs');
const {PNG} = require('pngjs');
module.exports = async (original, helpers) => {
    await require('./camera-defaults.cjs')(original, helpers);
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message));
            const url = new URL(original.url());
            for (const [key, value] of [['nogrid', '1'], ['opacity', '1'], ['msaa', '1']]) url.searchParams.set(key, value);
            await page.goto(url.href);
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const canvas = page.locator('canvas'), value = name => canvas.getAttribute(name).then(JSON.parse);
            const command = line => helpers.command(page, line);
            const pose = async () => { for (const line of ['View Reset', 'View Top', 'Zoom Out', 'Zoom Out', 'Select None']) await command(line); };
            for (const line of ['Layers Off', 'Select All', 'Delete', 'Select Object', 'Arctic Off', 'View Outline Off', 'CPlane XY']) await command(line);
            await pose(); await command('View Grid On');
            const png = async () => PNG.sync.read(Buffer.from((await require('./canvas-image.cjs')(canvas)).split(',')[1], 'base64'));
            const project = async point => {
                const m = await value('data-render-matrix'), origin = await value('data-render-origin'), local = point.map((v, i) => v - origin[i]);
                const clip = [0, 1, 2, 3].map(row => local.reduce((sum, v, axis) => sum + m[axis * 4 + row] * v, m[12 + row]));
                const size = await canvas.evaluate(c => [c.width, c.height]);
                return [(clip[0] / clip[3] + 1) * size[0] / 2, (1 - clip[1] / clip[3]) * size[1] / 2];
            };
            const pixel = (image, x, y) => [...image.data.subarray((y * image.width + x) * 4, (y * image.width + x) * 4 + 3)];
            const image = await png(), gray = [196, 196, 196], pink = [245, 144, 195], green = [204, 232, 122];
            const lineWidth = async (point, color, horizontal) => {
                const [x, y] = (await project(point)).map(Math.floor); let count = 0;
                for (let delta = -5; delta <= 5; delta++) if (JSON.stringify(pixel(image, x + (horizontal ? 0 : delta), y + (horizontal ? delta : 0))) === JSON.stringify(color)) count++;
                assert.equal(count, 1, 'Grid floor and axes occupy exactly one physical pixel');
            };
            await lineWidth([1000, 2500, 0], gray, false);
            await lineWidth([2500, 1000, 0], gray, true);
            await lineWidth([2500, 0, 0], pink, true);
            await lineWidth([0, 2500, 0], green, false);
            assert.deepEqual(await value('data-cads'), []);
            const emptyGrid = await helpers.drawing(page);
            await command('View Grid Off'); assert.notEqual(await helpers.drawing(page), emptyGrid);
            await command('View Grid On'); assert.equal(await helpers.drawing(page), emptyGrid);
            const bounds = await canvas.boundingBox();
            for (const point of [[1000, 2500, 0], [0, 2500, 0]]) {
                const [x, y] = await project(point);
                await page.keyboard.press('Escape'); await page.mouse.click(bounds.x + x / density, bounds.y + y / density);
                await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-selected-ids')).length === 0);
                assert.deepEqual(await value('data-cads'), []);
            }
            await command('Background'); await command('Box w0,0,-2000 1000 1000 1000');
            for (const line of ['Select None', 'Fit', 'Zoom Out', 'Zoom Out']) await command(line);
            await adaptive.snapshot(page, {lane: 'mesh', source: 'cad', expectedRows: 1, expectedQualities: 1});
            const sources = await value('data-cads'); assert.equal(sources.length, 1);
            const owner = sources[0].id, uploads = (await value('data-gpu-stats'))[0];
            const projected = (await project([0, 250, 0])).map(Math.floor);
            const axisSamples = [-1, 0, 1].map(dx => [projected[0] + dx, projected[1]]).filter(([x, y]) => JSON.stringify(pixel(image, x, y)) === JSON.stringify(green));
            assert.equal(axisSamples.length, 1, 'Sample the actual one-physical-pixel axis inside the original');
            const [x, y] = axisSamples[0];
            const opaqueGrid = pixel(await png(), x, y);
            await command('View Grid Off'); const plainImage = await png(), opaquePlain = pixel(plainImage, x, y), background = pixel(plainImage, 8, 8);
            assert.notDeepEqual(opaquePlain, green, 'The sampled original is distinct from the grid axis');
            assert.notDeepEqual(opaquePlain, background, 'The sampled original must actually draw against the declared contrasting background');
            assert.deepEqual(opaqueGrid, opaquePlain, 'An opaque original below the grid plane covers the background grid');
            await command('Opacity 0.5'); const glassPlain = pixel(await png(), x, y);
            await command('View Grid On'); const glassGrid = pixel(await png(), x, y);
            assert.notDeepEqual(glassGrid, glassPlain, 'A transparent original reveals the grid behind it');
            const submitted = (await value('data-pick-gpu'))[3], location = await project([0, 250, -1000]);
            await page.keyboard.press('Escape'); await page.mouse.click(bounds.x + location[0] / density, bounds.y + location[1] / density);
            await page.waitForFunction(({owner, submitted}) => {
                const c = document.querySelector('canvas'), picks = JSON.parse(c.getAttribute('data-pick-gpu'));
                return JSON.parse(c.getAttribute('data-selected-ids')).includes(owner) && picks[3] > submitted && picks[0] === 0 && picks[1] === 0;
            }, {owner, submitted});
            await command('Select None'); await command('Opacity 1');
            assert.deepEqual(await value('data-cads'), sources);
            if (!(await adaptive.snapshot(page, {lane: 'mesh', source: 'cad', expectedRows: 1, expectedQualities: 1})).adaptive) assert.equal((await value('data-gpu-stats'))[0], uploads);
            for (const line of ['Select All', 'Delete', 'Background', 'View Grid On']) await command(line);
            await pose(); assert.deepEqual(await value('data-cads'), []);
            assert.equal(await helpers.drawing(page), emptyGrid);
            assert.deepEqual(errors, []);
            console.log(`Physical grid: exact one-pixel floor/axes, current 1000-unit recipe, On/Off restoration, no grid owners, below-plane opaque/glass order, original GPU picks/source reuse and empty restoration passed at actual DPR ${density}`);
        } finally { await context.close(); await original.bringToFront(); }
    }
};
