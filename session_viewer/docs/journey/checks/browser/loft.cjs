const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./gather.cjs')(page,helpers);
    const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const line of ['Close','Curve w0,0,0 w0.5,0,1 w1,0,0','Curve w0,1,0 w0.5,1,1 w1,1,0','Select All','Loft Closed'])await helpers.command(page,line);assert.equal((await value('data-surfaces')).length,0);assert.equal(await value('data-gather'),null);await helpers.command(page,'Nurbs Surface Loft Straight');assert.equal((await value('data-surfaces')).length,1);await helpers.command(page,'Undo');assert.equal((await value('data-surfaces')).length,0);await helpers.command(page,'Redo');assert.equal((await value('data-surfaces')).length,1);
    for(const line of ['Close','Line w0,0,0 w3,0,0','Polyline w0,-0.2,-0.2 w0,0.2,-0.2 w0,0.2,0.2 w0,-0.2,0.2 w0,-0.2,-0.2','Select All','Nurbs Surface Sweep1'])await helpers.command(page,line);assert.equal((await value('data-surfaces')).length,1);
    for(const line of ['Close','Line w0,0,0 w3,0,0','Line w0,1,0 w3,1,0.5','Curve w0,0,0 w0,0.5,0.3 w0,1,0','Select All','Nurbs Surface Sweep2'])await helpers.command(page,line);assert.equal((await value('data-surfaces')).length,1);
    for(const line of ['Close','Curve w-1,-1,0 w0,-1,1 w1,-1,0','Curve w-1,0,0 w0,0,1.5 w1,0,0','Curve w-1,1,0 w0,1,0.75 w1,1,0','Select All','Nurbs Surface Loft Smooth','View Isometric','Select None','Fit'])await helpers.command(page,line);assert.equal((await value('data-surfaces')).length,1);assert.equal(await value('data-gather'),null);
};
