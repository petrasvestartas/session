const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),{PNG}=require('pngjs');
const original=require('./surface-lighting-original.json'),output=path.resolve(__dirname,'../../../screenshots/journey/review');
const capture=helpers=>!helpers.step||helpers.step.browser_check==='journey/checks/browser/surface-lighting.cjs';
const name=helpers=>helpers.step?.id||'surface-lighting';
async function ready(page,address) {
    const url=new URL(address);url.searchParams.set('nogrid','1');url.searchParams.set('opacity','1');url.searchParams.delete('dpr');
    await page.goto(url.href);await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()&&JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready');
}
const value=(page,key)=>page.locator('canvas').getAttribute(key).then(JSON.parse);
const image=async page=>PNG.sync.read(Buffer.from((await require('./canvas-image.cjs')(page.locator('canvas'))).split(',')[1],'base64'));
async function picture(page,kind,density,helpers) {if(capture(helpers)){fs.mkdirSync(output,{recursive:true});await page.screenshot({path:path.join(output,`${name(helpers)}-${kind}-dpr${density}.png`)});}}
async function saved(page,helpers) {const waiting=page.waitForEvent('download');await helpers.command(page,'Save');return fs.readFileSync(await(await waiting).path());}
async function network(page,address,density,helpers) {
    await ready(page,address);const command=line=>helpers.command(page,line);
    const choosing=page.waitForEvent('filechooser');await command('Open');await(await choosing).setFiles({name:'original-network.session',mimeType:'application/octet-stream',buffer:Buffer.from(original.archive,'base64')});
    await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas').getAttribute('data-surfaces')).length===2);
    for(const line of ['Layers Off','View Grid Off','Arctic Off','View Outline Off','View Isometric','Fit','Select None'])await command(line);
    const surfaces=await value(page,'data-surfaces'),drawing=await value(page,'data-drawing-originals'),placements=await value(page,'data-row-placements'),uploads=(await value(page,'data-gpu-stats'))[0];
    assert.equal(surfaces.length,2);assert.equal(drawing.lines.length,6);
    const flat=await helpers.drawing(page),png=await image(page),matrix=await value(page,'data-render-matrix'),origin=await value(page,'data-render-origin');
    const project=point=>{const p=point.map((v,i)=>v-origin[i]);const clip=[0,1,2,3].map(row=>p.reduce((sum,v,i)=>sum+matrix[i*4+row]*v,matrix[12+row]));return [(clip[0]/clip[3]+1)*png.width/2,(1-clip[1]/clip[3])*png.height/2];};
    for(const [u,v]of [[0.3,0.3],[0.7,0.3],[0.3,0.7],[0.7,0.7]]) {
        const [x,y]=project([3+u,v,0.3*u*v]),i=(Math.round(y)*png.width+Math.round(x))*4;
        for(let c=0;c<3;c++)assert(Math.abs(png.data[i+c]-original.flatFillRGB[c])<=1,'Unlit original patch fill matches actual maintained-native source-color reference');
    }
    for(const [a,b]of [[[0,1,0],[2,1,1]],[[1,0,0],[1,2,1]]])for(let n=5;n<=95;n++) {
        const t=n/100;if(t>.45&&t<.55)continue;const [x,y]=project(a.map((v,i)=>v+t*(b[i]-v)));const r=Math.max(1,Math.round(density));let dark=false;
        for(let py=Math.floor(y)-r;py<=Math.floor(y)+r;py++)for(let px=Math.floor(x)-r;px<=Math.floor(x)+r;px++){const i=(py*png.width+px)*4;if(Math.max(png.data[i],png.data[i+1],png.data[i+2])<180)dark=true;}
        assert(dark,'Actual original network input curves remain continuous in the flat surface mode');
    }
    const source=await saved(page,helpers);await picture(page,'flat',density,helpers);
    await command('Arctic On');let state=await value(page,'data-clipping');assert.equal(state.arctic,true);assert.equal(state.ambient_error,null);assert(state.ambient[4]>0,'Actual ambient compositor executes');
    const arctic=await image(page);let interior=0,changed=0;const margin=Math.ceil(5*density);
    for(let y=margin;y<png.height-margin;y+=2)for(let x=margin;x<png.width-margin;x+=2) {
        let inside=true;for(const [dx,dy]of [[0,0],[-margin,0],[margin,0],[0,-margin],[0,margin]]) {const i=((y+dy)*png.width+x+dx)*4;if([0,1,2].some(c=>Math.abs(png.data[i+c]-248)>1))inside=false;}
        if(inside){interior++;const i=(y*png.width+x)*4;if([0,1,2].some(c=>arctic.data[i+c]<png.data[i+c]-1))changed++;}
    }
    assert(interior>200,'Actual interior source-color pixels exist');assert(changed>50,'Arctic changes actual surface interiors, independently of the background');
    const arcticDrawing=await helpers.drawing(page);await picture(page,'arctic',density,helpers);
    await command('Arctic Off');await command('View Outline Off');assert.equal(await helpers.drawing(page),flat,'Explicit current sticky-outline restoration returns exact flat pixels');
    assert.deepEqual(await saved(page,helpers),source,'View modes preserve exact saved original archive bytes');
    assert.deepEqual(await value(page,'data-surfaces'),surfaces);assert.deepEqual(await value(page,'data-drawing-originals'),drawing);assert.deepEqual(await value(page,'data-row-placements'),placements);assert.equal((await value(page,'data-gpu-stats'))[0],uploads);
    await command('Arctic On');assert.equal(await helpers.drawing(page),arcticDrawing,'Final asserted current Arctic original scene repeats exactly');
    console.log(`Flat/Arctic surfaces: same saved NURBS originals, maintained-native flat fill color, continuous input curves, real changed surface interiors, exact mode/archive restoration and geometry reuse passed at DPR ${density}`);
    return {density,interior,changed,surfaces,uploads,archiveBytes:source.length};
}
async function wide(page,address,density,helpers) {
    await ready(page,address);const command=line=>helpers.command(page,line);
    for(const line of ['Layers Off','Select All','Delete','Select None','Box w0,0,0 6000 4000 1000','View Reset','View Top','View Orthographic','View Grid Off','Arctic On'])await command(line);
    const originals=await value(page,'data-cads'),placements=await value(page,'data-row-placements'),uploads=(await value(page,'data-gpu-stats'))[0],matrix=await value(page,'data-camera-matrix'),source=await saved(page,helpers),near=await helpers.drawing(page);
    for(let i=0;i<3;i++)await command('Zoom Out');const state=await value(page,'data-clipping');assert.equal(state.ambient_error,null,'Wide current orthographic camera remains valid for the real ambient pass');assert(state.ambient[4]>0);
    const png=await image(page);let face=0;for(let i=0;i<png.data.length;i+=4)if(png.data[i]<240&&png.data[i+1]<240&&png.data[i+2]<240)face++;assert(face>100,'The actual wide Box has visible contrasting pixels');
    const widePicture=await helpers.drawing(page),owner=originals[0].id,render=await value(page,'data-render-matrix'),origin=await value(page,'data-render-origin');
    const point=[1800,900,1000].map((v,i)=>v-origin[i]),clip=[0,1,2,3].map(row=>point.reduce((sum,v,i)=>sum+render[i*4+row]*v,render[12+row]));
    assert(clip[2]/clip[3]>0&&clip[2]/clip[3]<1,'Actual wide original top face is inside the current depth range');
    const location=[(clip[0]/clip[3]+1)/2,(1-clip[1]/clip[3])/2],bounds=await page.locator('canvas').boundingBox(),submitted=(await value(page,'data-pick-gpu'))[3];
    await page.keyboard.press('Escape');await page.mouse.click(bounds.x+location[0]*bounds.width,bounds.y+location[1]*bounds.height);
    await page.waitForFunction(({owner,submitted})=>{const c=document.querySelector('canvas'),p=JSON.parse(c.getAttribute('data-pick-gpu'));return JSON.parse(c.getAttribute('data-selected-ids')).includes(owner)&&p[3]>submitted&&p[0]===0&&p[1]===0;},{owner,submitted});
    const selected=await image(page),index=(Math.floor(location[1]*selected.height)*selected.width+Math.floor(location[0]*selected.width))*4,fill=[...selected.data.subarray(index,index+3)];
    assert(fill[0]>240&&fill[1]>240&&fill[2]<20,`Actual selected interior yellow fill accompanies the original GPU owner away from the gizmo: ${fill}`);
    await picture(page,'wide-selected',density,helpers);await command('Select None');assert.equal(await helpers.drawing(page),widePicture,'Clearing the real original selection restores exact wide Arctic pixels');
    await picture(page,'wide',density,helpers);for(let i=0;i<3;i++)await command('Zoom In');assert.deepEqual(await value(page,'data-camera-matrix'),matrix);assert.equal(await helpers.drawing(page),near,'Wide zoom and back restores exact current Arctic pixels');
    assert.deepEqual(await saved(page,helpers),source);assert.deepEqual(await value(page,'data-cads'),originals);assert.deepEqual(await value(page,'data-row-placements'),placements);assert.equal((await value(page,'data-gpu-stats'))[0],uploads);
    console.log(`Wide Arctic camera: real original Box, stable ambient inverse, actual selected face/GPU owner, exact zoom/archive restoration and geometry reuse passed at DPR ${density}`);return {density,face,originals,uploads};
}
async function feature(page,address,density,helpers) {
    const errors=[];page.on('pageerror',e=>errors.push(e.message));const result={wide:await wide(page,address,density,helpers),network:await network(page,address,density,helpers)};assert.deepEqual(errors,[]);return result;
}
module.exports=async(originalPage,helpers)=>{
    await require('./camera-navigation-lighting-evidence.cjs')(originalPage,helpers);const address=originalPage.url();
    for(const density of [1,2]) {const context=await originalPage.context().browser().newContext({viewport:{width:640,height:480},deviceScaleFactor:density});try{await feature(await context.newPage(),address,density,helpers);}finally{await context.close();await originalPage.bringToFront();}}
    await originalPage.setViewportSize({width:640,height:480});return feature(originalPage,address,await originalPage.evaluate(()=>devicePixelRatio),helpers);
};
module.exports.feature=feature;module.exports.network=network;module.exports.wide=wide;
