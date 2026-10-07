const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const assert = require('node:assert/strict');
const {chromium} = require('playwright');
const {PNG} = require('pngjs');
const browserFingerprint = require('./journey/checks/fingerprint.cjs');
const course = JSON.parse(fs.readFileSync('docs/journey/course.json', 'utf8'));
const builds = JSON.parse(fs.readFileSync('target/course-checks/results.json', 'utf8'));
const output = 'docs/screenshots/journey';
const selected = process.argv.slice(2);
assert(selected.every(id => course.steps.some(step => step.id === id)), 'Unknown checkpoint');
const hash = bytes => crypto.createHash('sha256').update(bytes).digest('hex');

// Drawing pixels exclude CSS borders and focus outlines from neighbouring controls.
async function drawing(page) {
    const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
    const pixels = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
    const ui = await page.locator('canvas').getAttribute('data-command-ui');
    // Compare the scene above the folded dock; text, caret and history are separate UI state.
    const rows = ui ? pixels.height - Math.ceil(34 * pixels.width / (await page.locator('canvas').boundingBox()).width) : pixels.height;
    return hash(pixels.data.subarray(0, rows * pixels.width * 4));
}


async function focusCommand(page) {
    await page.waitForFunction(() => {
        const canvas = document.querySelector('canvas');
        const ui = JSON.parse(canvas?.getAttribute('data-command-ui') || '{}');
        const field = ui.controls?.find(control => control.key === 'command/input');
        const height = canvas?.getBoundingClientRect().height;
        return field && field.rect[3] <= height && field.rect[3] > height - 12;
    });
    const ui = JSON.parse(await page.locator('canvas').getAttribute('data-command-ui'));
    const control = ui.controls.find(control => control.key === 'command/input');
    assert(control, 'The actual egui command field must be drawn');
    const box = await page.locator('canvas').boundingBox();
    const [left, top, right, bottom] = control.rect;
    await page.mouse.click(box.x + (left + right) / 2, box.y + (top + bottom) / 2);
    await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).focused);
}

async function command(page, line) {
    await focusCommand(page);
    const previous = JSON.parse(await page.locator('canvas').getAttribute('data-command-ui')).history.length;
    await page.keyboard.press('ControlOrMeta+A');
    await page.keyboard.type(line);
    await page.keyboard.press('Enter');
    await page.waitForTimeout(120);
    const ui = JSON.parse(await page.locator('canvas').getAttribute('data-command-ui'));
    assert.equal(ui.history.length, Math.min(200, previous + 1), 'Exactly one command must be submitted');
    assert.equal(ui.command, '', 'Enter must submit and clear the command field');
    assert(ui.history.at(-1)?.startsWith('> ' + line + '\n'), 'The dock must record the typed command: ' + line);
    assert(!ui.history.at(-1).includes('Unknown command'), 'The checkpoint must implement ' + line);
}

