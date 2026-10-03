// Hold GPU completion deliberately: this checks frame coalescing, not phone speed.
const assert = require('node:assert/strict');
const {chromium} = require('playwright');
(async () => {
    const browser = await chromium.launch({channel:'chrome',headless:false,args:['--enable-unsafe-webgpu','--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE','--disable-vulkan-surface','--ozone-platform=x11']});
    try {
        for (const gesture of ['mouse-orbit', 'touch-orbit', 'touch-pan-pinch', 'touch-cancel']) {
            const matrices = [];
            for (const held of [false, true]) {
                const page = await browser.newPage({viewport:{width:412,height:915},deviceScaleFactor:2.625,isMobile:true,hasTouch:true});
                const errors=[];
                page.on('pageerror', e=>errors.push(e.message));
                await page.addInitScript(() => {
                    const done = GPUQueue.prototype.onSubmittedWorkDone, submit = GPUQueue.prototype.submit;
                    const gate = window.testGpuGate = {hold:false,waiters:[],submissions:0};
                    GPUQueue.prototype.onSubmittedWorkDone = function() {
                        const promise = done.call(this);
                        return gate.hold ? promise.then(()=>new Promise(resolve=>gate.waiters.push(resolve))) : promise;
                    };
                    GPUQueue.prototype.submit = function(...args) {gate.submissions++;return submit.apply(this,args);};
                    gate.release = () => {gate.hold=false;gate.waiters.splice(0).forEach(resolve=>resolve());};
                });
                await page.goto((process.env.VIEWER_URL||'http://127.0.0.1:8770/')+'?live=off&scene=view_live&notify=off&inspect=1');
                await page.waitForFunction(()=>window.viewerDiagnostics?.read().outcome==='ready',null,{timeout:90000});
                await page.waitForTimeout(1200);
                const before = await snapshot(page);
                const cdp = await page.context().newCDPSession(page);
                await page.evaluate(value=>{testGpuGate.hold=value;testGpuGate.submissions=0;},held);
                const two = gesture==='touch-pan-pinch';
                const touch = (type,i) => cdp.send('Input.dispatchTouchEvent',{type,touchPoints:type==='touchEnd'||type==='touchCancel'?[]:two?[{id:1,x:130-i*.5,y:400+i},{id:2,x:250+i*2,y:400+i}]:[{id:1,x:190+i*3,y:400+i}]});
                if (gesture==='mouse-orbit') {await page.mouse.move(190,400);await page.mouse.down({button:'right'});}
                else await touch('touchStart',0);
                for (let i=1;i<=24;i++) {
                    if (gesture==='mouse-orbit') await page.mouse.move(190+i*3,400+i);
                    else await touch('touchMove',i);
                    await page.waitForTimeout(25);
                    if (i===2 && held) await page.waitForTimeout(120);
                }
                if (gesture==='mouse-orbit') await page.mouse.up({button:'right'});
                else await touch(gesture==='touch-cancel'?'touchCancel':'touchEnd',24);
                await page.waitForTimeout(250);
                if (held) {
                    const blocked = await page.evaluate(()=>({submissions:testGpuGate.submissions,waiters:testGpuGate.waiters.length}));
                    assert(blocked.submissions<=3,`a busy GPU must stop old frames: ${JSON.stringify(blocked)}`);
                    await page.waitForFunction(()=>testGpuGate.waiters.length>0,null,{timeout:5000});
                    await page.evaluate(()=>testGpuGate.release());
                }
                await page.waitForTimeout(1200);
                const after = await snapshot(page);
                assert(after.frames>before.frames,'completion must wake the pending camera without another input');
                assert.notDeepEqual(after.mvp,before.mvp,'the gesture changed the camera');
                assert.deepEqual(after.canvas,[824,1830]);assert.equal(after.samples,1);
                matrices.push(after.mvp);
                const frames=after.frames;
                await page.waitForTimeout(700);
                assert.equal((await snapshot(page)).frames,frames,'completion must not start an idle redraw loop');
                assert.deepEqual(errors,[]);
                await page.close();
            }
            for(let i=0;i<16;i++) assert(Math.abs(matrices[0][i]-matrices[1][i])<1e-6,`${gesture}: latest pose must survive coalescing, matrix ${i}`);
            console.log(`PASS ${gesture}: bounded GPU queue, final camera preserved, release/cancel wakes, no idle loop`);
        }
    } finally {await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
async function snapshot(page) {return page.locator('canvas').getAttribute('data-viewer-inspection').then(JSON.parse);}
