const {chromium,devices}=require('playwright');
const fs=require('node:fs/promises');
const path=require('node:path');
const assert=require('node:assert/strict');
(async()=>{
 const samples=Number(process.env.AO_SAMPLES||4),dpr=Number(process.env.AO_DPR||1),phone=process.env.AO_PHONE==='1';
 const scenes=(process.env.AO_SCENES||'view_mixed,view_mixed_solids,view_meshes,view_lines,view_sheets,view_pointclouds,view_live').split(',');
 const root=path.resolve(process.env.AO_OUTPUT||'target/review/ambient-scenes',`${samples}x-${phone?'pixel7':'dpr'+dpr}`);await fs.mkdir(root,{recursive:true});
 const browser=await chromium.launch({executablePath:'/usr/bin/google-chrome',headless:false,args:['--enable-unsafe-webgpu','--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE','--ozone-platform=x11']});
 const reports=[];
 try {for(const scene of scenes){
  const context=await browser.newContext(phone?devices['Pixel 7']:{viewport:{width:1280,height:800},deviceScaleFactor:dpr});
  const page=await context.newPage(),errors=[],messages=[];
  page.on('pageerror',e=>errors.push(String(e)));
  page.on('console',m=>{messages.push(m.text());if(m.type()==='error')errors.push(m.text());});
  await page.route('**/favicon.ico',r=>r.fulfill({status:204}));
  const state=()=>page.locator('canvas').getAttribute('data-viewer-inspection').then(JSON.parse);
  const url=new URL(process.env.VIEWER_URL||'http://127.0.0.1:8770');
  url.searchParams.set('inspect','1');url.searchParams.set('nogrid','1');url.searchParams.set('msaa',String(samples));
  if(scene!=='local')url.searchParams.set('scene',`scenes/${scene}.yaml`);
  try {
   await page.goto(url.href);
   await page.waitForFunction(()=>document.querySelector('canvas')?.getAttribute('data-viewer-inspection'),null,{timeout:120000});
   const start=Date.now();while(!messages.some(m=>m.includes('scene posted'))&&Date.now()-start<120000)await page.waitForTimeout(200);
   assert(messages.some(m=>m.includes('scene posted')),`${scene} finished loading`);
   await page.waitForTimeout(600);await page.keyboard.press('Escape');
   const off=await state();assert(off.objects>0);assert.equal(off.samples,samples);
   await page.screenshot({path:path.join(root,scene+'-off.png')});
   await page.keyboard.press('g');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).ssao);
   await page.waitForTimeout(600);const on=await state();assert.deepEqual(on.mvp,off.mvp);
   await page.screenshot({path:path.join(root,scene+'-still.png')});
   const viewport=page.viewportSize();const x=viewport.width*.55,y=viewport.height*.4;
   await page.mouse.move(x,y);await page.mouse.down({button:'right'});await page.mouse.move(x+40,y+25,{steps:12});await page.waitForTimeout(100);
   const drag=await state();await page.screenshot({path:path.join(root,scene+'-drag.png')});
   await page.mouse.up({button:'right'});await page.waitForTimeout(100);
   const released=await state();assert.deepEqual(released.mvp,drag.mvp);assert(released.ssao);
   await page.screenshot({path:path.join(root,scene+'-release.png')});
   assert.deepEqual(errors,[]);
   const report={scene,samples,phone,dpr,off,on,drag,released,errors};reports.push(report);
   await fs.writeFile(path.join(root,scene+'.json'),JSON.stringify(report));console.log('PASS',scene,samples,phone?'Pixel7':dpr,on.canvas,on.objects);
  }catch(e){await page.screenshot({path:path.join(root,scene+'-failure.png')});await fs.writeFile(path.join(root,scene+'-failure.json'),JSON.stringify({error:String(e),errors,messages,state:await state()}));throw e;}
  finally{await context.close();}
 }}finally{await browser.close();await fs.writeFile(path.join(root,'results.json'),JSON.stringify(reports));}
})().catch(e=>{console.error(e);process.exitCode=1;});
