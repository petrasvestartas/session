const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./transform.cjs')(page,helpers);
    const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const line of ['Close','CPlane XY','Polyline Rectangle w0,0,0 w1,1,0','Select All','Extrude','Cap','Off','0'])await helpers.command(page,line);
    assert.equal((await value('data-gather')).name,'Extrude');assert.equal((await value('data-cads')).length,0);await helpers.command(page,'2');assert.equal(await value('data-gather'),null);assert.equal((await value('data-cads')).length,1);await helpers.command(page,'Undo');assert.equal((await value('data-cads')).length,0);await helpers.command(page,'Redo');assert.equal((await value('data-cads')).length,1);
    for(const line of ['Close','CPlane XY','Polyline Rectangle w-2,0,0 w-1,1,0','Select All','Extrude 1.5','Line w1.9,0,0 w1.9,0,1.5','Select Next','Nurbs Surface Revolve w1.5,0,0 w1.5,0,1','View Isometric','Select None','Fit'])await helpers.command(page,line);
    assert.equal((await value('data-cads')).length,1);assert.equal((await value('data-surfaces')).length,1);assert.equal(await value('data-gather'),null);await helpers.command(page,'Undo');assert.equal((await value('data-surfaces')).length,0);await helpers.command(page,'Redo');assert.equal((await value('data-surfaces')).length,1);
};
