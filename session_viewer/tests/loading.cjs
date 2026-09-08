/** Last-valid-scene, revision race and malformed-payload browser regression. */
const assert=require('node:assert/strict');
const fs=require('node:fs/promises');
const {chromium}=require('playwright');
function varint(value){const bytes=[];do{bytes.push((value&127)|(value>127?128:0));value=Math.floor(value/128);}while(value);return Buffer.from(bytes);}
function field(number,body){if(typeof body==='string')body=Buffer.from(body);return Buffer.concat([varint(number*8+2),varint(body.length),body]);}
function scalar(number,value){const bytes=Buffer.alloc(9);bytes[0]=number*8+1;bytes.writeDoubleLE(value,1);return bytes;}
function fixture(count){const points=[];for(let index=0;index<count;index++)points.push(field(3,Buffer.concat([field(1,'point-'+index),scalar(3,index*100),scalar(4,0),scalar(5,0),scalar(6,8)])));return Buffer.concat([field(1,'load-test-'+count),field(3,Buffer.concat(points))]);}
function inspection(){const raw=document.querySelector('#canvas')?.getAttribute('data-viewer-inspection');return raw?JSON.parse(raw):null;}
function loaded(count){const raw=document.querySelector('#canvas')?.getAttribute('data-viewer-inspection');return raw&&JSON.parse(raw).objects===count;}
function reload(scene){window.wasmBindings.reload_scene(scene);}

async function main(){
 const browser=await chromium.launch({executablePath:process.env.CHROME_BIN||'/usr/bin/google-chrome',headless:process.env.VIEWER_HEADLESS==='1',args:process.env.VIEWER_CHROME_ARGS?JSON.parse(process.env.VIEWER_CHROME_ARGS):[]});
 const context=await browser.newContext({viewport:{width:1000,height:700},deviceScaleFactor:1});
 const page=await context.newPage();let releaseSlow;let slowRequested;const slowSeen=new Promise(resolve=>{slowRequested=resolve;});const slowGate=new Promise(resolve=>{releaseSlow=resolve;});
 const failures=[];page.on('pageerror',error=>failures.push(String(error)));
 async function route(request){
   const url=new URL(request.request().url());
   if(url.pathname==='/scenes/slow.toml'){slowRequested();await slowGate;return request.fulfill({status:200,contentType:'application/toml',body:'name="slow"\n[[items]]\nfile="pb/one.pb"\n'});}
   if(url.pathname.startsWith('/scenes/')){
     const name=url.pathname.split('/').at(-1).split('.')[0];
     if(name==='missing')return request.fulfill({status:404,body:'absent'});
     return request.fulfill({status:200,contentType:'application/toml',body:'name="'+name+'"\n[[items]]\nfile="pb/'+name+'.pb"\n'});
   }
   if(url.pathname.startsWith('/pb/')){
     const name=url.pathname.split('/').at(-1).split('.')[0];
     const body=name==='broken'?Buffer.from([0x1a,0xff]):fixture(name==='two'?2:1);
     return request.fulfill({status:200,contentType:'application/octet-stream',body});
   }
   return request.continue();
 }
 await page.route('**/scenes/**',route);await page.route('**/pb/**',route);
 try{
   await page.goto((process.env.VIEWER_URL||'http://localhost:8770/')+'?scene=one.toml&data=off&inspect=1');await page.bringToFront();
   await page.waitForFunction(loaded,1,{timeout:60000});
   const initial=await page.evaluate(inspection);
   await page.evaluate(reload,'broken.toml');
   await page.waitForFunction(()=>document.getElementById('viewer-status').textContent.includes('last valid scene'));
   assert.equal((await page.evaluate(inspection)).objects,1,'malformed replacement must retain source rows');
   await page.evaluate(reload,'slow.toml');await slowSeen;
   await page.evaluate(reload,'two.toml');await page.waitForFunction(loaded,2);
   releaseSlow();await page.waitForTimeout(600);
   assert.equal((await page.evaluate(inspection)).objects,2,'older completion must not replace the newer route');
   const cycles=[];
   for(let cycle=0;cycle<6;cycle++){
     const count=cycle%2?2:1;await page.evaluate(reload,count===1?'one.toml':'two.toml');await page.waitForFunction(loaded,count);
     await page.waitForTimeout(80);cycles.push(await page.evaluate(inspection));
   }
   assert.deepEqual(failures,[]);
   const allocations=cycles.filter(value=>value.objects===2).map(value=>value.gpu_buffer_capacity_bytes+value.gpu_texture_estimate_bytes);
   assert.equal(new Set(allocations).size,1,'same replacement workload must release prior GPU growth');
   const report={initial,cycles,errors:failures};
   await fs.mkdir(process.env.VIEWER_TEST_OUTPUT||'/tmp/session-viewer-loading',{recursive:true});
   await fs.writeFile((process.env.VIEWER_TEST_OUTPUT||'/tmp/session-viewer-loading')+'/loading.json',JSON.stringify(report,null,2));
   console.log('loading: malformed replacement, newer route wins, six resource cycles passed');
 }finally{releaseSlow();await browser.close();}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
// Viewer: NODE_PATH=/tmp/viewer-browser-test/node_modules node tests/loading.cjs. Local mocked HTTP only.
