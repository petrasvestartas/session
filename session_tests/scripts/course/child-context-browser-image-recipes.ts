import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {course,docs,hash,json,viewer,write} from './model.ts';
const self=fileURLToPath(import.meta.url),sha=(f:string)=>hash(fs.readFileSync(f)),ref=(f:string)=>({path:f,sha256:sha(f)});
const approvedSources:Record<string,string> = {
  "current-command": "a4817cf4a7d4d493011bfd9c630908b729c62f3e3159cc2f5086afd9b55182f0",
  "selection-palette": "d87eff89f46e2b98aeb923e07d65bfeb83813a97d4deb045e90823c13bd3817b",
  "ordinary-slot": "d243782750031dfbf1bc7657a816c0f311d9eb2d7e86723c941fd9e925905f9b",
  "scene-reload": "fc319d6f16a163dc4ea43ec06f1246c2621b5305f05d3882917e4d3a6d5e98d0",
  "record-properties": "6b4eaab16229449b663533b542c4a173a0e1a65c95858f935b9b6ff8953949c5",
  "arctic-background": "29959cded0f310012f59d9ecc174677f1a63277b643079cc1ab4814a055b959c",
  "arena-reclamation": "09a1eb7b73b795d0c651de295ed499375d4e1d683d604475ebf1f89b5311c90b",
  "arena-empty": "35bd918ec1bcb3d93c5e49676342f276c4fc150a533b7089a79a5b17df71ab43",
  "precise-origin": "6e716a70019064910f6f2698830fab67a50a3178bea8d765ce1be6d200a3dc63",
  "precise-origin-wakes": "3e2530f8565613f47b7ff74183865c08941cffea084e3bbec56c7882f6b2d4ea",
  "reversed-depth": "755bda170dc6b9d22b9528fb33d7eb76360da0662ded1cd1aacdec387555b900",
  "camera-defaults": "f8eb177af4a35f00c8926be4a54301634f4fbf674ad7137140d9491ff832b5ef"
};
const callbacks=['precise-origin','precise-origin-wakes','reversed-depth','camera-defaults'];
const names=['current-command','selection-palette','ordinary-slot','scene-reload','record-properties','arctic-background','arena-reclamation','arena-empty',...callbacks];
export function childContextImageRecipe(id:string){const step=course().steps.find(s=>s.id===id);return step&&!step.browser_actions?.length&&callbacks.some(n=>step.browser_check==='journey/checks/browser/'+n+'.cjs')?{kind:'unchanged-original-page-ordinary-slots',callback:step.browser_check}:null;}
function sourceAudit(){
    for(const[name,expected]of Object.entries(approvedSources))assert.equal(sha(path.join(docs,'journey/checks/browser',name+'.cjs')),expected,name);
    const childNames=['scene-reload','arctic-background','arena-reclamation','arena-empty',...callbacks];
    const reads=Object.fromEntries(childNames.map(name=>{const source=fs.readFileSync(path.join(docs,'journey/checks/browser',name+'.cjs'),'utf8');return[name,source.split(/\r?\n/).map((line,i)=>({line:i+1,text:line.trim()})).filter(r=>/\boriginal\s*[.(]/.test(r.text))];}));
    for(const uses of Object.values(reads))for(const use of uses)assert(!/\boriginal\s*\.\s*(?:evaluate|locator|keyboard|mouse|goto|setViewportSize|reload|setInputFiles)\b/.test(use.text),'Unexpected original-page mutation: '+use.text);
    return{status:'PASS exact reviewed source chain',sources:approvedSources,originalPageApiReads:reads,chain:'precise-origin / precise-origin-wakes -> arena-empty -> arena-reclamation -> record-properties -> scene-reload -> ordinary-slot; reversed-depth -> precise-origin-wakes; camera-defaults -> reversed-depth',scope:'After the unchanged ordinary-slot main-page setup, later modules use child contexts. record-properties wraps arctic-background commands on its child pages. No inherited callback execution or camera-equivalence claim.'};
}
function relocate(source:string,original:string){const edits:any[]=[];let text=source.replace(/from '(\.\/[^']+)'/g,(before,spec)=>{const after="from '"+new URL(spec,pathToFileURL(original)).href+"'";edits.push({before,after});return after;});text=text.replace(/new URL\('([^']+)',\s*import\.meta\.url\)/g,(before,spec)=>{const after='new URL('+JSON.stringify(new URL(spec,pathToFileURL(original)).href)+')';edits.push({before,after});return after;});return{text,edits};}
export async function prepareChildContextHelper(){
    const audit=sourceAudit(),original=fileURLToPath(new URL('./browser-scene-presentation.ts',import.meta.url)),parent=fileURLToPath(new URL('./image-lane-browser-scene.ts',import.meta.url)),source=fs.readFileSync(original,'utf8');
    const before="const instanceChecks = ['scene-reload', 'record-properties', 'arena-reclamation', 'arena-empty'].map",after="const instanceChecks = ['scene-reload', 'record-properties', 'arena-reclamation', 'arena-empty', 'precise-origin', 'precise-origin-wakes', 'reversed-depth', 'camera-defaults'].map";assert.equal(source.split(before).length,2);
    const clone=relocate(source.replace(before,after),original),key=hash(JSON.stringify({helper:sha(self),audit,worker:sha(original),parent:sha(parent)})).slice(0,20),folder=path.join(viewer,'target/course-presentation/child-context-scene-helper',key),worker=path.join(folder,'worker.ts'),transport=path.join(folder,'parent.ts');
    let reverse=clone.text;for(const e of clone.edits)reverse=reverse.replace(e.after,e.before);assert.equal(reverse.replace(after,before),source);
    let parentSource=fs.readFileSync(parent,'utf8');const parentEdits=[['from \'./browser-scene-presentation.ts\'','from '+JSON.stringify(pathToFileURL(worker).href)],["fileURLToPath(new URL('browser-scene-presentation.ts', import.meta.url))",JSON.stringify(worker)]];
    for(const[a,b]of parentEdits){assert.equal(parentSource.split(a).length,2);parentSource=parentSource.replace(a,b);}const routed=relocate(parentSource,parent);let parentReverse=routed.text;for(const e of routed.edits)parentReverse=parentReverse.replace(e.after,e.before);for(const[a,b]of [...parentEdits].reverse())parentReverse=parentReverse.replace(b,a);assert.equal(parentReverse,fs.readFileSync(parent,'utf8'));
    for(const[f,text]of[[worker,clone.text],[transport,routed.text]])if(fs.existsSync(f))assert.equal(fs.readFileSync(f,'utf8'),text);else write(f,text);
    const bridge={audit,originalWorker:ref(original),worker:ref(worker),originalParent:ref(parent),parent:ref(transport),workerDelta:{before,after},workerImports:clone.edits,parentEdits,parentImports:routed.edits,reverseExact:true},bridgeFile=path.join(folder,'bridge.json');if(fs.existsSync(bridgeFile))assert.deepEqual(json(bridgeFile),bridge);else write(bridgeFile,JSON.stringify(bridge,null,2)+'\n');
    await import(pathToFileURL(worker).href);await import(pathToFileURL(transport).href);return{worker,transport,bridgeFile,folder,audit};
}
export async function prepareChildContextScene(id:string,builtFile:string){
    assert(childContextImageRecipe(id));const{worker,transport,bridgeFile}=await prepareChildContextHelper();const w=await import(pathToFileURL(worker).href),base=w.prepareScene(id,builtFile);for(const name of names){const f=path.join(docs,'journey/checks/browser',name+'.cjs');base.pins[f]=sha(f);}base.pins[self]=sha(self);base.pins[bridgeFile]=sha(bridgeFile);write(path.join(base.directory,'prepared.json'),JSON.stringify(base,null,2)+'\n');const t=await import(pathToFileURL(transport).href),p=t.prepareImageLaneScene(id,builtFile,path.join(base.directory,'prepared.json'));
    const wrapper={id,folder:p.directory,preparedFile:path.join(p.directory,'prepared.json'),parent:ref(transport),bridge:ref(bridgeFile),helper:ref(self)};write(path.join(p.directory,'child-context-prepared.json'),JSON.stringify(wrapper,null,2)+'\n');return wrapper;
}
export async function runChildContextScene(file:string){const p=json(path.resolve(file));for(const r of[p.parent,p.bridge,p.helper])assert.equal(sha(r.path),r.sha256);const t=await import(pathToFileURL(p.parent.path).href),r=await t.runImageLaneScene(p.preparedFile),record={...r,childContextSourceBridge:p.bridge,canonicalBehaviorExecuted:false,scope:'Actual source-exact original-main-page illustration; later inherited feature assertions execute on separate child contexts and were not replayed.'};const output=path.join(p.folder,'child-context-capture.json');assert(!fs.existsSync(output));write(output,JSON.stringify(record,null,2)+'\n');return record;}
