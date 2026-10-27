import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath, pathToFileURL} from 'node:url';
import {buildIncrementalImageBrowser} from './incremental-browser-image-build.ts';
import {sceneRecipe} from './browser-scene-presentation.ts';
import {prepareImageLaneScene, runImageLaneScene} from './image-lane-browser-scene.ts';
import {additionalImageRecipe, prepareAdditionalScene, runAdditionalScene} from './additional-browser-image-recipes.ts';
import {exportedImageRecipe, prepareExportedScene, runExportedScene} from './exported-browser-image-recipes.ts';
import {historicalImageRecipe, prepareHistoricalScene, runHistoricalScene} from './historical-browser-actions-presentation.ts';
import {terminalImageRecipe, prepareTerminalScene, runTerminalScene} from './terminal-browser-image-recipes-v2.ts';
import {childContextImageRecipe,prepareChildContextScene,runChildContextScene} from './child-context-browser-image-recipes.ts';
import {bindings} from './presentation.ts';
import {course, docs, filesIn, hash, json, viewer, withExpectedSequence, write} from './model.ts';

const self=fileURLToPath(import.meta.url),sha=(f:string)=>hash(fs.readFileSync(f));
const ref=(f:string)=>({path:path.resolve(f),sha256:sha(path.resolve(f))});
const done=(r:any)=>['CAPTURED_AWAITING_ROOT_VISUAL_REVIEW','PUBLISHED_ROOT_REVIEWED'].includes(r?.status);
const modules=['incremental-browser-image-build.ts','browser-scene-presentation.ts','image-lane-browser-scene.ts','additional-browser-image-recipes.ts','exported-browser-image-recipes.ts','historical-browser-actions-presentation.ts','terminal-browser-image-recipes-v2.ts','child-context-browser-image-recipes.ts','maintained-browser-presentation.ts','browser-image-fallback.ts','parallel-image-browser-host.ts','parallel-image-guard.ps1'];

function imageCheck(image:any){const bytes=fs.readFileSync(image.path);assert.equal(hash(bytes),image.sha256);assert.equal(bytes.readUInt32BE(16),image.width);assert.equal(bytes.readUInt32BE(20),image.height);assert(image.width>=1280&&image.height>=960);}
function actualRecord(id:string,file:string,source?:any){
    const r=json(file);assert.deepEqual(r.source,source||withExpectedSequence(()=>bindings(id)));imageCheck(r.image);
    const resources=r.resources||r.host?.resources;assert(resources);assert.equal(resources.exitCode,0);assert.equal(resources.timedOut,false);
    if(r.host?.resourcesFile)assert.equal(sha(r.host.resourcesFile),r.host.resourcesSha256);
    if(r.host?.log)assert.equal(sha(r.host.log),r.host.logSha256);
    return r;
}
function check(c:any){
    assert.equal(sha(c.originalCohort.path),c.originalCohort.sha256);assert.equal(sha(c.incrementalReview.path),c.incrementalReview.sha256);
    for(const[f,p]of Object.entries(c.inputs)as[string,any][]){const s=fs.statSync(f);if(s.size!==p.size||s.mtimeMs!==p.mtimeMs)assert.equal(sha(f),p.sha256,f);}
    for(const[root,names]of Object.entries(c.sourceFileLists))assert.deepEqual(filesIn(root).sort(),names,root);
}
function summary(p:any){const rows=Object.values(p.items)as any[];p.completed=rows.filter(done).length;p.remaining=p.total-p.completed;p.primaryCompleted=rows.filter(r=>r.primary&&done(r)).length;p.primaryRemaining=p.primaryTotal-p.primaryCompleted;p.published=rows.filter(r=>r.status==='PUBLISHED_ROOT_REVIEWED').length;p.updated=new Date().toISOString();return p;}

