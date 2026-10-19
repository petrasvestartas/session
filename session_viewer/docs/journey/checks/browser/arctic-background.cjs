const assert = require('node:assert/strict');
const {PNG} = require('pngjs');

module.exports = async (original, helpers) => {
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message));
            const url = new URL(original.url()); url.searchParams.set('nogrid', '1'); url.searchParams.set('opacity', '1');
            await page.goto(url.href);
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
                && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const value = async name => JSON.parse(await page.locator('canvas').getAttribute(name));
            const pixels = async () => {
                await page.keyboard.press('Escape'); await page.waitForTimeout(120);
                const data = await require('./canvas-image.cjs')(page.locator('canvas'));
                return PNG.sync.read(Buffer.from(data.split(',')[1], 'base64'));
            };
            const corners = image => [[8,8],[image.width-9,8]].map(([x,y]) =>
                [...image.data.subarray((y*image.width+x)*4,(y*image.width+x)*4+4)]);
            await helpers.command(page, 'Layers Off');
            await helpers.command(page, 'View Outline On');
            await helpers.command(page, 'Arctic Off');
            const before = {objects: await value('data-object-count'), placements: await value('data-row-placements'), uploads: await value('data-gpu-stats')};
            const plain = await pixels();
            const plainDrawing = await helpers.drawing(page);
            assert.deepEqual(corners(plain), [[255,255,255,255],[255,255,255,255]]);
            await helpers.command(page, 'Arctic On');
            const grey = await pixels();
            const encoded = Math.round((1.055*Math.pow(0.94,1/2.4)-0.055)*255);
            for (const pixel of corners(grey)) {
                assert(pixel.slice(0,3).every(channel => Math.abs(channel-encoded)<=1));
                assert.equal(pixel[3],255);
            }
            assert.equal((await value('data-clipping')).arctic, true);
            assert.deepEqual({objects: await value('data-object-count'), placements: await value('data-row-placements'), uploads: await value('data-gpu-stats')}, before);
            await helpers.command(page, 'Arctic Off');
            await pixels();
            assert.equal(await helpers.drawing(page), plainDrawing, 'Turning Arctic off restores the ordinary drawing exactly');
            assert.deepEqual(errors, []);
            console.log(`${helpers.step.id}: current linear Arctic background, sRGB pixels, restoration and geometry reuse passed at DPR${density}`);
        } finally { await context.close(); await original.bringToFront(); }
    }
};
