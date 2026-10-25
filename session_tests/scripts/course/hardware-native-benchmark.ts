import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {bindings, prepare as prepareSnapshot} from './presentation.ts';
import {assetsFor, expected, filesIn, hash, json, lockFor, materialize, read, viewer, write} from './model.ts';
import {nativeTarget} from './verify.ts';
import {png, ppm} from './png.ts';

const self=fileURLToPath(import.meta.url),sha=(file:string)=>hash(fs.readFileSync(file));
const imageGuard=fileURLToPath(new URL('./parallel-image-guard.ps1',import.meta.url));
const warpGuard=fileURLToPath(new URL('./capture-guard.ps1',import.meta.url));
const target=path.join(viewer,'target/course-hardware-image-build');
const adapterBefore='info.backend == wgpu::Backend::Dx12 && info.device_type == wgpu::DeviceType::Cpu\n                && info.vendor == 0x1414 && info.device == 0x8c';
const adapterAfter=`info.backend == wgpu::Backend::Dx12 && match std::env::var("COURSE_BENCHMARK_ADAPTER").as_deref() {
                Ok("nvidia") => info.vendor == 0x10de && info.device == 0x27e0 && info.device_type != wgpu::DeviceType::Cpu,
                Ok("warp") => info.vendor == 0x1414 && info.device == 0x8c && info.device_type == wgpu::DeviceType::Cpu,
                _ => false,
            }`;
const raw='    std::fs::write("frame.rgba", &pixels)?;\n';

function tool(args:string[]){const result=spawnSync(args[0],args.slice(1),{encoding:'utf8',windowsHide:true});assert.equal(result.status,0);return result.stdout.trim();}
function check(record:any){
    assert.deepEqual(bindings(record.id),record.source);
    for(const[file,digest]of Object.entries(record.pins))assert.equal(sha(file),digest,file);
    for(const[name,digest]of Object.entries(record.workspaceFiles))assert.equal(sha(path.join(record.workspace,name)),digest,name);
    const original=read(record.originalDriver.path),candidate=read(path.join(record.workspace,'examples/presentation.rs'));
    assert.equal(candidate.replace(adapterAfter,adapterBefore).replace('Requested explicit benchmark adapter unavailable','Windows frame evidence requires the Microsoft WARP DX12 adapter').replace(raw,''),original,'Complete reverse adapter/readback diff');
}

