const assert = require('node:assert/strict');
module.exports = async (original, helpers) => {
    await require('./point-tools.cjs')(original, {...helpers, snappingDefined:true});
    const setup = async page => {
        await helpers.command(page,'Close'); await helpers.command(page,'View Reset'); await helpers.command(page,'View Grid Off');
        await helpers.command(page,'Snap On'); await helpers.command(page,'Grid Snap 1000');
        await helpers.command(page,'Line'); await helpers.command(page,'-0.65,-0.35,0'); await helpers.command(page,'0.65,0.35,0');
        await helpers.command(page,'Line'); await helpers.command(page,'-0.5,0.5,0');
    };
    const screen = async (page,p) => {
        const canvas=page.locator('canvas'), m=JSON.parse(await canvas.getAttribute('data-camera-matrix')), b=await canvas.boundingBox();
        const q=[0,1,2,3].map(row=>p.reduce((s,v,i)=>s+m[i*4+row]*v,m[12+row]));
        return [b.x+(q[0]/q[3]+1)/2*b.width,b.y+(1-q[1]/q[3])/2*b.height];
    };
    const hover = async (page,p,offset=0) => {
        const [x,y]=await screen(page,p);
        await page.evaluate(()=>{
            const canvas=document.querySelector('canvas');
            if(!canvas.hasAttribute('data-check-pointer')) {
                canvas.setAttribute('data-check-pointer','null');
                canvas.addEventListener('pointermove',e=>canvas.setAttribute('data-check-pointer',JSON.stringify([e.clientX,e.clientY])));
            }
        });
        await page.mouse.move(x+offset+1,y+1);
        await page.mouse.move(x+offset,y);
        await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-tool'))?.cursor);
    };
    for (const density of [1,2]) {
        const context=await original.context().browser().newContext({viewport:{width:960,height:640},deviceScaleFactor:density});
        try {
            const page=await context.newPage(), errors=[]; page.on('pageerror',e=>errors.push(e.message)); await page.goto(original.url());
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.() && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
            const canvas=page.locator('canvas'), value=async name=>JSON.parse(await canvas.getAttribute(name));
            assert.deepEqual(await value('data-snap-settings'),{objects:true,modes:15,grid:true,step:1000},'The latest startup default is enabled1000mm');
            for (const projection of ['View Perspective','View Orthographic']) {
                await setup(page); await helpers.command(page,projection);
                const gpu=await value('data-gpu-stats'); await hover(page,[0.65,0.35,0],5);
                await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-tool')).snap==='End');
                assert.deepEqual((await value('data-tool')).cursor,[0.65,0.35,0],'Original endpoint wins ahead of1000mm grid and nearer Near target');
                assert.deepEqual(await value('data-gpu-stats'),gpu,'Snap hover leaves primary geometry uploads unchanged');
                const builds=(await value('data-snap-usage'))[0]; await hover(page,[0.65,0.35,0],6);
                assert.equal((await value('data-snap-usage'))[0],builds,'The unchanged view reuses screen cells');
                await page.mouse.click(...await screen(page,[0.65,0.35,0]));
                assert.equal(await value('data-tool'),null); assert.equal((await value('data-stroke-usage'))[0],2);
                await helpers.command(page,'Undo'); assert.equal((await value('data-stroke-usage'))[0],1);
                await helpers.command(page,'Redo'); assert.equal((await value('data-stroke-usage'))[0],2);
                await helpers.command(page,'Line'); await helpers.command(page,'0.123456789012345,0.5,0');
                const points=(await value('data-tool')).points; assert.equal(points[0][0],0.123456789012345,'Typed world coordinates remain exact');
                await helpers.command(page,'Snap Off'); assert.equal((await value('data-snap-settings')).grid,true);
                await hover(page,[0.4,-0.4,0]); assert.deepEqual((await value('data-tool')).cursor,[0,0,0]); assert.equal((await value('data-tool')).snap,'Grid');
                await helpers.command(page,'Grid Snap 0.25'); assert.deepEqual((await value('data-tool')).points,points);
                await hover(page,[0.39,-0.39,0]); assert.deepEqual((await value('data-tool')).cursor,[0.5,-0.5,0]);
                const settings=await value('data-snap-settings'); await helpers.command(page,'Grid Snap 0');
                assert.deepEqual(await value('data-snap-settings'),settings); assert.deepEqual((await value('data-tool')).points,points);
                assert((await value('data-command-ui')).status.includes('above zero'));
                await helpers.command(page,'Grid Snap Off'); await hover(page,[0.39,-0.39,0]);
                const free=await value('data-tool'); assert.equal(free.snap,null); const pointer=await value('data-check-pointer'), projected=await screen(page,free.cursor), requested=await screen(page,[0.39,-0.39,0]);
                // This checkpoint reads client_x/client_y through web-sys's i32 mouse bindings.
                const received=pointer.map(Math.trunc);
                assert(Math.hypot(pointer[0]-requested[0],pointer[1]-requested[1])<=1,'Browser pointer coordinates stay within one CSS pixel of the requested position');
                assert(Math.hypot(projected[0]-received[0],projected[1]-received[1])<0.001 && Math.abs(free.cursor[2])<1e-8,
                    JSON.stringify({free,pointer,received,projected,density,projection}));
                await page.keyboard.press('Escape'); assert.equal(await value('data-tool'),null); assert.equal((await value('data-stroke-usage'))[0],2);
                await helpers.command(page,'Undo'); assert.equal((await value('data-stroke-usage'))[0],1,'Settings and cancellation add no source history');
            }
            assert.deepEqual(errors,[]);
        } finally { await context.close(); }
    }
    await setup(original); await hover(original,[0.65,0.35,0],5);
    await original.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-tool')).snap==='End');
};
