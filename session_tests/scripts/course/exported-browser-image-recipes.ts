import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import Module from 'node:module';
import {fileURLToPath} from 'node:url';
import {bindings} from './presentation.ts';
import {buildImageBrowser} from './image-browser-build.ts';
import {privateImageBrowserHost} from './parallel-image-browser-host.ts';
import {observeGpu, validateGpu} from './private-browser-host.ts';
import {course, docs, filesIn, hash, json, read, viewer, write} from './model.ts';

const self=fileURLToPath(import.meta.url), sha=(file:string)=>hash(fs.readFileSync(file));
const exportsByModule:Record<string,[string,number,number]>={
    'camera-analytic-depth':['feature',960,640], 'camera-extent':['feature',960,640],
    'camera-navigation':['framing',960,640], 'surface-lighting':['network',640,480],
    'surface-controls':['backfaces',640,480], 'shared-view':['prepare',640,480],
    'contact-fill':['feature',640,480], 'ink-metadata':['feature',640,480],
    'contact-boundary':['feature',640,480], 'source-faces':['feature',640,480],
    'source-pens':['finish',640,480],
};

export function exportedImageRecipe(id:string){
    const step=course().steps.find(s=>s.id===id);assert(step);
    if(!step.browser_check)return null;
    const module=path.basename(step.browser_check,'.cjs'), file=path.join(docs,step.browser_check), text=read(file);
    if(exportsByModule[module]){
        const [exportName,width,height]=exportsByModule[module];assert(text.includes('module.exports.'+exportName+'=' )||text.includes('module.exports.'+exportName+' ='));
        return {module,file,sha256:sha(file),kind:'unchanged-export',exportName,viewport:{width,height},
            scope:'One unchanged source-scoped export at DPR2. No inherited prefix or child-context matrix, and no whole canonical callback acceptance.'};
    }
    let authority = null;
    if (module === 'caps' || module === 'performance') {
        const review = path.join(docs, 'journey/release', module === 'caps' ? 'exported88ad-browser-image-review.json' : 'exported-primary-three-browser-image-review.json');
        const archived = json(review), recipe = module === 'caps' ? archived.recipe : archived.images.find((image: any) => image.recipe?.module === module)?.recipe;
        assert(recipe?.authority && recipe.module === module, 'Visible setup needs its archived publication authority');
        assert.equal(recipe.sha256, sha(file), 'The original visible callback changed');
        authority = {path: review, sha256: sha(review), archivedHelper: recipe.authority,
            recovery: 'Visible setup reconstructed from this maintained recipe and archived publication; missing private helper is not execution evidence.'};
    }
    if(['current-commands','caps','performance','hardware-grid'].includes(module))return {module,file,sha256:sha(file),kind:'original-visible-setup',authority,viewport:{width:module==='hardware-grid'?960:900,height:module==='hardware-grid'?640:760},
        scope:module==='current-commands'?'Exact original final main-page tail with all tail assertions; inherited measurement/history prefix is not executed.':module==='hardware-grid'?'Exact original first empty-grid scene and its four physical-pixel assertions at DPR2 only.':'Previously root-reviewed visible original fixture; the canonical callback ends Close and its empty termination frame is not used as a feature image.'};
    return null;
}

function recipeFor(id:string,variant=''){
    if(!variant)return exportedImageRecipe(id);
    assert.equal(variant,'unselected-rgb');const step=course().steps.find(s=>s.id===id);assert(step?.browser_check?.endsWith('/cloud-points.cjs'));
    const file=path.join(docs,step.browser_check),commands=['Close','Layers Off','View Grid Off','Example Cloud','Fit','View Isometric','Select Next','Controls'];
    assert(read(file).includes(JSON.stringify(commands).replaceAll('"',"'")));
    return {module:'cloud-points',file,sha256:sha(file),kind:'supplemental-unselected-rgb',viewport:{width:1100,height:800},commands,
        scope:'Same original synthetic1681-point colored graph as the selected yellow main figure; only Controls Off and Select None remove selection overlays. No scan, new shader, photographic density or Potree parity claim.'};
}

