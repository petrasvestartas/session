const assert = require('node:assert/strict');
const path = require('node:path');
const {PNG} = require('pngjs');

module.exports = async (original, helpers) => {
    await require('./curve-input.cjs')(original, helpers);
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message)); await page.goto(original.url());
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
                && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const value = async name => JSON.parse(await page.locator('canvas').getAttribute(name));
            const quiet = async () => { await page.keyboard.press('Escape'); await page.waitForTimeout(120); };
            const colours = async mode => {
                await quiet(); const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
                const image = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64')), m = await value('data-camera-matrix');
                const sample = x => {
                    const w = m[3] * x + m[15], sx = (m[0] * x + m[12]) / w, sy = (m[1] * x + m[13]) / w;
                    const col = Math.floor((sx + 1) * image.width / 2), row = Math.floor((1 - sy) * image.height / 2);
                    return [...image.data.subarray((row * image.width + col) * 4, (row * image.width + col) * 4 + 3)];
                };
                const left = sample(-0.35), right = sample(0.35), middle = sample(0);
                if (mode === 'Object') { assert.deepEqual(left, right); assert(left[0] > left[1] && left[1] > left[2] + 50); }
                else if (mode === 'Points') {
                    assert(middle[1] > 180 && middle[0] < 30 && middle[2] < 30);
                    assert(left[0] > 100 && left[1] > 100 && left[2] < 30); assert(right[1] > 100 && right[2] > 100 && right[0] < 30);
                } else {
                    assert(left[0] > 180 && left[1] < 30 && left[2] < 30); assert(right[2] > 180 && right[0] < 30 && right[1] < 30);
                }
            };
            for (const [mode, command] of [['Object', 'Example Object Color'], ['Points', 'Example Point Colors'], ['Faces', 'Example Face Colors']]) {
                await helpers.command(page, 'Close'); await helpers.command(page, 'View Reset'); await quiet(); const empty = await helpers.drawing(page);
                await helpers.command(page, command); await colours(mode); const expected = await helpers.drawing(page), uploads = await value('data-gpu-stats');
                for (const projection of ['View Perspective', 'View Orthographic']) {
                    await helpers.command(page, 'View Reset'); await helpers.command(page, projection); await colours(mode);
                    await helpers.command(page, 'Zoom In'); await colours(mode); assert.deepEqual(await value('data-gpu-stats'), uploads);
                }
                await helpers.command(page, 'View Reset'); await helpers.command(page, 'Undo'); await quiet(); assert.equal(await helpers.drawing(page), empty);
                await helpers.command(page, 'Redo'); await colours(mode); assert.equal(await helpers.drawing(page), expected);
                const pending = page.waitForEvent('download'); await helpers.command(page, 'Save'); const download = await pending;
                const file = path.resolve('target', `course-${helpers.step.id}`, `colour-${density}-${mode}.session`); await download.saveAs(file);
                await helpers.command(page, 'Close'); const chooser = page.waitForEvent('filechooser'); await helpers.command(page, 'Open');
                await (await chooser).setFiles(file); await page.waitForFunction(() => document.getElementById('status')?.textContent === 'File imported. Undo removes the entire import.');
                await colours(mode); assert.equal(await helpers.drawing(page), expected, 'The downloaded original mode must reconstruct identical colours');
                await helpers.command(page, 'Close'); assert.deepEqual(await value('data-gpu-usage'), [0, 0, 0, 0]);
            }
            assert.deepEqual(errors, []);
        } finally { await context.close(); await original.bringToFront(); }
    }
    console.log(`${helpers.step.id}: actual object/point/face colours, camera reuse, history and editable download/reopen passed at DPR1/2`);
};
