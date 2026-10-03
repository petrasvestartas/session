const assert = require('node:assert/strict');
const path = require('node:path');
module.exports = async (page, helpers) => {
    await require('./listener-owner.cjs')(page, helpers);
    const {command, drawing, step} = helpers, specimen = path.resolve('target', `course-${step.id}`, 'sample.pb');
    const data = async key => JSON.parse(await page.locator('canvas').getAttribute(key));
    const ready = () => page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && document.querySelector('canvas')?.hasAttribute('data-command-ui'));
    const choose = async verb => { const picker = page.waitForEvent('filechooser'); await command(page, verb); await (await picker).setFiles(specimen); };
    const replaced = () => page.waitForFunction(() => document.getElementById('status')?.textContent === 'Document replaced. Undo restores the previous document.');
    const expected = [['canvas', 'pointerdown'], ['canvas', 'pointermove'], ['canvas', 'pointerup'], ['canvas', 'pointercancel'], ['canvas', 'lostpointercapture'],
        ['canvas', 'contextmenu'], ['canvas', 'keydown'], ['canvas', 'keyup'], ['canvas', 'blur'], ['canvas', 'wheel'], ['window', 'resize'], ['window', 'blur'],
        ['document', 'change'], ['document', 'cancel'], ['window', 'viewer-file'], ['window', 'viewer-file-error'], ['window', 'viewer-reload'], ['window', 'pagehide']].map(row => row.join(':')).sort();
    const hooks = () => page.evaluate(() => {
        window.__runtimeRemoved = []; window.__runtimeRevoked = []; window.__runtimeSubmits = 0;
        window.__runtimeRemove = EventTarget.prototype.removeEventListener; window.__runtimeRevoke = URL.revokeObjectURL; window.__runtimeSubmit = GPUQueue.prototype.submit;
        EventTarget.prototype.removeEventListener = function (name, callback, options) {
            const kind = this === window ? 'window' : this === document ? 'document' : this === document.querySelector('canvas') ? 'canvas' : null;
            if (kind) window.__runtimeRemoved.push(kind + ':' + name);
            return window.__runtimeRemove.call(this, name, callback, options);
        };
        URL.revokeObjectURL = function (url) { window.__runtimeRevoked.push(url); return window.__runtimeRevoke.call(URL, url); };
        GPUQueue.prototype.submit = function (commands) { window.__runtimeSubmits++; return window.__runtimeSubmit.call(this, commands); };
    });
    const stop = async () => {
        await page.evaluate(() => window.dispatchEvent(new Event('pagehide')));
        await page.waitForFunction(() => window.wasmBindings.runtime_running() === false);
        assert.deepEqual((await page.evaluate(() => window.__runtimeRemoved)).sort(), expected);
    };
    await hooks();
    await command(page, 'Close'); assert(await page.evaluate(() => window.wasmBindings.runtime_running()), 'Document Close retains the viewer');
    await choose('Open Replace'); await replaced(); await command(page, 'Select Next');
    await command(page, 'Unload Sources'); const urls = [...new Set((await data('data-source-origins')).map(row => row[3]))];
    await page.evaluate(() => {
        window.__runtimeFetch = window.fetch;
        window.fetch = function (url, options) {
            const body = window.__runtimeFetch.call(this, url).then(value => ({value}), error => ({error}));
            return new Promise((resolve, reject) => { window.__runtimeSignal = options.signal;
                window.__runtimeRelease = () => body.then(result => result.error ? reject(result.error) : resolve(result.value)); });
        };
    });
    try {
        await command(page, 'Reload Sources'); await page.waitForFunction(() => typeof window.__runtimeRelease === 'function');
        await stop(); assert(await page.evaluate(() => window.__runtimeSignal.aborted));
        const revoked = await page.evaluate(() => window.__runtimeRevoked);
        assert(urls.every(url => revoked.filter(value => value === url).length === 1));
        const image = await drawing(page), messages = (await data('data-command-ui')).history, counters = await page.evaluate(() => window.__runtimeSubmits);
        await page.mouse.click(10, 10); await page.keyboard.type('Background'); await page.keyboard.press('Enter');
        await page.mouse.wheel(0, 200); await page.evaluate(() => {
            window.dispatchEvent(new Event('viewer-reload')); window.dispatchEvent(new Event('viewer-file')); window.dispatchEvent(new Event('resize'));
            window.dispatchEvent(new Event('pagehide')); return window.__runtimeRelease();
        });
        await page.waitForTimeout(150);
        assert.equal(await page.evaluate(() => window.__runtimeSubmits), counters, 'Disposed input and late replies submit no frames');
        assert.equal(await drawing(page), image); assert.deepEqual((await data('data-command-ui')).history, messages);
        assert.deepEqual((await page.evaluate(() => window.__runtimeRemoved)).sort(), expected, 'Repeated exit does not remove handlers twice');
    } finally {
        await page.evaluate(() => { window.fetch = window.__runtimeFetch; EventTarget.prototype.removeEventListener = window.__runtimeRemove;
            URL.revokeObjectURL = window.__runtimeRevoke; GPUQueue.prototype.submit = window.__runtimeSubmit; });
    }
    // Dispose another live runtime during a real file read, before it can adopt any source URL.
    await page.reload(); await ready(); await hooks();
    await page.evaluate(() => {
        window.__runtimeRead = File.prototype.arrayBuffer; window.__runtimeCreate = URL.createObjectURL; window.__runtimeCreated = [];
        File.prototype.arrayBuffer = function () {
            const body = window.__runtimeRead.call(this).then(value => ({value}), error => ({error}));
            return new Promise((resolve, reject) => { window.__runtimeReadRelease = () => body.then(result => result.error ? reject(result.error) : resolve(result.value)); });
        };
        URL.createObjectURL = function (blob) { const url = window.__runtimeCreate.call(URL, blob); window.__runtimeCreated.push(url); return url; };
    });
    try {
        await choose('Open Replace'); await page.waitForFunction(() => typeof window.__runtimeReadRelease === 'function');
        await stop(); const image = await drawing(page), messages = (await data('data-command-ui')).history, counters = await page.evaluate(() => window.__runtimeSubmits);
        await page.evaluate(() => window.__runtimeReadRelease()); await page.waitForTimeout(150);
        assert.deepEqual(await page.evaluate(() => window.__runtimeCreated), [], 'Cancelled read never adopts a Blob URL');
        assert.equal(await page.evaluate(() => window.__runtimeSubmits), counters); assert.equal(await drawing(page), image);
        assert.deepEqual((await data('data-command-ui')).history, messages);
    } finally {
        await page.evaluate(() => { File.prototype.arrayBuffer = window.__runtimeRead; URL.createObjectURL = window.__runtimeCreate;
            EventTarget.prototype.removeEventListener = window.__runtimeRemove; URL.revokeObjectURL = window.__runtimeRevoke; GPUQueue.prototype.submit = window.__runtimeSubmit; });
    }
    // Restart this same test page and reproduce the final native proof view.
    await page.reload(); await ready(); await choose('Open Replace'); await replaced();
    await command(page, 'Select Next'); await command(page, 'Move 0.35,0,0.25'); await command(page, 'View Isometric'); await command(page, 'Fit');
    await command(page, 'Move 0.25,0,0.15'); await command(page, 'Move 0,-0.5,0');
    await command(page, 'View Isometric'); await command(page, 'Orbit Right'); await command(page, 'Fit');
};
