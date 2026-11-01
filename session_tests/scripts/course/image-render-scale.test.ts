import test from 'node:test';
import assert from 'node:assert/strict';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {createRequire} from 'node:module';
import vm from 'node:vm';
import {imageProfile, nativeScaleBridge, browserScaleBridge, validateBrowserScale, applyBrowserScale, capSnapshotOptions, comparisonSnapshotOptions, surfaceSnapshotOptions, ordinaryImageOwners} from './image-render-scale.ts';
import {course, expected, withExpectedSequence, docs, read} from './model.ts';

const here = path.dirname(fileURLToPath(import.meta.url));
const scope = JSON.parse(read(path.join(here, 'image-render-scale-scope.json')));
const steps = course().steps;

test('settle failure diagnostics preserve the original error even when reading, attaching or logging fails', async () => {
    const before = {css:[900,760],internal:[1800,1520],dpr:2,requestedScale:1,
        sources:[[4,5,6,7].map(id=>({id})),[],4],clipping:{usage:[0,1,1],rows:[],caps:0,error:null,cap_error:null}};
    const originalLog = console.error;
    try {
        for (const mode of ['raw', 'read-failure', 'frozen-error', 'log-failure']) {
            const original = new Error('original 30s quality timeout');
            if(mode==='frozen-error') Object.freeze(original);
            let calls=0, logs=0;
            console.error=()=>{logs++;if(mode==='log-failure')throw new Error('stderr unavailable');};
            const page={async evaluate(fn:any){
                calls++;if(calls===1)return structuredClone(before);if(calls===2)return;
                assert.equal(calls,3);
                if(mode==='read-failure')throw new Error('page closed');
                const canvas={width:3600,height:3040,getBoundingClientRect:()=>({width:900,height:760}),
                    getAttribute:(name:string)=>name==='data-math-display'?'{"pending":true,"bytes":67108864}':null};
                return structuredClone(vm.runInNewContext('('+fn.toString()+')()',
                    {document:{querySelector:()=>canvas},window:{__courseImageRenderScale:2},devicePixelRatio:2}));
            }};
            await assert.rejects(applyBrowserScale(page,{renderScale:2,scene:'comparison'},async()=>{throw original;}), error=>error===original);
            assert.equal(calls,3);assert.equal(logs,1);
            if(mode!=='frozen-error') {
                const diagnostic=(original as any).imageRenderScaleFailure;
                assert.equal(diagnostic.options.expectedQualities,4);assert.deepEqual(diagnostic.before,before);
                if(mode==='read-failure')assert.match(diagnostic.finalState.readError,/page closed/);
                else assert.equal(diagnostic.finalState.attributes['data-math-display'],'{"pending":true,"bytes":67108864}');
            }
        }
    } finally {console.error=originalLog;}
});

test('successful settling does not read or emit failure diagnostics and retains the original 30s helper', async () => {
    const before={css:[900,760],internal:[1800,1520],dpr:2,requestedScale:1,camera:[],
        sources:[[4,5,6,7].map(id=>({id})),[],4],selected:[],clipping:{usage:[0,1,1],rows:[],caps:0,error:null,cap_error:null}};
    const after={...structuredClone(before),internal:[3600,3040],requestedScale:2,rendererProfile:{renderScale:2,density:4,internal:[3600,3040]}};
    let calls=0, logs=0;const originalLog=console.error;
    try {
        console.error=()=>{logs++;};
        const page={async evaluate(){calls++;return calls===1?structuredClone(before):calls===2?undefined:structuredClone(after);}};
        await applyBrowserScale(page,{renderScale:2,scene:'comparison'},async()=>({adaptive:true,quality:{qualities:[4,5,6,7].map(id=>[id,.25,.2,false])}}));
        assert.equal(calls,3);assert.equal(logs,0);
    } finally {console.error=originalLog;}
    const helper=read(path.join(docs,'journey/checks/browser/adaptive-gpu-reuse.cjs'));
    assert(helper.includes('const timeout=options.timeoutMs??30000;'));
    const worker=read(path.join(here,'browser-scene-presentation.ts'));
    assert(worker.includes('imageRenderScaleFailure: (error as any)?.imageRenderScaleFailure || null'));
});

test('exact 291-profile scope binds fixtures/assets and excludes other lanes and defaults', () => {
    assert.equal(scope.length, 291); assert.equal(scope.filter((r:any)=>r.lane==='native').length, 278);
    assert.equal(scope.filter((r:any)=>r.lane==='browser').length, 13);
    assert.equal(scope.filter((r:any)=>!r.scene).length, 48);
    for (const row of scope) {
        assert.equal(imageProfile(row.id, row.lane, '2', docs)?.renderScale, 2);
        assert.equal(imageProfile(row.id, row.lane, '1', docs), null);
    }
    assert.equal(imageProfile('71aj-retain-nearby-curve-search-results', 'browser', '2', docs), null);
    assert.equal(imageProfile(scope.find((r:any)=>r.lane==='native' && !scope.some((b:any)=>b.id===r.id&&b.lane==='browser')).id, 'browser', '2', docs), null);
    assert.throws(()=>imageProfile(scope[0].id,scope[0].lane,'3',docs));
});

