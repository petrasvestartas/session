const assert = require('node:assert/strict');
const path = require('node:path');
module.exports = async (page, {command, drawing, step}) => {
    const specimen = path.resolve('target', `course-${step.id}`, 'sample.pb');
    const data = async key => JSON.parse(await page.locator('canvas').getAttribute(key));
    const placements = () => data('data-row-placements');
    const selected = () => page.locator('canvas').getAttribute('data-selected-id');
    const history = async () => (await data('data-command-ui')).history;
    const submit = async line => {
        const ui = await data('data-command-ui');
        const field = ui.controls.find(control => control.key === 'command/input');
        const box = await page.locator('canvas').boundingBox(), r = field.rect;
        await page.mouse.click(box.x + (r[0] + r[2]) / 2, box.y + (r[1] + r[3]) / 2);
        await page.keyboard.press('ControlOrMeta+A'); await page.keyboard.type(line); await page.keyboard.press('Enter');
        await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).command === '');
    };
    const reset = async () => {
        const picker = page.waitForEvent('filechooser'); await command(page, 'Open Replace');
        await (await picker).setFiles(specimen);
        await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Document replaced. Undo restores the previous document.');
        await command(page, 'Select Next'); await command(page, 'Move 0.35,0,0.25');
        await command(page, 'View Isometric'); await command(page, 'Fit');
    };
    await page.evaluate(() => {
        window.__autoFetch = window.fetch; window.__autoHeld = []; window.__autoHold = true;
        window.fetch = function (url, options) {
            if (!window.__autoHold) return window.__autoFetch.call(this, url, options);
            const body = window.__autoFetch.call(this, url); // real source, intentionally ignoring abort only for held-response delivery
            return new Promise(resolve => window.__autoHeld.push({signal: options.signal, release: () => body.then(resolve)}));
        };
    });
    const hold = async line => {
        await page.evaluate(() => { window.__autoHeld = []; window.__autoHold = true; });
        await submit(line); await page.waitForFunction(() => window.__autoHeld.length === 1);
    };
    const release = () => page.evaluate(() => window.__autoHeld[0].release());
    try {
        await reset(); const original = await placements(), id = await selected();
        await command(page, 'Move 0.25,0,0.15'); const expected = await placements();
        await command(page, 'Undo'); assert.deepEqual(await placements(), original);
        await command(page, 'Unload Sources'); const frozen = await drawing(page);
        await hold('Move 0.25,0,0.15'); assert.deepEqual(await placements(), original); assert.equal(await drawing(page), frozen);
        await command(page, 'Select Next'); await command(page, 'Orbit Right');
        const laterSelection = await selected(), laterCamera = await data('data-camera-matrix'); assert.notEqual(laterSelection, id);
        await release(); await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Move completed after source reload.');
        assert.deepEqual(await placements(), expected); assert.equal(await selected(), laterSelection);
        assert.deepEqual(await data('data-camera-matrix'), laterCamera);
        assert((await history()).at(-1).startsWith('> Move\n'));
        await command(page, 'Undo'); assert.deepEqual(await placements(), original, 'One Undo reverses only the captured Move');
        await command(page, 'Redo'); assert.deepEqual(await placements(), expected);
        await command(page, 'Undo');
        await reset(); const beforeDelete = await placements(), target = await selected();
        await command(page, 'Unload Sources'); await hold('Delete'); await command(page, 'Select Next');
        const afterSelect = await selected(); await release();
        await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Delete completed after source reload.');
        assert.equal(await selected(), afterSelect); assert.deepEqual(await placements(), beforeDelete.filter(row => row[0] !== target));
        await command(page, 'Undo'); assert.deepEqual(await placements(), beforeDelete);
        await reset(); await command(page, 'Move 0.25,0,0.15'); const saveState = await placements();
        await command(page, 'Unload Sources'); let downloads = 0; const onDownload = () => downloads++; page.on('download', onDownload);
        try {
            await hold('Save'); await page.waitForTimeout(100); assert.equal(downloads, 0);
            await command(page, 'Orbit Up'); const saveCamera = await data('data-camera-matrix');
            const download = page.waitForEvent('download'); await release(); assert.equal(await (await download).failure(), null);
            await page.waitForFunction(() => document.getElementById('status')?.textContent?.startsWith('Saved editable document'));
            assert.equal(downloads, 1); assert.deepEqual(await placements(), saveState); assert.deepEqual(await data('data-camera-matrix'), saveCamera);
            assert((await data('data-source-residency')).every(row => row[1]));
            await command(page, 'Undo'); assert.notDeepEqual(await placements(), saveState, 'Save consumes no edit history');
        } finally { page.off('download', onDownload); }
        await reset(); const newerBaseline = await placements();
        await command(page, 'Move 0,0,0.15'); const newerExpected = await placements(); await command(page, 'Undo');
        await command(page, 'Unload Sources'); await hold('Move 0.25,0,0.15'); await submit('Move 0,0,0.15');
        await page.waitForFunction(() => window.__autoHeld.length === 2);
        assert(await page.evaluate(() => window.__autoHeld[0].signal.aborted));
        const newerMessages = await history(); await release(); await page.waitForTimeout(120);
        assert.deepEqual(await placements(), newerBaseline); assert.deepEqual(await history(), newerMessages);
        await page.evaluate(() => window.__autoHeld[1].release());
        await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Move completed after source reload.');
        assert.deepEqual(await placements(), newerExpected, 'Only the newer captured edit may replay');
        for (const cancel of ['Cancel Reload', 'Undo', 'Close']) {
            await reset(); await command(page, 'Unload Sources'); await hold('Move 0.25,0,0.15');
            await command(page, cancel); const rows = await placements(), view = await drawing(page), messages = await history();
            assert(await page.evaluate(() => window.__autoHeld[0].signal.aborted)); await release(); await page.waitForTimeout(120);
            assert.deepEqual(await placements(), rows); assert.equal(await drawing(page), view); assert.deepEqual(await history(), messages);
        }
        // Finish at the exact rendered scene used by the native frame.
        await page.evaluate(() => { window.__autoHold = false; }); await reset();
        await command(page, 'Unload Sources'); await submit('Move 0.25,0,0.15');
        await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Move completed after source reload.');
        await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).history.at(-1)?.startsWith('> Move\n'));
        await submit('Move 0,-0.5,0');
        await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).history.at(-1)?.startsWith('> Move 0,-0.5,0\n'));
        await command(page, 'Fit');
    } finally {
        await page.evaluate(() => { window.fetch = window.__autoFetch; delete window.__autoFetch; delete window.__autoHeld; delete window.__autoHold; });
    }
};
