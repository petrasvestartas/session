const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./freeform.cjs')(page,helpers);
    const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const line of ['Close','CPlane XY','Box w0,0,0 1 1 1','Select All','Copy','w0,0,0 w2,0,0 w4,0,0',''])await helpers.command(page,line);assert.equal((await value('data-cads')).length,3);assert.equal(await value('data-tool'),null);await helpers.command(page,'Undo');assert.equal((await value('data-cads')).length,2);await helpers.command(page,'Undo');assert.equal((await value('data-cads')).length,1);await helpers.command(page,'Redo');await helpers.command(page,'Redo');
    for(const line of ['Scale','1D','w0,0,0','2'])await helpers.command(page,line);assert.equal((await value('data-tool')).points.length,1);await helpers.command(page,'w1,0,0');await helpers.command(page,'3');assert.equal(await value('data-tool'),null);await helpers.command(page,'Undo');await helpers.command(page,'Rotate');await helpers.command(page,'w0,0,0 w1,0,0');await helpers.command(page,'Escape');await helpers.command(page,'Redo');assert.equal((await value('data-cads')).length,3);
    for(const line of ['Close','CPlane XY','Box w0,0,0 1 1 1','Select All','Copy 2,0,0','Scale 0.8','Rotate z 30','Copy -2,0,0','View Isometric','Select None','Fit'])await helpers.command(page,line);assert.equal((await value('data-cads')).length,3);
};
