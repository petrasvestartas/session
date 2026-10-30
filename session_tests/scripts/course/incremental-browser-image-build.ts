import {cargoTarget, compilerTools, checkWorkbenchOwnership, workbenchPath} from './capture-storage.ts';
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {isDeepStrictEqual} from 'node:util';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {prepareBrowser} from './browser-presentation.ts';
import {bindings} from './presentation.ts';
import {exactBindgen} from './exact-bindgen.ts';
import {assetsFor, docs, expected, filesIn, hash, json, lockFor, materialize, read, viewer, write} from './model.ts';
import {nativeTarget} from './verify.ts';

const self=fileURLToPath(import.meta.url), original=fileURLToPath(new URL('./browser-presentation.ts',import.meta.url));
const guard=fileURLToPath(new URL('./parallel-image-guard.ps1',import.meta.url));
const sha=(file:string)=>hash(fs.readFileSync(file));
const root=path.join(viewer,'target/course-presentation/incremental-browser-builds');
const policy={purpose:'Image-only stable incremental compiler experiment; no canonical callback acceptance',processors:8,memoryMiB:8192,jobs:8,affinityMask:'0xFF00',timeoutSeconds:1800,profile:'debug',incremental:'1'};
const manifest=(folder:string)=>Object.fromEntries(filesIn(folder).filter(f=>!path.relative(folder,f).split(path.sep).includes('target')).map(f=>[path.relative(folder,f).replaceAll('\\','/'),sha(f)]));

export function copyTrunkFiles(native:string,dist:string){
    for(const link of read(path.join(native,'index.html')).matchAll(/<link\b[^>]*data-trunk[^>]*rel="copy-file"[^>]*href="([^"]+)"[^>]*>/g)){
        const source=path.resolve(native,link[1]),relative=path.relative(native,source);
        assert(relative&&!relative.startsWith('..')&&!path.isAbsolute(relative),'Trunk copy-file must stay inside the materialized source');
        assert.equal(fs.realpathSync(source),source,'Trunk copy-file source cannot be redirected');
        const destination=path.join(dist,path.basename(source));
        if(fs.existsSync(destination)){assert.equal(fs.realpathSync(destination),path.resolve(destination));assert.equal(sha(destination),sha(source),'Trunk copy-file destination collision');}
        else fs.copyFileSync(source,destination,fs.constants.COPYFILE_EXCL);
        assert.equal(sha(destination),sha(source));
    }
}

function tools(){return Object.fromEntries(['rustc','cargo'].map(exe=>{const r=spawnSync(exe,['-Vv'],{encoding:'utf8',windowsHide:true});assert.equal(r.status,0);return[exe,r.stdout];}));}
function sourceCheck(p:any){
    assert.equal(p.helperSha256,sha(self));assert.deepEqual(p.source,bindings(p.id));
    for(const[f,digest]of Object.entries(p.pins))assert.equal(sha(f),digest,f);
    assert.deepEqual(manifest(p.native),p.canonicalFiles);
    const kernel=path.relative(p.native,path.resolve(viewer,'../session_rust')).replaceAll('\\','/');
    for(const[name,text]of Object.entries(expected(p.id)))assert.equal(read(path.join(p.native,name)),name==='Cargo.toml'?text.replaceAll('../../../session_rust',kernel):text,name);
    assert.equal(sha(path.join(p.native,'Cargo.lock')),sha(lockFor(p.id)));
    for(const name of assetsFor(p.id))assert.equal(sha(path.join(p.native,name)),sha(path.join(viewer,name)),name);
    if(p.bundle)assert.deepEqual(Object.fromEntries(filesIn(p.dist).map(f=>[path.relative(p.dist,f).replaceAll('\\','/'),sha(f)])),p.bundle);
}

