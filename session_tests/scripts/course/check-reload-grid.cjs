const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const {createHash} = require('node:crypto');
const {execFileSync} = require('node:child_process');
const {setTimeout: pause} = require('node:timers/promises');
const {chromium} = require(process.env.PLAYWRIGHT_MODULE || 'playwright');

const timeout = Number(process.env.B3_TIMEOUT_MS || 90000);
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
const xy = {origin:[0,0,0], x:[1,0,0], y:[0,1,0]};
const xz = {origin:[0,0,0], x:[1,0,0], y:[0,0,1]};

// Observe actual submissions and trusted input without changing the render schedule.
function observe() {
    const state = window.__reloadGridProof = {queues:new Set(), submits:0, adapters:[], events:[], revision:0};
    const submit = GPUQueue.prototype.submit;
    GPUQueue.prototype.submit = function(commands) {
        state.queues.add(this); state.submits++;
        return submit.call(this, commands);
    };
    const request = GPU.prototype.requestAdapter;
    GPU.prototype.requestAdapter = async function(options) {
        const adapter = await request.call(this, options);
        if (adapter) {
            const info = adapter.info;
            state.adapters.push({vendor:info.vendor, architecture:info.architecture,
                description:info.description, isFallbackAdapter:info.isFallbackAdapter});
        }
        return adapter;
    };
    for (const type of ['pointerdown','pointermove','pointerup','wheel','keydown']) {
        window.addEventListener(type, event => {
            if (state.events.length < 2000) state.events.push({type, trusted:event.isTrusted,
                button:event.button, buttons:event.buttons, key:event.key, deltaY:event.deltaY});
        }, {capture:true});
    }
    new MutationObserver(records => {
        if (records.some(record => record.type === 'attributes' && record.target.id === 'canvas')) state.revision++;
    }).observe(document, {subtree:true, attributes:true});
}

async function bounded(promise, label) {
    let timer;
    try {return await Promise.race([promise,new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error(label+' timed out')),timeout);})]);}
    finally {clearTimeout(timer);}
}

async function settle(page) {
    await page.evaluate(async () => {
        let timer;
        try {
            await Promise.race([(async () => {
                await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
                await Promise.all([...window.__reloadGridProof.queues].map(queue => queue.onSubmittedWorkDone()));
            })(), new Promise((_, reject) => {timer=setTimeout(() => reject(Error('GPU completion timed out')),30000);})]);
        } finally {clearTimeout(timer);}
    });
}

async function snapshot(page, kind) {
    return page.evaluate(kind => {
        const canvas=document.querySelector('#canvas');
        const value=name=>JSON.parse(canvas.getAttribute(name));
        const observed=window.__reloadGridProof;
        const plane=grid=>Object.fromEntries(Object.entries(grid).map(([key,axis])=>[key,axis.map(v=>v===0?0:v)]));
        if (kind==='maintained') {
            const state=value('data-viewer-inspection');
            return {pose:state.camera_pose, grid:plane(state.shown_grid), count:state.objects,
                sourceRevision:state.scene_revision, submittedAt:state.submitted_at_ms,
                anchor:state.origin, tool:state.tool, gridGpu:null,
                submits:observed.submits, revision:observed.revision};
        }
        const grid=value('data-grid-plane'), matrix=value('data-camera-matrix');
        return {
            // Reload may legitimately change depth reach. X/Y/W rows retain the actual view and projection.
            pose:{target:value('data-camera-target'), frame:value('data-camera-frame'),
                projection:canvas.getAttribute('data-projection'), xyw:matrix.filter((_,i)=>i%4!==2)},
            grid:plane({origin:grid.origin,x:grid.matrix.slice(0,3),y:grid.matrix.slice(4,7)}),
            count:Number(canvas.getAttribute('data-object-count')), sourceRevision:value('data-source-origins'),
            submittedAt:null, anchor:value('data-render-origin'), tool:value('data-tool'), gridGpu:value('data-grid-gpu'),
            submits:observed.submits, revision:observed.revision,
        };
    }, kind);
}

