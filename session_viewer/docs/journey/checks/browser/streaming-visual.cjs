const assert = require('node:assert/strict');
const fs = require('node:fs');
const originals = require('./visual-originals.json');
const {PNG} = require('pngjs');

async function feature(page, helpers) {
    const command = line => helpers.command(page, line);
    const value = name => page.locator('canvas').getAttribute(name).then(JSON.parse);
    for (const line of ['Close', 'Layers Off', 'View Grid Off', 'Example Cloud LOD']) await command(line);
    const downloading = page.waitForEvent('download');
    await command('Save');
    const archive = fs.readFileSync(await (await downloading).path());
    const magic = Buffer.from('SESSION-VIEWER\x01\n');
    assert(archive.subarray(0, magic.length).equals(magic));
    let at = magic.length;
    assert.equal(archive[at++], 10, 'The first archive field is an original document');
    let length = 0, shift = 0, byte;
    do {
        assert(shift < 35 && at < archive.length);
        byte = archive[at++]; length += (byte & 127) * 2 ** shift; shift += 7;
    } while (byte & 128);
    assert(at + length <= archive.length);
    const source = archive.subarray(at, at + length);
    const origin = new URL(page.url()).origin;
    const context = page.context(), ranges = [];
    let whole = 0;
    const manifest = 'items: [{file: visual/cad.pb, name: CAD, at: [-3,0,0]}, {file: visual/roof.pb, name: Roof, at: [1.5,0,0]}, {file: visual/points.pb, name: First, point_size: 12, at: [4,0,0]}, {file: visual/points.pb, name: Second, point_size: 7, at: [6,0,0]}]';
    await context.route(origin + '/visual/index.yaml', route => route.fulfill({status: 200, contentType: 'application/yaml', body: manifest}));
    for (const name of ['cad', 'roof']) {
        await context.route(origin + '/visual/' + name + '.pb', route => route.fulfill({status: 200, contentType: 'application/octet-stream', body: Buffer.from(originals[name], 'base64')}));
    }
    await context.route(origin + '/visual/points.pb', async route => {
        const range = route.request().headers().range;
        if (!range) { whole++; await route.fulfill({status: 400, body: 'This fixture requires ranged point loading'}); return; }
        const match = /^bytes=(\d+)-(\d+)$/.exec(range); assert(match);
        const first = Number(match[1]), last = Math.min(Number(match[2]), source.length - 1);
        assert(first <= last && last - first + 1 <= 64 * 1024 * 1024);
        ranges.push([first, last]);
        await route.fulfill({status: 206, contentType: 'application/octet-stream', headers: {ETag: '"visual-original"', 'Content-Range': `bytes ${first}-${last}/${source.length}`, 'Content-Length': String(last - first + 1)}, body: source.subarray(first, last + 1)});
    });
    await page.setViewportSize({width: 1100, height: 800});
    const url = new URL(page.url());
    url.searchParams.set('scene', 'visual/index.yaml'); url.searchParams.set('data', origin + '/'); url.searchParams.set('live', 'off');
    await page.goto(url.href);
    await page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
    await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-range-documents')).length === 2 && JSON.parse(document.querySelector('canvas').getAttribute('data-cads')).length === 1 && JSON.parse(document.querySelector('canvas').getAttribute('data-surfaces')).length === 1);
    for (const line of ['Layers Off', 'View Grid Off', 'Fit', 'View Isometric', 'Select None']) await command(line);
    const matrix = await value('data-camera-matrix');
    const image = PNG.sync.read(Buffer.from((await require('./canvas-image.cjs')(page.locator('canvas'))).split(',')[1], 'base64'));
    const sample = point => {
        const clip = [0, 1, 2, 3].map(row => point.reduce((sum, v, axis) => sum + matrix[axis * 4 + row] * v, matrix[12 + row]));
        assert(clip[2] / clip[3] > 0 && clip[2] / clip[3] < 1);
        const x = Math.floor((clip[0] / clip[3] + 1) * image.width / 2), y = Math.floor((1 - clip[1] / clip[3]) * image.height / 2);
        const i = (y * image.width + x) * 4;
        return [...image.data.subarray(i, i + 3)];
    };
    assert(sample([-3, 0, 0]).every(v => v > 245), 'The separated CAD bore reveals the background');
    assert(sample([-2.1, 0, 0.3]).some(v => v < 235), 'The adjacent original CAD face is visible');
    let colored = 0;
    for (let i = 0; i < image.data.length; i += 4) if (image.data[i + 2] > image.data[i] + 80 && image.data[i + 1] > 80) colored++;
    assert(colored > 100, 'The actual colored streamed point pages are visible');
    const owners = await value('data-range-documents'), usage = await value('data-range-sources');
    assert.equal(usage[2], 2000064); assert.equal(usage[1], 1);
    assert.equal(owners[0].source_guid, owners[1].source_guid); assert.notEqual(owners[0].id, owners[1].id);
    assert.deepEqual(owners.map(row => row.point_px), [12, 7]);
    assert.equal(owners[1].model[12] - owners[0].model[12], 2);
    assert(ranges.length > 0); assert.equal(whole, 0);
    console.log('Feature capture: open CAD bore, separated NURBS roof and two ranged original cloud instances are visible; one shared 2000064-point source, actual colors, bounded byte ranges and distinct owners verified.');
    return {status: await page.locator('#status').textContent(), owners, usage, ranges};
}

module.exports = async (page, helpers) => {
    await require('./streaming-cache.cjs')(page, helpers);
    await feature(page, helpers);
};
module.exports.feature = feature;
