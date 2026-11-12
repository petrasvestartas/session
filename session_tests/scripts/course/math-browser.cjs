// Run against the built 03a-math checkpoint. Playwright and pngjs may be supplied through NODE_PATH.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const {chromium} = require('playwright');
const {PNG} = require('pngjs');

const viewer = path.resolve(__dirname, '../../../session_viewer');
const checkpoint = path.join(viewer, 'docs/lessons/03a-math');
const output = path.resolve(process.argv[3] || path.join(viewer, 'target/math-browser'));
const digest = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
const args = ['--enable-unsafe-webgpu', '--enable-unsafe-swiftshader', '--use-angle=swiftshader',
    '--enable-features=Vulkan', '--use-vulkan=swiftshader'];

async function main() {
    fs.mkdirSync(output, {recursive: true});
    const browser = await chromium.launch({executablePath: process.env.CHROMIUM || '/usr/bin/chromium',
        headless: true, args});
    const cases = [];
    try {
        for (const [name, width, height, deviceScaleFactor] of [['landscape', 1280, 720, 1], ['portrait', 390, 844, 2]]) {
            const page = await browser.newPage({viewport: {width, height}, deviceScaleFactor});
            const errors = [];
            page.on('pageerror', error => errors.push(error.message));
            page.on('console', message => {
                if (message.type() === 'error' && !message.text().startsWith('Failed to load resource:'))
                    errors.push(message.text());
            });
            page.on('response', response => {
                if (response.status() >= 400 && !response.url().endsWith('/favicon.ico'))
                    errors.push(`${response.status()} ${response.url()}`);
            });
            await page.goto(process.argv[2] || 'http://127.0.0.1:8780/');
            const wait = step => page.waitForFunction(expected =>
                document.querySelector('#status')?.textContent === `Step ${expected}/4; animated: false`, step,
                {timeout: 60000});
            async function capture(label) {
                await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
                const png = PNG.sync.read(await page.screenshot());
                assert.equal(png.width, width * deviceScaleFactor);
                assert.equal(png.height, height * deviceScaleFactor);
                let ink = 0;
                const background = png.data.subarray(0, 3);
                for (let i = 0; i < png.data.length; i += 4)
                    if (Math.max(...background.map((value, c) => Math.abs(value - png.data[i + c]))) > 30) ink++;
                assert(ink > 500, `${name}/${label}: no meaningful visible ink (${ink} pixels)`);
                const file = `${name}-${label}.png`;
                const bytes = PNG.sync.write(png);
                fs.writeFileSync(path.join(output, file), bytes);
                return {file, sha256: digest(bytes), pixels: digest(png.data), ink};
            }
            const frames = [];
            for (let step = 1; step <= 4; step++) {
                if (step > 1) await page.keyboard.press('ArrowRight');
                await wait(step);
                frames.push(await capture(`step-${step}`));
                if (step > 1) assert.notEqual(frames[step - 1].pixels, frames[step - 2].pixels, 'Step must change visible content');
            }
            await page.keyboard.press('ArrowLeft');
            await wait(3);
            const previous = await capture('previous');
            assert.equal(previous.pixels, frames[2].pixels, 'Previous reconstructs the same matrix frame');
            await page.keyboard.press('t');
            await page.waitForTimeout(1000);
            const dark = await capture('theme');
            assert.notEqual(dark.pixels, previous.pixels, 'Theme must change the image');
            await page.keyboard.press('t');
            await page.waitForTimeout(1000);
            const restored = await capture('theme-restored');
            assert.equal(restored.pixels, previous.pixels, 'Theme round trip preserves content');
            assert.deepEqual(errors, []);
            const adapter = await page.evaluate(async () => {
                const {info} = await navigator.gpu.requestAdapter();
                return {vendor: info.vendor, architecture: info.architecture, device: info.device, description: info.description};
            });
            cases.push({name, width, height, deviceScaleFactor, adapter, errors, frames, previous, dark, restored});
            await page.close();
        }
        const sourceSha256 = {};
        function sources(directory) {
            for (const entry of fs.readdirSync(directory, {withFileTypes: true})) {
                const file = path.join(directory, entry.name);
                if (entry.isDirectory()) sources(file);
                else if (/\.(rs|toml|lock|html|yaml)$/.test(entry.name))
                    sourceSha256[path.relative(checkpoint, file).split(path.sep).join('/')] = digest(fs.readFileSync(file));
            }
        }
        sources(checkpoint);
        const inputs = {};
        for (const name of ['NewCMMath-Book.otf', 'NewCM10-Book.otf', 'NewCM10-Bold.otf', 'NewCMMono10-Book.otf',
            'logo-eth.png', 'logo-brg.png']) inputs[`assets/stage/${name}`] = digest(fs.readFileSync(path.join(viewer, 'assets/stage', name)));
        fs.writeFileSync(path.join(output, 'evidence.json'), JSON.stringify({
            checkpoint: 'docs/lessons/03a-math', scope: 'Full standalone stage core; cumulative integration remains separate',
            browser: browser.version(), args, sourceSha256, inputs, cases,
        }, null, 2) + '\n');
        console.log(`Passed ${cases.length} viewports: four visible steps, previous-frame equality and theme round trip.`);
    } finally { await browser.close(); }
}
main().catch(error => { console.error(error); process.exitCode = 1; });
