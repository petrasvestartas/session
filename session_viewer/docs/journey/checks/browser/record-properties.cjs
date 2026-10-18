const assert=require('node:assert/strict');
module.exports=async(original,helpers)=>{
 await require('./scene-reload.cjs')(original,helpers);
 const active=new WeakSet();
 const value=async(page,name)=>JSON.parse(await page.locator('canvas').getAttribute(name));
 await require('./arctic-background.cjs')(original,{...helpers,command:async(page,line)=>{
  if(line==='Arctic Off'&&active.has(page)){
   const shaded=await helpers.drawing(page),geometry=(await value(page,'data-gpu-stats')).slice(0,2);
   await helpers.command(page,'Select All');assert.notEqual(await helpers.drawing(page),shaded,'Original selection remains visible over shared ambient properties');
   await helpers.command(page,'Select None');assert.equal(await helpers.drawing(page),shaded,'Clearing selection exactly restores shaded placements');
   assert.deepEqual((await value(page,'data-gpu-stats')).slice(0,2),geometry);active.delete(page);
  }
  await helpers.command(page,line);
  if(line==='Layers Off'){
   for(const command of ['Example Instances','Fit','Opacity 0.95'])await helpers.command(page,command);
   assert.equal((await value(page,'data-gpu-usage'))[2],4,'Two ordinary surfaces and two original instances exercise shared properties');
  }
  if(line==='Arctic On'){
   await page.waitForFunction(()=>{const state=JSON.parse(document.querySelector('canvas').getAttribute('data-clipping'));return state.arctic&&state.ambient[0]>320&&state.ambient[1]===1&&state.ambient[4]>=14;});
   assert.equal((await value(page,'data-clipping')).ambient_error,null);active.add(page);
  }
 }});
 console.log('Shared ambient properties: real ordinary/placed surfaces, current pixels, original selection/restoration and unchanged geometry passed at DPR 1 and 2');
};
