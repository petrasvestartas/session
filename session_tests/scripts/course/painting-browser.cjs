const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const {chromium} = require('playwright');
const {PNG} = require('pngjs');

async function main() {
    const [url, colour, destination] = process.argv.slice(2);
    assert(url && ['load', 'clear'].includes(colour) && destination,
        'Usage: painting-browser.cjs URL load|clear OUTPUT_DIRECTORY');
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
            await page.waitForFunction(() => document.querySelector('#status')?.textContent === 'Our command line is drawn. Input comes next.');
            await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
            const bytes = await page.screenshot();
            const png = PNG.sync.read(bytes);
            assert.equal(png.width, width * deviceScaleFactor);
            assert.equal(png.height, height * deviceScaleFactor);
            const colourAt = (x, y) => [...png.data.subarray((y * png.width + x) * 4, (y * png.width + x) * 4 + 3)];
            const pink = [0.9, 0.25, 0.45].map(c => Math.round(255 * (1.055 * c ** (1 / 2.4) - 0.055)));
            const close = (a, b) => a.every((c, i) => Math.abs(c - b[i]) <= 2);
            let triangles = 0, ink = 0;
            for (let y = 0; y < png.height; y++) for (let x = 0; x < png.width; x++) {
                const pixel = colourAt(x, y);
                if (close(pixel, pink)) triangles++;
                if (y > png.height - 30 * deviceScaleFactor && x < 100 * deviceScaleFactor && pixel.every(c => c < 100)) ink++;
            }
            assert(ink > 50 * deviceScaleFactor, 'The Noto font must paint visible Command label ink');
            assert(close(colourAt(png.width / 2, png.height - 15 * deviceScaleFactor), [255, 255, 255]));
            if (colour === 'load') {
                assert(close(colourAt(png.width / 2, png.height / 2), pink), 'The scene must survive the UI pass');
                const coverage = triangles / (png.width * png.height);
                assert(coverage > 0.16 && coverage < 0.17, `Wrong triangle coverage: ${coverage}`);
            } else {
                assert.equal(triangles, 0, 'Clearing in the UI pass must erase the scene triangle');
                const loaded = PNG.sync.read(fs.readFileSync(path.join(destination, `load-${width}x${height}.png`)));
                const dock = (png.height - 30 * deviceScaleFactor) * png.width * 4;
                assert.deepEqual(png.data.subarray(dock), loaded.data.subarray(dock), 'The experiment must preserve every dock pixel');
                assert(close(colourAt(png.width / 2, png.height / 2), [255, 255, 255]));
            }
            assert.deepEqual(errors, []);
            const file = `${colour}-${width}x${height}.png`;
            fs.writeFileSync(path.join(destination, file), bytes);
            cases.push({width, height, deviceScaleFactor, colour, errors, file,
                sha256: crypto.createHash('sha256').update(bytes).digest('hex')});
            await page.close();
        }
        fs.writeFileSync(path.join(destination, `${colour}.json`), JSON.stringify({browser: browser.version(),
            renderer: 'Vulkan SwiftShader software WebGPU', cases}, null, 2) + '\n');
        console.log(`${colour}: triangle coverage and Noto label verified in both viewports`);
    } finally { await browser.close(); }
}
main().catch(error => { console.error(error); process.exitCode = 1; });
