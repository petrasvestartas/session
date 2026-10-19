const assert=require('node:assert/strict');
const fs=require('node:fs');const path=require('node:path');
module.exports=async(page,helpers)=>{
 await require('./selection-palette.cjs')(page,helpers);
 const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
 for(const line of ['Example Instances','Fit','View Isometric','Select None','Opacity 1'])await helpers.command(page,line);
 const sharedOrdinary=fs.readFileSync(path.resolve('target/course-'+helpers.step.id,'src/instance_gpu.rs'),'utf8').includes('pub(crate) ordinary:');
 const sparse=fs.readFileSync(path.resolve('target/course-'+helpers.step.id,'src/instance_gpu.rs'),'utf8').includes('lookup:Option<crate::primitive_slot_table::Table>');
 let expectedGpu=[1,2,sharedOrdinary?4*176+5*8:2*176+3*8];
 if(sparse){const parts=await value('data-instance-allocations');assert.equal(parts.length,3);assert(parts.every(Number.isSafeInteger));assert(parts[0]>=4*176&&parts[0]%176===0,'Complete record allocation');assert(parts[1]>=5*8&&parts[1]%8===0,'Retained slot allocation including the sentinel');assert(parts[2]>=1024*16&&parts[2]%(1024*16)===0,'Actual padded inverse lookup allocation');expectedGpu=[1,2,parts.reduce((sum,part)=>sum+part,0)];}
 if(sharedOrdinary)assert.equal((await value('data-gpu-usage'))[2],4,'Two ordinary surfaces and two original instances share the current record table');
 const originals=await value('data-instances');assert.equal(originals.length,2);assert.notEqual(originals[0].id,originals[1].id);assert.equal(originals[0].definition_guid,originals[1].definition_guid);
 assert.deepEqual((await value('data-instance-gpu')).slice(0,3),expectedGpu);
 const opaque=await helpers.drawing(page),geometry=(await value('data-gpu-stats')).slice(0,2);
 for(const opacity of [1,0.5]){
  await helpers.command(page,'Opacity '+opacity);assert.equal((await value('data-clipping')).opacity,opacity);
  const plain=await helpers.drawing(page);await helpers.command(page,'Select All');assert.notEqual(await helpers.drawing(page),plain);await helpers.command(page,'Select None');assert.equal(await helpers.drawing(page),plain);
  assert.deepEqual(await value('data-instances'),originals);assert.deepEqual((await value('data-gpu-stats')).slice(0,2),geometry);assert.deepEqual((await value('data-instance-gpu')).slice(0,3),expectedGpu);
 }
 await helpers.command(page,'Opacity 1');assert.equal(await helpers.drawing(page),opaque);
 const document=await value('data-drawing-originals');await helpers.command(page,'Close');assert.deepEqual(await value('data-drawing-originals'),document);assert.equal(await helpers.drawing(page),opaque);
 console.log('Ordinary own-row and instance slots: both drawing paths, exact selection restoration, unchanged originals and geometry uploads passed at DPR 1');
};