function check(p:any){
    assert.equal(p.helperSha256,sha(self));assert.deepEqual(p.source,bindings(p.id));assert.deepEqual(p.recipe,recipeFor(p.id,p.variant));
    for(const [file,digest]of Object.entries(p.pins))assert.equal(sha(file),digest,file);
    assert.equal(sha(path.join(p.privateDocs,'capture_journey.cjs')),p.pins[path.join(docs,'capture_journey.cjs')]);
    for(const [file,digest]of Object.entries(p.checks))assert.equal(sha(path.join(p.privateDocs,file)),digest,file);
    assert.deepEqual(Object.fromEntries(filesIn(p.dist).map(f=>[path.relative(p.dist,f).replaceAll('\\','/'),sha(f)])),p.bundle);
}

export function prepareExportedScene(id:string,buildFile:string,variant=''){
    const built=json(path.resolve(buildFile)),recipe=recipeFor(id,variant);assert(recipe);assert.equal(built.id,id);assert.deepEqual(built.source,bindings(id));
    assert(['BUILT_NOT_CAPTURED','CAPTURED_AWAITING_ROOT_VISUAL_REVIEW'].includes(built.status));
    const dist=path.join(built.workspace,'target/course-checks',id,'dist');
    const checks=Object.fromEntries(filesIn(path.join(docs,'journey/checks')).filter(f=>/\.(cjs|json|pb|session)$/.test(f)).map(f=>[path.relative(docs,f),sha(f)]));
    const inputs=[path.resolve(buildFile),path.join(docs,'capture_journey.cjs'),path.join(docs,'journey/course.json'),
        ...['parallel-image-browser-host.ts','parallel-image-guard.ps1','private-browser-host.ts'].map(f=>fileURLToPath(new URL('./'+f,import.meta.url)))];
    const pins=Object.fromEntries(inputs.map(f=>[f,sha(f)])),key=hash(JSON.stringify({source:built.source,recipe,checks,pins,helper:sha(self)}));
    const folder=path.join(viewer,'target/course-presentation/exported-scenes',id+'-'+key.slice(0,20)),preparedFile=path.join(folder,'prepared.json');
    if(fs.existsSync(preparedFile)){const retained=json(preparedFile);check(retained);return retained;}
    const privateDocs=path.join(folder,'private/docs');
    for(const file of Object.keys(checks)){const target=path.join(privateDocs,file);fs.mkdirSync(path.dirname(target),{recursive:true});fs.copyFileSync(path.join(docs,file),target);}
    fs.copyFileSync(path.join(docs,'capture_journey.cjs'),path.join(privateDocs,'capture_journey.cjs'));
    const p={status:'PREPARED_IMAGE_ONLY_NOT_EXECUTED',id,variant,folder,workspace:built.workspace,dist,bundle:built.bundle,source:built.source,
        recipe,pins,checks,privateDocs,helperSha256:sha(self),viewport:recipe.viewport,density:2,canonicalBehaviorExecuted:false,
        imagePolicy:{processors:8,memoryMiB:8192,affinityMask:'0xFF00',lane:'separate-image'}};
    write(preparedFile,JSON.stringify(p,null,2)+'\n');check(p);return p;
}

