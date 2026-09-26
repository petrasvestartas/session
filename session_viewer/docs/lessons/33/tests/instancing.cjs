/** Instancing browser check: a scene of definitions and instances uploads each definition once and draws every instance. */
const assert=require('node:assert/strict');
const fs=require('node:fs/promises');
const {chromium}=require('playwright');
function inspection(){const raw=document.querySelector('#canvas')?.getAttribute('data-viewer-inspection');if(!raw)return null;const s=JSON.parse(raw);delete s.text_labels;delete s.source_cpu_known_payload;return s;}
function ui(){return JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-ui'));}
async function command(page,text){const c=(await page.evaluate(ui)).controls.find(c=>c.key==='command/input');const [x,y,r,b]=c.rect;await page.mouse.click((x+r)/2,(y+b)/2);await page.keyboard.press('Control+a');await page.keyboard.type(text);await page.keyboard.press('Enter');await page.waitForFunction(text=>JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-ui')).history.at(-1)?.startsWith('> '+text+'\n'),text,{timeout:30000});await page.waitForTimeout(800);}
function manifest(file){return 'name: '+file+'\nitems:\n  - file: pb/'+file+'.pb\n';}

// the grid scene with INSTANCES on and off; VIEWER_ON / VIEWER_OFF name the .pb files
const scenes={on:process.env.VIEWER_ON,off:process.env.VIEWER_OFF};
const out=process.env.VIEWER_TEST_OUTPUT||'/tmp/session-viewer-instancing';

async function load(browser,name,query=''){
 const context=await browser.newContext({viewport:{width:1400,height:900},deviceScaleFactor:1});
 const page=await context.newPage();const errors=[],perf=[];
 page.on('pageerror',error=>errors.push(String(error)));
 page.on('console',m=>{const p=/perf: ([\d.]+) fps \| ([\d.]+) ms \| (\d+) draws/.exec(m.text());if(p)perf.push({fps:+p[1],ms:+p[2],draws:+p[3]});if(m.type()==='error'||/validation|panicked|device lost/i.test(m.text()))errors.push(m.text());});
 await page.route('**/scenes/**',route=>route.fulfill({status:200,contentType:'application/yaml',body:manifest(name)}));
 await page.route('**/pb/**',async route=>route.fulfill({status:200,contentType:'application/octet-stream',body:await fs.readFile(scenes[name])}));
 await page.goto((process.env.VIEWER_URL||'http://localhost:8770/')+'?scene='+name+'&data=off&inspect=1'+query);await page.bringToFront();
 await page.waitForFunction(()=>{const raw=document.querySelector('#canvas')?.getAttribute('data-viewer-inspection');return raw&&JSON.parse(raw).objects>0;},null,{timeout:120000});
 // settled: the same counts for a second
 let last='';for(let still=0;still<5;){await page.waitForTimeout(200);const s=await page.evaluate(inspection);const key=[s.objects,s.vertices,s.pipes,s.frames>0].join();still=key===last?still+1:0;last=key;}
 return {page,context,errors,perf,state:await page.evaluate(inspection)};
}

async function main(){
 const browser=await chromium.launch({executablePath:process.env.CHROME_BIN||'/usr/bin/google-chrome',headless:process.env.VIEWER_HEADLESS==='1',args:process.env.VIEWER_CHROME_ARGS?JSON.parse(process.env.VIEWER_CHROME_ARGS):['--enable-unsafe-webgpu','--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE','--ozone-platform=x11']});
 try{
   await fs.mkdir(out,{recursive:true});
   const off=await load(browser,'off');const a=off.state;
   await off.page.screenshot({path:out+'/instancing-off.png'});
   await command(off.page,'Arctic On');await off.page.screenshot({path:out+'/instancing-off-ssao.png'});
   await off.context.close();
   const on=await load(browser,'on');const b=on.state;const i=b.instancing;
   assert.deepEqual(off.errors,[],'no errors without instances');
   assert.deepEqual(on.errors,[],'no errors with instances');
   assert.ok(i.instances>0&&i.definitions>0,'the scene holds definitions and instances');
   assert.equal(b.objects,a.objects,'one row per instance, as many as the elements they replace');
   assert.equal(i.definition_uploads,i.definitions,'every definition uploaded once, none per instance');
   assert.equal(i.gpu_instances,i.instances,'the instanced draws place every instance');
   assert.equal(i.gpu_draws,i.definitions,'one instanced draw per definition');
   assert.equal(i.shared_instance_rows,i.instances,'every instance drawn from its shared definition');
   assert.ok(b.vertices<a.vertices,'fewer vertices than the same scene with every element walked');
   assert.ok(b.pipes<a.pipes,'fewer edge segments too');
   await on.page.screenshot({path:out+'/instancing.png'});
   // a click picks one instance; H hides that one alone
   const [w,h]=b.logical_canvas;let picked=null;
   for(let k=0;k<40&&!picked;k++){await on.page.mouse.click(w*(0.3+0.4*((k*7)%10)/10),h*(0.35+0.4*((k*3)%10)/10));await on.page.waitForTimeout(250);const s=await on.page.evaluate(inspection);if(s.selected!=null)picked=s;}
   assert.ok(picked,'a click selects an object');
   assert.equal(typeof picked.selected_instance,'string','the selected row is an instance');
   await on.page.screenshot({path:out+'/instancing-selected.png'});
   await on.page.keyboard.press('h');await on.page.waitForTimeout(500);
   const hidden=await on.page.evaluate(inspection);
   assert.equal(hidden.hidden_count,1,'one instance hidden');
   assert.equal(hidden.instancing.definition_uploads,i.definition_uploads,'hiding changes no upload');
   await on.page.screenshot({path:out+'/instancing-hidden.png'});
   await command(on.page,'Arctic On');await on.page.screenshot({path:out+'/instancing-ssao.png'});
   assert.deepEqual(on.errors,[],'no errors after picking, hiding and SSAO');
   // the instanced frame against the same scene with every element walked: faces, edges and features
   const {PNG}=require('pngjs');const pa=PNG.sync.read(await fs.readFile(out+'/instancing.png')),pb=PNG.sync.read(await fs.readFile(out+'/instancing-off.png'));
   let far=0;for(let k=0;k<pa.data.length;k+=4)if(Math.max(Math.abs(pa.data[k]-pb.data[k]),Math.abs(pa.data[k+1]-pb.data[k+1]),Math.abs(pa.data[k+2]-pb.data[k+2]))>40)far++;
   const share=far/(pa.width*pa.height);
   assert.ok(share<0.005,'instanced and walked frames differ in '+far+' pixels');
   // frame time with the viewer's own counter, orbiting every frame
   const timing={};
   for(const name of ['off','on']){const run=await load(browser,name,'&perf=1&spin=1');await run.page.waitForTimeout(6000);const ms=run.perf.map(p=>p.ms).sort((x,y)=>x-y);timing[name]={ms:ms[ms.length>>1],fps:run.perf.at(-1)?.fps,draws:run.perf.at(-1)?.draws};await run.context.close();}
   const report={pixels_differ:far,timing,off:a,on:b,delta:{vertices:b.vertices-a.vertices,pipes:b.pipes-a.pipes,gpu_buffer_bytes:b.gpu_buffer_capacity_bytes-a.gpu_buffer_capacity_bytes,wasm_heap_bytes:b.wasm_capacity_bytes-a.wasm_capacity_bytes,source_bytes:b.source_cpu_known_payload_bytes-a.source_cpu_known_payload_bytes}};
   await fs.writeFile(out+'/instancing.json',JSON.stringify(report,null,2));
   console.log('instancing: '+i.instances+' instances of '+i.definitions+' definitions, one upload each; '+far+' pixels differ; '+JSON.stringify(timing)+' '+JSON.stringify(report.delta));
 }finally{await browser.close();}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
// Viewer: VIEWER_ON=grid_on.pb VIEWER_OFF=grid_off.pb NODE_PATH=/tmp/viewer-browser-test/node_modules node tests/instancing.cjs.
// The .pb files come from wood/examples/templates_grid.cpp built with INSTANCES true and false. Local mocked HTTP only.
