const {chromium}=require('playwright');
const assert=require('node:assert/strict');
const fs=require('node:fs/promises');
const path=require('node:path');
const {PNG}=require('pngjs');
(async()=>{
 const browser=await chromium.launch({executablePath:'/usr/bin/google-chrome',headless:process.env.HEADLESS==='1',args:['--enable-unsafe-webgpu','--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE','--ozone-platform=x11']});
 const dpr=Number(process.env.VIEWER_DPR||1);
 const page=await browser.newPage({viewport:{width:1440,height:1000},deviceScaleFactor:dpr});
 const ui=()=>page.locator('canvas').getAttribute('data-viewer-ui').then(JSON.parse);
 const state=()=>page.locator('canvas').getAttribute('data-viewer-inspection').then(JSON.parse);
 const settle=()=>page.waitForTimeout(700);
 const control=async key=>{const c=(await ui()).controls.find(c=>c.key===key);assert(c,key);const [x,y,r,b]=c.rect;await page.mouse.click((x+r)/2,(y+b)/2);await settle();};
 const command=async text=>{await control('command/input');await page.keyboard.press('Control+a');await page.keyboard.type(text);await page.keyboard.press('Enter');await page.waitForFunction(text=>JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui'))?.history.at(-1)?.startsWith('> '+text+'\n'),text);await settle();};
 const ready=page.waitForEvent('console',{predicate:m=>m.text().includes('scene posted'),timeout:120000});
 const output=process.env.VIEWER_TEST_OUTPUT||path.resolve(__dirname,'../target/review/ambient-details');await fs.mkdir(output,{recursive:true});
 ready.catch(()=>{});
 await page.route('**/favicon.ico',route=>route.fulfill({status:204}));
 const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 try{
 await page.goto((process.env.VIEWER_URL||'http://127.0.0.1:8770')+'/view_mixed?inspect=1&nogrid=1');await ready;await settle();
 for(const [name,match] of [['primitives',/solids: BReps/],['plates',/plates \+ contact/],['floor',/floor model/]]){
  await command('Arctic Off');await command('Layers On');const row=(await ui()).rows.find(r=>match.test(r.label));assert(row,name);await control(row.key);assert((await state()).selected_group_count>0);await command('Fit');await command('Escape');await command('Layers Off');
  const off=await state();const plain=PNG.sync.read(await page.screenshot({path:output+'/'+name+'-off.png'}));
  await command('Arctic On');const on=await state();assert.deepEqual(on.mvp,off.mvp);const shaded=PNG.sync.read(await page.screenshot({path:output+'/'+name+'-on.png'}));
  assert.equal(off.ssao,false);assert.equal(on.ssao,true);
  if(name==='primitives') {
   for(const [label,rect,min,max] of [['cone',[325,630,435,740],50,1000],['torus',[520,560,630,660],100,2000]]) {
    let dark=0;
    for(let y=rect[1]*dpr;y<rect[3]*dpr;y++) for(let x=rect[0]*dpr;x<rect[2]*dpr;x++) {
     const at=(y*shaded.width+x)*4;
     if(plain.data[at]===255&&plain.data[at+1]===255&&plain.data[at+2]===255&&shaded.data[at]<shaded.data[0]-5) dark++;
    }
    dark/=dpr*dpr;
    assert(dark>min&&dark<max,label+' keeps a compact contact shadow: '+dark);
   }
  }
  if(name==='plates') {
   let count=0,variation=0;
   for(let y=565*dpr;y<725*dpr;y++) for(let x=200*dpr;x<600*dpr;x++) {
    const at=(y*shaded.width+x)*4,neighbors=[at-4,at+4,at-shaded.width*4,at+shaded.width*4];
    if([at,...neighbors].some(i=>plain.data[i]!==255||plain.data[i+1]!==255||plain.data[i+2]!==255)) continue;
    const residual=shaded.data[at]-neighbors.reduce((sum,i)=>sum+shaded.data[i],0)/4;
    variation+=residual*residual;count++;
   }
   assert(count>50000*dpr*dpr&&Math.sqrt(variation/count)<0.5,'plate contact shading stays smooth between its edges');
  }
  await fs.writeFile(output+'/'+name+'.json',JSON.stringify({off,on}));
 }
 assert.deepEqual(errors,[]);console.log('PASS '+output);
 }catch(error){console.error(await ui());await page.screenshot({path:output+'/failure.png'});throw error;}finally{await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
