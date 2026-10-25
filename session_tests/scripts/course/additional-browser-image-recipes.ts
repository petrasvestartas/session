import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import Module from 'node:module';
import {fileURLToPath} from 'node:url';
import {bindings} from './presentation.ts';
import {buildImageBrowser} from './image-browser-build.ts';
import {observeGpu, validateGpu} from './private-browser-host.ts';
import {privateImageBrowserHost} from './parallel-image-browser-host.ts';
import {course, docs, filesIn, hash, json, read, viewer, write} from './model.ts';

const self = fileURLToPath(import.meta.url);
const sha = (file: string) => hash(fs.readFileSync(file));
const primitive = new Set(['path-input', 'marker-input', 'curve-input', 'colour-input', 'grid-input']);
const terminal: Record<string, string[]> = {
    'labels': ['Close','Layers Off','View Grid Off','Example CAD','Example Scene Text','Fit','View Isometric','Select Next','Select Next'],
    'nested-placements': ['Example Nested Instances','Fit','View Isometric','Select Next','Select Next'],
    'freeform': ['Close','CPlane XY','Nurbs Curve Circle w-2,0,0 0.5','Nurbs Curve Ellipse w-0.7,0,0 0.55 0.3','Nurbs Curve Arc Center w0.7,0,0 0.5 270','Nurbs Curve Parabola w1.5,0,0 1 0.5','Curve w-0.5,1,0 w0,1.6,0 w0.5,1,0','View Reset','Fit'],
};

export function additionalImageRecipe(id: string) {
    const step = course().steps.find(step => step.id === id); assert(step);
    if (!step.browser_check) return null;
    const name = path.basename(step.browser_check, '.cjs'), callback = path.join(docs, step.browser_check), text = read(callback);
    if (primitive.has(name) && step.browser_actions?.some((action: any) => action.command === 'Close')) {
        assert(text.includes('newContext(') && text.includes('original.bringToFront()'));
        return {kind: 'documented-original-' + name, callback, callbackSha256: sha(callback), actions: step.browser_actions,
            authority: 'Exact course browser_actions for this lesson, matching the original example commands used by its separate-context checks; no inherited callback execution.'};
    }
    if (terminal[name] && !step.browser_actions?.length) {
        const commands = terminal[name];
        assert(text.includes(JSON.stringify(commands).replaceAll('"', "'")), 'Exact original final main-page setup must remain present');
        if (name === 'nested-placements') {
            assert(text.includes("await helpers.command(page,'Close');assert.deepEqual(await value('data-original-tree-placements'),[])"));
            assert(text.includes("['Close','Layers Off','View Grid Off','Example Nested Instances','Fit','View Isometric']"));
        }
        return {kind: 'original-terminal-' + name, callback, callbackSha256: sha(callback),
            actions: [...(name === 'nested-placements' ? ['Close','Layers Off','View Grid Off'].map(command=>({command})) : []), ...commands.map(command => ({command}))],
            authority: 'Exact final original main-page setup; nested placements restores its declared Layers/Grid viewport settings and immediately preceding document Close. Earlier tests/history roundtrips are not executed.'};
    }
    return null;
}

function check(prepared: any) {
    assert.equal(prepared.helperSha256, sha(self)); assert.deepEqual(prepared.source, bindings(prepared.id));
    assert.deepEqual(prepared.recipe, additionalImageRecipe(prepared.id));
    for (const [file, digest] of Object.entries(prepared.pins)) assert.equal(sha(file), digest, file);
    assert.deepEqual(Object.fromEntries(filesIn(prepared.dist).map(file => [path.relative(prepared.dist, file).split(path.sep).join('/'), sha(file)])), prepared.bundle);
}