async function until(read, predicate, label) {
    const end=Date.now()+timeout;
    while (Date.now()<end) {
        const value=await read();
        if (predicate(value)) return value;
        await pause(50);
    }
    throw Error(label+' timed out');
}

async function command(page, kind, text) {
    const attribute=kind==='maintained'?'data-viewer-ui':'data-command-ui';
    const ui=await page.locator('#canvas').getAttribute(attribute).then(JSON.parse);
    const control=ui.controls.find(control=>control.key==='command/input');
    assert(control,'Actual command input control must be exposed');
    const box=await page.locator('#canvas').boundingBox(), [x,y,right,bottom]=control.rect;
    await page.mouse.click(box.x+(x+right)/2,box.y+(y+bottom)/2);
    if(ui.command) await page.keyboard.press('ControlOrMeta+A');
    await page.keyboard.type(text,{delay:12});
    await page.waitForFunction(({attribute,text})=>JSON.parse(document.querySelector('#canvas').getAttribute(attribute)).command===text,{attribute,text});
    const history=await page.locator('#canvas').getAttribute(attribute).then(raw=>JSON.parse(raw).history);
    await page.keyboard.press('Enter');
    await page.waitForFunction(({attribute,history})=>JSON.stringify(JSON.parse(document.querySelector('#canvas').getAttribute(attribute)).history)!==JSON.stringify(history),{attribute,history});
    await settle(page);
    const after=await page.locator('#canvas').getAttribute(attribute).then(JSON.parse);
    return after.history.at(-1);
}

async function scenePoint(page, kind) {
    const box=await page.locator('#canvas').boundingBox();
    if (kind==='maintained') {
        const ui=JSON.parse(await page.locator('#canvas').getAttribute('data-viewer-ui'));
        const r=ui.scene_rect;
        assert(r?.length===4,'Maintained actual scene rectangle');
        return [box.x+(r[0]+r[2])/2,box.y+(r[1]+r[3])/2];
    }
    const frame=JSON.parse(await page.locator('#canvas').getAttribute('data-camera-frame'));
    return [box.x+(frame[0]+frame[2])*box.width/2,box.y+(frame[1]+frame[3])*box.height/2];
}

async function navigate(page, kind) {
    await page.keyboard.press('Escape');
    const [x,y]=await scenePoint(page,kind), motions=[];
    for (const [name,button,dx,dy] of [['orbit','right',37,23],['pan','middle',29,-17]]) {
        const before=await snapshot(page,kind);
        await page.mouse.move(x,y);await page.mouse.down({button});
        await page.mouse.move(x+dx,y+dy,{steps:8});await page.mouse.up({button});
        await until(()=>snapshot(page,kind),state=>JSON.stringify(state.pose)!==JSON.stringify(before.pose),name);
        await settle(page);
        const after=await snapshot(page,kind);
        assert.deepEqual(after.grid,before.grid,name+' must retain the displayed world plane');
        motions.push({name,before:before.pose,after:after.pose});
    }
    const before=await snapshot(page,kind);
    await page.mouse.move(x,y);await page.mouse.wheel(0,120);
    await until(()=>snapshot(page,kind),state=>JSON.stringify(state.pose)!==JSON.stringify(before.pose),'wheel zoom');
    await settle(page);
    const after=await snapshot(page,kind);
    assert.deepEqual(after.grid,before.grid);
    motions.push({name:'wheel',before:before.pose,after:after.pose});
    return motions;
}

