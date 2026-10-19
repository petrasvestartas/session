const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./network.cjs')(page,helpers);
    const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const line of ['Close','Line w0,0,1 w1,0,1','Select All','Project To Plane 3Point','w0,0,0 w1,0,0','w2,0,0'])await helpers.command(page,line);
    assert.equal((await value('data-project')).points.length,2);assert.equal((await value('data-drawing-originals')).lines[0][0][2],1);
    await helpers.command(page,'w0,1,0');assert.equal(await value('data-project'),null);assert.equal((await value('data-drawing-originals')).lines[0][0][2],0);
    await helpers.command(page,'Undo');assert.equal((await value('data-drawing-originals')).lines[0][0][2],1);await helpers.command(page,'Redo');
    await helpers.command(page,'Extend Distance');await helpers.command(page,'0');assert.equal((await value('data-extend')).asking,true);await helpers.command(page,'1');assert.equal((await value('data-extend')).distance,1);assert.equal((await value('data-extend')).candidates,1);await helpers.command(page,'Escape');assert.equal(await value('data-extend'),null);
    for(const line of ['Close','Line w0,0,1 w1,0,1','Select All','Project To Plane XY','Extend -0.5 1.5','Curve w2,0,1 w2.5,1,2 w3,0,1','Select All','Project To Plane XY','View Isometric','Select None','Fit'])await helpers.command(page,line);
    const original=await value('data-drawing-originals');assert.deepEqual(original.lines[0],[[-0.5,0,0],[1.5,0,0]]);assert.equal((await value('data-curve-originals')).length,1);assert.equal(await value('data-tool'),null);
};