async function fitted(page) {
    await command(page, 'Fit');
    await page.waitForTimeout(200);
    const result = await drawing(page);
    await command(page, 'Fit');
    await page.waitForTimeout(200);
    assert.equal(await drawing(page), result, 'Repeated Fit must leave the view unchanged');
    await command(page, 'Pan Right');
    await page.waitForTimeout(200);
    assert.notEqual(await drawing(page), result, 'Pan must move the fitted picture');
    await command(page, 'Fit');
    await page.waitForTimeout(200);
    assert.equal(await drawing(page), result, 'Fit must recover the same view after panning');

    const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
    const {data, width, height} = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
    let count = 0;
    for (let y = 0; y < height - 30; y++) {
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

async function projection(page) {
    const ortho = await drawing(page);
    for (const mode of ['Perspective', 'Orthographic']) {
        await command(page, 'View ' + mode);
        assert.equal(await page.locator('canvas').getAttribute('data-projection'), mode);
        const before = await drawing(page);
        if (mode === 'Perspective') assert.notEqual(before, ortho, 'Projection must change depth-dependent scale');
        else assert.equal(before, ortho, 'Projection round-trip must restore all scene pixels');
        const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
        const image = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
        const orange = [];
        for (let y = 0; y < image.height - 34; y++) for (let x = 0; x < image.width; x++) {
            const i = (y * image.width + x) * 4;
            const [r, g, b] = image.data.subarray(i, i + 3);
            if (r > 100 && r > g * 1.25 && g > b * 1.25) orange.push([x, y]);
        }
        assert(orange.length > 100, 'An imported beam must be visible in ' + mode);
        const [x, y] = orange[Math.floor(orange.length / 2)];
        const box = await page.locator('canvas').boundingBox();
        await page.mouse.click(box.x + (x + .5) * box.width / image.width, box.y + (y + .5) * box.height / image.height);
        await page.waitForTimeout(120);
        assert.notEqual(await drawing(page), before, 'A visible beam must be selectable in ' + mode);
        const selected = PNG.sync.read(Buffer.from((await page.locator('canvas').evaluate(c => c.toDataURL())).split(',')[1], 'base64'));
        let yellow = 0;
        for (let i = 0; i < selected.width * (selected.height - 34) * 4; i += 4) {
            const [r, g, b] = selected.data.subarray(i, i + 3);
            if (r > 100 && g > r * .8 && g > b * 1.8) yellow++;
        }
        assert(yellow > 100, 'Picking must highlight the imported beam');
        await page.mouse.click(box.x + 4, box.y + 4);
        await page.waitForTimeout(120);
        assert.equal(await drawing(page), before, 'Clearing selection must restore the beam colour');
    }
}

async function capture() {

    const args = process.env.VIEWER_CHROME_ARGS ? JSON.parse(process.env.VIEWER_CHROME_ARGS)
        : process.platform === 'linux' ? ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--disable-vulkan-surface', '--ozone-platform=x11'] : [];
    const headless = process.env.VIEWER_HEADLESS === '1';
    const browser = await chromium.launch({channel: 'chrome', headless, args});
    const saved = path.join(output, 'browser.json');
    const records = selected.length && fs.existsSync(saved) ? JSON.parse(fs.readFileSync(saved))
        : {browser: browser.version(), headless, args, captured: new Date().toISOString(), steps: {}};
    try {
        const page = await browser.newPage({viewport: {width: 900, height: 760}, deviceScaleFactor: 1});
        const errors = [];
        page.on('pageerror', error => errors.push(error.message));
        page.on('console', message => {
            if (message.type() === 'error') errors.push(`${message.text()} ${message.location().url}`);
        });
        const base = process.env.JOURNEY_URL || 'http://127.0.0.1:8781/';
        for (const step of course.steps.filter(step => !selected.length || selected.includes(step.id))) {
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
            const viewport = page.viewportSize();
            const box = await page.locator('canvas').boundingBox();
            assert.deepEqual(box, {x: 0, y: 0, ...viewport}, id + ': canvas must fill the window from the first lesson');
            assert.equal(await page.locator('h1, h2, p:visible, button, fieldset').count(), 0,
                id + ': teaching text and feature controls belong in the documentation');
            if (id !== '01-canvas') {
                assert.deepEqual(await page.locator('canvas').evaluate(c => ({width: c.width, height: c.height})),
                    viewport, id + ': GPU image must match the initial window at display density 1');
                const url = await page.locator('canvas').evaluate(c => c.toDataURL());
                const pixels = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
                assert.deepEqual([...pixels.data.subarray(0, 4)], [255, 255, 255, 255],
                    id + ': the initial GPU background must be white');
            }
            if (id === '03a-panel') {
                const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
                const image = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
                const pixel = (x, y) => [...image.data.subarray((y * image.width + x) * 4, (y * image.width + x) * 4 + 3)];
                const pink = [0.9, 0.25, 0.45].map(value => Math.round(255 * (1.055 * value ** (1 / 2.4) - 0.055)));
                assert.deepEqual(pixel(Math.floor(image.width / 2), Math.floor(image.height / 2)), pink, 'The interface must preserve the triangle');
                assert.deepEqual(pixel(Math.floor(image.width / 2), image.height - 15), [255, 255, 255], 'The production command strip must be white');
                let ink = 0;
                for (let y = image.height - 27; y < image.height - 3; y++) for (let x = 6; x < 205; x++) {
                    if (pixel(x, y).every(channel => channel < 180)) ink++;
                }
                assert(ink > 100, 'Noto letters must actually render, not just a blank panel');
            }
            if (id === '03d-input') {
                await page.mouse.click(8, 8);
                await page.keyboard.type('He');
                await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).command === 'Help');
                await page.keyboard.press('Escape');
                await command(page, 'Help');
                await focusCommand(page);
                await page.keyboard.type('He');
                await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).command === 'Help');
                assert.equal(JSON.parse(await page.locator('canvas').getAttribute('data-command-ui')).command, 'Help');
                await page.keyboard.press('Escape');
            }
            if (id === '22-shortcuts') {
                const before = await drawing(page);
                await page.mouse.click(8, 8);
                await page.keyboard.type('g');
                await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).command.toLowerCase().startsWith('g'));
                await page.keyboard.press('Escape');
                await page.waitForTimeout(120);
                assert.equal(await drawing(page), before, 'Typing must not trigger a feature shortcut');
                await page.mouse.click(8, 8);
                await page.keyboard.press('ArrowRight');
                await page.waitForTimeout(120);
                assert.equal(await drawing(page), before, 'An arrow outside the dock must not navigate');
            }
            if (parseInt(id) >= 4) {
                assert.equal(await page.locator('button, input:not([type=file]), fieldset').count(), 0,
                    'Interactive checkpoints must use the canvas command dock only');
                await command(page, 'Help');
                const before = await drawing(page);
                await focusCommand(page);
                await page.keyboard.type('Bac');
                await page.waitForFunction(() => JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).command === 'Background');
                const ui = JSON.parse(await page.locator('canvas').getAttribute('data-command-ui'));
                assert.equal(ui.command, 'Background', 'Production inline completion must work');
                await page.keyboard.press('Escape');
                await page.waitForTimeout(100);
                assert.equal(await drawing(page), before, 'Cancelling a command must not affect geometry');
            }
            if (id === '04-input') {
                const dark = await drawing(page);
                await command(page, 'Background');
                assert.notEqual(await drawing(page), dark, 'Typed Background must change scene pixels');
                await command(page, 'Background');
                assert.equal(await drawing(page), dark, 'The keyboard round-trip must restore every scene pixel');
            }
            for (const action of step.browser_actions || []) {
                if (action.command) {
                    await command(page, action.command);
                    if (action.command === 'Fit') {
                        const wide = await fitted(page);
                        await command(page, 'Undo');
                        await page.waitForTimeout(200);
                        assert.notEqual(await drawing(page), wide, 'Fit must leave import undoable');
                        await command(page, 'Redo');
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
                    const choosing = page.waitForEvent('filechooser');
                    await command(page, 'Open');
                    await (await choosing).setFiles(`target/course-${id}/${action.file}`);
                    await page.waitForFunction(() => document.querySelector('#status')?.textContent === 'File imported. Undo removes the entire import.');
                    await page.waitForTimeout(200);
                    const imported = await drawing(page);
                    assert.notEqual(imported, before, 'Import must draw new objects');
                    await command(page, 'Undo');
                    await page.waitForTimeout(200);
                    assert.equal(await drawing(page), before, 'One undo must remove the whole import');
                    await command(page, 'Redo');
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
                        const box = await canvas.boundingBox();
                        await page.mouse.click(box.x + 8, box.y + 8);
                        await page.keyboard.press(action.key);
                        await page.waitForTimeout(200);
                    }
                    const after = await drawing(page);
                    assert.notEqual(after, before, 'Navigation input must change the picture');
                    if (action.key) {
                        await focusCommand(page);
                        await page.keyboard.press(action.key);
                        await page.waitForTimeout(200);
                        // Up/Down opens completion above the dock. Dismiss that overlay before comparing scene pixels.
                        await page.keyboard.press('Escape');
                        await page.waitForTimeout(120);
                        assert.equal(await drawing(page), after, 'A command-field ' + action.key + ' must not navigate');
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
            if (id === '25-projection') await projection(page);
            if (step.browser_check) {
                assert(/^journey\/checks\/browser\/[a-z0-9-]+\.cjs$/.test(step.browser_check), 'Invalid checkpoint browser check');
                await require(path.resolve('docs', step.browser_check))(page, {command, drawing, step});
            }
            const canvas = await drawing(page);
            assert.deepEqual(errors, []);
            assert.equal(await page.locator('#status').textContent(), step.browser_result_status || step.browser_status);
            const file = `${id}-browser.png`;
            await page.screenshot({path: path.join(output, file), fullPage: true});
            records.steps[id] = {source: builds[id].source, checker: hash(fs.readFileSync(__filename)),
                extraChecker: step.browser_check ? browserFingerprint(step.browser_check) : null,
                browser: browser.version(), headless, captured: new Date().toISOString(), status, canvas, file,
                initialViewport: viewport, initialCanvas: box, screenshotViewport: page.viewportSize(),
                bundle: hash(fs.readFileSync(`target/course-checks/${id}/dist/index.html`)),
                commands: (step.browser_actions || []).filter(action => action.command).map(action => action.command),
                keyboardRoundtrip: id === '04-input'};
            fs.writeFileSync(saved, JSON.stringify(records, null, 2) + '\n');
            console.log(`${id}: browser checks and screenshot passed`);
        }
        fs.writeFileSync(path.join(output, 'browser.json'), JSON.stringify(records, null, 2) + '\n');
    } finally {
        await browser.close();
    }
}

capture().catch(error => { console.error(error); process.exit(1); });
