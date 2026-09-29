const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const {chromium} = require('playwright');

async function main() {
    const output = process.env.VIEWER_TEST_OUTPUT || 'target/device-loss';
    await fs.mkdir(output, {recursive: true});
    const args = process.env.VIEWER_CHROME_ARGS ? JSON.parse(process.env.VIEWER_CHROME_ARGS)
        : ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE',
            '--disable-vulkan-surface', '--ozone-platform=x11'];
    const browser = await chromium.launch({channel: 'chrome', headless: false, args});
    try {
        const page = await browser.newPage({viewport: {width: 1100, height: 760}});
        await page.route('**/view_local.yaml', route => route.fulfill({
            contentType: 'application/yaml', body: 'name: Device loss check\nitems:\n  - file: pb/view_local_boxes.pb\n',
        }));
        const errors = [];
        page.on('pageerror', error => errors.push(error.message));
        page.on('console', message => {
            if (message.type() === 'error') errors.push(`${message.text()} ${message.location().url}`);
        });
        await page.addInitScript(() => {
            const lost = new WeakSet();
            const owners = new WeakMap();
            window.gpuProbe = {devices: [], callsAfterLoss: []};
            const request = GPUAdapter.prototype.requestDevice;
            GPUAdapter.prototype.requestDevice = async function (...args) {
                const device = await request.apply(this, args);
                owners.set(device.queue, device);
                window.gpuProbe.devices.push(device);
                device.lost.then(() => lost.add(device));
                return device;
            };
            for (const [prototype, methods] of [
                [GPUQueue.prototype, ['writeBuffer', 'writeTexture', 'submit']],
                [GPUDevice.prototype, ['createBuffer', 'createTexture', 'createCommandEncoder', 'createBindGroup']],
                [GPUCanvasContext.prototype, ['configure']],
            ]) {
                for (const method of methods) {
                    const original = prototype[method];
                    prototype[method] = function (...args) {
                        const device = owners.get(this) || args[0]?.device || this;
                        if (lost.has(device)) window.gpuProbe.callsAfterLoss.push(method);
                        return original.apply(this, args);
                    };
                }
            }
        });
        const url = new URL(process.env.VIEWER_URL || 'http://127.0.0.1:8789/');
        url.search = '?data=off&inspect=1&dpr=1&recovered=device-loss-test';
        await page.goto(url.href);
        await page.waitForFunction(() => {
            const raw = document.querySelector('#canvas')?.getAttribute('data-viewer-inspection');
            return raw && JSON.parse(raw).objects > 0;
        }, null, {timeout: 60000});
        assert.deepEqual(errors, [], 'the initial scene must load without browser errors');
        await page.screenshot({path: `${output}/before.png`});
        const download = page.waitForEvent('download');
        await page.evaluate(() => window.gpuProbe.devices[0].destroy());
        const file = await download;
        await file.saveAs(`${output}/report.json`);
        await page.locator('#viewer-error').waitFor({state: 'visible'});
        const report = JSON.parse(await fs.readFile(`${output}/report.json`, 'utf8'));
        assert.match(report.failure.message, /device lost/);
        assert.equal(report.outcome, 'failed');
        assert(report.adapter);

        await page.setViewportSize({width: 900, height: 600});
        await page.locator('#canvas').focus();
        await page.keyboard.press('g');
        await page.evaluate(() => window.wasmBindings?.reload_scene?.());
        await page.waitForTimeout(500);
        assert.deepEqual(await page.evaluate(() => window.gpuProbe.callsAfterLoss), [],
            'loss must stop uploads, UI resources, surface configuration and submissions');
        await page.screenshot({path: `${output}/failed.png`});

        await page.reload();
        await page.locator('#viewer-diagnostics').waitFor({state: 'visible'});
        const recovered = page.waitForEvent('download');
        await page.getByRole('button', {name: 'Download diagnostic report'}).click();
        await (await recovered).saveAs(`${output}/recovered.json`);
        assert.equal(JSON.parse(await fs.readFile(`${output}/recovered.json`, 'utf8')).failure.message,
            report.failure.message);
        await fs.writeFile(`${output}/result.json`, JSON.stringify({browser: browser.version(), args,
            deviceLoss: 'device.destroy()', callsAfterLoss: [], reportRecovered: true}, null, 2));
        console.log('Device loss: GPU work stopped, report downloaded and recovered after reload.');
    } finally {
        await browser.close();
    }
}

main().catch(error => { console.error(error); process.exitCode = 1; });