export function prepareIncrementalImageBrowser(id:string,density=2){
    assert.equal(density,2);
    const source=bindings(id),capture=path.join(docs,'capture_journey.cjs'),identity={source,density,attempt:process.env.COURSE_CAPTURE_ATTEMPT||null,helperSha256:sha(self),guardSha256:sha(guard),policy,toolFiles:compilerTools(),tools:tools()};
    const folder=path.join(root,'cases',id+'-'+hash(JSON.stringify(identity)).slice(0,20)),preparedFile=path.join(folder,'prepared.json');
    if(fs.existsSync(preparedFile)){const p=json(preparedFile);sourceCheck(p);return p;}
    const baselineKey=hash(JSON.stringify({source,density,attempt:process.env.COURSE_CAPTURE_ATTEMPT||null,helper:sha(original),capture:sha(capture)}));
    const baselineFile=path.join(viewer,'target/course-presentation',`browser-${id}-${baselineKey.slice(0,16)}-dpr${density}`,'prepared.json');
    const base=fs.existsSync(baselineFile)?json(baselineFile):prepareBrowser(id,density);assert.deepEqual(base.source,source);
    const native=path.join(folder,'canonical-source');materialize(expected(id),native,lockFor(id),assetsFor(id));
    const canonicalFiles=manifest(native),canonicalCargo=read(path.join(base.native,'Cargo.toml'));
    const groupIdentity={cargo:hash(expected(id)['Cargo.toml']),lock:sha(lockFor(id)),dependencies:source.dependencies,kernel:source.kernel,tools:identity.tools,policy,helperSha256:sha(self),guardSha256:sha(guard)};
    const groupKey='compact-browser-v1',group=path.join(viewer,'target/course-ib',groupKey),workbench=path.join(group,'workspace'),target=cargoTarget();
    const workspace=path.join(folder,'workspace'),dist=path.join(workspace,'target/course-checks',id,'dist');
    write(path.join(workspace,'docs/journey/course.json'),read(path.join(docs,'journey/course.json')));
    fs.cpSync(path.join(docs,'journey/checks'),path.join(workspace,'docs/journey/checks'),{recursive:true});
    fs.mkdirSync(path.join(workspace,'docs/screenshots/journey'),{recursive:true});
    write(path.join(workspace,'target/course-checks/results.json'),JSON.stringify({[id]:{source:source.files}}));
    const pins={...base.pins,[self]:sha(self),[guard]:sha(guard),[fileURLToPath(new URL('./exact-bindgen.ts',import.meta.url))]:sha(fileURLToPath(new URL('./exact-bindgen.ts',import.meta.url)))};
    const dependencyPaths:any[]=[];let table=false;
    const rebased=canonicalCargo.split('\n').map(line=>{
        if(/^\s*\[/.test(line))table=/(?:^|\.)(?:dependencies|dev-dependencies|build-dependencies)(?:\.|\])/.test(line.trim().slice(1))||/^\s*\[(?:patch\.|replace\])/.test(line);
        return !table||/^\s*#/.test(line)?line:line.replace(/\bpath\s*=\s*"([^"]+)"/g,(match,relative)=>{
            const absolute=path.resolve(base.native,relative);assert(fs.existsSync(path.join(absolute,'Cargo.toml')),'Dependency must exist: '+absolute);
            const relocated=path.relative(workbench,absolute).replaceAll('\\','/');dependencyPaths.push({relative,absolute,relocated});return match.replace('"'+relative+'"','"'+relocated+'"');
        });
    }).join('\n');
    const old=rebased.match(/^name\s*=\s*"([^"]+)"/m)![1],packageName='image_incremental',lock=read(path.join(native,'Cargo.lock'));
    assert.equal(lock.split('name = "'+old+'"').length,2);
    const overlayCargo=rebased.replace('name = "'+old+'"','name = "'+packageName+'"').replace('[lib]','[lib]\nname = "viewer_journey"');
    const overlayLock=lock.replace('name = "'+old+'"','name = "'+packageName+'"');
    const p={...base,...identity,status:'PREPARED_INCREMENTAL_IMAGE_BUILD_NOT_EXECUTED',id,density,folder,preparedFile,workspace,dist,native,pins,
        baseline:{path:baselineFile,sha256:sha(baselineFile),sourceOnlyReused:true,priorBuiltStatus:base.status},canonicalFiles,
        group:{key:groupKey,identity:groupIdentity,folder:group,workbench,target,package:packageName,library:'viewer_journey'},
        overlay:{cargo:overlayCargo,lock:overlayLock,dependencyPaths,cargoSha256:hash(overlayCargo),lockSha256:hash(overlayLock)},
        canonicalNativeUnchanged:{path:base.native,files:manifest(base.native)},canonicalBehaviorExecuted:false};
    delete p.bundle;delete p.commands;delete p.imageBuild;delete p.image;
    write(preparedFile,JSON.stringify(p,null,2)+'\n');sourceCheck(p);return p;
}

