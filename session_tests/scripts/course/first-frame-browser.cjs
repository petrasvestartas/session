const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const {chromium} = require('playwright');
const {PNG} = require('pngjs');

async function main() {
    const [url, colour, destination] = process.argv.slice(2);
    assert(url && ['white', 'red'].includes(colour) && destination,
        'Usage: first-frame-browser.cjs URL white|red OUTPUT_DIRECTORY');
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
            await page.waitForFunction(() => document.querySelector('#status')?.textContent === 'The GPU painted the canvas.');
            await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
            const bytes = await page.screenshot();
            const png = PNG.sync.read(bytes);
            assert.equal(png.width, width * deviceScaleFactor);
            assert.equal(png.height, height * deviceScaleFactor);
            const expected = colour === 'white' ? [255, 255, 255, 255] : [255, 0, 0, 255];
            for (let i = 0; i < png.data.length; i++)
                assert(Math.abs(png.data[i] - expected[i % 4]) <= 2, `${colour}: unexpected channel at ${i}`);
            assert.deepEqual(errors, []);
            const file = `${colour}-${width}x${height}.png`;
            fs.writeFileSync(path.join(destination, file), bytes);
            cases.push({width, height, deviceScaleFactor, colour, errors, file,
                sha256: crypto.createHash('sha256').update(bytes).digest('hex')});
            await page.close();
        }
        fs.writeFileSync(path.join(destination, `${colour}.json`), JSON.stringify({browser: browser.version(),
            renderer: 'Vulkan SwiftShader software WebGPU', cases}, null, 2) + '\n');
        console.log(`${colour}: every pixel matches in both viewports`);
    } finally { await browser.close(); }
}
main().catch(error => { console.error(error); process.exitCode = 1; });
