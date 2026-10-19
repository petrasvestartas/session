const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./primitives.cjs')(page,helpers);
    const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const line of ['Close','CPlane XY','Torus','w0,0,0','2','3'])await helpers.command(page,line);
    assert.equal((await value('data-tool')).points.length,1);assert.equal((await value('data-cads')).length,0);await helpers.command(page,'0.5');assert.equal((await value('data-cads')).length,1);await helpers.command(page,'Undo');assert.equal((await value('data-cads')).length,0);await helpers.command(page,'Redo');assert.equal((await value('data-cads')).length,1);
    for(const line of ['Close','Capsule Brep w0,0,0 1 3','Quad Sphere Brep w0,0,0 1'])await helpers.command(page,line);assert.equal(await value('data-tool'),null);assert.equal((await value('data-cads')).length,0);
    for(const line of ['Capsule w0,0,0 0.3 1.5','Quad Sphere w2,0,0 0.5'])await helpers.command(page,line);assert.equal((await value('data-cads')).length,0);await helpers.command(page,'Undo');await helpers.command(page,'Redo');
    for(const line of ['Close','CPlane XY','Cylinder w-2,0,0 0.45 1','Cone w-0.7,0,0 0.45 1','Sphere w0.7,0,0.5 0.5','Torus w2,0,0.5 0.5 0.15','View Isometric','Fit'])await helpers.command(page,line);assert.equal((await value('data-cads')).length,4);
};
