const assert=require('node:assert/strict');
module.exports=async(original,{command,drawing})=>{
 const run=async(page,density)=>{
 const viewport=page.viewportSize();assert.deepEqual(await page.locator('canvas').evaluate(c=>[c.width,c.height]),[viewport.width*density,viewport.height*density]);
 const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
 const count=async()=>new Set((await value('data-tree')).flatMap(row=>row.ids)).size;
 await command(page,'Layers Off');await command(page,'View Grid Off');await command(page,'Snap Off');await command(page,'Grid Snap Off');await command(page,'CPlane XY');
 const originalCount=await count(),original=await value('data-drawing-originals'),pixels=await drawing(page);
 await command(page,'Close');assert.equal(await count(),originalCount);assert.deepEqual(await value('data-drawing-originals'),original);assert.equal(await drawing(page),pixels);
 await command(page,'Polyline');await command(page,'w0,0,0');await command(page,'Close');assert.deepEqual((await value('data-tool')).points,[[0,0,0]]);assert.equal(await count(),originalCount);
 await command(page,'w1,0,0');await command(page,'w1,1,0');await command(page,'Close');assert.equal(await value('data-tool'),null);assert.equal(await count(),originalCount+1);assert.deepEqual((await value('data-drawing-originals')).polylines.at(-1),[[0,0,0],[1,0,0],[1,1,0],[0,0,0]]);
 await command(page,'Undo');assert.equal(await count(),originalCount);assert.deepEqual(await value('data-drawing-originals'),original);await command(page,'Redo');assert.equal(await count(),originalCount+1);
 await command(page,'Polyline');await command(page,'w2,0,0');const draft=(await value('data-tool')).points;
 await command(page,'Opacity 0.5 extra');assert.equal((await value('data-clipping')).opacity,0.5);assert.deepEqual((await value('data-tool')).points,draft);const alpha=(await value('data-clipping')).opacity;await command(page,'Opacity 1.000000001 extra');assert.equal((await value('data-clipping')).opacity,alpha);assert.deepEqual((await value('data-tool')).points,draft);
 await command(page,'Escape');await command(page,'Opacity 1');await command(page,'Fit');
 console.log('Current Close and Opacity: full command and geometry assertions passed at DPR '+density);
 };
 await run(original,1);
 const context=await original.context().browser().newContext({viewport:original.viewportSize(),deviceScaleFactor:2});
 try{
  const page=await context.newPage(),errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.goto(original.url());await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&document.querySelector('canvas')?.hasAttribute('data-drawing-originals'));
  await run(page,2);assert.deepEqual(errors,[]);
 }finally{await context.close();await original.bringToFront();}
};
