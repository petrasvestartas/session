import test from 'node:test';
import assert from 'node:assert/strict';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {createRequire} from 'node:module';
import vm from 'node:vm';
import {imageProfile, nativeScaleBridge, browserScaleBridge, validateBrowserScale, applyBrowserScale, capSnapshotOptions} from './image-render-scale.ts';
import {course, expected, withExpectedSequence, docs, read} from './model.ts';

const here = path.dirname(fileURLToPath(import.meta.url));
const scope = JSON.parse(read(path.join(here, 'image-render-scale-scope.json')));
const steps = course().steps;

test('exact 48-profile scope binds current fixture bytes and excludes other lanes and defaults', () => {
    assert.equal(scope.length, 48); assert.equal(scope.filter((r:any)=>r.lane==='native').length, 42);
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

test('all six actual browser sources transform reversibly without changing single-sample cap shaders', () => {
    for(const {row,files} of actual.filter(x=>x.row.lane==='browser')) {
        const original=files['src/browser.rs'], bridge=browserScaleBridge(original,imageProfile(row.id,'browser','2',docs));
        let reversed=bridge.code; for(const c of [...bridge.changes].reverse())reversed=reversed.replace(c.after,c.before);
        assert.equal(reversed,original);assert.equal(bridge.beforeSha256,bridge.reversedSha256);
        assert(bridge.code.indexOf('density * course_image_scale(window)')<bridge.code.indexOf('renderer.set_density',bridge.code.indexOf('fn resize(')));
        assert(!bridge.code.includes('set_device_pixel_ratio'));assert(!bridge.code.includes('sample_count: 4'));
        assert.throws(()=>browserScaleBridge(original.replace('fn resize(', 'fn changed_resize('),bridge.profile));
    }
});

test('42 real native fixtures retain draw/pixel tails, refine at internal size, read back only output size', () => {
    for (const {row,step,files} of actual.filter(x=>x.row.lane==='native')) for(const density of [2,3]) {
        const [w,h]=(step.frame_size||[640,480]).map((n:number)=>n*density);
        const renderer=files['src/renderer.rs'];const signature=renderer.match(/pub fn resize\([^)]*\)/)![0];
        const resize=signature.includes('width: u32, height: u32')?'renderer.resize(WIDTH, HEIGHT);':'renderer.resize(viewer_journey::viewport::Viewport { width: WIDTH, height: HEIGHT });';
        let code=read(path.join(docs,'journey/frame.rs')).replace('// COURSE_SIZE',`const WIDTH: u32 = ${w};\nconst HEIGHT: u32 = ${h};`)
            .replace('// COURSE_NEW',`let mut renderer = ${step.frame_constructor};\n    ${resize}\n    renderer.set_density(${density}.0);`);
        code=code.slice(0,code.indexOf('fn verify_pixels('))+read(path.join(docs,step.frame!));
        code=code.replace('// COURSE_DRAW','draw(&renderer, &view);').replace('    verify_pixels(&pixels);','    assert_eq!(pixels.len(), WIDTH as usize * HEIGHT as usize * 4);');
        const bridge=nativeScaleBridge(code,imageProfile(row.id,'native','2',docs),[w,h],density);
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
    for(const row of scope.filter((r:any)=>r.lane==='browser'&&!r.id.startsWith('88ad')))assert(wrapper.includes("'"+row.id+"'"));
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
        const route=row.id.startsWith('88ad-')?'exported':'main-page';
        assert.equal(preflight(configured,row.id,'browser',docs,route,assert).renderScale,2);
        assert.throws(()=>preflight(configured,row.id,'browser',docs,'generic',assert),/exact image recipe/);
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
    for(const {files} of actual.filter(x=>x.row.lane==='browser')) {
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
