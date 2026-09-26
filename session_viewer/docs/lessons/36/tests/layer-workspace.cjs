/** One layer tree; descendant visibility, selection locks, colors and portable persistence. */
const assert=require('node:assert/strict');
const fs=require('node:fs/promises');
const path=require('node:path');
const {chromium}=require('playwright');
const ui=require('../docs/extensions/browser.cjs');
const root=path.resolve(__dirname,'..');
const out='/tmp/viewer-layer-workspace';
async function state(page){return JSON.parse(await page.locator('canvas').getAttribute('data-viewer-inspection'));}
async function wait(page,predicate){await page.waitForFunction(source=>{const raw=document.querySelector('canvas')?.getAttribute('data-viewer-inspection');return raw&&Function('s',`return (${source})(s)`)(JSON.parse(raw));},predicate.toString());}
(async()=>{
 await fs.mkdir(out,{recursive:true});
 const browser=await chromium.launch({executablePath:'/usr/bin/google-chrome',headless:false,args:JSON.parse(process.env.VIEWER_CHROME_ARGS||'[]')});
 const context=await browser.newContext({viewport:{width:1200,height:800},acceptDownloads:true});const page=await context.newPage();const errors=[];page.on('pageerror',e=>errors.push(e.message));
 try {
 await page.route('**/view_local.yaml',r=>r.fulfill({contentType:'application/yaml',body:'name: Editing lessons\nitems:\n  - file: extension-nested.pb\n'}));
 await page.route('**/extension-nested.pb',r=>r.fulfill({path:path.join(root,'docs/extensions/nested.pb')}));
 await page.goto(new URL('?data=off&inspect=1',process.env.VIEWER_URL||'http://127.0.0.1:8770/').href);await wait(page,s=>s.objects===3);
 for(const name of ['Editing lessons','Assembly','Nested']) await ui.button(page,'open/',name);
 let controls=(await ui.ui(page)).controls;
 assert(!controls.some(c=>c.key.startsWith('doc/')||c.key.startsWith('kind/')||c.label.includes('edge: joint')),'one tree, without duplicate summary or graph lists');
 await ui.button(page,'select/','Select Nested');await wait(page,s=>s.selected_group_count===2);
 await ui.button(page,'lock/','Lock Nested');await wait(page,s=>s.locked_count===2&&s.selected_group_count===0&&s.selected===null);
 await ui.button(page,'select/','Select Nested');assert.equal((await state(page)).selected_group_count,0);
 await ui.button(page,'hide/','Hide Nested');await wait(page,s=>s.hidden_count===2);
 await ui.button(page,'hide/','Show Nested');await wait(page,s=>s.hidden_count===0);
 await ui.button(page,'color/','Color Nested');await ui.button(page,'color/','Green');await wait(page,s=>s.color_count===2);
 await page.locator('canvas').focus();await page.keyboard.press('7');await page.keyboard.press('f');await page.mouse.move(500,300);await page.mouse.wheel(0,900);await page.waitForTimeout(200);
 await ui.button(page,'select/','Select Placed polyline');await page.locator('canvas').focus();await page.keyboard.press('t');await page.waitForTimeout(150);
 await page.screenshot({path:path.join(out,'layers.png')});
 const download=page.waitForEvent('download');await ui.button(page,'toolbar/Save','Save');const file=path.join(out,'layers.session');await (await download).saveAs(file);
 const chooser=page.waitForEvent('filechooser');await ui.button(page,'toolbar/Open','Open');await (await chooser).setFiles(file);await page.waitForFunction(()=>document.querySelector('#viewer-status')?.textContent==='Session opened');await wait(page,s=>s.objects===3&&s.locked_count===2&&s.color_count===2);
 for(const name of ['Editing lessons','Assembly','Nested']) await ui.button(page,'open/',name);
 await ui.button(page,'lock/','Unlock Nested');await wait(page,s=>s.locked_count===0);
 await ui.button(page,'select/','Select Nested');await wait(page,s=>s.selected_group_count===2);
 assert.deepEqual(errors,[]);console.log('PASS one hierarchy, child visibility/lock/color, save/open and unlock selection');
 }catch(e){await page.screenshot({path:path.join(out,'failure.png')});console.error(await ui.ui(page));throw e;}finally{await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
