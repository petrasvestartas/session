import {adapterMetadataPath, adapterMetadata, observeExecution, finishExecution, bundleInputs} from './capture-storage.ts';
import fs from 'node:fs';
import path from 'node:path';
import http from 'node:http';
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {fileURLToPath, pathToFileURL} from 'node:url';
import {buildImageBrowser} from './image-browser-build.ts';
import {bindings} from './presentation.ts';
import {assetsFor, expected, hash, json, read, viewer, withExpectedSequence, write} from './model.ts';

const self = fileURLToPath(import.meta.url);
const guard = fileURLToPath(new URL('./parallel-image-guard.ps1', import.meta.url));
const sha = (file: string) => hash(fs.readFileSync(file));
const reference = (file: string) => ({path:path.resolve(file),sha256:sha(file)});

function check(prepared: any) {
    assert.deepEqual(withExpectedSequence(() => bindings(prepared.id)), prepared.source);
    for (const [file,digest] of Object.entries(prepared.pins)) assert.equal(sha(file),digest,file);
    assert.equal(sha(prepared.worker),prepared.workerSha256);
    assert.equal(sha(prepared.adapted),prepared.adaptedSha256);
    const kernel = path.relative(prepared.native,path.resolve(viewer,'../session_rust')).split(path.sep).join('/');
    for (const [name,text] of Object.entries(withExpectedSequence(() => expected(prepared.id))))
        assert.equal(read(path.join(prepared.native,name)), name==='Cargo.toml'?text.replaceAll('../../../session_rust',kernel):text,name);
    for (const name of assetsFor(prepared.id)) assert.equal(sha(path.join(prepared.native,name)),sha(path.join(viewer,name)),name);
    for (const [name,digest] of Object.entries(prepared.bundle)) assert.equal(sha(path.join(prepared.workspace,'target/course-checks',prepared.id,'dist',name)),digest,name);
}

async function host(workspace: string, folder: string, worker: string, config: any) {
    assert.equal(config.canonicalBehaviorExecuted,true);
    assert.equal(config.canonicalValidationProfile,false);
    const base = path.join(workspace, 'target/course-checks'), served: Record<string, string> = {};
    const types: Record<string, string> = {'.html':'text/html','.js':'text/javascript','.wasm':'application/wasm','.json':'application/json','.ttf':'font/ttf'};
    const server = http.createServer((request,response) => {
        try {
            const url = new URL(request.url!, 'http://localhost');
            if (url.pathname === '/favicon.ico') { response.writeHead(204).end(); return; }
            const file = path.resolve(base, '.' + decodeURIComponent(url.pathname));
            assert(file.startsWith(base + path.sep));
            const actual = fs.statSync(file).isDirectory() ? path.join(file,'index.html') : file;
            const bytes = fs.readFileSync(actual);served[path.relative(base,actual).split(path.sep).join('/')] = hash(bytes);
            response.writeHead(200,{'Content-Type':types[path.extname(actual)]||'application/octet-stream','Cache-Control':'no-store'}).end(bytes);
        }catch{response.writeHead(404).end('Not found');}
    });
    await new Promise<void>(resolve=>server.listen(0,'127.0.0.1',resolve));
    const port=(server.address() as {port:number}).port;
    const metadataFile=adapterMetadataPath(),metadata=adapterMetadata();
    assert.equal(metadata.selected.vendor,0x10de);assert.equal(metadata.selected.device,0x27e0);
    const chrome=process.env.VIEWER_BROWSER_EXECUTABLE||path.join(viewer,'target/course-browsers/chromium_headless_shell-1208/chrome-headless-shell-win64/chrome-headless-shell.exe');
    const configFile=path.join(folder,'config.json'),commandFile=path.join(folder,'image-worker.command.json');write(configFile,JSON.stringify(config,null,2)+'\n');
    const inputs=Object.fromEntries([self,guard,worker,chrome,metadataFile,configFile].map(file=>[file,sha(file)]));
    const loader=fileURLToPath(new URL('../typescript.mjs',import.meta.url));
    const address=`http://127.0.0.1:${port}/${config.id}/dist/`;
    const workerArguments: string[] = [];
    assert(workerArguments.every((argument:any)=>typeof argument==='string'));
    const arguments_=['--require',fileURLToPath(new URL('./warp-browser-timeout.cjs',import.meta.url)),'--import',pathToFileURL(loader).href,worker,...workerArguments];
    write(commandFile,JSON.stringify({executable:process.execPath,arguments:arguments_})+'\n');
    const log=path.join(folder,'image-browser.log'),descriptor=fs.openSync(log,'wx'),resourcesFile=path.join(folder,'image-resources.json');
    const beforeExecution = observeExecution([...bundleInputs(workspace, config.id), process.execPath, loader, worker, chrome, metadataFile, configFile]);
    try{
        const args=['-NoProfile','-ExecutionPolicy','Bypass','-File',guard,'-ProcessorCount','8','-MemoryLimitMiB','8192','-TimeoutSeconds','2400',
            '-WorkingDirectory',workspace,'-ManifestPath',resourcesFile,'-CommandFile',commandFile];
        const code=await new Promise((resolve,reject)=>{
            const child=spawn('powershell.exe',args,{cwd:workspace,windowsHide:true,stdio:['ignore',descriptor,descriptor],env:{...process.env,
                NODE_PATH:path.join(viewer,'target/course-tools/node_modules'),COURSE_SAMPLES_PREPARED:'1',VIEWER_HEADLESS:'1',
                VIEWER_WARP_ACTION_TIMEOUT_MS:'120000',VIEWER_BROWSER_EXECUTABLE:chrome,
                JOURNEY_URL:`http://127.0.0.1:${port}/`,VIEWER_URL:`http://127.0.0.1:${port}/${config.id}/dist/`,VIEWER_DIST:path.join(base,config.id,'dist'),
                VIEWER_CHROME_ARGS:JSON.stringify(['--enable-unsafe-webgpu','--ignore-gpu-blocklist','--use-gl=angle','--use-angle=d3d11',
                    '--use-adapter-luid='+metadata.selected.luid,'--enable-dawn-features=d3d_skip_shader_optimizations'])}});
            child.once('error',reject);child.once('exit',resolve);
        });
        assert.equal(code,0,log);const resources=json(resourcesFile);
        assert.equal(resources.exitCode,0);assert.equal(resources.processors,8);assert.equal(resources.jobCommitLimitBytes,8589934592);
        assert.equal(resources.laneMutex,'Local\\SessionViewerCourseImageJob');
        assert.equal(resources.affinityMask,'0xFF00');
        for(const[file,digest]of Object.entries(inputs))assert.equal(sha(file),digest,file);
        return{executionIntegrity:finishExecution(beforeExecution),served,resources,log,logSha256:sha(log),resourcesFile,resourcesSha256:sha(resourcesFile),inputs,
            policy:'One image lane8CPU8GiB alongside at most one original controlled lane<=8CPU8GiB. Aggregate OUR guarded jobs<=16CPU16GiB; no whole-machine/external-process claim.'};
    }finally{fs.closeSync(descriptor);await new Promise<void>(resolve=>server.close(()=>resolve()));}
}

