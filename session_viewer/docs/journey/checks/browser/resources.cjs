const assert = require('node:assert/strict');
const path = require('node:path');
const ids = ['32-history', '32a-cpu', '32b-gpu', '32c-close', '32d-command', '32e-release'];

module.exports = async (page, helpers) => {
    const {command, drawing, step} = helpers;
    const stage = ids.indexOf(step.id);
    assert(stage >= 0);
    await require('./replace.cjs')(page, helpers);
    const count = async () => Number(await page.locator('canvas').getAttribute('data-object-count'));
    const data = async key => JSON.parse(await page.locator('canvas').getAttribute(key));
    const history = async () => (await data('data-command-ui')).history;
    const baseline = await drawing(page);
    const cpu = stage >= 1 ? await data('data-cpu-usage') : null;
    const gpu = stage >= 2 ? await data('data-gpu-usage') : null;
    await command(page, 'Move 0.125,0,0');
    assert.notEqual(await drawing(page), baseline);
    if (cpu) {
        const moved = await data('data-cpu-usage');
        assert.equal(moved[0], cpu[0] + 3, 'One history snapshot adds three row records');
        assert.equal(moved[5], cpu[5] + 1);
        assert.deepEqual(moved.slice(1, 5), cpu.slice(1, 5), 'Move shares source/documents and display payload');
    }
    if (gpu) assert.deepEqual(await data('data-gpu-usage'), gpu, 'Move changes no live document allocation');
    await command(page, 'Undo');
    assert.equal(await drawing(page), baseline);
    if (cpu) assert.deepEqual((await data('data-cpu-usage')).slice(1, 5), cpu.slice(1, 5));
    if (stage >= 4) {
        const specimen = path.resolve('target', `course-${step.id}`, 'sample.pb');
        const choose = async file => {
            const picker = page.waitForEvent('filechooser');
            await command(page, 'Open');
            await (await picker).setFiles(file);
        };
        const closed = async () => {
            assert.equal(await count(), 0);
            assert.deepEqual(await data('data-cpu-usage'), [0, 0, 0, 0, 0, 1]);
            assert.deepEqual(await data('data-gpu-usage'), [0, 0, 0, 0]);
            assert.equal(await page.locator('#status').textContent(), 'Document closed.');
            assert.match((await history()).at(-1), /Document closed/);
        };
        const oldCounters = await data('data-gpu-stats');
        const projection = await page.locator('canvas').getAttribute('data-projection');
        await page.evaluate(() => {
            window.__originalRead = File.prototype.arrayBuffer;
            File.prototype.arrayBuffer = function () {
                const bytes = window.__originalRead.call(this);
                return new Promise((resolve, reject) => {
                    window.__releaseRead = fail => fail ? reject(new Error('late failure after Close')) : bytes.then(resolve, reject);
                });
            };
        });
        try {
            await choose(specimen);
            await page.waitForFunction(() => typeof window.__releaseRead === 'function');
            await command(page, 'Close');
            await closed();
            const empty = await drawing(page), entries = await history();
            assert.notEqual(empty, baseline, 'Close draws an empty scene');
            // Inspect actual GPU pixels above the command strip: all must be white.
            const {PNG} = require('pngjs');
            const canvas = page.locator('canvas');
            const url = await canvas.evaluate(canvas => canvas.toDataURL());
            const pixels = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
            const rows = pixels.height - Math.ceil(34 * pixels.width / (await canvas.boundingBox()).width);
            for (let offset = 0; offset < pixels.width * rows * 4; offset += 4) {
                assert.deepEqual([...pixels.data.subarray(offset, offset + 4)], [255, 255, 255, 255],
                    'Closed GPU drawing pixels must be opaque white');
            }
            await page.evaluate(() => window.__releaseRead(false));
            await page.waitForTimeout(150);
            await closed();
            assert.equal(await drawing(page), empty); assert.deepEqual(await history(), entries);
            assert.deepEqual(await data('data-gpu-stats'), oldCounters, 'Close does not reset cumulative allocation counters');
            assert.equal(await page.locator('canvas').getAttribute('data-projection'), projection);
            await command(page, 'Undo'); await command(page, 'Redo');
            assert.equal(await count(), 0); assert.equal(await drawing(page), empty);
        } finally {
            await page.evaluate(() => { File.prototype.arrayBuffer = window.__originalRead; delete window.__originalRead; delete window.__releaseRead; });
        }
        if (stage === 5) {
            let downloads = 0;
            const download = () => downloads++;
            page.on('download', download);
            try {
                await command(page, 'Save'); await page.waitForTimeout(100);
                assert.equal(downloads, 0); assert.match((await history()).at(-1), /1–64 meshes/);
            } finally { page.off('download', download); }
            await page.evaluate(() => {
                window.__originalRead = File.prototype.arrayBuffer;
                File.prototype.arrayBuffer = () => new Promise((resolve, reject) => { window.__rejectRead = () => reject(new Error('late rejected read')); });
            });
            try {
                await choose(specimen);
                await page.waitForFunction(() => typeof window.__rejectRead === 'function');
                await command(page, 'Close');
                const entries = await history(), empty = await drawing(page);
                await page.evaluate(() => window.__rejectRead()); await page.waitForTimeout(150);
                await closed(); assert.deepEqual(await history(), entries); assert.equal(await drawing(page), empty);
            } finally {
                await page.evaluate(() => { File.prototype.arrayBuffer = window.__originalRead; delete window.__originalRead; delete window.__rejectRead; });
            }
        }
        await choose(specimen);
        await page.waitForFunction(() => document.getElementById('status')?.textContent === 'File imported. Undo removes the entire import.');
        assert.equal(await count(), 3);
        await command(page, 'Select Next'); await command(page, 'Select Next');
        await command(page, 'Move 0.35,0,0.25');
        await command(page, 'View Isometric');
    }
    await command(page, 'Orbit Right'); await command(page, 'Orbit Up');
    const shift = Number((0.06 * (stage + 1)).toFixed(2)), rise = Number((0.03 * stage).toFixed(2));
    await command(page, `Move ${shift},0,${rise}`);
    await command(page, 'Fit');
};
