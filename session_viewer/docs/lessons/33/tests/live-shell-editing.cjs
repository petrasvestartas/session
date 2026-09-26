/** Real pointer latency, stable scene rows, source commit, and discoverable point creation. */
const {chromium}=require('playwright');
const assert=require('node:assert/strict');
const ui=require('../docs/extensions/browser.cjs');
const state=page=>page.evaluate(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')));
function project(s,p){const v=p.map((x,i)=>x-s.origin[i]);const c=[0,1,2,3].map(r=>s.mvp[r]*v[0]+s.mvp[4+r]*v[1]+s.mvp[8+r]*v[2]+s.mvp[12+r]);return [(c[0]/c[3]*.5+.5)*s.logical_canvas[0],(.5-c[1]/c[3]*.5)*s.logical_canvas[1]];}
async function settle(page){await page.waitForTimeout(120);await page.waitForFunction(()=>!JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).pick_busy);}
(async()=>{
 const stress=process.env.VIEWER_STRESS==='1';
 const browser=await chromium.launch({executablePath:process.env.CHROME_BIN||'/usr/bin/google-chrome',headless:false,args:JSON.parse(process.env.VIEWER_CHROME_ARGS||'[]')});
 try {
  const page=await browser.newPage({viewport:{width:1400,height:900}});const errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.route('**/view_local.yaml',r=>r.fulfill({contentType:'application/yaml',body:stress?'name: Live shell stress\nitems:\n  - file: fixture.pb\n    xform: [1000,0,0,0, 0,1000,0,0, 0,0,1000,0, 0,0,0,1]\n  - file: pb/view_local_bunny.pb\n    xform: [12845,0,0,0, 0,0,12845,0, 0,-12845,0,0, 3000,0,600,1]\n  - file: pb/view_local_lion.pb\n    at: [-3000,0,2000]\n':'name: Live shell editing\nitems:\n  - file: fixture.pb\n'}));
  await page.route('**/fixture.pb',r=>r.fulfill({path:process.env.VIEWER_INTERACTION_FIXTURE||'/tmp/viewer-interaction-current.pb'}));
  await page.goto(new URL('?inspect=1',process.env.VIEWER_URL||'http://127.0.0.1:8770/').href);
  await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas')?.getAttribute('data-viewer-inspection')||'{}').objects>=7).catch(async e=>{console.log('LOAD', await state(page).catch(()=>null), errors);await page.screenshot({path:'/tmp/viewer-live-shell-load.png'});throw e;});
  if(stress)await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).cloud_points>300000,{},{timeout:60000});
  await ui.button(page,'layers/close','Close layers');await page.locator('canvas').focus();await page.keyboard.press('5');await page.keyboard.press('f');await settle(page);
  let s=await state(page);assert.equal(s.preview_cache_bytes,0,'no preview is held before a drag');
  await page.keyboard.down('Control');await page.keyboard.down('Shift');await page.mouse.click(...project(s,stress?[-3000,-3000,200]:[-3,-3,.2]));await page.keyboard.up('Shift');await page.keyboard.up('Control');await settle(page);
  s=await state(page);assert(s.selection.Face,'Ctrl+Shift selects the original shell face');const revision=s.scene_revision;
  const [origin,scale]=s.widget;const start=project(s,[origin[0]+86*scale,origin[1],origin[2]]);
  await page.mouse.move(...start);await page.mouse.down();const started=Date.now();
  for(let i=1;i<=12;++i)await page.mouse.move(start[0]+i*2,start[1]);
  await settle(page);const preview=await state(page);const dragMs=Date.now()-started;assert(preview.preview_cache_bytes>0,'the dragged shell has live parameter samples');
  assert.equal(preview.scene_revision,revision,'drag does not rebuild any document');assert.notDeepEqual(preview.widget[0],origin,'gumball follows each move');
  const releaseStart=Date.now();await page.mouse.up();await settle(page);const released=await state(page);const releaseMs=Date.now()-releaseStart;
  assert.equal(released.scene_revision,revision,'release commits without remeshing the scene');assert(released.selection.Face,'face selection survives commit');
  assert(dragMs<3000,`12 live drag events took ${dragMs} ms`);assert(releaseMs<1000,`release took ${releaseMs} ms`);
  await ui.button(page,'toolbar/Vtx','source');await settle(page);assert((await state(page)).controls.some(c=>Math.abs(c.position[0])>1.01),'drag commits original BRep controls');
  await ui.command(page,'undo');const count=(await state(page)).objects;
  await ui.typeCommand(page,'Point');assert((await ui.ui(page)).hint.includes('Point 0,0,0'),'typing names shows an executable syntax example');
  await ui.command(page,'Point 0,0,0');s=await state(page);assert.equal(s.objects,count+1);assert.notEqual(s.selected,null);assert((await ui.ui(page)).history.at(-1).includes('Created and selected point'));
  await ui.command(page,'Fit');await settle(page);assert.deepEqual(errors,[]);console.log(`PASS ${stress?'large-scene':'fixture'} shell live drag ${dragMs} ms / 12 moves; release ${releaseMs} ms; source commit, undo and Point hints/selection`);
 } finally {await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
