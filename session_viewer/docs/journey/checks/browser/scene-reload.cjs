const assert=require('node:assert/strict');
const fs=require('node:fs');const path=require('node:path');
module.exports=async(original,helpers)=>{
 await require('./ordinary-slot.cjs')(original,helpers);
 const bytes=fs.readFileSync(path.resolve('target/course-'+helpers.step.id,'sample.pb'));
 for(const density of [1,2]){
  const context=await original.context().browser().newContext({viewport:original.viewportSize(),deviceScaleFactor:density});
  try{
   await context.addInitScript(()=>{window.sourceUrls={created:[],revoked:[]};const create=URL.createObjectURL.bind(URL),revoke=URL.revokeObjectURL.bind(URL);URL.createObjectURL=blob=>{const url=create(blob);window.sourceUrls.created.push(url);return url;};URL.revokeObjectURL=url=>{window.sourceUrls.revoked.push(url);return revoke(url);};});
   const page=await context.newPage(),errors=[];page.on('pageerror',error=>errors.push(error.message));await page.goto(original.url());await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&typeof window.wasmBindings.reload_scene==='function'&&document.querySelector('canvas')?.hasAttribute('data-tree'));
   const value=async name=>JSON.parse(await page.locator('canvas').getAttribute(name));
   const load=async url=>page.evaluate(url=>window.wasmBindings.reload_scene(url),url);
   const until=async predicate=>{for(let i=0;i<200;i++){if(await predicate())return;await page.waitForTimeout(25);}assert.fail('Scene reload condition did not complete');};
   assert.deepEqual(await page.locator('canvas').evaluate(c=>[c.width,c.height]),[original.viewportSize().width*density,original.viewportSize().height*density]);
   const source=new URL('reload-original.pb',page.url()).href;await page.route(source,route=>route.fulfill({status:200,contentType:'application/octet-stream',body:bytes}));
   const manifest=new URL('reload-scene.yaml',page.url()).href;await page.route(manifest,route=>route.fulfill({status:200,contentType:'text/yaml',body:'items: [{file: '+JSON.stringify(source)+', name: Reloaded original}]\n'}));
   await load(manifest);await until(async()=>(await value('data-tree')).some(row=>row.label==='Reloaded original'));await helpers.command(page,'Fit');
   const before={tree:await value('data-tree'),origins:await value('data-source-origins'),drawing:await helpers.drawing(page)};
   const bad=new URL('reload-invalid.yaml',page.url()).href;await page.route(bad,route=>route.fulfill({status:200,contentType:'text/yaml',body:'items: ['}));await load(bad);await page.waitForFunction(()=>document.querySelector('#status').textContent.startsWith('Cannot load scene:'));
   assert.deepEqual(await value('data-tree'),before.tree);assert.deepEqual(await value('data-source-origins'),before.origins);assert.equal(await helpers.drawing(page),before.drawing);
   let finish,arrive;const held=new Promise(resolve=>{finish=resolve}),started=new Promise(resolve=>{arrive=resolve});const slow=new URL('reload-slow.yaml',page.url()).href;
   await page.route(slow,async route=>{arrive();await held;await route.fulfill({status:200,contentType:'text/yaml',body:'items: [{file: '+JSON.stringify(source)+', name: Route-less call kept pending read}]\n'});});await load(slow);await Promise.race([started,page.waitForTimeout(5000).then(()=>assert.fail("Pending reload did not begin"))]);await load(undefined);finish();await until(async()=>(await value('data-tree')).some(row=>row.label==='Route-less call kept pending read'));
   const chooser=page.waitForEvent('filechooser');await helpers.command(page,'Open Replace');await(await chooser).setFiles({name:'reload-original.pb',mimeType:'application/octet-stream',buffer:bytes});await page.waitForFunction(()=>document.querySelector('#status').textContent==='Document replaced. Undo restores the previous document.');
   const created=(await page.evaluate(()=>window.sourceUrls.created)).at(-1);assert(created);assert(!(await page.evaluate(()=>window.sourceUrls.revoked)).includes(created));assert((await value('data-cpu-usage'))[0]>0);assert((await value('data-gpu-usage'))[0]>0);
   const stale=new URL('reload-stale.yaml',page.url()).href;let release,startedStale;const late=new Promise(resolve=>{release=resolve}),arrived=new Promise(resolve=>{startedStale=resolve});await page.route(stale,async route=>{startedStale();await late;try{await route.fulfill({status:200,contentType:'text/yaml',body:'items: [{file: '+JSON.stringify(source)+', name: Stale must not return}]\n'});}catch(error){console.log('Obsolete manifest fulfilment after cancellation: '+error.message);}});await load(stale);await Promise.race([arrived,page.waitForTimeout(5000).then(()=>assert.fail("Obsolete reload did not begin"))]);
   const empty=new URL('reload-empty.yaml',page.url()).href;await page.route(empty,route=>route.fulfill({status:200,contentType:'text/yaml',body:'name: empty\nitems: []\n'}));await load(empty);await until(async()=>new Set((await value('data-tree')).flatMap(row=>row.ids)).size===0);release();await page.waitForTimeout(500);
   for(const line of ['Undo','Redo'])await helpers.command(page,line);
   assert.equal(new Set((await value('data-tree')).flatMap(row=>row.ids)).size,0);assert.deepEqual(await value('data-source-origins'),[]);assert.deepEqual((await value('data-cpu-usage')).slice(0,5),[0,0,0,0,0]);assert.deepEqual(await value('data-gpu-usage'),[0,0,0,0]);assert((await page.evaluate(()=>window.sourceUrls.revoked)).includes(created));assert.deepEqual(errors,[]);
   console.log('Current reload_scene: full candidate, atomic failure, route-less no-op, obsolete-read cancellation, empty release, Undo/Redo and original URL/mesh CPU/surface GPU release passed at DPR '+density);
  }finally{await context.close();await original.bringToFront();}
 }
};
