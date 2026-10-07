const assert = require('node:assert/strict');
const path = require('node:path');
const {chromium} = require('playwright');
const fixture = process.argv[2] || path.join(__dirname,'fixtures/timber-floor.pb');
const objects = Number(process.argv[3] || 529);
const state = page => page.locator('canvas').getAttribute('data-viewer-inspection').then(JSON.parse);
(async () => {
    const browser = await chromium.launch({channel:'chrome', headless:false,
        args:['--enable-unsafe-webgpu','--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE','--disable-vulkan-surface','--ozone-platform=x11']});
    try {
        const page = await browser.newPage({viewport:{width:1200,height:800}});
        const errors = [], preferences = [];
        page.on('pageerror', error => errors.push(error.message));
        await page.addInitScript(() => {
            const request = GPU.prototype.requestAdapter;
            window.adapterPreferences = [];
            GPU.prototype.requestAdapter = function(options) {
                adapterPreferences.push(options.powerPreference);
                return request.call(this, options);
            };
            const now = performance.now.bind(performance);
            let offset = 0;
            Object.defineProperty(performance, 'now', {value:() => now() + offset});
            window.advanceNavigationClock = () => { offset += 80; };
        });
        await page.route('**/view_local.yaml', route => route.fulfill({contentType:'application/yaml',
            body:'name: Floor navigation\nitems:\n  - file: floor.pb\n    name: floor\n'}));
        await page.route('**/floor.pb', route => route.fulfill({path:fixture}));
        await page.goto(new URL('?data=off&live=off&inspect=1&notify=off', process.env.VIEWER_URL || 'http://127.0.0.1:8770/').href);
        await page.waitForFunction(n => JSON.parse(document.querySelector('canvas')?.getAttribute('data-viewer-inspection') || '{}').objects === n, objects, {timeout:120000});
        await page.mouse.click(10,10);
        await page.waitForTimeout(1600);
        const before = await state(page);
        assert.equal(before.interacting, false);
        await page.mouse.move(600,350);
        let rough = false;
        for (let i=0;i<16;i++) {
            const previous = (await state(page)).frames;
            await page.evaluate(() => advanceNavigationClock());
            await page.mouse.wheel(0, i === 15 || i%4 < 2 ? -30 : 30);
            await page.waitForFunction(n => {
                const s=JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection'));
                return s.frames > n && s.interacting;
            }, previous);
            const during = await state(page);
            assert.equal(during.interacting, true, 'Wheel input enters navigation mode');
            rough ||= during.navigation_tier > 0;
        }
        assert(rough, 'Sustained slow wheel frames learn a temporary tier');
        await page.waitForTimeout(1600);
        const after = await state(page);
        assert.equal(after.interacting, false);
        assert.equal(after.navigation_tier, 0);
        assert.equal(after.rough, false, 'Wheel expiry redraws full quality');
        assert.equal(after.samples, before.samples);
        assert.deepEqual(after.canvas, before.canvas);
        assert.notDeepEqual(after.mvp, before.mvp);
        await page.waitForTimeout(700);
        assert.equal((await state(page)).frames, after.frames, 'Wheel expiry returns to idle');
        preferences.push(...await page.evaluate(() => adapterPreferences));
        assert.equal(preferences[0], 'high-performance');
        assert.deepEqual(errors, []);
        console.log(JSON.stringify({fixture:path.basename(fixture), result:'wheel navigation, temporary tiers, full-quality restoration and idle passed', objects:after.objects, preferences}));
    } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode=1; });
