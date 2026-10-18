const assert=require('node:assert/strict');
module.exports=async(original,helpers)=>{
    await require('./gizmo.cjs')(original,helpers);
    const value=async(page,name)=>JSON.parse(await page.locator('canvas').getAttribute(name));
    const control=async(page,key)=>{
        const b=await page.locator('canvas').boundingBox(),ui=await value(page,'data-command-ui');
        const c=ui.controls.find(c=>c.key===key);assert(c,`Actual tree control ${key}`);
        await page.mouse.click(b.x+(c.rect[0]+c.rect[2])/2,b.y+(c.rect[1]+c.rect[3])/2);
    };
    for(const density of [1,2]){
        const context=await original.context().browser().newContext({viewport:{width:960,height:640},deviceScaleFactor:density});
        try{
            const page=await context.newPage(),errors=[];page.on('pageerror',error=>errors.push(error.message));await page.goto(original.url());
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
            for(const line of ['Close','View Grid Off','Select Object','Example Nested','Fit','View Isometric','Layers All'])await helpers.command(page,line);
            const rows=await value(page,'data-tree'),posts=rows.find(r=>r.label==='Posts'),attribute=rows.find(r=>r.readonly&&r.ids.length===1);
            assert(posts&&attribute);assert.equal(posts.ids.length,2);assert(attribute.depth>posts.depth);
            const before=await value(page,'data-row-placements'),pick=await value(page,'data-pick-gpu');
            await control(page,`tree/select/${posts.key}`);
            const selected=await value(page,'data-selected-ids');assert.deepEqual(selected,[posts.ids[0]],'The group selects its eligible parent and leaves the source attribute read only');
            await control(page,`tree/select/${attribute.key}`);assert.deepEqual(await value(page,'data-selected-ids'),selected);
            assert.equal((await value(page,'data-pick-gpu'))[3],pick[3],'Tree controls own input before asynchronous canvas picking');
            const camera=await value(page,'data-camera-matrix'),ui=await value(page,'data-command-ui'),b=await page.locator('canvas').boundingBox();
            await page.mouse.move(b.x+ui.tree_rect[0]+40,b.y+ui.tree_rect[1]+70);await page.mouse.wheel(0,120);assert.deepEqual(await value(page,'data-camera-matrix'),camera,'Panel wheel input leaves the camera unchanged');
            await helpers.command(page,'Move 0.5,0,0');const moved=await value(page,'data-row-placements');
            for(let index=0;index<2;index++)assert.equal(moved[index][1][12],before[index][1][12]+0.5);assert.deepEqual(moved[2],before[2]);
            await helpers.command(page,'Undo');assert.deepEqual(await value(page,'data-row-placements'),before);await helpers.command(page,'Redo');assert.deepEqual(await value(page,'data-row-placements'),moved);
            const download=page.waitForEvent('download');await helpers.command(page,'Save');const path=await(await download).path();assert(path);
            const chooser=page.waitForEvent('filechooser');await helpers.command(page,'Open Replace');await(await chooser).setFiles(path);
            await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).status==='Document replaced. Undo restores the previous document.');
            const reopened=await value(page,'data-row-placements');assert.deepEqual(reopened.map(r=>r[1]),moved.map(r=>r[1]));
            await helpers.command(page,'Layers All');const saved=await value(page,'data-tree');assert(saved.some(r=>r.label==='Posts'));assert(saved.some(r=>r.readonly&&r.ids.length===1));
            const parent=saved.find(r=>r.label==='Posts');await control(page,`tree/select/${parent.key}`);await helpers.command(page,'Delete');assert.equal((await value(page,'data-row-placements')).length,1);
            await helpers.command(page,'Undo');assert.deepEqual(await value(page,'data-row-placements'),reopened);
            const restored=await value(page,'data-tree');assert.equal(restored.find(r=>r.label==='Posts').key,parent.key,'Undo restores its stable tree control key');
            assert.deepEqual(errors,[]);
        }finally{await context.close();}
    }
    for(const line of ['Close','View Grid Off','Select Object','Example Nested','Fit','View Isometric','Layers All'])await helpers.command(original,line);
};