export function prepareIncrementalBrowserBatch(originalFile:string,reviewFile:string,qualificationFile=''){
    originalFile=path.resolve(originalFile);reviewFile=path.resolve(reviewFile);const original=json(originalFile),decision=json(reviewFile);
    assert.equal(original.total,379);assert.equal(original.primaryTotal,36);assert(decision.status.startsWith('APPROVED'));assert.equal(decision.helper.sha256,sha(fileURLToPath(new URL('./incremental-browser-image-build.ts',import.meta.url))));
    assert.deepEqual(decision.cases.map((r:any)=>r.id),['35d-input','36e-input']);for(const row of decision.cases){assert(row.exactOriginalPngBytes);assert.equal(row.differentPixels,0);assert.equal(sha(row.actualBuild.path),row.actualBuild.sha256);assert.equal(sha(row.actualImageCapture.path),row.actualImageCapture.sha256);actualRecord(row.id,row.actualImageCapture.path);}
    const sources:any={},queue=withExpectedSequence(()=>[...original.queue].sort((a:any,b:any)=>a.index-b.index).map((r:any)=>{
        const step=course().steps.find(s=>s.id===r.id),generic=sceneRecipe(r.id,step),extra=original.additionalReview&&additionalImageRecipe(r.id),exported=exportedImageRecipe(r.id),historical=historicalImageRecipe(r.id),terminal=terminalImageRecipe(r.id),child=childContextImageRecipe(r.id);sources[r.id]=bindings(r.id);
        const route=r.id==='97-finish-with-the-maintained-browser-runtime'?'retained-maintained':r.id==='68ac-connect-current-pages-to-the-browser-renderer'?'retained-streaming':generic.kind!=='full-canonical-fallback'?'generic':extra?'additional':exported?'exported':terminal?'terminal':child?'child-context':historical?'historical':'fallback';
        return {...r,plannedRoute:route,recipeContract:route==='generic'?generic:route==='additional'?extra:route==='exported'?exported:route==='terminal'?terminal:route==='child-context'?child:route==='historical'?historical:null};
    })).sort((a:any,b:any)=>Number(b.primary)-Number(a.primary)||a.index-b.index);
    const qualification=qualificationFile?json(path.resolve(qualificationFile)):null;
    if(qualification){assert.equal(qualification.status,'APPROVED exact archived typing-reference relevance; not unchanged reference bytes');assert.equal(sha(qualification.sourceBaseline.path),qualification.sourceBaseline.sha256);assert.equal(sha(qualification.actualReferenceReview.path),qualification.actualReferenceReview.sha256);const baseline=json(qualification.sourceBaseline.path);assert.equal(Object.keys(baseline.sources).length,379);assert.deepEqual(sources,baseline.sources,'All379 source bindings must remain exact despite archived-reference import correction');assert.equal(qualification.changes.length,1);assert.equal(qualification.changes[0].path,path.join(docs,'typing/manifest.json'));}
    const inputs:any={},resolvedOriginalInputs:any[]=[];
    for(const[file,value]of Object.entries(original.inputs)as[string,any][]){const candidates=path.isAbsolute(file)?[file]:[path.resolve(viewer,file),path.resolve(viewer,'..',file)],found=[...new Set(candidates)].filter(fs.existsSync);assert(found.length,'Original input path missing: '+file);let matching=found.filter(f=>sha(f)===value.sha256);let acceptedSha=value.sha256;if(!matching.length&&qualification){const q=qualification.changes.find((r:any)=>found.includes(r.path)&&r.beforeSha256===value.sha256);if(q){assert.equal(sha(q.path),q.afterSha256);matching=[q.path];acceptedSha=q.afterSha256;}}assert.equal(matching.length,1,'Original relative input requires one exact frozen or source-qualified hash: '+file);const target=matching[0],s=fs.statSync(target);inputs[target]={sha256:acceptedSha,size:s.size,mtimeMs:s.mtimeMs};if(target!==file)resolvedOriginalInputs.push({original:file,resolved:target,sha256:value.sha256});}
    for(const f of[self,reviewFile,...(qualificationFile?[path.resolve(qualificationFile)]:[]),...modules.map(m=>fileURLToPath(new URL('./'+m,import.meta.url)))]){const s=fs.statSync(f);inputs[f]={sha256:sha(f),size:s.size,mtimeMs:s.mtimeMs};}
    const identity={originalCohort:ref(originalFile),incrementalReview:ref(reviewFile),queue,sources,inputs,resolvedOriginalInputs,inputQualification:qualificationFile?ref(qualificationFile):null,sourceFileLists:original.sourceFileLists};
    const key=hash(JSON.stringify(identity)),directory=path.join(viewer,'target/course-presentation/incremental-browser-batches',key.slice(0,20)),file=path.join(directory,'cohort.json');
    if(fs.existsSync(file)){const c=json(file);check(c);return c;}
    const c={status:'PREPARED_NO_NEW_CAPTURE',key,directory,density:2,total:379,primaryTotal:36,...identity,scope:'Independent resumable successor. Existing source-bound executions keep their exact original profile/checker scope; new actual builds use approved stable incremental image compiler. No original ledger or frozen helper changes.'};
    check(c);
    const prior=json(path.join(path.dirname(originalFile),'progress.json')),items:any={};
    for(const item of queue){const row=prior.items[item.id];if(done(row)){const file=row.sourceRecord.path||row.sourceRecord;assert.equal(sha(file),row.sourceRecord.sha256||row.sourceRecordSha256);actualRecord(item.id,file,sources[item.id]);items[item.id]={...row,plannedRoute:item.plannedRoute,reusedOriginalScope:true};}else items[item.id]={...item,status:'PENDING'};}
    const publicationFile=path.join(docs,'journey/release/image-quality-publication-index.json');
    if(fs.existsSync(publicationFile))for(const published of json(publicationFile).images||[]){
        const item=queue.find((q:any)=>q.path===published.path);if(!item||done(items[item.id]))continue;
        assert.equal(sha(published.path),published.sha256);assert.equal(sha(published.receipt.path),published.receipt.sha256);
        const review=json(published.receipt.path),entry=(review.images||[review.image]).find((r:any)=>(r.destination||r.publicPath||r.path)===item.path),candidate=entry?.sourceRecord||review.sourceRecord;
        const file=typeof candidate==='string'?candidate:candidate?.path;if(!file||!fs.existsSync(file))continue;
        const r=json(file);if(!r.image||r.image.sha256!==published.sha256)continue;
        actualRecord(item.id,file,sources[item.id]);items[item.id]={...item,status:'PUBLISHED_ROOT_REVIEWED',sourceRecord:ref(file),source:r.source,image:r.image,reusedOriginalScope:true,rootReview:published.receipt,canonicalBehaviorExecuted:r.canonicalBehaviorExecuted,canonicalValidationProfile:r.canonicalValidationProfile};
    }
    write(path.join(directory,'progress.json'),JSON.stringify(summary({total:379,primaryTotal:36,items,originalProgress:path.join(path.dirname(originalFile),'progress.json')}),null,2)+'\n');write(file,JSON.stringify(c,null,2)+'\n');return c;
}

