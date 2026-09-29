const {chromium}=require('playwright');
const fs=require('node:fs/promises');
const assert=require('node:assert/strict');
(async()=>{
 const output=process.env.AO_OUTPUT||'target/review/ambient-motion';await fs.mkdir(output,{recursive:true});
 const browser=await chromium.launch({executablePath:'/usr/bin/google-chrome',headless:false,args:['--enable-unsafe-webgpu','--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE','--ozone-platform=x11']});
 const page=await browser.newPage({viewport:{width:1280,height:800}}),errors=[];
 page.on('pageerror',e=>errors.push(String(e)));
 page.on('console',m=>{if(m.type()==='error')errors.push(m.text());});
 await page.route('**/favicon.ico',r=>r.fulfill({status:204}));
 const state=()=>page.locator('canvas').getAttribute('data-viewer-inspection').then(JSON.parse);
 try{
  await page.goto((process.env.VIEWER_URL||'http://127.0.0.1:8770')+'/view_live?inspect=1&nogrid=1&msaa=4');
  await page.waitForFunction(()=>{const s=document.querySelector('canvas')?.getAttribute('data-viewer-inspection');return s&&JSON.parse(s).objects===4;},null,{timeout:90000});
  await page.waitForTimeout(1000);await page.keyboard.press('Escape');await page.keyboard.press('g');await page.waitForTimeout(750);
  assert((await state()).ssao);
  const frames=[];await page.mouse.move(700,350);await page.mouse.down({button:'right'});
  for(let i=0;i<32;i++){
   await page.mouse.move(700+i*2,350);await page.waitForTimeout(50);
   frames.push(await state());await page.screenshot({path:output+'/'+String(i).padStart(2,'0')+'.png'});
  }
  await page.mouse.up({button:'right'});await page.waitForTimeout(300);
  assert.deepEqual((await state()).mvp,frames.at(-1).mvp);
  await page.screenshot({path:output+'/released.png'});
  await fs.writeFile(output+'/frames.json',JSON.stringify(frames));assert.deepEqual(errors,[]);
  console.log('PASS rotating view_live',output);
 }finally{await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