const actual = withExpectedSequence(() => {
    const rows:any[]=[];
    for (const step of steps) {
        const interested=scope.filter((r:any)=>r.id===step.id); if(!interested.length) continue;
        const files=expected(step.id);
        for(const row of interested) rows.push({row,step,files});
    }
    return rows;
});

test('all thirteen actual browser sources transform reversibly without changing single-sample shaders', () => {
    for(const {row,files} of actual.filter(x=>x.row.lane==='browser')) {
        const original=files['src/browser.rs'], bridge=browserScaleBridge(original,imageProfile(row.id,'browser','2',docs));
        let reversed=bridge.code; for(const c of [...bridge.changes].reverse())reversed=reversed.replace(c.after,c.before);
        assert.equal(reversed,original);assert.equal(bridge.beforeSha256,bridge.reversedSha256);
        assert(bridge.code.indexOf('density * course_image_scale(window)')<bridge.code.indexOf('renderer.set_density',bridge.code.indexOf('fn resize(')));
        assert(!bridge.code.includes('set_device_pixel_ratio'));assert(!bridge.code.includes('sample_count: 4'));
        assert.throws(()=>browserScaleBridge(original.replace('fn resize(', 'fn changed_resize('),bridge.profile));
        if(row.scene==='surface-network') {
            assert.equal(bridge.initialization,'temporary-css-config-then-scaled-resize');
            const boot='    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);';
            assert(bridge.code.includes(boot));
            assert.throws(()=>browserScaleBridge(original.replace(boot,'        math_view.refresh();\n'+boot),bridge.profile),/math refresh/);
            assert.throws(()=>browserScaleBridge(original.replace(boot,'        present();\n'+boot),bridge.profile),/presentation/);
            assert.throws(()=>browserScaleBridge(original.replace(boot,boot.replace(' as u32',' as u64')),bridge.profile));
            assert.throws(()=>browserScaleBridge(original,{renderScale:2,lane:'browser',scene:'comparison'}),/initialization/);
        }
    }
});

test('278 real native fixtures retain draw/pixel tails, refine at internal size, read back only output size', () => {
    for (const {row,step,files} of actual.filter(x=>x.row.lane==='native')) for(const density of [2,3]) {
        const [w,h]=(step.frame_size||[640,480]).map((n:number)=>n*density);
        const renderer=files['src/renderer.rs'];const signature=renderer.match(/pub fn resize\([^)]*\)/)![0];
        const resize=signature.includes('width: u32, height: u32')?'renderer.resize(WIDTH, HEIGHT);':'renderer.resize(viewer_journey::viewport::Viewport { width: WIDTH, height: HEIGHT });';
        let code=read(path.join(docs,'journey/frame.rs')).replace('// COURSE_SIZE',`const WIDTH: u32 = ${w};\nconst HEIGHT: u32 = ${h};`)
            .replace('// COURSE_NEW',`let mut renderer = ${step.frame_constructor};\n    ${resize}\n    renderer.set_density(${density}.0);`);
        code=code.slice(0,code.indexOf('fn verify_pixels('))+read(path.join(docs,step.frame!));
        code=code.replace('// COURSE_DRAW','draw(&renderer, &view);').replace('    verify_pixels(&pixels);','    assert_eq!(pixels.len(), WIDTH as usize * HEIGHT as usize * 4);');
        const bridge=nativeScaleBridge(code,imageProfile(row.id,'native','2',docs),[w,h],density,read(path.join(docs,step.frame!)));
        assert.equal(bridge.code.slice(bridge.code.indexOf('fn draw(')),code.slice(code.indexOf('fn draw(')));
        const ctor=bridge.code.slice(bridge.code.indexOf('fn make_renderer('));
        assert(ctor.indexOf('renderer.set_density(IMAGE_DENSITY)')<ctor.indexOf('math_quality.drain_matrix'));
        assert(bridge.code.includes('[WIDTH,HEIGHT],None'));assert(bridge.code.includes(`const WIDTH: u32 = ${w*2};`));
        const readback=bridge.code.slice(bridge.code.indexOf('    let row_bytes ='),bridge.code.indexOf('const COURSE_PRESENT_SHADER'));
        assert(!/\bWIDTH\b|\bHEIGHT\b/.test(readback));assert(readback.includes('OUTPUT_WIDTH as usize * OUTPUT_HEIGHT as usize * 4'));
        assert(bridge.code.includes('resolve_target: None'));assert.equal(bridge.beforeSha256,bridge.reversedSha256);
    }
});

test('browser qualification rejects clamping, stale density, changed camera/source/CSS/external DPR', () => {
    const before={css:[900,760],internal:[1800,1520],dpr:2,camera:[1,0,0,1],sources:[[{id:7}],[],2],selected:[7],clipping:{usage:[1,1,1],rows:[{id:8,normal:[0,0,1],offset:0}],caps:1,error:null,cap_error:null}};
    const after={...structuredClone(before),internal:[3600,3040],requestedScale:2,rendererProfile:{renderScale:2,density:4,internal:[3600,3040]}};
    validateBrowserScale(before,after,2);
    for(const bad of [{internal:[2048,2048]}, {rendererProfile:{renderScale:2,density:2,internal:[3600,3040]}}, {camera:[0]}, {sources:[]}, {css:[901,760]}, {dpr:4}, {selected:[]}, {clipping:{...after.clipping,usage:[1,0,1]}}, {clipping:{...after.clipping,rows:[{id:8,normal:[0,0,1],offset:1}]}}])
        assert.throws(()=>validateBrowserScale(before,{...after,...bad},2));
});

