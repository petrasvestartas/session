import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import Module from 'node:module';
import {fileURLToPath} from 'node:url';
import {bindings} from './presentation.ts';
import {buildBrowser} from './browser-presentation.ts';
import {privateBrowserHost, observeGpu, validateGpu} from './private-browser-host.ts';
import {docs, viewer, filesIn, referenceFiles, referenceAssets, materialize, fingerprint, hash, json, read, write} from './model.ts';
import {nativeTarget, runCommand} from './verify.ts';
import {png, ppm} from './png.ts';

const script = fileURLToPath(import.meta.url);
const sha = (file: string) => hash(fs.readFileSync(file));
const measurementId = '85ay-exercise-current-original-commands-together';
const aliases: Record<string, string> = {
    '83zc-match-original-face-edge-directions.png': '84-measure-original-curves-surfaces-and-closed-solids',
    '83zd-orient-original-solid-face-groups.png': '84-measure-original-curves-surfaces-and-closed-solids',
    '96zza-read-section-plane-slopes-from-reserved-addresses.png': '96zz-separate-hidden-ink-color-from-pick-ownership',
    '96zzb-write-section-addresses-beside-color.png': '96zz-separate-hidden-ink-color-from-pick-ownership',
    '96zzc-write-section-addresses-before-the-first-pick.png': '96zz-separate-hidden-ink-color-from-pick-ownership',
};
const snapshot = () => {
    const roots = ['src', '../session_rust/src', '../../wood/rust/src', '../../wood/src', '../session_cpp/src', 'docs/lessons/37', 'docs/lessons/01'];
    const files = roots.flatMap(root => filesIn(path.resolve(viewer, root)));
    files.push(script, path.join(viewer, 'Cargo.toml'), path.join(viewer, 'Cargo.lock'),
        path.join(docs, 'capture_practice.cjs'), path.join(docs, 'capture_journey.cjs'),
        path.join(docs, 'journey/current-native-scenes.json'), path.join(docs, 'screenshots/journey/command-dock-preview.json'),
        path.join(viewer, 'assets/pb/course-comparison.pb'), path.resolve(viewer, '../../wood/data/floor_model.pb'),
        path.resolve(viewer, '../../wood/CMakeLists.txt'), path.resolve(viewer, '../session_cpp/CMakeLists.txt'),
        path.resolve(viewer, '../../wood/examples/templates_vault_4_dome.cpp'), path.resolve(viewer, '../../wood/examples/main_session_round_trip.cpp'));
    return Object.fromEntries(files.map(file => [file, sha(file)]));
};
const check = (plan: any) => { for (const [file, digest] of Object.entries({...plan.inputs,...plan.generatorFiles})) assert.equal(sha(file), digest, file); };
const image = (file: string, destination: string) => {
    const bytes = fs.readFileSync(file); assert(bytes.subarray(0, 8).equals(Buffer.from([137,80,78,71,13,10,26,10])));
    return {path: file, destination, sha256: hash(bytes), width: bytes.readUInt32BE(16), height: bytes.readUInt32BE(20)};
};
function command(label: string, args: string[], cwd: string, env: Record<string, string> = {}) {
    return runCommand(label, args, cwd, {COURSE_MEMORY_MIB: '4096', COURSE_TIMEOUT_SECONDS: '1800', ...env});
}
function nativeExample(label:string,folder:string,name:string,args:string[],env:Record<string,string>,retained?:any) {
    const commands:any[]=retained?[]:[command(label+'-build',['cargo','build','--example',name,'--locked','--offline','--target',nativeTarget(),'-j1'],folder)];
    const executable=path.join(viewer,'target/course-build',nativeTarget(),'debug/examples',name+(process.platform==='win32'?'.exe':''));
    if(retained)assert.equal(sha(executable),retained.executable.sha256,'Reuse only this exact unchanged native executable');
    const transport=[];
    if(process.platform==='win32') {
        const directory=path.dirname(process.env.VIEWER_DXC_PATH||path.join(viewer,'target/course-windows-20261018/native-checkpoint/dxcompiler.dll'));
        for(const name of ['dxcompiler.dll','dxil.dll']) {const original=path.join(directory,name),target=path.join(path.dirname(executable),name);assert(fs.existsSync(original));if(!fs.existsSync(target))fs.copyFileSync(original,target);assert.equal(sha(original),sha(target));transport.push({path:target,sha256:sha(target)});}
    }
    commands.push(command(label+'-run',[executable,...args],folder,env));
    return {commands,executable:{path:executable,sha256:sha(executable)},transport};
}
function meaningfulFrame(frame:string) {
    const decoded=ppm(frame),counts=new Map<number,number>();
    for(let i=0;i<decoded.pixels.length;i+=3){const key=decoded.pixels[i]*65536+decoded.pixels[i+1]*256+decoded.pixels[i+2];counts.set(key,(counts.get(key)||0)+1);}
    let modal=0;for(const count of counts.values())modal=Math.max(modal,count);
    const foreground=decoded.width*decoded.height-modal;
    assert(counts.size>4&&foreground>1000,'A supporting CAD scene must contain actual nonblank geometry');
    return {colors:counts.size,nonModalPixels:foreground};
}

