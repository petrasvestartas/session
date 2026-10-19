const assert=require('node:assert/strict');
const {PNG}=require('pngjs');
module.exports=async(page,helpers)=>{
 await require('./current-command.cjs')(page,helpers);
 await helpers.command(page,'Opacity 1');await helpers.command(page,'Select None');await helpers.command(page,'Fit');
 const plain=await helpers.drawing(page),uploads=JSON.parse(await page.locator('canvas').getAttribute('data-gpu-stats'));
 await helpers.command(page,'Select All');assert.notEqual(await helpers.drawing(page),plain);
 const image=PNG.sync.read(Buffer.from((await require('./canvas-image.cjs')(page.locator('canvas'))).split(',')[1],'base64'));let yellow=0;
 for(let i=0;i<image.width*(image.height-34)*4;i+=4){const [r,g,b]=image.data.subarray(i,i+3);if(r>40&&r===g&&b===0)yellow++;}
 assert(yellow>100,'Current surface selection uses equal red/green and zero blue, including its lighting');
 await helpers.command(page,'Select None');assert.equal(await helpers.drawing(page),plain,'Clearing selection restores every scene pixel');
 assert(Array.isArray(uploads)&&uploads.length>0);
 const restored=JSON.parse(await page.locator('canvas').getAttribute('data-gpu-stats'));
 assert.deepEqual(restored.slice(0,2),uploads.slice(0,2),'Selection does not upload primary geometry');
 assert(restored[2]>uploads[2]&&restored[3]>uploads[3],'Selection settings are updated independently');
 console.log('Current shared selection yellow: actual selected pixels and exact restoration passed');
};