function fixtures(page, address, bytes) {
    const origin=new URL(address).origin, prefix='/__reload_grid__';
    let version=1, release, arrive;
    const arrived=new Promise(resolve=>{arrive=resolve;});
    const held=new Promise(resolve=>{release=resolve;});
    const requests=[];
    const manifest=next=>JSON.stringify({name:next?'B3 replacement':'B3 initial',view:'top',grid:true,
        items:[{file:origin+prefix+'/base.pb'},...(next?[{file:origin+prefix+'/next.pb',at:[500,0,0]}]:[])]});
    const handler=async route=>{
        const request=route.request(), url=new URL(request.url()), next=url.pathname.endsWith('/replacement.yaml')||version===2;
        requests.push({url:url.href,method:request.method(),range:request.headers().range||null,version});
        const headers={'access-control-allow-origin':'*','cache-control':'no-cache','content-type':url.pathname.endsWith('.pb')?'application/octet-stream':'application/json'};
        if (url.pathname.endsWith('.yaml')) {
            const etag='"manifest-'+(next?2:1)+'"';
            return route.fulfill({status:request.headers()['if-none-match']===etag?304:200,headers:{...headers,etag},body:request.headers()['if-none-match']===etag?'':manifest(next)});
        }
        if (url.pathname.endsWith('/next.pb')) {arrive();await held;}
        const etag='"original-'+hash(bytes)+'"';
        return route.fulfill({status:request.headers()['if-none-match']===etag?304:200,headers:{...headers,etag},body:request.headers()['if-none-match']===etag?'':bytes});
    };
    return {origin,prefix,requests,handler,arrived,release,change(){version=2;}};
}

async function gridCommands(page,kind) {
    const records=[];
    const check=async(label,expected)=>{
        const state=await snapshot(page,kind);assert.deepEqual(state.grid,expected,label);records.push({label,...state});return state;
    };
    if(kind==='course')await command(page,kind,'View Grid On');
    await command(page,kind,'Construction Plane XY');await check('explicit XY',xy);
    await command(page,kind,'View Front');await check('Front retains XY',xy);
    await command(page,kind,'Construction Plane View');const held=await check('explicit View resolves XZ',xz);
    await command(page,kind,'Construction Plane View');const repeated=await check('repeat View keeps XZ',xz);
    if(kind==='course')assert.deepEqual(repeated.gridGpu,held.gridGpu,'Repeated plane command must reuse course GPU rows');
    await command(page,kind,'View Top');await check('Top retains XZ',xz);
    await command(page,kind,'View Reset');await check('Reset retains XZ',xz);
    await command(page,kind,'Construction Plane 3 Point');
    const active=await snapshot(page,kind);assert(active.tool!==null,'Three-point tool actually began');
    await page.keyboard.press('Escape');await page.keyboard.press('Escape');await settle(page);
    const cancelled=await check('cancelled three-point tool retains XZ',xz);
    assert.equal(cancelled.tool,null,'Escape actually cancels the three-point tool');
    await command(page,kind,'Construction Plane 3 Point');
    const answers=[];
    for(const point of ['w0,0,0','w1,0,0','w2,0,0'])answers.push(await command(page,kind,point));
    await check('collinear three-point input retains XZ',xz);
    assert(/line|collinear/i.test(answers.at(-1)),'The actual UI must reject collinear points');
    await page.keyboard.press('Escape');await page.keyboard.press('Escape');await settle(page);
    records.push({collinearAnswers:answers});
    return records;
}

async function farGrid(page,kind) {
    const origin=[1e12-4+0.125,-1e12+4+0.25,1e12-4+0.5];
    const expected={origin,x:[1,0,0],y:[0,0,1]};
    const points=[origin,origin.map((v,i)=>v+(i===0?1:0)),origin.map((v,i)=>v+(i===2?1:0))];
    await command(page,kind,'Construction Plane 3 Point');
    for(const point of points)await command(page,kind,'w'+point.join(','));
    const records=[];
    for(const line of ['View Front','View Top','View Reset']) {
        await command(page,kind,line);
        const state=await snapshot(page,kind);
        assert.deepEqual(state.grid,expected,'Far world frame survives '+line);
        records.push({command:line,grid:state.grid,anchor:state.anchor});
    }
    return records;
}

