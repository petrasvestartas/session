const adaptive=require('./adaptive-gpu-reuse.cjs');
const assert=require('node:assert/strict');
const {PNG}=require('pngjs');
module.exports=async(original,helpers)=>{
    await require('./labels.cjs')(original,helpers);
    const value=async(page,name)=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const density of [1,2]){
        const context=await original.context().browser().newContext({viewport:{width:1100,height:800},deviceScaleFactor:density});
        try{
            const page=await context.newPage(),errors=[];page.on('pageerror',e=>errors.push(e.message));await page.goto(original.url());
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
            for(const line of ['Close','Layers Off','View Grid Off','Example CAD','Fit','View Isometric'])await helpers.command(page,line);
            assert.deepEqual(await value(page,'data-surface-outline'),[0,0,0,0]);
            const gpuSnapshot=await adaptive.snapshot(page,{lane:'mesh',source:'cad',expectedRows:1,expectedQualities:1}),gpu=gpuSnapshot.stats.slice(0,2);
            await helpers.command(page,'Select Next');
            await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-surface-outline'))[3]>0);
            const usage=await value(page,'data-surface-outline');assert(usage[0]>8208&&usage[1]>0);if(!(await adaptive.snapshot(page,{lane:'mesh',source:'cad',expectedRows:1,expectedQualities:1})).adaptive)assert.deepEqual((await value(page,'data-gpu-stats')).slice(0,2),gpu);
            const ids=await value(page,'data-selected-ids');assert.equal(ids.length,1);
            const shot=PNG.sync.read(await page.locator('canvas').screenshot());let gold=0,black=0;
            for(let i=0;i<shot.data.length;i+=4){if(shot.data[i]>110&&shot.data[i+1]>90&&shot.data[i+2]<100&&shot.data[i]>shot.data[i+1]&&shot.data[i]>2*shot.data[i+2]&&shot.data[i+1]>2*shot.data[i+2])gold++;if(shot.data[i]<50&&shot.data[i+1]<50&&shot.data[i+2]<50)black++;}
            assert(gold>1000,'The original selected CAD solid remains visible');assert(black>100,'Actual contour ink is black');
            await helpers.command(page,'Select None');assert.deepEqual(await value(page,'data-surface-outline'),[0,0,0,0]);
            await helpers.command(page,'Select Next');assert.deepEqual(await value(page,'data-selected-ids'),ids);
            await helpers.command(page,'Move 0.25 0 0');assert.deepEqual(await value(page,'data-selected-ids'),ids);if(!(await adaptive.snapshot(page,{lane:'mesh',source:'cad',expectedRows:1,expectedQualities:1})).adaptive)assert.deepEqual((await value(page,'data-gpu-stats')).slice(0,2),gpu);
            await helpers.command(page,'Undo');await helpers.command(page,'Close');assert.deepEqual(await value(page,'data-surface-outline'),[0,0,0,0]);assert.deepEqual(errors,[]);
        }finally{await context.close();}
    }
    for(const line of ['Close','Layers Off','View Grid Off','Example CAD','Fit','View Isometric','Select Next'])await helpers.command(original,line);
};
