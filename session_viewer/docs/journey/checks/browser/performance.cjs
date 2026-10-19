const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./comparison.cjs')(page,helpers);
    const originals=JSON.parse(await page.locator('canvas').getAttribute('data-cads'));
    const before=JSON.parse(await page.locator('canvas').getAttribute('data-camera-matrix'));
    const rect=await page.locator('canvas').boundingBox();
    await page.mouse.move(rect.x+rect.width/2,rect.y+rect.height/2);await page.mouse.down({button:'right'});await page.mouse.move(rect.x+rect.width/2+60,rect.y+rect.height/2+20,{steps:12});await page.mouse.up({button:'right'});
    await page.mouse.wheel(0,-200);await page.waitForFunction(before=>document.querySelector('canvas').getAttribute('data-camera-matrix')!==JSON.stringify(before),before);
    assert.deepEqual(JSON.parse(await page.locator('canvas').getAttribute('data-cads')),originals);
    for(const line of ['Select All','Hide','Show','Close'])await helpers.command(page,line);
    assert.equal(await page.locator('canvas').getAttribute('data-object-count'),'0');
};