test('all five main-page routes inherit the exported-worker profile activation after their unchanged recipe',()=>{
    const wrapper=read(path.join(here,'main-page-browser-image-recipes.ts'));
    assert(wrapper.includes("core=fileURLToPath(new URL('./exported-browser-image-recipes.ts',import.meta.url))"));
    assert(wrapper.includes('original=read(core)'));assert(wrapper.includes('api:await import(pathToFileURL(file).href)'));
    const worker=read(path.join(here,'exported-browser-image-recipes.ts'));
    const needle="loaded._compile(read(privateFile),privateFile);";
    assert.equal(worker.split(needle).length,2);
    const adapted=worker.replace(needle,"loaded._compile(read(privateFile)+'\\nmodule.exports.__presentation=async(page,address,density,h)=>{'+p.recipe.mainSetup.body+'};\\n',privateFile);");
    assert(adapted.includes("import {applyBrowserScale} from './image-render-scale.ts';"));
    assert(adapted.indexOf('await applyBrowserScale')>adapted.indexOf('else await fn(page,address,2,helpers)'));
    assert(adapted.indexOf('await applyBrowserScale')<adapted.indexOf('bytes=await page.screenshot'));
    for(const row of scope.filter((r:any)=>r.lane==='browser'&&!r.scene&&!r.id.startsWith('88ad')))assert(wrapper.includes("'"+row.id+"'"));
});

test('inactive browser profile never touches a page or invokes refinement', async()=>{
    assert.equal(await applyBrowserScale(new Proxy({}, {get(){throw Error('Unexpected page access');}}),null,async()=>{throw Error('Unexpected settle');}),null);
});

test('compact browser profile rejects the wrong recipe before opening locks or compiling',()=>{
    const source=read(path.join(here,'compact-capture.ts'));
    const start=source.indexOf('    const captureImageProfile ='), end=source.indexOf('    fs.mkdirSync(root,',start);
    assert(start>0&&end>start);
    const preflight=new Function('imageProfile','id','lane','docs','route','assert',source.slice(start,end)+'\nreturn captureImageProfile;');
    const configured=(id:string,lane:string,_request:any,root:string)=>imageProfile(id,lane,'2',root);
    for(const row of scope.filter((r:any)=>r.lane==='browser')) {
        const route=row.route||(row.id.startsWith('88ad-')?'exported':'main-page');
        assert.equal(preflight(configured,row.id,'browser',docs,route,assert).renderScale,2);
        for(const wrong of ['generic','exported','main-page','fallback'].filter(r=>r!==route))
            assert.throws(()=>preflight(configured,row.id,'browser',docs,wrong,assert),/exact image recipe/);
    }
    assert(source.includes('captureImageProfile?.files.map(file => file.path)'));
    assert(source.indexOf('record.renderScale?.profile')<source.indexOf('const receipt = sealCapture('));
});

test('image activation waits for accepted new-resolution math and refuses a nonadaptive result', async()=>{
    const before={css:[900,760],internal:[1800,1520],dpr:2,requestedScale:1,camera:[1,0,0,1],sources:[[{id:7}],[],2],selected:[7],
        clipping:{usage:[1,1,1],rows:[{id:8,normal:[0,0,1],offset:0}],caps:1,error:null,cap_error:null}};
    const after={...structuredClone(before),internal:[3600,3040],requestedScale:2,rendererProfile:{renderScale:2,density:4,internal:[3600,3040]}};
    let stage=0,settled=false;
    const page={async evaluate(_fn:any){stage++; if(stage===1)return structuredClone(before);if(stage===2)return;assert(settled);return structuredClone(after);}};
    const result=await applyBrowserScale(page,{renderScale:2},async options=>{assert.equal(stage,2);assert.equal(options.expectedRows,2);assert.equal(options.expectedQualities,1);await Promise.resolve();settled=true;return{adaptive:true,quality:{qualities:[[7,.25,.2,false]]}};});
    assert.equal(stage,3);assert.deepEqual(result.after,after);assert.equal(result.canonicalBehaviorRenderScale,1);
    stage=0;settled=false;
    await assert.rejects(applyBrowserScale(page,{renderScale:2},async()=>({adaptive:false})),/accepted mathematical quality/);
    assert.equal(stage,2);
    stage=0;
    await assert.rejects(applyBrowserScale(page,{renderScale:2},async()=>({adaptive:true,quality:{qualities:[[8,.25,.2,false]]}})),/belongs to the original CAD/);
});

