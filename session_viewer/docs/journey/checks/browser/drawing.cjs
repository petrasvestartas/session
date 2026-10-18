const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./sheets.cjs')(page,helpers);
    const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const line of ['Close','Layers Off','View Grid Off','Snap Off','Grid Snap Off','CPlane XZ','Line 1,2 @3,4'])await helpers.command(page,line);
    assert.deepEqual((await value('data-drawing-originals')).lines,[[[1,0,2],[4,0,6]]]);
    await helpers.command(page,'Undo');assert.deepEqual((await value('data-drawing-originals')).lines,[]);await helpers.command(page,'Redo');
    await helpers.command(page,'CPlane');assert.equal((await value('data-cplane')).chooser,true);
    await helpers.command(page,'3 Point');for(const line of ['w4,5,6','w5,5,7','w4,6,6'])await helpers.command(page,line);
    assert.equal(await value('data-tool'),null);assert.equal((await value('data-cplane')).fixed,true);
    await helpers.command(page,'Line');await helpers.command(page,'1,0,0');const a=(await value('data-tool')).points[0];
    await helpers.command(page,'w9.123456789012345,8,7');assert.deepEqual((await value('data-drawing-originals')).lines[1],[a,[9.123456789012345,8,7]]);
    await helpers.command(page,'Polyline');await helpers.command(page,'w0,0,0 w1,0,0');assert.deepEqual((await value('data-drawing-originals')).polylines,[]);
    await helpers.command(page,'NaN,0,0');assert.equal((await value('data-tool')).points.length,2);await helpers.command(page,'w1,1,0');await helpers.command(page,'Close');
    assert.deepEqual((await value('data-drawing-originals')).polylines,[[[0,0,0],[1,0,0],[1,1,0],[0,0,0]]]);
    const before=await value('data-drawing-originals');const download=page.waitForEvent('download');await helpers.command(page,'Save');const stream=await(await download).createReadStream();assert(stream);const chunks=[];for await(const chunk of stream)chunks.push(chunk);
    await helpers.command(page,'Close');const chooser=page.waitForEvent('filechooser');await helpers.command(page,'Open');await(await chooser).setFiles({name:'drawing.session',mimeType:'application/octet-stream',buffer:Buffer.concat(chunks)});await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-drawing-originals')).polylines.length===1);assert.deepEqual(await value('data-drawing-originals'),before);
    for(const line of ['Close','CPlane XY','Line w-0.75,-0.4,0 w0.75,-0.4,0','Polyline w-0.6,0,0 w0,0.6,0 w0.6,0,0','Point w0,0.3,0','Fit'])await helpers.command(page,line);
};
