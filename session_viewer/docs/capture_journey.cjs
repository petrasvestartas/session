const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const assert = require('node:assert/strict');
const {chromium} = require('playwright');
const {PNG} = require('pngjs');
const course = JSON.parse(fs.readFileSync('docs/journey/course.json', 'utf8'));
const builds = JSON.parse(fs.readFileSync('target/course-checks/results.json', 'utf8'));
const output = 'docs/screenshots/journey';
const hash = bytes => crypto.createHash('sha256').update(bytes).digest('hex');

// Drawing pixels exclude CSS borders and focus outlines from neighbouring controls.
async function drawing(page) {
    const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
    const pixels = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
    return hash(pixels.data);
}

async function fitted(page) {
    const button = page.getByRole('button', {name: 'Fit scene', exact: true});
    await button.click();
    await page.waitForTimeout(200);
    const result = await drawing(page);
    await button.click();
    await page.waitForTimeout(200);
    assert.equal(await drawing(page), result, 'Repeated Fit must leave the view unchanged');
    await page.getByRole('button', {name: 'Look right', exact: true}).click();
    await page.waitForTimeout(200);
    assert.notEqual(await drawing(page), result, 'Pan must move the fitted picture');
    await button.click();
    await page.waitForTimeout(200);
    assert.equal(await drawing(page), result, 'Fit must recover the same view after panning');

    const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
    const {data, width, height} = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
    let count = 0;
    for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
            const i = (y * width + x) * 4;
            if (data[i] === data[0] && data[i + 1] === data[1] && data[i + 2] === data[2]) continue;
            assert(x > width * 0.04 && x < width * 0.96 && y > height * 0.04 && y < height * 0.96,
                'Fitted geometry must have space around it');
            count++;
        }
    }
    assert(count > 2000, 'Fit must leave visible geometry');
    return result;
}