export function prepareSupplemental(nativeCohort: string, browserCohort: string, attempt = 'v1') {
    const native = json(path.join(nativeCohort, 'cohort.json')), browser = json(path.join(browserCohort, 'cohort.json'));
    assert.equal(native.density, 2); assert.equal(browser.density, 2);
    assert.equal(native.inventory.sha256,browser.inventory.sha256);
    const inventory = json(native.inventory.path), inputs = snapshot();
    const key = hash(JSON.stringify({inputs, native: native.key, browser: browser.key, attempt}));
    const folder = path.join(viewer, 'target/course-presentation/supplemental-' + key.slice(0, 20));
    const planFile = path.join(folder, 'plan.json');
    if (fs.existsSync(planFile)) { const plan = json(planFile); check(plan); return plan; }
    const rows = inventory.images.filter((row: any) => aliases[path.basename(row.path)]);
    for (const row of rows) {
        const original = inventory.images.find((other: any) => other.kind === 'native' && other.checkpoint === aliases[path.basename(row.path)]);
        assert(original); assert.equal(row.sha256, original.sha256, 'Alias must retain actual original byte-identity evidence');
    }
    const plan:any = {status: 'PREPARED_NOT_EXECUTED', folder, key, attempt, inputs, nativeCohort: path.resolve(nativeCohort), browserCohort: path.resolve(browserCohort),
        inventory: native.inventory, aliases: rows.map((row: any) => ({...row, representative: aliases[path.basename(row.path)]})),
        dock: {id: '04-input', original: path.join(docs, 'screenshots/journey/command-dock-preview.json'),
            scope: 'Fresh actual Background completion/history integration on canonical04; original prototype source identity remains historical, not a claimed exact source match.'},
        measurement: {id: measurementId, original: path.join(viewer, 'target/course-windows-20261018/current85ay-acceptance-publication-preparation/framed-measurement/capture-visible.cjs')},
        scenes: json(path.join(docs, 'journey/current-native-scenes.json')).records.map((row: any) => ({name: row.name, originalSourceSha256: row.source_sha256,
            settings: row.settings, originalCameraAndTimings: 'Retained unchanged in original current-native-scenes.json; fresh execution is not the old benchmark.'})),
        scope: 'Fifteen supporting destinations only; source-bound real GPU captures or explicit fresh representative aliases. No completed historical behavior claim.'};
    plan.generatorRecipe=generatorSources(plan);
    plan.generatorFiles=Object.fromEntries(['floor.cpp','dome.cpp','CMakeLists.txt'].map(name=>{const file=path.join(plan.generatorRecipe.folder,name);return[file,sha(file)];}));
    write(planFile, JSON.stringify(plan, null, 2) + '\n'); return plan;
}

