const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const {chromium} = require('playwright');
const out = 'docs/screenshots/practice';

(async () => {
    const browser = await chromium.launch({channel:'chrome', headless:true,
        args:['--enable-unsafe-webgpu', '--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE', '--disable-vulkan-surface', '--ozone-platform=x11']});
    try {
        const page = await browser.newPage({viewport:{width:1100,height:760}, deviceScaleFactor:1});
        const errors = [];
        page.on('pageerror', error => errors.push(error.message));
        page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
        await page.goto((process.env.VIEWER_URL || 'http://127.0.0.1:8788/')+'?data=off&inspect=1&dpr=1');
        await page.waitForFunction(() => {
            const value = document.querySelector('canvas')?.getAttribute('data-viewer-inspection');
            return value && JSON.parse(value).objects > 1;
        }, {}, {timeout:60000});

        const state = async () => JSON.parse(await page.locator('canvas').getAttribute('data-viewer-inspection'));
        const ui = async () => JSON.parse(await page.locator('canvas').getAttribute('data-viewer-ui'));
        async function settle() {
            await page.waitForTimeout(750);
            await page.waitForFunction(() => !JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).pick_busy);
        }
        async function control(key) {
            const item = (await ui()).controls.find(item => item.key === key);
            assert(item, key);
            const [x,y,right,bottom] = item.rect;
            await page.mouse.click((x+right)/2,(y+bottom)/2);
            await settle();
        }
        async function command(text) {
            await control('command/input');
            await page.keyboard.press('Control+a');
            await page.keyboard.type(text);
            await page.keyboard.press('Enter');
            await settle();
            assert((await ui()).history.at(-1)?.startsWith(`> ${text}\n`), text);
        }
        const records = {captured:new Date().toISOString(), browser:browser.version(),
            viewerIndex:crypto.createHash('sha256').update(fs.readFileSync(path.join(process.env.VIEWER_DIST || 'dist', 'index.html'))).digest('hex'),
            description:'Actual finished-viewer captures from a served handwritten-course build, with no scene or network request interception. Earlier lessons use these only as labelled visual references.', frames:{}};
        async function capture(name) {
            await settle();
            records.frames[name] = await state();
            await page.screenshot({path:path.join(out,`viewer-${name}.png`)});
        }
        await capture('loaded');
        const count = (await state()).objects;
        await command('Point 300,200,200');
        assert.equal((await state()).objects,count+1);
        await capture('point');
        await command('Undo');
        assert.equal((await state()).objects,count);
        await command('Layers On');
        assert((await ui()).layers_open);
        const group = (await ui()).rows.find(row => row.count > 1 && row.expanded === false);
        assert(group, 'The supplied box group');
        await control(group.key.replace('select/','open/'));
        await capture('layers');
        const row = (await ui()).rows.find(row => row.key.startsWith('select/') && row.count === 1 && row.has_faces);
        assert(row, 'A selectable leaf row');
        await control(row.key);
        assert((await state()).selected_rows.length > 0);
        await capture('selected');
        await command('Layers Off');
        const camera = (await state()).mvp;
        const selection = (await state()).selected_rows;
        await command('Arctic On');
        assert((await state()).ssao);
        await capture('arctic');
        await command('Opacity 0.4');
        assert.deepEqual((await state()).mvp,camera);
        assert.deepEqual((await state()).selected_rows,selection);
        assert.equal((await state()).objects,count);
        await capture('opacity');
        await command('Opacity 1');
        await command('Arctic Off');
        assert.deepEqual(errors, []);
        fs.writeFileSync(path.join(out,'viewer-captures.json'),JSON.stringify(records,null,2)+'\n');
    } finally {
        await browser.close();
    }
})().catch(error => { console.error(error); process.exit(1); });

// Serve the handwritten-course Trunk build and set VIEWER_URL and VIEWER_DIST, then: NODE_PATH=target/course-tools/node_modules node docs/capture_viewer.cjs.
