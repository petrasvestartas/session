const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./loft.cjs')(page,helpers);
    const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const line of ['Close','Nurbs Surface 4 Points','w0,0,0 w1,0,0 w2,0,0','w3,0,0'])await helpers.command(page,line);assert.equal((await value('data-tool')).points.length,3);assert.equal((await value('data-surfaces')).length,0);await helpers.command(page,'Escape');assert.equal(await value('data-gather'),null);
    for(const line of ['Close','Line w0,1,0 w2,1,1','Line w2,0,0 w2,2,2','Line w2,0,0 w0,0,0','Line w1,0,0 w1,2,1','Line w0,2,0 w2,2,2','Line w0,0,0 w0,2,0','Select All','Nurbs Surface Network','Nurbs Surface 4 Points w3,0,0 w4,0,0 w4,1,0.3 w3,1,0','View Isometric','Select None','Fit'])await helpers.command(page,line);assert.equal((await value('data-surfaces')).length,2);assert.equal(await value('data-gather'),null);await helpers.command(page,'Undo');assert.equal((await value('data-surfaces')).length,1);await helpers.command(page,'Redo');assert.equal((await value('data-surfaces')).length,2);
};
