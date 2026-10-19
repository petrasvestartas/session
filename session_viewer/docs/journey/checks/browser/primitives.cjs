const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./planar.cjs')(page,helpers);
    const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const line of ['Close','CPlane XY','Box','w0,0,0','2','Mesh',''])await helpers.command(page,line);
    assert.equal((await value('data-cads')).length,0);assert.equal((await value('data-tool')).name,'Box');await helpers.command(page,'');assert.equal(await value('data-tool'),null);const before=await value('data-gpu-stats');assert(before[0]>0);await helpers.command(page,'Undo');assert.equal((await value('data-gpu-stats'))[0],before[0]);await helpers.command(page,'Redo');
    for(const line of ['Close','Box'])await helpers.command(page,line);await helpers.command(page,'w0,0,0 2 3 4 extra');assert.equal((await value('data-tool')).points.length,0);assert.equal((await value('data-cads')).length,0);await helpers.command(page,'w0,0,0 2 3 4');assert.equal((await value('data-cads')).length,1);
    await helpers.command(page,'Undo');assert.equal((await value('data-cads')).length,0);await helpers.command(page,'Box');await helpers.command(page,'w0,0,0');await helpers.command(page,'0');assert.equal((await value('data-tool')).points.length,1);await helpers.command(page,'Escape');await helpers.command(page,'Redo');assert.equal((await value('data-cads')).length,1);
    for(const line of ['Close','View Reset','CPlane XY','Snap Off','Grid Snap Off','Point'])await helpers.command(page,line);
    const box=await page.locator('canvas').boundingBox();assert(box);const at=[box.x+box.width/2,box.y+box.height/2];await page.mouse.move(...at);await page.mouse.down();await page.mouse.move(at[0]+5,at[1]);await page.mouse.up();await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-drawing-originals')).points.length===1);assert.equal(await value('data-tool'),null);
    for(const line of ['Close','CPlane XY','Box w-1.3,0,0 0.8 0.6 0.8','Pyramid w0,0,0 0.8 1','Block With Hole w1.3,0,0 0.8 0.6 0.5 0.15','View Isometric','Fit'])await helpers.command(page,line);
    assert.equal((await value('data-cads')).length,3);
};