async function incrementalFallback(id:string,built:any){
    const original=fileURLToPath(new URL('./browser-image-fallback.ts',import.meta.url)),source=fs.readFileSync(original,'utf8');
    const before='const built = withExpectedSequence(() => buildImageBrowser(id,2)); check(built);';assert.equal(source.split(before).length,2);
    const after=`const built = json(${JSON.stringify(built.preparedFile)}); assert.equal(built.id,id); check(built);`;
    const importChanges:any[]=[];let adapted=source.replace(before,after).replace(/from '(\.\/[^']+)'/g,(match,spec)=>{const replacement="from '"+new URL(spec,pathToFileURL(original)).href+"'";importChanges.push({match,replacement});return replacement;});
    adapted=adapted.replace(/new URL\('([^']+)',\s*import\.meta\.url\)/g,(match,spec)=>{const replacement='new URL('+JSON.stringify(new URL(spec,pathToFileURL(original)).href)+')';importChanges.push({match,replacement});return replacement;});
    let reversed=adapted;for(const p of importChanges)reversed=reversed.replace(p.replacement,p.match);assert.equal(reversed.replace(after,before),source,'Only source-bound builder handoff/import relocation may differ');
    const folder=path.join(built.folder,'unchanged-fallback-adapter'),file=path.join(folder,'adapter.ts');
    if(fs.existsSync(file))assert.equal(fs.readFileSync(file,'utf8'),adapted);else write(file,adapted);
    const bridge={original:ref(original),adapted:ref(file),built:ref(built.preparedFile),before,after,importChanges,reverseExact:true,scope:'Only the image-build acquisition is replaced by this actual incremental build record. Original host, callback worker, source checks, input, assertions, timing and presentation are byte-preserved.'};
    const bridgeFile=path.join(folder,'bridge.json');if(!fs.existsSync(bridgeFile))write(bridgeFile,JSON.stringify(bridge,null,2)+'\n');else assert.deepEqual(json(bridgeFile),bridge);
    const adapter=await import(pathToFileURL(file).href),r=await adapter.captureFallback(id),originalRecord=path.join(r.folder,'capture.json'),result={...r,incrementalBuild:built.imageBuild,sourceOnlyFallbackAdapter:ref(bridgeFile),originalAdaptedRecord:ref(originalRecord)};
    const output=path.join(folder,'capture.json');if(!fs.existsSync(output))write(output,JSON.stringify(result,null,2)+'\n');return output;
}

