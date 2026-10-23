const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const assert = require('node:assert/strict');
const {chromium} = require('playwright');
const {PNG} = require('pngjs');
const fingerprint = require('./journey/checks/fingerprint.cjs');
const hash = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
async function capture() {
    const [id] = process.argv.slice(2);
    const course = JSON.parse(fs.readFileSync('docs/journey/course.json'));
    const step = course.steps.find(step => step.id === id);
    assert(step?.browser_check === 'journey/checks/browser/final.cjs', 'Choose the maintained-runtime endpoint');
    const builds = JSON.parse(fs.readFileSync('target/course-checks/results.json'));
    assert(builds[id]?.commands.every(command => command.returncode === 0), 'Verify the endpoint before capturing');
    const args = process.env.VIEWER_CHROME_ARGS ? JSON.parse(process.env.VIEWER_CHROME_ARGS)
        : process.platform === 'linux' ? ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--disable-vulkan-surface', '--ozone-platform=x11'] : [];
    const headless = process.env.VIEWER_HEADLESS === '1';
    const executablePath = process.env.VIEWER_BROWSER_EXECUTABLE;
    const browser = await chromium.launch({...(executablePath ? {executablePath} : {channel: 'chrome'}), headless, args});
    const output = 'docs/screenshots/journey';
    const saved = path.join(output, 'browser.json');
    const records = fs.existsSync(saved) ? JSON.parse(fs.readFileSync(saved)) : {steps: {}};
    const images = [];
    try {
        for (const density of [1, 2]) {
            const viewport = {width: 900, height: 760};
            const context = await browser.newContext({viewport, deviceScaleFactor: density});
            const page = await context.newPage();
            const errors = [];
            const messages = [];
            try {
                page.on('console', message => messages.push({type: message.type(), text: message.text()}));
                page.on('requestfailed', request => messages.push({type: 'requestfailed', url: request.url(), error: request.failure()}));
                page.on('pageerror', error => errors.push(error.message));
                page.on('console', message => {if (message.type() === 'error') errors.push(message.text());});
                const url = new URL(`${id}/dist/`, process.env.JOURNEY_URL || 'http://127.0.0.1:8781/');
                url.searchParams.set('inspect', '1');
                url.searchParams.set('nogrid', '1');
                await page.goto(url.href);
                await require(path.resolve('docs', step.browser_check))(page);
                url.searchParams.set('cmd', 'Layers Off;View Isometric;Opacity 1;Arctic On;View Outline On;Fit');
                // The maintained query reader decodes URI components, not form-style plus signs.
                url.search = url.search.replace(/\+/g, '%20');
                await page.goto(url.href);
                await page.waitForFunction(() => {
                    const text = document.querySelector('canvas').getAttribute('data-viewer-inspection');
                    if (!text) return false;
                    const state = JSON.parse(text);
                    const ui = JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui'));
                    return state.objects === 5 && state.text_labels.length === 1 && state.text_labels[0].text === 'original pedestal' && state.opacity === 1 && state.outlines && state.ssao && !ui.layers_open;
                });
                const canvas = page.locator('canvas');
                assert.deepEqual(await canvas.boundingBox(), {x: 0, y: 0, ...viewport});
                const physical = await canvas.evaluate(canvas => [canvas.width, canvas.height]);
                assert.deepEqual(physical, [viewport.width * density, viewport.height * density]);
                assert.equal(await page.locator('h1, h2, p:visible, button, fieldset').count(), 0);
                assert.equal(await page.locator('input:not(#command-agent)').count(), 0);
                assert.equal(await page.locator('#command-agent').getAttribute('tabindex'), '-1');
                const ui = JSON.parse(await canvas.getAttribute('data-viewer-ui'));
                assert.equal(ui.framework, 'egui 0.34.3');
                const file = density === 1 ? `${id}-browser.png` : `${id}-browser-dpr2.png`;
                const bytes = await page.screenshot({path: path.join(output, file), fullPage: true});
                const image = PNG.sync.read(bytes);
                const [left, top, right, bottom] = ui.scene_rect;
                let blue = 0, black = 0;
                for (let y = Math.ceil(top * density); y < Math.floor(bottom * density); y++) {
                    for (let x = Math.ceil(left * density); x < Math.floor(right * density); x++) {
                        const at = (y * image.width + x) * 4;
                        const [r, g, b] = image.data.subarray(at, at + 3);
                        if (b > r + 20 && b > g + 10 && r < 230) blue++;
                        if (r < 40 && g < 40 && b < 40) black++;
                    }
                }
                assert(blue > 5000 * density * density, 'Original blue geometry must actually render');
                assert(black > 200 * density * density, 'Black original boundaries must actually render');
                assert.deepEqual(errors, []);
                images.push({density, file, sha256: hash(bytes), viewport, physical, blue, black});
            } catch (error) {
                const status = await page.locator('body').innerText().catch(() => 'Page unavailable');
                const diagnostic = path.join('target/course-checks', `${id}-browser-dpr${density}-failure.json`);
                const canvas = await page.locator('canvas').evaluate(canvas => ({inspection: canvas.getAttribute('data-viewer-inspection'), ui: canvas.getAttribute('data-viewer-ui')})).catch(() => null);
                fs.writeFileSync(diagnostic, JSON.stringify({url: page.url(), browser: browser.version(), args, density, status, errors, messages, canvas}, null, 2) + '\n');
                console.error(`Browser diagnostics: ${diagnostic}`);
                throw new Error(`${error.message}\nViewer status: ${status}\nBrowser errors: ${errors.join('\n')}`, {cause: error});
            } finally {await context.close();}
        }
        records.steps[id] = {source: builds[id].source, checker: hash(fs.readFileSync(__filename)),
            extraChecker: fingerprint(step.browser_check), browser: browser.version(), headless, args,
            captured: new Date().toISOString(), file: images[0].file, images,
            bundle: hash(fs.readFileSync(`target/course-checks/${id}/dist/index.html`)),
            scope: 'Maintained runtime: original scene, Element/box deletion, history, command ownership, shortcuts, orbit/zoom, Close semantics and empty-manifest release, full canvas and actual original rendering at two pixel densities. Hardware performance and additional visual cases remain separate.'};
        fs.writeFileSync(saved, JSON.stringify(records, null, 2) + '\n');
        console.log(`${id}: maintained browser behavior and two-density screenshots passed`);
    } finally {await browser.close();}
}
capture().catch(error => {console.error(error); process.exitCode = 1;});
