/** Real Split command, cutter picking, cancellation, source topology, undo and persistence. */
const assert=require('node:assert/strict');
const {chromium}=require('playwright');
const ui=require('../docs/extensions/browser.cjs');
async function state(page){return JSON.parse(await page.locator('canvas').getAttribute('data-viewer-inspection'));}
async function wait(page,fn){await page.waitForFunction(source=>{const raw=document.querySelector('canvas')?.getAttribute('data-viewer-inspection');return raw&&Function('s',`return (${source})(s)`)(JSON.parse(raw));},fn.toString());}
function project(s,p){const v=p.map((n,i)=>n-s.origin[i]);const c=[0,1,2,3].map(r=>s.mvp[r]*v[0]+s.mvp[r+4]*v[1]+s.mvp[r+8]*v[2]+s.mvp[r+12]);return [(c[0]/c[3]*.5+.5)*s.logical_canvas[0],(.5-c[1]/c[3]*.5)*s.logical_canvas[1]];}
async function settle(page){await page.waitForTimeout(160);await wait(page,s=>!s.pick_busy);}
async function pick(page,p,face=false,phone=false){await page.locator('canvas').focus();if(face)await ui.button(page,'toolbar/Face','faces');if(face){await page.keyboard.down('Control');await page.keyboard.down('Shift');}if(phone)await page.touchscreen.tap(...project(await state(page),p));else await page.mouse.click(...project(await state(page),p));if(face){await page.keyboard.up('Shift');await page.keyboard.up('Control');}await settle(page);}
async function begin(page){await ui.button(page,'toolbar/Split','Split');await wait(page,s=>s.split!==null);}
async function run(browser,fixture,phone=false,nurbs=false){
 const context=await browser.newContext({viewport:phone?{width:390,height:844}:{width:1200,height:800},hasTouch:phone,deviceScaleFactor:phone?2:1,acceptDownloads:true});const page=await context.newPage();const errors=[];page.on('pageerror',e=>errors.push(e.message));
 try {
 await page.route('**/view_local.yaml',r=>r.fulfill({contentType:'application/yaml',body:fixture?'name: Face split\nitems:\n  - file: fixture.pb\n    xform: [1000,0,0,0, 0,1000,0,0, 0,0,1000,0, 0,0,0,1]\n':'name: Curve split\nitems: []\n'}));
 if(fixture)await page.route('**/fixture.pb',r=>r.fulfill({path:process.env.VIEWER_INTERACTION_FIXTURE||'/tmp/viewer-interaction-current.pb'}));
 await page.goto(new URL('?data=off&inspect=1',process.env.VIEWER_URL||'http://127.0.0.1:8770/').href);await wait(page,s=>typeof s.objects==='number');if((await ui.ui(page)).layers_open)await ui.button(page,'layers/close','Close layers');
 if(fixture){await wait(page,s=>s.objects===8);await ui.command(page,'Line -3000,-5000,200 -3000,-1000,200');}
 else {await ui.command(page,nurbs?'Curve -100,40,0 0,100,0 100,40,0':'Line -100,40,0 100,40,0');await ui.command(page,'Line 0,-60,0 0,140,0');}
 await page.mouse.click(phone?365:1175,60);await settle(page);await page.locator('canvas').focus();await page.keyboard.press('Escape');await settle(page);await page.keyboard.press('5');await settle(page);await ui.command(page,'Fit');await page.mouse.move(phone?230:500,300);await page.mouse.wheel(0,400);await settle(page);
 const target=fixture?[-3300,-3650,200]:(nurbs?[-60,59.2,0]:[-60,40,0]);const cutter=fixture?[-3000,-4800,200]:[0,100,0];
 await pick(page,target,fixture,phone);let before=await state(page);assert.notEqual(before.selected,null);if(fixture)assert(before.selection.Face);
 await begin(page);await pick(page,cutter,false,phone);await wait(page,s=>s.split?.[2].length===1);await page.keyboard.press('Escape');await settle(page);assert.equal((await state(page)).split,null);assert.equal((await state(page)).objects,before.objects);
 await pick(page,target,fixture,phone);await begin(page);await pick(page,cutter,false,phone);await wait(page,s=>s.split?.[2].length===1);if(phone){const control=(await ui.ui(page)).controls.find(c=>c.key==='toolbar/Split');const [a,b,c,d]=control.rect;await page.touchscreen.tap((a+c)/2,(b+d)/2);}else await page.keyboard.press('Enter');await settle(page);
 if(fixture)await wait(page,s=>s.source_faces===7);else await wait(page,s=>s.objects===3);
 const after=await state(page);assert.equal(after.objects,before.objects+(fixture?0:1));
 await ui.command(page,'Undo');await settle(page);assert.equal((await state(page)).objects,before.objects);
 await ui.command(page,'Redo');await settle(page);assert.equal((await state(page)).objects,after.objects);
 const download=page.waitForEvent('download');await ui.button(page,'toolbar/Save','Save');const file=`/tmp/viewer-split-${fixture?'face':phone?'phone':nurbs?'nurbs':'curve'}.session`;await (await download).saveAs(file);
 const chooser=page.waitForEvent('filechooser');await ui.button(page,'toolbar/Open','Open');await (await chooser).setFiles(file);await page.waitForFunction(()=>document.querySelector('#viewer-status')?.textContent==='Session opened');await settle(page);assert.equal((await state(page)).objects,after.objects);
 if(fixture){await pick(page,target,true);await ui.command(page,'Fit');await pick(page,[-3400,-3400,200],true);assert((await state(page)).selection.Face);assert.equal((await state(page)).source_faces,7);}await page.locator('canvas').focus();await page.keyboard.press('7');await settle(page);await ui.command(page,'Fit');await page.locator('canvas').focus();await page.keyboard.press('t');await page.mouse.move(phone?230:500,300);await page.mouse.wheel(0,200);await settle(page);await page.screenshot({path:`/tmp/viewer-split-${fixture?'face':phone?'phone':nurbs?'nurbs':'curve'}.png`});
 assert.deepEqual(errors,[]);console.log(`PASS ${fixture?'joined BRep face':phone?'phone line':nurbs?'NURBS curve':'line'} split: cutter picking, cancel, retained regions, Undo/Redo and Save/Open`);
 }catch(e){await page.screenshot({path:'/tmp/viewer-split-failure.png'});console.log(await state(page).catch(()=>null),await ui.ui(page));throw e;}finally{await context.close();}
}
(async()=>{const browser=await chromium.launch({executablePath:'/usr/bin/google-chrome',headless:false,args:JSON.parse(process.env.VIEWER_CHROME_ARGS||'[]')});try{await run(browser,false);await run(browser,true);await run(browser,false,true);await run(browser,false,false,true);}finally{await browser.close();}})().catch(e=>{console.error(e);process.exitCode=1});
