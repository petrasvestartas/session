const assert=require('node:assert/strict');
module.exports=async(original,helpers)=>{
 await require('./arena-reclamation.cjs')(original,helpers);
 for(const density of [1,2]){
  const context=await original.context().browser().newContext({viewport:{width:960,height:640},deviceScaleFactor:density});
  try{
   const page=await context.newPage(),errors=[];page.on('pageerror',error=>errors.push(error.message));
   const url=new URL(original.url());url.searchParams.set('nogrid','1');url.searchParams.set('opacity','1');await page.goto(url.href);
   await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
   const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
   for(const line of ['Layers Off','Select All','Delete','Example CAD','Clipping Plane XY 0,0,0','Clipping Plane Solid','View Top','Fit','Select None'])await helpers.command(page,line);
   const originals=await value('data-row-placements'),picture=await helpers.drawing(page);assert(originals.length>=2);
   for(let attempt=0;attempt<3;attempt++){
    await helpers.command(page,'Select All');await helpers.command(page,'Delete');assert.deepEqual(await value('data-gpu-usage'),[0,0,0,0]);
    await helpers.command(page,'Undo');await helpers.command(page,'Select None');assert.deepEqual(await value('data-row-placements'),originals);assert.equal(await helpers.drawing(page),picture);
   }
   assert.deepEqual(errors,[]);console.log(`Empty arena sources: actual CAD plus clipping plane, repeated Delete/Undo, exact originals/pixels and GPU release passed at DPR ${density}`);
  }finally{await context.close();await original.bringToFront();}
 }
};