function referenceStage(plan: any, id: string, variant: string) {
    const folder = path.join(plan.folder, 'reference-' + id + '-' + variant);
    const files = referenceFiles(id), originalSource = fingerprint(files);
    assert(!fs.existsSync(folder), 'Preserve previous supplemental native attempts: ' + folder);
    materialize(files, folder, path.join(docs, 'lessons/37/Cargo.lock')); referenceAssets(folder);
    const cargoFile = path.join(folder, 'Cargo.toml'), lockFile = path.join(folder, 'Cargo.lock');
    let cargo = read(cargoFile), lock = read(lockFile);
    const original = cargo.match(/\[package\]\s*\nname = "([^"]+)"/)![1];
    const unique = 'supplemental_' + id + '_' + variant.replaceAll('-', '_') + '_' + originalSource.slice(0, 12);
    cargo = cargo.replace('name = "' + original + '"', 'name = "' + unique + '"').replace('[lib]', '[lib]\nname = "session_viewer"');
    assert.equal(lock.split('name = "' + original + '"').length, 2);
    lock = lock.replace('name = "' + original + '"', 'name = "' + unique + '"');
    write(cargoFile, cargo); write(lockFile, lock);
    return {folder, originalSource, unique, cargoSha256: sha(cargoFile), lockSha256: sha(lockFile)};
}

function nativeReference(plan: any, id: string, variant: string) {
    const stage = referenceStage(plan, id, variant), commands: any[] = [];
    if (id === '01') {
        const example = read(path.join(stage.folder, 'examples/course_frame.rs'));
        assert(example.includes('Gpu::new_headless(640, 480)'));
        write(path.join(stage.folder, 'examples/supplemental_frame.rs'), example.replace('Gpu::new_headless(640, 480)', 'Gpu::new_headless(1280, 960)').replace('P6\\n640 480\\n255\\n', 'P6\\n1280 960\\n255\\n'));
        if (variant === 'grey') {
            const shader = path.join(stage.folder, 'src/shaders/background.wgsl'), text = read(shader);
            assert.equal(text.split('select(1.0,0.94,').length, 2);
            write(shader, text.replace('select(1.0,0.94,', 'select(0.5,0.94,'));
        }
        commands.push(nativeExample('supplemental-' + plan.key.slice(0,12) + '-01-' + variant,stage.folder,'supplemental_frame',[],{VIEWER_ADAPTER:'Microsoft'}));
    } else {
        const selftest = path.join(stage.folder, 'src/selftest.rs'), text = read(selftest);
        const before = 'let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");';
        assert.equal(text.split(before).length, 2);
        write(selftest, text.replace(before, before + '\n    gpu.logical_size = [f64::from(w)/2.0,f64::from(h)/2.0];'));
        commands.push(nativeExample('supplemental-' + plan.key.slice(0,12) + '-37',stage.folder,'selftest',['lesson.ppm','assets/view_local.yaml'],
            {VIEWER_W:'1800',VIEWER_H:'1400',VIEWER_ADAPTER:'Microsoft'}));
    }
    const frame = path.join(stage.folder, 'lesson.ppm'), decoded = ppm(frame);
    assert.equal(decoded.width, id === '01' ? 1280 : 1800); assert.equal(decoded.height, id === '01' ? 960 : 1400);
    if (id === '01') {
        const first = decoded.pixels.subarray(0,3);
        for (let i=0;i<decoded.pixels.length;i+=3) assert(decoded.pixels.subarray(i,i+3).equals(first), 'Actual empty frame must be uniform');
        assert(variant === 'white' ? first.every(v=>v===255) : first.every(v=>v>20&&v<240));
    }
    const content=id==='37'?meaningfulFrame(frame):null;
    const target = path.join(stage.folder, 'native.png'); write(target, png(frame));
    return {...stage, commands, content, image: image(target, id === '37' ? 'docs/screenshots/steps/37-native.png' : ''),
        overlays: id === '01' ? ['Physical readback1280x960; original example/empty GPU draw; original gray shader exercise retained.']
            : ['Physical1800x1400; original reference37 selftest/manifest/scene; logical900x700; only presentation scale initialization added.']};
}