async function incrementalMaintained(built:any){
    const original=fileURLToPath(new URL('./maintained-browser-presentation.ts',import.meta.url)),source=fs.readFileSync(original,'utf8');
    const edits=[
        ["import {observeGpu, privateBrowserHost, validateGpu} from './private-browser-host.ts';","import {observeGpu, validateGpu} from './private-browser-host.ts';\nimport {privateImageBrowserHost as privateBrowserHost} from './parallel-image-browser-host.ts';"],
        ['const built = buildBrowser(id, 2), clone = maintainedClone(), source = bindings(id);',`const built = json(${JSON.stringify(built.preparedFile)}), clone = maintainedClone(), source = bindings(id);`],
        ["{id, originalFile, adapted: path.join(folder, 'capture-adapted.cjs')}","{id, originalFile, adapted: path.join(folder, 'capture-adapted.cjs'), canonicalBehaviorExecuted: false}"],
    ];
    let adapted=source;for(const[before,after]of edits){assert.equal(adapted.split(before).length,2);adapted=adapted.replace(before,after);}
    const imports:any[]=[];adapted=adapted.replace(/from '(\.\/[^']+)'/g,(match,spec)=>{const replacement="from '"+new URL(spec,pathToFileURL(original)).href+"'";imports.push({match,replacement});return replacement;});
    adapted=adapted.replace(/new URL\('([^']+)',\s*import\.meta\.url\)/g,(match,spec)=>{const replacement='new URL('+JSON.stringify(new URL(spec,pathToFileURL(original)).href)+')';imports.push({match,replacement});return replacement;});
    let reversed=adapted;for(const p of imports)reversed=reversed.replace(p.replacement,p.match);for(const[before,after]of [...edits].reverse())reversed=reversed.replace(after,before);assert.equal(reversed,source);
    const folder=path.join(built.folder,'maintained-image-lane-adapter'),file=path.join(folder,'adapter.ts');if(fs.existsSync(file))assert.equal(fs.readFileSync(file,'utf8'),adapted);else write(file,adapted);
    const bridge={original:ref(original),adapted:ref(file),built:ref(built.preparedFile),edits,imports,reverseExact:true,scope:'Only source-bound build acquisition, image-only resource host and import relocation change. Original maintained DPR1/2 inputs and all assertions are byte-preserved; this does not claim the original validation resource profile.'},bridgeFile=path.join(folder,'bridge.json');if(fs.existsSync(bridgeFile))assert.deepEqual(json(bridgeFile),bridge);else write(bridgeFile,JSON.stringify(bridge,null,2)+'\n');
    const adapter=await import(pathToFileURL(file).href),r=await adapter.runMaintainedPresentation(),result={...r,canonicalBehaviorExecuted:true,canonicalValidationProfile:false,incrementalBuild:built.imageBuild,sourceOnlyImageAdapter:ref(bridgeFile)};
    const output=path.join(folder,'capture.json');if(fs.existsSync(output))assert.deepEqual(json(output),result);else write(output,JSON.stringify(result,null,2)+'\n');return output;
}

