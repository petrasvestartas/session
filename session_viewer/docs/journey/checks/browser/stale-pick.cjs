const assert = require('node:assert/strict');

module.exports = async (original, helpers) => {
    await require('./gpu-pick.cjs')(original, helpers);
    const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}});
    try {
        await context.addInitScript(() => {
            const map = GPUBuffer.prototype.mapAsync;
            window.pickHold = {enabled: false, waiting: false, release: null};
            GPUBuffer.prototype.mapAsync = function(...args) {
                const result = map.apply(this, args);
                if (!window.pickHold.enabled) return result;
                window.pickHold.enabled = false;
                return result.then(() => new Promise(resolve => {
                    window.pickHold.waiting = true;
                    window.pickHold.release = () => { window.pickHold.waiting = false; window.pickHold.release = null; resolve(); };
                }));
            };
        });
        const page = await context.newPage(), errors = [];
        page.on('pageerror', error => errors.push(error.message)); await page.goto(original.url());
        await page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
            && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
        const selected = () => page.locator('canvas').getAttribute('data-selected-id');
        const centre = async () => {
            const box = await page.locator('canvas').boundingBox();
            const m = JSON.parse(await page.locator('canvas').getAttribute('data-camera-matrix'));
            await page.mouse.click(box.x + (m[12] / m[15] + 1) * box.width / 2, box.y + (1 - m[13] / m[15]) * box.height / 2);
        };
        const release = async () => { await page.evaluate(() => window.pickHold.release()); await page.waitForTimeout(150); };
        for (const change of ['Pan Right', 'View Orthographic', 'Undo', 'Close', 'resize', 'blur', 'pagehide']) {
            await helpers.command(page, 'Close'); await helpers.command(page, 'View Reset'); await helpers.command(page, 'Example Point');
            await page.keyboard.press('Escape'); await page.evaluate(() => { window.pickHold.enabled = true; });
            await centre(); await page.waitForFunction(() => window.pickHold.waiting);
            assert.equal(await selected(), '', 'The genuine GPU map is held before adoption');
            if (change === 'resize') {
                await page.setViewportSize({width: 980, height: 650});
                await page.waitForFunction(() => {
                    const canvas = document.querySelector('canvas');
                    return canvas.width === 980 && canvas.height === 650;
                });
            }
            else if (change === 'blur') await page.evaluate(() => window.dispatchEvent(new Event('blur')));
            else if (change === 'pagehide') await page.evaluate(() => window.dispatchEvent(new PageTransitionEvent('pagehide', {persisted: true})));
            else await helpers.command(page, change);
            await release(); assert.equal(await selected(), '', change + ' must invalidate the old hit');
            await helpers.command(page, 'Close'); await helpers.command(page, 'View Reset'); await helpers.command(page, 'Example Point');
            await page.keyboard.press('Escape'); await centre();
            await page.waitForFunction(() => /^ObjectId\(\d+\)$/.test(document.querySelector('canvas').getAttribute('data-selected-id')));
        }
        await helpers.command(page, 'Close'); await helpers.command(page, 'View Reset'); await helpers.command(page, 'Example Point');
        await page.keyboard.press('Escape'); await page.evaluate(() => { window.pickHold.enabled = true; }); await centre();
        await page.waitForFunction(() => window.pickHold.waiting);
        const box = await page.locator('canvas').boundingBox(); await page.mouse.click(box.x + 4, box.y + 4);
        await release(); await page.waitForFunction(() => {
            const pick = JSON.parse(document.querySelector('canvas').getAttribute('data-pick-gpu')); return pick[0] === 0 && pick[1] === 0;
        });
        assert.equal(await selected(), '', 'The newer blank cursor supersedes the older point hit');
        assert.deepEqual(errors, []);
    } finally { await context.close(); }
};