test('all six cap recipes expose one CAD and one ordinary mesh plane, but only the CAD requests mathematical quality',()=>{
    for(const {files} of actual.filter(x=>x.row.lane==='browser'&&!x.row.scene)) {
        assert(files['src/plane_original.rs'].includes('let id=scene.insert(prepared(&plane)?)?;'));
        assert(files['src/plane_original.rs'].includes('original=Some(Geometry::Plane(plane))'));
        assert(files['src/renderer.rs'].includes('for object in scene.objects().iter().filter(|row| scene.visible(row.id))'));
        assert(files['src/renderer.rs'].includes('crate::memory::gpu(&self.meshes)'));
        assert(files['src/memory.rs'].includes('count[2] += 1;'));
        const requests=files['src/math_view.rs'].split('fn requests(scene:')[1].split('impl Presentation')[0];
        assert(requests.includes('if let Some(cad)=&row.cad'));assert(!requests.includes('Geometry::Plane'));
        const browser=files['src/browser.rs'];
        assert(browser.includes('"id":row.id.value(),"source":cad.source.guid()'));
        assert(browser.includes('"id":r.id.value(),"normal":r.plane.normal,"offset":r.plane.offset'));
        assert(browser.includes('"data-object-count", &editor.scene.objects().len().to_string()'));
    }
});

test('cap ownership requires exact CPU owners and exact two GPU rows with one CAD quality, retaining full static reuse',async()=>{
    const cpu={sources:[[{id:7}],[],2],clipping:{rows:[{id:8,normal:[0,0,1],offset:0}]}};
    const options=capSnapshotOptions(cpu);
    for(const bad of [{sources:[[{id:7}],[],3]}, {sources:[[{id:7},{id:9}],[],2]}, {sources:[[{id:7}],[{id:9}],2]},
        {clipping:{rows:[]}}, {clipping:{rows:[{id:7}]}}, {clipping:{rows:[{owner:8}]}}])assert.throws(()=>capSnapshotOptions({...cpu,...bad}));
    const {snapshot}=createRequire(import.meta.url)(path.join(docs,'journey/checks/browser/adaptive-gpu-reuse.cjs'));
    const run=async(change:(frame:any)=>void=()=>{},unstable:'stats'|'usage'|null=null)=>{
        const frame:any={quality:{pending:false,limited:null,bytes:1000,generations:1,qualities:[[7,.25,.2,false]]},
            stats:[10,12,30,4000],usage:[2,8192,2,256],path:[0,0,0,0],surfaces:[],cads:[{id:7}],curveCpu:[0,0,0,0]};
        change(frame);let flushes=0;
        const attributes:any={'data-math-display':'quality','data-gpu-stats':'stats','data-gpu-usage':'usage','data-path-gpu':'path',
            'data-surfaces':'surfaces','data-cads':'cads','data-curve-cpu':'curveCpu'};
        const context:any=vm.createContext({input:null,document:{querySelector:()=>({getAttribute:(name:string)=>JSON.stringify(frame[attributes[name]])})},
            Event:class{constructor(public type:string){}},requestAnimationFrame:(callback:any)=>queueMicrotask(callback),
            window:{dispatchEvent:()=>{flushes++;if(unstable&&flushes===2)frame[unstable][1]++;}}});
        const evaluate=async(fn:any,arg?:any)=>{context.input=arg;return structuredClone(await vm.runInContext('('+fn.toString()+')(input)',context));};
        const page={evaluate,waitForFunction:async(fn:any,arg:any)=>{assert(await evaluate(fn,arg),'Mathematical quality must qualify');}};
        const result=await snapshot(page,options);assert.equal(flushes,2);return result;
    };
    assert.equal((await run()).usage[2],2);
    for(const rows of [1,3])await assert.rejects(run(frame=>{frame.usage[2]=rows;}),/Live GPU mesh rows/);
    await assert.rejects(run(frame=>frame.quality.qualities.push([8,.25,.2,false])),/Mathematical quality must qualify/);
    await assert.rejects(run(()=>{},'stats'),/same-camera redraw/);
    await assert.rejects(run(()=>{},'usage'),/complete GPU usage/);
});

test('sRGB final presentation uses a GPU linear sample and no multisample resolve or CPU image rewrite',()=>{
    const shader=read(path.join(here,'image-filtered-present.wgsl')), native=read(path.join(here,'image-filtered-present.rs'));
    assert(shader.includes('textureSample(image, linear_sampler, v.uv)'));assert(!shader.includes('pow('));
    assert(native.includes('wgpu::TextureFormat::Rgba8UnormSrgb'));assert(native.includes('wgpu::FilterMode::Linear'));
    assert(native.includes('resolve_target: None'));assert(!native.includes('resolve_target: Some'));
    assert(native.includes('source.width(), source.height(), target.width(), target.height()'));
});