export async function captureFallback(id: string) {
    const built = withExpectedSequence(() => buildImageBrowser(id,2)); check(built);
    const profile = {purpose:'Image generation with unchanged original callback assertions',canonicalBehaviorExecuted:true,canonicalValidationProfile:false,
        processors:8,memoryMiB:8192,affinityMask:'0xFF00',helper:reference(self),guard:reference(guard),worker:reference(built.worker),adapted:reference(built.adapted)};
    const key=hash(JSON.stringify({source:built.source,bundle:built.bundle,profile})).slice(0,20);
    const folder=path.join(viewer,'target/course-presentation/image-fallback',id+'-'+key),file=path.join(folder,'capture.json');
    if(fs.existsSync(file)) {
        const prior=json(file);assert.deepEqual(prior.source,built.source);assert.equal(sha(prior.image.path),prior.image.sha256);
        assert.equal(sha(prior.host.log),prior.host.logSha256);assert.equal(sha(prior.host.resourcesFile),prior.host.resourcesSha256);
        assert.equal(prior.host.resources.exitCode,0);return prior;
    }
    assert(!fs.existsSync(folder),'Preserve failed image-profile callback attempt');
    const config={id,source:built.source,...profile,built:reference(path.join(built.folder,'prepared.json'))};
    write(path.join(folder,'prepared.json'),JSON.stringify(config,null,2)+'\n');
    const hosted=await host(built.workspace,folder,built.worker,config);check(built);
    const imageFile=path.join(built.workspace,'docs/screenshots/journey',id+'-browser.png'),bytes=fs.readFileSync(imageFile);
    const presentation=json(path.join(built.workspace,'docs/screenshots/journey',id+'-presentation.json'));
    assert.equal(bytes.readUInt32BE(16),presentation.viewport.width*2);assert.equal(bytes.readUInt32BE(20),presentation.viewport.height*2);
    assert.equal(presentation.behaviorDpr,1);assert.equal(presentation.imageDpr,2);
    assert.equal(hosted.served[id+'/dist/index.html'],built.bundle['index.html']);
    for(const name of Object.keys(built.bundle).filter(name=>name.endsWith('.wasm'))) assert.equal(hosted.served[id+'/dist/'+name],built.bundle[name]);
    const record={status:'IMAGE_PROFILE_CALLBACK_PASSED_AWAITING_ROOT_VISUAL_REVIEW',id,folder,source:built.source,bundle:built.bundle,
        canonicalBehaviorExecuted:true,canonicalValidationProfile:false,
        scope:'Unchanged original callback assertions ran at DPR1 under image-only8CPU8GiB resources, followed by original DPR2 presentation. This is not original2CPU canonical-validation acceptance.',
        profile,built:config.built,worker:profile.worker,adapted:profile.adapted,presentation,host:hosted,resources:hosted.resources,
        image:{path:imageFile,sha256:hash(bytes),width:bytes.readUInt32BE(16),height:bytes.readUInt32BE(20)}};
    write(file,JSON.stringify(record,null,2)+'\n');return record;
}

if(process.argv[1]&&path.resolve(process.argv[1])===self){
    assert.equal(process.argv[2],'capture','Usage: browser-image-fallback.ts capture ID');
    console.log(JSON.stringify(await captureFallback(process.argv[3]),null,2));
}
