const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const ids = ['32fc-version', '32fd-origin', '32fe-location', '32ff-bridge', '32fg-state',
    '32fh-policy', '32fi-history', '32fj-proof', '32fk-command', '32fl-guards'];

module.exports = async (page, helpers) => {
    const {command, drawing, step} = helpers;
    const stage = ids.indexOf(step.id); assert(stage >= 0);
    const specimen = path.resolve('target', `course-${step.id}`, 'sample.pb');
    const version = crypto.createHash('sha256').update(fs.readFileSync(specimen)).digest('hex');
    const data = async key => JSON.parse(await page.locator('canvas').getAttribute(key));
    const rows = () => data('data-row-metadata'), origins = () => data('data-source-origins');
    const choose = async (verb = 'Open', file = specimen) => {
        const picker = page.waitForEvent('filechooser');
        await command(page, verb); await (await picker).setFiles(file);
    };
    const imported = () => page.waitForFunction(() => document.getElementById('status')?.textContent === 'File imported. Undo removes the entire import.');
    if (stage >= 3) await page.evaluate(() => {
        window.__urlRecord = {created: [], revoked: [], create: URL.createObjectURL, revoke: URL.revokeObjectURL};
        URL.createObjectURL = function (file) {
            const url = window.__urlRecord.create.call(URL, file); window.__urlRecord.created.push(url); return url;
        };
        URL.revokeObjectURL = function (url) {
            window.__urlRecord.revoked.push(url); return window.__urlRecord.revoke.call(URL, url);
        };
    });
    try {
        await require('./replace.cjs')(page, helpers);
        const baseline = await rows(), pixels = await drawing(page), cpu = await data('data-cpu-usage');
        const gpu = await data('data-gpu-usage');
        assert.equal(baseline.length, 3);
        const original = stage >= 1 ? await origins() : null;
        if (original) {
            assert.equal(original.length, 3); assert.equal(new Set(original.map(row => row[1])).size, 1);
            assert(original.every(row => row[2] === version));
        }
        if (stage >= 3) {
            assert(original.every(row => row[3] === original[0][3])); assert.match(original[0][3], /^blob:/);
            const bytes = await page.evaluate(async url => [...new Uint8Array(await (await fetch(url)).arrayBuffer())], original[0][3]);
            assert.equal(crypto.createHash('sha256').update(Buffer.from(bytes)).digest('hex'), version);
            // Cancel a held read before adoption: it must allocate no URL.
            const before = await page.evaluate(() => window.__urlRecord.created.length);
            await page.evaluate(() => {
                window.__originalRead = File.prototype.arrayBuffer;
                File.prototype.arrayBuffer = function () {
                    const bytes = window.__originalRead.call(this);
                    return new Promise((resolve, reject) => { window.__releaseRead = () => bytes.then(resolve, reject); });
                };
            });
            try {
                await choose(); await page.waitForFunction(() => typeof window.__releaseRead === 'function');
                await command(page, 'Cancel Open'); await page.evaluate(() => window.__releaseRead());
                await page.waitForTimeout(150);
                assert.equal(await page.evaluate(() => window.__urlRecord.created.length), before);
                assert.deepEqual(await rows(), baseline); assert.equal(await drawing(page), pixels);
            } finally {
                await page.evaluate(() => { File.prototype.arrayBuffer = window.__originalRead; delete window.__originalRead; delete window.__releaseRead; });
            }
            // A URL allocation failure cannot commit a document or claim success.
            await page.evaluate(() => { window.__trackingCreate = URL.createObjectURL; URL.createObjectURL = () => { throw new Error('URL allocation refused'); }; });
            try {
                await choose();
                await page.waitForFunction(() => document.getElementById('status')?.textContent?.startsWith('Cannot retain reload file'));
                assert.deepEqual(await rows(), baseline); assert.equal(await drawing(page), pixels);
            } finally { await page.evaluate(() => { URL.createObjectURL = window.__trackingCreate; delete window.__trackingCreate; }); }
            // A byte-only external event has no accepted File to adopt.
            await page.evaluate(() => window.dispatchEvent(new CustomEvent('viewer-file', {detail: [false, new Uint8Array([255])]})));
            assert.deepEqual(await rows(), baseline);
        }
        await command(page, 'Move 0.2,0,0.1');
        assert.deepEqual(await rows(), baseline);
        if (original) assert.deepEqual(await origins(), original);
        assert.deepEqual((await data('data-cpu-usage')).slice(1, 5), cpu.slice(1, 5));
        assert.deepEqual(await data('data-gpu-usage'), gpu);
        await command(page, 'Undo'); assert.equal(await drawing(page), pixels);
        await choose(); await imported();
        const duplicate = await rows(); assert.equal(duplicate.length, 6);
        assert.equal(new Set(duplicate.map(row => row[0])).size, 6);
        for (let i = 0; i < 3; i++) assert.deepEqual(duplicate[i].slice(1), duplicate[i + 3].slice(1));
        const copied = stage >= 1 ? await origins() : null;
        if (copied) {
            assert.equal(new Set(copied.map(row => row[1])).size, 2);
            assert(copied.every(row => row[2] === version));
            if (stage >= 3) assert.notEqual(copied[0][3], copied[3][3]);
        }
        await command(page, 'Undo'); assert.deepEqual(await rows(), baseline); assert.equal(await drawing(page), pixels);
        await command(page, 'Redo'); assert.deepEqual(await rows(), duplicate);
        if (copied) assert.deepEqual(await origins(), copied);
        await command(page, 'Undo');
        if (stage >= 8) {
            const warm = await drawing(page), counters = await data('data-gpu-stats');
            const usage = await data('data-gpu-usage'), before = await data('data-cpu-usage');
            const originValues = await origins();
            await command(page, 'Unload Sources');
            assert.equal(await page.locator('#status').textContent(), 'Editable sources unloaded; display retained.');
            assert.equal(await drawing(page), warm); assert.deepEqual(await rows(), baseline);
            assert.deepEqual(await origins(), originValues);
            assert.deepEqual(await data('data-gpu-stats'), counters); assert.deepEqual(await data('data-gpu-usage'), usage);
            const cold = await data('data-cpu-usage');
            assert(cold[1] < before[1]); assert.equal(cold[2], 0, 'All eligible imported sources across active/Undo/Redo roots unload');
            assert.deepEqual(cold.slice(3), before.slice(3), 'Displays, payload and history roots remain');
            const residency = await data('data-source-residency'); assert(residency.every(row => row[1] === false && row[2] === 1));
            await command(page, 'Unload Sources');
            assert.match(await page.locator('#status').textContent(), /No eligible/);
            assert.equal(await drawing(page), warm); assert.deepEqual(await data('data-source-residency'), residency);
            let downloads = 0; const listener = () => downloads++; page.on('download', listener);
            try {
                for (const line of ['Move 0.2,0,0', 'Delete', 'Save']) {
                    await command(page, line);
                    const history = (await data('data-command-ui')).history;
                    assert.match(history.at(-1), /Reload editable sources before/);
                    assert.deepEqual(await rows(), baseline); assert.equal(await drawing(page), warm);
                }
                await page.waitForTimeout(100); assert.equal(downloads, 0);
            } finally { page.off('download', listener); }
            await command(page, 'Undo');
            assert.notEqual(await drawing(page), warm, 'Unloading and refused edits preserve the earlier Move Undo');
            assert((await data('data-source-residency')).every(row => row[1] === false && row[2] === 1));
            await command(page, 'Redo'); assert.equal(await drawing(page), warm);
            assert.deepEqual((await data('data-gpu-stats')).slice(0, 2), counters.slice(0, 2));
            await command(page, 'Close'); assert.equal((await rows()).length, 0);
            assert.deepEqual(await data('data-gpu-usage'), [0, 0, 0, 0]);
            const recorded = await page.evaluate(() => ({created: window.__urlRecord.created, revoked: window.__urlRecord.revoked}));
            assert(recorded.created.length > 0);
            for (const url of recorded.created) assert.equal(recorded.revoked.filter(value => value === url).length, 1, 'Each adopted or refused owned URL revokes once');
            assert(recorded.revoked.includes(original[0][3]), 'Close revokes the actual adopted source URL');
            await choose(); await imported();
            await command(page, 'Select Next'); await command(page, 'Select Next');
            await command(page, 'Move 0.35,0,0.25'); await command(page, 'View Isometric');
        }
        if (stage >= 3 && stage < 8) {
            await command(page, 'Close'); assert.equal((await rows()).length, 0);
            const recorded = await page.evaluate(() => ({created: window.__urlRecord.created, revoked: window.__urlRecord.revoked}));
            assert(recorded.created.length > 0);
            for (const url of recorded.created) assert.equal(recorded.revoked.filter(value => value === url).length, 1);
            assert(recorded.revoked.includes(original[0][3]));
            await choose(); await imported();
            await command(page, 'Select Next'); await command(page, 'Select Next');
            await command(page, 'Move 0.35,0,0.25'); await command(page, 'View Isometric');
        }
        await command(page, 'Orbit Right'); await command(page, 'Orbit Up');
        await command(page, `Move ${(0.60 + stage * 0.06).toFixed(2)},0,${(0.27 + stage * 0.03).toFixed(2)}`);
        await command(page, 'Fit');
        if (stage >= 8) {
            const warm = await drawing(page), counters = await data('data-gpu-stats');
            await command(page, 'Unload Sources'); assert.equal(await drawing(page), warm);
            assert.deepEqual(await data('data-gpu-stats'), counters);
            assert((await data('data-source-residency')).every(row => row[1] === false && row[2] === 2));
            assert.equal((await data('data-cpu-usage'))[2], 0);
        }
    } finally {
        if (stage >= 3) await page.evaluate(() => {
            URL.createObjectURL = window.__urlRecord.create; URL.revokeObjectURL = window.__urlRecord.revoke;
            delete window.__urlRecord;
        });
    }
};
