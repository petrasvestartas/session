const adaptive=require('./adaptive-gpu-reuse.cjs');
const assert=require('node:assert/strict');
const {PNG}=require('pngjs');
module.exports=async(original,helpers)=>{
    await require('./cad.cjs')(original,helpers);
    const value=async(page,name)=>JSON.parse(await page.locator('canvas').getAttribute(name));
    const ink=async page=>{
        const image=PNG.sync.read(await page.locator('canvas').screenshot());let count=0;
        const density=image.width/(await page.locator('canvas').boundingBox()).width;
        for(let y=0;y<50*density;y++)for(let x=0;x<430*density;x++){const i=(y*image.width+x)*4;if(image.data[i]<200&&image.data[i+1]<200&&image.data[i+2]<200)count++;}
        return count;
    };
    for(const density of [1,2]){
        const context=await original.context().browser().newContext({viewport:{width:1100,height:800},deviceScaleFactor:density});
        try{
            const page=await context.newPage(),errors=[];page.on('pageerror',error=>errors.push(error.message));await page.goto(original.url());
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
            for(const line of ['Close','Layers Off','View Grid Off','Example CAD','Fit','View Isometric'])await helpers.command(page,line);
            const gpuSnapshot=await adaptive.snapshot(page,{lane:'mesh',source:'cad',expectedRows:1,expectedQualities:1});
            const placements=await value(page,'data-row-placements'),gpu=gpuSnapshot.stats.slice(0,2);const before=await ink(page);
            await helpers.command(page,'Example Text');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-text-atlas'))[1]>0);
            const drawn=await ink(page);assert(drawn>before+200,'The actual glyph specimen has ink above the scene');const stats=await value(page,'data-text-atlas');assert.equal(stats[0],1);assert.equal(stats[3],0,'Glyphon 0.11 keeps temporary raster images out of the CPU image cache');assert.equal(stats[4],0);
            const canvas=await page.locator('canvas').boundingBox();await page.mouse.move(canvas.x+canvas.width*0.5,canvas.y+canvas.height*0.35);await page.mouse.down({button:'right'});await page.mouse.move(canvas.x+canvas.width*0.5+35,canvas.y+canvas.height*0.35+15);await page.mouse.up({button:'right'});await page.mouse.wheel(0,-100);
            assert.equal((await value(page,'data-text-atlas'))[0],1,'Camera changes keep glyph shaping');if(!(await adaptive.snapshot(page,{lane:'mesh',source:'cad',expectedRows:1,expectedQualities:1})).adaptive)assert.deepEqual((await value(page,'data-gpu-stats')).slice(0,2),gpu);assert.deepEqual(await value(page,'data-row-placements'),placements);assert.equal(await page.locator('canvas').getAttribute('data-text-error'),'');
            await helpers.command(page,'Close');assert.deepEqual(await value(page,'data-text-atlas'),[0,0,0,0,0,0]);assert.deepEqual(errors,[]);
        }finally{await context.close();}
    }
    for(const line of ['Close','Layers Off','View Grid Off','Example CAD','Fit','View Isometric','Example Text'])await helpers.command(original,line);
};
