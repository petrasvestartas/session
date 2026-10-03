const assert = require('node:assert/strict');
const path = require('node:path');
module.exports = async (page, helpers) => {
    await require('./reload-precision.cjs')(page, helpers);
    const {command, drawing, step} = helpers, specimen = path.resolve('target', `course-${step.id}`, 'sample.pb');
    const data = async key => JSON.parse(await page.locator('canvas').getAttribute(key));
    const snapshot = async () => ({placements: await data('data-row-placements'), selected: await page.locator('canvas').getAttribute('data-selected-id'),
        camera: await data('data-camera-matrix'), residency: await data('data-source-residency'), counters: await data('data-gpu-stats'), usage: await data('data-gpu-usage'), drawing: await drawing(page)});
    const submit = async line => {
        const ui = await data('data-command-ui'), field = ui.controls.find(row => row.key === 'command/input');
        const box = await page.locator('canvas').boundingBox(), r = field.rect;
        await page.mouse.click(box.x + (r[0] + r[2]) / 2, box.y + (r[1] + r[3]) / 2);
        await page.keyboard.press('ControlOrMeta+A'); await page.keyboard.type(line); await page.keyboard.press('Enter');
    };
    await command(page, 'Unload Sources'); const frozen = await snapshot();
    await page.evaluate(() => {
        window.__failureFetch = window.fetch; window.__failureMode = 'real'; window.__failureCount = 0; window.__failureHeld = [];
        window.fetch = function (url, options) {
            window.__failureCount++;
            if (options.signal?.aborted) return Promise.reject(new DOMException("Aborted", "AbortError"));
            const mode = window.__failureMode;
            if (mode === 'real' || (mode === 'second' && window.__failureCount === 1)) return window.__failureFetch.call(this, url, options);
            if (mode === 'hold') {
                const body = window.__failureFetch.call(this, url).then(value => ({value}), error => ({error}));
                return new Promise((resolve, reject) => window.__failureHeld.push({signal: options.signal,
                    release: () => body.then(result => result.error ? reject(result.error) : resolve(result.value))}));
            }
            if (mode === 'reject' || mode === 'second') return Promise.reject(new Error('Current source request rejected'));
            if (mode === 'invalid') return Promise.resolve({});
            if (mode === 'http') return Promise.resolve(new Response(null, {status: 503}));
            if (mode === 'length') return Promise.resolve(new Response(new Uint8Array([1]), {headers: {'Content-Length': 'invalid'}}));
            if (mode === 'no-body') return Promise.resolve(new Response(null));
            if (mode === 'oversize') return Promise.resolve(new Response(new Uint8Array(4 * 1024 * 1024 + 1)));
            if (mode === 'read') return Promise.resolve(new Response(new ReadableStream({start(controller) { controller.error(new Error('Failed source body')); }})));
            if (mode === 'changed') return Promise.resolve(new Response(new Uint8Array([255])));
            throw new Error('Unknown controlled source failure');
        };
        window.__failureRevoke = URL.revokeObjectURL; window.__failureRevoked = [];
        URL.revokeObjectURL = function (url) { window.__failureRevoked.push(url); return window.__failureRevoke.call(URL, url); };
    });
    let downloads = 0; const onDownload = () => downloads++; page.on('download', onDownload);
    const fail = async (line, mode, error) => {
        await page.evaluate(mode => { window.__failureMode = mode; window.__failureCount = 0; }, mode);
        await submit(line);
        await page.waitForFunction(({name, error}) => {
            const ui = JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui'));
            return ui.history.at(-1)?.startsWith('> ' + name + '\n') && ui.history.at(-1).includes(error);
        }, {name: line.split(' ')[0], error});
    };
    try {
        for (const [mode, error] of [['http', 'Reload HTTP 503'], ['length', 'Invalid source length'], ['no-body', 'no body'],
            ['reject', 'Cannot fetch source'], ['oversize', 'exceeds 4 MiB'], ['read', 'Cannot read source'], ['invalid', 'Invalid fetch response'], ['changed', 'version changed']]) {
            for (const line of ['Move 0.25,0,0.15', 'Delete', 'Save']) {
                await fail(line, mode, error); assert.deepEqual(await snapshot(), frozen); assert.equal(downloads, 0);
            }
        }
        await page.evaluate(() => { window.__failureMode = 'real'; });
        const append = page.waitForEvent('filechooser'); await command(page, 'Open'); await (await append).setFiles(specimen);
        await page.waitForFunction(() => document.getElementById('status')?.textContent === 'File imported. Undo removes the entire import.');
        await command(page, 'Unload Sources'); const pair = await snapshot();
        assert.equal(pair.placements.length, 6); assert(pair.residency.every(row => !row[1]));
        await fail('Save', 'second', 'Cannot fetch source');
        assert.equal(await page.evaluate(() => window.__failureCount), 2, 'First real body succeeded before second fetch failed');
        assert.deepEqual(await snapshot(), pair, 'Multiple-source failure installs neither source'); assert.equal(downloads, 0);
        // Ordinary edit history is still intact after every failed operation.
        await command(page, 'Undo'); assert.equal((await data('data-row-placements')).length, 3);
        await command(page, 'Redo'); assert.equal((await data('data-row-placements')).length, 6);
        await page.evaluate(() => { window.__failureMode = 'hold'; window.__failureHeld = []; });
        await submit('Save'); await page.waitForFunction(() => window.__failureHeld.length === 1);
        const sources = (await data('data-source-origins')).map(row => row[3]);
        await command(page, 'Close'); const closed = await snapshot(), messages = (await data('data-command-ui')).history;
        assert(await page.evaluate(() => window.__failureHeld[0].signal.aborted));
        const revoked = await page.evaluate(() => window.__failureRevoked); assert(sources.every(url => revoked.includes(url)));
        await page.evaluate(() => window.__failureHeld[0].release()); await page.waitForTimeout(150);
        assert.deepEqual(await snapshot(), closed); assert.deepEqual((await data('data-command-ui')).history, messages); assert.equal(downloads, 0);
    } finally {
        page.off('download', onDownload);
        await page.evaluate(() => { window.fetch = window.__failureFetch; URL.revokeObjectURL = window.__failureRevoke;
            delete window.__failureFetch; delete window.__failureMode; delete window.__failureCount; delete window.__failureHeld;
            delete window.__failureRevoke; delete window.__failureRevoked; });
    }
    const picker = page.waitForEvent('filechooser'); await command(page, 'Open Replace'); await (await picker).setFiles(specimen);
    await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Document replaced. Undo restores the previous document.');
    await command(page, 'Select Next'); await command(page, 'Move 0.35,0,0.25'); await command(page, 'View Isometric'); await command(page, 'Fit');
    await command(page, 'Move 0.25,0,0.15'); await command(page, 'Move 0,-0.5,0'); await command(page, 'Orbit Up'); await command(page, 'Orbit Right'); await command(page, 'Fit');
};