function generatorSources(plan: any) {
    const folder = path.join(plan.folder, 'generators'), wood = path.resolve(viewer, '../../wood');
    const domeFile = path.join(wood, 'examples/templates_vault_4_dome.cpp');
    let dome = read(domeFile); assert.equal(dome.split('wood_session.pb_dump(pb_path("live"));').length,2);
    dome = dome.replace('int main() {','int main(int argc,char** argv) {\n    if(argc!=2)return 64;').replace('wood_session.pb_dump(pb_path("live"));','wood_session.pb_dump(argv[1]);');
    const floor = '#include "wood_session.h"\nusing namespace session_cpp;using namespace wood_session;\nint main(int argc,char**argv){if(argc!=3)return 64;config::reset_defaults();auto scene=WoodSession::pb_load(argv[1]);scene.compute_face_contacts();scene.compute_features();scene.pb_dump(argv[2]);return 0;}\n';
    const cmake = `cmake_minimum_required(VERSION 3.22)\nproject(SupplementalScenes LANGUAGES CXX)\nset(CMAKE_CXX_STANDARD 23)\nadd_subdirectory("${wood.split(path.sep).join('/')}" wood-core EXCLUDE_FROM_ALL)\nforeach(scene floor dome)\n add_executable(supplemental_\${scene} \${scene}.cpp)\n target_link_libraries(supplemental_\${scene} PRIVATE wood_core session_core)\n LINK_PROTOBUF(supplemental_\${scene})\n set_target_properties(supplemental_\${scene} PROPERTIES RUNTIME_OUTPUT_DIRECTORY "\${CMAKE_BINARY_DIR}" RUNTIME_OUTPUT_DIRECTORY_RELEASE "\${CMAKE_BINARY_DIR}")\nendforeach()\n`;
    write(path.join(folder,'dome.cpp'),dome); write(path.join(folder,'floor.cpp'),floor); write(path.join(folder,'CMakeLists.txt'),cmake);
    return {folder, originalDome:{path:domeFile,sha256:sha(domeFile)}, originalFloorRecipe:path.join(wood,'examples/main_session_round_trip.cpp'),
        scope:'Current production generation recipes in scratch outputs only. Old byte hashes and old C++ baseline remain historical; no old native timing claim.'};
}

function generateInputs(plan: any) {
    const recipe = plan.generatorRecipe, build = path.join(plan.folder,'generator-build');
    const output = path.join(plan.folder,'scene-inputs'); fs.mkdirSync(output,{recursive:true});
    const commands = [command('supplemental-cmake-'+plan.key.slice(0,12),['cmake','-S',recipe.folder,'-B',build,'-DCMAKE_BUILD_TYPE=Release','-DSESSION_CPP_LOCAL='+path.resolve(viewer,'../session_cpp')],viewer),
        command('supplemental-cpp-'+plan.key.slice(0,12),['cmake','--build',build,'--config','Release','--target','supplemental_floor','supplemental_dome','--parallel','1'],viewer)];
    const exe = (name:string)=>path.join(build,'supplemental_'+name+(process.platform==='win32'?'.exe':''));
    const floor=path.join(output,'floor-output.pb'),dome=path.join(output,'dome.pb');
    assert(!fs.existsSync(floor)&&!fs.existsSync(dome),'Preserve generated inputs from previous execution');
    commands.push(command('supplemental-floor-'+plan.key.slice(0,12),[exe('floor'),path.resolve(viewer,'../../wood/data/floor_model.pb'),floor],viewer));
    commands.push(command('supplemental-dome-'+plan.key.slice(0,12),[exe('dome'),dome],viewer));
    const record={status:'GENERATED_CURRENT_SOURCE_VARIANTS',recipe,commands,inputs:{'floor-output':{path:floor,sha256:sha(floor)},dome:{path:dome,sha256:sha(dome)}}};
    write(path.join(plan.folder,'generated-inputs.json'),JSON.stringify(record,null,2)+'\n'); return record;
}