export function prepareHardwareBenchmark(id:string){
    assert(['43b-view','59f-cad','96zz-separate-hidden-ink-color-from-pick-ownership'].includes(id));
    const lane=process.env.COURSE_HARDWARE_IMAGE_LANE||'image';assert(['image','original'].includes(lane));
    const snapshot=prepareSnapshot(id,2),source=bindings(id),dxc=path.resolve(process.env.VIEWER_DXC_PATH||path.join(viewer,'target/course-windows-20261018/native-checkpoint/dxcompiler.dll'));
    const pins=Object.fromEntries([self,imageGuard,warpGuard,path.join(snapshot.directory,'presentation.json'),path.join(viewer,snapshot.driver.path),dxc,path.join(path.dirname(dxc),'dxil.dll')].map(file=>[file,sha(file)]));
    const profile={lane,processors:8,memoryMiB:8192,jobs:8,target,host:nativeTarget(),rustc:tool(['rustc','-vV']),cargo:tool(['cargo','--version']),
        dxc:{path:dxc,sha256:sha(dxc),dxilSha256:sha(path.join(path.dirname(dxc),'dxil.dll'))},
        nvidia:{vendor:0x10de,device:0x27e0,nonCpu:true,affinityMask:lane==='original'?'0xFF':'0xFF00',guard:lane==='original'?warpGuard:imageGuard},
        warp:{vendor:0x1414,device:0x8c,cpu:true,affinityMask:'0xFF',guard:warpGuard}};
    const key=hash(JSON.stringify({source,pins,profile})),folder=path.join(viewer,'target/course-presentation/hardware-native-benchmark',id+'-'+key.slice(0,20)),file=path.join(folder,'prepared.json');
    if(fs.existsSync(file)){const retained=json(file);check(retained);return retained;}
    const workspace=path.join(folder,'workspace');materialize(expected(id),workspace,lockFor(id),assetsFor(id));
    const originalDriver=path.join(viewer,snapshot.driver.path),original=read(originalDriver);
    assert.equal(original.split(adapterBefore).length,2);assert.equal(original.split('    let mut output = std::fs::File::create("frame.ppm")?;').length,2);
    const candidate=original.replace(adapterBefore,adapterAfter).replace('Windows frame evidence requires the Microsoft WARP DX12 adapter','Requested explicit benchmark adapter unavailable')
        .replace('    let mut output = std::fs::File::create("frame.ppm")?;',raw+'    let mut output = std::fs::File::create("frame.ppm")?;');
    write(path.join(workspace,'examples/presentation.rs'),candidate);
    const cargoFile=path.join(workspace,'Cargo.toml'),lockFile=path.join(workspace,'Cargo.lock'),cargo=read(cargoFile),lock=read(lockFile),old=cargo.match(/^name\s*=\s*"([^"]+)"/m)![1],unique='hardware_image_'+source.files.slice(0,16);
    assert.equal(lock.split('name = "'+old+'"').length,2);
    write(cargoFile,cargo.replace('name = "'+old+'"','name = "'+unique+'"').replace('[lib]','[lib]\nname = "viewer_journey"'));
    write(lockFile,lock.replace('name = "'+old+'"','name = "'+unique+'"'));
    const workspaceFiles=Object.fromEntries(filesIn(workspace).map(file=>[path.relative(workspace,file).split(path.sep).join('/'),sha(file)]));
    const record={status:'PREPARED_NOT_EXECUTED',id,source,folder,workspace,profile,pins,workspaceFiles,
        originalDriver:{path:originalDriver,sha256:sha(originalDriver)},driver:{path:path.join(workspace,'examples/presentation.rs'),sha256:hash(candidate)},
        originalSnapshot:path.join(snapshot.directory,'presentation.json'),originalImage:(snapshot as any).image||null,
        geometry:'Every renderer, shader, fixture, lock and asset is unchanged. Cargo package/library identity overlay only; driver differs by exact reversible adapter selection and passive raw RGBA write.',
        scope:'Finite image-profile benchmark only. No canonical test, WARP proof or bulk speed acceptance is inferred.'};
    write(file,JSON.stringify(record,null,2)+'\n');check(record);return record;
}

function execute(record:any,label:string,args:string[],cwd:string,role:'nvidia'|'warp',extra:Record<string,string>={}){
    check(record);const folder=path.join(record.folder,label),guard=record.profile[role].guard;
    const log=path.join(folder,'output.log'),resourcesFile=path.join(folder,'resources.json'),commandFile=path.join(folder,'command.json');assert(!fs.existsSync(log));
    write(commandFile,JSON.stringify({executable:args[0],arguments:args.slice(1)})+'\n');const fd=fs.openSync(log,'wx');let result;
    try{result=spawnSync('powershell.exe',['-NoProfile','-ExecutionPolicy','Bypass','-File',guard,'-ProcessorCount','8','-MemoryLimitMiB','8192','-TimeoutSeconds','1800',
        '-WorkingDirectory',cwd,'-ManifestPath',resourcesFile,'-CommandFile',commandFile],{cwd,windowsHide:true,stdio:['ignore',fd,fd],env:{...process.env,CARGO_TARGET_DIR:target,CARGO_BUILD_JOBS:'8',CARGO_INCREMENTAL:'1',CARGO_NET_OFFLINE:'true',RUSTC_WRAPPER:'',REGEN_PROTO:'0',RUST_TEST_THREADS:'1',...extra}});}finally{fs.closeSync(fd);}
    assert.equal(result.status,0,log);const resources=json(resourcesFile);assert.equal(resources.exitCode,0);assert.equal(resources.processors,8);assert.equal(resources.jobCommitLimitBytes,8589934592);
    assert.equal(resources.affinityMask,record.profile[role].affinityMask);check(record);
    return{role,args,guard,guardSha256:sha(guard),log,logSha256:sha(log),resourcesFile,resourcesSha256:sha(resourcesFile),resources};
}

