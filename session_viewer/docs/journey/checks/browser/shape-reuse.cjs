const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./instances.cjs')(page,helpers);
    const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
    for(const line of ['Close','Layers Off','View Grid Off','Example Shape Reuse','Fit','View Isometric'])await helpers.command(page,line);
    let rows=await value('data-cads');assert.equal(rows.length,2);assert.notEqual(rows[0].source,rows[1].source);assert.notEqual(rows[0].id,rows[1].id);assert.deepEqual(await value('data-cad-walks'),{scope:'last imported original document',walks:1,replays:1});assert.equal((await value('data-instances')).length,0);
    const event=page.waitForEvent('download');await helpers.command(page,'Save');const download=await event,stream=await download.createReadStream();assert(stream);const chunks=[];for await(const chunk of stream)chunks.push(chunk);const bytes=Buffer.concat(chunks),original=rows.map(r=>r.source);
    await helpers.command(page,'Close');assert.deepEqual(await value('data-cad-walks'),{scope:'last imported original document',walks:0,replays:0});const chooser=page.waitForEvent('filechooser');await helpers.command(page,'Open');await(await chooser).setFiles({name:'viewer.session',mimeType:'application/octet-stream',buffer:bytes});await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-cads')).length===2);rows=await value('data-cads');assert.deepEqual(rows.map(r=>r.source),original);assert.equal((await value('data-cad-walks')).replays,1);
    await helpers.command(page,'Select Next');await require('./delete-selection.cjs')(page,[rows[0].id]);await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-cads')).length===1);assert.equal((await value('data-cads')).length,1);await helpers.command(page,'Undo');assert.equal((await value('data-cads')).length,2);await helpers.command(page,'Redo');assert.equal((await value('data-cads')).length,1);
    for(const line of ['Close','Example Shape Reuse','Fit','View Isometric','Select Next','Select Next'])await helpers.command(page,line);
};
