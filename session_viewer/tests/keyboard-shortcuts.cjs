const assert = require('node:assert/strict');
const {chromium} = require('playwright');
const state = page => page.locator('canvas').getAttribute('data-viewer-inspection').then(JSON.parse);
const ui = page => page.locator('canvas').getAttribute('data-viewer-ui').then(JSON.parse);
const projection = s => [0,1,3].flatMap(row => [s.mvp[row],s.mvp[row+4],s.mvp[row+8],
    s.mvp[row+12] - s.origin.reduce((value,axis,i) => value + axis*s.mvp[row+i*4],0)]);
async function command(page, text) {
    const {rect} = (await ui(page)).controls.find(row => row.key === 'command/input');
    await page.mouse.click((rect[0]+rect[2])/2, (rect[1]+rect[3])/2);
    await page.keyboard.press('Control+a'); await page.keyboard.type(text); await page.keyboard.press('Enter');
    await page.waitForFunction(text => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).history.at(-1)?.startsWith(`> ${text}\n`), text);
    await page.waitForTimeout(300);
}
async function key(page, text, document=true) {
    const history=(await ui(page)).history.length;
    await page.keyboard.press(text);
    if (document) await page.waitForFunction(n => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).history.length === n+1, history);
    await page.waitForTimeout(300);
}
(async () => {
    const browser = await chromium.launch({channel:'chrome', headless:false,
        args:['--enable-unsafe-webgpu','--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE','--disable-vulkan-surface','--ozone-platform=x11']});
    try {
        const page = await browser.newPage({viewport:{width:1200,height:800}}), errors=[];
        page.on('pageerror', error => errors.push(error.message));
        await page.route('**/view_local.yaml', route => route.fulfill({contentType:'application/yaml', body:'name: Shortcuts\nitems: []\n'}));
        await page.goto(new URL('?data=off&live=off&inspect=1&notify=off', process.env.VIEWER_URL || 'http://127.0.0.1:8770/').href);
        await page.waitForFunction(() => document.querySelector('canvas')?.hasAttribute('data-viewer-inspection'), null, {timeout:90000});
        await command(page, 'Box Brep 0,0,0 300 400 120');
        await command(page, 'Box Brep 1000,0,0 200 300 100');
        await command(page, 'Fit');
        const selected=(await state(page)).selected;
        await page.mouse.click(50,50,{button:'right'});
        await page.waitForTimeout(500);
        await page.waitForFunction(() => !JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).pick_busy);
        assert.equal((await state(page)).selected,selected);
        const fitted = projection(await state(page));
        await page.mouse.move(600,350); await page.mouse.wheel(0,-150);
        await page.waitForTimeout(300);
        assert.notDeepEqual(projection(await state(page)), fitted);
        const echoes=(await ui(page)).history.length;
        await key(page,'f');
        assert.equal((await ui(page)).history.length,echoes+1,'F must echo exactly one new Fit command');
        const restored=projection(await state(page));
        restored.forEach((value,i)=>assert(Math.abs(value-fitted[i])<1e-5,'F restores the fitted view across GPU rebasing'));
        assert((await ui(page)).history.at(-1).startsWith('> Fit\n'));
        const count=(await state(page)).objects;
        await key(page,'h'); assert.equal((await state(page)).hidden_count,1);
        await key(page,'s'); assert.equal((await state(page)).hidden_count,0);
        await key(page,'Control+z'); assert.equal((await state(page)).objects,count-1);
        await key(page,'Control+Shift+z'); assert.equal((await state(page)).objects,count);
        await key(page,'Control+z'); await key(page,'Control+y');
        assert.equal((await state(page)).objects,count);
        const history=(await ui(page)).history;
        const {rect}=(await ui(page)).controls.find(row=>row.key==='command/input');
        await page.mouse.click((rect[0]+rect[2])/2,(rect[1]+rect[3])/2);
        await page.keyboard.type('fhs'); await page.waitForTimeout(100);
        assert.equal((await ui(page)).command.toLowerCase(),'fhs');
        await key(page,'Delete',false);
        assert.equal((await state(page)).objects,count);
        await key(page,'Control+z',false);
        assert.deepEqual((await ui(page)).history,history);
        assert.equal((await state(page)).objects,count);
        assert.equal((await state(page)).hidden_count,0);
        await command(page,'Box Brep 2500,0,0 200 300 100');
        await command(page,'Length BoundingBox');
        assert((await ui(page)).history.at(-1).includes('X 200 · Y 100 · Z 300 mm'), (await ui(page)).history.at(-1));
        const bounds=(await state(page)).selected_bounds;
        await command(page,'BoundingBox');
        assert.deepEqual((await state(page)).selected_bounds,bounds);
        assert.equal((await state(page)).objects,count+2);
        await command(page,'Fit');
        await page.mouse.click(50,50,{button:'right'}); await page.waitForTimeout(500);
        await key(page,'Delete'); assert.equal((await state(page)).objects,count+1);
        await key(page,'Control+z'); assert.equal((await state(page)).objects,count+2);
        await key(page,'Control+y'); assert.equal((await state(page)).objects,count+1);
        assert.deepEqual(errors,[]);
        console.log('PASS F/H/S, Ctrl Undo/Redo and command text ownership');
    } finally { await browser.close(); }
})().catch(error=>{console.error(error);process.exitCode=1;});