async function worker(p:any){
    const originalRequire=Module.createRequire(path.join(docs,'capture_journey.cjs')),{chromium}=originalRequire('playwright');
    const browser=await chromium.launch({headless:true,executablePath:process.env.VIEWER_BROWSER_EXECUTABLE,args:JSON.parse(process.env.VIEWER_CHROME_ARGS!)});
    try{
        const context=await browser.newContext({viewport:p.viewport,deviceScaleFactor:2,acceptDownloads:true}),page=await context.newPage(),errors:string[]=[];
        page.on('pageerror',(e:any)=>errors.push(e.message));await(new Function('page','return(async()=>{'+observeGpu+'})()'))(page);
        const capture=path.join(p.privateDocs,'capture_journey.cjs'),text=read(capture),boundary='\nasync function capture()';assert.equal(text.split(boundary).length,2);
        const helper=new Module(capture);helper.filename=capture;helper.paths=Module._nodeModulePaths(path.dirname(path.join(docs,'capture_journey.cjs')));
        const argv=process.argv;try{process.argv=[argv[0],capture,p.id];helper._compile(text.split(boundary)[0]+'\nmodule.exports={command,drawing};',capture);}finally{process.argv=argv;}
        const helpers={...helper.exports,step:course().steps.find(s=>s.id===p.id)},command=(line:string)=>helpers.command(page,line);
        const privateFile=path.join(p.privateDocs,path.relative(docs,p.recipe.file));
        const loaded=new Module(privateFile);loaded.filename=privateFile;loaded.paths=Module._nodeModulePaths(path.dirname(privateFile)).concat(Module._nodeModulePaths(path.dirname(path.join(docs,'capture_journey.cjs'))));
        // Relative original modules retain their private __dirname; external packages resolve from the actual viewer installation.
        loaded._compile(read(privateFile),privateFile);
        await page.goto(process.env.VIEWER_URL);
        await page.waitForFunction(()=>{const w=window as any;return w.wasmBindings?.runtime_running?.()&&JSON.parse(w.wasmBindings.diagnostic_snapshot()).outcome==='ready';});
        const address=page.url(),kind=p.recipe.module,semantic:any={};
        if(p.recipe.kind==='supplemental-unselected-rgb'){
            for(const line of p.recipe.commands)await command(line);
            const value=(name:string)=>page.locator('canvas').getAttribute(name).then(JSON.parse),before=await value('data-cloud-sources');
            assert.equal((await value('data-cloud-gpu'))[0],1681);
            await command('Controls Off');await command('Select None');
            assert.deepEqual(await value('data-selected-ids'),[]);assert.equal(await canvasAttribute(page,'data-pick-mode'),'Object');
            assert.deepEqual(await value('data-cloud-sources'),before);assert.equal((await value('data-cloud-gpu'))[0],1681);
            semantic.cloudSources=before;semantic.cloudGpu=await value('data-cloud-gpu');semantic.selectedIds=[];
        }else if(p.recipe.kind==='unchanged-export'){
            const fn=loaded.exports[p.recipe.exportName];assert.equal(typeof fn,'function');
            if(kind==='shared-view')await fn(page,address,helpers,json(path.join(p.privateDocs,'journey/checks/browser/shared-view-original.json')).archive);
            else if(kind==='source-pens')await fn(page,address,helpers);
            else await fn(page,address,2,helpers);
            semantic.originalExportReturned=true;
        }else if(kind==='current-commands'){
            const source=read(privateFile),marker="    for(const line of ['Close','CPlane XY','View Top','Line w-2,0,0 w2,0,0'",start=source.indexOf(marker);assert(start>0);assert.equal(source.indexOf(marker,start+1),-1);
            const tail=source.slice(start,source.lastIndexOf('};'));
            const value=(name:string)=>page.locator('canvas').getAttribute(name).then(JSON.parse),canvas=await page.locator('canvas').boundingBox();assert(canvas);
            await(new Function('page','helpers','assert','value','canvas','return(async()=>{'+tail+'})()'))(page,helpers,assert,value,canvas);
        }else if(kind==='caps'){
            for(const line of ['Close','View Grid Off','Example CAD','Clipping Plane XY 0,0,0','Clipping Plane Solid','View Orthographic','View Top','Fit','Select None'])await command(line);
            await page.waitForFunction(()=>{const c=JSON.parse(document.querySelector('canvas')!.getAttribute('data-clipping')!);return c.caps[1]===1&&c.usage[2]===1&&c.caps[0]>24&&c.cap_error===null;});
            assert.equal(JSON.parse(await page.locator('canvas').getAttribute('data-cads')).length,1);
            assert(Number(await page.locator('canvas').getAttribute('data-object-count'))>=2);semantic.clipping=JSON.parse(await page.locator('canvas').getAttribute('data-clipping'));
        }else if(kind==='performance'){
            const raw=await page.evaluate(async()=>{const r=await fetch('assets/pb/course-comparison.pb');if(!r.ok)throw Error('Missing original comparison fixture');return Array.from(new Uint8Array(await r.arrayBuffer()));});
            assert.equal(hash(Buffer.from(raw)),sha(path.join(p.dist,'assets/pb/course-comparison.pb')));
            await command('Close');const chooser=page.waitForEvent('filechooser');await command('Open');await(await chooser).setFiles({name:'comparison.pb',mimeType:'application/octet-stream',buffer:Buffer.from(raw)});
            await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas')!.getAttribute('data-cads')!).length===4);
            for(const line of ['Layers Off','View Grid Off','View Isometric','Opacity 1','View Outline On','Fit'])await command(line);
            assert(Number(await page.locator('canvas').getAttribute('data-object-count'))>=4);semantic.fixtureSha256=hash(Buffer.from(raw));semantic.cadCount=4;
        }else if(kind==='hardware-grid'){
            const source=read(privateFile),start=source.indexOf('            const url = new URL(original.url());'),end=source.indexOf('            const emptyGrid =');assert(start>0&&end>start);
            const body=source.slice(start,end),original={url:()=>address};
            await(new Function('page','helpers','assert','PNG','require','original','return(async()=>{'+body+'})()'))(page,helpers,assert,originalRequire('pngjs').PNG,Module.createRequire(privateFile),original);
        }
        await page.evaluate(()=>new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r))));
        const canvas=page.locator('canvas'),camera=JSON.parse(await canvas.getAttribute('data-camera-matrix'));assert.equal(camera.length,16);assert(camera.every(Number.isFinite));
        const size=await canvas.evaluate((c:any)=>{const r=c.getBoundingClientRect();return {width:c.width,height:c.height,cssWidth:r.width,cssHeight:r.height,dpr:devicePixelRatio};});
        assert.equal(size.dpr,2);assert.equal(size.width,Math.round(size.cssWidth*2));assert.equal(size.height,Math.round(size.cssHeight*2));
        await(new Function('page','assert','return(async()=>{'+validateGpu+'})()'))(page,assert);assert.deepEqual(errors,[]);
        const drawing=await helpers.drawing(page),file=path.join(p.folder,p.id+'-dpr2.png'),bytes=await page.screenshot({path:file,scale:'device'});
        assert.equal(bytes.readUInt32BE(16),p.viewport.width*2);assert.equal(bytes.readUInt32BE(20),p.viewport.height*2);assert.equal(await helpers.drawing(page),drawing);
        if(p.variant==='unselected-rgb'){const image=originalRequire('pngjs').PNG.sync.read(bytes),colors=new Set<string>();let chromatic=0;for(let i=0;i<image.data.length;i+=4){const rgb=[...image.data.subarray(i,i+3)];if(Math.max(...rgb)-Math.min(...rgb)>24){chromatic++;colors.add(rgb.join(','));}}assert(chromatic>10&&colors.size>8,'The actual source colors must remain visible without selection overlay');semantic.chromaticPixels=chromatic;semantic.distinctChromaticColors=colors.size;}
        write(path.join(p.folder,'worker-result.json'),JSON.stringify({status:'PRESENTATION_ONLY_AWAITING_ROOT_VISUAL_REVIEW',canonicalBehaviorExecuted:false,source:p.source,recipe:p.recipe,errors,semantic,camera,physicalCanvas:size,drawingSha256:drawing,image:{path:file,sha256:hash(bytes),width:p.viewport.width*2,height:p.viewport.height*2}},null,2)+'\n');
    }finally{await browser.close();}
}

