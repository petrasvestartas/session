const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./current-commands.cjs')(page,helpers);
    const usage=async()=>JSON.parse(await page.locator('canvas').getAttribute('data-triangle-tiles'));
    for(const line of ['Close','Example CAD','Fit','View Isometric'])await helpers.command(page,line);
    await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-triangle-tiles'))[3]>0);
    const canvas=await page.locator('canvas').boundingBox();assert.ok(canvas);
    const before=await usage();await page.mouse.move(canvas.x+canvas.width*0.55,canvas.y+canvas.height*0.55);await page.mouse.down({button:'right'});await page.mouse.move(canvas.x+canvas.width*0.62,canvas.y+canvas.height*0.6,{steps:4});await page.mouse.up({button:'right'});
    await page.waitForFunction(count=>JSON.parse(document.querySelector('canvas').getAttribute('data-triangle-tiles'))[5]>count,before[5]);
    assert.ok((await usage())[0]>=96);
    await helpers.command(page,'Close');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-triangle-tiles'))[3]===0);
    assert.deepEqual((await usage()).slice(0,5),[96,16,48,0,0]);
};
