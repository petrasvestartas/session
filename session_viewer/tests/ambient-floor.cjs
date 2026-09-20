const {chromium}=require('playwright');
const fs=require('node:fs/promises');
const assert=require('node:assert/strict');
const {PNG}=require('pngjs');
const path=require('node:path');
const output=path.resolve(__dirname,'../target/review/ambient-floor');
(async()=>{
 const browser=await chromium.launch({executablePath:'/usr/bin/google-chrome',headless:false,args:['--enable-unsafe-webgpu','--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE','--ozone-platform=x11']});
 await fs.mkdir(output,{recursive:true});
 const page=await browser.newPage({viewport:{width:1440,height:1000}});
 let release;
 const gate=new Promise(resolve=>release=resolve);
 const posted=new Promise(resolve=>page.on('console',message=>{if(message.text().includes('scene posted')) resolve();}));
 await page.route('**/view_sheet_treppenhaus.pb',async route=>{await gate;await route.continue();});
 const state=()=>page.locator('canvas').getAttribute('data-viewer-inspection').then(JSON.parse);
 const ui=()=>page.locator('canvas').getAttribute('data-viewer-ui').then(JSON.parse);
 const settle=()=>page.waitForTimeout(800);
 const control=async key=>{const c=(await ui()).controls.find(c=>c.key===key);assert(c,key);const [x,y,r,b]=c.rect;await page.mouse.click((x+r)/2,(y+b)/2);await settle();};
 const command=async text=>{await control('command/input');await page.keyboard.press('Control+a');await page.keyboard.type(text);await page.keyboard.press('Enter');await page.waitForFunction(text=>{const u=JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui'));return text==='SSAO'?u.command==='SSAO ':u.history.at(-1)?.startsWith('> '+text+'\n');},text,{timeout:30000});await settle();};
 try{
 await page.goto(new URL('/view_mixed?inspect=1&nogrid=1',process.env.VIEWER_URL||'http://127.0.0.1:8770/').href);
 await page.waitForFunction(()=>document.querySelector('canvas')?.getAttribute('data-viewer-inspection'),null,{timeout:120000});
 await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).objects>107000,null,{timeout:120000});
 await page.waitForTimeout(3000);
 await settle();
 await command('Layers On');
 const rows=(await ui()).rows;
 const floor=rows.find(r=>/floor model/i.test(r.label));assert(floor);
 await control(floor.key);assert.equal((await state()).selected_group_count,491);await command('Fit');await command('Escape');await command('Layers Off');
 const off=await state();const plain=PNG.sync.read(await page.screenshot({path:path.join(output,'off.png')}));
 await command('SSAO On');const on=await state();const shaded=PNG.sync.read(await page.screenshot({path:path.join(output,'on.png')}));

 assert.deepEqual(on.mvp,off.mvp);
 release();
 await posted;await settle();
 assert.deepEqual((await state()).mvp,off.mvp,'late scene loading preserves the fitted floor view');
 for(const option of ['Off','On','Off','On']) {
  await command('SSAO');await control('command/option/'+option);
  assert.equal((await state()).ssao,option==='On');
  assert.deepEqual((await state()).mvp,off.mvp,'clicking '+option+' preserves camera');
 }
 await page.keyboard.press('Escape');await settle();
 for(const key of ['g','g']) {await page.keyboard.press(key);await settle();assert.deepEqual((await state()).mvp,off.mvp,'G preserves camera');}

 let count=0,variation=0,darkened=0;
 // Exposed ground around the right column in this fixed viewport and fitted camera.
 for(let y=685;y<750;y++) for(let x=1015;x<1180;x++) {
  const at=(y*shaded.width+x)*4;
  const neighbors=[at-16,at+16,at-shaded.width*16,at+shaded.width*16];
  if([at,...neighbors].some(i=>plain.data[i]!==255||plain.data[i+1]!==255||plain.data[i+2]!==255)) continue;
  const residual=shaded.data[at]-neighbors.reduce((sum,i)=>sum+shaded.data[i],0)/4;
  variation+=residual*residual;count++;
  if(shaded.data[at]<shaded.data[0]-5) darkened++;
 }
 const roughness=Math.sqrt(variation/count);
 assert(count>9000&&darkened>200,'the sampled area contains ground contact shading');
 assert(roughness<0.8,'ground shadow contains blotchy sampling noise: '+roughness);
 await fs.writeFile(path.join(output,'inspection.json'),JSON.stringify({off,on,roughness},null,2));
 console.log('PASS view_mixed floor: smooth ground shadows and unchanged camera; roughness='+roughness);

 }catch(error){console.error(await ui());throw error;}finally{release();await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