function recipeGate(file:string,cohort:any,id:string,kind='exported'){
    assert(file,'An actual image representative/root-review gate is required before batch adoption');const gate=json(path.resolve(file));assert.equal(gate.rootPersonallyViewedOriginalResolution,true);
    const helper=kind==='historical'?'historical-browser-actions-presentation.ts':kind==='terminal'?'terminal-browser-image-recipes-v2.ts':kind==='child-context'?'child-context-browser-image-recipes.ts':'exported-browser-image-recipes.ts';
    assert.equal(gate.helperSha256,cohort.inputs[fileURLToPath(new URL('./'+helper,import.meta.url))].sha256);assert(gate.approvedIds.includes(id));assert(gate.cases.length);
    for(const row of gate.cases){assert.equal(sha(row.record),row.sha256);const r=actualRecord(row.id,row.record,cohort.sources[row.id]);assert.equal(r.image.sha256,row.reviewedImageSha256);assert.equal(r.canonicalBehaviorExecuted,false);}return ref(file);
}

export async function runIncrementalBrowserBatch(cohortFile:string,scope='all',reviewFile='',limit=0,historicalReview='',terminalReview='',childReview=''){
    assert(['all','primary','exported'].includes(scope));const c=json(path.resolve(cohortFile));check(c);const progressFile=path.join(c.directory,'progress.json'),p=json(progressFile);
    let count=0;
    for(const item of c.queue){
        if(scope==='primary'&&!item.primary||scope==='exported'&&item.plannedRoute!=='exported')continue;
        const old=p.items[item.id];if(done(old)){assert.equal(sha(old.sourceRecord.path),old.sourceRecord.sha256);actualRecord(item.id,old.sourceRecord.path,c.sources[item.id]);continue;}
        assert(!['FAILED','RUNNING'].includes(old.status),'Preserve previous incomplete/failed attempt for explicit diagnosis: '+item.id);if(limit&&count>=limit)break;
        check(c);const approval=item.plannedRoute==='exported'?recipeGate(reviewFile,c,item.id):item.plannedRoute==='historical'?recipeGate(historicalReview,c,item.id,'historical'):item.plannedRoute==='terminal'?recipeGate(terminalReview,c,item.id,'terminal'):item.plannedRoute==='child-context'?recipeGate(childReview,c,item.id,'child-context'):null;
        assert(item.plannedRoute!=='retained-streaming','Retain the already accepted special streaming record before running this successor');
        p.items[item.id]={...item,status:'RUNNING',started:new Date().toISOString()};write(progressFile,JSON.stringify(summary(p),null,2)+'\n');
        try{
            const built=withExpectedSequence(()=>buildIncrementalImageBrowser(item.id,2));let file:string;
            if(item.plannedRoute==='retained-maintained')file=await incrementalMaintained(built);
            else if(item.plannedRoute==='generic'){const q=prepareImageLaneScene(item.id,built.preparedFile);await runImageLaneScene(path.join(q.directory,'prepared.json'));file=path.join(q.directory,'result.json');}
            else if(item.plannedRoute==='additional'){const q=prepareAdditionalScene(item.id,built.preparedFile);await runAdditionalScene(path.join(q.folder,'prepared.json'));file=path.join(q.folder,'capture.json');}
            else if(item.plannedRoute==='exported'){const q=prepareExportedScene(item.id,built.preparedFile);await runExportedScene(path.join(q.folder,'prepared.json'));file=path.join(q.folder,'capture.json');}
            else if(item.plannedRoute==='historical'){const q=prepareHistoricalScene(item.id,built.preparedFile);await runHistoricalScene(path.join(q.folder,'prepared.json'));file=path.join(q.folder,'capture.json');}
            else if(item.plannedRoute==='terminal'){const q=await prepareTerminalScene(item.id,built.preparedFile);await runTerminalScene(path.join(q.folder,'prepared.json'));file=path.join(q.folder,'terminal-capture.json');}
            else if(item.plannedRoute==='child-context'){const q=await prepareChildContextScene(item.id,built.preparedFile);await runChildContextScene(path.join(q.folder,'child-context-prepared.json'));file=path.join(q.folder,'child-context-capture.json');}
            else file=await incrementalFallback(item.id,built);
            check(c);const r=actualRecord(item.id,file,c.sources[item.id]);p.items[item.id]={...item,status:'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW',sourceRecord:ref(file),source:r.source,image:r.image,build:ref(built.preparedFile),buildMethod:built.imageBuild,
                approval,canonicalBehaviorExecuted:r.canonicalBehaviorExecuted,canonicalValidationProfile:r.canonicalValidationProfile??false,behaviorScope:r.scope||'Image-only exact recipe; no inherited callback acceptance',elapsedSeconds:r.resources?.elapsedSeconds??r.host?.resources?.elapsedSeconds};count++;
        }catch(error:any){p.items[item.id]={...p.items[item.id],status:'FAILED',error:String(error.stack||error),finished:new Date().toISOString()};write(progressFile,JSON.stringify(summary(p),null,2)+'\n');throw error;}
        write(progressFile,JSON.stringify(summary(p),null,2)+'\n');
    }return summary(p);
}

