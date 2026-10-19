const assert=require('node:assert/strict');
async function focused(page,helpers){
    const state=async()=>JSON.parse(await page.locator('canvas').getAttribute('data-clipping'));
    for(const command of ['Close','View Grid Off','Example CAD','Clipping Plane YZ 0,0,0'])await helpers.command(page,command);
    await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).usage[0]===1);
    const original=await state();assert.equal(original.error,null);assert.equal(original.usage[4],448);assert.equal(original.rows.length,1);
    await helpers.command(page,'Clipping Plane Off');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).usage[1]===0);assert.deepEqual((await state()).rows,original.rows);
    await helpers.command(page,'Clipping Plane On');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).usage[1]===1);
    await helpers.command(page,'Clipping Plane Flip');await page.waitForFunction(before=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).rows[0].normal.some((v,i)=>v!==before[i]),original.rows[0].normal);
    const flipped=await state();assert.equal(flipped.rows[0].id,original.rows[0].id);assert.ok(flipped.rows[0].normal.every((v,i)=>Math.abs(v+original.rows[0].normal[i])<1e-12));
    await helpers.command(page,'Undo');await page.waitForFunction(before=>JSON.stringify(JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).rows)===JSON.stringify(before),original.rows);
    await helpers.command(page,'Close');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).usage[0]===0);assert.deepEqual((await state()).ink_flags,[0,0]);
}
module.exports=async(page,helpers)=>{
    await require('./finite-ink.cjs')(page,helpers);
    await focused(page,helpers);
};
module.exports.focused=focused;
