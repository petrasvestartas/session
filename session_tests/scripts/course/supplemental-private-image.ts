import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {docs,viewer,filesIn,hash,json,read,write} from './model.ts';

const self=fileURLToPath(import.meta.url),original=fileURLToPath(new URL('./supplemental-presentation.ts',import.meta.url));
const guard=fileURLToPath(new URL('./parallel-image-guard.ps1',import.meta.url));
export const supportTarget=path.join(viewer,'target/course-sup/native-cache');
const sha=(file:string)=>hash(fs.readFileSync(file));
const ref=(file:string)=>({path:path.resolve(file),sha256:sha(file)});

export function rebaseReferenceDependencies(cargo:string,id:string,folder:string){
    assert(read(fileURLToPath(new URL('./reference.ts',import.meta.url))).includes("const target = path.join(viewer, 'target/course-reference');"),'Preserve canonical reference materialization base');
    const changes:any[]=[];let dependency=false;
    const after=cargo.split('\n').map(line=>{
        if(/^\s*\[/.test(line))dependency=/dependencies/.test(line);
        if(!dependency||/^\s*#/.test(line))return line;
        return line.replace(/\bpath\s*=\s*"([^"]+)"/g,(before,relative)=>{
            const target=path.resolve(viewer,'target/course-reference',relative);assert(fs.existsSync(path.join(target,'Cargo.toml')),target);
            const replacement=target.split(path.sep).join('/');changes.push({relative,target});return 'path = "'+replacement+'"';
        });
    }).join('\n');
    write(path.join(folder,'dependency-path-qualification.json'),JSON.stringify({beforeSha256:hash(cargo),afterSha256:hash(after),changes},null,2)+'\n');return after;
}

export function supportCommand(label:string,originalArgs:string[],cwd:string,environment:Record<string,string>={}){
    assert(!label.includes('cmake')&&!label.includes('cpp-'),'C++ generation belongs to the separately prepared maintained runner');
    const folder=path.join(viewer,'target/course-sup/executions',label);assert(!fs.existsSync(folder),'Preserve actual attempt: '+folder);fs.mkdirSync(folder,{recursive:true});
    let args=originalArgs.map(arg=>arg==='-j1'?'-j8':arg);
    if(args[0]==='cargo')args=[...args,'-vv','--message-format=json-render-diagnostics'];
    const compiler=process.env.VIEWER_DXC_PATH||path.join(viewer,'target/course-windows-20261018/native-checkpoint/dxcompiler.dll');
    const env={...process.env,...environment,CARGO_TARGET_DIR:supportTarget,CARGO_NET_OFFLINE:'true',VIEWER_DXC_PATH:compiler,WGPU_DX12_COMPILER:'dxc',WGPU_DX12_DXC_PATH:compiler};
    const executable=args.shift()!,commandFile=path.join(folder,'command.json'),resourcesFile=path.join(folder,'resources.json'),log=path.join(folder,'output.log');
    write(commandFile,JSON.stringify({executable,arguments:args})+'\n');const descriptor=fs.openSync(log,'wx');let execution;
    try{execution=spawnSync('powershell.exe',['-NoProfile','-ExecutionPolicy','Bypass','-File',guard,'-ProcessorCount','8','-MemoryLimitMiB','8192','-TimeoutSeconds','1800','-WorkingDirectory',cwd,'-ManifestPath',resourcesFile,'-CommandFile',commandFile],{cwd,env,windowsHide:true,stdio:['ignore',descriptor,descriptor]});}finally{fs.closeSync(descriptor);}
    assert.equal(execution.status,0,log);const resources=json(resourcesFile);assert.equal(resources.exitCode,0);assert.equal(resources.processors,8);assert.equal(resources.jobCommitLimitBytes,8589934592);assert.equal(resources.affinityMask,'0xFF00');
    const artifacts=read(log).split(/\r?\n/).filter(line=>line.startsWith('{')).map(line=>{try{return JSON.parse(line);}catch{return null;}}).filter(row=>row?.reason==='compiler-artifact'&&path.resolve(row.manifest_path)===path.join(cwd,'Cargo.toml'));
    if(executable==='cargo'){
        assert(artifacts.some(row=>(row.target.kind.includes('lib')||row.target.crate_types?.some((kind:string)=>['rlib','lib','cdylib'].includes(kind)))),'Actual own-library Cargo artifact required');
        const at=args.indexOf('--example');assert(at>=0);assert(artifacts.some(row=>row.target.name===args[at+1]&&row.executable),'Actual requested example artifact required');
        for(const artifact of artifacts)for(const file of artifact.filenames)assert(fs.existsSync(file),file);
    }
    const record={name:label,returncode:0,scope:'Private image8CPU8GiB transport; original image algorithm/assertions retained',command:ref(commandFile),log:ref(log),resourcesFile:ref(resourcesFile),resources,environment:{CARGO_TARGET_DIR:supportTarget,CARGO_NET_OFFLINE:'true',VIEWER_DXC_PATH:compiler,WGPU_DX12_COMPILER:'dxc',WGPU_DX12_DXC_PATH:compiler},artifacts:artifacts.map(row=>({...row,retainedArtifacts:row.filenames.map((file:string)=>{const retained=path.join(folder,'artifacts',path.basename(file));fs.mkdirSync(path.dirname(retained),{recursive:true});fs.copyFileSync(file,retained,fs.constants.COPYFILE_EXCL);return{original:file,retained:ref(retained)};})}))};
    write(path.join(folder,'execution.json'),JSON.stringify(record,null,2)+'\n');return record;
}

