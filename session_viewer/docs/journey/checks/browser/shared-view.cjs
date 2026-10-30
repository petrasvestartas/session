const adaptive=require('./adaptive-gpu-reuse.cjs');
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),{PNG}=require('pngjs');
const output=path.resolve(__dirname,'../../../screenshots/journey/review');
const value=(page,key)=>page.locator('canvas').getAttribute(key).then(JSON.parse);
function hook(){
    if(window.sharedViewWrites)return;const stats=window.sharedViewWrites={armed:false,created:{},writes:{}};
    const create=GPUDevice.prototype.createBuffer;GPUDevice.prototype.createBuffer=function(descriptor){const result=create.call(this,descriptor),label=descriptor.label||'';const item=stats.created[label]||(stats.created[label]={count:0,bytes:0});item.count++;item.bytes+=descriptor.size;return result;};
    const write=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(buffer,...args){if(stats.armed){const label=buffer.label||'',item=stats.writes[label]||(stats.writes[label]={count:0,bytes:0}),element=args[1].BYTES_PER_ELEMENT||1;item.count++;item.bytes+=args[3]===undefined?args[1].byteLength-(args[2]||0)*element:args[3]*element;}return write.call(this,buffer,...args);};
}
async function prepare(page,address,helpers,archive,point=false){
    await page.addInitScript(hook);const url=new URL(address);for(const key of ['lit','backface','dpr'])url.searchParams.delete(key);url.searchParams.set('nogrid','1');url.searchParams.set('opacity','1');await page.goto(url.href);
    await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
    const choosing=page.waitForEvent('filechooser');await helpers.command(page,'Open');await(await choosing).setFiles({name:'original-shared-frame.session',mimeType:'application/octet-stream',buffer:Buffer.from(archive,'base64')});
    await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-surfaces')).length===2);
    for(const line of ['Layers Off','View Grid Off','Arctic Off','View Outline Off','View Lighting Off','View Backfaces Off',...(point?['Point w2.5,0.5,1']:[]),'Select None','View Isometric','Fit'])await helpers.command(page,line);
    assert.equal((await value(page,'data-drawing-originals')).points.length,1);assert.equal((await value(page,'data-drawing-originals')).lines.length,6);
    await page.waitForTimeout(200);
}
async function saved(page,helpers){const waiting=page.waitForEvent('download');await helpers.command(page,'Save');return fs.readFileSync(await(await waiting).path());}
async function measured(page,helpers){
    const gpu=await adaptive.snapshot(page,{lane:'mesh',source:'surface',expectedRows:2,expectedQualities:2});
    const before=await helpers.drawing(page),matrix=await value(page,'data-camera-matrix'),originals=await value(page,'data-drawing-originals'),placements=await value(page,'data-row-placements'),uploads=gpu.stats;
    await page.evaluate(()=>{window.sharedViewWrites.writes={};window.sharedViewWrites.armed=true;});
    await helpers.command(page,'Zoom Out');assert.notEqual(await helpers.drawing(page),before);await helpers.command(page,'Zoom In');
    const stats=await page.evaluate(()=>{window.sharedViewWrites.armed=false;return window.sharedViewWrites;});
    assert.equal(await helpers.drawing(page),before,'Shared frame restores every actual surface, input-curve and point pixel');assert.deepEqual(await value(page,'data-camera-matrix'),matrix);assert.deepEqual(await value(page,'data-drawing-originals'),originals);assert.deepEqual(await value(page,'data-row-placements'),placements);if(!(await adaptive.snapshot(page,{lane:'mesh',source:'surface',expectedRows:2,expectedQualities:2})).adaptive)assert.equal((await value(page,'data-gpu-stats'))[0],uploads[0]);
    assert(stats.writes['view transform and normal diagnostic']?.count>0,'Observe actual color-frame GPU writes');return {stats,before,originals,placements,uploads};
}
async function picture(page,kind,density,helpers){if(!helpers.step||helpers.step.browser_check==='journey/checks/browser/shared-view.cjs'){fs.mkdirSync(output,{recursive:true});await page.screenshot({path:path.join(output,`${helpers.step?.id||'shared-view'}-${kind}-dpr${density}.png`)});}}
async function project(page,point){const m=await value(page,'data-render-matrix'),origin=await value(page,'data-render-origin'),p=point.map((v,i)=>v-origin[i]),c=[0,1,2,3].map(row=>p.reduce((s,v,i)=>s+m[i*4+row]*v,m[12+row]));assert(c[3]>0&&c[2]/c[3]>0&&c[2]/c[3]<1);return [(c[0]/c[3]+1)/2,(1-c[1]/c[3])/2];}
async function feature(page,address,density,helpers){
    const original=require('./shared-view-original.json');await prepare(page,address,helpers,original.archive);const source=await saved(page,helpers),result=await measured(page,helpers),baseline=original.baseline[String(density)];
    assert.equal(result.before,baseline.before,'Every actual original pixel matches the preceding accepted implementation at this density');
    assert.equal(result.stats.created['stroke view']?.count||0,0,'Ordinary ink owns no duplicate frame buffers');assert.equal(result.stats.writes['stroke view']?.count||0,0,'Ordinary ink performs no duplicate frame writes');
    assert.equal(baseline.stats.created['stroke view'].count,13,'The accepted preceding implementation really allocates thirteen lane frames');assert(baseline.stats.writes['stroke view'].count>0,'The preceding implementation really writes its active lane frames');
    await picture(page,'originals',density,helpers);
    const owner=(await value(page,'data-surfaces'))[1].id,location=await project(page,[3.3,0.7,0.063]),bounds=await page.locator('canvas').boundingBox(),submitted=(await value(page,'data-pick-gpu'))[3];
    await page.keyboard.press('Escape');await page.mouse.click(bounds.x+location[0]*bounds.width,bounds.y+location[1]*bounds.height);
    await page.waitForFunction(({owner,submitted})=>{const c=document.querySelector('canvas'),p=JSON.parse(c.getAttribute('data-pick-gpu'));return JSON.parse(c.getAttribute('data-selected-ids')).includes(owner)&&p[3]>submitted&&p[0]===0&&p[1]===0;},{owner,submitted});
    const png=PNG.sync.read(Buffer.from((await require('./canvas-image.cjs')(page.locator('canvas'))).split(',')[1],'base64')),sample=await project(page,[3.7,0.3,0.063]),index=(Math.floor(sample[1]*png.height)*png.width+Math.floor(sample[0]*png.width))*4,rgb=[...png.data.subarray(index,index+3)];
    assert(rgb[0]>240&&rgb[1]>240&&rgb[2]<20,`Actual original surface pick produces yellow interior fill away from the gizmo: ${rgb}`);await picture(page,'selected',density,helpers);
    await helpers.command(page,'Select None');assert.equal(await helpers.drawing(page),result.before,'Independent original pick and clearing preserve exact restored color-frame pixels');assert.deepEqual(await saved(page,helpers),source,'Shared frames preserve exact saved NURBS/input-line/point archive bytes');assert.deepEqual(await value(page,'data-drawing-originals'),result.originals);assert.deepEqual(await value(page,'data-row-placements'),result.placements);if(!(await adaptive.snapshot(page,{lane:'mesh',source:'surface',expectedRows:2,expectedQualities:2})).adaptive)assert.equal((await value(page,'data-gpu-stats'))[0],result.uploads[0]);
    console.log(`Shared color/ink frame: exact preceding original pixels, zero duplicate lane buffers/writes, real original GPU pick/yellow fill, zoom/selection/archive restoration and geometry reuse passed at DPR ${density}`);return {density,stats:result.stats,baseline:baseline.stats,before:result.before,owner,rgb};
}
module.exports=async(originalPage,helpers)=>{
    await require('./surface-controls.cjs')(originalPage,helpers);const address=originalPage.url();
    for(const density of [1,2]){const context=await originalPage.context().browser().newContext({viewport:{width:640,height:480},deviceScaleFactor:density});try{await feature(await context.newPage(),address,density,helpers);}finally{await context.close();await originalPage.bringToFront();}}
    await originalPage.setViewportSize({width:640,height:480});return feature(originalPage,address,await originalPage.evaluate(()=>devicePixelRatio),helpers);
};
module.exports.feature=feature;module.exports.prepare=prepare;module.exports.measured=measured;module.exports.saved=saved;
