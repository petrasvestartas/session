// Advance the timing clock during real drags to exercise the slow-navigation policy.
// This is a deterministic quality regression, not a phone speed benchmark.
const assert = require('node:assert/strict');
const {chromium} = require('playwright');
(async () => {
    const browser = await chromium.launch({channel:'chrome',headless:false,args:['--enable-unsafe-webgpu','--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE','--disable-vulkan-surface','--ozone-platform=x11']});
    try {
      for (const [name, settings, query, expected] of [
        ['phone', {viewport:{width:412,height:915},deviceScaleFactor:2.625,isMobile:true,hasTouch:true}, '', [824,1830,1]],
        ['phone-explicit', {viewport:{width:412,height:915},deviceScaleFactor:2.625,isMobile:true,hasTouch:true}, '&dpr=1.5&msaa=4', [618,1373,4]],
        ['desktop', {viewport:{width:1200,height:800}}, '', [1200,800,4]],
      ]) {
        const page = await browser.newPage(settings);
        const errors=[];
        page.on('pageerror',e=>errors.push(e.message));
        await page.addInitScript(() => {
            const realNow = performance.now.bind(performance);
            let slow=false,last=0;
            Object.defineProperty(performance,'now',{value:()=> {
                const now=realNow();
                last=slow ? Math.max(now,last+150) : Math.max(now,last);
                return last;
            }});
            window.slowNavigationClock = enabled => {slow=enabled;};
        });
        await page.goto((process.env.VIEWER_URL || 'http://127.0.0.1:8770/')+'?live=off&scene=view_live&notify=off&inspect=1'+query);
        await page.waitForFunction(()=>viewerDiagnostics.read().events.some(e=>e.kind==='milestone' && e.message==='geometry on screen'), null, {timeout:90000});
        await page.waitForTimeout(1000);
        const read=()=>page.locator('canvas').evaluate(c=>({width:c.width,height:c.height,inspection:JSON.parse(c.getAttribute('data-viewer-inspection'))}));
        const before=await read();
        assert.equal(before.width,expected[0]);assert.equal(before.height,expected[1]);
        assert.equal(before.inspection.samples,expected[2]);
        await page.mouse.click(10,10);
        await page.evaluate(()=>slowNavigationClock(true));
        await page.mouse.move(190,400);await page.mouse.down({button:'right'});
        for(let i=0;i<48;i++) {
            const frames=(await read()).inspection.frames;
            await page.mouse.move(190+(i%2 ? 2 : -2),400+i*.2);
            await page.waitForFunction(n=>JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).frames>n,frames);
        }
        await page.mouse.up({button:'right'});
        await page.evaluate(()=>slowNavigationClock(false));
        await page.waitForTimeout(1800);
        const after=await read();
        const report = await page.evaluate(()=>viewerDiagnostics.read());
        assert(report.events.some(e=>e.kind==='quality' && e.message==='Slow navigation: retained canvas resolution and antialiasing'), 'the slow-navigation detector must actually fire');
        const out = process.env.VIEWER_TEST_OUTPUT || '/home/petras/viewer_review_work';
        await page.screenshot({path:`${out}/navigation-quality-${name}.png`});
        console.log(JSON.stringify({name,before:{width:before.width,height:before.height},after:{width:after.width,height:after.height},errors}));
        assert.equal(after.width,before.width,'slow navigation must preserve crisp canvas resolution');
        assert.equal(after.height,before.height,'slow navigation must preserve crisp canvas resolution');
        assert.equal(after.inspection.samples,before.inspection.samples,'slow navigation must preserve antialiasing');
        assert.deepEqual(errors,[]);
        await page.close();
      }
    } finally {await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