export function buildIncrementalImageBrowser(id:string,density=2,options:{beforeIdentityChange?:(previous:any,prepared:any)=>unknown}={}){
    const p=prepareIncrementalImageBrowser(id,density);sourceCheck(p);
    if(p.status==='BUILT_NOT_CAPTURED')return p;
    assert.equal(p.status,'PREPARED_INCREMENTAL_IMAGE_BUILD_NOT_EXECUTED');
    const g=p.group;fs.mkdirSync(g.folder,{recursive:true});const lockFile=path.join(g.folder,'workbench.lock'),lockFd=fs.openSync(lockFile,'wx');
    fs.writeFileSync(lockFd,JSON.stringify({pid:process.pid,id,preparedFile:p.preparedFile}));
    try{
        assert.deepEqual(tools(),p.tools);assert.deepEqual(compilerTools(),p.toolFiles);assert.deepEqual(manifest(p.canonicalNativeUnchanged.path),p.canonicalNativeUnchanged.files);
        const stateFile=path.join(g.folder,'state.json'),previous=fs.existsSync(stateFile)?json(stateFile):null;
        if(previous){assert.equal(previous.groupKey,g.key);assert(fs.existsSync(previous.completedReceipt),'Previous source must be frozen before reuse');assert.equal(sha(previous.completedReceipt),previous.completedReceiptSha256);}
        const pendingFile=path.join(g.folder,'pending.json');assert(!fs.existsSync(pendingFile),'An incomplete prior workbench attempt requires explicit recovery');
        assert(!previous || previous.ownedInputs, 'Workbench requires an exact prior owned-input manifest');
        checkWorkbenchOwnership(g.workbench, previous?.ownedInputs || {});
        let compilerCacheTransition:unknown=null;
        if(previous&&options.beforeIdentityChange&&!isDeepStrictEqual(json(previous.completedReceipt).group.identity,g.identity))
            compilerCacheTransition=options.beforeIdentityChange(previous,p);
        write(pendingFile,JSON.stringify({id,preparedFile:p.preparedFile,previousReceipt:previous?.completedReceipt||null},null,2)+'\n');
        const desired={...p.canonicalFiles,'Cargo.toml':p.overlay.cargoSha256,'Cargo.lock':p.overlay.lockSha256},synced:string[]=[],removed:string[]=[];
        for(const file of Object.keys(previous?.ownedInputs||{}))if(!(file in desired)){const target=workbenchPath(g.workbench,file);fs.unlinkSync(target);removed.push(file);}
        for(const[file,digest]of Object.entries(desired)){
            const target=workbenchPath(g.workbench,file);if(fs.existsSync(target)&&sha(target)===digest)continue;
            if(file==='Cargo.toml')write(target,p.overlay.cargo);else if(file==='Cargo.lock')write(target,p.overlay.lock);else{fs.mkdirSync(path.dirname(target),{recursive:true});fs.copyFileSync(path.join(p.native,file),target);}synced.push(file);
        }
        const check=()=>{sourceCheck(p);for(const[file,digest]of Object.entries(desired))assert.equal(sha(path.join(g.workbench,file)),digest,file);assert.deepEqual(manifest(p.canonicalNativeUnchanged.path),p.canonicalNativeUnchanged.files);};
        const commands:any[]=[];
        const execute=(label:string,args:string[])=>{
            check();const prefix=path.join(p.folder,label),commandFile=prefix+'.command.json',log=prefix+'.log',resourcesFile=prefix+'.resources.json';assert(!fs.existsSync(log),'Preserve old attempt: '+log);
            write(commandFile,JSON.stringify({executable:args[0],arguments:args.slice(1)})+'\n');const fd=fs.openSync(log,'wx');let result;
            try{result=spawnSync('powershell.exe',['-NoProfile','-ExecutionPolicy','Bypass','-File',guard,'-ProcessorCount','8','-MemoryLimitMiB','8192','-TimeoutSeconds','1800','-WorkingDirectory',g.workbench,'-ManifestPath',resourcesFile,'-CommandFile',commandFile],
                {cwd:g.workbench,windowsHide:true,stdio:['ignore',fd,fd],env:{...process.env,RUSTC_WRAPPER:'',CARGO_NET_OFFLINE:process.env.CARGO_NET_OFFLINE||'true',NO_COLOR:'true',CARGO_BUILD_JOBS:'8',RUST_TEST_THREADS:'1',REGEN_PROTO:'0',CARGO_INCREMENTAL:'1',CARGO_TARGET_DIR:g.target}});
            }finally{fs.closeSync(fd);}
            const resources=json(resourcesFile);assert.equal(result.status,0,log);assert.equal(resources.exitCode,0);assert.equal(resources.timedOut,false);assert.equal(resources.processors,8);assert.equal(resources.jobCommitLimitBytes,8589934592);assert.equal(resources.affinityMask,'0xFF00');assert.equal(resources.laneMutex,'Local\\SessionViewerCourseImageJob');check();
            const r={label,args,commandFile,log,logSha256:sha(log),resourcesFile,resourcesSha256:sha(resourcesFile),resources};commands.push(r);return r;
        };
        if(fs.existsSync(path.join(g.workbench,'examples/sample.rs'))){const sample=path.join(g.workbench,'sample.pb');if(fs.existsSync(sample))fs.unlinkSync(sample);execute('sample',['cargo','run','--example','sample','--locked','--target',nativeTarget(),'-j8']);assert(fs.statSync(sample).size>0);fs.copyFileSync(sample,path.join(p.folder,'sample.pb'));}
        const wasmCommand=execute('wasm',['cargo','build','--lib','--locked','--target','wasm32-unknown-unknown','-j8','-vv','--message-format=json-render-diagnostics']);
        const artifacts=read(wasmCommand.log).split(/\r?\n/).flatMap(line=>{try{return[JSON.parse(line)];}catch{return[];}}).filter(r=>r.reason==='compiler-artifact'&&r.target?.name==='viewer_journey'&&path.resolve(r.manifest_path)===path.join(g.workbench,'Cargo.toml'));
        assert.equal(artifacts.length,1,'Actual own source-bound Cargo WASM artifact required');
        const wasm=path.join(g.target,'wasm32-unknown-unknown/debug/viewer_journey.wasm');assert(artifacts[0].filenames.some((f:string)=>path.resolve(f)===wasm));
        const retained=path.join(p.folder,'raw.wasm');assert(!fs.existsSync(retained));fs.copyFileSync(wasm,retained);const wasmSha256=sha(wasm),bindgen=exactBindgen(path.join(g.workbench,'Cargo.lock'));
        execute('bindgen',[bindgen.executable,wasm,'--target','web','--out-dir',p.dist,'--out-name','viewer_journey']);assert.equal(sha(wasm),wasmSha256);assert.equal(sha(bindgen.executable),bindgen.sha256);
        const start=`<script type="module">import init,* as bindings from './viewer_journey.js'; const wasm=await init({module_or_path:'./viewer_journey_bg.wasm'}); window.wasmBindings=bindings; dispatchEvent(new CustomEvent('TrunkApplicationStarted',{detail:{wasm}}));</script>`;
        let index=read(path.join(g.workbench,'index.html')).replace(/<link\b[^>]*\bdata-trunk\b[^>]*\brel="rust"[^>]*>/,start);assert(index.includes(start));index=index.replace(/<link\b[^>]*\bdata-trunk\b[^>]*>/g,'');write(path.join(p.dist,'index.html'),index);
        for(const asset of assetsFor(id)){const target=path.join(p.dist,asset.startsWith('assets/text/')?asset.replace('assets/text/','text/'):asset);fs.mkdirSync(path.dirname(target),{recursive:true});fs.copyFileSync(path.join(g.workbench,asset),target);}
        copyTrunkFiles(p.native,p.dist);
        const sceneSource=path.join(p.workspace,'target','course-'+id);fs.cpSync(p.native,sceneSource,{recursive:true});if(fs.existsSync(path.join(p.folder,'sample.pb')))fs.copyFileSync(path.join(p.folder,'sample.pb'),path.join(sceneSource,'sample.pb'));
        const retainedArtifacts=artifacts[0].filenames.map((file:string)=>({originalPath:file,sha256:sha(file)}));
        check();assert.equal(sha(retained),wasmSha256);
        const method={status:'BUILT_IMAGE_ONLY_SOURCE_BOUND',helperSha256:sha(self),policy,toolFiles:p.toolFiles,group:g,source:p.source,overlay:p.overlay,canonicalFiles:p.canonicalFiles,
            compilerCacheTransition,
            synchronization:{previous:previous?{id:previous.id,receipt:previous.completedReceipt,sha256:previous.completedReceiptSha256}:null,copiedChangedFiles:synced,removedOwnedStaleFiles:removed},commands,compilerArtifact:artifacts[0],retainedArtifacts,bindgen,
            wasm:{path:retained,sha256:wasmSha256,compilerReportedPath:wasm},bundle:Object.fromEntries(filesIn(p.dist).map(f=>[path.relative(p.dist,f).replaceAll('\\','/'),sha(f)])),canonicalBehaviorExecuted:false,
            scope:'Actual Cargo invocation for this checkpoint with stable package/path and debug CARGO_INCREMENTAL=1. Complete source, one raw WASM and one bundle retained until compact publication retirement; compiler JSON and exact artifact hashes preserved without duplicate compiler-artifact copies. No GPU image or canonical callback acceptance.'};
        const methodFile=path.join(p.folder,'built.json');assert(!fs.existsSync(methodFile));write(methodFile,JSON.stringify(method,null,2)+'\n');
        const built={...p,status:'BUILT_NOT_CAPTURED',commands,wasm:method.wasm,bundle:method.bundle,imageBuild:{path:methodFile,sha256:sha(methodFile)}};sourceCheck(built);write(p.preparedFile,JSON.stringify(built,null,2)+'\n');
        write(stateFile,JSON.stringify({groupKey:g.key,id,ownedFiles:Object.keys(desired),ownedInputs:manifest(g.workbench),completedReceipt:methodFile,completedReceiptSha256:sha(methodFile)},null,2)+'\n');fs.unlinkSync(pendingFile);return built;
    }finally{fs.closeSync(lockFd);fs.unlinkSync(lockFile);}
}

if(process.argv[1]&&path.resolve(process.argv[1])===self){const[mode,id]=process.argv.slice(2);assert(['prepare','build'].includes(mode));console.log(JSON.stringify(mode==='prepare'?prepareIncrementalImageBrowser(id):buildIncrementalImageBrowser(id),null,2));}