function nativeScenes(plan:any, generated:any) {
    const sources:Record<string,string>={comparison:path.join(viewer,'assets/pb/course-comparison.pb'),'floor-input':path.resolve(viewer,'../../wood/data/floor_model.pb'),
        'floor-output':generated.inputs['floor-output'].path,dome:generated.inputs.dome.path};
    let retained:any;
    return plan.scenes.map((entry:any)=>{
        const input=sources[entry.name],before=sha(input),frame=path.join(plan.folder,'current-native-'+entry.name+'.ppm');
        if(generated.inputs[entry.name])assert.equal(before,generated.inputs[entry.name].sha256);
        const executed=nativeExample('supplemental-scene-'+entry.name+'-'+plan.key.slice(0,12),viewer,'selftest',[frame,input],
            {...entry.settings,VIEWER_W:'1280',VIEWER_H:'960',VIEWER_DPR:'2',VIEWER_ADAPTER:'Microsoft'},retained);
        retained=executed;
        assert.equal(sha(input),before);const decoded=ppm(frame);assert.equal(decoded.width,1280);assert.equal(decoded.height,960);
        const target=frame.replace(/\.ppm$/,'.png');write(target,png(frame));
        return {name:entry.name,input:{path:input,sha256:before,originalSha256:entry.originalSourceSha256,originalBytesMatch:before===entry.originalSourceSha256},executed,content:meaningfulFrame(frame),
            image:image(target,'docs/screenshots/journey/current-native-'+entry.name+'.png'),
            scope:'Fresh actual maintained native render at1280x960/logical640x480 with original eight-frame/orbit/settings recipe. Source variants recorded explicitly; no historical byte/timing identity claim.'};
    });
}

async function browserRecipe(plan:any, mode:string, native?:any) {
    const id=mode==='measurement'?measurementId:'04-input';
    const folder=path.join(plan.folder,mode);assert(!fs.existsSync(folder),'Preserve previous browser attempt: '+folder);
    let workspace:string,built:any;
    if(mode==='practice') {workspace=path.join(folder,'workspace');fs.mkdirSync(path.join(workspace,'target/course-checks'),{recursive:true});}
    else {built=buildBrowser(id,2);workspace=path.join(folder,'workspace');fs.cpSync(built.workspace,workspace,{recursive:true});}
    const host=await privateBrowserHost(workspace,folder,script,{id,mode,folder,native,originalCapture:path.join(docs,'capture_journey.cjs')});
    const result=json(path.join(folder,'worker-result.json'));
    if(built) {assert.deepEqual(built.source,bindings(id));for(const [name,digest]of Object.entries(built.bundle))assert.equal(sha(path.join(workspace,'target/course-checks',id,'dist',name)),digest);}
    return {...result,host,source:built?.source||native,scope:mode==='practice'?'Original practice caption layout around genuine higher-resolution native readbacks; browser has no CAD/GPU behavior claim.':
        'Actual source-bound rendering of the declared compact presentation recipe; no skipped/default callback acceptance claim.'};
}

