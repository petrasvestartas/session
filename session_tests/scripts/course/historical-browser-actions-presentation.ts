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
export function historicalImageRecipe(id: string) {
    const step=course().steps.find(step=>step.id===id);assert(step);
    const actions=step.browser_actions;
    if(!actions?.length)return null;
    for(const action of actions)assert(['command','file','viewport','wheel','key','drag','canvas'].some(key=>key in action),'Unknown historical action');
    return {kind:'source-exact-declared-browser-actions',actions,
        callback:step.browser_check||null,declaredMetadataStatus:step.browser_result_status||step.browser_status,
        authority:'Exact browser_actions from this checkpoint. Its documented source illustration is captured; inherited callbacks, current callback terminal scene and whole behavior validation are not executed or claimed.'};
}

function check(prepared: any) {
    assert.equal(prepared.helperSha256, sha(self)); assert.deepEqual(prepared.source, bindings(prepared.id));
    assert.deepEqual(prepared.recipe, historicalImageRecipe(prepared.id));
    for (const [file, digest] of Object.entries(prepared.pins)) assert.equal(sha(file), digest, file);
    assert.deepEqual(Object.fromEntries(filesIn(prepared.dist).map(file => [path.relative(prepared.dist, file).split(path.sep).join('/'), sha(file)])), prepared.bundle);
}

export function prepareHistoricalScene(id: string, buildFile: string) {
    buildFile = path.resolve(buildFile);
    const recipe = historicalImageRecipe(id); assert(recipe, 'No declared browser actions for ' + id);
    const built = json(buildFile); assert.equal(built.id, id); assert.deepEqual(built.source, bindings(id));
    assert(['BUILT_NOT_CAPTURED','CAPTURED_AWAITING_ROOT_VISUAL_REVIEW'].includes(built.status));
    const dist = path.join(built.workspace, 'target/course-checks', id, 'dist');
    const bundle = Object.fromEntries(filesIn(dist).map(file => [path.relative(dist, file).split(path.sep).join('/'), sha(file)])); assert.deepEqual(bundle, built.bundle);
    const inputs = [buildFile, path.join(docs,'capture_journey.cjs'), path.join(docs,'journey/course.json'), ...(recipe.callback?[path.join(docs,recipe.callback)]:[]),
        ...['private-browser-host.ts','parallel-image-browser-host.ts','parallel-image-guard.ps1'].map(name=>fileURLToPath(new URL('./'+name, import.meta.url)))];
    for(const action of recipe.actions)if(action.file){
        const file=path.resolve(built.workspace,'target','course-'+id,action.file);
        assert(file.startsWith(path.resolve(built.workspace,'target','course-'+id)+path.sep));
        assert(fs.existsSync(file),'Use only this source checkpoint actual generated fixture: '+file);inputs.push(file);
    }
    const pins = Object.fromEntries(inputs.map(file => [file,sha(file)]));
    const key = hash(JSON.stringify({source: built.source, recipe, bundle, pins, helper: sha(self)}));
    const folder = path.join(viewer, 'target/course-presentation/historical-action-scenes', id + '-' + key.slice(0,20));
    const preparedFile = path.join(folder,'prepared.json');
    if(fs.existsSync(preparedFile)) {const existing=json(preparedFile);check(existing);return existing;}
    const prepared = {status:'PREPARED_IMAGE_ONLY_NOT_EXECUTED', id, folder, workspace:built.workspace, dist, bundle, source:built.source,
        recipe, pins, helperSha256:sha(self), viewport:{width:900,height:760}, density:2,
        canonicalBehaviorExecuted:false, imagePolicy:{processors:8,memoryMiB:8192,affinityMask:'0xFF00',lane:'separate-image'},
        scope:'Exact checkpoint browser_actions only, with source-bound observed rendering. Inherited callback terminal state and original behavior checks are not claimed.'};
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
        await page.waitForFunction(()=>{const w=window as any;return w.__presentationDevices?.length>0&&!/^Waiting/.test(document.querySelector('#status')?.textContent||'Waiting');});
        await page.waitForFunction(()=>{const c=document.querySelector('canvas')!,r=c.getBoundingClientRect();return devicePixelRatio===2&&c.width===Math.round(r.width*2)&&c.height===Math.round(r.height*2);});
        for(const action of prepared.recipe.actions) {
            if(action.command)await command(action.command);
            else if(action.file){const file=path.join(prepared.workspace,'target','course-'+prepared.id,action.file);assert(fs.existsSync(file));const chooser=page.waitForEvent('filechooser');await command('Open');await(await chooser).setFiles(file);await page.waitForFunction(()=>/^(File imported|Document replaced)/.test(document.querySelector('#status')!.textContent!));if(action.fit)await command('Fit');}
            else if(action.viewport)await page.setViewportSize({width:action.viewport[0],height:action.viewport[1]});
            else if(action.wheel!==undefined){await page.locator('canvas').hover();await page.mouse.wheel(0,action.wheel);}
            else if(action.key){const b=await page.locator('canvas').boundingBox();await page.mouse.click(b.x+8,b.y+8);await page.keyboard.press(action.key);}
            else if(action.drag){const b=await page.locator('canvas').boundingBox(),[from,to]=action.drag;await page.mouse.move(b.x+from[0]*b.width,b.y+from[1]*b.height);await page.mouse.down({button:'right'});await page.mouse.move(b.x+to[0]*b.width,b.y+to[1]*b.height,{steps:5});await page.mouse.up({button:'right'});}
            else if(action.canvas){const b=await page.locator('canvas').boundingBox();await page.locator('canvas').click({position:{x:action.canvas[0]*b.width,y:action.canvas[1]*b.height}});}
            else throw Error('Unknown declared action');
            await page.waitForTimeout(200);
        }
        await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));
        await page.waitForFunction(()=>{const c=document.querySelector('canvas')!,r=c.getBoundingClientRect();return devicePixelRatio===2&&c.width===Math.round(r.width*2)&&c.height===Math.round(r.height*2);});
        const semantic:any=await page.evaluate(()=>{
            const canvas=document.querySelector('canvas')!;const names=['data-command-ui','data-camera-matrix','data-row-placements','data-selected-ids','data-gpu-stats','data-object-ids'];
            return{attributes:Object.fromEntries(names.filter(name=>canvas.hasAttribute(name)).map(name=>[name,JSON.parse(canvas.getAttribute(name)!)]))};
        });
        const camera=semantic.attributes['data-camera-matrix'];
        if(camera!==undefined){assert.equal(camera.length,16);assert(camera.every(Number.isFinite));}
        semantic.cameraEvidence={observed:camera||null,declaredCameraActions:prepared.recipe.actions.filter((action:any)=>action.wheel!==undefined||action.drag||/^(View|Pan|Orbit|Fit)/.test(action.command||'')),
            scope:camera?'Actual finite16-value camera export read from this source.':'This source exposes no camera matrix readback; exact declared navigation commands and visible output are retained without claiming internal camera equality.'};
        const status=await page.locator('#status').textContent();assert(!/^(Cannot|Error|Invalid|Failed|Unknown|Unsupported)/.test(status));
        await (new Function('page','assert','return(async()=>{'+validateGpu+'})()'))(page,assert);assert.deepEqual(errors,[]);
        const drawingSha256=await helpers.exports.drawing(page),file=path.join(prepared.folder,prepared.id+'-dpr2.png');
        const bytes=await page.screenshot({path:file,scale:'device'});const viewport=page.viewportSize();assert.equal(bytes.readUInt32BE(16),viewport.width*2);assert.equal(bytes.readUInt32BE(20),viewport.height*2);
        const {PNG}=require('pngjs'),pixels=PNG.sync.read(bytes);let visiblePixels=0;
        for(let y=0;y<pixels.height-100;y++)for(let x=0;x<pixels.width;x++){const i=(y*pixels.width+x)*4;if(pixels.data[i+3]>0&&765-pixels.data[i]-pixels.data[i+1]-pixels.data[i+2]>40)visiblePixels++;}
        assert(visiblePixels>20,'Actual geometry must be visibly nonempty above the dock');semantic.visiblePixels=visiblePixels;
        assert.equal(await helpers.exports.drawing(page),drawingSha256,'Passive screenshot preserves the actual scene');
        write(path.join(prepared.folder,'worker-result.json'),JSON.stringify({status:'PRESENTATION_ONLY_AWAITING_ROOT_VISUAL_REVIEW',source:prepared.source,recipe:prepared.recipe,
            canonicalBehaviorExecuted:false,originalCallbackExecuted:false,declaredActionsExecuted:true,canonicalTerminalStateClaimed:false,helperSha256:prepared.helperSha256,semantic,statusText:status,errors,drawingSha256,
            image:{path:file,sha256:hash(bytes),width:bytes.readUInt32BE(16),height:bytes.readUInt32BE(20)}},null,2)+'\n');
    } finally {await browser.close();}
}

