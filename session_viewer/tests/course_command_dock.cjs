const {chromium} = require('playwright');
const {PNG} = require('pngjs');
const fs = require('node:fs/promises');
const path = require('node:path');
const crypto = require('node:crypto');
const assert = require('node:assert/strict');

const root = path.resolve(__dirname, '..');
const output = path.join(root, 'docs/screenshots/journey');
const hash = data => crypto.createHash('sha256').update(data).digest('hex');

async function check() {
    const build = JSON.parse(await fs.readFile(path.join(root, 'target/course-checks/real-dock/build.json')));
    const browser = await chromium.launch({
        executablePath: '/usr/bin/google-chrome', headless: false,
        args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE',
            '--disable-vulkan-surface', '--ozone-platform=x11'],
    });
    const page = await browser.newPage({viewport: {width: 900, height: 760}});
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    const ui = async () => JSON.parse(await page.locator('canvas').getAttribute('data-command-ui'));
    const settle = () => page.waitForTimeout(150);

    async function control(key) {
        const item = (await ui()).controls.find(control => control.key === key);
        assert(item, key);
        const box = await page.locator('canvas').boundingBox();
        const [left, top, right, bottom] = item.rect;
        await page.mouse.click(box.x + (left + right) / 2, box.y + (top + bottom) / 2);
        await settle();
    }

    async function drawing() {
        const url = await page.locator('canvas').evaluate(canvas => canvas.toDataURL());
        const image = PNG.sync.read(Buffer.from(url.split(',')[1], 'base64'));
        // The folded dock occupies the bottom 30 rows; compare all scene rows above it.
        return Buffer.from(image.data.subarray(0, image.width * (image.height - 30) * 4));
    }

    try {
        await page.goto('http://127.0.0.1:8781/real-dock/dist/');
        await page.waitForFunction(() => document.querySelector('canvas')?.getAttribute('data-command-ui'), null, {timeout: 90000});
        await control('command/input');
        const before = await drawing();
        await page.keyboard.type('Bac');
        await settle();
        assert.equal((await ui()).command, 'Background');
        const completion = 'command-dock-preview-completion.png';
        await page.screenshot({path: path.join(output, completion)});
        await page.keyboard.press('Enter');
        await settle();
        assert(!(await drawing()).equals(before), 'Background must change the scene pixels');
        assert.equal((await ui()).command, '');
        await page.keyboard.type('Background');
        await page.keyboard.press('Enter');
        await settle();
        assert((await drawing()).equals(before), 'The command round-trip must restore every scene pixel');
        await page.keyboard.type('Bac');
        await page.keyboard.press('Escape');
        await settle();
        assert.equal((await ui()).command, '');
        assert((await drawing()).equals(before), 'Escape must leave the picture unchanged');
        const folded = (await ui()).controls.find(item => item.key === 'command/input').rect[1];
        await control('command/collapse');
        assert((await ui()).controls.find(item => item.key === 'command/input').rect[1] < folded);
        assert.equal((await ui()).history.length, 2);
        const history = 'command-dock-preview-history.png';
        await page.screenshot({path: path.join(output, history)});
        assert.deepEqual(errors, []);
        const pictures = {};
        for (const name of [completion, history]) pictures[name] = hash(await fs.readFile(path.join(output, name)));
        await fs.writeFile(path.join(output, 'command-dock-preview.json'), JSON.stringify({
            source: build.source, chrome: browser.version(), headed: true,
            checks: ['inline completion', 'Enter execution', 'exact scene-pixel round trip', 'Escape', 'history expansion'],
            pictures, status: 'integration preview; not a completed typing lesson',
        }, null, 2) + '\n');
        console.log('PASS early project with the real production command dock');
    } finally {
        await browser.close();
    }
}

check().catch(error => { console.error(error); process.exitCode = 1; });
