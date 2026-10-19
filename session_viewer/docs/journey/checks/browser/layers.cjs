const assert=require('node:assert/strict');
module.exports=async(original,helpers)=>{
    await require('./tree.cjs')(original,helpers);
    const value=async(page,name)=>JSON.parse(await page.locator('canvas').getAttribute(name));
    const control=async(page,key,button='left')=>{
        await page.waitForFunction(key=>JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).controls.some(c=>c.key===key),key);
        const b=await page.locator('canvas').boundingBox(),ui=await value(page,'data-command-ui'),c=ui.controls.find(c=>c.key===key);assert(c,key);
        await page.mouse.click(b.x+(c.rect[0]+c.rect[2])/2,b.y+(c.rect[1]+c.rect[3])/2,{button});
    };
    const menu=async(page,key,name)=>{await control(page,`tree/select/${key}`,'right');await control(page,`layer/menu/${key}/${name}`);};
    const start=async page=>{for(const line of ['Close','View Grid Off','Select Object','Example Nested','Fit','View Isometric','Layers All'])await helpers.command(page,line);};
    for(const density of [1,2]){
        const context=await original.context().browser().newContext({viewport:{width:1100,height:800},deviceScaleFactor:density});
        try{
            const page=await context.newPage(),errors=[];page.on('pageerror',error=>errors.push(error.message));await page.goto(original.url());
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');await start(page);
            const initial=await value(page,'data-row-placements'),metadata=await value(page,'data-row-metadata'),rows=await value(page,'data-tree'),root=rows[0],posts=rows.find(r=>r.label==='Posts'),initialCamera=await value(page,'data-camera-matrix');
            assert((await value(page,'data-command-ui')).tree_rect[2]-(await value(page,'data-command-ui')).tree_rect[0]>=350,'Current wider panel formula');
            await menu(page,root.key,'New sublayer');await helpers.command(page,'Layers All');let layer=(await value(page,'data-tree')).find(r=>r.label==='Layer 01');assert(layer);assert.equal(layer.ids.length,0);
            await menu(page,layer.key,'Set current');assert.equal((await value(page,'data-view-flags')).current,layer.key);
            const count=(await value(page,'data-row-placements')).length;await helpers.command(page,'Example Box');await helpers.command(page,'Layers All');layer=(await value(page,'data-tree')).find(r=>r.key===layer.key);assert.equal(layer.ids.length,1);assert.equal((await value(page,'data-row-placements')).length,count+1);
            await helpers.command(page,'Undo');assert.equal((await value(page,'data-row-placements')).length,count);
            await menu(page,layer.key,'Rename');await control(page,'layer/name');await page.keyboard.press('ControlOrMeta+A');await page.keyboard.type('Posts');await page.keyboard.press('Enter');
            await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).controls.some(c=>c.key==='layer/name'));
            assert.equal((await value(page,'data-row-placements')).length,count,'Focused name submission does not delete or hide geometry');
            await page.keyboard.press('ControlOrMeta+A');await page.keyboard.type('Timber HS');await page.keyboard.press('Enter');
            await page.waitForFunction(key=>JSON.parse(document.querySelector('canvas').getAttribute('data-tree')).some(r=>r.key===key&&r.label==='Timber HS'),layer.key);
            await page.waitForFunction(()=>!JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).controls.some(c=>c.key==='layer/name'));
            assert.equal((await value(page,'data-view-flags')).current,layer.key);assert.deepEqual(await value(page,'data-row-metadata'),metadata);
            assert.deepEqual(await value(page,'data-camera-matrix'),initialCamera,'Layer creation and rename preserve the camera');
            await control(page,`tree/select/${posts.key}`);await page.waitForFunction(id=>{const selected=JSON.parse(document.querySelector('canvas').getAttribute('data-selected-ids'));return selected.length===1&&selected[0]===id;},posts.ids[0]);
            const camera=await value(page,'data-camera-matrix');
            await page.keyboard.press('Escape');await page.keyboard.press('h');let flags=await value(page,'data-view-flags');assert(flags.rows.filter(r=>posts.ids.includes(r.id)).every(r=>!r.visible));assert.deepEqual(await value(page,'data-row-metadata'),metadata);
            await page.keyboard.press('s');assert((await value(page,'data-view-flags')).rows.every(r=>r.visible));
            await control(page,`layer/colour/${posts.key}`);await control(page,`layer/colour/${posts.key}/false/Blue`);assert((await value(page,'data-view-flags')).rows.filter(r=>posts.ids.includes(r.id)).every(r=>JSON.stringify(r.faces)==='[65,130,225]'));
            await control(page,`layer/colour/${posts.key}`);await control(page,`layer/channel/${posts.key}/Edges`);await control(page,`layer/colour/${posts.key}/true/Green`);
            await control(page,`layer/colour/${posts.key}`);await control(page,`layer/channel/${posts.key}/Faces`);await control(page,`layer/original/${posts.key}/false`);flags=await value(page,'data-view-flags');assert(flags.rows.filter(r=>posts.ids.includes(r.id)).every(r=>r.faces===null&&JSON.stringify(r.edges)==='[60,170,100]'));
            assert.deepEqual(await value(page,'data-row-metadata'),metadata);assert.deepEqual(await value(page,'data-row-placements'),initial);assert.deepEqual(await value(page,'data-camera-matrix'),camera);
            await control(page,`layer/lock/${posts.key}`);await control(page,`tree/select/${posts.key}`);assert.equal((await value(page,'data-selected-ids')).length,0);await control(page,`layer/lock/${posts.key}`);
            await menu(page,posts.key,'Duplicate layer');await helpers.command(page,'Layers All');const copy=(await value(page,'data-tree')).find(r=>r.label==='Posts copy 01');assert(copy);assert.equal(copy.ids.length,2);
            await menu(page,copy.key,'Delete');assert.equal((await value(page,'data-row-placements')).length,count+2);await control(page,'layer/delete/Cancel');assert.equal((await value(page,'data-row-placements')).length,count+2);
            await menu(page,copy.key,'Delete');await control(page,'layer/delete/Delete');assert.equal((await value(page,'data-row-placements')).length,count);await helpers.command(page,'Undo');assert.equal((await value(page,'data-tree')).find(r=>r.label==='Posts copy 01').key,copy.key);
            await helpers.command(page,'Undo');await control(page,`tree/select/${posts.key}`);await page.keyboard.press('Escape');await page.keyboard.press('Delete');assert.equal((await value(page,'data-row-placements')).length,1);await helpers.command(page,'Undo');assert.deepEqual(await value(page,'data-row-placements'),initial);
            assert.deepEqual(errors,[]);
        }finally{await context.close();}
    }
    await start(original);const posts=(await value(original,'data-tree')).find(r=>r.label==='Posts');await control(original,`layer/colour/${posts.key}`);await control(original,`layer/colour/${posts.key}/false/Blue`);
};
