const assert=require('node:assert/strict');
module.exports=async(original,helpers)=>{
    await require('./snapping.cjs')(original,helpers);
    const start=async(page,mixed=false)=>{
        for(const line of ['Close','View Reset','View Grid Off','Select Object','Snap Off','Grid Snap Off','Example Box'])await helpers.command(page,line);
        if(mixed)for(const line of ['Example Line','Example Polyline','Example Point'])await helpers.command(page,line);
        await helpers.command(page,'Select All');await helpers.command(page,'Fit');
    };
    const hover=async(page,p)=>{
        const canvas=page.locator('canvas'),m=JSON.parse(await canvas.getAttribute('data-camera-matrix')),b=await canvas.boundingBox();
        const q=[0,1,2,3].map(row=>p.reduce((sum,v,i)=>sum+m[i*4+row]*v,m[12+row]));
        await page.evaluate(()=>{
            const canvas=document.querySelector('canvas');
            if(!canvas.hasAttribute('data-check-direct-pointer')) {
                canvas.setAttribute('data-check-direct-pointer','null');
                canvas.addEventListener('pointermove',e=>canvas.setAttribute('data-check-direct-pointer',JSON.stringify([e.clientX,e.clientY])));
            }
        });
        const x=b.x+(q[0]/q[3]+1)/2*b.width,y=b.y+(1-q[1]/q[3])/2*b.height;
        await page.mouse.move(x+1,y+1);await page.mouse.move(x,y);
        await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-tool'))?.cursor);
    };
    for(const density of [1,2]){
        const context=await original.context().browser().newContext({viewport:{width:960,height:640},deviceScaleFactor:density});
        try{
            const page=await context.newPage(),errors=[];page.on('pageerror',e=>errors.push(e.message));await page.goto(original.url());
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
            const canvas=page.locator('canvas'),value=async name=>JSON.parse(await canvas.getAttribute(name));
            for(const projection of ['View Perspective','View Orthographic']){
                await start(page);await helpers.command(page,projection);await page.mouse.move(10,10);
                const before=await value('data-row-placements'),geometry=(await value('data-gpu-stats')).slice(0,2),image=await helpers.drawing(page);
                await helpers.command(page,'Move');await helpers.command(page,'0,0,0');await hover(page,[0.5,0.25,0]);
                assert.deepEqual(await value('data-row-placements'),before,'Mouse preview preserves saved source placements');
                const preview=await value('data-edit-preview');assert(preview);
                const cursor=(await value('data-tool')).cursor,received=(await value('data-check-direct-pointer')).map(Math.trunc);
                const m=await value('data-camera-matrix'),box=await canvas.boundingBox();
                const clip=[0,1,2,3].map(row=>cursor.reduce((sum,v,i)=>sum+m[i*4+row]*v,m[12+row]));
                const projected=[box.x+(clip[0]/clip[3]+1)/2*box.width,box.y+(1-clip[1]/clip[3])/2*box.height];
                assert(Math.hypot(projected[0]-received[0],projected[1]-received[1])<0.001 && Math.abs(cursor[2])<1e-8,JSON.stringify({cursor,received,projected}));
                assert.deepEqual(preview.matrix.slice(12,15),cursor,'Move preview follows the interpreted world cursor from the typed zero origin');
                assert.deepEqual((await value('data-gpu-stats')).slice(0,2),geometry,'Placement preview reuses existing primary geometry and settings allocations');
                assert.notDeepEqual(await helpers.drawing(page),image,'Actual displayed geometry follows the temporary placement');
                await helpers.command(page,'NaN,0,0');assert.deepEqual(await value('data-edit-preview'),preview);assert.deepEqual((await value('data-tool')).points,[[0,0,0]]);
                const downloaded=page.waitForEvent('download');await helpers.command(page,'Save');const file=await downloaded;const path=await file.path();assert(path);
                assert.deepEqual(await value('data-row-placements'),before);assert.deepEqual(await value('data-edit-preview'),preview,'Save keeps the running preview and exports original placements');
                await page.keyboard.press('Escape');assert.equal(await value('data-edit-preview'),null);assert.equal(await value('data-tool'),null);
                await page.mouse.move(10,10);
                const restored=await helpers.drawing(page);
                assert.deepEqual(restored,image,'Escape restores the original display with the same idle hover state');
                const chooser=page.waitForEvent('filechooser');await helpers.command(page,'Open Replace');await(await chooser).setFiles(path);
                await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).status==='Document replaced. Undo restores the previous document.');
                assert.deepEqual((await value('data-row-placements')).map(r=>r[1]),before.map(r=>r[1]),'Actual downloaded Save reopens at the original placements');
                await start(page,true);await helpers.command(page,projection);
                const mixed=await value('data-row-placements');assert.equal((await value('data-selected-ids')).length,4);
                await helpers.command(page,'Move');await helpers.command(page,'0,0,0');await helpers.command(page,'0.123456789012345,0.25,-0.5');
                const moved=await value('data-row-placements');assert.equal(moved[0][1][12],mixed[0][1][12]+0.123456789012345);assert.equal(await value('data-edit-preview'),null);
                await helpers.command(page,'Undo');assert.deepEqual(await value('data-row-placements'),mixed);
                await helpers.command(page,'Move');await helpers.command(page,'0,0,0');await hover(page,[0.3,0.2,0]);
                await page.keyboard.press('Escape');await helpers.command(page,'Redo');assert.deepEqual(await value('data-row-placements'),moved,'Cancelling a preview preserves the earlier redo branch');
                await helpers.command(page,'Undo');await helpers.command(page,'Move');await helpers.command(page,'0,0,0');await helpers.command(page,'0,0,0');
                await helpers.command(page,'Redo');assert.deepEqual(await value('data-row-placements'),moved,'A zero-distance Move preserves redo');
                await helpers.command(page,'Undo');await helpers.command(page,'Undo');assert.equal((await value('data-point-gpu'))[0],0,'One Undo undoes the entire mixed Move, so the next removes the earlier point creation');
                await helpers.command(page,'Close');assert.equal(await value('data-edit-preview'),null);
            }
            assert.deepEqual(errors,[]);
        }finally{await context.close();}
    }
    await start(original);for(const line of ['Move -1.8,0,0','Example Box','Select All','Fit','Move','0,0,0'])await helpers.command(original,line);await hover(original,[0.4,0.2,0]);
};
