/** Exhaustive streamed F10 regression. Virtual source ranges avoid writing a 192 MB fixture. */
const assert = require('node:assert/strict');
const http = require('node:http');
const fs = require('node:fs/promises');
const path = require('node:path');
const { chromium } = require('playwright');

const REMOTE = 6_000_000, PAGE = 65536, TOTAL = REMOTE + PAGE + 3, ORIGINAL = 0xfedcba98;
function varint(value) {let n=BigInt(value), out=[];while(n>=128n){out.push(Number(n&127n)|128);n>>=7n;}out.push(Number(n));return Buffer.from(out);}
function header(field, length) {return Buffer.concat([varint(field*8+2),varint(length)]);}
function doubles(values) {const b=Buffer.alloc(values.length*8);values.forEach((value,i)=>b.writeDoubleLE(value,i*8));return b;}
function packed(field, values, floats=false) {const b=floats?doubles(values):Buffer.concat(values.map(v=>varint(v<0?BigInt.asUintN(64,BigInt(v)):v)));return Buffer.concat([header(field,b.length),b]);}
function point(row) {if(row<4)return [[-1000,-1000,0],[1000,-1000,0],[1000,1000,0],[-1000,1000,0]][row];if(row<REMOTE)return [-1000,-1000,0];if(row===REMOTE)return [0,0,0];if(row===TOTAL-1)return [-1,0,10];return row>=TOTAL-3?[0,0,-10]:[200,0,0];}
function source() {
  const tail=Buffer.concat([
    packed(8,[-1000,-1000,-1000,-1000,-1000,0,-1,-1,-10],true),packed(9,[2000,1,256],true),packed(10,[1,1,1],true),
    packed(11,[0,1,1]),packed(12,[0,4,REMOTE]),packed(13,[4,REMOTE-4,TOTAL-REMOTE]),packed(14,[1,2,-1,-1,-1,-1,-1,-1,...Array(16).fill(-1)]),header(15,TOTAL*4),
  ]);
  const cloudBytes=header(3,TOTAL*24).length+TOTAL*24+header(4,0).length+tail.length+TOTAL*4;
  const objectsBytes=header(8,cloudBytes).length+cloudBytes;
  const head=Buffer.concat([header(3,objectsBytes),header(8,cloudBytes),header(3,TOTAL*24)]);
  const after=Buffer.concat([header(4,0),tail]);
  const coordsAt=head.length, idsAt=head.length+TOTAL*24+after.length, length=idsAt+TOTAL*4;
  function read(start,end) {
    const out=Buffer.alloc(end-start);
    for(let at=start;at<end;){
      if(at<coordsAt){const count=Math.min(end,coordsAt)-at;head.copy(out,at-start,at,at+count);at+=count;}
      else if(at<coordsAt+TOTAL*24){const local=Math.floor((at-coordsAt)/24), within=(at-coordsAt)%24, count=Math.min(end-at,24-within);doubles(point(local)).copy(out,at-start,within,within+count);at+=count;}
      else if(at<idsAt){const offset=at-(coordsAt+TOTAL*24),count=Math.min(end,idsAt)-at;after.copy(out,at-start,offset,offset+count);at+=count;}
      else {const local=Math.floor((at-idsAt)/4),within=(at-idsAt)%4,count=Math.min(end-at,4-within),value=Buffer.alloc(4);value.writeUInt32LE(local===TOTAL-1?ORIGINAL:local);value.copy(out,at-start,within,within+count);at+=count;}
    }
    return out;
  }
  return {coordsAt,idsAt,length,read};
}
function inspect() {return JSON.parse(document.querySelector('#canvas').getAttribute('data-viewer-inspection')||'null');}
function ready() {const s=window.fixtureInspect();return s&&s.cloud_points===250000&&s.canvas[0]>0;}
function controls() {const s=window.fixtureInspect();return s?.selection?.Controls?.cloud===true;}
function selectedSource() {const s=window.fixtureInspect();return s?.selection?.Controls?.selected?.Point===0xfedcba98;}
function projected(snapshot,position) {
  function mul(m,p){return Array.from({length:4},(_,r)=>m[r]*p[0]+m[r+4]*p[1]+m[r+8]*p[2]+m[r+12]*p[3]);}
  const p=mul(snapshot.mvp,mul(snapshot.model||[1,0,0,0,0,1,0,0,0,0,1,0,...snapshot.origin.map(v=>-v),1],[...position,1]));
  return [(p[0]/p[3]+1)*snapshot.canvas[0]/2,(1-p[1]/p[3])*snapshot.canvas[1]/2];
}
async function main(){
  const dpr=Number(process.env.VIEWER_DPR||1);
  const output=process.env.VIEWER_TEST_OUTPUT||`/tmp/session-viewer-streamed-controls-dpr${dpr}`;await fs.mkdir(output,{recursive:true});
  const fixture=source(), requests=[];let delayRemote=false, delayed=false, releaseTail=null, holdTail=true;
  const server=http.createServer((request,response)=>{
    const headers={'Access-Control-Allow-Origin':'*','Access-Control-Expose-Headers':'ETag, Content-Range','ETag':'"stream-fixture-v1"','Cache-Control':'no-store'};
    if(request.url.endsWith('.yaml')){response.writeHead(200,headers);response.end('name: Stream source query\nitems:\n  - file: cloud.pb\n    name: source\n    point_size: 3\n');return;}
    const match=/^bytes=(\d+)-(\d+)$/.exec(request.headers.range||'');
    if(!match){response.writeHead(400,headers);response.end('Range required');return;}
    const start=Number(match[1]),end=Math.min(Number(match[2])+1,fixture.length);requests.push({start,length:end-start});
    if(start>=end){response.writeHead(416,headers);response.end();return;}
    const send=()=>{response.writeHead(206,{...headers,'Content-Type':'application/octet-stream','Content-Range':`bytes ${start}-${end-1}/${fixture.length}`});response.end(fixture.read(start,end));};
    if(holdTail&&start===fixture.coordsAt+(REMOTE+PAGE)*24){releaseTail=send;holdTail=false;}
    else if(delayRemote&&start===fixture.coordsAt+REMOTE*24){delayed=true;setTimeout(send,700);}else send();
  });
  await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
  const base=`http://127.0.0.1:${server.address().port}/`;
  const browser=await chromium.launch({executablePath:process.env.CHROME_BIN||'/usr/bin/google-chrome',headless:process.env.VIEWER_HEADLESS==='1',args:process.env.VIEWER_CHROME_ARGS?JSON.parse(process.env.VIEWER_CHROME_ARGS):['--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE','--ozone-platform=x11']});
  let page;const errors=[],messages=[];
  try{
    page=await browser.newPage({viewport:{width:1200,height:900},deviceScaleFactor:dpr});
    page.on('pageerror',e=>errors.push(String(e)));page.on('console',m=>{messages.push(`${m.type()}: ${m.text()}`);if(m.type()==='error'){errors.push(m.text());console.error(m.text());}});
    await page.addInitScript(() => { window.fixtureInspect = () => JSON.parse(document.querySelector('#canvas')?.getAttribute('data-viewer-inspection')||'null'); });
    await page.goto((process.env.VIEWER_URL||'http://127.0.0.1:8770/')+`?scene=stream-test.yaml&data=${encodeURIComponent(base)}&points=250000&inspect=1`);
    await page.waitForFunction(ready,null,{timeout:60000});
    const metadataReads=requests.filter(r=>r.start>=fixture.coordsAt+TOTAL*24&&r.start<fixture.idsAt);
    assert.equal(metadataReads.length,2,'one color header and one cached window locate all seven LOD arrays and the ID header');
    assert.deepEqual(metadataReads.map(r=>r.length),[16,65536],'metadata does not fetch the full fixed32 ID payload');
    await page.locator('#canvas').focus();await page.keyboard.press('5');await page.waitForTimeout(250);
    // The four root samples locate the parent; no source-control selection is injected.
    let snapshot=await page.evaluate(inspect),xy=projected(snapshot,[-1000,-1000,0]);
    const rect=await page.locator('#canvas').boundingBox();
    await page.mouse.click(rect.x+xy[0]/dpr,rect.y+xy[1]/dpr);
    await page.waitForFunction(()=>window.fixtureInspect()?.selected===0,null,{timeout:15000});
    await page.keyboard.press('F10');await page.waitForFunction(controls,null,{timeout:10000});
    snapshot=await page.evaluate(inspect);xy=projected(snapshot,[0,0,0]);
    const before=requests.length;
    await page.mouse.click(rect.x+xy[0]/dpr,rect.y+xy[1]/dpr);
    for(let i=0;i<200&&!releaseTail;i++)await page.waitForTimeout(20);
    assert(releaseTail,'all-source query reached its second bounded detail page');
    assert.equal((await page.evaluate(inspect)).selection.Controls.selected,null,'first visible page must not commit a partial selection');
    releaseTail();releaseTail=null;
    await page.waitForFunction(selectedSource,null,{timeout:20000});
    snapshot=await page.evaluate(inspect);
    assert.equal(snapshot.cloud_points,250000,'F10 must retain bounded display residency');
    assert.equal(snapshot.markers,1,'only the actual selected source marker remains');
    assert.deepEqual(snapshot.controls[0].position,[-1,0,10],'front source point occludes closer-to-cursor points within and across pages');
    const queryReads=requests.slice(before);
    assert(queryReads.some(r=>r.start===fixture.coordsAt+REMOTE*24&&r.length===PAGE*24),'read the first nonresident source page');
    assert(queryReads.some(r=>r.start===fixture.coordsAt+(REMOTE+PAGE)*24&&r.length===72),'read the final nonresident source page before selecting');
    assert(queryReads.some(r=>r.start===fixture.idsAt+(TOTAL-1)*4&&r.length===4),'resolve original fixed32 source ID');
    assert(queryReads.every(r=>r.length<=65536*24),'query reads remain page bounded');
    const capture=await page.locator('#canvas').screenshot();
    const yellow=await page.evaluate(async ({png,at})=>{
      const bitmap=await createImageBitmap(await (await fetch('data:image/png;base64,'+png)).blob());
      const canvas=document.createElement('canvas');canvas.width=bitmap.width;canvas.height=bitmap.height;
      const context=canvas.getContext('2d');context.drawImage(bitmap,0,0);bitmap.close();
      const pixels=context.getImageData(Math.floor(at[0])-5,Math.floor(at[1])-5,11,11).data;
      let count=0;for(let i=0;i<pixels.length;i+=4)if(pixels[i]>200&&pixels[i+1]>180&&pixels[i+2]<80)count++;
      return count;
    },{png:capture.toString('base64'),at:projected(snapshot,[-1,0,10])});
    assert(yellow>=8,'the actual nonresident source point must be visibly yellow');
    await fs.writeFile(path.join(output,'selected-original.png'),capture);
    // A delayed page must never restore a selection after Escape invalidates its generation.
    delayRemote=true;await page.mouse.click(rect.x+xy[0]/dpr,rect.y+xy[1]/dpr);
    for(let i=0;i<100&&!delayed;i++)await page.waitForTimeout(20);
    assert(delayed,'the cancellation probe reached an asynchronous source read');
    await page.keyboard.press('Escape');await page.waitForTimeout(1000);
    assert.equal((await page.evaluate(inspect)).selection,'Object');
    assert.equal((await page.evaluate(inspect)).markers,0);
    assert.deepEqual(errors,[]);
    await fs.writeFile(path.join(output,'result.json'),JSON.stringify({dpr,source_points:TOTAL,resident_points:250000,snapshot,metadataReads,queryReads,cancelled:true,errors},null,2));
    await fs.writeFile(path.join(output,'console.log'),messages.join('\n'));
    console.log('PASS streamed controls: original ID beyond6M, bounded range/GPU residency, exhaustive node completion, cancellation');
  }catch(error){await fs.writeFile(path.join(output,'console.log'),messages.join('\n'));if(page){await page.screenshot({path:path.join(output,'failure.png')});await fs.writeFile(path.join(output,'failure.txt'),`${error}\n${await page.locator('body').innerText()}\n${JSON.stringify(await page.evaluate(inspect))}`);}throw error;}
  finally{await browser.close();await new Promise(resolve=>server.close(resolve));}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