test('comparison contract retains four distinct CAD owners, zero planes, and exact four accepted quality owners',async()=>{
    const before={css:[900,760],internal:[1800,1520],dpr:2,requestedScale:1,camera:[1,0,0,1],
        sources:[[{id:4},{id:5},{id:6},{id:7}],[],4,{}, {polylines:[]},[1,2048]],selected:[],
        clipping:{usage:[0,1,1],rows:[],caps:0,error:null,cap_error:null,appearance:{opacity:1,outlines:true,arctic:false}}};
    const after={...structuredClone(before),internal:[3600,3040],requestedScale:2,rendererProfile:{renderScale:2,density:4,internal:[3600,3040]}};
    assert.deepEqual(comparisonSnapshotOptions(before),{lane:'mesh',source:'cad',expectedRows:4,expectedQualities:4});
    for(const bad of [{sources:[before.sources[0],[],5]}, {sources:[[{id:4},{id:4},{id:6},{id:7}],[],4]},
        {sources:[[{id:4},{id:5},{id:6},{owner:7}],[],4]}, {sources:[before.sources[0],[{id:8}],4]},
        {clipping:{...before.clipping,rows:[{id:8}]}}, {clipping:{...before.clipping,usage:[1,1,1]}}, {clipping:{...before.clipping,caps:1}}])
        assert.throws(()=>comparisonSnapshotOptions({...before,...bad}));
    const run=async(owners:number[],changed:any=after)=>{
        let stage=0;
        const page={async evaluate(){stage++;return stage===1?structuredClone(before):stage===2?undefined:structuredClone(changed);}};
        return applyBrowserScale(page,{renderScale:2,scene:'comparison'},async options=>{
            assert.equal(stage,2);assert.equal(options.expectedRows,4);assert.equal(options.expectedQualities,4);
            return {adaptive:true,quality:{qualities:owners.map(id=>[id,.25,.2,false])}};
        });
    };
    assert.equal((await run([7,4,6,5])).after.sources[2],4);
    for(const owners of [[4],[4,5,6],[4,5,6,8],[4,5,6,6],[4,5,6,7,8]])await assert.rejects(run(owners));
    for(const sourceIndex of [4,5]){const changed=structuredClone(after);changed.sources[sourceIndex]=[];await assert.rejects(run([4,5,6,7],changed));}
    assert.throws(()=>capSnapshotOptions(before));
});

test('comparison refinement requires four live rows and full same-camera GPU reuse',async()=>{
    const options={lane:'mesh',source:'cad',expectedRows:4,expectedQualities:4};
    const {snapshot}=createRequire(import.meta.url)(path.join(docs,'journey/checks/browser/adaptive-gpu-reuse.cjs'));
    const run=async(rows=4,owners=[4,5,6,7],unstable:'stats'|'usage'|null=null)=>{
        const frame:any={quality:{pending:false,limited:null,bytes:43536,generations:1,qualities:owners.map(id=>[id,.25,.2,false])},
            stats:[4,8,20,44000],usage:[4,43536,rows,640],path:[0,0,0,0],surfaces:[],cads:[4,5,6,7].map(id=>({id})),curveCpu:[0,0,0,0]};
        let flushes=0;const attributes:any={'data-math-display':'quality','data-gpu-stats':'stats','data-gpu-usage':'usage','data-path-gpu':'path','data-surfaces':'surfaces','data-cads':'cads','data-curve-cpu':'curveCpu'};
        const context:any=vm.createContext({input:null,document:{querySelector:()=>({getAttribute:(name:string)=>JSON.stringify(frame[attributes[name]])})},
            Event:class{constructor(public type:string){}},requestAnimationFrame:(callback:any)=>queueMicrotask(callback),window:{dispatchEvent:()=>{flushes++;if(unstable&&flushes===2)frame[unstable][1]++;}}});
        const evaluate=async(fn:any,arg?:any)=>{context.input=arg;return structuredClone(await vm.runInContext('('+fn.toString()+')(input)',context));};
        return snapshot({evaluate,waitForFunction:async(fn:any,arg:any)=>assert(await evaluate(fn,arg),'Mathematical quality must qualify')},options);
    };
    assert.equal((await run()).usage[2],4);
    for(const rows of [3,5])await assert.rejects(run(rows),/Live GPU mesh rows/);
    await assert.rejects(run(4,[4,5,6]),/Mathematical quality must qualify/);
    await assert.rejects(run(4,[4,5,6,7],'stats'),/same-camera redraw/);
    await assert.rejects(run(4,[4,5,6,7],'usage'),/complete GPU usage/);
});

test('generic image recipe qualifies scale only after original semantic checks and archives final state',()=>{
    const worker=read(path.join(here,'browser-scene-presentation.ts'));
    const activate=worker.indexOf('const renderScale = await applyBrowserScale');
    assert(activate>worker.indexOf('if (prepared.baseline) {',worker.indexOf('async function worker')));
    assert(activate>worker.indexOf("assert.equal(semantic.settings?.outlines, true)"));
    assert(activate<worker.indexOf('await page.screenshot({path: image'));
    assert(worker.indexOf('state = await readState();',activate)>activate);
    assert(worker.includes('assert.deepEqual(semanticState(state.attributes), semantic)'));
    assert(worker.includes('semantic, state, renderScale, url:'));
    assert(worker.includes("assert.equal(recipe.kind, 'retained-four-brep')"));
    assert(worker.includes('extraPins.push(...source.imageProfile.browser.files.map'));
});