async function check(browser,kind,address,mode,bytes,output) {
    const context=await browser.newContext({viewport:{width:1100,height:800},deviceScaleFactor:1});
    const page=await context.newPage(), report={kind,mode,sourceSha256:hash(bytes),errors:[],status:'running'};
    page.setDefaultTimeout(timeout);
    page.on('pageerror',error=>report.errors.push(error.message));
    const fixture=fixtures(page,address,bytes);
    const stem=kind+'-'+mode, save=()=>fs.writeFileSync(path.join(output,stem+'.json'),JSON.stringify(report,null,2)+'\n');
    try {
        await page.addInitScript(observe);
        await page.route('**/__reload_grid__/**',fixture.handler);
        const url=new URL(address);
        for(const key of ['scene','live','notify','poll','spin','data'])url.searchParams.delete(key);
        url.searchParams.set('inspect','1');url.searchParams.set('data',fixture.origin+'/');url.searchParams.set('notify','off');
        if(mode==='live') {url.searchParams.set('live',fixture.origin+fixture.prefix+'/initial.yaml');url.searchParams.set('poll','1');}
        else {url.searchParams.set('scene','__reload_grid__/initial.yaml');url.searchParams.set('live','off');}
        report.url=url.href;
        await page.goto(url.href,{waitUntil:'domcontentloaded',timeout});await page.bringToFront();
        await page.waitForFunction(kind=>{
            const canvas=document.querySelector('#canvas');
            if(!window.wasmBindings?.reload_scene||!canvas)return false;
            if(kind==='maintained') {const raw=canvas.getAttribute('data-viewer-inspection');if(!raw)return false;const state=JSON.parse(raw);return state.camera_pose&&state.shown_grid&&state.objects>0;}
            return window.wasmBindings.runtime_running?.()&&Number(canvas.getAttribute('data-object-count'))>0&&canvas.hasAttribute('data-grid-plane');
        },kind);
        await settle(page);
        report.adapters=await page.evaluate(()=>window.__reloadGridProof.adapters);
        assert(report.adapters.length>0,'Actual viewer adapter was observed');
        for(const adapter of report.adapters)assert(adapter.isFallbackAdapter&&adapter.vendor==='microsoft'&&adapter.architecture==='warp','Actual viewer must use WARP: '+JSON.stringify(adapter));
        report.initial=await snapshot(page,kind);
        assert.deepEqual(report.initial.grid,xy,'Initial displayed grid is XY');
        report.grid=await gridCommands(page,kind);
        report.beforeRequest=await snapshot(page,kind);
        if(mode==='live') fixture.change();
        else await page.evaluate(route=>window.wasmBindings.reload_scene(route),'__reload_grid__/replacement.yaml');
        await bounded(fixture.arrived,'Deferred original PB request');
        report.pending=await snapshot(page,kind);
        assert.equal(report.pending.count,report.beforeRequest.count,'Old scene stays visible while replacement waits');
        report.motion=await navigate(page,kind);
        report.latest=await snapshot(page,kind);
        assert.equal(report.latest.count,report.beforeRequest.count,'All navigation happened before replacement delivery');
        assert.notDeepEqual(report.latest.pose,report.beforeRequest.pose);
        fixture.release();
        await until(()=>snapshot(page,kind),state=>state.count===report.initial.count*2,'replacement publication');
        await settle(page);
        report.delivered=await snapshot(page,kind);
        assert.deepEqual(report.delivered.pose,report.latest.pose,'Delivery must preserve the latest camera, including motion during loading');
        assert.deepEqual(report.delivered.grid,report.latest.grid,'Scene replacement must preserve the held world grid');
        assert(report.delivered.submits>report.latest.submits,'A fresh actual GPU submission presented replacement');
        if(kind==='maintained')assert(report.delivered.submittedAt>report.latest.submittedAt,'Fresh production inspection accompanies delivery');
        if(kind==='course')assert(report.delivered.revision>report.latest.revision,'Fresh course presentation attributes accompany delivery');
        await page.screenshot({path:path.join(output,stem+'.png')});
        report.farGrid=await farGrid(page,kind);
        report.events=await page.evaluate(()=>window.__reloadGridProof.events);
        assert(report.events.filter(event=>event.type==='wheel'||event.type==='pointerdown').every(event=>event.trusted),'Motion must use trusted browser input');
        assert.deepEqual(report.errors,[]);
        report.status='pass';
        report.gaps=['No exact grid pixel parity is claimed by this browser check; the production native far-grid witness covers fractional pixels.',
            'Course camera equality uses target/frame/projection and matrix X/Y/W rows; depth reach may change with replacement geometry.',
            'The maintained grid has no public upload counter, so repeated-command GPU write suppression is covered by source/native checks.',
            'Far-world inspection is checked through actual 3-point input and camera presets; browser camera travel to the far anchor is not exercised.',
            'No cancelled fetch, overlapping reload, or initial-fit behavior acceptance is implied.'];
        console.log('PASS '+stem+': latest orbit/pan/zoom preserved across deferred PB delivery; explicit-only held grid');
    } catch(error) {report.status='fail';report.failure=error.stack||String(error);throw error;}
    finally {fixture.release();report.requests=fixture.requests;save();await context.close();}
}

