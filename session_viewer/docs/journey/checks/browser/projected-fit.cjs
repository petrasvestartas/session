const assert = require('node:assert/strict');
module.exports = async (page, helpers) => {
    await require('./feature-owner-selection.cjs')(page, helpers);
    const value = name => page.locator('canvas').getAttribute(name).then(JSON.parse);
    for (const line of ['Close', 'CPlane XY', 'View Grid Off', 'Select Object', 'Box w0,0,0 1000 2 20', 'Select None', 'Layers All']) {
        await helpers.command(page, line);
    }
    assert.equal(await value('data-object-count'), 1);
    const original = await value('data-row-placements');
    const geometry = await value('data-cads');
    for (const view of ['View Front', 'View Top', 'View Isometric']) {
        for (const projection of ['View Orthographic', 'View Perspective']) {
            for (const line of [view, projection, 'Fit']) await helpers.command(page, line);
            const matrix = await value('data-camera-matrix');
            const frame = await value('data-camera-frame');
            assert(frame[2] < 1 || frame[3] < 1, 'Panels reserve canvas space');
            for (let corner = 0; corner < 8; corner++) {
                const p = [corner & 1 ? 500 : -500, corner & 2 ? 1 : -1, corner & 4 ? 20 : 0];
                const clip = [0, 1, 2, 3].map(i => matrix[i] * p[0] + matrix[4 + i] * p[1] + matrix[8 + i] * p[2] + matrix[12 + i]);
                const x = clip[0] / clip[3], y = clip[1] / clip[3];
                assert(x >= 2 * frame[0] - 1 - 1e-7 && x <= 2 * frame[2] - 1 + 1e-7, 'Horizontal corners fit');
                assert(y >= 1 - 2 * frame[3] - 1e-7 && y <= 1 - 2 * frame[1] + 1e-7, 'Vertical corners fit');
            }
            assert.deepEqual(await value('data-row-placements'), original);
            assert.deepEqual(await value('data-cads'), geometry);
        }
    }
    for (const line of ['View Front', 'View Orthographic', 'Fit']) await helpers.command(page, line);
    const matrix = await value('data-camera-matrix');
    assert(1000 * Math.abs(matrix[0]) > 1.2, 'A thin part fills the free horizontal extent');
};
