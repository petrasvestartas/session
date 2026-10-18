const assert=require('node:assert/strict');
module.exports=async(original,helpers)=>{
    await require('./normal-view.cjs')(original,helpers);
    for(const density of [1,2]){
        const context=await original.context().browser().newContext({viewport:{width:960,height:640},deviceScaleFactor:density});
        try{
            const page=await context.newPage(),errors=[];page.on('pageerror',error=>errors.push(error.message));
            await page.goto(original.url());await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
            const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
            const quiet=async()=>{await page.keyboard.press('Escape');await page.waitForTimeout(150);};
            for(const line of ['Close','Layers Off','View Grid Off','View Normals Off','Example Instances','Fit','View Isometric'])await helpers.command(page,line);
            const source=await value('data-instances');assert.equal(source.length,2);assert.equal(source[0].definition_guid,source[1].definition_guid);assert.notEqual(source[0].id,source[1].id);
            assert.deepEqual((await value('data-instance-gpu')).slice(0,3),[1,2,helpers.instanceAllocationBytes??2*(helpers.objectRecordBytes??160)]);await quiet();const baseline=await helpers.drawing(page),uploads=(await value('data-gpu-stats')).slice(0,2);
            await helpers.command(page,'View Normals On');await quiet();assert.notEqual(await helpers.drawing(page),baseline);assert.equal(await value('data-normal-view'),true);
            await helpers.command(page,'View Normals Off');await quiet();assert.equal(await helpers.drawing(page),baseline);assert.deepEqual((await value('data-gpu-stats')).slice(0,2),uploads);
            await helpers.command(page,'Select Next');await quiet();assert.notEqual(await helpers.drawing(page),baseline);assert.deepEqual((await value('data-gpu-stats')).slice(0,2),uploads);
            await helpers.command(page,'Select None');await quiet();assert.equal(await helpers.drawing(page),baseline);assert.deepEqual(await value('data-instances'),source);
            await helpers.command(page,'Close');await quiet();assert.deepEqual((await value('data-instance-gpu')).slice(0,3),[0,0,0]);assert.deepEqual(await value('data-gpu-usage'),[0,0,0,0]);assert.deepEqual(errors,[]);
        }finally{await context.close();await original.bringToFront();}
    }
    await helpers.command(original,'Close');
    console.log(`${helpers.step.id}: shared color vertex reads, source/normal pixels, placed batches, selection, exact restoration and release passed in Chrome at DPR1/2`);
};
