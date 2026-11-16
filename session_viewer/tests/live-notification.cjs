const assert=require('node:assert/strict');const fs=require('node:fs');const path=require('node:path');const {chromium}=require('playwright');
function varint(value){const bytes=[];do{bytes.push((value&127)|(value>127?128:0));value=Math.floor(value/128);}while(value);return Buffer.from(bytes);}
function field(number,body){if(typeof body==='string')body=Buffer.from(body);return Buffer.concat([varint(number*8+2),varint(body.length),body]);}
function scalar(number,value){const bytes=Buffer.alloc(9);bytes[0]=number*8+1;bytes.writeDoubleLE(value,1);return bytes;}
function fixture(count){const points=[];for(let index=0;index<count;index++)points.push(field(3,Buffer.concat([field(1,'point-'+index),scalar(3,index*100),scalar(4,0),scalar(5,0),scalar(6,8)])));return Buffer.concat([field(1,'load-test-'+count),field(3,Buffer.concat(points))]);}
function inspection(){const raw=document.querySelector('#canvas')?.getAttribute('data-viewer-inspection');return raw?JSON.parse(raw):null;}
function loaded(count){const raw=document.querySelector('#canvas')?.getAttribute('data-viewer-inspection');return raw&&JSON.parse(raw).objects===count;}
function reload(scene){window.wasmBindings.reload_scene(scene);}


(async()=>{const out=process.env.VIEWER_TEST_OUTPUT;fs.mkdirSync(out,{recursive:true});const browser=await chromium.launch({executablePath:process.env.CHROME_BIN,headless:true,args:JSON.parse(process.env.VIEWER_CHROME_ARGS)});const results=[];
try{for(const width of [1280,390]){
const page=await browser.newPage({viewport:{width,height:width===390?844:800}});let count=1,polls=0,files=0,events=0;const errors=[];page.on('pageerror',e=>errors.push(String(e)));let release;const gate=new Promise(r=>release=r);
await page.route('https://course.invalid/**',async route=>{const url=new URL(route.request().url());const headers={'Access-Control-Allow-Origin':'*','Cache-Control':'no-store'};
if(url.pathname==='/events'){events++;if(events>1)return route.fulfill({status:204,headers});await gate;return route.fulfill({headers,contentType:'text/event-stream',body:'data: {"event":"message"}\n\n'});}
if(url.pathname==='/scenes/live.yaml'){polls++;return route.fulfill({headers,contentType:'application/yaml',body:'name: notification fixture\nitems:\n  - file: https://course.invalid/pb/current.pb\n'});}
if(url.pathname==='/pb/current.pb'){files++;return route.fulfill({headers,contentType:'application/octet-stream',body:fixture(count)});}
return route.fulfill({status:404,headers});});
await page.goto(process.env.VIEWER_URL+'?live=https%3A%2F%2Fcourse.invalid%2Fscenes%2Flive.yaml&notify=https%3A%2F%2Fcourse.invalid%2Fevents&poll=3600&inspect=1');
await page.waitForFunction(loaded,1,{timeout:60000});const before=await page.evaluate(inspection);await page.screenshot({path:path.join(out,'before-'+width+'.png')});assert.equal(polls,1);assert.equal(files,1);
count=2;const announcedAt=Date.now();release();await page.waitForFunction(loaded,2,{timeout:15000});await page.waitForTimeout(250);const after=await page.evaluate(inspection);await page.screenshot({path:path.join(out,'after-'+width+'.png')});
assert.equal(polls,2,'notification causes one manifest revalidation before hourly polling');assert.equal(files,2,'mutable geometry fetched again');assert.deepEqual(after.camera_pose,before.camera_pose);assert.deepEqual(errors,[]);
results.push({width,polls,files,events,latencyMs:Date.now()-announcedAt,before,after,errors,diagnostics:await page.evaluate(()=>viewerDiagnostics.read())});await page.close();
}fs.writeFileSync(path.join(out,'notification.json'),JSON.stringify({passed:true,scope:'Mocked HTTP EventSource announcement triggers mutable geometry revalidation; no external relay availability acceptance',results},null,2));}finally{await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1});