const assert=require('node:assert/strict');
module.exports=async(original,helpers)=>{
    await require('./navigation.cjs')(original,helpers);
    const value=async(page,name)=>JSON.parse(await page.locator('canvas').getAttribute(name));
    const start=async page=>{for(const line of ['Close','Layers Off','View Reset','View Grid Off','Select Object','Example Surface','Fit','View Isometric'])await helpers.command(page,line);};
    for(const density of [1,2]){
        const context=await original.context().browser().newContext({viewport:{width:1100,height:800},deviceScaleFactor:density});
        try{
            const page=await context.newPage(),errors=[];page.on('pageerror',error=>errors.push(error.message));await page.goto(original.url());
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');await start(page);
            const sources=await value(page,'data-surfaces'),placements=await value(page,'data-row-placements'),gpu=(await value(page,'data-gpu-stats')).slice(0,2);
            assert.equal(sources.length,1);assert.deepEqual(sources[0].controls,[3,3]);assert.deepEqual(sources[0].steps,[16,16]);assert.equal(sources[0].samples,289);assert.deepEqual(sources[0].faces,[0]);assert.deepEqual(sources[0].edges,[0,1,2,3]);
            assert.deepEqual((await value(page,'data-surface-cpu')).slice(0,3),[1,1,1]);
            await helpers.command(page,'Select All');await helpers.command(page,'Move 0.123456789012345 0 0');
            assert.notDeepEqual(await value(page,'data-row-placements'),placements);assert.deepEqual(await value(page,'data-surfaces'),sources);
            assert.deepEqual((await value(page,'data-surface-cpu')).slice(1,3),[1,1]);assert.deepEqual((await value(page,'data-gpu-stats')).slice(0,2),gpu);
            await helpers.command(page,'Undo');assert.deepEqual(await value(page,'data-row-placements'),placements);assert.deepEqual(await value(page,'data-surfaces'),sources);
            await helpers.command(page,'Redo');assert.deepEqual(await value(page,'data-surfaces'),sources);await helpers.command(page,'Undo');
            await helpers.command(page,'Close');assert.deepEqual(await value(page,'data-surfaces'),[]);assert.deepEqual(await value(page,'data-surface-cpu'),[0,0,0,0,0]);assert.deepEqual(errors,[]);
        }finally{await context.close();}
    }
    await start(original);
};
