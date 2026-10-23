const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const {execFileSync} = require('node:child_process');
const {chromium} = require('playwright');

async function main() {
    const [before, after, destination] = process.argv.slice(2);
    assert(before && after && destination, 'Usage: measure-runtime.cjs BEFORE_URL AFTER_URL OUTPUT');
    for (const url of [before, after]) assert(['127.0.0.1', 'localhost'].includes(new URL(url).hostname));
    const output = path.resolve(destination);
    fs.mkdirSync(output, {recursive: true});
    const warp = JSON.parse(execFileSync('powershell.exe', ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File',
        path.join(__dirname, '../warp-adapter.ps1')], {encoding:'utf8'}));
    const args = ['--enable-unsafe-webgpu', '--ignore-gpu-blocklist', '--use-gl=angle', '--use-angle=d3d11',
        '--use-adapter-luid=' + warp.luid, '--enable-dawn-features=d3d_skip_shader_optimizations'];
    let browser;
    const report = {scope:'Fresh-browser Windows WARP software GPU measurements; OS driver caches are not cleared; no hardware FPS claim', browser:null,
        args, viewport:{width:900,height:760}, deviceScaleFactor:1, samples:[], status:'running'};
    const save = () => fs.writeFileSync(path.join(output, 'manifest.json'), JSON.stringify(report,null,2) + '\n');
    save();
    try {
        for (const [label,url] of [['before',before],['after',after]]) {
            browser=await chromium.launch({executablePath:process.env.VIEWER_BROWSER_EXECUTABLE,headless:true,args});
            report.browser=browser.version();
            const context = await browser.newContext({viewport:report.viewport,deviceScaleFactor:1});
            const page = await context.newPage(), errors=[];
            page.on('pageerror', error => errors.push(error.message));
            page.on('console', message => {if(message.type()==='error') errors.push(message.text());});
            await page.addInitScript(() => {
                const counts={frames:0,submits:0,completed:0,buffers:0,textures:0,writeBytes:0}, queues=new Set();
                const timing={lastCompletedAt:0,lastPresentedAt:0};
                const fences={interaction:null,limit:256,records:[],dropped:0};
                const submit=GPUQueue.prototype.submit, write=GPUQueue.prototype.writeBuffer;
                const buffer=GPUDevice.prototype.createBuffer, texture=GPUDevice.prototype.createTexture;
                const current=GPUCanvasContext.prototype.getCurrentTexture;
                GPUQueue.prototype.submit=function(commands){queues.add(this);counts.submits++;
                    let record;
                    if(fences.interaction!==null){
                        if(fences.records.length<fences.limit){record={interaction:fences.interaction,
                            submission:counts.submits,frame:counts.frames,textureAcquiredAt:timing.lastPresentedAt,
                            submittedAt:performance.now(),submitReturnedAt:null,completedAt:null,error:null};
                            fences.records.push(record);
                        }else fences.dropped++;
                    }
                    const result=submit.call(this,commands);
                    if(record)record.submitReturnedAt=performance.now();
                    this.onSubmittedWorkDone().then(()=>{
                        counts.completed++;timing.lastCompletedAt=performance.now();
                        if(record)record.completedAt=timing.lastCompletedAt;
                    },error=>{if(record)record.error=String(error);});return result;};
                GPUQueue.prototype.writeBuffer=function(...args){const data=args[2];
                    const element=data.BYTES_PER_ELEMENT||1;
                    counts.writeBytes+=args[4]===undefined ? data.byteLength-(args[3]||0)*element : args[4]*element;
                    return write.apply(this,args);};
                GPUDevice.prototype.createBuffer=function(...args){counts.buffers++;return buffer.apply(this,args);};
                GPUDevice.prototype.createTexture=function(...args){counts.textures++;return texture.apply(this,args);};
                GPUCanvasContext.prototype.getCurrentTexture=function(...args){counts.frames++;timing.lastPresentedAt=performance.now();return current.apply(this,args);};
                window.__runtimeMeasure={counts,queues,timing,fences};
            });
            const started=Date.now();
            await page.goto(url,{waitUntil:'domcontentloaded',timeout:30000});
            await page.waitForFunction(()=>window.wasmBindings?.runtime_running?.()
                && JSON.parse(window.wasmBindings.diagnostic_snapshot()).outcome==='ready',null,{timeout:180000});
            const coldLoadReadyWallMs=Date.now()-started;
            const adapter=await page.evaluate(async()=>{const info=(await navigator.gpu.requestAdapter()).info;
                return {vendor:info.vendor,architecture:info.architecture,isFallbackAdapter:info.isFallbackAdapter};});
            assert.deepEqual(adapter,{vendor:'microsoft',architecture:'warp',isFallbackAdapter:true});
            const measured=await page.evaluate(async()=>{
                const {counts,queues,timing,fences}=window.__runtimeMeasure, canvas=document.querySelector('canvas');
                const frame=()=>new Promise(resolve=>requestAnimationFrame(resolve));
                const bounded=async promise=>{let timer;try{return await Promise.race([promise,new Promise((_,reject)=>{
                    timer=setTimeout(()=>reject(Error('GPU/frame measurement timed out')),15000);
                })]);}finally{clearTimeout(timer);}};
                const settled=async()=>{
                    const start=performance.now();let stable=0;
                    while(stable<2){const before=[counts.frames,counts.submits];await bounded(frame());
                        await bounded(Promise.all([...queues].map(queue=>queue.onSubmittedWorkDone())));await bounded(frame());
                        stable=before[0]===counts.frames&&before[1]===counts.submits ? stable+1 : 0;
                        if(performance.now()-start>15000)throw Error('Presentation did not settle');}
                };
                await settled();
                const initial=JSON.parse(window.wasmBindings.diagnostic_snapshot());
                const matrix=()=>JSON.parse(canvas.getAttribute('data-camera-matrix'));
                const geometry=()=>Object.fromEntries(['data-object-count','data-row-placements','data-gpu-stats',
                    'data-point-gpu','data-path-gpu','data-stroke-usage'].map(name=>[name,canvas.getAttribute(name)]));
                const initialGeometry=geometry(),initialMatrix=matrix();
                const rect=canvas.getBoundingClientRect();
                const wheel=delta=>canvas.dispatchEvent(new WheelEvent('wheel',{deltaY:delta,deltaMode:0,
                    clientX:rect.left+rect.width/2,clientY:rect.top+rect.height/2,bubbles:true,cancelable:true}));
                const interaction=[];
                for(const delta of [50,-20,40,-40,20,-50]) {
                    fences.interaction=interaction.length+1;
                    const frames=counts.frames,start=performance.now();wheel(delta);
                    while(counts.frames===frames){if(performance.now()-start>15000)throw Error('No input frame');await frame();}
                    await settled();interaction.push({inputToPresentationStartMs:timing.lastPresentedAt-start,
                        inputToGpuCompletionMs:timing.lastCompletedAt-start,settledMs:performance.now()-start});
                }
                fences.interaction=null;
                const frameGroups=new Map();
                for(const record of fences.records){const key=`${record.interaction}:${record.frame}`;
                    if(!frameGroups.has(key))frameGroups.set(key,[]);frameGroups.get(key).push(record);}
                const frameFences=[...frameGroups.values()].map(records=>{
                    const complete=records.every(record=>record.completedAt!==null&&record.error===null);
                    const submittedAt=Math.min(...records.map(record=>record.submittedAt));
                    const completedAt=complete?Math.max(...records.map(record=>record.completedAt)):null;
                    return {interaction:records[0].interaction,frame:records[0].frame,submissions:records.length,
                        submittedAt,completedAt,submissionToFenceCallbackMs:complete?completedAt-submittedAt:null};
                });
                const durations=frameFences.map(frame=>frame.submissionToFenceCallbackMs).filter(ms=>ms!==null).sort((a,b)=>a-b);
                const n=durations.length;
                const warmInteractionFrameFences={submissionLimit:fences.limit,submissions:fences.records,
                    droppedSubmissions:fences.dropped,frames:frameFences,completeFrames:n,
                    incompleteFrames:frameFences.length-n,
                    summaryMs:n&&fences.dropped===0?{min:durations[0],max:durations[n-1],
                        median:(durations[Math.floor((n-1)/2)]+durations[Math.floor(n/2)])/2,
                        mean:durations.reduce((sum,ms)=>sum+ms,0)/n}:null,
                    scope:'Six settled warm wheel interactions only. Frames group submissions by the current getCurrentTexture counter. Duration is first submit invocation to latest observed onSubmittedWorkDone callback for that group; includes CPU submission, prior queue work and callback scheduling, not isolated GPU execution or compositor presentation. Summary excludes incomplete frames and is omitted if capture overflows.'};
                const before={...counts},start=performance.now();
                for(let i=0;i<48;i++)window.dispatchEvent(new Event('resize'));
                for(const delta of [50,-20,40,-40,20,-50])wheel(delta);
                await settled();
                const burst={durationMs:performance.now()-start,...Object.fromEntries(Object.keys(counts).map(key=>[key,counts[key]-before[key]]))};
                const finalGeometry=geometry(),finalMatrix=matrix(),idleBefore={...counts},idleStart=performance.now();
                await new Promise(resolve=>setTimeout(resolve,200));await frame();await frame();await frame();
                const idle={durationMs:performance.now()-idleStart,...Object.fromEntries(Object.keys(counts).map(key=>[key,counts[key]-idleBefore[key]]))};
                if(JSON.stringify(initialGeometry)!==JSON.stringify(finalGeometry))throw Error('View-only input changed geometry counters');
                if(!finalMatrix.every((value,i)=>Math.abs(value-initialMatrix[i])<=1e-9*Math.max(1,Math.abs(initialMatrix[i]))))throw Error('Balanced input changed final pose');
                if(idle.frames||idle.submits)throw Error('Idle runtime submitted frames');
                return {initial,initialGeometry,initialMatrix,finalGeometry,finalMatrix,interaction,warmInteractionFrameFences,burst,idle,
                    timingScope:'Presentation start is getCurrentTexture; completion is the GPU queue fence, not a compositor timestamp'};
            });
            const entry={label,url,adapter,coldLoadReadyWallMs,...measured,errors};
            // Cold-load timing comes from the browser's first-frame phase, separate from later probes.
            entry.coldLoadFirstFrameMs=measured.initial.phases.find(phase=>phase.name==='first frame complete')?.elapsedMs;
            await page.screenshot({path:path.join(output,`${label}.png`)});
            assert.deepEqual(errors,[]);
            report.samples.push(entry);save();
            console.log(`${label}: first frame ${entry.coldLoadFirstFrameMs}ms; burst ${entry.burst.frames} frames/${entry.burst.submits} submits; idle ${entry.idle.submits}`);
            if(label==='after') {
                const check=path.resolve(__dirname,'../../../session_viewer/docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-performance-browser-check.cjs');
                await require(check)(page);report.coalescingSemanticCheck='passed';save();
            }
            await context.close();
            await browser.close();browser=undefined;
        }
        assert.deepEqual(report.samples[0].initialGeometry,report.samples[1].initialGeometry,'Same source and upload counters');
        assert.deepEqual(report.samples[0].initialMatrix,report.samples[1].initialMatrix,'Same starting camera');
        assert(report.samples[1].burst.frames<report.samples[0].burst.frames,'Burst presentation count decreases');
        report.status='passed';
    } catch(error) {report.status='failed';report.error=error.stack||String(error);throw error;}
    finally {if(browser)await browser.close();save();}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
