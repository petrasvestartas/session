const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./caps.cjs')(page,helpers);
    const state=async()=>JSON.parse(await page.locator('canvas').getAttribute('data-clipping'));
    assert.deepEqual((await state()).pyramid,[0,0,0,0]);
    assert.equal((await state()).pyramid_error,null);
    for(const command of ['Example CAD','View Top','View Isometric'])await helpers.command(page,command);
    assert.deepEqual((await state()).pyramid,[0,0,0,0]);
    await helpers.command(page,'Close');
};