export function buildHardwareBenchmark(id:string){
    const record=prepareHardwareBenchmark(id),file=path.join(record.folder,'built.json');
    if(fs.existsSync(file)){const built=json(file);check(record);assert.equal(sha(built.executable.path),built.executable.sha256);return built;}
    const build=execute(record,'build',['cargo','build','--example','presentation','--locked','--offline','--target',record.profile.host,'-j8','--message-format=json-render-diagnostics'],record.workspace,'nvidia');
    const artifacts=read(build.log).split(/\r?\n/).flatMap(line=>{try{return[JSON.parse(line)];}catch{return[];}}).filter(row=>row.reason==='compiler-artifact'&&row.target?.name==='presentation'&&row.executable);
    assert.equal(artifacts.length,1);const artifact=artifacts[0];assert.equal(path.resolve(artifact.manifest_path),path.join(record.workspace,'Cargo.toml'));assert.equal(path.resolve(artifact.target.src_path),record.driver.path);
    const executable=path.join(record.folder,'bin/presentation.exe');write(executable,fs.readFileSync(artifact.executable));assert.equal(sha(executable),sha(artifact.executable));
    for(const name of['dxcompiler.dll','dxil.dll'])write(path.join(path.dirname(executable),name),fs.readFileSync(path.join(path.dirname(record.profile.dxc.path),name)));
    const built={...record,status:'BUILT_NOT_EXECUTED',build,artifact,executable:{path:executable,sha256:sha(executable)}};
    write(file,JSON.stringify(built,null,2)+'\n');return built;
}

