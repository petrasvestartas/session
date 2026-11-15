const fs=require('node:fs'),path=require('node:path'),assert=require('node:assert/strict'),crypto=require('node:crypto');
const {chromium}=require(process.env.COURSE_PLAYWRIGHT || 'playwright');
const {PNG}=require(process.env.COURSE_PNG || 'pngjs');
const out=path.resolve(process.argv[3] || '../session_viewer/target/completion-browser');
const url=process.argv[2] || 'http://127.0.0.1:8780/';
assert(!fs.existsSync(out), 'Choose a new evidence directory');
fs.mkdirSync(out, {recursive:true});
const args=process.env.COURSE_BROWSER_ARGS ? JSON.parse(process.env.COURSE_BROWSER_ARGS) : process.platform==='win32'
 ? ['--enable-unsafe-webgpu','--ignore-gpu-blocklist','--use-angle=d3d11']
 : ['--enable-unsafe-webgpu','--enable-unsafe-swiftshader','--use-angle=swiftshader','--enable-features=Vulkan','--use-vulkan=swiftshader'];
assert(Array.isArray(args) && args.every(arg=>typeof arg==='string'));
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
(async()=>{
 const progress=s=>fs.appendFileSync(path.join(out,'progress.log'),new Date().toISOString()+' '+s+'\n');progress('launch');
 const browser=await chromium.launch({executablePath:process.env.CHROMIUM || (process.platform==='win32' ? 'C:/Program Files/Google/Chrome/Application/chrome.exe' : '/usr/bin/chromium'),headless:true,args});
 progress('launched');let bytes=0;const cases=[];
 try{
 for(const viewport of [{width:1280,height:800},{width:390,height:844}]){
  progress('page '+JSON.stringify(viewport));const page=await browser.newPage({viewport}),errors=[];progress('page ready');page.on('pageerror',e=>errors.push(String(e)));const consoleLog=[];page.on('console',m=>{consoleLog.push({type:m.type(),text:m.text()});fs.writeFileSync(path.join(out,'console.json'),JSON.stringify(consoleLog,null,2));});
  page.on('response',r=>{if(r.status()>=400&&!r.url().endsWith('favicon.ico'))errors.push(r.status()+' '+r.url());});
  page.setDefaultTimeout(10000);const cdp=await page.context().newCDPSession(page);cdp.on('Debugger.paused',event=>{fs.writeFileSync(path.join(out,'paused.json'),JSON.stringify(event,null,2));cdp.send('Debugger.resume').catch(()=>{});});const record={viewport,frames:[],errors};cases.push(record);
  async function capture(name,inkRequired=true){
   await page.waitForTimeout(1200);
   let image=PNG.sync.read(await page.screenshot());
   for(let i=0;i<15;i++){
    await page.waitForTimeout(150);const next=PNG.sync.read(await page.screenshot());
    if(hash(image.data)===hash(next.data)){image=next;break;}
    image=next;if(i===14)throw Error('Image did not settle: '+name);
   }
   assert.equal(image.width,viewport.width);assert.equal(image.height,viewport.height);
   let ink=0;for(let i=0;i<image.data.length;i+=4)if(Math.min(...image.data.subarray(i,i+3))<225)ink++;
   if(inkRequired)assert(ink>500,name+' has no visible content');else assert.equal(ink,0,'First frame must be white');
   const file=viewport.width+'-'+name+'.png',png=PNG.sync.write(image);bytes+=png.length;assert(bytes<32*1024**2);
   fs.writeFileSync(path.join(out,file),png);const frame={name,file,ink,pixels:hash(image.data),sha256:hash(png)};record.frames.push(frame);return frame;
  }
  async function background(value){const pixels=PNG.sync.read(await page.screenshot()).data;assert.deepEqual([...pixels.subarray(0,3)],[value,value,value],'Theme must cover the whole canvas, including behind the heading');}
  async function mode(name){progress('mode '+name);await page.goto(url+'?tutorial='+name,{waitUntil:'commit'});try{await page.waitForFunction(()=>document.querySelector('canvas')?.dataset.coursePresented==='true',{},{timeout:25000});}catch(e){fs.writeFileSync(path.join(out,'startup-failure.json'),JSON.stringify({error:String(e),consoleLog}));await cdp.send('Debugger.pause');record.diagnostic={consoleLog,html:await page.content(),gpu:await page.evaluate(()=>({secure:isSecureContext,available:!!navigator.gpu}))};await page.screenshot({path:path.join(out,'failure.png')});throw e;}assert.equal(await page.locator('canvas').getAttribute('data-course-error'),null);const size=await page.locator('canvas').evaluate(e=>({w:e.width,h:e.height,r:e.getBoundingClientRect().toJSON(),dpr:devicePixelRatio}));assert.equal(size.w,Math.round(size.r.width*size.dpr));assert.equal(size.h,Math.round(size.r.height*size.dpr));}
  await mode('frame');await capture('frame',false);await page.setViewportSize({width:viewport.width-40,height:viewport.height-40});await page.waitForFunction(()=>{const e=document.querySelector('canvas');return e.width===Math.round(innerWidth*devicePixelRatio)&&e.height===Math.round(innerHeight*devicePixelRatio);});await page.setViewportSize(viewport);await page.waitForFunction(()=>{const e=document.querySelector('canvas');return e.width===Math.round(innerWidth*devicePixelRatio)&&e.height===Math.round(innerHeight*devicePixelRatio);});await capture('frame-resized',false);
  await mode('painting');const painted=await capture('painting');assert(painted.ink<viewport.width*viewport.height/2,'Painting must retain a white background');
  await mode('mathematics');const math=[await capture('math-1')];await background(255);
  for(let n=2;n<=4;n++){await page.keyboard.press('ArrowRight');math.push(await capture('math-'+n));assert.notEqual(math[n-1].pixels,math[n-2].pixels);}
  await page.keyboard.press('ArrowLeft');assert.equal((await capture('math-back')).pixels,math[2].pixels);
  await page.keyboard.press('t');assert.notEqual((await capture('math-dark')).pixels,math[2].pixels);await background(0);
  await page.keyboard.press('t');assert.equal((await capture('math-light')).pixels,math[2].pixels);
  await page.keyboard.press('ArrowUp');assert.equal((await capture('math-slide-up')).pixels,math[0].pixels);
  await page.keyboard.press('ArrowDown');assert.equal((await capture('math-slide-down')).pixels,math[2].pixels);
  await mode('dock');const initial=await capture('dock');
  async function control(key){await page.waitForFunction(k=>JSON.parse(document.querySelector('canvas')?.dataset.courseControls||'[]').some(c=>c.key===k),key);const r=await page.locator('canvas').evaluate((e,k)=>JSON.parse(e.dataset.courseControls).find(c=>c.key===k).rect,key);await page.mouse.click((r[0]+r[2])/2,(r[1]+r[3])/2);}
  async function command(text){await control('command/input');await page.waitForTimeout(100);await page.keyboard.press('Control+a');await page.waitForTimeout(100);await page.keyboard.type(text,{delay:40});await page.waitForTimeout(100);await page.keyboard.press('Enter');}
  await command('Hello');assert.notEqual((await capture('dock-hello')).pixels,initial.pixels);
  await command('Color');await capture('dock-options');await control('command/option/Blue');await capture('dock-blue');
  // Assert the authored circle changed to blue, rather than merely accepting a new screenshot hash.
  const png=PNG.sync.read(await page.screenshot()),offset=((viewport.height/2-45)*viewport.width+viewport.width/2)*4;
  assert(png.data[offset+2]>png.data[offset]+80,'Blue option must change circle pixels');
  await command('Clear');await capture('dock-clear');assert.deepEqual(errors,[]);
  if(process.argv.includes('--camera')){
   await mode('camera');let original=await capture('camera');
   await page.keyboard.press('ArrowRight');assert.notEqual((await capture('camera-orbit')).pixels,original.pixels);
   await page.keyboard.press('Home');const home=await capture('camera-home');assert.notEqual(home.pixels,original.pixels);original=home;
   await page.keyboard.press('p');assert.notEqual((await capture('camera-projection')).pixels,original.pixels);
   await page.keyboard.press('p');assert.equal((await capture('camera-projection-back')).pixels,original.pixels);
   await page.mouse.move(viewport.width/2,viewport.height/2);await page.mouse.wheel(0,100);
   assert.notEqual((await capture('camera-zoom')).pixels,original.pixels);
   await page.keyboard.press('f');assert.equal((await capture('camera-fit')).pixels,original.pixels);
   await page.mouse.down();await page.mouse.move(viewport.width/2+50,viewport.height/2+30,{steps:5});await page.mouse.up();
   assert.notEqual((await capture('camera-drag')).pixels,original.pixels);
   await page.keyboard.press('Home');assert.equal((await capture('camera-drag-home')).pixels,original.pixels);
  }
  if(process.argv.includes('--rows')){
   await mode('rows');const initial=await capture('rows');
   const pixel=async(x,y)=>{const image=PNG.sync.read(await page.screenshot());const at=(Math.round(y)*image.width+Math.round(x))*4;return [...image.data.slice(at,at+3)];};
   const fit=Math.min(viewport.width/500,1),left=viewport.width/2-120*fit,right=viewport.width/2+120*fit,y=viewport.height/2;
   const pink=await pixel(left,y),blue=await pixel(right,y);assert(pink[0]>pink[2]+80);assert(blue[2]>blue[0]+80);
   await page.keyboard.press('ArrowRight');assert.notEqual((await capture('rows-move')).pixels,initial.pixels);assert.deepEqual(await pixel(right,y),blue);
   await page.keyboard.press('ArrowLeft');assert.equal((await capture('rows-move-back')).pixels,initial.pixels);
   await page.keyboard.press('h');await capture('rows-hide');assert.deepEqual(await pixel(left,y),[255,255,255]);assert.deepEqual(await pixel(right,y),blue);
   await page.keyboard.press('h');assert.equal((await capture('rows-show')).pixels,initial.pixels);
   await page.keyboard.press('r');await capture('rows-reveal');assert.deepEqual(await pixel(left+35*fit,y),[255,255,255]);assert.deepEqual(await pixel(left,y),pink);
   await page.keyboard.press('r');assert.equal((await capture('rows-reveal-back')).pixels,initial.pixels);
   await page.keyboard.press('c');await capture('rows-color');const tint=await pixel(left,y);assert(tint[2]>tint[0]+80);
   await page.keyboard.press('c');assert.equal((await capture('rows-color-back')).pixels,initial.pixels);assert.deepEqual(errors,[]);
  }
  if(process.argv.includes('--geometry')){
   await mode('geometry');const initial=await capture('geometry');
   for(const [key,name] of [['r','reveal'],['w','lines'],['o','faces'],['p','projection']]){
    await page.keyboard.press(key);assert.notEqual((await capture('geometry-'+name)).pixels,initial.pixels);
    await page.keyboard.press(key);assert.equal((await capture('geometry-'+name+'-back')).pixels,initial.pixels);
   }
   await page.keyboard.press('ArrowRight');assert.notEqual((await capture('geometry-orbit')).pixels,initial.pixels);
   await page.keyboard.press('ArrowLeft');assert.equal((await capture('geometry-orbit-back')).pixels,initial.pixels);
   assert.deepEqual(errors,[]);
  }
  record.adapter=await page.evaluate(async()=>{const a=await navigator.gpu.requestAdapter();return {...a.info,vendor:a.info.vendor,architecture:a.info.architecture,description:a.info.description};});await page.close();
 }
 fs.writeFileSync(path.join(out,'result.json'),JSON.stringify({scope:'Cumulative candidates (camera/rows/geometry only when explicitly requested); actual adapter recorded per viewport; upright emulation is not physical-phone acceptance',browser:browser.version(),args,checkerSha256:hash(fs.readFileSync(__filename)),cases},null,2));
 }catch(e){fs.writeFileSync(path.join(out,'failure.json'),JSON.stringify({error:String(e.stack),cases},null,2));throw e;}finally{await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
