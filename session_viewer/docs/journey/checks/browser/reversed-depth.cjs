const assert = require('node:assert/strict');
module.exports = async (original, helpers) => {
    await require('./precise-origin-wakes.cjs')(original, helpers);
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message));
            const url = new URL(original.url()); url.searchParams.set('nogrid', '1'); url.searchParams.set('opacity', '1');
            await page.goto(url.href);
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const canvas = page.locator('canvas'), value = name => canvas.getAttribute(name).then(JSON.parse);
            const command = line => helpers.command(page, line);
            for (const line of ['Layers Off', 'Select All', 'Delete', 'CPlane XY', 'Box w0,0,0 2 2 1', 'Box w0,0,2 1 1 1', 'Select Object', 'Arctic Off', 'View Outline Off', 'View Grid Off', 'Opacity 1', 'View Top', 'Select None', 'Fit']) await command(line);
            const originals = await value('data-cads'); assert.equal(originals.length, 2);
            const [back, front] = originals.map(row => row.id);
            const project = async point => {
                const matrix = await value('data-render-matrix'), origin = await value('data-render-origin');
                const local = point.map((v, i) => v - origin[i]);
                const clip = [0, 1, 2, 3].map(row => local.reduce((sum, v, axis) => sum + matrix[axis * 4 + row] * v, matrix[12 + row]));
                assert(clip[3] > 0); return clip.slice(0, 3).map(v => v / clip[3]);
            };
            const pick = async (point, owner) => {
                const ndc = await project(point), bounds = await canvas.boundingBox(), submitted = (await value('data-pick-gpu'))[3];
                await page.keyboard.press('Escape');
                await page.mouse.click(bounds.x + (ndc[0] + 1) * bounds.width / 2, bounds.y + (1 - ndc[1]) * bounds.height / 2);
                await page.waitForFunction(({owner, submitted}) => {
                    const c = document.querySelector('canvas'), selected = JSON.parse(c.getAttribute('data-selected-ids')), pick = JSON.parse(c.getAttribute('data-pick-gpu'));
                    return selected.length === 1 && selected[0] === owner && pick[3] > submitted && pick[0] === 0 && pick[1] === 0;
                }, {owner, submitted});
            };
            for (const projection of ['Orthographic', 'Perspective']) {
                for (const line of ['View ' + projection, 'Select None', 'Fit']) await command(line);
                const near = await project([0, 0, 3]), far = await project([0, 0, 0]);
                assert(near[2] > far[2] && far[2] >= 0 && near[2] <= 1, 'The actual drawing matrix orders near depth above far depth');
                const picture = await helpers.drawing(page), placement = await value('data-row-placements'), uploads = (await value('data-gpu-stats'))[0];
                await pick([0, 0, 2.5], front);
                for (const line of ['Hide', 'Select None']) await command(line);
                assert.notEqual(await helpers.drawing(page), picture);
                await pick([0, 0, 0.5], back);
                for (const line of ['Show', 'Select None']) await command(line);
                assert.equal(await helpers.drawing(page), picture, 'Showing the nearer original restores the exact drawing');
                const shownUploads = (await value('data-gpu-stats'))[0];
                assert.equal(shownUploads, uploads + 1, 'Showing a retired independent original uploads its geometry once');
                await pick([0, 0, 2.5], front);
                for (const line of ['Move 0,0,-4', 'Select None', 'Fit']) await command(line);
                await pick([0, 0, 0.5], back);
                for (const line of ['Select None', 'Undo', 'Select None', 'Fit']) await command(line);
                assert.deepEqual(await value('data-row-placements'), placement);
                assert.equal(await helpers.drawing(page), picture, 'Undo restores original depth ordering and pixels');
                assert.deepEqual(await value('data-cads'), originals);
                assert.equal((await value('data-gpu-stats'))[0], shownUploads, 'Move and Undo reuse the visible original geometry');
            }
            assert.deepEqual(errors, []);
            console.log(`Reversed depth: actual near/far matrix ordering, overlapping original CAD picks, Hide/Show, placement Undo, exact pixels, original ownership, bounded Show uploads and Move/Undo geometry reuse passed for both projections at DPR ${density}`);
        } finally { await context.close(); await original.bringToFront(); }
    }
};
