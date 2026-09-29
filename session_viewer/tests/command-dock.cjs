const {chromium} = require('playwright');
const {PNG} = require('pngjs');
const fs = require('node:fs/promises');
const path = require('node:path');
const assert = require('node:assert/strict');

const root = path.resolve(__dirname, '..');
const run = process.argv[2] || 'current';
assert(['before', 'after', 'current'].includes(run));
const output = path.join(root, 'target/command-dock', run);
const states = ['idle', 'completion', 'options', 'history', 'collapsed'];

async function check() {
    await fs.mkdir(output, {recursive: true});
    const browser = await chromium.launch({
        executablePath: '/usr/bin/google-chrome', headless: false,
        args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE',
            '--disable-vulkan-surface', '--ozone-platform=x11'],
    });
    const page = await browser.newPage({viewport: {width: 1200, height: 800}});
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    const ui = async () => JSON.parse(await page.locator('canvas').getAttribute('data-viewer-ui'));
    const settle = () => page.waitForTimeout(300);

    async function control(key) {
        const item = (await ui()).controls.find(control => control.key === key);
        assert(item, key);
        const [left, top, right, bottom] = item.rect;
        await page.mouse.click((left + right) / 2, (top + bottom) / 2);
        await settle();
    }

    async function capture(name) {
        await page.mouse.move(600, 250);
        await settle();
        const state = await ui();
        await fs.writeFile(path.join(output, name + '.json'), JSON.stringify(state, null, 2));
        const top = Math.floor(state.scene_rect[3]);
        await page.screenshot({path: path.join(output, name + '.png'),
            clip: {x: 0, y: top, width: 1200, height: 800 - top}});
    }

    try {
        await page.route('**/view_local.yaml', route => route.fulfill({
            contentType: 'application/yaml',
            body: 'name: Command check\nitems:\n  - file: extension-nested.pb\n',
        }));
        await page.route('**/extension-nested.pb', route => route.fulfill({
            path: path.join(root, 'docs/extensions/nested.pb'),
        }));
        await page.goto(new URL('?data=off&inspect=1', process.env.VIEWER_URL || 'http://127.0.0.1:8770/').href);
        await page.waitForFunction(() => document.querySelector('canvas')?.getAttribute('data-viewer-ui'), null, {timeout: 90000});
        await capture('idle');
        await control('command/input');
        await page.keyboard.type('Lay');
        await settle();
        assert.equal((await ui()).command, 'Layers');
        await capture('completion');
        await page.keyboard.press('Tab');
        await settle();
        assert.equal((await ui()).command, 'Layers ');
        await capture('options');
        await page.keyboard.press('Escape');
        await settle();
        assert.equal((await ui()).command, '');
        await control('command/input');
        await page.keyboard.type('Outline On');
        await page.keyboard.press('Enter');
        await settle();
        assert((await ui()).history.at(-1).startsWith('> Outline On\n'));
        await control('command/collapse');
        await capture('history');
        await control('command/collapse');
        await capture('collapsed');
        assert.deepEqual(errors, []);
        if (run === 'after') {
            for (const name of states) {
                const before = PNG.sync.read(await fs.readFile(path.join(output, '../before', name + '.png')));
                const after = PNG.sync.read(await fs.readFile(path.join(output, name + '.png')));
                assert.equal(after.width, before.width, name);
                assert.equal(after.height, before.height, name);
                assert(after.data.equals(before.data), name + ': pixels changed');
            }
        }
        console.log(`PASS real command dock: ${run}; completion, Tab, Escape, execution and history`);
    } finally {
        await browser.close();
    }
}

check().catch(error => { console.error(error); process.exitCode = 1; });
