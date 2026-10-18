const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./ambient.cjs')(page,helpers);
    const state=async()=>JSON.parse(await page.locator('canvas').getAttribute('data-clipping'));
    for(const line of ['Close','Example CAD','View Isometric','View Outline On'])await helpers.command(page,line);
    await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).global_outline[0]>0);
    assert.equal((await state()).outlines,true);
    await helpers.command(page,'View Outline Off');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).global_outline[0]===0);
    await helpers.command(page,'Arctic On');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).global_outline[0]>0);
    await helpers.command(page,'Arctic Off');assert.equal((await state()).outlines,true);
    await helpers.command(page,'View Outline Off');assert.equal((await state()).outlines,false);
    await helpers.command(page,'Close');assert.equal((await state()).global_outline[0],0);
};
