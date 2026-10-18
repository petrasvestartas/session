const assert=require('node:assert/strict');
module.exports=async(original,helpers)=>{
    await require('./layers.cjs')(original,helpers);
    const value=async(page,name)=>JSON.parse(await page.locator('canvas').getAttribute(name));
    const start=async page=>{for(const line of ['Close','Layers Off','View Reset','View Grid Off','Snap Off','Grid Snap Off','Select Object','Example Box','Example Line','Example Point','Select None','Fit'])await helpers.command(page,line);};
    const drag=async(page,from,to,button='left')=>{await page.mouse.move(...from);await page.mouse.down({button});await page.mouse.move(...to,{steps:8});await page.mouse.up({button});};
    const idle=page=>page.waitForFunction(()=>{
        const c=document.querySelector('canvas'),drag=JSON.parse(c.getAttribute('data-canvas-drag')),pick=JSON.parse(c.getAttribute('data-pick-gpu'));return drag===null&&pick[0]===0&&pick[1]===0;
    });
    for(const density of [1,2]){
        const context=await original.context().browser().newContext({viewport:{width:1100,height:800},deviceScaleFactor:density,hasTouch:true});
        try{
            const page=await context.newPage(),errors=[];page.on('pageerror',error=>errors.push(error.message));await page.goto(original.url());
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');await start(page);
            const b=await page.locator('canvas').boundingBox(),ui=await value(page,'data-command-ui'),saved=await value(page,'data-row-placements'),metadata=await value(page,'data-row-metadata'),gpu=(await value(page,'data-gpu-stats')).slice(0,2);
            const command=ui.controls.find(c=>c.key==='command/input');assert(command);await page.mouse.click(b.x+(command.rect[0]+command.rect[2])/2,b.y+(command.rect[1]+command.rect[3])/2);
            await page.keyboard.press('ControlOrMeta+A');const all=await value(page,'data-selected-ids');assert.equal(all.length,3,'An empty command field forwards Select All');
            await page.keyboard.type('Move');await page.keyboard.press('ControlOrMeta+A');assert.deepEqual(await value(page,'data-selected-ids'),all,'Typed command text owns Select All');await page.keyboard.press('Escape');
            await helpers.command(page,'Select None');const from=[b.x+5,b.y+5],to=[b.x+b.width-5,b.y+ui.top-8];
            await drag(page,from,to);await idle(page);assert.deepEqual(await value(page,'data-selected-ids'),all,'Left-to-right window contains all precise sources');
            await page.keyboard.down('ControlOrMeta');await page.keyboard.down('Shift');await drag(page,from,to);await page.keyboard.up('Shift');await page.keyboard.up('ControlOrMeta');await idle(page);assert.deepEqual(await value(page,'data-selected-ids'),[],'Ctrl removes even with Shift held');
            await page.keyboard.down('Shift');await drag(page,[to[0],from[1]],[from[0],to[1]]);await page.keyboard.up('Shift');await idle(page);assert.deepEqual(await value(page,'data-selected-ids'),all,'Right-to-left crossing adds');
            assert.deepEqual(await value(page,'data-row-placements'),saved);assert.deepEqual(await value(page,'data-row-metadata'),metadata);
            await helpers.command(page,'Select None');const centre=[b.x+b.width*0.4,b.y+ui.top*0.45];
            for(const [button,ctrl] of [['right',false],['middle',false],['right',true]]){
                const before=await value(page,'data-camera-matrix');if(ctrl)await page.keyboard.down('ControlOrMeta');await drag(page,centre,[centre[0]+35,centre[1]+20],button);if(ctrl)await page.keyboard.up('ControlOrMeta');assert.notDeepEqual(await value(page,'data-camera-matrix'),before);
            }
            await page.mouse.move(...centre);const before=await value(page,'data-camera-matrix');await page.mouse.wheel(0,-100);assert.notDeepEqual(await value(page,'data-camera-matrix'),before);assert.deepEqual(await value(page,'data-row-placements'),saved);assert.deepEqual((await value(page,'data-gpu-stats')).slice(0,2),gpu);
            await helpers.command(page,'Layers All');await helpers.command(page,'Fit');const frame=await value(page,'data-camera-frame');assert(frame[2]<1&&frame[3]<1,'Fit uses the canvas left by panels');
            const owned=await value(page,'data-command-ui'),camera=await value(page,'data-camera-matrix');await page.mouse.move(b.x+owned.tree_rect[0]+20,b.y+owned.tree_rect[1]+30);await page.mouse.wheel(0,160);assert.deepEqual(await value(page,'data-camera-matrix'),camera,'Layer wheel owns input');
            await start(page);const touchSaved=await value(page,'data-row-placements');const client=await context.newCDPSession(page),touch=async(type,points)=>client.send('Input.dispatchTouchEvent',{type,touchPoints:points.map(([id,x,y])=>({id,x,y,radiusX:2,radiusY:2,force:1}))});
            const touchBefore=await value(page,'data-camera-matrix');await touch('touchStart',[[1,...centre]]);await touch('touchMove',[[1,centre[0]+30,centre[1]+20]]);assert.notDeepEqual(await value(page,'data-camera-matrix'),touchBefore);
            await touch('touchStart',[[1,centre[0]+30,centre[1]+20],[2,centre[0]+100,centre[1]]]);await touch('touchMove',[[1,centre[0]+20,centre[1]+30],[2,centre[0]+135,centre[1]+15]]);const pinched=await value(page,'data-camera-matrix');
            await touch('touchEnd',[[1,centre[0]+20,centre[1]+30]]);await touch('touchMove',[[1,centre[0]+50,centre[1]+40]]);assert.deepEqual(await value(page,'data-camera-matrix'),pinched,'Remaining finger waits for all lifts');await touch('touchEnd',[]);
            assert.deepEqual(await value(page,'data-row-placements'),touchSaved);assert.deepEqual(errors,[]);await client.detach();
        }finally{await context.close();}
    }
    for(const line of ['Close','View Grid Off','Example Nested','Fit','View Isometric','Layers All'])await helpers.command(original,line);
    const rows=await value(original,'data-tree'),posts=rows.find(r=>r.label==='Posts'),ui=await value(original,'data-command-ui'),b=await original.locator('canvas').boundingBox();
    const control=async key=>{const current=await value(original,'data-command-ui'),c=current.controls.find(c=>c.key===key);assert(c,key);await original.mouse.click(b.x+(c.rect[0]+c.rect[2])/2,b.y+(c.rect[1]+c.rect[3])/2);};
    await control(`layer/colour/${posts.key}`);await control(`layer/colour/${posts.key}/false/Blue`);
};