export function preparePrivateSupport(oldPlanFile:string){
    oldPlanFile=path.resolve(oldPlanFile);const old=json(oldPlanFile);
    for(const[file,digest]of Object.entries({...old.inputs,...old.generatorFiles}))assert.equal(sha(file),digest,file);
    const replacements:{before:string,after:string}[]=[];let source=read(original);
    const replace=(before:string,after:string)=>{assert.equal(source.split(before).length,2,before);source=source.replace(before,after);replacements.push({before,after});};
    replace("import {buildBrowser} from './browser-presentation.ts';","import {buildIncrementalImageBrowser as buildBrowser} from './incremental-browser-image-build.ts';");
    replace("import {privateBrowserHost, observeGpu, validateGpu} from './private-browser-host.ts';","import {observeGpu,validateGpu} from './private-browser-host.ts';\nimport {privateImageBrowserHost as privateBrowserHost} from './parallel-image-browser-host.ts';\nimport {supportCommand,supportTarget,rebaseReferenceDependencies} from '"+pathToFileURL(self).href+"';");
    const start=source.indexOf('function command(label:');const end=source.indexOf('\nfunction nativeExample',start);assert(start>=0&&end>start);
    replace(source.slice(start,end),"function command(label:string,args:string[],cwd:string,env:Record<string,string>={}){return supportCommand(label,args,cwd,env);}");
    replace("const executable=path.join(viewer,'target/course-build',nativeTarget(),'debug/examples',name+(process.platform==='win32'?'.exe':''));", "const compiled=path.join(supportTarget,nativeTarget(),'debug/examples',name+(process.platform==='win32'?'.exe':''));\n    const executable=retained?retained.executable.path:path.join(viewer,'target/course-sup/retained',label,name+(process.platform==='win32'?'.exe':''));\n    fs.mkdirSync(path.dirname(executable),{recursive:true});\n    if(!retained){assert(!fs.existsSync(executable));fs.copyFileSync(compiled,executable,fs.constants.COPYFILE_EXCL);assert.equal(sha(compiled),sha(executable));}");
    replace('let cargo = read(cargoFile), lock = read(lockFile);','let cargo = rebaseReferenceDependencies(read(cargoFile),id,folder), lock = read(lockFile);');
    replace("{id,mode,folder,native,originalCapture:path.join(docs,'capture_journey.cjs')}","{canonicalBehaviorExecuted:false,id,mode,folder,native,originalCapture:path.join(docs,'capture_journey.cjs')}");
    const beforeImports=source;
    source=source.replace(/from '([.][^']+)'/g,(_,relative)=>"from '"+pathToFileURL(path.resolve(path.dirname(original),relative)).href+"'");
    replacements.push({before:beforeImports,after:source});
    let reversed=source;for(const change of [...replacements].reverse()){assert.equal(reversed.split(change.after).length,2);reversed=reversed.replace(change.after,change.before);}assert.equal(reversed,read(original));
    const extraInputs:Record<string,string>={};
    for(const name of ['model.ts','verify.ts','png.ts','reference.ts','private-browser-host.ts','parallel-image-browser-host.ts','incremental-browser-image-build.ts','exact-bindgen.ts']){const file=fileURLToPath(new URL('./'+name,import.meta.url));extraInputs[file]=sha(file);}
    for(const root of [path.join(viewer,'examples'),path.join(viewer,'assets')])for(const file of filesIn(root))extraInputs[file]=sha(file);
    const key=hash(JSON.stringify({old:ref(oldPlanFile),adapter:ref(self),source:hash(source),guard:ref(guard),extraInputs}));const folder=path.join(viewer,'target/course-presentation/private-support-'+key.slice(0,20));
    const privateHelper=path.join(folder,'supplemental-adapted.ts');write(privateHelper,source);
    const newInputs={...old.inputs,...extraInputs,[self]:sha(self),[privateHelper]:sha(privateHelper),[guard]:sha(guard)};
    for(const name of ['parallel-image-browser-host.ts','incremental-browser-image-build.ts']){const file=fileURLToPath(new URL('./'+name,import.meta.url));newInputs[file]=sha(file);}
    for(const root of [path.join(viewer,'examples'),path.join(viewer,'assets')])for(const file of filesIn(root))newInputs[file]=sha(file);
    const reverseProof={original:ref(original),adapted:ref(privateHelper),adapter:ref(self),replacements,reversesExactlyToOriginal:true,scope:'Execution transport/browser acquisition/dependency path relocation only; original native image recipes, source overlays, pixels and measurement/cache assertions remain byte-for-byte retained.'};
    write(path.join(folder,'adapter-proof.json'),JSON.stringify(reverseProof,null,2)+'\n');
    const plan={...old,status:'PREPARED_PRIVATE_IMAGE_TRANSPORT_NOT_EXECUTED',key,folder,attempt:'private8-'+key.slice(0,12),inputs:newInputs,originalPlan:ref(oldPlanFile),privateHelper:ref(privateHelper),adapterProof:ref(path.join(folder,'adapter-proof.json')),captureProfile:{processors:8,memoryMiB:8192,affinityMask:'0xFF00',cache:supportTarget,canonicalBehaviorExecuted:false,canonicalValidationProfile:false}};
    write(path.join(folder,'plan.json'),JSON.stringify(plan,null,2)+'\n');return plan;
}

