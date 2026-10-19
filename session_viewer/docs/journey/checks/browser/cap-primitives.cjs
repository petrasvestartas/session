module.exports=async(original,helpers,emptyBytes=24)=>{
    await require('./finite-surfaces.cjs')(original,helpers);
    for(const density of [1,2]){
        const context=await original.context().browser().newContext({viewport:{width:960,height:640},deviceScaleFactor:density});
        try{
            const page=await context.newPage(),errors=[];
            page.on('pageerror',error=>errors.push(error.message));
            await page.goto(original.url());
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
            await require('./clipping.cjs').focused(page,helpers);
            await require('./caps.cjs').focused(page,helpers,emptyBytes);
            require('node:assert/strict').deepEqual(errors,[]);
        }finally{await context.close();await original.bringToFront();}
    }
    await helpers.command(original,'Close');
    for(const line of ['Example CAD','View Orthographic','View Top','Fit','Clipping Plane XY 0,0,0','Clipping Plane Solid','Select All'])await helpers.command(original,line);
    await original.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).caps[1]===1);
    const cads=JSON.parse(await original.locator('canvas').getAttribute('data-cads'));
    require('node:assert/strict').equal(cads.length,1);
    require('node:assert/strict').ok(JSON.parse(await original.locator('canvas').getAttribute('data-selected-ids')).includes(cads[0].id));
    await original.waitForTimeout(150);
    const image=await require('./canvas-image.cjs')(original.locator('canvas'));
    const png=require('pngjs').PNG.sync.read(Buffer.from(image.split(',')[1],'base64'));
    let yellow=0;for(let at=0;at<png.data.length;at+=4)if(png.data[at]>220&&png.data[at+1]>220&&png.data[at+2]<40)yellow++;
    require('node:assert/strict').ok(yellow>200,'The final lesson image contains actual selected section pixels');
    console.log(`${helpers.step.id}: focused section and clipping checks plus finite ink acceptance passed in Chrome at DPR1/2; integer cap readback is checked by the supplied GPU fixtures`);
};