async function worker(config:any) {
    const require=Module.createRequire(path.join(docs,'capture_journey.cjs'));
    const {chromium}=require('playwright');
    const browser=await chromium.launch({headless:true,executablePath:process.env.VIEWER_BROWSER_EXECUTABLE,args:JSON.parse(process.env.VIEWER_CHROME_ARGS!)});
    try {
        const context=await browser.newContext({viewport:{width:config.mode==='practice'?1000:900,height:760},deviceScaleFactor:2,acceptDownloads:true});
        const page=await context.newPage(),images:any[]=[],errors:string[]=[];page.on('pageerror',(e:any)=>errors.push(e.message));
        if(config.mode==='practice') {
            for(const row of config.native) {
                const grey=row.variant==='grey',data=fs.readFileSync(row.image.path).toString('base64');
                await page.setContent(`<style>body{margin:0;padding:24px;font:18px system-ui;background:#eef0f4}main{display:inline-block;background:white;padding:24px}img{display:block;border:1px solid #bbb;width:640px;height:480px}p{max-width:640px}</style><main><h2>Lesson01 · ${grey?'Change the shader colour':'First GPU frame'}</h2><img src="data:image/png;base64,${data}"><p>Actual1280 ×960 pixel readback at640 ×480 logical pixels. The fullscreen triangle paints a ${grey?'grey':'white'} background.</p></main>`);
                const destination='docs/screenshots/practice/01-frame'+(grey?'-grey':'')+'.png',target=path.join(config.folder,path.basename(destination));
                await page.locator('main').screenshot({path:target});images.push(image(target,destination));
            }
        } else {
            await (new Function('page','return (async()=>{'+observeGpu+'})()'))(page);
            const original=read(config.originalCapture),boundary='\nasync function capture()';assert.equal(original.split(boundary).length,2);
            const helpers=new Module(config.originalCapture);helpers.filename=config.originalCapture;helpers.paths=Module._nodeModulePaths(path.dirname(config.originalCapture));
            const previous=process.argv;try{process.argv=[previous[0],config.originalCapture,config.id];helpers._compile(original.split(boundary)[0]+'\nmodule.exports={command,drawing,focusCommand};',config.originalCapture);}finally{process.argv=previous;}
            const {command,drawing,focusCommand}=helpers.exports;
            await page.goto(process.env.VIEWER_URL);
            await page.waitForFunction(()=>!!document.querySelector('canvas')?.getAttribute('data-command-ui'));
            await page.waitForFunction(()=>{const c=document.querySelector('canvas')!,r=c.getBoundingClientRect();return window.devicePixelRatio===2&&c.width===Math.round(r.width*2)&&c.height===Math.round(r.height*2);});
            const save=async(name:string,destination:string)=>{await (new Function('page','assert','return (async()=>{'+validateGpu+'})()'))(page,assert);const target=path.join(config.folder,name);await page.screenshot({path:target});images.push(image(target,destination));};
            if(config.mode==='measurement') {
                for(const line of ['Close','CPlane XY','Box w-1,-1,0 2 2 0.4','Select All','BoundingBox','Length BoundingBox','Undo','Redo','View Isometric','Select None','Fit','Measure Distance w-1,0,1 w1,0,1'])await command(page,line);
                const measurement=JSON.parse(await page.locator('canvas').getAttribute('data-measurement'));assert.deepEqual(measurement,{points:[[-1,0,1],[1,0,1]],label:'2 mm'});
                const invariant=async()=>page.locator('canvas').evaluate(c=>Object.fromEntries(['data-measurement','data-measurement-gpu-bytes','data-object-count','data-drawing-originals','data-row-placements','data-row-metadata','data-selected-ids'].map(k=>[k,c.getAttribute(k)])));
                const beforeFraming=await invariant(),box=await page.locator('canvas').boundingBox();await page.mouse.move(box.x+box.width/2,box.y+box.height/2);await page.mouse.wheel(0,480);await page.waitForTimeout(200);
                assert.deepEqual(await invariant(),beforeFraming,'Trusted framing preserves measurement/source/history');assert.equal(Number(beforeFraming['data-object-count']),2);
                assert.deepEqual(JSON.parse(await page.locator('canvas').getAttribute('data-measurement')),measurement);assert(Number(await page.locator('canvas').getAttribute('data-measurement-gpu-bytes'))>0);
                const m=JSON.parse(await page.locator('canvas').getAttribute('data-camera-matrix')),clip=[0,1,2,3].map(i=>m[8+i]+m[12+i]);
                const anchor={x:(clip[0]/clip[3]+1)*box.width/2,y:(1-clip[1]/clip[3])*box.height/2};assert(anchor.x>80&&anchor.x<box.width-80&&anchor.y>60&&anchor.y<box.height-100);
                await save('current85ay-visible-two-mm.png','docs/journey/release/current85ay-visible-two-mm.png');
            } else {
                const initial=JSON.parse(await page.locator('canvas').getAttribute('data-command-ui')),canvasBox=await page.locator('canvas').boundingBox();
                if(initial.top<canvasBox.height-70){const fold=initial.controls.find((c:any)=>c.key==='command/collapse');assert(fold);await page.mouse.click(canvasBox.x+(fold.rect[0]+fold.rect[2])/2,canvasBox.y+(fold.rect[1]+fold.rect[3])/2);await page.waitForFunction(()=>{const c=document.querySelector('canvas')!;return JSON.parse(c.getAttribute('data-command-ui')!).top>c.getBoundingClientRect().height-70;});}
                const baseline=await drawing(page);await command(page,'Background');assert.notEqual(await drawing(page),baseline);await command(page,'Background');assert.equal(await drawing(page),baseline);
                await focusCommand(page);await page.keyboard.type('Bac');await page.waitForFunction(()=>JSON.parse(document.querySelector('canvas')!.getAttribute('data-command-ui')!).command==='Background');
                await save('command-dock-preview-completion.png','docs/screenshots/journey/command-dock-preview-completion.png');
                await page.keyboard.press('Escape');const ui=JSON.parse(await page.locator('canvas').getAttribute('data-command-ui'));const fold=ui.controls.find((c:any)=>c.key==='command/collapse');assert(fold);
                await page.mouse.click(canvasBox.x+(fold.rect[0]+fold.rect[2])/2,canvasBox.y+(fold.rect[1]+fold.rect[3])/2);
                await page.waitForFunction(top=>JSON.parse(document.querySelector('canvas')!.getAttribute('data-command-ui')!).top<top-20,ui.top);
                await save('command-dock-preview-history.png','docs/screenshots/journey/command-dock-preview-history.png');
            }
        }
        assert.deepEqual(errors,[]);write(path.join(config.folder,'worker-result.json'),JSON.stringify({status:'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW',images,errors},null,2)+'\n');
    } finally {await browser.close();}
}