async function main() {
    const [maintained,course,sourceFile,outputArg,runtime='both',mode='both']=process.argv.slice(2);
    assert(maintained&&course&&sourceFile&&outputArg,'Usage: check-reload-grid.cjs MAINTAINED_URL COURSE_URL ORIGINAL_MESH_PB OUTPUT [maintained|course|both] [route|live|both]');
    assert(['maintained','course','both'].includes(runtime)&&['route','live','both'].includes(mode));
    for(const address of [maintained,course])assert(['127.0.0.1','localhost','[::1]'].includes(new URL(address).hostname),'Local built viewers only');
    const bytes=fs.readFileSync(sourceFile);assert(bytes.length>0&&bytes.length<=4*1024*1024,'Use a small original PB containing mesh/CAD/surface objects');
    const output=path.resolve(outputArg);fs.mkdirSync(output,{recursive:true});
    assert.equal(process.platform,'win32','This supplemental check requires the verified Windows WARP setup');
    const warp=JSON.parse(execFileSync('powershell.exe',['-NoProfile','-ExecutionPolicy','Bypass','-File',path.resolve(__dirname,'../warp-adapter.ps1')],{encoding:'utf8'}));
    const args=['--enable-unsafe-webgpu','--ignore-gpu-blocklist','--use-gl=angle','--use-angle=d3d11',`--use-adapter-luid=${warp.luid}`];
    if(process.env.VIEWER_DAWN_SKIP_OPTIMIZATIONS==='1')args.push('--enable-dawn-features=d3d_skip_shader_optimizations');
    const executablePath=process.env.VIEWER_BROWSER_EXECUTABLE;
    const browser=await chromium.launch({...(executablePath?{executablePath}:{channel:'chrome'}),headless:true,args,timeout:15000});
    fs.writeFileSync(path.join(output,'environment.json'),JSON.stringify({browser:browser.version(),warp,args,executablePath:executablePath||null,source:path.resolve(sourceFile),sourceSha256:hash(bytes)},null,2)+'\n');
    try {
        for(const kind of runtime==='both'?['maintained','course']:[runtime]) {
            for(const which of mode==='both'?['route','live']:[mode])await check(browser,kind,kind==='maintained'?maintained:course,which,bytes,output);
        }
    } finally {await browser.close();}
}
if(require.main===module)main().catch(error=>{console.error(error);process.exitCode=1;});
