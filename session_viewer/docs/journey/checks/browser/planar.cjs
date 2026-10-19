const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./drawing.cjs')(page,helpers);
    const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const line of ['Close','CPlane XZ','Polyline Rectangle 1,2 @3,4'])await helpers.command(page,line);
    assert.deepEqual((await value('data-drawing-originals')).polylines,[[[1,0,2],[4,0,2],[4,0,6],[1,0,6],[1,0,2]]]);
    await helpers.command(page,'Undo');assert.deepEqual((await value('data-drawing-originals')).polylines,[]);await helpers.command(page,'Redo');
    for(const line of ['Polyline Polygon','Sides 5','w0,0,0'])await helpers.command(page,line);
    await helpers.command(page,'Sides 4096');assert.equal((await value('data-tool')).points.length,1);await helpers.command(page,'w0,0,0');assert.equal((await value('data-tool')).points.length,1);await helpers.command(page,'w0,0,2');
    const polygon=(await value('data-drawing-originals')).polylines[1];assert.equal(polygon.length,6);assert.deepEqual(polygon[0],polygon[5]);for(const point of polygon){assert.equal(point[1],0);assert(Math.abs(Math.hypot(point[0],point[2])-2)<1e-12);}
    const original=await value('data-drawing-originals');await helpers.command(page,'Undo');await helpers.command(page,'Polyline Rectangle');await helpers.command(page,'w0,0,0');await helpers.command(page,'w1,0,0');assert.equal((await value('data-tool')).points.length,1);await helpers.command(page,'Escape');await helpers.command(page,'Redo');assert.deepEqual(await value('data-drawing-originals'),original);
    for(const line of ['Close','CPlane XY','Polyline Rectangle w-0.8,-0.4,0 w-0.1,0.4,0','Polyline Polygon','Sides 5','w0.5,0,0','w0.8,0,0','Fit'])await helpers.command(page,line);
};