export async function runSupplemental(planFile:string,scope:string) {
    assert(['all','aliases','practice','reference','native-inputs','scenes','dock','measurement'].includes(scope));
    const plan=json(planFile);check(plan);const file=path.join(plan.folder,'result.json');
    const result=fs.existsSync(file)?json(file):{status:'PARTIAL_NOT_ACCEPTED',sourcePlan:{path:planFile,sha256:sha(planFile)},items:{},images:[]};
    const save=()=>{result.images=Object.values(result.items).flatMap((item:any)=>item.images||[item.image]).filter(Boolean);write(file,JSON.stringify(result,null,2)+'\n');};
    for(const mode of ['aliases','practice','reference','native-inputs','scenes','dock','measurement']) {
        if(scope!=='all'&&scope!==mode||result.items[mode])continue;check(plan);
        if(mode==='aliases') {
            const progress=json(path.join(plan.nativeCohort,'progress.json'));const images=[];
            for(const alias of plan.aliases) {const actual=progress.items[alias.representative];assert(actual?.image,'Fresh representative capture required: '+alias.representative);assert.equal(sha(actual.sourceRecord),actual.sourceRecordSha256);
                const record=json(actual.sourceRecord);assert.deepEqual(record.source,bindings(alias.representative));const input=path.resolve(viewer,actual.image.path);assert.equal(sha(input),actual.image.sha256);
                images.push({...image(input,alias.path),representative:alias.representative,originalAliasSha256:alias.sha256,sourceRecord:actual.sourceRecord,retiredCheckpointRebuilt:false,
                    scope:'Filename-only alias of this fresh actual representative scene. Original historical identical PNG and retired source identities remain historical.'});}
            result.items[mode]={images};
        } else if(mode==='practice') {const native=['white','grey'].map(variant=>({...nativeReference(plan,'01',variant),variant}));result.items[mode]=await browserRecipe(plan,mode,native);}
        else if(mode==='reference')result.items[mode]=nativeReference(plan,'37','scene');
        else if(mode==='native-inputs')result.items[mode]=generateInputs(plan);
        else if(mode==='scenes') {const generated=result.items['native-inputs']||json(path.join(plan.folder,'generated-inputs.json'));result.items[mode]={images:nativeScenes(plan,generated).map((row:any)=>({...row.image,execution:row}))};}
        else result.items[mode]=await browserRecipe(plan,mode);
        check(plan);save();
    }
    result.status=result.images.length===15?'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW':'PARTIAL_NOT_ACCEPTED';save();return result;
}

