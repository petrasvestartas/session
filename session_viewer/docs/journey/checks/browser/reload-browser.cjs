const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const ids = ["32gf-request","32gfa-ownership","32gg-fetch","32gga-flight","32ggb-delivery","32gh-command","32gha-cancel"];

async function baseline(page, helpers) {
    const {command, drawing, step} = helpers;
    const index = ids.indexOf(step.id); assert(index >= 0);
    const stage = 16 + index;
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
}

module.exports = async (page, helpers) => {
    await baseline(page, helpers);
    const stage = ids.indexOf(helpers.step.id);
    if (stage < 5) return;
    const {command, drawing, step} = helpers;
    const data = async key => JSON.parse(await page.locator('canvas').getAttribute(key));
    const history = async () => (await data('data-command-ui')).history;
    const cold = async () => (await data('data-source-residency')).every(row => !row[1]);
    const loaded = async () => (await data('data-source-residency')).every(row => row[1]);
    const pixels = await drawing(page), rows = await data('data-row-metadata');
    const origins = await data('data-source-origins'), gpu = await data('data-gpu-stats');
    const usage = await data('data-gpu-usage');
    const specimen = path.resolve('target', `course-${step.id}`, 'sample.pb');
    async function submit() {
        const before = (await history()).length;
        const ui = await data('data-command-ui'), field = ui.controls.find(control => control.key === 'command/input');
        const box = await page.locator('canvas').boundingBox(), rect = field.rect;
        await page.mouse.click(box.x + (rect[0] + rect[2]) / 2, box.y + (rect[1] + rect[3]) / 2);
        await page.keyboard.press('ControlOrMeta+A'); await page.keyboard.type('Reload Sources'); await page.keyboard.press('Enter');
        await page.waitForFunction(() => {
            const ui = JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui'));
            return ui.command === '' && ui.history.at(-1)?.startsWith('> Reload Sources\n');
        });
        return before;
    }
    async function restored(before) {
        await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Editable sources restored; display retained.');
        const now = await history(); assert.equal(now.length, Math.min(200, before + 2), 'One submission and one owned asynchronous reply');
        assert(now.at(-2).endsWith('Reloading editable sources…'));
        assert(now.at(-1).endsWith('Editable sources restored; display retained.'));
        assert(await loaded());
    }
    await page.evaluate(() => window.dispatchEvent(new Event('viewer-reload')));
    assert(await cold(), 'External event has no accepted source slot');
    await restored(await submit());
    assert.equal(await drawing(page), pixels); assert.deepEqual(await data('data-row-metadata'), rows);
    assert.deepEqual(await data('data-source-origins'), origins);
    assert.deepEqual(await data('data-gpu-stats'), gpu); assert.deepEqual(await data('data-gpu-usage'), usage);
    assert.equal((await data('data-cpu-usage'))[2], 1, 'One restored source Session shared across history');
    await command(page, 'Reload Sources'); assert.match((await history()).at(-1), /No released sources/);
    const pending = page.waitForEvent('download'); await command(page, 'Save'); const download = await pending;
    const saved = path.resolve('target', `course-${step.id}`, 'reloaded.session'); await download.saveAs(saved);
    assert(fs.statSync(saved).size > 100); assert.equal(await drawing(page), pixels);
    if (stage >= 6) {
        const {spawnSync} = require('node:child_process');
        const result = spawnSync('buildslot', ['timeout', '10m', 'prlimit', '--data=6442450944', '--', 'cargo', 'run', '--example', 'reload_check', '--locked', '--target', 'x86_64-unknown-linux-gnu', '-j4', '--', specimen, saved], {
            cwd: path.resolve('target', `course-${step.id}`), encoding: 'utf8',
            env: {...process.env, REGEN_PROTO: '0', RUSTC_WRAPPER: '', CARGO_NET_OFFLINE: 'true', CARGO_BUILD_JOBS: '4', CARGO_TARGET_DIR: path.resolve('target/course-build')},
        });
        assert.equal(result.status, 0, result.stdout + result.stderr);
    }
    await command(page, 'Undo'); assert.notEqual(await drawing(page), pixels, 'Reload and Save did not consume the preceding Move');
    assert(await loaded(), 'Matching history sources were restored too');
    await command(page, 'Redo'); assert.equal(await drawing(page), pixels);
    if (stage < 6) {
        await command(page, 'Unload Sources'); await restored(await submit()); return;
    }
    // Intercept browser fetch promises while retaining their real Blob responses.
    await page.evaluate(() => {
        window.__fetchOriginal = window.fetch;
        window.__heldFetch = []; window.__fetchMode = 'hold'; window.__revokedReload = [];
        window.__revokeOriginal = URL.revokeObjectURL;
        URL.revokeObjectURL = url => { window.__revokedReload.push(url); window.__revokeOriginal.call(URL, url); };
        window.fetch = (url, init) => {
            const mode = window.__fetchMode;
            if (mode === 'real') return window.__fetchOriginal.call(window, url, init);
            if (mode === 'hold') {
                const real = window.__fetchOriginal.call(window, url); // Deliberately does not honour abort.
                return new Promise((resolve, reject) => window.__heldFetch.push({signal: init.signal,
                    release: () => real.then(resolve, reject), fail: () => reject(new Error('held source failed'))}));
            }
            if (mode === 'http') return Promise.resolve(new Response('failure', {status: 503}));
            if (mode === 'header') return Promise.resolve(new Response(new Uint8Array([1]), {headers: {'Content-Length': '4194305'}}));
            if (mode === 'stream') return Promise.resolve(new Response(new ReadableStream({
                start(controller) { controller.enqueue(new Uint8Array(4194304)); controller.enqueue(new Uint8Array([1])); controller.close(); }
            })));
            if (mode === 'read-error') return Promise.resolve(new Response(new ReadableStream({start(controller) { controller.error(new Error('stream refused')); }})));
            if (mode === 'changed') return Promise.resolve(new Response(new Uint8Array([255])));
            throw new Error('Unexpected test mode');
        };
    });
    const hold = async () => {
        await page.evaluate(() => { window.__fetchMode = 'hold'; window.__heldFetch = []; });
        await submit(); await page.waitForFunction(() => window.__heldFetch.length === 1);
        assert(await cold());
    };
    const release = async (fail = false) => {
        await page.evaluate(fail => fail ? window.__heldFetch[0].fail() : window.__heldFetch[0].release(), fail);
        await page.waitForTimeout(180);
    };
    const real = async () => { await page.evaluate(() => window.__fetchMode = 'real'); await restored(await submit()); };
    try {
        // A current completion appends its own reply after an intervening camera command.
        await command(page, 'Unload Sources'); await hold(); await command(page, 'View Isometric');
        const cameraEntry = (await history()).at(-1), cameraPixels = await drawing(page);
        await release();
        await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Editable sources restored; display retained.');
        assert.equal((await history()).at(-2), cameraEntry); assert(await loaded());
        assert.equal(await drawing(page), cameraPixels);
        // An old failure cannot consume or abort the newer request still waiting.
        await command(page, 'Unload Sources'); await hold(); const newerBefore = await submit();
        await page.waitForFunction(() => window.__heldFetch.length === 2);
        assert(await page.evaluate(() => window.__heldFetch[0].signal.aborted && !window.__heldFetch[1].signal.aborted));
        const newerHistory = await history(); await release(true);
        assert.deepEqual(await history(), newerHistory); assert(await cold());
        assert(await page.evaluate(() => !window.__heldFetch[1].signal.aborted));
        await page.evaluate(() => window.__heldFetch[1].release()); await restored(newerBefore);
        await command(page, 'Unload Sources'); await hold();
        await command(page, 'View Isometric'); const latest = (await history()).at(-1);
        await command(page, 'Cancel Reload'); const cancelled = await history(), view = await drawing(page);
        assert(await page.evaluate(() => window.__heldFetch[0].signal.aborted));
        await release(); assert.deepEqual(await history(), cancelled); assert.equal(await drawing(page), view); assert(await cold());
        assert(cancelled.includes(latest), 'Cancellation did not overwrite the newer camera command');
        await real();
        await command(page, 'Unload Sources'); await hold(); await command(page, 'Undo');
        const undone = await drawing(page), afterUndo = await history(); await release(true);
        assert.equal(await drawing(page), undone); assert.deepEqual(await history(), afterUndo); assert(await cold());
        await command(page, 'Redo'); await real();
        // A replacement picker invalidates old reload authority immediately, even when cancelled.
        await command(page, 'Unload Sources'); await hold();
        const picker = page.waitForEvent('filechooser'); await command(page, 'Open Replace'); await picker;
        await page.locator('#open').evaluate(input => input.dispatchEvent(new Event('cancel', {bubbles: true})));
        assert(await page.evaluate(() => window.__heldFetch[0].signal.aborted));
        const afterPicker = await history(); await release(); assert.deepEqual(await history(), afterPicker); assert(await cold());
        await real();
        // Close must release the real URL despite a delayed task holding the flight object.
        await command(page, 'Unload Sources'); await hold();
        const urls = (await data('data-source-origins')).map(row => row[3]);
        await command(page, 'Close'); const afterClose = await history(), empty = await drawing(page);
        assert(await page.evaluate(() => window.__heldFetch[0].signal.aborted));
        const revoked = await page.evaluate(() => window.__revokedReload);
        assert(urls.every(url => revoked.includes(url)));
        await release(); assert.deepEqual(await history(), afterClose); assert.equal(await drawing(page), empty);
        assert.equal((await data('data-row-metadata')).length, 0);
        await page.evaluate(() => window.__fetchMode = 'real');
        const choose = page.waitForEvent('filechooser'); await command(page, 'Open'); await (await choose).setFiles(specimen);
        await page.waitForFunction(() => document.getElementById('status')?.textContent === 'File imported. Undo removes the entire import.');
        await command(page, 'Select Next'); await command(page, 'Select Next'); await command(page, 'Move 0.35,0,0.25');
        await command(page, 'View Isometric'); await command(page, 'Orbit Right'); await command(page, 'Orbit Up');
        await command(page, 'Move 1.92,0,0.93'); await command(page, 'Fit'); await command(page, 'Unload Sources');
        const frozen = await drawing(page), rowValues = await data('data-row-metadata'), counters = await data('data-gpu-stats');
        for (const [mode, message] of [['http', 'Reload HTTP 503'], ['header', 'exceeds 4 MiB'], ['stream', 'exceeds 4 MiB'], ['read-error', 'Cannot read source'], ['changed', 'version changed']]) {
            await page.evaluate(mode => window.__fetchMode = mode, mode); const before = await submit();
            await page.waitForFunction(text => document.getElementById('status')?.textContent?.includes(text), message);
            assert.equal((await history()).length, Math.min(200, before + 2)); assert(await cold());
            assert.equal(await drawing(page), frozen); assert.deepEqual(await data('data-row-metadata'), rowValues);
            assert.deepEqual(await data('data-gpu-stats'), counters);
        }
        await real(); assert.equal(await drawing(page), frozen); assert.deepEqual(await data('data-gpu-stats'), counters);
    } finally {
        await page.evaluate(() => {
            window.fetch = window.__fetchOriginal; URL.revokeObjectURL = window.__revokeOriginal;
            delete window.__fetchOriginal; delete window.__heldFetch; delete window.__fetchMode;
            delete window.__revokeOriginal; delete window.__revokedReload;
        });
    }
};
