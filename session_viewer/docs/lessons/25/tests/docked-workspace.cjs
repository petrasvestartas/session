/** Docked layout, touch editing, cancellation and portable save/open through real browser UI. */
const assert=require('node:assert/strict');
const fs=require('node:fs/promises');
const path=require('node:path');
const {chromium}=require('playwright');
const ui=require('../docs/extensions/browser.cjs');
const root=path.resolve(__dirname,'..');
const out=process.env.VIEWER_TEST_OUTPUT||'/tmp/viewer-docked-workspace';
async function state(page){return JSON.parse(await page.locator('canvas').getAttribute('data-viewer-inspection'));}
async function settle(page){await page.waitForTimeout(200);await page.waitForFunction(()=>{const s=JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection'));return !s.pick_busy;});}
function project(s,p){const v=p.map((n,i)=>n-s.origin[i]);const c=[0,1,2,3].map(r=>s.mvp[r]*v[0]+s.mvp[r+4]*v[1]+s.mvp[r+8]*v[2]+s.mvp[r+12]);return [(c[0]/c[3]*.5+.5)*s.logical_canvas[0],(.5-c[1]/c[3]*.5)*s.logical_canvas[1]];}
async function touch(cdp,type,points){await cdp.send('Input.dispatchTouchEvent',{type,touchPoints:points.map(([x,y],id)=>({x,y,id,radiusX:8,radiusY:8,force:1}))});}
async function run(browser,width,height,dpr=width<700?2:1){
 const context=await browser.newContext({viewport:{width,height},hasTouch:true,deviceScaleFactor:dpr,acceptDownloads:true});
 const page=await context.newPage();const errors=[];page.on('pageerror',e=>errors.push(e.message));
 try{
 await page.route('**/view_local.yaml',r=>r.fulfill({contentType:'application/yaml',body:'name: Editing lessons\nitems:\n  - file: extension-nested.pb\n'}));
 await page.route('**/extension-nested.pb',r=>r.fulfill({path:path.join(root,'docs/extensions/nested.pb')}));
 await page.route('**/favicon.ico',r=>r.fulfill({status:204}));
 await page.goto(new URL('?data=off&inspect=1',process.env.VIEWER_URL||'http://127.0.0.1:8770/').href);
 await page.waitForFunction(()=>{const s=document.querySelector('canvas')?.getAttribute('data-viewer-inspection');const u=document.querySelector('canvas')?.getAttribute('data-viewer-ui');return s&&JSON.parse(s).objects===3&&u&&JSON.parse(u).controls.some(c=>c.key==='toolbar/Save');});
 let model=await ui.ui(page);const input=model.controls.find(c=>c.key==='command/input');assert(input.rect[1]>height*.75);assert(input.rect[2]>width-150);
 assert(model.controls.find(c=>c.key==='toolbar/Obj').rect[2]<70);
 if(width>=700){assert(model.controls.find(c=>c.key==='layers/close').rect[0]>width*.65);await ui.button(page,'layers/close','Close layers');}
 else assert.equal(model.layers_open,false,'phone starts with room for the scene');
 await ui.command(page,'line -100,130,0 110,130,0');
 await page.locator('canvas').focus();await page.keyboard.press('Escape');await page.keyboard.press('5');await page.keyboard.press('f');await page.mouse.move(width*.5,height*.4);await page.mouse.wheel(0,400);await settle(page);
 let s=await state(page);let pick=project(s,[0,130,0]);await page.mouse.click(...pick);await settle(page);s=await state(page);assert.notEqual(s.selected,null,'line selected');
 const cdp=await context.newCDPSession(page);
 const before=s.model.slice();let origin=s.widget[0];let from=project(s,[origin[0]+96*.9*s.widget[1],origin[1],origin[2]]);let to=[from[0]+32,from[1]];
 assert(from[0]>58&&from[0]<width-10&&from[1]<height-80,'handle is in the scene');
 await touch(cdp,'touchStart',[from]);await touch(cdp,'touchMove',[to]);await settle(page);await touch(cdp,'touchEnd',[]);await settle(page);
 s=await state(page);assert(Math.abs(s.model[12]-before[12])>1,'finger drag commits source placement');const moved=s.model.slice();const savedCenter=s.widget[0].slice();
 origin=s.widget[0];from=project(s,[origin[0]+96*.9*s.widget[1],origin[1],origin[2]]);to=[from[0]+20,from[1]];
 await touch(cdp,'touchStart',[from]);await touch(cdp,'touchMove',[to]);await settle(page);await touch(cdp,'touchCancel',[]);await settle(page);
 assert.deepEqual((await state(page)).model,moved,'cancel restores the placement');
 await touch(cdp,'touchStart',[from]);await touch(cdp,'touchMove',[to]);await touch(cdp,'touchStart',[to,[to[0]-30,to[1]+40]]);await touch(cdp,'touchEnd',[]);await settle(page);
 assert.deepEqual((await state(page)).model,moved,'a second finger cancels the edit without recording it');
 await page.screenshot({path:path.join(out,`gumball-${width}.png`)});
 const download=page.waitForEvent('download');await ui.button(page,'toolbar/Save','Save');const file=await download;const saved=path.join(out,`workspace-${width}.session`);await file.saveAs(saved);assert((await fs.stat(saved)).size>100);
 const chooser=page.waitForEvent('filechooser');await ui.button(page,'toolbar/Open','Open');await (await chooser).setFiles(saved);await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).objects===4);await settle(page);assert.equal((await state(page)).objects,4,'all source documents restored');
 await page.locator('canvas').focus();await page.keyboard.press('5');await page.keyboard.press('f');await page.mouse.move(width*.5,height*.4);await page.mouse.wheel(0,300);await settle(page);
 s=await state(page);await page.mouse.click(...project(s,savedCenter));await settle(page);assert.notEqual((await state(page)).selected,null,'saved edited line can be selected again');await page.keyboard.press('7');await page.keyboard.press('t');await settle(page);
 if(width>=700)await ui.button(page,'toolbar/Layer','layers');
 await ui.typeCommand(page,'point 0,0,0');await page.screenshot({path:path.join(out,`workspace-${width}.png`)});
 assert.deepEqual(errors,[]);console.log(`PASS ${width}x${height}: docks, touch drag/cancel, complete save/open`);
 }catch(e){await page.screenshot({path:path.join(out,`failure-${width}.png`)});throw e;}finally{await context.close();}
}
(async()=>{await fs.mkdir(out,{recursive:true});const browser=await chromium.launch({executablePath:process.env.CHROME_BIN||'/usr/bin/google-chrome',headless:false,args:JSON.parse(process.env.VIEWER_CHROME_ARGS||'[]')});try{await run(browser,1200,800);await run(browser,390,844);await run(browser,1200,800,2);}finally{await browser.close();}})().catch(e=>{console.error(e);process.exitCode=1});
