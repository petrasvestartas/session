/** Large source mesh and point-cloud dragging; stationary-camera raster invalidation. */
const assert=require('node:assert/strict');
const {PNG}=require('pngjs');
const {chromium}=require('playwright');
const ui=require('../docs/extensions/browser.cjs');
const state=async p=>JSON.parse(await p.locator('canvas').getAttribute('data-viewer-inspection'));
function project(s,p){const v=p.map((x,i)=>x-s.origin[i]);const c=[0,1,2,3].map(r=>s.mvp[r]*v[0]+s.mvp[4+r]*v[1]+s.mvp[8+r]*v[2]+s.mvp[12+r]);return [(c[0]/c[3]*.5+.5)*s.logical_canvas[0],(.5-c[1]/c[3]*.5)*s.logical_canvas[1]];}
async function settle(p){await p.waitForTimeout(120);await p.waitForFunction(()=>!JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).pick_busy);}
function changed(a,b){a=PNG.sync.read(a);b=PNG.sync.read(b);let n=0;for(let y=120;y<650;y++)for(let x=100;x<1150;x++){const i=(y*a.width+x)*4;if(Math.abs(a.data[i]-b.data[i])+Math.abs(a.data[i+1]-b.data[i+1])+Math.abs(a.data[i+2]-b.data[i+2])>60)n++;}return n;}
(async()=>{
 const browser=await chromium.launch({executablePath:'/usr/bin/google-chrome',headless:false,args:JSON.parse(process.env.VIEWER_CHROME_ARGS||'[]')});
 try{for(const scenario of ['bunny-edge','bunny-face','lion']){const kind=scenario.split('-')[0]; const face=scenario==='bunny-face';
 const page=await browser.newPage({viewport:{width:1400,height:900}});const errors=[];page.on('pageerror',e=>errors.push(e.message));
 await page.route('**/view_local.yaml',r=>r.fulfill({contentType:'application/yaml',body:`name: Drag test\nitems:\n  - file: pb/view_local_${kind}.pb\n`}));
 await page.goto(new URL('?data=off&inspect=1',process.env.VIEWER_URL||'http://127.0.0.1:8770/').href);
 await page.waitForFunction(kind=>{const s=JSON.parse(document.querySelector('canvas')?.getAttribute('data-viewer-inspection')||'{}');return kind==='lion'?s.cloud_points>300000:s.vertices>30000;},kind,{timeout:60000});
 if((await ui.ui(page)).layers_open)await ui.button(page,'layers/close','Close layers');
 await page.mouse.click(1350,50);await settle(page);await page.keyboard.press('7');await settle(page);await page.keyboard.press('f');await settle(page);
 // Find a visible object/face through real picks, avoiding its central nameplate.
 let found=false;
 if(face)await ui.button(page,'toolbar/Face','Face');
 if(kind==='bunny'&&!face){await page.keyboard.down('Control');await page.keyboard.down('Shift');}
 for(const [x,y] of [[620,380],[650,350],[700,300],[570,330],[720,420],[700,450]]){
 await page.mouse.click(x,y);await settle(page);const s=await state(page);
 if(kind==='bunny'?(face?s.selection.Face:s.selection.Edge):s.selected!==null&&s.cloud_points>0&&s.widget){found=true;break;}}
 if(kind==='bunny'&&!face){await page.keyboard.up('Shift');await page.keyboard.up('Control');}
 if(!found){await page.screenshot({path:`/tmp/viewer-${kind}-pick-failure.png`});console.log(await state(page));}
 assert(found,`${kind}: visible geometry was selected`);
 let s=await state(page);const rev=s.scene_revision;const [origin,scale]=s.widget;const start=project(s,[origin[0]+86*scale,origin[1],origin[2]]);
 const before=await page.screenshot();await page.mouse.move(...start);await page.mouse.down();const begin=Date.now();
 for(let i=1;i<=12;i++)await page.mouse.move(start[0]+i*3,start[1]);await settle(page);const drag=Date.now()-begin;
 const preview=await state(page);assert.notDeepEqual(preview.widget[0],origin);assert.equal(preview.scene_revision,rev);const after=await page.screenshot();
 if(kind==='lion')assert(changed(before,after)>5000,'cloud itself moves with the camera stationary');
 const releaseStart=Date.now();await page.mouse.up();await settle(page);const release=Date.now()-releaseStart;
 assert(drag<2000,`${kind} drag ${drag}ms`);assert(release<1500,`${kind} release ${release}ms`);assert.deepEqual(errors,[]);
 console.log(`PASS ${scenario}: ${s.vertices} render vertices / ${s.cloud_points} points; 12 drag updates ${drag}ms, release ${release}ms`);await page.close();
 }}finally{await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