test('fallback retains full callback and DPR checks before scale activation and forwards its receipt',()=>{
    const source=read(path.join(here,'browser-presentation.ts'));
    const start=source.indexOf('function highDensity('),end=source.indexOf('\nexport function prepareBrowser(',start);
    const declaration=source.slice(start,end).replace('density: number, early: boolean, profile: any = null','density, early, profile = null').replaceAll('import.meta.url','sourceUrl');
    const highDensity=new Function('path','docs','sourceUrl','return '+declaration)(path,docs,new URL('./browser-presentation.ts',import.meta.url).href);
    const profile=imageProfile('96h-prove-current-source-grammar-and-explicit-differences','browser','2',docs);
    assert.equal(profile.route,'fallback');assert.equal(profile.scene,'comparison');
    const code=highDensity(2,false,profile),activate=code.indexOf('densityRecord.renderScale = await applyBrowserScale');
    new vm.Script('(async()=>{'+code+'})');
    assert(activate>code.indexOf('assert.deepEqual(densityState.gpuErrors, []'));
    assert(activate>code.indexOf("assert.equal(await page.locator('#status').textContent()"));
    assert(activate>code.indexOf('const densityRecord ='));
    assert(code.indexOf('assert.deepEqual(errors, []);',activate)>activate);
    assert(code.indexOf('window.__presentationGpuErrors',activate)>activate);
    assert(activate<code.indexOf("fs.writeFileSync(path.join(output, id + '-presentation.json')"));
    assert(!highDensity(2,false,null).includes('applyBrowserScale'));
    assert(source.includes('.replace(imageBoundary, insert + presentationImage)'));
    assert(read(path.join(here,'browser-image-fallback.ts')).includes('renderScale:presentation.renderScale'));
});

test('exact editor fixture bridge supports 55g 59f and80 network at DPR2/3 without enabling their scope',()=>{
    const ids=['55g-surface','59f-cad','80d-exercise-networks-corner-retries-and-exact-original-archives'];
    withExpectedSequence(()=>{for(const id of ids){
        const step=steps.find(s=>s.id===id)!,files=expected(id),fixture=read(path.join(docs,step.frame!));
        assert.equal(imageProfile(id,'native','2',docs),null);
        for(const density of [2,3]){
            const [w,h]=(step.frame_size||[640,480]).map((n:number)=>n*density);
            const signature=files['src/renderer.rs'].match(/pub fn resize\([^)]*\)/)![0];
            const resize=signature.includes('width: u32, height: u32')?'renderer.resize(WIDTH, HEIGHT);':'renderer.resize(viewer_journey::viewport::Viewport { width: WIDTH, height: HEIGHT });';
            let code=read(path.join(docs,'journey/frame.rs')).replace('// COURSE_SIZE',`const WIDTH: u32 = ${w};\nconst HEIGHT: u32 = ${h};`)
                .replace('// COURSE_NEW',`let mut renderer = ${step.frame_constructor};\n    ${resize}\n    renderer.set_density(${density}.0);`);
            code=code.slice(0,code.indexOf('fn verify_pixels('))+fixture;
            code=code.replace('// COURSE_DRAW','draw(&renderer, &view);').replace('    verify_pixels(&pixels);','    assert_eq!(pixels.len(), WIDTH as usize * HEIGHT as usize * 4);');
            const profile={renderScale:2,lane:'native'},bridge=nativeScaleBridge(code,profile,[w,h],density,fixture);
            assert.equal(bridge.code.slice(bridge.code.indexOf('fn draw(')),code.slice(code.indexOf('fn draw(')));
            const body=bridge.code.slice(bridge.code.indexOf('fn make_renderer('));
            assert(body.indexOf('renderer.set_density(IMAGE_DENSITY)')<body.indexOf('math_quality.drain_matrix'));
            assert(body.includes('[WIDTH,HEIGHT],editor.selected)'));assert.equal(bridge.beforeSha256,bridge.reversedSha256);
            assert(bridge.code.includes(`const WIDTH: u32 = ${w*2};`));assert(bridge.code.includes(`const OUTPUT_WIDTH: u32 = ${w};`));
            assert.throws(()=>nativeScaleBridge(code,profile,[w,h],density,fixture+'\n'));
            for(const bad of ['',fixture.replace('&editor.scene);','&e.scene);'),fixture.replace('let mut renderer=Renderer::new','let renderer=Renderer::new'),
                fixture+'\nlet mut renderer=Renderer::new(device,queue,format,&math_source);'])
                assert.throws(()=>nativeScaleBridge(code.slice(0,-fixture.length)+bad,profile,[w,h],density,bad));
            const repeated=fixture.replace('let mut renderer=Renderer::new(device,queue,format,&editor.scene);','let mut renderer=Renderer::new(device,queue,format,&editor.scene);let mut renderer=Renderer::new(device,queue,format,&editor.scene);');
            assert.throws(()=>nativeScaleBridge(code.slice(0,-fixture.length)+repeated,profile,[w,h],density,repeated));
        }
    }});
});

