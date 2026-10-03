const assert = require('node:assert/strict');
const path = require('node:path');
module.exports = async (page, helpers) => {
    await require('./gpu-fault.cjs')(page, helpers);
    const {command, step} = helpers;
    const ready = () => page.waitForFunction(() => window.wasmBindings?.runtime_running?.() && document.querySelector('canvas')?.hasAttribute('data-command-ui'));
    const choose = async () => {
        const picker = page.waitForEvent('filechooser'); await command(page, 'Open Replace');
        await (await picker).setFiles(path.resolve('target', `course-${step.id}`, 'sample.pb'));
        await page.waitForFunction(() => document.getElementById('status')?.textContent === 'Document replaced. Undo restores the previous document.');
    };
    await page.addInitScript(() => {
        // Back-navigation checks also visit about:blank, which has no WebGPU interfaces.
        if (typeof GPUAdapter === 'undefined') return;
        const lost = new WeakSet(), owners = new WeakMap();
        window.__lossProbe = {devices: [], calls: []};
        const request = GPUAdapter.prototype.requestDevice;
        GPUAdapter.prototype.requestDevice = async function (...args) {
            const device = await request.apply(this, args); owners.set(device.queue, device);
            window.__lossProbe.devices.push(device); device.lost.then(() => lost.add(device)); return device;
        };
        for (const [prototype, methods] of [
            [GPUQueue.prototype, ['writeBuffer', 'writeTexture', 'submit']],
            [GPUDevice.prototype, ['createBuffer', 'createTexture', 'createCommandEncoder', 'createBindGroup', 'createRenderPipeline', 'createShaderModule']],
            [GPUCanvasContext.prototype, ['configure', 'getCurrentTexture']],
        ]) for (const method of methods) {
            const original = prototype[method];
            prototype[method] = function (...args) {
                const device = owners.get(this) || args[0]?.device || this;
                if (method === 'configure') owners.set(this, device);
                if (lost.has(device)) window.__lossProbe.calls.push(method);
                return original.apply(this, args);
            };
        }
    });
    await page.reload(); await ready(); await choose(); await command(page, 'Select Next'); await command(page, 'Unload Sources');
    const urls = [...new Set(JSON.parse(await page.locator('canvas').getAttribute('data-source-origins')).map(row => row[3]))];
    await page.evaluate(() => {
        window.__lossRevoked = []; window.__lossRemoved = 0;
        const revoke = URL.revokeObjectURL, remove = EventTarget.prototype.removeEventListener;
        URL.revokeObjectURL = function (url) { window.__lossRevoked.push(url); return revoke.call(URL, url); };
        EventTarget.prototype.removeEventListener = function (...args) {
            if (this === window || this === document || this === document.querySelector('canvas')) window.__lossRemoved++;
            return remove.apply(this, args);
        };
        const fetch = window.fetch;
        window.fetch = function (url, options) {
            const body = fetch.call(this, url).then(value => ({value}), error => ({error}));
            return new Promise((resolve, reject) => { window.__lossSignal = options.signal;
                window.__lossRelease = () => body.then(result => result.error ? reject(result.error) : resolve(result.value)); });
        };
    });
    await command(page, 'Reload Sources'); await page.waitForFunction(() => typeof window.__lossRelease === 'function');
    await page.evaluate(() => window.__lossProbe.devices[0].destroy());
    await page.waitForFunction(() => window.wasmBindings.runtime_running() === false && document.getElementById('status')?.textContent.includes('WebGPU device lost'));
    assert(await page.locator('#status').isVisible());
    assert(await page.evaluate(() => window.__lossSignal.aborted));
    assert.equal(await page.evaluate(() => window.__lossRemoved), 18);
    const revoked = await page.evaluate(() => window.__lossRevoked);
    assert(urls.every(url => revoked.filter(value => value === url).length === 1));
    const message = await page.locator('#status').textContent();
    await page.setViewportSize({width: 880, height: 730}); await page.mouse.click(10, 10);
    await page.keyboard.type('Background'); await page.keyboard.press('Enter'); await page.mouse.wheel(0, 200);
    await page.mouse.move(100, 100); await page.mouse.down({button:'right'}); await page.mouse.move(180, 150); await page.mouse.up({button:'right'});
    await page.evaluate(() => { window.dispatchEvent(new Event('viewer-reload')); window.dispatchEvent(new Event('viewer-file')); window.dispatchEvent(new Event('resize')); return window.__lossRelease(); });
    await page.waitForTimeout(150);
    assert.deepEqual(await page.evaluate(() => window.__lossProbe.calls), [], 'Known GPU loss blocks uploads, allocations, surface work and submissions');
    assert.equal(await page.locator('#status').textContent(), message, 'Late source completion cannot overwrite the first failure');
    await page.screenshot({path: path.resolve('target/course-checks', `${step.id}-failed.png`)});
    console.log(`${step.id}: real device.destroy, eighteen detached listeners, aborted source flight, released URLs and no further GPU work`);
    await page.setViewportSize({width: 900, height: 760}); await page.reload(); await ready(); await choose();
    await command(page, 'Select Next'); await command(page, 'Move 0.35,0,0.25'); await command(page, 'Move 0.25,0,0.15');
    await command(page, 'Move 0,-0.5,0'); await command(page, 'View Isometric'); await command(page, 'Orbit Right');
    await command(page, 'Move 0.1,0,0'); await command(page, 'Orbit Up'); await command(page, 'Orbit Right'); await command(page, 'Fit');
};