export function publishSupplemental(planFile:string,reviewFile:string) {
    const plan=json(planFile),result=json(path.join(plan.folder,'result.json')),review=json(reviewFile);check(plan);
    assert.equal(review.rootPersonallyViewedOriginalResolution,true);assert(review.images.length>0);
    const inventory=json(plan.inventory.path),copied=[];
    for(const approved of review.images) {
        const row=result.images.find((row:any)=>row.destination===approved.destination);assert(row);assert.equal(row.sha256,approved.sha256);assert.equal(sha(row.path),row.sha256);
        const original=inventory.images.find((image:any)=>image.path===row.destination);assert(original);
        if(path.basename(row.destination).startsWith('current-native-'))assert.equal(review.sourceVariantCaptionsReviewed,true,'Native scene captions must distinguish fresh inputs/adapter/resolution from historical records');
        const destination=path.resolve(viewer,row.destination);assert(destination.startsWith(docs+path.sep));
        const backup=path.join(plan.folder,'original-images',row.destination);
        if(sha(destination)!==row.sha256) {assert.equal(sha(destination),original.sha256,'Preserve edits outside this image cohort');fs.mkdirSync(path.dirname(backup),{recursive:true});if(!fs.existsSync(backup))fs.copyFileSync(destination,backup,fs.constants.COPYFILE_EXCL);assert.equal(sha(backup),original.sha256);fs.copyFileSync(row.path,destination);}
        copied.push({destination:row.destination,sha256:sha(destination),sourceRecord:path.join(plan.folder,'result.json')});
    }
    const publication={status:'PUBLISHED_ROOT_REVIEWED_IMAGES_ONLY',review:{path:reviewFile,sha256:sha(reviewFile)},copied};
    write(path.join(plan.folder,'publication.json'),JSON.stringify(publication,null,2)+'\n');return publication;
}

if(process.argv[2]==='--worker') await worker(json(process.argv[3]));
else if(process.argv[1]&&path.resolve(process.argv[1])===script) {
    const args=process.argv.slice(2),mode=args.shift(),option=(name:string)=>{const index=args.indexOf(name);return index<0?undefined:args[index+1];};
    const result=mode==='prepare'?prepareSupplemental(option('--native-cohort')!,option('--browser-cohort')!,option('--attempt')||'v1'):
        mode==='run'?await runSupplemental(option('--plan')!,option('--scope')||'all'):
        mode==='publish'?publishSupplemental(option('--plan')!,option('--review')!):null;
    assert(result,'Usage: supplemental-presentation.ts prepare --native-cohort DIR --browser-cohort DIR | run --plan FILE --scope aliases|practice|reference|native-inputs|scenes|dock|measurement|all');
    console.log(JSON.stringify(result,null,2));
}