export async function runExportedScene(file:string){const p=json(file);check(p);const output=path.join(p.folder,'capture.json');if(fs.existsSync(output)){const r=json(output);assert.equal(sha(r.image.path),r.image.sha256);return r;}const host=await privateImageBrowserHost(p.workspace,p.folder,self,p);check(p);const result={...p,...json(path.join(p.folder,'worker-result.json')),host,resources:host.resources};write(output,JSON.stringify(result,null,2)+'\n');return result;}
const canvasAttribute=(page:any,name:string)=>page.locator('canvas').getAttribute(name);
export async function captureExportedScene(id:string,variant=''){const built=buildImageBrowser(id,2),p=prepareExportedScene(id,path.join(built.folder,'prepared.json'),variant);return runExportedScene(path.join(p.folder,'prepared.json'));}
if(process.argv[2]==='--worker')await worker(json(process.argv[3]));
else if(process.argv[1]&&path.resolve(process.argv[1])===self){const[mode,id,file]=process.argv.slice(2);const result=mode==='recipe'?exportedImageRecipe(id):mode==='prepare'?prepareExportedScene(id,file):mode==='run'?await runExportedScene(id):mode==='capture'?await captureExportedScene(id):mode==='capture-rgb'?await captureExportedScene(id,'unselected-rgb'):null;assert(result,'recipe ID | prepare ID BUILD_JSON | run PREPARED_JSON | capture ID | capture-rgb ID');console.log(JSON.stringify(result,null,2));}
