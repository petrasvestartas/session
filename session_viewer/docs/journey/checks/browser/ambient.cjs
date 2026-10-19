const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./pyramid.cjs')(page,helpers);
    const state=async()=>JSON.parse(await page.locator('canvas').getAttribute('data-clipping'));
    for(const command of ['Close','Example CAD','View Isometric','Arctic On'])await helpers.command(page,command);
    await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).ambient[0]>320);
    const active=await state();assert.equal(active.arctic,true);assert.equal(active.ambient_error,null);assert.equal(active.ambient[1],1);
    await helpers.command(page,'Arctic Off');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).ambient[0]===320);assert.equal((await state()).arctic,false);
    await helpers.command(page,'Arctic On');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).ambient[0]>320);assert.equal((await state()).ambient[1],1);
    await helpers.command(page,'Close');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).ambient[0]===320);assert.equal((await state()).ambient_error,null);
    await helpers.command(page,'Arctic Off');
};
