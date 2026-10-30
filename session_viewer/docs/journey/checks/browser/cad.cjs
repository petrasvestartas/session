const adaptive=require('./adaptive-gpu-reuse.cjs');
const assert=require('node:assert/strict');
module.exports=async(original,helpers)=>{
    await require('./surface.cjs')(original,helpers);
    const value=async(page,name)=>JSON.parse(await page.locator('canvas').getAttribute(name));
    const start=async page=>{for(const line of ['Close','Layers Off','View Reset','View Grid Off','Select Object','Example CAD','Fit'])await helpers.command(page,line);};
    const click=async(page,point)=>{
        const m=await value(page,'data-camera-matrix'),b=await page.locator('canvas').boundingBox();
        const c=[0,1,2,3].map(i=>m[i]*point[0]+m[4+i]*point[1]+m[8+i]*point[2]+m[12+i]);
        await page.mouse.click(b.x+(c[0]/c[3]+1)*b.width/2,b.y+(1-c[1]/c[3])*b.height/2);
        await page.waitForFunction(()=>{const c=document.querySelector('canvas'),p=JSON.parse(c.getAttribute('data-pick-gpu'));return p[0]===0&&p[1]===0&&JSON.parse(c.getAttribute('data-canvas-drag'))===null;});
    };
    for(const density of [1,2]){
        const context=await original.context().browser().newContext({viewport:{width:1100,height:800},deviceScaleFactor:density});
        try{
            const page=await context.newPage(),errors=[];page.on('pageerror',error=>errors.push(error.message));await page.goto(original.url());
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');await start(page);
            const gpuSnapshot=await adaptive.snapshot(page,{lane:'mesh',source:'cad',expectedRows:1,expectedQualities:1});
            const cads=await value(page,'data-cads'),placements=await value(page,'data-row-placements'),gpu=gpuSnapshot.stats.slice(0,2);
            assert.equal(cads.length,1);assert.deepEqual(cads[0].faces,[0,1,2,3,4,5,6]);assert.deepEqual(cads[0].edges,Array.from({length:15},(_,i)=>i));assert(cads[0].vertices>0);assert.deepEqual((await value(page,'data-cad-cache')).slice(1,3),[1,1]);
            await helpers.command(page,'Select Face');await click(page,[0,0,0.3]);assert.equal(await page.locator('canvas').getAttribute('data-selection-part'),'','The hole has no face pick target');
            await click(page,[1.4,0,0.3]);assert((await page.locator('canvas').getAttribute('data-selection-part')).endsWith('source face 6'));
            await helpers.command(page,'Select Edge');await click(page,[0.65*Math.SQRT1_2,0.65*Math.SQRT1_2,0.3]);assert((await page.locator('canvas').getAttribute('data-selection-part')).endsWith('original BRep edge 13'));
            await helpers.command(page,'Select Object');await helpers.command(page,'Select All');await helpers.command(page,'Move 0.123456789012345 0 0');
            assert.notDeepEqual(await value(page,'data-row-placements'),placements);assert.deepEqual(await value(page,'data-cads'),cads);assert.deepEqual((await value(page,'data-cad-cache')).slice(1,3),[1,1]);if(!(await adaptive.snapshot(page,{lane:'mesh',source:'cad',expectedRows:1,expectedQualities:1})).adaptive)assert.deepEqual((await value(page,'data-gpu-stats')).slice(0,2),gpu);
            await helpers.command(page,'Undo');assert.deepEqual(await value(page,'data-row-placements'),placements);await helpers.command(page,'Redo');assert.deepEqual(await value(page,'data-cads'),cads);await helpers.command(page,'Undo');
            await helpers.command(page,'Close');assert.deepEqual(await value(page,'data-cads'),[]);assert.deepEqual(await value(page,'data-cad-cache'),[0,0,0,0,0,0]);assert.deepEqual(errors,[]);
        }finally{await context.close();}
    }
    await start(original);await helpers.command(original,'View Isometric');
};