test('surface network preserves two original surface quality owners and all six authored lines',async()=>{
    const before={css:[640,480],internal:[1280,960],dpr:2,requestedScale:1,camera:[1,0,0,1],selected:[],
        sources:[[],[{id:6},{id:7}],2,{}, {points:[],lines:Array.from({length:6},(_,i)=>[[i,0,0],[i,1,0]]),polylines:[]}],
        ownership:{instances:[],curves:[],placements:[['ObjectId(6)',[1]],['ObjectId(7)',[1]]]},
        clipping:{usage:[0,1,1],rows:[],caps:0,error:null,cap_error:null,ambient_error:null,appearance:{arctic:true,outlines:true}}};
    const after={...structuredClone(before),internal:[2560,1920],requestedScale:2,rendererProfile:{renderScale:2,density:4,internal:[2560,1920]}};
    assert.deepEqual(surfaceSnapshotOptions(before),{lane:'mesh',source:'surface',expectedRows:2,expectedQualities:2});
    const run=async(owners:number[])=>{let index=0;return applyBrowserScale({async evaluate(){return ++index===1?structuredClone(before):index===2?undefined:structuredClone(after);}},
        {renderScale:2,scene:'surface-network'},async options=>{assert.deepEqual(options,{lane:'mesh',source:'surface',expectedRows:2,expectedQualities:2});return{adaptive:true,quality:{qualities:owners.map(id=>[id,.25,.2,false])}};});};
    assert.equal((await run([7,6])).after.sources[4].lines.length,6);
    for(const owners of [[6],[6,6],[6,8],[6,7,8]])await assert.rejects(run(owners));
    for(const change of [(f:any)=>f.sources[0].push({id:8}),(f:any)=>f.sources[1][1].id=6,(f:any)=>f.sources[2]=3,
        (f:any)=>f.sources[4].lines.pop(),(f:any)=>f.ownership.curves.push({id:8}),(f:any)=>f.clipping.appearance.outlines=false]){
        const changed=structuredClone(before);change(changed);assert.throws(()=>surfaceSnapshotOptions(changed));
    }
});

function ordinaryFixture() {
    const matrix=[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1];
    return {css:[900,760],internal:[1800,1520],dpr:2,requestedScale:1,camera:matrix,selected:[],
        sources:[[],[],2,{current:null,rows:[0,1].map(id=>({id,visible:true,locked:false,faces:null,edges:null}))},{points:[],lines:[],polylines:[]},[0,0]],
        ownership:{instances:[],curves:[],residency:[['pink',true,null],['turquoise',true,null]],metadata:[['pink','pink-source','pink',true,false],['turquoise','turquoise-source','turquoise',true,false]],origins:[],placements:[['ObjectId(0)',matrix],['ObjectId(1)',matrix]]},
        clipping:{usage:[0,1,1],rows:[],caps:0,error:null,cap_error:null,ambient_error:null,appearance:{opacity:1,outlines:true,arctic:true}}};
}

test('Arctic ordinary owners use actual resident-null epochs and finite typed placements, not imported sample assumptions',()=>{
    ordinaryImageOwners(ordinaryFixture());
    for(const change of [(f:any)=>f.sources[0].push({id:8}),(f:any)=>f.sources[2]=3,(f:any)=>f.sources[3].rows[1].id=2,
        (f:any)=>f.sources[4].lines.push([[0,0,0],[1,0,0]]),(f:any)=>f.ownership.residency[0][1]=false,(f:any)=>f.ownership.residency[0][2]=0,
        (f:any)=>f.ownership.residency[1][0]='pink',(f:any)=>f.ownership.metadata[0][0]='other',(f:any)=>f.ownership.metadata[0][3]=false,
        (f:any)=>f.ownership.placements[0][0]='0',(f:any)=>f.ownership.placements[0][1]=[1],(f:any)=>f.ownership.placements[0][1][0]=Infinity,
        (f:any)=>f.ownership.instances.push({id:2}),(f:any)=>f.ownership.curves.push({id:2}),(f:any)=>f.clipping.rows.push({id:2})]){
        const frame=structuredClone(ordinaryFixture());change(frame);assert.throws(()=>ordinaryImageOwners(frame));
    }
});