export function prepareAdditionalScene(id: string, buildFile: string) {
    buildFile = path.resolve(buildFile);
    const recipe = additionalImageRecipe(id); assert(recipe, 'No reviewed additional recipe for ' + id);
    const built = json(buildFile); assert.equal(built.id, id); assert.deepEqual(built.source, bindings(id));
    assert(['BUILT_NOT_CAPTURED','CAPTURED_AWAITING_ROOT_VISUAL_REVIEW'].includes(built.status));
    const dist = path.join(built.workspace, 'target/course-checks', id, 'dist');
    const bundle = Object.fromEntries(filesIn(dist).map(file => [path.relative(dist, file).split(path.sep).join('/'), sha(file)])); assert.deepEqual(bundle, built.bundle);
    const inputs = [buildFile, path.join(docs,'capture_journey.cjs'), path.join(docs,'journey/course.json'), recipe.callback,
        ...['private-browser-host.ts','parallel-image-browser-host.ts','parallel-image-guard.ps1'].map(name=>fileURLToPath(new URL('./'+name, import.meta.url)))];
    const pins = Object.fromEntries(inputs.map(file => [file,sha(file)]));
    const key = hash(JSON.stringify({source: built.source, recipe, bundle, pins, helper: sha(self)}));
    const folder = path.join(viewer, 'target/course-presentation/additional-scenes', id + '-' + key.slice(0,20));
    const preparedFile = path.join(folder,'prepared.json');
    if(fs.existsSync(preparedFile)) {const existing=json(preparedFile);check(existing);return existing;}
    const prepared = {status:'PREPARED_IMAGE_ONLY_NOT_EXECUTED', id, folder, workspace:built.workspace, dist, bundle, source:built.source,
        recipe, pins, helperSha256:sha(self), viewport:{width:900,height:760}, density:2,
        canonicalBehaviorExecuted:false, imagePolicy:{processors:8,memoryMiB:8192,affinityMask:'0xFF00',lane:'separate-image'},
        scope:'Source-proven lesson image only. All canonical callbacks remain unchanged; inherited behavior, history, failure and child-context checks are not claimed.'};
    write(preparedFile,JSON.stringify(prepared,null,2)+'\n');check(prepared);return prepared;
}

