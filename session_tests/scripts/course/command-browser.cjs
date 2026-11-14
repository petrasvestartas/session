const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const {chromium} = require('playwright');
const {PNG} = require('pngjs');

const output = path.resolve(process.argv[3]);
const url = new URL(process.argv[2]);
const hash = value => crypto.createHash('sha256').update(value).digest('hex');
const args = JSON.parse(process.env.MATH_BROWSER_ARGS);
let captureBytes = 0;
let createdOutput = false;

async function main() {
    assert(!fs.existsSync(output), 'Preserve earlier captures');
    fs.mkdirSync(output, {recursive: true});
    createdOutput = true;
    const browser = await chromium.launch({executablePath: process.env.CHROMIUM, headless: true, args});
    const cases = [];
    try {
        for (const [name, width, height, dpr] of [['landscape', 1280, 720, 1], ['portrait', 390, 844, 2]]) {
            const page = await browser.newPage({viewport: {width, height}, deviceScaleFactor: dpr});
            const errors = [];
            page.on('pageerror', error => errors.push(error.message));
            page.on('console', message => {
                if (message.type() === 'error' && !message.text().startsWith('Failed to load resource:')) errors.push(message.text());
            });
            page.on('response', response => {
                if (response.status() >= 400 && !response.url().endsWith('/favicon.ico')) errors.push(`${response.status()} ${response.url()}`);
            });
            url.searchParams.set('tutorial', 'viewer');
            await page.goto(url.href);
            await page.waitForFunction(() => document.querySelector('#status')?.textContent === 'Change the background. The triangle keeps its shape and colour.', null, {timeout: 60000});
            await page.locator('canvas').focus();
            async function capture(label) {
                await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
                const bytes = await page.screenshot();
                captureBytes += bytes.length;
                assert(captureBytes <= 16 * 1024 * 1024, 'Capture budget');
                const png = PNG.sync.read(bytes);
                assert.equal(png.width, width * dpr);
                assert.equal(png.height, height * dpr);
                const file = `${name}-${label}.png`;
                fs.writeFileSync(path.join(output, file), bytes);
                // The upper half contains the triangle and clear colour, above the dock.
                const upper = png.data.subarray(0, png.width * Math.floor(png.height / 2) * 4);
                const triangleMask = Buffer.alloc(upper.length / 4);
                let trianglePixels = 0;
                for (let i = 0; i < triangleMask.length; i++) {
                    if (upper[i * 4] > upper[i * 4 + 2] + 20 && upper[i * 4 + 2] > upper[i * 4 + 1] + 20) {
                        triangleMask[i] = 1;
                        trianglePixels++;
                    }
                }
                const pixel = (x, y) => Array.from(png.data.subarray((Math.floor(y * png.height) * png.width + Math.floor(x * png.width)) * 4, (Math.floor(y * png.height) * png.width + Math.floor(x * png.width)) * 4 + 3));
                return {file, sha256: hash(bytes), pixels: hash(png.data), upper: hash(upper), triangleMask: hash(triangleMask), trianglePixels, corner: pixel(0.05, 0.05), triangle: pixel(0.5, 0.4)};
            }
            const initial = await capture('white');
            assert(initial.corner.every(c => c === 255), 'Initial background is white');
            assert(initial.triangle[0] > initial.triangle[2] && initial.triangle[2] > initial.triangle[1], 'Visible pink triangle');
            assert(initial.trianglePixels > 500, 'Triangle covers a meaningful area');
            await page.keyboard.type('Background');
            const typed = await capture('typed');
            assert.equal(typed.upper, initial.upper, 'Typing does not submit');
            await page.keyboard.press('Enter');
            const grey = await capture('grey');
            assert(grey.corner.every(c => c >= 240 && c < 255), 'Linear 0.9 clear colour appears light grey in sRGB');
            assert.deepEqual(grey.triangle, initial.triangle, 'Submission preserves triangle colour');
            assert.equal(grey.triangleMask, initial.triangleMask, 'Submission preserves visible triangle shape and position');
            await page.keyboard.type('Background');
            await page.keyboard.press('Enter');
            const restored = await capture('restored');
            assert.equal(restored.upper, initial.upper, 'Two submissions restore every upper-frame pixel');
            await page.keyboard.type('Help');
            await page.keyboard.press('Enter');
            const help = await capture('help');
            assert.equal(help.upper, initial.upper, 'Other commands do not toggle background');
            url.searchParams.set('tutorial', 'math');
            await page.goto(url.href);
            await page.waitForFunction(() => document.querySelector('#status')?.textContent === 'Step 1/4; animated: false', null, {timeout: 60000});
            const math = await capture('math');
            await page.keyboard.press('ArrowRight');
            await page.waitForFunction(() => document.querySelector('#status')?.textContent === 'Step 2/4; animated: false');
            const next = await capture('math-next');
            assert.notEqual(next.pixels, math.pixels, 'Retained mathematics responds to navigation');
            assert.deepEqual(errors, []);
            cases.push({name, width, height, dpr, initial, typed, grey, restored, help, math, next, errors});
            await page.close();
        }
        fs.writeFileSync(path.join(output, 'result.json'), JSON.stringify({checkerSha256: hash(fs.readFileSync(__filename)), args, cases, scope: 'Desktop and upright viewport emulation; not physical-phone acceptance'}, null, 2) + '\n');
    } finally {
        await browser.close();
    }
}
main().catch(error => {
    if (createdOutput) fs.writeFileSync(path.join(output, 'failure.txt'), String(error.stack));
    console.error(error);
    process.exitCode = 1;
});
