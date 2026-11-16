const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const {chromium} = require('playwright');

function varint(value) {
  const bytes = [];
  do { bytes.push((value & 127) | (value > 127 ? 128 : 0)); value = Math.floor(value / 128); } while (value);
  return Buffer.from(bytes);
}
function field(number, body) {
  body = Buffer.isBuffer(body) ? body : Buffer.from(body);
  return Buffer.concat([varint(number * 8 + 2), varint(body.length), body]);
}
function uint(number, value) { return Buffer.concat([varint(number * 8), varint(value)]); }
function array(values, bytes, write) {
  const result = Buffer.alloc(values.length * bytes);
  values.forEach((value, i) => result[write](value, i * bytes));
  return result;
}
function fixture() {
  const sheet = Buffer.concat([
    field(1, 'sheet-fixture'), field(2, 'Two entities'),
    field(3, array([-1000,500,0,1000,500,0,-1000,-500,0,1000,-500,0], 8, 'writeDoubleLE')),
    field(4, array([0xff000000,0xff000000], 4, 'writeUInt32LE')),
    field(5, array([3,3], 4, 'writeFloatLE')), uint(6, 2), uint(7, 2),
    field(8, 'plan.shm'), field(15, array([0,1], 4, 'writeUInt32LE')),
  ]);
  const blobs = ['Wall A','Door B'].map((name,id) => Buffer.from(JSON.stringify({guid:'entity-'+id,name,kind:id?'door':'wall',width:3,color:[0,0,0,255]})));
  const head = Buffer.alloc(40); head.write('SHM1'); head.writeUInt32LE(2,4);
  let offset=0;
  blobs.forEach((blob,id) => {head.writeBigUInt64LE(BigInt(offset),8+16*id);head.writeBigUInt64LE(BigInt(blob.length),16+16*id);offset+=blob.length;});
  return {geometry:field(3,field(17,sheet)), metadata:Buffer.concat([head,...blobs]), secondBlob:40+blobs[0].length};
}
function inspect() { return JSON.parse(document.querySelector('#canvas')?.getAttribute('data-viewer-inspection') || 'null'); }
function projected(snapshot, position) {
  function mul(m,p) {return Array.from({length:4},(_,r)=>m[r]*p[0]+m[r+4]*p[1]+m[r+8]*p[2]+m[r+12]*p[3]);}
  const p=mul(snapshot.mvp,mul(snapshot.model||[1,0,0,0,0,1,0,0,0,0,1,0,...snapshot.origin.map(v=>-v),1],[...position,1]));
  return [(p[0]/p[3]+1)*snapshot.canvas[0]/2,(1-p[1]/p[3])*snapshot.canvas[1]/2];
}
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
  const browser=await chromium.launch({executablePath:process.env.CHROME_BIN,headless:true,args:JSON.parse(process.env.VIEWER_CHROME_ARGS)});
  const results=[];
  try {for(const width of [1280,390]) {
    const page=await browser.newPage({viewport:{width,height:width===390?844:800}});
    const data=fixture(),requests=[],errors=[];let hold=false,release=null;
    page.on('pageerror',e=>errors.push(String(e)));
    await page.route('https://course.invalid/**',async route=>{
      const url=new URL(route.request().url());
      const headers={'Access-Control-Allow-Origin':'*','Access-Control-Expose-Headers':'ETag, Content-Range','Cache-Control':'no-store','ETag':'"sheet-v1"'};
      if(url.pathname.endsWith('.yaml'))return route.fulfill({headers,contentType:'application/yaml',body:'name: Two sheet entities\nitems:\n  - file: https://course.invalid/plan.pb\n'});
      const body=url.pathname.endsWith('.shm')?data.metadata:data.geometry;
      const range=route.request().headers().range;
      const match=/^bytes=(\d+)-(\d+)$/.exec(range||'');assert(match,'Every sheet and metadata read must be ranged');
      const start=Number(match[1]),end=Math.min(Number(match[2])+1,body.length);
      requests.push({file:url.pathname,start,length:end-start});assert(start<end);
      if(hold&&url.pathname.endsWith('.shm')&&start===data.secondBlob){hold=false;await new Promise(r=>release=r);}
      return route.fulfill({status:206,headers:{...headers,'Content-Range':`bytes ${start}-${end-1}/${body.length}`},contentType:'application/octet-stream',body:body.subarray(start,end)});
    });
    await page.goto(process.env.VIEWER_URL+'?scene=sheet.yaml&data=https%3A%2F%2Fcourse.invalid%2F&inspect=1');
    await page.waitForFunction(()=>JSON.parse(document.querySelector('#canvas')?.getAttribute('data-viewer-inspection')||'null')?.objects===1,null,{timeout:60000});
    await command(page,'View Top');
    async function pick(y){const s=await page.evaluate(inspect),xy=projected(s,[0,y,0]),box=await page.locator('#canvas').boundingBox();await page.mouse.click(box.x+xy[0],box.y+xy[1]);}
    async function resolved(id){await page.waitForFunction(id=>JSON.parse(document.querySelector('#canvas')?.getAttribute('data-viewer-inspection')||'null')?.sheet_entity?.id===id,id,{timeout:15000});}
    await pick(500);await resolved(0);const first=await page.evaluate(inspect);
    assert.deepEqual(first.sheet_entity,{id:0,guid:'entity-0',name:'Wall A',kind:'wall'});
    await page.screenshot({path:path.join(out,'wall-'+width+'.png')});
    hold=true;await pick(-500);
    for(let n=0;n<150&&!release;n++)await page.waitForTimeout(20);
    assert(release,'Second entity reaches its delayed JSON range');
    await page.keyboard.press('Escape');release();await page.waitForTimeout(500);
    const cancelled=await page.evaluate(inspect);assert.equal(cancelled.sheet_entity,null,'Late reply must not restore cancelled selection');
    await page.screenshot({path:path.join(out,'cancelled-'+width+'.png')});
    await pick(-500);await resolved(1);const second=await page.evaluate(inspect);
    assert.deepEqual(second.sheet_entity,{id:1,guid:'entity-1',name:'Door B',kind:'door'});
    assert.equal(requests.filter(r=>r.file.endsWith('.shm')&&r.start===0).length,1,'Successful first lookup caches table header');
    assert.deepEqual(second.camera_pose,first.camera_pose);assert.deepEqual(errors,[]);
    await page.screenshot({path:path.join(out,'door-'+width+'.png')});
    results.push({width,first,cancelled,second,requests,errors,diagnostics:await page.evaluate(()=>viewerDiagnostics.read())});await page.close();
  }
  fs.writeFileSync(path.join(out,'sheet.json'),JSON.stringify({passed:true,scope:'Actual pointer selection, ranged side-table identity, cached header and cancelled delayed reply; synthetic two-entity fixture',results},null,2));
  } finally {await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