export async function runHistoricalScene(preparedFile:string) {
    const prepared=json(preparedFile);check(prepared);const file=path.join(prepared.folder,'capture.json');
    if(fs.existsSync(file)){const retained=json(file);assert.equal(sha(retained.image.path),retained.image.sha256);return retained;}
    const host=await privateImageBrowserHost(prepared.workspace,prepared.folder,self,prepared);check(prepared);
    const result={...prepared,...json(path.join(prepared.folder,'worker-result.json')),host,resources:host.resources};
    write(file,JSON.stringify(result,null,2)+'\n');return result;
}

export async function captureHistoricalScene(id:string) {
    const built=buildImageBrowser(id,2),prepared=prepareHistoricalScene(id,path.join(built.folder,'prepared.json'));
    return runHistoricalScene(path.join(prepared.folder,'prepared.json'));
}

if(process.argv[2]==='--worker')await worker(json(process.argv[3]));
else if(process.argv[1]&&path.resolve(process.argv[1])===self){
    const [mode,id,arg]=process.argv.slice(2);
    const result=mode==='recipe'?historicalImageRecipe(id):mode==='prepare'?prepareHistoricalScene(id,arg):mode==='run'?await runHistoricalScene(id):mode==='capture'?await captureHistoricalScene(id):null;
    assert(result,'historical-browser-actions-presentation.ts recipe ID | prepare ID BUILT_JSON | run PREPARED_JSON | capture ID');console.log(JSON.stringify(result,null,2));
}
