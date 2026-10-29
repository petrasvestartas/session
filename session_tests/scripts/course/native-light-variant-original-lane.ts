import {cargoTarget} from './capture-storage.ts';
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {prepare, bindings} from './presentation.ts';
import {filesIn, hash, json, viewer, write} from './model.ts';
import {nativeTarget} from './verify.ts';
import {png, ppm} from './png.ts';

const self=fileURLToPath(import.meta.url),guard=fileURLToPath(new URL('./capture-guard.ps1',import.meta.url));
const sha=(file:string)=>hash(fs.readFileSync(file));
const tree=(folder:string)=>Object.fromEntries(filesIn(folder).filter(f=>!path.relative(folder,f).split(path.sep).includes('target')&&path.relative(folder,f)!=='frame.ppm').map(f=>[path.relative(folder,f).replaceAll('\\','/'),sha(f)]));

export function prepareNativeLightVariant(){
    const original=prepare('04-input-light',2),source=bindings('04-input');
    assert.deepEqual(original.source,source);
    const identity={source,driver:original.driver.sha256,helper:sha(self),guard:sha(guard),profile:{processors:8,memoryMiB:8192,affinity:'0xFF',adapter:'original Microsoft WARP DX12'}};
    const folder=path.join(viewer,'target/course-presentation/native-light-private-'+hash(JSON.stringify(identity)).slice(0,20)),file=path.join(folder,'prepared.json');
    if(fs.existsSync(file)){const p=json(file);check(p);return p;}
    assert(!fs.existsSync(folder));fs.mkdirSync(folder,{recursive:true});
    const workspace=path.join(folder,'workspace');
    fs.cpSync(original.directory,workspace,{recursive:true,filter:f=>!path.relative(original.directory,f).split(path.sep).includes('target')&&!['frame.ppm','presentation.json'].includes(path.basename(f))});
    const sourceFiles=tree(workspace);assert.equal(sourceFiles['examples/presentation.rs'],original.driver.sha256);
    const dxc=path.join(viewer,'target/course-windows-20261018/native-checkpoint/dxcompiler.dll'),dxil=path.join(path.dirname(dxc),'dxil.dll');
    const p={status:'PREPARED_PRIVATE_SOURCE_ONLY',id:'04-input-light',folder,workspace,target:cargoTarget(),source,identity,sourceFiles,original:{path:path.join(original.directory,'presentation.json'),sha256:sha(path.join(original.directory,'presentation.json'))},pins:Object.fromEntries([self,guard,dxc,dxil].map(f=>[f,sha(f)])),dxc,driver:original.driver,width:1280,height:960};
    write(file,JSON.stringify(p,null,2)+'\n');check(p);return p;
}

function check(p:any){
    assert.deepEqual(bindings('04-input'),p.source);
    for(const[f,h]of Object.entries(p.pins))assert.equal(sha(f),h,f);
    assert.deepEqual(tree(p.workspace),p.sourceFiles);
    assert.equal(sha(p.original.path),p.original.sha256);
    assert(path.resolve(p.target).startsWith(path.resolve(p.folder)+path.sep));
}

export function runNativeLightVariant(file:string){
    const p=json(file);check(p);assert(!fs.existsSync(path.join(p.folder,'resources.json')),'Preserve prior execution');
    const commandFile=path.join(p.folder,'command.json'),resourcesFile=path.join(p.folder,'resources.json');
    write(commandFile,JSON.stringify({executable:'cargo',arguments:['run','--example','presentation','--locked','--offline','--target',nativeTarget(),'-j1','-vv','--message-format=json-render-diagnostics']},null,2)+'\n');
    const env={...process.env,CARGO_TARGET_DIR:p.target,CARGO_NET_OFFLINE:'true',RUSTC_WRAPPER:'',VIEWER_DXC_PATH:p.dxc,NO_COLOR:'true'};
    const result=spawnSync('powershell.exe',['-NoProfile','-ExecutionPolicy','Bypass','-File',guard,'-ProcessorCount','8','-MemoryLimitMiB','8192','-TimeoutSeconds','1800','-ManifestPath',resourcesFile,'-WorkingDirectory',p.workspace,'-CommandFile',commandFile],{cwd:viewer,env,encoding:'utf8',windowsHide:true,maxBuffer:64*1024*1024});
    write(path.join(p.folder,'stdout.log'),result.stdout||'');write(path.join(p.folder,'stderr.log'),result.stderr||'');
    check(p);assert.equal(result.status,0);const resources=json(resourcesFile);assert.equal(resources.exitCode,0);assert.equal(resources.timedOut,false);
    const messages=(result.stdout||'').split(/\r?\n/).flatMap(line=>{try{return[JSON.parse(line)];}catch{return[];}});
    const artifact=messages.find(r=>r.reason==='compiler-artifact'&&r.target?.name==='presentation'&&r.executable);assert(artifact,'Actual private example artifact required');assert.equal(artifact.fresh,false);assert.equal(path.resolve(artifact.target.src_path),path.join(p.workspace,'examples/presentation.rs'));
    const library=messages.find(r=>r.reason==='compiler-artifact'&&r.target?.name==='viewer_journey'&&path.resolve(r.target.src_path)===path.join(p.workspace,'src/lib.rs'));assert(library,'Actual private library source required');assert.equal(library.fresh,false);
    assert((result.stderr||'').includes('vendor: 5140')&&(result.stderr||'').includes('device: 140')&&(result.stderr||'').includes('device_type: Cpu'),'Actual original WARP adapter required');
    const frame=path.join(p.workspace,'frame.ppm'),pixels=ppm(frame);assert.equal(pixels.width,1280);assert.equal(pixels.height,960);
    const near=(at:number,wanted:number[])=>wanted.every((value,i)=>Math.abs(pixels.pixels[at+i]-value)<=2);
    assert(near(0,[243,243,243]));assert(near((480*1280+640)*3,[243,137,179]));let changed=0;for(let at=0;at<pixels.pixels.length;at+=3)if(!near(at,[243,243,243]))changed++;assert(changed>=196000&&changed<208000,'Original triangle coverage scaled to DPR2');
    const image=path.join(p.folder,'04-input-light-dpr2.png');write(image,png(frame));
    const record={...p,status:'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW',canonicalValidationProfile:false,scope:'Exact original source/driver/Background.toggle light variant, wholly private target and workspace. Original WARP algorithm retained; no shared active native workbench or target copied.',resources,artifact:{path:artifact.executable,sha256:sha(artifact.executable),compilerMessage:artifact},library,image:{path:image,sha256:sha(image),width:1280,height:960},semantic:{background:[243,243,243],triangleCentre:[243,137,179],changedPixels:changed},destinations:['docs/screenshots/journey/04-input-light.png']};
    write(path.join(p.folder,'capture.json'),JSON.stringify(record,null,2)+'\n');return record;
}

if(process.argv[1]&&path.resolve(process.argv[1])===self){const mode=process.argv[2];assert(['prepare','run'].includes(mode));console.log(JSON.stringify(mode==='prepare'?prepareNativeLightVariant():runNativeLightVariant(path.resolve(process.argv[3])),null,2));}
