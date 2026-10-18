const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./round.cjs')(page,helpers);
    const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const line of ['Close','CPlane XY','Nurbs Curve Arc Center','w0,0,0','1','360'])await helpers.command(page,line);assert.equal((await value('data-curve-originals')).length,0);assert.equal((await value('data-tool')).points.length,1);await helpers.command(page,'90');assert.equal((await value('data-curve-originals'))[0].count,3);assert.equal((await value('data-curve-originals'))[0].rational,true);
    const saved=await value('data-curve-originals');await helpers.command(page,'Undo');assert.equal((await value('data-curve-originals')).length,0);await helpers.command(page,'Redo');assert.deepEqual(await value('data-curve-originals'),saved);
    for(const line of ['Close','CPlane XZ','Curve','0,0,0 2,0,0 2,2,0 0,2,0','Close'])await helpers.command(page,line);assert.equal((await value('data-curve-originals')).length,1);assert.equal(await value('data-tool'),null);
    for(const line of ['Close','CPlane XY','Nurbs Curve Circle w-2,0,0 0.5','Nurbs Curve Ellipse w-0.7,0,0 0.55 0.3','Nurbs Curve Arc Center w0.7,0,0 0.5 270','Nurbs Curve Parabola w1.5,0,0 1 0.5','Curve w-0.5,1,0 w0,1.6,0 w0.5,1,0','View Reset','Fit'])await helpers.command(page,line);assert.equal((await value('data-curve-originals')).length,5);
};
