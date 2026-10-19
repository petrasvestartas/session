const assert=require('node:assert/strict');
const {PNG}=require('pngjs');
module.exports=async(original,helpers)=>{
    await require('./vertex-pulling.cjs')(original,helpers);
    for(const density of [1,2]){
        const context=await original.context().browser().newContext({viewport:{width:960,height:640},deviceScaleFactor:density});
        try{
            const page=await context.newPage(),errors=[];page.on('pageerror',error=>errors.push(error.message));
            await page.goto(original.url());await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
            const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
            const quiet=async()=>{await page.keyboard.press('Escape');await page.waitForTimeout(150);};
            const screen=async()=>{
                const matrix=await value('data-camera-matrix'),p=[-.35,0,0];
                const clip=[0,1,2,3].map(row=>p.reduce((sum,n,axis)=>sum+matrix[axis*4+row]*n,matrix[12+row]));
                const at=[(clip[0]/clip[3]+1)/2,(1-clip[1]/clip[3])/2];assert(at[0]>0&&at[0]<1&&at[1]>0&&at[1]<.85,'The actual source lies above the dock');return at;
            };
            const center=async at=>{const url=await require('./canvas-image.cjs')(page.locator('canvas'));const png=PNG.sync.read(Buffer.from(url.split(',')[1],'base64')),col=Math.floor(at[0]*png.width),row=Math.floor(at[1]*png.height);return [...png.data.subarray((row*png.width+col)*4,(row*png.width+col)*4+3)];};
            for(const ink of ['Point w-0.35,0,-0.2','Line w-0.45,0,-0.2 w-0.25,0,-0.2']){
                for(const line of ['Close','Layers Off','View Grid Off','Snap Off','Grid Snap Off','View Reset','View Orthographic','View Top','Example Face Colors','Fit','Opacity 1'])await helpers.command(page,line);
                const faces=(await value('data-view-flags')).rows.map(row=>row.id);assert.equal(faces.length,1);
                await helpers.command(page,'Opacity 0.1');await quiet();const clear=await center(await screen());
                await helpers.command(page,ink);await helpers.command(page,'Fit');await helpers.command(page,'Select None');await quiet();
                const sources=await value('data-drawing-originals'),at=await screen();

                await helpers.command(page,'Opacity 1');await quiet();const opaque=await center(at);assert(opaque[0]>180&&opaque[1]<30&&opaque[2]<30,`The opaque original face covers ${ink} at DPR${density}: ${opaque}`);
                await helpers.command(page,'Opacity 0.1');await quiet();const glass=await center(at);assert(glass[0]<clear[0]-10&&glass[1]<=clear[1]+2&&glass[2]<=clear[2]+2,`Hidden black ink contributes through glass at DPR${density}: ${glass}`);
                const box=await page.locator('canvas').boundingBox(),before=(await value('data-pick-gpu'))[3];assert(box);
                await page.mouse.click(box.x+at[0]*box.width,box.y+at[1]*box.height);
                await page.waitForFunction(before=>{const pick=JSON.parse(document.querySelector('canvas').getAttribute('data-pick-gpu'));return pick[3]>before&&pick[0]===0&&pick[1]===0;},before);
                const selected=await value('data-selected-ids');assert.deepEqual(selected,faces,'The physically nearer original mesh keeps ownership through glass');
                assert.deepEqual(await value('data-drawing-originals'),sources);assert.deepEqual(errors,[]);
            }
            await helpers.command(page,'Close');await quiet();assert.equal((await value('data-point-gpu'))[0],0);
        }finally{await context.close();await original.bringToFront();}
    }
    await helpers.command(original,'Close');
    console.log(`${helpers.step.id}: fitted point/stroke color and original face pick ownership through glass passed in Chrome at DPR1/2`);
};
