const assert=require('node:assert/strict');const fs=require('node:fs');const path=require('node:path');const {chromium}=require('playwright');
function varint(value){const bytes=[];do{bytes.push((value&127)|(value>127?128:0));value=Math.floor(value/128);}while(value);return Buffer.from(bytes);}
function field(number,body){if(typeof body==='string')body=Buffer.from(body);return Buffer.concat([varint(number*8+2),varint(body.length),body]);}
function scalar(number,value){const bytes=Buffer.alloc(9);bytes[0]=number*8+1;bytes.writeDoubleLE(value,1);return bytes;}
function fixture(count){const points=[];for(let index=0;index<count;index++)points.push(field(3,Buffer.concat([field(1,'point-'+index),scalar(3,index*100),scalar(4,0),scalar(5,0),scalar(6,8)])));return Buffer.concat([field(1,'load-test-'+count),field(3,Buffer.concat(points))]);}
function inspection(){const raw=document.querySelector('#canvas')?.getAttribute('data-viewer-inspection');return raw?JSON.parse(raw):null;}
function loaded(count){const raw=document.querySelector('#canvas')?.getAttribute('data-viewer-inspection');return raw&&JSON.parse(raw).objects===count;}
function reload(scene){window.wasmBindings.reload_scene(scene);}


async function command(page,text) {
  const ui=JSON.parse(await page.locator('#canvas').getAttribute('data-viewer-ui'));
  const {rect}=ui.controls.find(c=>c.key==='command/input');
  await page.mouse.click((rect[0]+rect[2])/2,rect[3]-2);
  if(ui.command) await page.keyboard.press('Control+a');
  await page.keyboard.type(text,{delay:20});await page.keyboard.press('Enter');await page.waitForTimeout(300);
  assert(JSON.stringify(JSON.parse(await page.locator('#canvas').getAttribute('data-viewer-ui')).history).includes(text));
}

(async()=>{
const out=process.env.VIEWER_TEST_OUTPUT;fs.mkdirSync(out,{recursive:true});
const browser=await chromium.launch({executablePath:process.env.CHROME_BIN,headless:true,args:JSON.parse(process.env.VIEWER_CHROME_ARGS)});const results=[];
try {for(const width of [1280,390]) {
 const page=await browser.newPage({viewport:{width,height:width===390?844:800},acceptDownloads:true});
 let reads=0,fail=false;const errors=[],downloads=[];page.on('pageerror',e=>errors.push(String(e)));page.on('download',d=>downloads.push(d));
 await page.route('https://course.invalid/**',async route=>{
  const headers={'Access-Control-Allow-Origin':'*','Access-Control-Expose-Headers':'Content-Range, Content-Length','Cache-Control':'no-store'};
  if(new URL(route.request().url()).pathname.endsWith('.yaml'))return route.fulfill({headers,contentType:'application/yaml',body:'name: Released source\nitems:\n  - file: https://course.invalid/source.pb\n    display_only: true\n'});
  reads++;console.log(JSON.stringify({width,read:reads,method:route.request().method(),url:route.request().url(),range:route.request().headers().range,fail}));if(fail)return route.fulfill({status:500,headers,body:'Hydration fixture failure'});
  const body=fixture(2),range=/^bytes=(\d+)-(\d+)$/.exec(route.request().headers().range||'');
  if(range){const start=Number(range[1]),end=Math.min(Number(range[2])+1,body.length);return route.fulfill({status:206,headers:{...headers,'Content-Range':`bytes ${start}-${end-1}/${body.length}`},contentType:'application/octet-stream',body:body.subarray(start,end)});}
  return route.fulfill({headers,contentType:'application/octet-stream',body});
 });
 await page.goto(process.env.VIEWER_URL+'?scene=released.yaml&data=https%3A%2F%2Fcourse.invalid%2F&inspect=1');
 await page.waitForFunction(loaded,2,{timeout:60000});await page.waitForTimeout(200);
 const released=await page.evaluate(inspection);assert.equal(released.source_cpu_known_payload.unique_geometry_values,0,'Display rows remain after kernel objects released');assert.equal(reads,1);
 await page.screenshot({path:path.join(out,'released-'+width+'.png')});
 fail=true;await command(page,'Save');
 await page.waitForFunction(()=>document.querySelector('#viewer-status')?.textContent.includes('Cannot edit'),null,{timeout:15000});
 assert.equal(reads,2);assert.equal(downloads.length,0,'Failed hydration cannot save an incomplete document');
 const failed=await page.evaluate(inspection);assert.equal(failed.objects,2);assert.equal(failed.source_cpu_known_payload.unique_geometry_values,0);
 await page.screenshot({path:path.join(out,'failed-'+width+'.png')});
 fail=false;const downloading=page.waitForEvent('download',{timeout:15000});await command(page,'Save');const download=await downloading;
 await download.saveAs(path.join(out,'restored-'+width+'.session'));
 await page.waitForFunction(()=>JSON.parse(document.querySelector('#canvas')?.getAttribute('data-viewer-inspection')||'null')?.source_cpu_known_payload.unique_geometry_values===2,null,{timeout:15000});
 const restored=await page.evaluate(inspection);assert.equal(reads,3);assert.equal(downloads.length,1);assert.equal(restored.objects,2);assert.deepEqual(restored.camera_pose,released.camera_pose);assert.deepEqual(errors,[]);
 await page.screenshot({path:path.join(out,'restored-'+width+'.png')});
 results.push({width,reads,download:download.suggestedFilename(),released,failed,restored,errors,diagnostics:await page.evaluate(()=>viewerDiagnostics.read())});await page.close();
}
fs.writeFileSync(path.join(out,'hydration.json'),JSON.stringify({passed:true,scope:'Display-only source release, failed Save hydration preserves visible rows, explicit retry restores source and downloads complete session; synthetic two-point fixture',results},null,2));
}finally{await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