export async function runPrivateSupport(planFile:string,scope:string,generationFile=''){
    assert(['practice','reference','dock','measurement','scenes'].includes(scope));const plan=json(path.resolve(planFile));
    for(const[file,digest]of Object.entries({...plan.inputs,...plan.generatorFiles}))assert.equal(sha(file),digest,file);
    if(scope==='scenes'){
        assert(generationFile,'Require actual separately completed C++ input generation');const generation=json(path.resolve(generationFile));assert.equal(generation.status,'PASS actual fresh source-variant generation; no historical output identity claim');
        for(const row of generation.results){assert.equal(row.exitCode,0);for(const f of [row.command,row.log,row.resources])assert.equal(sha(f.path),f.sha256);}
        const inputs:any={};for(const[name,key]of[['floor-output','floor'],['dome','dome']]){const row=generation.outputs[key];assert.equal(sha(row.output),row.sha256);inputs[name]={path:row.output,sha256:row.sha256};}
        write(path.join(plan.folder,'generated-inputs.json'),JSON.stringify({status:generation.status,sourceReceipt:ref(path.resolve(generationFile)),inputs},null,2)+'\n');
    }
    const module=await import(pathToFileURL(plan.privateHelper.path).href);const result=await module.runSupplemental(path.resolve(planFile),scope);
    const qualified={...result,originalFifteenDestinationStatus:result.status,privateScopeStatus:result.images.length===10?'TEN_SUPPORT_IMAGES_CAPTURED_AWAITING_ROOT_VISUAL_REVIEW':'PRIVATE_SUPPORT_PARTIAL',
        excludedScope:'Five already-published filename aliases are outside these ten captures; original15-destination result/status remains unchanged.',captureProfile:plan.captureProfile,adapterProof:plan.adapterProof};
    write(path.join(plan.folder,'private-result.json'),JSON.stringify(qualified,null,2)+'\n');return qualified;
}

if(process.argv[1]&&path.resolve(process.argv[1])===self){const[mode,file,scope,generation]=process.argv.slice(2);const result=mode==='prepare'?preparePrivateSupport(file):mode==='run'?await runPrivateSupport(file,scope,generation):null;assert(result,'supplemental-private-image.ts prepare OLD_PLAN | run PRIVATE_PLAN practice|reference|dock|measurement|scenes [ACTUAL_GENERATION_JSON]');console.log(JSON.stringify(result,null,2));}
