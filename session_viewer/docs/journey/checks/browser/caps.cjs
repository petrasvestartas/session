const assert=require('node:assert/strict');
async function focused(page,helpers,emptyBytes=24){
    const state=async()=>JSON.parse(await page.locator('canvas').getAttribute('data-clipping'));
    const checks=(await state()).caps[2];
    for(const command of ['Close','Example CAD','Clipping Plane XY 0,0,0'])await helpers.command(page,command);
    await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).caps[1]===1);
    const active=await state();assert.equal(active.cap_error,null);assert.ok(active.caps[0]>emptyBytes);assert.equal(active.caps[2],checks+1,'The new original is classified exactly once');
    await helpers.command(page,'Clipping Plane Hatch');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).usage[2]===0);assert.equal((await state()).caps[2],active.caps[2],'Presentation changes retain the original closedness cache');
    await helpers.command(page,'Clipping Plane Solid');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).usage[2]===1);
    await helpers.command(page,'Clipping Plane Off');await page.waitForFunction(bytes=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).caps[0]===bytes,emptyBytes);
    await helpers.command(page,'Clipping Plane On');await page.waitForFunction(bytes=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).caps[0]>bytes,emptyBytes);assert.equal((await state()).caps[2],active.caps[2],'Presentation changes retain the original closedness cache');
    await helpers.command(page,'Close');await page.waitForFunction(bytes=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).caps[0]===bytes,emptyBytes);assert.equal((await state()).caps[1],0);
}
module.exports=async(page,helpers)=>{
    await require('./clipping.cjs')(page,helpers);
    await focused(page,helpers);
};
module.exports.focused=focused;
