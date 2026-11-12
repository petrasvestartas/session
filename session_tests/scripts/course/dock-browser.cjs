const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const {chromium} = require('playwright');
const {PNG} = require('pngjs');

async function main() {
    const [url, destination] = process.argv.slice(2);
    assert(url && destination,
        'Usage: dock-browser.cjs URL OUTPUT_DIRECTORY');
    fs.mkdirSync(destination, {recursive: true});
    const browser = await chromium.launch({executablePath: process.env.CHROMIUM || '/usr/bin/chromium',
        headless: true, args: ['--enable-unsafe-webgpu', '--enable-unsafe-swiftshader', '--use-angle=swiftshader',
            '--enable-features=Vulkan', '--use-vulkan=swiftshader']});
    const cases = [];
    try {
        for (const [width, height, deviceScaleFactor] of [[1280, 720, 1], [390, 844, 2]]) {
            const page = await browser.newPage({viewport: {width, height}, deviceScaleFactor});
            const errors = [];
            page.on('pageerror', error => errors.push(error.message));
            page.on('response', response => {
                if (response.status() >= 400 && !response.url().endsWith('/favicon.ico')) errors.push(`${response.status()} ${response.url()}`);
            });
            await page.goto(url);
            await page.waitForFunction(() => document.querySelector('#status')?.textContent === 'Type Help in the command field and press Enter.');
            await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
            const initial = await page.screenshot();
            const png = PNG.sync.read(initial);
            assert.equal(png.width, width * deviceScaleFactor);
            assert.equal(png.height, height * deviceScaleFactor);
            await require('../../../session_viewer/docs/journey/checks/browser/dock-steps.cjs')(page, {step: {id: '03d-input'}});
            assert.deepEqual(errors, []);
            const bytes = await page.screenshot();
            const file = `dock-${width}x${height}.png`;
            fs.writeFileSync(path.join(destination, file), bytes);
            cases.push({width, height, deviceScaleFactor, errors, file,
                sha256: crypto.createHash('sha256').update(bytes).digest('hex')});
            await page.close();
        }
        fs.writeFileSync(path.join(destination, 'dock.json'), JSON.stringify({browser: browser.version(),
            renderer: 'Vulkan SwiftShader software WebGPU', cases}, null, 2) + '\n');
        console.log('Dock typing, completion, history, pointer cancellation and scene preservation passed in both viewports');
    } finally { await browser.close(); }
}
main().catch(error => { console.error(error); process.exitCode = 1; });
