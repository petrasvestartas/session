const assert=require('node:assert/strict');
module.exports=async(page,helpers)=>{
    await require('./shape-reuse.cjs')(page,helpers);
    const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
    const models=async()=>(await value('data-instances')).sort((a,b)=>a.source_guid.localeCompare(b.source_guid)).map(r=>[r.source_guid,r.model]);
    const locals=async()=>(await value('data-original-tree-placements')).sort((a,b)=>a.name.localeCompare(b.name)).map(r=>[r.name,r.local]);
    for(const line of ['Close','Layers Off','View Grid Off','Example Nested Instances','Fit','View Isometric'])await helpers.command(page,line);
    const original=await models(),frames=await locals();assert.equal(original.length,2);const branch=frames.find(([name])=>name==='branch'),mirror=frames.find(([name])=>name==='mirror-branch');assert(branch&&mirror);assert(branch[1][1]>0.7);assert.equal(mirror[1][0],-1);assert.deepEqual((await value('data-instance-gpu')).slice(0,3),[1,2,320]);
    await helpers.command(page,'Select All');await helpers.command(page,'Move 0.25 0 0');const moved=await models();for(let i=0;i<original.length;i++)assert(Math.abs(moved[i][1][12]-original[i][1][12]-0.25)<1e-12);await helpers.command(page,'Undo');assert.deepEqual(await models(),original);await helpers.command(page,'Redo');assert.deepEqual(await models(),moved);await helpers.command(page,'Undo');
    const event=page.waitForEvent('download');await helpers.command(page,'Save');const download=await event,stream=await download.createReadStream();assert(stream);const chunks=[];for await(const chunk of stream)chunks.push(chunk);const bytes=Buffer.concat(chunks);await helpers.command(page,'Close');const chooser=page.waitForEvent('filechooser');await helpers.command(page,'Open');await(await chooser).setFiles({name:'viewer.session',mimeType:'application/octet-stream',buffer:bytes});await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-instances')).length===2);assert.deepEqual(await models(),original);assert.deepEqual(await locals(),frames);
    await helpers.command(page,'Close');assert.deepEqual(await value('data-original-tree-placements'),[]);assert.deepEqual(await value('data-instance-cpu'),[0,0,0]);
    for(const line of ['Example Nested Instances','Fit','View Isometric','Select Next','Select Next'])await helpers.command(page,line);
};
