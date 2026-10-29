import {adapterMetadataPath, adapterMetadata, observeExecution, finishExecution, bundleInputs} from './capture-storage.ts';
import fs from 'node:fs';
import path from 'node:path';
import http from 'node:http';
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {fileURLToPath, pathToFileURL} from 'node:url';
import {hash, json, viewer, write} from './model.ts';

const guard = fileURLToPath(new URL('./parallel-image-guard.ps1', import.meta.url));
const sha = (file: string) => hash(fs.readFileSync(file));

export async function privateImageBrowserHost(workspace: string, folder: string, worker: string, config: any) {
    assert.equal(config.canonicalBehaviorExecuted, false, 'Separate lane is restricted to image-only workers');
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
    const inputs=Object.fromEntries([guard,worker,chrome,metadataFile,configFile].map(file=>[file,sha(file)]));
    const loader=fileURLToPath(new URL('../typescript.mjs',import.meta.url));
    const address=`http://127.0.0.1:${port}/${config.id}/dist/`;
    const workerArguments=config.workerArguments?config.workerArguments.map((argument:string)=>argument==='{VIEWER_URL}'?address:argument):['--worker',configFile];
    assert(workerArguments.every((argument:any)=>typeof argument==='string'));
    const arguments_=['--require',fileURLToPath(new URL('./warp-browser-timeout.cjs',import.meta.url)),'--import',pathToFileURL(loader).href,worker,...workerArguments];
    write(commandFile,JSON.stringify({executable:process.execPath,arguments:arguments_})+'\n');
    const log=path.join(folder,'image-browser.log'),descriptor=fs.openSync(log,'wx'),resourcesFile=path.join(folder,'image-resources.json');
    const beforeExecution = observeExecution([...bundleInputs(workspace, config.id), process.execPath, loader, worker, chrome, adapterMetadataPath(), configFile]);
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
