const assert = require('node:assert/strict');
const adaptive = require('./adaptive-gpu-reuse.cjs');
const fs = require('node:fs');
const {PNG} = require('pngjs');

module.exports = async (original, helpers) => {
    await require('./arena-empty.cjs')(original, helpers);
    for (const density of [1, 2]) {
        const context = await original.context().browser().newContext({viewport: {width: 960, height: 640}, deviceScaleFactor: density});
        try {
            await context.addInitScript(() => {
                const read = name => JSON.parse(document.querySelector('canvas').getAttribute(name));
                let dragging = false;
                window.__originWakes = [];
                window.addEventListener('pointerdown', () => { dragging = true; }, true);
                window.addEventListener('pointerup', () => { dragging = false; }, true);
                window.addEventListener('viewer-origin', () => {
                    const record = {dragging, beforePending: document.querySelector('canvas').getAttribute('data-origin-pending'), before: read('data-render-origin')};
                    queueMicrotask(() => {
                        record.after = read('data-render-origin');
                        record.afterPending = document.querySelector('canvas').getAttribute('data-origin-pending');
                        window.__originWakes.push(record);
                    });
                }, true);
            });
            const page = await context.newPage(), errors = [];
            page.on('pageerror', error => errors.push(error.message));
            const url = new URL(original.url()); url.searchParams.set('nogrid', '1'); url.searchParams.set('opacity', '1');
            await page.goto(url.href);
            await page.waitForFunction(() => window.wasmBindings?.runtime_running?.()
                && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome === 'ready');
            const canvas = page.locator('canvas'), value = name => canvas.getAttribute(name).then(JSON.parse);
            const command = line => helpers.command(page, line);
            const settled = () => page.waitForFunction(() => {
                const c = document.querySelector('canvas');
                if (c.getAttribute('data-origin-pending') !== 'false') return false;
                const origin = JSON.parse(c.getAttribute('data-render-origin'));
                const target = JSON.parse(c.getAttribute('data-camera-target'));
                return origin && target && origin.every((v, i) => Math.abs(v - target[i]) < 1e-5);
            });
            const topology = cads => cads.map(({id, ...source}) => source);
            for (const line of ['Layers Off', 'Select All', 'Delete', 'Box w0,0,0 1 1 1', 'Arctic Off', 'View Outline Off', 'View Grid Off', 'Opacity 1', 'Select Object', 'View Top', 'View Orthographic', 'Fit', 'Select None']) await command(line);
            const source = await value('data-cads'); assert.equal(source.length, 1); assert(source[0].faces.length >= 6);
            await adaptive.snapshot(page, {lane: 'mesh', source: 'cad', expectedRows: 1, expectedQualities: 1});
            const localPlacement = await value('data-row-placements'), localPicture = await helpers.drawing(page);
            const imageUrl = await require('./canvas-image.cjs')(canvas);
            const image = PNG.sync.read(Buffer.from(imageUrl.split(',')[1], 'base64'));
            let ink = 0;
            for (let y = 0; y < image.height - 40 * density; y++) for (let x = 0; x < image.width; x++) {
                const i = (y * image.width + x) * 4;
                if (image.data[i] !== 255 || image.data[i + 1] !== 255 || image.data[i + 2] !== 255) ink++;
            }
            assert(ink > 1000, 'The actual original CAD must produce drawing pixels');
            const uploads = (await value('data-gpu-stats'))[0], shift = [1e9, -1e9, 1e9];
            for (const line of ['Select All', 'Move 1000000000,-1000000000,1000000000', 'Fit', 'Select None']) await command(line);
            await settled();
            const distantPlacement = await value('data-row-placements'), origin = await value('data-render-origin');
            assert(origin.every((v, i) => Math.abs(v - shift[i]) < 10));
            for (let i = 0; i < 3; i++) assert.equal(distantPlacement[0][1][12 + i], localPlacement[0][1][12 + i] + shift[i]);
            assert.deepEqual(await value('data-cads'), source, 'Moving a placed original retains its source/topology identity');
            if (!(await adaptive.snapshot(page, {lane: 'mesh', source: 'cad', expectedRows: 1, expectedQualities: 1})).adaptive) assert.equal((await value('data-gpu-stats'))[0], uploads, 'A precise placement reuses original geometry');
            const farPicture = await helpers.drawing(page);
            assert.equal(farPicture, localPicture, 'A binary unit CAD box keeps exact pixels at a distant placement');
            const matrix = await value('data-render-matrix'), target = await value('data-camera-target'), box = await canvas.boundingBox();
            const p = target.map((v, i) => v - origin[i]);
            const clip = [0, 1, 2, 3].map(row => p.reduce((sum, v, axis) => sum + matrix[axis * 4 + row] * v, matrix[12 + row]));
            assert(clip[3] > 0 && Math.abs(clip[0] / clip[3]) < 1 && Math.abs(clip[1] / clip[3]) < 1);
            const submitted = (await value('data-pick-gpu'))[3];
            await page.keyboard.press('Escape');
            await page.mouse.click(box.x + (clip[0] / clip[3] + 1) * box.width / 2, box.y + (1 - clip[1] / clip[3]) * box.height / 2);
            await page.waitForFunction(({id, submitted}) => {
                const c = document.querySelector('canvas'), pick = JSON.parse(c.getAttribute('data-pick-gpu'));
                return JSON.parse(c.getAttribute('data-selected-ids')).includes(id) && pick[3] > submitted && pick[0] === 0 && pick[1] === 0;
            }, {id: source[0].id, submitted});
            assert.notEqual(await helpers.drawing(page), farPicture);
            await command('Select None'); assert.equal(await helpers.drawing(page), farPicture);
            for (const line of ['Undo', 'Fit', 'Select None']) await command(line);
            assert.deepEqual(await value('data-row-placements'), localPlacement);
            assert.equal(await helpers.drawing(page), localPicture, 'Undo restores the exact original drawing');
            for (const line of ['Redo', 'Fit', 'Select None']) await command(line);
            await settled(); assert.deepEqual(await value('data-row-placements'), distantPlacement);
            assert.equal(await helpers.drawing(page), farPicture); if (!(await adaptive.snapshot(page, {lane: 'mesh', source: 'cad', expectedRows: 1, expectedQualities: 1})).adaptive) assert.equal((await value('data-gpu-stats'))[0], uploads);
            const downloading = page.waitForEvent('download'); await command('Save');
            const download = await downloading, bytes = fs.readFileSync(await download.path()); assert(bytes.length > 100);
            await command('Close');
            const choosing = page.waitForEvent('filechooser'); await command('Open');
            await (await choosing).setFiles({name: 'precise-original-roundtrip.pb', mimeType: 'application/octet-stream', buffer: bytes});
            await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-cads')).length === 1);
            for (const line of ['Fit', 'Select None']) await command(line);
            await settled(); assert.deepEqual(topology(await value('data-cads')), topology(source));
            assert.deepEqual((await value('data-row-placements')).map(row => row[1]), distantPlacement.map(row => row[1]));
            assert.equal(await helpers.drawing(page), farPicture, 'Save/reopen preserves exact distant original pixels');
            // Zoom is bounded by source radius; enlarge the real source before testing the 1000-unit rebase threshold.
            for (const line of ['Select All', 'Scale 10000', 'Fit', 'Select None']) await command(line);
            for (let i = 0; i < 6; i++) await command('Zoom Out');
            await page.keyboard.press('Escape');
            await page.evaluate(() => { window.__originWakes.length = 0; });
            const bounds = await canvas.boundingBox(), y = bounds.y + bounds.height * 0.35;
            const pan = async stop => {
                await page.mouse.move(bounds.x + bounds.width * 0.2, y);
                await page.mouse.down({button: 'middle'});
                if (!stop) {
                    // Send release without an attribute round-trip that could outlast the throttle.
                    await page.mouse.move(bounds.x + bounds.width * 0.8, y);
                    await page.mouse.move(bounds.x + bounds.width * 0.2, y);
                    await page.mouse.up({button: 'middle'});
                    return true;
                }
                for (let i = 0; i < 8; i++) {
                    await page.mouse.move(bounds.x + bounds.width * (i % 2 ? 0.2 : 0.8), y);
                    if (await page.evaluate(() => {
                        const c = document.querySelector('canvas');
                        if (c.getAttribute('data-origin-pending') !== 'true') return false;
                        window.dispatchEvent(new PageTransitionEvent('pagehide', {persisted: false}));
                        return true;
                    })) { await page.mouse.up({button: 'middle'}); return true; }
                }
                await page.mouse.up({button: 'middle'});
                return false;
            };
            let wokeAfterRelease = false;
            for (let attempt = 0; attempt < 8 && !wokeAfterRelease; attempt++) {
                await pan(false);
                await page.waitForTimeout(250);
                wokeAfterRelease = await page.evaluate(() => window.__originWakes.some(record => !record.dragging && record.beforePending === 'true' && record.afterPending === 'false' && record.before.some((v, i) => v !== record.after[i])));
            }
            if (!wokeAfterRelease) console.log('Owned wake probe:', JSON.stringify(await page.evaluate(() => window.__originWakes)));
            assert(wokeAfterRelease, 'An actual owned wake must finish a pending origin change after the real pointer is released');
            let stoppedPending = false;
            for (let attempt = 0; attempt < 8 && !stoppedPending; attempt++) stoppedPending = await pan(true);
            assert(stoppedPending, 'Dispose the runtime atomically while its origin timer is pending');
            await page.waitForFunction(() => !window.wasmBindings.runtime_running());
            const stoppedOrigin = await value('data-render-origin');
            await page.waitForTimeout(300);
            assert.equal(await page.evaluate(() => window.wasmBindings.runtime_running()), false);
            assert.deepEqual(await value('data-render-origin'), stoppedOrigin, 'A stopped runtime cannot be woken by its old origin timer');
            assert.deepEqual(errors, []);
            console.log(`Precise origin: real CAD, distant pixels/original picks, Undo/Redo, Save/reopen, idle deferred rebase and stopped-runtime wake cancellation passed at DPR ${density}`);
        } finally { await context.close(); await original.bringToFront(); }
    }
};
