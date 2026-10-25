import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath, pathToFileURL} from 'node:url';
import {course, docs, hash, read, viewer, write} from './model.ts';

const self=fileURLToPath(import.meta.url),core=fileURLToPath(new URL('./exported-browser-image-recipes.ts',import.meta.url));
const sha=(f:string)=>hash(fs.readFileSync(f));
const setups:Record<string,{body:string;authority:string;requires?:string[]}>={
    'round-caps':{body:"await start(page,address);await open(page,'mesh-round-corners',h);",authority:"await start(page,address);await open(page,'mesh-round-corners',h);"},
    'mesh-markers':{body:"await start(page,address);await open(page,'coplanar-center',h);",authority:"await start(page,address);await open(page,'coplanar-center',h);"},
    'mesh-density':{body:"await start(page,address);await open(page,'dense',h);",authority:"await start(page,address);await open(page,'dense',h);"},
    'ambient-inputs':{body:"await start(page,address);await open(page,'single',h);",authority:"await start(page,address);await open(page,'single',h);"},
    'ambient-order':{body:'await start(page,address);await open(page,h);',authority:'await start(page,address);await open(page,h);'},
    'view-presentation':{body:"const url=new URL(address);url.searchParams.set('msaa','1');await contacts.start(page,url.href);await contacts.open(page,'ordinary',h);await page.bringToFront();",authority:"await contacts.start(page,url.href);await contacts.open(page,'ordinary',h);await page.bringToFront();",requires:["url.searchParams.set('msaa','1')"]},
    'cad-wire':{body:"await contacts.start(page,address);await contacts.open(page,'ordinary',h);await page.bringToFront();",authority:"await contacts.start(page,address);await contacts.open(page,'ordinary',h);await page.bringToFront();"},
};
const compact=(s:string)=>s.replace(/\s+/g,'');

export function terminalImageRecipe(id:string){
    const step=course().steps.find(s=>s.id===id);assert(step);if(!step.browser_check)return null;
    const module=path.basename(step.browser_check,'.cjs'),setup=setups[module];if(!setup)return null;
    const file=path.join(docs,step.browser_check),source=read(file);assert(compact(source).includes(compact(setup.authority)),'Exact original final main-page writer required');
    for(const text of setup.requires||[])assert(compact(source).includes(compact(text)));
    return {module,file,sha256:sha(file),setup,viewport:{width:900,height:760},scope:'Only the exact original final main-page start/open calls. Their original functions/assertions remain unchanged; inherited feature matrices and whole canonical callback acceptance are not claimed.'};
}

async function implementation(){
    const source=read(core),changes:{before:string;after:string}[]=[];
    let adapted=source;
    const replace=(before:string,after:string)=>{assert.equal(adapted.split(before).length,2);adapted=adapted.replace(before,after);changes.push({before,after});};
    const start=source.indexOf('const exportsByModule:'),end=source.indexOf('\n};',start)+3;assert(start>0&&end>start);
    const mapping=Object.fromEntries(Object.keys(setups).map(name=>[name,['__presentation',900,760]]));
    replace(source.slice(start,end),'const exportsByModule:Record<string,[string,number,number]>='+JSON.stringify(mapping)+';\nconst terminalSetups='+JSON.stringify(setups)+';');
    replace("assert(text.includes('module.exports.'+exportName+'=' )||text.includes('module.exports.'+exportName+' ='));","assert(text.replace(/\\s+/g,'').includes(terminalSetups[module].authority.replace(/\\s+/g,'')));for(const required of terminalSetups[module].requires||[])assert(text.replace(/\\s+/g,'').includes(required.replace(/\\s+/g,'')));");
    replace("return {module,file,sha256:sha(file),kind:'unchanged-export',exportName,viewport:{width,height},","return {module,file,sha256:sha(file),kind:'unchanged-export',exportName,setup:terminalSetups[module],viewport:{width,height},");
    replace('loaded._compile(read(privateFile),privateFile);',"loaded._compile(read(privateFile)+'\\nmodule.exports.__presentation=async(page,address,density,h)=>{'+p.recipe.setup.body+'};\\n',privateFile);");
    adapted=adapted.replace(/from '(\.\/[^']+)'/g,(before,spec)=>{const after="from '"+new URL(spec,pathToFileURL(core)).href+"'";changes.push({before,after});return after;});
    let reversed=adapted;for(const change of [...changes].reverse())reversed=reversed.replace(change.after,change.before);assert.equal(reversed,source,'The original image worker must reverse exactly');
    const folder=path.join(viewer,'target/course-presentation/terminal-image-helper',hash(JSON.stringify({self:sha(self),core:sha(core),adapted})).slice(0,20)),file=path.join(folder,'terminal-worker.ts');
    if(fs.existsSync(file))assert.equal(read(file),adapted);else write(file,adapted);
    const proof={status:'SOURCE_ONLY_IMAGE_WORKER_ADAPTATION',helper:{path:self,sha256:sha(self)},originalWorker:{path:core,sha256:sha(core)},adaptedWorker:{path:file,sha256:sha(file)},changes,reverseExact:true,scope:'Same image transport/GPU/DPR/source/bundle checks. Only terminal recipe selection and one passive module export for original final start/open calls differ.'};
    const bridge=path.join(folder,'bridge.json');if(!fs.existsSync(bridge))write(bridge,JSON.stringify(proof,null,2)+'\n');
    return {api:await import(pathToFileURL(file).href),bridge};
}

export async function prepareTerminalScene(id:string,builtFile:string){assert(terminalImageRecipe(id));const {api}=await implementation();return api.prepareExportedScene(id,builtFile);}
export async function runTerminalScene(preparedFile:string){const {api,bridge}=await implementation(),r=await api.runExportedScene(preparedFile),result={...r,terminalSourceBridge:{path:bridge,sha256:sha(bridge)}},file=path.join(r.folder,'terminal-capture.json');if(fs.existsSync(file))assert.equal(read(file),JSON.stringify(result,null,2)+'\n');else write(file,JSON.stringify(result,null,2)+'\n');return result;}
if(process.argv[1]&&path.resolve(process.argv[1])===self){const[mode,id,file]=process.argv.slice(2);const result=mode==='recipe'?terminalImageRecipe(id):mode==='prepare'?await prepareTerminalScene(id,file):mode==='run'?await runTerminalScene(id):mode==='source-check'?(await implementation()).bridge:null;assert(result,'recipe ID | prepare ID BUILT_JSON | run PREPARED_JSON | source-check');console.log(JSON.stringify(result,null,2));}
