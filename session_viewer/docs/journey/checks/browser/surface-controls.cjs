const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),{PNG}=require('pngjs');
const original=require('./surface-controls-original.json'),output=path.resolve(__dirname,'../../../screenshots/journey/review');
const value=(page,key)=>page.locator('canvas').getAttribute(key).then(JSON.parse);
const image=async page=>PNG.sync.read(Buffer.from((await require('./canvas-image.cjs')(page.locator('canvas'))).split(',')[1],'base64'));
async function ready(page,address){await page.goto(address);await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');}
async function saved(page,helpers){const waiting=page.waitForEvent('download');await helpers.command(page,'Save');return fs.readFileSync(await(await waiting).path());}
async function picture(page,kind,density,helpers){if(!helpers.step||helpers.step.browser_check==='journey/checks/browser/surface-controls.cjs'){fs.mkdirSync(output,{recursive:true});await page.screenshot({path:path.join(output,`${helpers.step?.id||'surface-controls'}-${kind}-dpr${density}.png`)});}}
async function samples(page){
    const png=await image(page),matrix=await value(page,'data-render-matrix'),origin=await value(page,'data-render-origin');
    return [-350,350].map(x=>{const p=[x,100,0].map((v,i)=>v-origin[i]),c=[0,1,2,3].map(row=>p.reduce((s,v,i)=>s+matrix[i*4+row]*v,matrix[12+row]));assert(c[3]>0&&c[2]/c[3]>0&&c[2]/c[3]<1);const px=Math.round((c[0]/c[3]+1)*png.width/2),py=Math.round((1-c[1]/c[3])*png.height/2);assert(px>0&&px<png.width&&py>0&&py<png.height-34);const i=(py*png.width+px)*4;return [...png.data.subarray(i,i+3)];});
}
function same(actual,expected,label){actual.forEach((color,i)=>color.forEach((v,c)=>assert(Math.abs(v-expected[i][c])<=1,`${label}: original patch${i} ${color} expected ${expected[i]}`)));}
async function setup(page,address,helpers){
    const url=new URL(address);url.searchParams.set('nogrid','1');url.searchParams.set('opacity','1');for(const key of ['dpr','lit','backface'])url.searchParams.delete(key);await ready(page,url.href);
    const choosing=page.waitForEvent('filechooser');await helpers.command(page,'Open');await(await choosing).setFiles({name:'original-winding.session',mimeType:'application/octet-stream',buffer:Buffer.from(original.archive,'base64')});
    await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-row-placements')).length===1);
    for(const line of ['Layers Off','View Grid Off','Arctic Off','View Outline Off','View Top','Fit','Select None'])await helpers.command(page,line);
}
async function headlight(page,address,density,helpers){
    await setup(page,address,helpers);const command=line=>helpers.command(page,line);await command('View Isometric');await command('Fit');const flat=await helpers.drawing(page),uploads=(await value(page,'data-gpu-stats'))[0];
    same(await samples(page),[[188,188,188],[188,188,188]],'Actual flat original colors');await picture(page,'flat',density,helpers);
    await command('View Lighting On');assert.equal((await value(page,'data-clipping')).lighting,true);const head=await helpers.drawing(page);assert.notEqual(head,flat,'Headlight changes actual drawing pixels');
    const native=original.references.headlight.colors[0][0].split(',').map(Number),colors=await samples(page);same(colors,[native,native],'Actual maintained-native wrapped headlight interior color');assert(native.every(v=>v<185&&v>100));await picture(page,'headlight',density,helpers);
    await command('View Lighting Off');assert.equal(await helpers.drawing(page),flat,'Headlight Off restores exact original flat image');assert.equal((await value(page,'data-gpu-stats'))[0],uploads);return {density,colors,native,uploads};
}
async function backfaces(page,address,density,helpers){
    await setup(page,address,helpers);const command=line=>helpers.command(page,line),flat=await helpers.drawing(page),placements=await value(page,'data-row-placements'),uploads=(await value(page,'data-gpu-stats'))[0],source=await saved(page,helpers);
    same(await samples(page),[[188,188,188],[188,188,188]],'Opposite-winding originals retain flat source colors');await command('View Backfaces On');assert.equal((await value(page,'data-clipping')).backfaces,true);
    const colors=await samples(page);same(colors,[[188,188,188],[231,63,63]],'Only the actual original back-facing patch turns red');const back=await helpers.drawing(page);assert.notEqual(back,flat);await picture(page,'backfaces',density,helpers);
    await command('View Backfaces Off');assert.equal(await helpers.drawing(page),flat,'Backfaces Off restores exact original flat image');
    await command('Arctic On');const sky=await helpers.drawing(page);await command('View Lighting On');assert.equal(await helpers.drawing(page),sky,'Arctic sky takes precedence over optional headlight');await command('View Lighting Off');await command('Arctic Off');await command('View Outline Off');assert.equal(await helpers.drawing(page),flat);
    await command('Line');await command('w0,0,0');const tool=await value(page,'data-tool');assert.equal(tool.points.length,1);
    for(const line of ['View Lighting On','View Backfaces On','View Lighting Off','View Backfaces Off']){await command(line);const next=await value(page,'data-tool');assert.equal(next.name,tool.name);assert.deepEqual(next.points,tool.points,'Presentation leaves the unfinished original line point intact');}
    await command('Escape');assert.equal(await helpers.drawing(page),flat);const restored=await saved(page,helpers);assert.deepEqual(require('./mesh-archive-records.cjs')(restored),require('./mesh-archive-records.cjs')(source),'Every saved original record and presentation value remains exact; only unordered protobuf mesh-map entry order is normalized');assert.deepEqual(await value(page,'data-row-placements'),placements);assert.equal((await value(page,'data-gpu-stats'))[0],uploads);
    await command('View Backfaces On');assert.equal(await helpers.drawing(page),back,'Final actual backface picture repeats exactly');return {density,colors,uploads,archiveBytes:source.length};
}
async function feature(page,address,density,helpers){
    const errors=[];page.on('pageerror',e=>errors.push(e.message));const query=new URL(address);query.searchParams.set('lit','0');query.searchParams.set('backface','0');await ready(page,query.href);const startup=await value(page,'data-clipping');assert.equal(startup.lighting,true,'Current lit query is presence-based');assert.equal(startup.backfaces,true,'Current backface query is presence-based');
    const result={headlight:await headlight(page,address,density,helpers),backfaces:await backfaces(page,address,density,helpers)};assert.deepEqual(errors,[]);console.log(`Current surface controls: identical original winding archive, maintained-native headlight color, actual opposite front/back pixels, sky precedence, pending original points, exact pixel/decoded-archive-record restoration and geometry reuse passed at DPR ${density}`);return result;
}
module.exports=async(originalPage,helpers)=>{
    await require('./surface-lighting.cjs')(originalPage,helpers);const address=originalPage.url();
    for(const density of [1,2]){const context=await originalPage.context().browser().newContext({viewport:{width:640,height:480},deviceScaleFactor:density});try{await feature(await context.newPage(),address,density,helpers);}finally{await context.close();await originalPage.bringToFront();}}
    await originalPage.setViewportSize({width:640,height:480});return feature(originalPage,address,await originalPage.evaluate(()=>devicePixelRatio),helpers);
};
module.exports.feature=feature;module.exports.headlight=headlight;module.exports.backfaces=backfaces;