async function worker(prepared: any) {
    const require = Module.createRequire(path.join(docs,'capture_journey.cjs')), {chromium} = require('playwright');
    const browser = await chromium.launch({headless:true,executablePath:process.env.VIEWER_BROWSER_EXECUTABLE,args:JSON.parse(process.env.VIEWER_CHROME_ARGS!)});
    try {
        const context = await browser.newContext({viewport:prepared.viewport,deviceScaleFactor:prepared.density,acceptDownloads:true});
        const page=await context.newPage(),errors:string[]=[];page.on('pageerror',(e:any)=>errors.push(e.message));
        await (new Function('page','return(async()=>{'+observeGpu+'})()'))(page);
        const capture=path.join(docs,'capture_journey.cjs'),original=read(capture),boundary='\nasync function capture()';assert.equal(original.split(boundary).length,2);
        const helpers=new Module(capture);helpers.filename=capture;helpers.paths=Module._nodeModulePaths(path.dirname(capture));
        const previous=process.argv;
        try{process.argv=[previous[0],capture,prepared.id];helpers._compile(original.split(boundary)[0]+'\nmodule.exports={command,drawing};',capture);}finally{process.argv=previous;}
        const command=(line:string)=>helpers.exports.command(page,line);
        await page.goto(process.env.VIEWER_URL);
        await page.waitForFunction(()=>{const w=window as any;return w.wasmBindings?.runtime_running?.()&&JSON.parse(w.wasmBindings.diagnostic_snapshot()).outcome==='ready';});
        await page.waitForFunction(()=>{const c=document.querySelector('canvas')!,r=c.getBoundingClientRect();return devicePixelRatio===2&&c.width===Math.round(r.width*2)&&c.height===Math.round(r.height*2);});
        for(const action of prepared.recipe.actions) {
            if(action.command)await command(action.command);
            else if(action.file){const file=path.join(prepared.workspace,'target','course-'+prepared.id,action.file);assert(fs.existsSync(file));const chooser=page.waitForEvent('filechooser');await command('Open');await(await chooser).setFiles(file);await page.waitForFunction(()=>/^(File imported|Document replaced)/.test(document.querySelector('#status')!.textContent!));if(action.fit)await command('Fit');}
            else throw Error('Unreviewed action in additional image recipe: '+JSON.stringify(action));
        }
        await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));
        const value=async(name:string)=>JSON.parse(await page.locator('canvas').getAttribute(name));
        const kind=prepared.recipe.kind,semantic:any={};
        if(kind.endsWith('path-input')) {semantic.paths=await value('data-path-gpu');assert.equal(semantic.paths[0],3);}
        if(kind.endsWith('marker-input')) {semantic.points=await value('data-point-gpu');assert.equal(semantic.points[0],1);}
        if(kind.endsWith('curve-input')) {semantic.curves=await value('data-curve-cpu');semantic.controls=await value('data-control-count');assert.equal(semantic.curves[0],1);assert.equal(semantic.controls,3);}
        if(kind.endsWith('colour-input')) {semantic.usage=await value('data-gpu-usage');assert(semantic.usage.some((v:number)=>v>0));}
        if(kind.endsWith('grid-input')) {semantic.gridEnabled=await value('data-grid-enabled');semantic.grid=await value('data-grid-gpu');assert.equal(semantic.gridEnabled,true);assert(semantic.grid.some((v:number)=>v>0));}
        if(kind.endsWith('labels')) {semantic.texts=await value('data-scene-texts');semantic.selected=await value('data-selected-ids');assert.equal(semantic.texts.length,1);assert.deepEqual(semantic.selected,[semantic.texts[0].id]);}
        if(kind.endsWith('nested-placements')) {semantic.instances=await value('data-instances');semantic.placements=await value('data-original-tree-placements');assert.equal(semantic.instances.length,2);assert(semantic.placements.some((row:any)=>row.name==='mirror-branch'));}
        if(kind.endsWith('freeform')) {semantic.curves=await value('data-curve-originals');assert.equal(semantic.curves.length,5);}
        const camera=await value('data-camera-matrix');assert.equal(camera.length,16);assert(camera.every(Number.isFinite));semantic.camera=camera;
        const status=await page.locator('#status').textContent();assert(!/^(Cannot|Error|Invalid|Failed)/.test(status));
        await (new Function('page','assert','return(async()=>{'+validateGpu+'})()'))(page,assert);assert.deepEqual(errors,[]);
        const drawingSha256=await helpers.exports.drawing(page),file=path.join(prepared.folder,prepared.id+'-dpr2.png');
        const bytes=await page.screenshot({path:file,scale:'device'});assert.equal(bytes.readUInt32BE(16),1800);assert.equal(bytes.readUInt32BE(20),1520);
        assert.equal(await helpers.exports.drawing(page),drawingSha256,'Passive screenshot preserves the actual scene');
        write(path.join(prepared.folder,'worker-result.json'),JSON.stringify({status:'PRESENTATION_ONLY_AWAITING_ROOT_VISUAL_REVIEW',source:prepared.source,recipe:prepared.recipe,
            canonicalBehaviorExecuted:false,helperSha256:prepared.helperSha256,semantic,statusText:status,errors,drawingSha256,
            image:{path:file,sha256:hash(bytes),width:1800,height:1520}},null,2)+'\n');
    } finally {await browser.close();}
}

export async function runAdditionalScene(preparedFile:string) {
    const prepared=json(preparedFile);check(prepared);const file=path.join(prepared.folder,'capture.json');
    if(fs.existsSync(file)){const retained=json(file);assert.equal(sha(retained.image.path),retained.image.sha256);return retained;}
    const host=await privateImageBrowserHost(prepared.workspace,prepared.folder,self,prepared);check(prepared);
    const result={...prepared,...json(path.join(prepared.folder,'worker-result.json')),host,resources:host.resources};
    write(file,JSON.stringify(result,null,2)+'\n');return result;
}

export async function captureAdditionalScene(id:string) {
    const built=buildImageBrowser(id,2),prepared=prepareAdditionalScene(id,path.join(built.folder,'prepared.json'));
    return runAdditionalScene(path.join(prepared.folder,'prepared.json'));
}

if(process.argv[2]==='--worker')await worker(json(process.argv[3]));
else if(process.argv[1]&&path.resolve(process.argv[1])===self){
    const [mode,id,arg]=process.argv.slice(2);
    const result=mode==='recipe'?additionalImageRecipe(id):mode==='prepare'?prepareAdditionalScene(id,arg):mode==='run'?await runAdditionalScene(id):mode==='capture'?await captureAdditionalScene(id):null;
    assert(result,'additional-browser-image-recipes.ts recipe ID | prepare ID BUILT_JSON | run PREPARED_JSON | capture ID');console.log(JSON.stringify(result,null,2));
}
