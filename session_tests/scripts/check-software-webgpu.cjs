const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const {execFileSync} = require('node:child_process');
const {chromium} = require('playwright');
const {PNG} = require('pngjs');

async function main() {
    const [url, output, requestedBackend] = process.argv.slice(2);
    assert(url && output && process.argv.length <= 5,
        'Usage: check-software-webgpu.cjs LOCAL_URL OUTPUT.json [warp|swiftshader]');
    assert(['127.0.0.1', 'localhost', '[::1]'].includes(new URL(url).hostname));
    assert.equal(require('playwright/package.json').version, '1.58.2');
    const windows = process.platform === 'win32';
    const backend = requestedBackend || (windows ? 'warp' : 'swiftshader');
    assert(['warp', 'swiftshader'].includes(backend), 'Unknown software backend');
    assert(backend !== 'warp' || windows, 'WARP requires Windows');
    const warp = backend === 'warp' ? JSON.parse(execFileSync('powershell.exe', ['-NoProfile', '-ExecutionPolicy',
        'Bypass', '-File', path.join(__dirname, 'warp-adapter.ps1')], {encoding: 'utf8'})) : null;
    const args = backend === 'warp'
        ? ['--enable-unsafe-webgpu', '--ignore-gpu-blocklist', '--use-gl=angle',
            '--use-angle=d3d11', `--use-adapter-luid=${warp.luid}`]
        : ['--enable-unsafe-webgpu', '--use-webgpu-adapter=swiftshader',
            '--use-gl=angle', '--use-angle=swiftshader', '--enable-unsafe-swiftshader'];
    const executablePath = process.env.VIEWER_BROWSER_EXECUTABLE;
    if (executablePath) assert(fs.existsSync(executablePath), 'Browser executable is missing');
    console.log(`Software probe: launching ${backend}`);
    const browser = await chromium.launch({...(executablePath ? {executablePath} : {channel: 'chrome'}),
        headless: true, args, timeout: 15000});
    try {
        console.log(`Software probe: browser ${browser.version()}`);
        const page = await browser.newPage();
        await page.goto(url, {timeout: 15000});
        console.log('Software probe: page loaded');
        const result = await page.evaluate(async () => {
            const bounded = (promise, label) => Promise.race([promise,
                new Promise((_, reject) => setTimeout(() => reject(Error(label + ' timed out')), 15000))]);
            const adapter = await bounded(navigator.gpu.requestAdapter(), 'adapter');
            if (!adapter) throw Error('No WebGPU adapter');
            const info = adapter.info;
            if (!info.isFallbackAdapter || !((info.vendor === 'microsoft' && info.architecture === 'warp')
                || /swiftshader/i.test(`${info.vendor} ${info.architecture} ${info.description}`))) throw Error('Expected a software adapter');
            const device = await bounded(adapter.requestDevice(), 'device');
            const texture = device.createTexture({size: [4, 4], format: 'rgba8unorm',
                usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC});
            const buffer = device.createBuffer({size: 1024,
                usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ});
            try {
                const encoder = device.createCommandEncoder();
                const pass = encoder.beginRenderPass({colorAttachments: [{view: texture.createView(),
                    loadOp: 'clear', storeOp: 'store', clearValue: {r: 0.2, g: 0.4, b: 0.6, a: 1}}]});
                pass.end();
                encoder.copyTextureToBuffer({texture}, {buffer, bytesPerRow: 256}, [4, 4]);
                device.queue.submit([encoder.finish()]);
                await bounded(buffer.mapAsync(GPUMapMode.READ), 'readback');
                const bytes = new Uint8Array(buffer.getMappedRange());
                const pixels = Array.from({length: 16}, (_, i) =>
                    Array.from(bytes.slice(Math.floor(i / 4) * 256 + i % 4 * 4,
                        Math.floor(i / 4) * 256 + i % 4 * 4 + 4)));
                buffer.unmap();
                const canvas = document.createElement('canvas');
                canvas.width = canvas.height = 32;
                canvas.style.cssText = 'position:fixed;left:0;top:0;width:32px;height:32px;z-index:2147483647';
                document.body.append(canvas);
                const context = canvas.getContext('webgpu');
                const format = navigator.gpu.getPreferredCanvasFormat();
                const viewFormat = format + '-srgb';
                context.configure({device, format, viewFormats: [viewFormat], alphaMode: 'opaque'});
                const present = device.createCommandEncoder();
                const clear = present.beginRenderPass({colorAttachments: [{
                    view: context.getCurrentTexture().createView({format: viewFormat}),
                    loadOp: 'clear', storeOp: 'store', clearValue: {r: 0.2, g: 0.4, b: 0.6, a: 1}}]});
                clear.end(); device.queue.submit([present.finish()]);
                await bounded(device.queue.onSubmittedWorkDone(), 'canvas completion');
                await bounded(new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))), 'canvas presentation');
                window.softwareProbeResources = {device, context};
                return {vendor: info.vendor, architecture: info.architecture,
                    device: info.device, description: info.description,
                    isFallbackAdapter: info.isFallbackAdapter, pixels};
            } finally {
                buffer.destroy();
                texture.destroy();
                if (!window.softwareProbeResources) device.destroy();
            }
        });
        console.log('Software probe: readback and presentation completed');
        if (backend === 'warp') {
            assert.equal(result.vendor, 'microsoft');
            assert.equal(result.architecture, 'warp');
        } else assert.match(JSON.stringify(result), /swiftshader/i);
        assert.equal(result.isFallbackAdapter, true);
        for (const pixel of result.pixels) assert.deepEqual(pixel, [51, 102, 153, 255]);
        const cdp = await browser.newBrowserCDPSession();
        const {gpu} = await cdp.send('SystemInfo.getInfo');
        await cdp.detach();
        if (backend === 'swiftshader') {
            assert.match(gpu.auxAttributes.glRenderer, /swiftshader/i);
            assert.match(gpu.auxAttributes.glRenderer, /vulkan/i);
        }
        const screenshot = await page.screenshot({timeout: 15000});
        const image = PNG.sync.read(screenshot);
        const pixel = [...image.data.subarray((16 * image.width + 16) * 4, (16 * image.width + 16) * 4 + 4)];
        const expected = [0.2, 0.4, 0.6].map(x => Math.round(255 * (1.055 * x ** (1 / 2.4) - 0.055))).concat(255);
        assert(pixel.every((value, i) => Math.abs(value - expected[i]) <= 1), 'Expected presented sRGB canvas pixels');
        fs.writeFileSync(output + '.png', screenshot);
        await page.evaluate(() => {
            window.softwareProbeResources.context.unconfigure();
            window.softwareProbeResources.device.destroy();
        });
        fs.writeFileSync(output, JSON.stringify({browser: browser.version(), playwright: '1.58.2',
            url, backend, executablePath: executablePath || null, args, warp, gpu, result,
            canvas: {pixel, expected, screenshot: output + '.png'}}, null, 2) + '\n');
        console.log(`PASS: ${backend}, 16 texture pixels, canvas completion and presented pixels`);
    } finally {
        await browser.close();
    }
}

main().catch(error => {console.error(error); process.exitCode = 1;});
