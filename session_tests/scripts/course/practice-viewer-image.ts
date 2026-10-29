import {cargoTarget} from './capture-storage.ts';
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import crypto from 'node:crypto';
import {spawnSync} from 'node:child_process';
import {fileURLToPath,pathToFileURL} from 'node:url';
const self=fileURLToPath(import.meta.url),repo=fileURLToPath(new URL('../../../',import.meta.url)),viewer=path.join(repo,'session_viewer'),tools=path.join(repo,'session_tests/scripts/course');
const {filesIn}=await import(pathToFileURL(path.join(tools,'model.ts')).href);
const {exactBindgen}=await import(pathToFileURL(path.join(tools,'exact-bindgen.ts')).href);
const {privateImageBrowserHost}=await import(pathToFileURL(path.join(tools,'parallel-image-browser-host.ts')).href);
const {observeGpu,validateGpu}=await import(pathToFileURL(path.join(tools,'private-browser-host.ts')).href);
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex'),ref=p=>({path:path.resolve(p),sha256:sha(p)}),json=p=>JSON.parse(fs.readFileSync(p,'utf8'));
const guard=path.join(tools,'parallel-image-guard.ps1'),original=path.join(viewer,'docs/capture_viewer.cjs');
const defaultNative=path.join(viewer,'target/course-presentation/private-support-d4dea55c2d7da7ea6efa/reference-37-scene');
const snapshot=(native)=>Object.fromEntries([self,original,guard,path.join(tools,'parallel-image-browser-host.ts'),path.join(tools,'private-browser-host.ts'),...filesIn(path.join(native,'src')),path.join(native,'Cargo.toml'),path.join(native,'Cargo.lock'),...filesIn(path.join(native,'assets'))].map(f=>[f,sha(f)]));
function command(folder,label,args,cwd,env={}){
    const output=path.join(folder,label);assert(!fs.existsSync(output));fs.mkdirSync(output,{recursive:true});
    const commandFile=path.join(output,'command.json'),log=path.join(output,'output.log'),resourcesFile=path.join(output,'resources.json');
    fs.writeFileSync(commandFile,JSON.stringify({executable:args[0],arguments:args.slice(1)})+'\n');const fd=fs.openSync(log,'wx');let r;
    try{r=spawnSync('powershell.exe',['-NoProfile','-ExecutionPolicy','Bypass','-File',guard,'-ProcessorCount','8','-MemoryLimitMiB','8192','-TimeoutSeconds','1800','-WorkingDirectory',cwd,'-ManifestPath',resourcesFile,'-CommandFile',commandFile],{cwd,env:{...process.env,...env},stdio:['ignore',fd,fd],windowsHide:true});}finally{fs.closeSync(fd);}
    assert.equal(r.status,0,log);const resources=json(resourcesFile);assert.equal(resources.exitCode,0);assert.equal(resources.affinityMask,'0xFF00');return{command:ref(commandFile),log:ref(log),resources:ref(resourcesFile),measured:resources};
}
export async function practiceViewerImages(mode, file, nativeSource=defaultNative){
const native=path.resolve(nativeSource);
if(mode==='prepare'){
    const inputs=snapshot(native),key=crypto.createHash('sha256').update(JSON.stringify(inputs)).digest('hex');
    const folder=path.join(viewer,'target/course-presentation/practice-viewer-'+key.slice(0,20));assert(!fs.existsSync(folder));fs.mkdirSync(folder,{recursive:true});
    const source=path.join(folder,'source');fs.cpSync(native,source,{recursive:true,filter:f=>!/[.](png|ppm)$/.test(f)});
    let cargo=fs.readFileSync(path.join(source,'Cargo.toml'),'utf8'),lock=fs.readFileSync(path.join(source,'Cargo.lock'),'utf8');
    assert(!/^wood\s*=/m.test(cargo),'Reference browser must not invoke livewood generator');const old=cargo.match(/^name = "([^"]+)"/m)[1],unique='practice_viewer_'+key.slice(0,20);
    cargo=cargo.replace('name = "'+old+'"','name = "'+unique+'"');assert.equal(lock.split('name = "'+old+'"').length,2);lock=lock.replace('name = "'+old+'"','name = "'+unique+'"');
    fs.writeFileSync(path.join(source,'Cargo.toml'),cargo);fs.writeFileSync(path.join(source,'Cargo.lock'),lock);
    const workspace=path.join(folder,'workspace'),dist=path.join(workspace,'target/course-checks/reference37/dist');fs.mkdirSync(dist,{recursive:true});
    const changes=[];let worker=fs.readFileSync(original,'utf8');const replace=(before,after)=>{assert.equal(worker.split(before).length,2,before);worker=worker.replace(before,after);changes.push({before,after});};
    replace("const out = 'docs/screenshots/practice';","const out = "+JSON.stringify(path.join(folder,'images'))+";fs.mkdirSync(out,{recursive:true});");
    const launch=worker.slice(worker.indexOf('    const browser = await chromium.launch('),worker.indexOf('\n    try {'));replace(launch,"    const browser = await chromium.launch({headless:true,executablePath:process.env.VIEWER_BROWSER_EXECUTABLE,args:JSON.parse(process.env.VIEWER_CHROME_ARGS)});");
    replace('deviceScaleFactor:1','deviceScaleFactor:2');replace('?data=off&inspect=1&dpr=1','?data=off&inspect=1&dpr=2');
    replace('        const errors = [];','        '+observeGpu+'\n        const errors = [];');
    replace('            records.frames[name] = await state();','            '+validateGpu+'\n            await page.waitForFunction(()=>{const c=document.querySelector("canvas"),r=c.getBoundingClientRect();return devicePixelRatio===2&&c.width===Math.round(r.width*2)&&c.height===Math.round(r.height*2)});\n            records.frames[name] = await state();');
    const workerFile=path.join(folder,'worker.cjs');fs.writeFileSync(workerFile,worker);let reverse=worker;for(const row of changes.reverse())reverse=reverse.replace(row.after,row.before);assert.equal(reverse,fs.readFileSync(original,'utf8'));
    const plan={status:'PREPARED six original finished-viewer references; transport/DPR2 only',folder,source,workspace,dist,unique,inputs,sourceCargo:ref(path.join(source,'Cargo.toml')),sourceLock:ref(path.join(source,'Cargo.lock')),worker:ref(workerFile),original:ref(original),changes:changes.reverse(),reversesExactlyToOriginal:true,sourceFingerprint:json(path.join(path.dirname(native),'reference-result-frozen.json')).items.reference.originalSource};
    fs.writeFileSync(path.join(folder,'plan.json'),JSON.stringify(plan,null,2)+'\n');console.log(path.join(folder,'plan.json'));
}else if(mode==='run'){
    const p=json(file);for(const[f,h]of Object.entries(p.inputs))assert.equal(sha(f),h,f);assert.equal(sha(p.worker.path),p.worker.sha256);
    const target=cargoTarget(),build=command(p.folder,'wasm',['cargo','build','--lib','--locked','--offline','--target','wasm32-unknown-unknown','-j8','-vv','--message-format=json-render-diagnostics'],p.source,{CARGO_TARGET_DIR:target,CARGO_NET_OFFLINE:'true'});
    const artifacts=fs.readFileSync(build.log.path,'utf8').split(/\r?\n/).filter(s=>s.startsWith('{')).map(s=>{try{return JSON.parse(s)}catch{return null}}).filter(r=>r?.reason==='compiler-artifact'&&path.resolve(r.manifest_path)===path.join(p.source,'Cargo.toml'));
    const wasm=artifacts.flatMap(r=>r.filenames).find(f=>f.endsWith('.wasm'));assert(wasm);const retained=path.join(p.folder,'actual-source.wasm');fs.copyFileSync(wasm,retained,fs.constants.COPYFILE_EXCL);
    const bindgen=exactBindgen(path.join(p.source,'Cargo.lock'));const glue=command(p.folder,'bindgen',[bindgen.executable,retained,'--target','web','--out-dir',p.dist,'--out-name','session_viewer'],p.source);
    const start='<script type="module">import init,* as bindings from "./session_viewer.js";const wasm=await init({module_or_path:"./session_viewer_bg.wasm"});window.wasmBindings=bindings;dispatchEvent(new CustomEvent("TrunkApplicationStarted",{detail:{wasm}}));</script>';
    let html=fs.readFileSync(path.join(p.source,'index.html'),'utf8').replace(/<link\b[^>]*\bdata-trunk\b[^>]*\brel="rust"[^>]*>/,start);assert(html.includes(start));html=html.replace(/<link\b[^>]*\bdata-trunk\b[^>]*>/g,'');fs.writeFileSync(path.join(p.dist,'index.html'),html);
    for(const name of ['pb','text'])fs.cpSync(path.join(p.source,'assets',name),path.join(p.dist,name),{recursive:true});
    for(const name of ['view_local.yaml','text-quality.html'])fs.copyFileSync(path.join(p.source,'assets',name),path.join(p.dist,name));
    const host=await privateImageBrowserHost(p.workspace,path.join(p.folder,'capture'),p.worker.path,{id:'reference37',canonicalBehaviorExecuted:false});
    const images=['loaded','point','layers','selected','arctic','opacity'].map(name=>{const f=path.join(p.folder,'images/viewer-'+name+'.png');const bytes=fs.readFileSync(f);assert.equal(bytes.readUInt32BE(16),2200);assert.equal(bytes.readUInt32BE(20),1520);return{path:f,destination:'docs/screenshots/practice/viewer-'+name+'.png',sha256:sha(f),width:2200,height:1520};});
    for(const[f,h]of Object.entries(p.inputs))assert.equal(sha(f),h,f);
    fs.writeFileSync(path.join(p.folder,'capture.json'),JSON.stringify({status:'SIX_ORIGINAL_FINISHED_VIEWER_REFERENCES_CAPTURED_AWAITING_VISUAL_REVIEW',plan:ref(file),sourceFingerprint:p.sourceFingerprint,build,wasm:ref(retained),artifacts,bindgen,glue,host,sceneRecord:ref(path.join(p.folder,'images/viewer-captures.json')),images,canonicalBehaviorExecuted:false,scope:'Exact originalcapture_viewer actions/assertions at original1100x760logical viewport/DPR2; earlierlessons labelled finished-viewer references. Image-profile only.'},null,2)+'\n');console.log(JSON.stringify(images,null,2));
}else throw Error('practice-viewer-image.ts prepare [SOURCE_STAGE] | run PLAN');
}
if(process.argv[1]&&path.resolve(process.argv[1])===self){const[mode,arg]=process.argv.slice(2);await practiceViewerImages(mode,mode==='run'?arg:undefined,mode==='prepare'&&arg?arg:defaultNative);}
