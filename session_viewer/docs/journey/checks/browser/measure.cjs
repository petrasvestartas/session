const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./split-face.cjs')(page,helpers);
    const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const line of ['Close','CPlane XY','Box w-1,-1,0 2 2 0.4','Select All','BoundingBox','Length BoundingBox'])await helpers.command(page,line);
    assert.equal(await value('data-object-count'),2);
    assert.equal((await value('data-measurement')).label,'X 2 · Y 2 · Z 0.4 mm');
    await helpers.command(page,'Undo');assert.equal(await value('data-object-count'),1);
    await helpers.command(page,'Redo');assert.equal(await value('data-object-count'),2);
    for(const line of ['View Isometric','Select None','Fit','Measure Distance w-1,0,1 w1,0,1'])await helpers.command(page,line);
    assert.deepEqual(await value('data-measurement'),{points:[[-1,0,1],[1,0,1]],label:'2 mm'});
    assert.ok(await value('data-measurement-gpu-bytes')>0);
    const canvas=await page.locator('canvas').boundingBox();await page.mouse.move(canvas.x+canvas.width*0.7,canvas.y+canvas.height*0.6);await page.mouse.wheel(0,-120);
    assert.equal((await value('data-measurement')).label,'2 mm');
    await page.keyboard.press('Escape');await page.waitForFunction(()=>document.querySelector('canvas').getAttribute('data-measurement')==='null');
    await helpers.command(page,'Snap Off');await helpers.command(page,'CPlane XZ');await helpers.command(page,'Measure Distance');assert.equal((await value('data-snap-settings')).objects,true);
    await helpers.command(page,'0,0');await helpers.command(page,'@3,4');assert.deepEqual((await value('data-measurement')).points,[[0,0,0],[3,0,4]]);assert.equal((await value('data-measurement')).label,'5 mm');assert.equal(await value('data-object-count'),2);
    await helpers.command(page,'Escape');assert.equal(await value('data-measurement'),null);
};
