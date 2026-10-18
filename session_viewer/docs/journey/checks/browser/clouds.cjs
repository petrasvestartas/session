const assert=require('node:assert/strict');
const {PNG}=require('pngjs');
module.exports=async(original,helpers)=>{
    await require('./outline.cjs')(original,helpers);
    const value=async(page,name)=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const density of [1,2]){
        const context=await original.context().browser().newContext({viewport:{width:1100,height:800},deviceScaleFactor:density});
        try{
            const page=await context.newPage(),errors=[];page.on('pageerror',e=>errors.push(e.message));await page.goto(original.url());
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
            for(const line of ['Close','Layers Off','View Grid Off','Example Cloud','Fit','View Isometric'])await helpers.command(page,line);
            await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-cloud-gpu'))[0]===1681);
            const source=(await value(page,'data-cloud-sources'))[0],gpu=await value(page,'data-cloud-gpu');assert.equal(source.points,1681);assert(source.nodes>0);assert.equal(gpu[2],1);assert.equal(await page.locator('canvas').getAttribute('data-cloud-error'),'');
            const shot=PNG.sync.read(await page.locator('canvas').screenshot());let colored=0;
            for(let i=0;i<shot.data.length;i+=4)if(shot.data[i+2]>shot.data[i]+20&&shot.data[i+2]>shot.data[i+1]+10)colored++;assert(colored>1000,'Actual original coloured cloud pixels are visible');
            await helpers.command(page,'Select Next');assert.deepEqual(await value(page,'data-selected-ids'),[source.id]);await helpers.command(page,'Move 0.25 0 0');const moved=(await value(page,'data-cloud-sources'))[0];assert.equal(moved.source_guid,source.source_guid);assert.notDeepEqual(moved.model,source.model);assert.equal((await value(page,'data-cloud-gpu'))[2],gpu[2]);
            await helpers.command(page,'Undo');assert.deepEqual((await value(page,'data-cloud-sources'))[0].model,source.model);await require('./delete-selection.cjs')(page,[source.id]);await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-cloud-sources')).length===0);assert.deepEqual(await value(page,'data-cloud-sources'),[]);await helpers.command(page,'Undo');assert.equal((await value(page,'data-cloud-sources'))[0].source_guid,source.source_guid);
            await helpers.command(page,'Close');assert.deepEqual(await value(page,'data-cloud-cpu'),[0,0,0,0,0]);assert.deepEqual(await value(page,'data-cloud-gpu'),[0,0,0,0,0]);
            for(const line of ['Example Cloud LOD','Fit'])await helpers.command(page,line);
            await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-cloud-sources'))[0]?.points===2000064);
            await require('./zoom-cloud-coarse.cjs')(page,helpers);await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-cloud-gpu'))[0]===64);const uploads=(await value(page,'data-cloud-gpu'))[2];
            await helpers.command(page,'Fit');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-cloud-gpu'))[0]===2000064);assert.equal((await value(page,'data-cloud-gpu'))[2],uploads);
            await helpers.command(page,'Close');assert.deepEqual(await value(page,'data-cloud-gpu'),[0,0,0,0,0]);assert.deepEqual(errors,[]);console.log(`${helpers.step.id}: original cloud pixels, cached LOD, placement, keyboard deletion and Undo passed at DPR${density}`);
        }finally{await context.close();}
    }
    for(const line of ['Close','Layers Off','View Grid Off','Example Cloud','Fit','View Isometric'])await helpers.command(original,line);
};