async function capture() {
    fs.rmSync(path.join(output, 'browser.json'), {force: true});
    const args = process.env.VIEWER_CHROME_ARGS ? JSON.parse(process.env.VIEWER_CHROME_ARGS)
        : process.platform === 'linux' ? ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--disable-vulkan-surface', '--ozone-platform=x11'] : [];
    const headless = process.env.VIEWER_HEADLESS === '1';
    const browser = await chromium.launch({channel: 'chrome', headless, args});
    const records = {browser: browser.version(), headless, args, captured: new Date().toISOString(), steps: {}};
    try {
        const page = await browser.newPage({viewport: {width: 900, height: 760}, deviceScaleFactor: 1});
        const errors = [];
        page.on('pageerror', error => errors.push(error.message));
        page.on('console', message => {
            if (message.type() === 'error') errors.push(`${message.text()} ${message.location().url}`);
        });
        const base = process.env.JOURNEY_URL || 'http://127.0.0.1:8781/';
        const seen = new Set();
        for (const step of course.steps) {
            const id = step.id;
            errors.length = 0;
            await page.setViewportSize({width: 900, height: 760});
            await page.goto(new URL(`${id}/dist/`, base).href);
            await page.waitForFunction(() => {
                const status = document.querySelector('#status')?.textContent || '';
                return !status.startsWith('Waiting');
            }, {}, {timeout: 30000});
            const status = await page.locator('#status').textContent();
            assert.equal(status, step.browser_status);
            if (id === '04-input') {
                const button = page.getByRole('button', {name: 'Change background'});
                await button.waitFor();
                assert(await button.isEnabled());
                const dark = await drawing(page);
                await button.click();
                await page.waitForTimeout(200);
                const light = await drawing(page);
                assert.notEqual(dark, light, 'A click must change pixels');
                await button.focus();
                await page.keyboard.press('Enter');
                await page.waitForTimeout(200);
                assert.equal(await drawing(page), dark, 'Keyboard toggle must restore the original pixels');
                await button.click();
                await page.waitForTimeout(200);
            }
            for (const action of step.browser_actions || []) {
                if (action.button) {
                    const button = page.getByRole('button', {name: action.button, exact: true});
                    assert(await button.isEnabled());
                    await button.click();
                    if (action.button === 'Fit scene') {
                        const wide = await fitted(page);
                        await page.getByRole('button', {name: 'Undo', exact: true}).click();
                        await page.waitForTimeout(200);
                        assert.notEqual(await drawing(page), wide, 'Fit must leave import undoable');
                        await page.getByRole('button', {name: 'Redo', exact: true}).click();
                        await page.waitForTimeout(200);
                        assert.equal(await drawing(page), wide, 'Redo must restore the fitted import');
                        await page.setViewportSize({width: 480, height: 900});
                        await fitted(page);
                        await page.setViewportSize({width: 900, height: 760});
                        assert.equal(await fitted(page), wide, 'Fit after restoring the viewport must recover its picture');
                    }
                } else if (action.file) {
                    const canvas = page.locator('canvas');
                    const before = await drawing(page);
                    await page.getByLabel('Open mesh session').setInputFiles(`target/course-${id}/${action.file}`);
                    await page.waitForFunction(() => document.querySelector('#status')?.textContent === 'File imported. Undo removes the entire import.');
                    await page.waitForTimeout(200);
                    const imported = await drawing(page);
                    assert.notEqual(imported, before, 'Import must draw new objects');
                    await page.getByRole('button', {name: 'Undo', exact: true}).click();
                    await page.waitForTimeout(200);
                    assert.equal(await drawing(page), before, 'One undo must remove the whole import');
                    await page.getByRole('button', {name: 'Redo', exact: true}).click();
                    await page.waitForTimeout(200);
                    assert.equal(await drawing(page), imported, 'Redo must restore the import');
                } else if (action.viewport) {
                    await page.setViewportSize({width: action.viewport[0], height: action.viewport[1]});
                } else if (action.wheel !== undefined || action.key) {
                    const canvas = page.locator('canvas');
                    const before = await drawing(page);
                    if (action.wheel !== undefined) {
                        await canvas.hover();
                        const scroll = await page.evaluate(() => window.scrollY);
                        await page.mouse.wheel(0, action.wheel);
                        await page.waitForTimeout(200);
                        assert.equal(await page.evaluate(() => window.scrollY), scroll, 'Canvas wheel must not scroll the page');
                    } else {
                        await canvas.focus();
                        await page.keyboard.press(action.key);
                        await page.waitForTimeout(200);
                    }
                    const after = await drawing(page);
                    assert.notEqual(after, before, 'Navigation input must change the picture');
                    if (action.key) {
                        await page.getByRole('button', {name: 'Reset view', exact: true}).focus();
                        await page.keyboard.press(action.key);
                        await page.waitForTimeout(200);
                        assert.equal(await drawing(page), after, 'Keys outside the canvas must not navigate');
                    }
                } else if (action.drag) {
                    const box = await page.locator('canvas').boundingBox();
                    assert(box);
                    const [from, to] = action.drag;
                    const before = await drawing(page);
                    await page.mouse.move(box.x + from[0] * box.width, box.y + from[1] * box.height);
                    await page.mouse.down({button: 'right'});
                    await page.mouse.move(box.x + to[0] * box.width, box.y + to[1] * box.height, {steps: 5});
                    await page.mouse.up({button: 'right'});
                    await page.waitForTimeout(200);
                    const after = await drawing(page);
                    assert.notEqual(after, before, 'Right drag must orbit');
                    await page.mouse.move(box.x + from[0] * box.width, box.y + from[1] * box.height);
                    await page.waitForTimeout(200);
                    assert.equal(await drawing(page), after, 'Released drag must stop');
                } else {
                    const canvas = page.locator('canvas');
                    const box = await canvas.boundingBox();
                    assert(box);
                    await canvas.click({position: {x: action.canvas[0] * box.width, y: action.canvas[1] * box.height}});
                }
                await page.waitForTimeout(200);
            }
            const canvas = await drawing(page);
            assert(!seen.has(canvas), 'Each lesson must have a distinct visible result');
            seen.add(canvas);
            assert.deepEqual(errors, []);
            assert.equal(await page.locator('#status').textContent(), step.browser_result_status || step.browser_status);
            const file = `${id}-browser.png`;
            await page.screenshot({path: path.join(output, file), fullPage: true});
            records.steps[id] = {source: builds[id].source, status, canvas, file,
                bundle: hash(fs.readFileSync(`target/course-checks/${id}/dist/index.html`)),
                clickAndKeyboard: id === '04-input'};
            console.log(`${id}: browser checks and screenshot passed`);
        }
        fs.writeFileSync(path.join(output, 'browser.json'), JSON.stringify(records, null, 2) + '\n');
    } finally {
        await browser.close();
    }
}

capture().catch(error => { console.error(error); process.exit(1); });
