const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./elements.cjs')(page,helpers);
    const state=async()=>JSON.parse(await page.locator('canvas').getAttribute('data-clipping'));
    for(const value of [0,0.4,0.7,1]){
        await helpers.command(page,`Opacity ${value}`);
        await page.waitForFunction(value=>Math.abs(JSON.parse(document.querySelector('canvas').getAttribute('data-clipping')).opacity-value)<1e-6,value);
        assert.ok(Math.abs((await state()).opacity-value)<1e-6);
    }
    for(const line of ['Opacity NaN','Opacity -1','Opacity 2','Opacity 0.5 extra']){
        await helpers.command(page,line);assert.equal((await state()).opacity,1);
    }
};