test('zero-math image activation executes two fresh redraws and rejects missing quality, stale owners or changing complete counters',async()=>{
    const run=async(change:(values:any)=>void=()=>{},unstable:string|null=null)=>{
        const image=ordinaryFixture(),values:any={
            'data-cads':image.sources[0],'data-surfaces':image.sources[1],'data-object-count':2,'data-view-flags':image.sources[3],
            'data-drawing-originals':image.sources[4],'data-source-metadata':image.sources[5],'data-camera-matrix':image.camera,'data-selected-ids':[],
            'data-instances':[],'data-curve-originals':[],'data-source-residency':image.ownership.residency,'data-row-metadata':image.ownership.metadata,
            'data-source-origins':[],'data-row-placements':image.ownership.placements,
            'data-clipping':{...image.clipping,caps:[0,0,0],...image.clipping.appearance},
            'data-image-render-profile':{renderScale:1,density:2,internal:[1800,1520]},
            'data-math-display':{pending:false,limited:null,qualities:[],bytes:0,generations:0},
            'data-cpu-usage':[4,2,0,2,4096,2],'data-gpu-stats':[2,2,10,1024],'data-gpu-usage':[2,4096,2,320]};
        change(values);let resizeCount=0;
        const canvas:any={width:1800,height:1520,getBoundingClientRect:()=>({width:900,height:760}),getAttribute:(name:string)=>name in values?JSON.stringify(values[name]):null};
        const browserWindow:any={dispatchEvent:()=>{
            resizeCount++;canvas.width=3600;canvas.height=3040;values['data-image-render-profile']={renderScale:2,density:4,internal:[3600,3040]};
            if(resizeCount===3&&unstable){
                if(unstable==='owner')values['data-source-residency'][0][0]='changed';
                else if(unstable==='origin')values['data-source-origins'].push(['pink','file','version',null]);
                else if(unstable==='matrix')values['data-row-placements'][0][1][12]++;
                else values[unstable][1]++;
            }
        }};
        const context:any=vm.createContext({document:{querySelector:()=>canvas},window:browserWindow,devicePixelRatio:2,input:null,
            Event:class{constructor(public type:string){}},requestAnimationFrame:(fn:any)=>queueMicrotask(fn)});
        const evaluate=async(fn:any,arg?:any)=>{context.input=arg;return structuredClone(await vm.runInContext('('+fn.toString()+')(input)',context));};
        const page={evaluate,waitForFunction:async(fn:any,arg:any)=>assert(await evaluate(fn,arg),'Explicit empty mathematical diagnostics must qualify')};
        const result=await applyBrowserScale(page,{renderScale:2,scene:'arctic-demo'},async()=>{throw Error('Shared mathematical helper must never run for ordinary triangles');});
        assert.equal(resizeCount,3);assert.equal(result.settled.kind,'authored-mesh-static-reuse');assert.equal(result.settled.expectedQualities,0);
        assert.deepEqual(result.settled.first.cpu,[4,2,0,2,4096,2]);assert.equal(result.after.ownership.residency[0][2],null);return result;
    };
    await run();
    for(const field of ['data-cpu-usage','data-gpu-stats','data-gpu-usage','owner','origin','matrix'])await assert.rejects(run(()=>{},field));
    for(const change of [(v:any)=>delete v['data-math-display'],(v:any)=>v['data-math-display'].qualities.push([0,.25,.2,false]),
        (v:any)=>v['data-math-display'].pending=true,(v:any)=>v['data-math-display'].limited='limit',(v:any)=>v['data-math-display'].bytes=-1,
        (v:any)=>v['data-math-display'].generations=null,(v:any)=>v['data-gpu-usage'][0]=1,(v:any)=>v['data-gpu-usage'][2]=3,(v:any)=>v['data-gpu-usage'][2]=1,
        (v:any)=>v['data-cpu-usage'].pop(),(v:any)=>v['data-cpu-usage'][4]=NaN,(v:any)=>v['data-gpu-stats'][0]=1.5])await assert.rejects(run(change));
});

test('future Arctic recipes are exactly default-demo actions and preserve strict source schemas',()=>{
    const rows=actual.filter(x=>x.row.scene==='arctic-demo');assert.equal(rows.length,3);
    for(const {step,files}of rows){
        assert.deepEqual(step.browser_actions,[{command:'Arctic On'}]);assert(files['src/editor.rs'].includes('scene: Scene::demo()'));
        const demo=files['src/scene.rs'].split('    pub fn demo() -> Self {')[1].split('\n    pub fn restore(')[0];
        assert.equal(demo.split('PreparedMesh::triangle(').length-1,2);assert(demo.includes('scene.insert(pink)'));assert(demo.includes('scene.insert(turquoise)'));
        assert(files['src/browser.rs'].includes('row.geometry().is_some(), row.release_epoch()'));
        assert(files['src/browser.rs'].includes('format!("{:?}", row.id), row.model.m'));
        assert(files['src/browser.rs'].includes('let mut cpu = crate::memory::cpu(editor.presentation_scenes()'));
    }
    const historical=read(path.join(here,'historical-browser-actions-presentation.ts')),activation=historical.indexOf('const renderScale = await applyBrowserScale');
    assert(activation>historical.indexOf("assert.equal(camera.length,16)"));assert(activation>historical.indexOf('assert.deepEqual(errors,[])'));
    assert(activation<historical.indexOf('const drawingSha256=await helpers.exports.drawing(page)'));
    assert(historical.includes("assert.deepEqual(recipe.actions, [{command: 'Arctic On'}])"));assert(historical.includes('errors,drawingSha256,renderScale,'));
    const adaptive=read(path.join(docs,'journey/checks/browser/adaptive-gpu-reuse.cjs'));
    assert(adaptive.includes('options.expectedQualities>0'));assert(adaptive.includes('An explicit mathematical fixture cannot qualify with empty qualities'));
    const native=actual.filter(x=>x.row.scene==='element-solids');assert.equal(native.length,9);
    for(const {step}of native){const fixture=read(path.join(docs,step.frame!));assert(fixture.includes('renderer.set_outlines(true)'));assert(fixture.includes('global_outline_usage()[1],1'));}
});
