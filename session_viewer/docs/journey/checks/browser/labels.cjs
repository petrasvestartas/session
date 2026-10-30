const adaptive=require('./adaptive-gpu-reuse.cjs');
const assert=require('node:assert/strict');
const {PNG}=require('pngjs');
const focused=async(original,helpers)=>{
    const value=async(page,name)=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const density of [1,2]){
        const context=await original.context().browser().newContext({viewport:{width:1100,height:800},deviceScaleFactor:density});
        try{
            const page=await context.newPage(),errors=[];page.on('pageerror',e=>errors.push(e.message));await page.goto(original.url());
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
            for(const line of ['Close','Layers Off','View Grid Off','Example CAD','Example Scene Text','Fit','View Isometric'])await helpers.command(page,line);
            await page.waitForFunction(()=>{const s=JSON.parse(document.querySelector('canvas').getAttribute('data-scene-text-stats'));return s&&s.missing_glyphs===0&&s.world_plane_rasterizations>0;});
            const sources=await value(page,'data-scene-texts');assert.equal(sources.length,1);assert.equal(sources[0].source.label.text,'Original CAD · Ø25 · Ω');assert.equal(sources[0].source.label.object,null);const originalSource=sources[0].source;
            await helpers.command(page,'Select Next');await helpers.command(page,'Select Next');assert.deepEqual(await value(page,'data-selected-ids'),[sources[0].id]);
            const shot=PNG.sync.read(await page.locator('canvas').screenshot());let gold=0;for(let i=0;i<shot.data.length;i+=4)if(shot.data[i]>200&&shot.data[i+1]>200&&shot.data[i+2]<50)gold++;assert(gold>100,'Selected text has a real yellow plate');
            const gpuSnapshot=await adaptive.snapshot(page,{lane:'mesh',source:'cad',expectedRows:1,expectedQualities:1});
            const shape=(await value(page,'data-text-atlas'))[0],gpu=gpuSnapshot.stats.slice(0,2),raster=(await value(page,'data-scene-text-gpu'))[2];
            await helpers.command(page,'Move 0.25 0 0');const moved=await value(page,'data-scene-texts');assert.deepEqual(moved[0].source,originalSource);assert.notDeepEqual(moved[0].model,sources[0].model);assert.equal((await value(page,'data-text-atlas'))[0],shape);assert.equal((await value(page,'data-scene-text-gpu'))[2],raster);if(!(await adaptive.snapshot(page,{lane:'mesh',source:'cad',expectedRows:1,expectedQualities:1})).adaptive)assert.deepEqual((await value(page,'data-gpu-stats')).slice(0,2),gpu);
            await helpers.command(page,'Undo');assert.deepEqual((await value(page,'data-scene-texts'))[0].model,sources[0].model);await helpers.command(page,'Redo');assert.deepEqual((await value(page,'data-scene-texts'))[0].model,moved[0].model);
            const ui = JSON.parse(await page.locator('canvas').getAttribute('data-command-ui'));
            if (ui.focused) {
                const input = ui.controls.find(control => control.key === 'command/input'); assert(input);
                const box = await page.locator('canvas').boundingBox();
                const local = [input.rect[0]-20,(input.rect[1]+input.rect[3])/2];
                assert(local[0]>0 && local[0]<input.rect[0] && local[1]>ui.top && local[1]<box.height,'Command label is inside owned dock, outside input');
                const at = [box.x+local[0],box.y+local[1]];
                await page.mouse.move(...at);
                await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
                await page.mouse.down();
                await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
                await page.mouse.up();
            }
            await page.waitForFunction(()=>!JSON.parse(document.querySelector('canvas').getAttribute('data-command-ui')).focused);assert.deepEqual(await value(page,'data-selected-ids'),[sources[0].id]);await page.keyboard.press('Delete');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-scene-texts')).length===0);assert.deepEqual(await value(page,'data-scene-texts'),[]);await helpers.command(page,'Undo');assert.deepEqual((await value(page,'data-scene-texts'))[0].source,originalSource);assert.equal(await page.locator('canvas').getAttribute('data-text-error'),'');
            for(const name of ['Example Anchor','Example Nameplate','Example Billboard'])await helpers.command(page,name);assert.equal((await value(page,'data-scene-texts')).length,4);
            await helpers.command(page,'Close');assert.deepEqual(await value(page,'data-scene-texts'),[]);assert.deepEqual(await value(page,'data-scene-text-cpu'),[0,0,0,0]);assert.deepEqual(await value(page,'data-scene-text-gpu'),[0,0,0,0]);assert.deepEqual(errors,[]);
        }finally{await context.close();}
    }
    for(const line of ['Close','Layers Off','View Grid Off','Example CAD','Example Scene Text','Fit','View Isometric','Select Next','Select Next'])await helpers.command(original,line);
    assert.equal((await value(original,'data-command-ui')).status,'Command submitted.','The command dock reports the final selection command; the page status retains the last document event.');
};

module.exports=async(original,helpers)=>{
    await require('./glyphs.cjs')(original,helpers);
    await focused(original,helpers);
};
module.exports.focused=focused;
