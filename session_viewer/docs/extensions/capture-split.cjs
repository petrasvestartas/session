/** Capture the full joined-face result from the downloadable tutorial fixture. */
const assert=require('node:assert/strict');
const fs=require('node:fs/promises');
const path=require('node:path');
const {chromium}=require('playwright');
const ui=require('./browser.cjs');
const state=async page=>JSON.parse(await page.locator('canvas').getAttribute('data-viewer-inspection'));
async function wait(page,fn){await page.waitForFunction(source=>{const s=document.querySelector('canvas')?.getAttribute('data-viewer-inspection');return s&&Function('s',`return (${source})(s)`)(JSON.parse(s));},fn.toString());}
function project(s,p){const v=p.map((n,i)=>n-s.origin[i]);const c=[0,1,2,3].map(r=>s.mvp[r]*v[0]+s.mvp[r+4]*v[1]+s.mvp[r+8]*v[2]+s.mvp[r+12]);return [(c[0]/c[3]*.5+.5)*s.logical_canvas[0],(.5-c[1]/c[3]*.5)*s.logical_canvas[1]];}
async function settle(page){await page.waitForTimeout(160);await wait(page,s=>!s.pick_busy);}
async function pick(page,p,face=false){await page.locator('canvas').focus();if(face){await page.keyboard.down('Control');await page.keyboard.down('Shift');}await page.mouse.click(...project(await state(page),p));if(face){await page.keyboard.up('Shift');await page.keyboard.up('Control');}await settle(page);}
(async()=>{
 const browser=await chromium.launch({executablePath:'/usr/bin/google-chrome',headless:false,args:JSON.parse(process.env.VIEWER_CHROME_ARGS||'[]')});
 const phone=process.env.VIEWER_PHONE==='1';const suffix=phone?'-phone':'';
 const page=await browser.newPage({viewport:phone?{width:390,height:844}:{width:1400,height:900},hasTouch:phone,deviceScaleFactor:phone?2:1});const errors=[];page.on('pageerror',e=>errors.push(e.message));
 try {
 await page.route('**/view_local.yaml',r=>r.fulfill({contentType:'application/yaml',body:'name: Split tutorial\nitems:\n  - file: extension-split.pb\n'}));
 await page.route('**/extension-split.pb',r=>r.fulfill({path:path.join(__dirname,'split.pb')}));
 await page.goto(new URL('?data=off&inspect=1',process.env.VIEWER_URL||'http://127.0.0.1:8770/').href);await wait(page,s=>s.objects>=2);
 if((await ui.ui(page)).layers_open)await ui.button(page,'layers/close','Close layers');await page.mouse.click(phone?365:1375,60);await settle(page);await page.locator('canvas').focus();await page.keyboard.press('5');await settle(page);await pick(page,[0,0,0]);if((await state(page)).selected!==null && (await state(page)).source_faces===null)await ui.command(page,'Hide');
 await ui.command(page,'Line 0,-150,20 0,150,20');await ui.command(page,'Fit');await page.mouse.move(phone?230:600,300);await page.mouse.wheel(0,200);await settle(page);
 await pick(page,[-50,-50,20],true);assert((await state(page)).selection.Face);
 await ui.button(page,'toolbar/Split','Split');await pick(page,[0,-130,20]);await wait(page,s=>s.split?.[2].length===1);await page.keyboard.press('Enter');await wait(page,s=>s.source_faces===7);
 await pick(page,[-50,-50,20],true);assert((await state(page)).selection.Face);await page.keyboard.press('t');await page.keyboard.press('7');await settle(page);await page.keyboard.press('f');await settle(page);await page.mouse.move(phone?230:600,300);await page.mouse.wheel(0,300);await settle(page);
 if(!phone)await ui.button(page,'toolbar/Layer','layers');
 for(const name of (phone?[]:['Split tutorial','Created'])){if((await ui.ui(page)).controls.some(c=>c.key.startsWith('open/')&&c.label.includes(name)))await ui.button(page,'open/',name);}
 await settle(page);assert.deepEqual(errors,[]);await page.screenshot({path:`/tmp/viewer-split-tutorial${suffix}.png`});await fs.writeFile(`/tmp/viewer-split-tutorial${suffix}.json`,JSON.stringify({state:await state(page),ui:await ui.ui(page),errors},null,2));console.log('PASS full-viewer joined-face screenshot; seven retained source faces');
 }catch(e){await page.screenshot({path:'/tmp/viewer-split-tutorial-failure.png'});console.log(await state(page),await ui.ui(page));throw e;}finally{await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