export function prepareBrowserCopyPlan(cohortFile:string,reviewFile:string){
    const c=json(path.resolve(cohortFile)),p=json(path.join(c.directory,'progress.json')),review=json(path.resolve(reviewFile));check(c);assert.equal(review.rootPersonallyViewedOriginalResolution,true);assert(review.images.length);
    const copies=review.images.map((approved:any)=>{const row=p.items[approved.id];assert(done(row));assert.equal(row.image.sha256,approved.sha256);const r=actualRecord(row.id,row.sourceRecord.path),target=path.resolve(viewer,row.path);assert(target.startsWith(docs+path.sep));return {id:row.id,from:r.image.path,to:target,sha256:r.image.sha256,expectedDestinationSha256:row.originalSha256,sourceRecord:row.sourceRecord,rootReview:ref(reviewFile)};});
    const result={status:'PREPARED_ROOT_REVIEWED_COPYPLAN_NO_TRACKED_WRITES',cohort:ref(cohortFile),copies};const file=path.join(c.directory,'copyplan-'+hash(JSON.stringify(result)).slice(0,16)+'.json');write(file,JSON.stringify(result,null,2)+'\n');return {file,...result};
}

export function refreshBrowserPublications(cohortFile:string){
    const c=json(path.resolve(cohortFile));check(c);const file=path.join(c.directory,'progress.json'),p=json(file),index=json(path.join(docs,'journey/release/image-quality-publication-index.json'));
    for(const published of index.images||[]){
        const item=c.queue.find((q:any)=>q.path===published.path);if(!item||p.items[item.id]?.status==='PUBLISHED_ROOT_REVIEWED')continue;
        assert.equal(sha(published.path),published.sha256);assert.equal(sha(published.receipt.path),published.receipt.sha256);
        const review=json(published.receipt.path),entry=(review.images||[review.image]).find((r:any)=>(r?.destination||r?.publicPath||r?.path)===item.path),candidate=entry?.sourceRecord||review.sourceRecord,record=typeof candidate==='string'?candidate:candidate?.path;
        if(!record||!fs.existsSync(record))continue;const raw=json(record);if(!raw.image||raw.image.sha256!==published.sha256)continue;
        const r=actualRecord(item.id,record,c.sources[item.id]);p.items[item.id]={...item,status:'PUBLISHED_ROOT_REVIEWED',sourceRecord:ref(record),source:r.source,image:r.image,reusedOriginalScope:true,rootReview:published.receipt,canonicalBehaviorExecuted:r.canonicalBehaviorExecuted,canonicalValidationProfile:r.canonicalValidationProfile};
    }
    write(file,JSON.stringify(summary(p),null,2)+'\n');return summary(p);
}

if(process.argv[1]&&path.resolve(process.argv[1])===self){const[mode,...args]=process.argv.slice(2),option=(name:string,fallback='')=>{const at=args.indexOf(name);return at<0?fallback:args[at+1];};const result=mode==='prepare'?prepareIncrementalBrowserBatch(option('--cohort'),option('--incremental-review'),option('--input-qualification')):mode==='run'?await runIncrementalBrowserBatch(option('--cohort'),option('--scope','all'),option('--export-review'),Number(option('--limit','0')),option('--historical-review'),option('--terminal-review'),option('--child-context-review')):mode==='copyplan'?prepareBrowserCopyPlan(option('--cohort'),option('--review')):mode==='refresh'?refreshBrowserPublications(option('--cohort')):null;assert(result,'prepare --cohort OLD --incremental-review JSON | run --cohort NEW [--scope all|primary|exported] [--export-review JSON] [--historical-review JSON] [--terminal-review JSON] [--child-context-review JSON] [--limit N] | copyplan --cohort NEW --review JSON | refresh --cohort NEW');console.log(JSON.stringify(result,null,2));}
