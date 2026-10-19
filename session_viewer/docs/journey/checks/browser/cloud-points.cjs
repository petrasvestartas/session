const assert=require('node:assert/strict');
const {PNG}=require('pngjs');
module.exports=async(original,helpers)=>{
    await require('./clouds.cjs')(original,helpers);
    const value=async(page,name)=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const density of [1,2]){
        const context=await original.context().browser().newContext({viewport:{width:1100,height:800},deviceScaleFactor:density});
        try{
            const page=await context.newPage(),errors=[];page.on('pageerror',e=>errors.push(e.message));await page.goto(original.url());
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
            for(const line of ['Close','Layers Off','View Grid Off','Example Cloud','Fit','View Isometric','Select Next','Controls'])await helpers.command(page,line);
            await page.waitForFunction(()=>document.querySelector('canvas').getAttribute('data-pick-mode')==='Control');
            const source=(await value(page,'data-cloud-sources'))[0],gpu=await value(page,'data-cloud-gpu');
            const bounds=await page.locator('canvas').boundingBox();assert(bounds);await page.mouse.click(bounds.x+bounds.width/2,bounds.y+bounds.height/2);
            await page.waitForFunction(()=>/original point \d+/.test(document.querySelector('canvas').getAttribute('data-selection-part')));
            const selected=await page.locator('canvas').getAttribute('data-selection-part');await helpers.command(page,'Controls');assert.equal(await page.locator('canvas').getAttribute('data-selection-part'),selected);assert(Number(await page.locator('canvas').getAttribute('data-highlight-bytes'))>0);assert.equal((await value(page,'data-cloud-sources'))[0].source_guid,source.source_guid);assert.deepEqual((await value(page,'data-cloud-sources'))[0].model,source.model);assert.equal((await value(page,'data-cloud-gpu'))[2],gpu[2]);
            const shot=PNG.sync.read(await page.locator('canvas').screenshot());let yellow=0;
            for(let i=0;i<shot.data.length;i+=4)if(shot.data[i]>180&&shot.data[i+1]>120&&shot.data[i+2]<80)yellow++;assert(yellow>10,'One local original point marker is actually drawn');
            await helpers.command(page,'Controls Off');assert.deepEqual(await value(page,'data-selected-ids'),[source.id]);
            await page.mouse.click(bounds.x+bounds.width/2,bounds.y+bounds.height/2);await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-selected-ids')).length===1);await page.keyboard.press('F10');
            await page.waitForFunction(()=>document.querySelector('canvas').getAttribute('data-pick-mode')==='Control');
            await helpers.command(page,'Controls Off');await helpers.command(page,'Close');assert.deepEqual(await value(page,'data-cloud-gpu'),[0,0,0,0,0]);assert.equal(await page.locator('canvas').getAttribute('data-highlight-bytes'),'0');
            for(const line of ['Example Cloud LOD','Fit'])await helpers.command(page,line);await require('./zoom-cloud-coarse.cjs')(page,helpers);
            await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-cloud-gpu'))[0]===64);const uploads=(await value(page,'data-cloud-gpu'))[2];
            for(const line of ['Select Next','Controls'])await helpers.command(page,line);
            await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-cloud-gpu'))[0]===2000064);assert.equal((await value(page,'data-cloud-gpu'))[2],uploads);
            await helpers.command(page,'Controls Off');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-cloud-gpu'))[0]===64);assert.equal((await value(page,'data-cloud-gpu'))[2],uploads);
            await helpers.command(page,'Close');assert.deepEqual(await value(page,'data-cloud-gpu'),[0,0,0,0,0]);assert.deepEqual(errors,[]);console.log(`${helpers.step.id}: visible original-point marker, controls, cached full/coarse LOD and release passed at DPR${density}`);
        }finally{await context.close();}
    }
    for(const line of ['Close','Layers Off','View Grid Off','Example Cloud','Fit','View Isometric','Select Next','Controls'])await helpers.command(original,line);
};
