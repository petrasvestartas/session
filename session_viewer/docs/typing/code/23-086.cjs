/** Keyboard commands, recursive selection and one gumball for the selected set. */
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const {chromium} = require('playwright');
const root = path.resolve(__dirname,'..');
const out = '/tmp/viewer-command-workspace';
async function ui(page) { return JSON.parse(await page.locator('canvas').getAttribute('data-viewer-ui')); }
async function state(page) { return JSON.parse(await page.locator('canvas').getAttribute('data-viewer-inspection')); }
async function settle(page) { await page.waitForTimeout(750); await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)))); await page.waitForFunction(()=>!JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).pick_busy); }
async function control(page,key) { const c=(await ui(page)).controls.find(c=>c.key===key); assert(c, key); const [a,b,cx,d]=c.rect; await page.mouse.click((a+cx)/2,(b+d)/2); await settle(page); }
async function command(page,text) { await control(page,'command/input'); await page.keyboard.press('Control+a'); await page.keyboard.type(text); await page.keyboard.press('Enter'); await page.waitForFunction(text=>JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).history.at(-1)?.startsWith(`> ${text}\n`),text); await page.mouse.click(650,500); await settle(page); }
function project(s,p) { const v=p.map((n,i)=>n-s.origin[i]); const c=[0,1,2,3].map(r=>s.mvp[r]*v[0]+s.mvp[r+4]*v[1]+s.mvp[r+8]*v[2]+s.mvp[r+12]); return [(c[0]/c[3]*.5+.5)*s.logical_canvas[0],(.5-c[1]/c[3]*.5)*s.logical_canvas[1]]; }
(async()=>{
 await fs.mkdir(out,{recursive:true});
 const browser=await chromium.launch({executablePath:'/usr/bin/google-chrome',headless:process.env.HEADLESS === '1',args:['--enable-unsafe-webgpu','--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE','--ozone-platform=x11']});
 const page=await browser.newPage({viewport:{width:1200,height:800}}); const errors=[]; page.on('pageerror',e=>errors.push(e.message)); page.on('console',m=>{if(m.type()==='error') errors.push(m.text());});
 try {
 await page.route('**/view_local.yaml',r=>r.fulfill({contentType:'application/yaml',body:'name: Editing lessons\nitems:\n  - file: extension-nested.pb\n'}));
 await page.route('**/extension-nested.pb',r=>r.fulfill({path:path.join(root,'docs/extensions/nested.pb')}));
 await page.route('**/favicon.ico',r=>r.fulfill({status:204}));
 await page.goto(new URL('?data=off&inspect=1',process.env.VIEWER_URL || 'http://127.0.0.1:8770/').href);
 await page.waitForFunction(()=>{const s=document.querySelector('canvas')?.getAttribute('data-viewer-inspection');return s&&JSON.parse(s).objects===3;},{},{timeout:90000});
 const docsCorner=await page.locator('#viewer-docs').boundingBox();
 assert.equal(docsCorner.x+docsCorner.width,1200); assert.equal(docsCorner.y+docsCorner.height,800);
 assert(!(await ui(page)).controls.some(c=>c.key==='command/hint'), 'no idle instruction row');
 assert.equal((await ui(page)).layers_open,false);
 assert(!(await ui(page)).controls.some(c=>c.key.startsWith('toolbar/')||c.key==='command/run'||c.key==='command/close'));
 assert((await ui(page)).controls.find(c=>c.key==='command/input').rect[1]>680);
 await control(page,'command/collapse');
 let resize=(await ui(page)).controls.find(c=>c.key==='command/resize').rect;
 const originalScene=(await ui(page)).scene_rect;
 await page.mouse.move(600,(resize[1]+resize[3])/2); await page.mouse.down(); await settle(page);
 await page.mouse.move(600,(resize[1]+resize[3])/2-130,{steps:12}); await settle(page); await page.mouse.up(); await settle(page);
 assert((await ui(page)).scene_rect[3]<originalScene[3]-100,'dragging up expands history: '+JSON.stringify({originalScene,after:(await ui(page)).scene_rect,resize}));
 await control(page,'command/collapse'); await control(page,'command/collapse');
 assert((await ui(page)).scene_rect[3]<originalScene[3]-100,'expanded height survives collapse');
 resize=(await ui(page)).controls.find(c=>c.key==='command/resize').rect;
 await page.mouse.move(600,(resize[1]+resize[3])/2); await page.mouse.down(); await settle(page);
 await page.mouse.move(600,originalScene[3],{steps:12}); await settle(page); await page.mouse.up(); await settle(page);
 assert(Math.abs((await ui(page)).scene_rect[3]-originalScene[3])<5,'dragging down reduces history');
 const optionLabels = async () => (await ui(page)).controls.filter(c=>c.key.startsWith('command/option/')).map(c=>c.label);
 assert.deepEqual(await optionLabels(),[], 'idle input has no options');
 for (const [prefix, name, key, options] of [
   ['La','Layers','Tab',['On','Off']],
   ['Sn','Snap','Space',['On','Off','End','Near','Mid','Center','Perp']],
   ['Ar','Arctic','Enter',['On','Off']],
   ['Out','Outline','Enter',['On','Off']],
   ['Op','Opacity','Enter',['1','0.7','0.4','0']],
 ]) {
   await control(page,'command/input'); await page.keyboard.type(prefix); await settle(page);
   assert.equal((await ui(page)).command,name);
   assert.deepEqual(await optionLabels(),[],name+' suggestions do not expose options');
   await page.keyboard.press(key); await settle(page);
   assert.equal((await ui(page)).command,name+' ');
   assert.deepEqual(await optionLabels(),options,name+' exposes options after accepting the command');
   assert.equal((await ui(page)).completion_rect,null,'accepted commands close the command list');
   await page.keyboard.press('Escape'); await settle(page);
   assert.deepEqual(await optionLabels(),[], 'Escape clears options');
 }
 await control(page,'command/input'); await page.keyboard.type('Arctic'); await settle(page);
 assert.deepEqual(await optionLabels(),[], 'fully typed commands wait for acceptance');
 await page.keyboard.press('Enter'); await settle(page);
 assert.equal((await state(page)).ssao,false, 'accepting Arctic waits for an option');
 await control(page,'command/option/On'); assert.equal((await state(page)).ssao,true);
 assert.equal((await state(page)).outlines,true, 'Arctic enables outlines');
 assert.deepEqual(await optionLabels(),[], 'executing an option clears options');
 await page.keyboard.type('Arctic Off'); await page.keyboard.press('Enter'); await settle(page);
 assert.equal((await state(page)).ssao,false, 'complete commands still execute directly');
 assert.equal((await state(page)).outlines,true, 'Arctic Off preserves outlines');
 await command(page,'Outline Off'); assert.equal((await state(page)).outlines,false);
 await command(page,'Outline On'); assert.equal((await state(page)).outlines,true); assert.equal((await state(page)).ssao,false);
 await command(page,'Outline Off');
 await command(page,'Arctic On'); assert.equal((await state(page)).outlines,true);
 await command(page,'Outline Off'); assert.equal((await state(page)).outlines,false); assert.equal((await state(page)).ssao,true);
 await page.mouse.move(650,450); await page.mouse.down({button:'right'}); await page.mouse.move(670,460,{steps:4}); await page.mouse.up({button:'right'}); await settle(page);
 assert.equal((await state(page)).outlines,false, 'redrawing Arctic preserves Outline Off');
 await command(page,'Arctic On'); assert.equal((await state(page)).outlines,true, 'Arctic On restores outlines');
 await control(page,'command/input'); await page.keyboard.type('Outline'); await page.keyboard.press('Enter'); await settle(page); assert.deepEqual(await optionLabels(),['On','Off']);
 await control(page,'command/option/Off'); assert.equal((await state(page)).outlines,false); assert.equal((await state(page)).ssao,true);
 await command(page,'Arctic Off');
 await page.mouse.click(650,500); await page.keyboard.press('Escape'); await settle(page); await page.keyboard.press('g'); await settle(page);
 assert.equal((await state(page)).ssao,true); assert.equal((await state(page)).outlines,true, 'G enables outlines with Arctic');
 await page.keyboard.press('o'); await settle(page); assert.equal((await state(page)).outlines,false); assert.equal((await state(page)).ssao,true);
 await page.keyboard.press('g'); await settle(page); assert.equal((await state(page)).ssao,false); assert.equal((await state(page)).outlines,false);
 await control(page,'command/input');
 await page.keyboard.type('Lay'); await settle(page); assert.equal((await ui(page)).command,'Layers', 'first-load inline completion'); assert(Math.abs((await ui(page)).completion_rect[3] - (await ui(page)).controls.find(c=>c.key==='command/input').rect[1]) < 2, 'popup touches the input field'); if(process.env.SCREENSHOTS) await page.screenshot({path:out+'/inline-completion.png'}); await page.keyboard.press('Backspace'); await settle(page); assert.equal((await ui(page)).command,'Lay', 'delete the suggested suffix'); await page.keyboard.type('ers'); await settle(page); assert.equal((await ui(page)).command,'Layers', 'typing replaces the suffix'); await page.keyboard.press('Tab'); await settle(page); assert.equal((await ui(page)).command,'Layers ',JSON.stringify(await ui(page)));
 let inline=await ui(page), inputRect=inline.controls.find(c=>c.key==='command/input').rect;
 assert.equal(inline.completion_rect,null,'suboptions stay in the command row');
 for(const c of inline.controls.filter(c=>c.key.startsWith('command/option/'))) assert(Math.abs((c.rect[1]+c.rect[3]-inputRect[1]-inputRect[3])/2)<4,'inline option '+c.key);
 await control(page,'command/option/On'); assert((await ui(page)).layers_open);
 const expanded=(await ui(page)).scene_rect; await control(page,'layers/collapse'); assert((await ui(page)).scene_rect[2]>expanded[2]+100); await control(page,'layers/collapse'); assert.equal((await ui(page)).scene_rect[2],expanded[2]);
 await control(page,'command/collapse'); assert((await ui(page)).controls.find(c=>c.key==='command/input').rect[1]>750); await control(page,'command/collapse'); assert((await ui(page)).scene_rect[3]<700);
 assert.equal(page.context().pages().length,1, 'collapse controls must not open the documentation corner');
 let rows=(await ui(page)).rows; const group=rows.find(r=>r.count>=2&&r.key.startsWith('select/')); assert(group);
 await control(page,group.key); let s=await state(page); assert(s.selected_group_count>=2); assert(s.widget); assert((await ui(page)).rows.some(r=>r.selected));
 const before=s.selected_models; const origin=s.widget[0]; const from=project(s,[origin[0]+96*.85*s.widget[1],origin[1],origin[2]]);
 await page.mouse.move(...from); await page.mouse.down(); await settle(page); await page.mouse.move(from[0]+35,from[1],{steps:8}); await settle(page); await page.mouse.up(); await settle(page);
 s=await state(page); assert.equal(s.selected_models.length,before.length); const moves=s.selected_models.map((m,i)=>m[12]-before[i][12]); assert(moves.every(m=>Math.abs(m)>0.1), JSON.stringify({moves, before, after:s.selected_models})); assert(moves.every(m=>Math.abs(m-moves[0])<0.01));
 if (process.env.SCREENSHOTS) await page.screenshot({path:out+'/group-gumball.png'});
 const index=group.key.split('/')[1]; await control(page,'lock/'+index); assert.equal((await state(page)).selected_rows.length,0); await control(page,group.key); assert.equal((await state(page)).selected_rows.length,0); await control(page,'lock/'+index);
 await command(page,'Layers Off'); assert.equal((await ui(page)).layers_open,false);
 await command(page,'Line -100,130,0 100,130,0'); await command(page,'Line -100,180,0 100,180,0');
 await page.locator('canvas').focus(); await page.keyboard.press('Escape'); await page.keyboard.press('5'); await page.keyboard.press('f'); await page.mouse.move(600,450); await page.mouse.wheel(0,300); await settle(page);
 s=await state(page); await page.mouse.click(...project(s,[0,130,0])); await settle(page); await page.keyboard.down('Shift'); await page.mouse.click(...project(s,[0,180,0])); await page.keyboard.up('Shift'); await settle(page); assert.equal((await state(page)).selected_rows.length,2);
 await page.keyboard.press('g'); await settle(page); assert.equal((await state(page)).ssao,true); await page.keyboard.press('g'); await settle(page); assert.equal((await state(page)).ssao,false);
 await control(page,'command/input'); await page.keyboard.type('Layers'); await page.keyboard.press('Enter'); await settle(page); assert.equal((await ui(page)).command,'Layers '); await page.keyboard.type('On'); await page.keyboard.press('Enter'); await settle(page); assert((await ui(page)).layers_open);
 // A pending geometry command takes canvas clicks before selection or gumball.
 await command(page,'Layers Off');
 async function enter(text) { await control(page,'command/input'); await page.keyboard.press('Control+a'); await page.keyboard.type(text); await page.keyboard.press('Enter'); await settle(page); }
 const original=(await state(page)).objects;
 await enter('Line'); assert.equal((await state(page)).drawing.command,'line'); assert((await state(page)).snap_enabled);
 s=await state(page); let end=project(s,[100,130,0]);
 await page.mouse.move(end[0]+3,end[1]+2); await settle(page); assert.equal((await state(page)).drawing.snap,'End');
 if(process.env.SCREENSHOTS) await page.screenshot({path:out+'/snap-preview.png'});
 await page.mouse.click(end[0]+3,end[1]+2); await settle(page);
 assert.deepEqual((await state(page)).drawing.points,[[100,130,0]],'click snaps exactly to the existing endpoint');
 await page.keyboard.type('@0,25,0'); await page.keyboard.press('Enter'); await settle(page);
 assert.equal((await state(page)).objects,original+1); assert.equal((await state(page)).drawing,null);
 await enter('Undo'); assert.equal((await state(page)).objects,original,'one drawing is one undo');
 await enter('Line 0,250,0'); assert.deepEqual((await state(page)).drawing.points,[[0,250,0]]);
 s=await state(page); await page.mouse.click(...project(s,[100,180,0])); await settle(page); assert.equal((await state(page)).objects,original+1,'typed start and clicked endpoint');
 await enter('Point'); await page.mouse.click(470,280); await settle(page); assert.equal((await state(page)).objects,original+2,'Point accepts a canvas click');
 await enter('Polyline'); assert.deepEqual(await optionLabels(),['Points','Rectangle','Polygon','Close','Finish']); await enter('0,270,0'); await enter('100,270,0'); await page.keyboard.press('Enter'); await settle(page); assert.equal((await state(page)).objects,original+3,'Enter finishes polyline: '+JSON.stringify((await ui(page)).history.slice(-5)));
 await enter('Line'); await enter('Snap Off'); assert.equal((await state(page)).snap_enabled,false); assert((await state(page)).drawing,'Snap preserves the draft');
 s=await state(page); end=project(s,[100,130,0]); await page.mouse.move(end[0]+3,end[1]+2); await settle(page); assert.equal((await state(page)).drawing.snap,null);
 await page.keyboard.press('Escape'); await settle(page); assert.equal((await state(page)).drawing,null); assert.equal((await state(page)).objects,original+3,'Escape does not create partial geometry');
 await enter('Snap On'); assert.equal((await state(page)).snap_enabled,true);
 // Clicking a suggestion starts the command immediately.
 await control(page,'command/input'); await page.keyboard.type('Poi'); await settle(page);
 await control(page,'command/completion/Point'); assert.equal((await state(page)).drawing.command,'point');
 await page.mouse.click(510,320); await settle(page); assert.equal((await state(page)).objects,original+4);
 await enter('Polyline'); await control(page,'command/option/Rectangle');
 assert.equal((await state(page)).drawing.construction,'rectangle');
 if(process.env.SCREENSHOTS) await page.screenshot({path:out+'/polyline-constructions.png'});
 await page.mouse.click(470,280); await settle(page); await page.mouse.click(540,340); await settle(page);
 assert.equal((await state(page)).objects,original+5,'two corners create a rectangle');
 await enter('Undo'); assert.equal((await state(page)).objects,original+4,'rectangle is one undo');
 await enter('Polyline'); await control(page,'command/option/Polygon');
 await enter('Sides 5'); assert.equal((await state(page)).drawing.sides,5);
 await enter('0,300,0'); await enter('40,300,0');
 assert.equal((await state(page)).objects,original+5,'center and radius create a polygon');
 assert.equal((await state(page)).drawing,null);
 await enter('Polyline'); await control(page,'command/option/Points');
 await page.mouse.click(470,280); await settle(page); await page.mouse.click(550,330); await settle(page);
 await control(page,'command/option/Finish'); assert.equal((await state(page)).objects,original+6,'clicks and Finish create a polyline');
 await enter('Lay'); assert.equal((await ui(page)).command,'Layers ');
 await page.keyboard.press('Enter'); await settle(page); assert((await ui(page)).layers_open,'second Enter accepts the default option');
 await enter('Layers'); await page.keyboard.press('ArrowDown'); await settle(page); assert.equal((await ui(page)).command.trimEnd(),'Layers Off');
 assert.equal((await ui(page)).completion_rect,null,'arrow-selected options are inline');
 await page.keyboard.press('Enter'); await settle(page); assert.equal((await ui(page)).layers_open,false);
 await enter('Lin'); assert.equal((await state(page)).drawing.command,'line','partial command enters drawing without completing the word');
 await page.keyboard.press('Escape'); await control(page,'command/input'); await page.keyboard.press('ArrowDown'); await settle(page);
 assert.equal((await ui(page)).command,'Arctic','empty input browses every command');
 assert((await ui(page)).controls.some(c=>c.key==='command/completion/Undo'),'last command is available');
 await page.keyboard.press('Escape'); await control(page,'command/input'); await page.keyboard.type('La'); await settle(page);
 assert((await ui(page)).controls.some(c=>c.key==='command/completion/Undo'),'prefix matches do not hide other commands');
 const wheelRect=(await ui(page)).controls.find(c=>c.key==='command/input').rect;
 await page.mouse.move((wheelRect[0]+wheelRect[2])/2,(wheelRect[1]+wheelRect[3])/2);
 await page.mouse.wheel(0,100); await settle(page); assert.equal((await ui(page)).command,'Arctic','wheel moves from the matching Layers to the remaining commands');
 await page.keyboard.press('Escape');
 for (const width of [480,320]) {
   await page.setViewportSize({width,height:640}); await settle(page);
   await enter('Polyline');
   let narrow=await ui(page);
   for(const key of ['command/input','command/hint','command/collapse']) {
     const rect=narrow.controls.find(c=>c.key===key).rect;
     assert(rect[0]>=0 && rect[2]<=width && rect[1]>=0 && rect[3]<=640,key+' fits at '+width+': '+rect);
   }
   assert(narrow.controls.find(c=>c.key==='command/hint').rect[3]<=narrow.controls.find(c=>c.key==='command/input').rect[1],'wrapped hints do not overlap the input');
   await page.keyboard.press('Escape'); await control(page,'command/input');
   await page.keyboard.type('Move '+('1234567890,'.repeat(70))); await settle(page);
   narrow=await ui(page); assert(narrow.controls.find(c=>c.key==='command/input').rect[2]<=width,'long input scrolls inside its field');
   await page.keyboard.press('Enter'); await settle(page);
   await page.keyboard.type('L'); await settle(page); narrow=await ui(page);
   assert(narrow.completion_rect[0]>=0 && narrow.completion_rect[2]<=width && narrow.completion_rect[1]>=0,'popup fits narrow window');
   assert(Math.abs(narrow.completion_rect[3]-narrow.controls.find(c=>c.key==='command/input').rect[1])<2,'narrow popup touches input');
   assert(await page.evaluate(()=>document.documentElement.scrollWidth<=window.innerWidth),'no horizontal page overflow');
   if(process.env.SCREENSHOTS) await page.screenshot({path:out+'/narrow-'+width+'.png'});
   await page.keyboard.press('Escape');
 }
 if (process.env.SCREENSHOTS) await page.screenshot({path:out+'/command-options.png'}); assert.deepEqual(errors,[]); console.log('PASS resizable history, clickable command suggestions, Point/Line/Polyline drawing, rectangle/polygon constructors, command options, hidden layers, panel collapse, recursive selection/lock, group gumball, Shift selection, G toggle, mixed click/coordinate drawing, snap defaults/toggle, undo, Escape');
 } catch(e) { if (process.env.SCREENSHOTS) await page.screenshot({path:out+'/failure.png'}).catch(()=>{}); console.error(await page.locator('body').innerText()); throw e; } finally { await browser.close(); }
})().catch(e=>{console.error(e);process.exitCode=1;});