export function runHardwareBenchmark(id:string,role:'nvidia'|'warp'){
    assert(['nvidia','warp'].includes(role));const prepared=prepareHardwareBenchmark(id),built=json(path.join(prepared.folder,'built.json'));check(prepared);assert.equal(sha(built.executable.path),built.executable.sha256);
    const file=path.join(prepared.folder,role+'.json');if(fs.existsSync(file)){const retained=json(file);assert.equal(sha(retained.image.path),retained.image.sha256);assert.equal(sha(retained.rgba.path),retained.rgba.sha256);return retained;}
    const runtime=path.join(prepared.folder,role+'-runtime');assert(!fs.existsSync(runtime));fs.mkdirSync(runtime);
    for(const name of assetsFor(id))write(path.join(runtime,name),fs.readFileSync(path.join(prepared.workspace,name)));
    const executed=execute(prepared,'execute-'+role,[built.executable.path],runtime,role,{COURSE_BENCHMARK_ADAPTER:role,VIEWER_DXC_PATH:path.join(path.dirname(built.executable.path),'dxcompiler.dll')});
    const text=read(executed.log),adapter=text.match(/Native frame adapter: (AdapterInfo \{[^\r\n]+\})/)?.[1];assert(adapter);
    assert(adapter.includes('backend: Dx12'));
    if(role==='nvidia'){assert.match(adapter,/name: "NVIDIA/);assert.match(adapter,/vendor: 4318\b/);assert.match(adapter,/device: 10208\b/);assert(!adapter.includes('device_type: Cpu'));}
    else{assert.match(adapter,/vendor: 5140\b/);assert.match(adapter,/device: 140\b/);assert(adapter.includes('device_type: Cpu'));}
    const frame=path.join(runtime,'frame.ppm'),pixels=ppm(frame),rgba=path.join(runtime,'frame.rgba');assert.equal(pixels.width,1280);assert.equal(pixels.height,960);assert.equal(fs.statSync(rgba).size,1280*960*4);
    const rgbaBytes=fs.readFileSync(rgba);for(let i=0;i<pixels.pixels.length;i++)assert.equal(pixels.pixels[i],rgbaBytes[Math.floor(i/3)*4+i%3]);
    const image=path.join(runtime,id+'-'+role+'.png');write(image,png(frame));
    const originalAgreement=prepared.originalImage?hash(fs.readFileSync(path.resolve(viewer,prepared.originalImage.path)))===sha(image):null;
    if(role==='warp'&&prepared.originalImage)assert.equal(originalAgreement,true,'Reversible WARP variant must reproduce retained original RGB image');
    const result={status:'ACTUAL_IMAGE_AWAITING_ROOT_REVIEW',id,role,source:prepared.source,driver:prepared.driver,adapter,executed,executable:built.executable,
        rgba:{path:rgba,sha256:sha(rgba)},image:{path:image,sha256:sha(image),width:1280,height:960},matchesRetainedOriginalRGB:originalAgreement};
    write(file,JSON.stringify(result,null,2)+'\n');return result;
}

export function compareHardwareBenchmark(id:string){
    const prepared=prepareHardwareBenchmark(id),warp=json(path.join(prepared.folder,'warp.json')),nvidia=json(path.join(prepared.folder,'nvidia.json'));
    assert.deepEqual(warp.source,nvidia.source);assert.deepEqual(warp.executable,nvidia.executable);
    const a=fs.readFileSync(warp.rgba.path),b=fs.readFileSync(nvidia.rgba.path);assert.equal(hash(a),warp.rgba.sha256);assert.equal(hash(b),nvidia.rgba.sha256);assert.equal(a.length,b.length);
    let differingPixels=0,maxChannelDelta=0,totalAbsoluteDelta=0;const bbox=[1280,960,-1,-1];
    for(let i=0;i<a.length;i+=4){let changed=false;for(let c=0;c<4;c++){const delta=Math.abs(a[i+c]-b[i+c]);maxChannelDelta=Math.max(maxChannelDelta,delta);totalAbsoluteDelta+=delta;changed ||= delta>0;}if(changed){differingPixels++;const x=i/4%1280,y=Math.floor(i/4/1280);bbox[0]=Math.min(bbox[0],x);bbox[1]=Math.min(bbox[1],y);bbox[2]=Math.max(bbox[2],x);bbox[3]=Math.max(bbox[3],y);}}
    const result={status:'MEASURED_AWAITING_ROOT_GEOMETRY_AND_VISUAL_REVIEW',id,source:prepared.source,warp,nvidia,
        rgba:{differingPixels,maxChannelDelta,meanAbsoluteDelta:totalAbsoluteDelta/a.length,bbox:differingPixels?bbox:null,byteIdentical:differingPixels===0},
        seconds:{warp:warp.executed.resources.elapsedSeconds,nvidia:nvidia.executed.resources.elapsedSeconds},
        limits:`Explicit affinity WARP${prepared.profile.warp.affinityMask}/NVIDIA${prepared.profile.nvidia.affinityMask}; concurrent image work may affect timing. No isolated GPU causal claim, bulk adoption or WARP/canonical-proof relabel.`};
    write(path.join(prepared.folder,'comparison.json'),JSON.stringify(result,null,2)+'\n');return result;
}

if(process.argv[1]&&path.resolve(process.argv[1])===self){const args=process.argv.slice(2),lane=args.indexOf('--lane');if(lane>=0){process.env.COURSE_HARDWARE_IMAGE_LANE=args[lane+1];args.splice(lane,2);}const[mode,id,role]=args;
    const result=mode==='prepare'?prepareHardwareBenchmark(id):mode==='build'?buildHardwareBenchmark(id):mode==='run'?runHardwareBenchmark(id,role as any):mode==='compare'?compareHardwareBenchmark(id):null;
    assert(result,'hardware-native-benchmark.ts prepare|build|compare ID | run ID nvidia|warp');console.log(JSON.stringify(result,null,2));}
