import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import vm from 'node:vm';
import assert from 'node:assert/strict';
import {test} from 'node:test';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {imageBuildMode, nativeSamplePhase} from './image-low-disk.ts';

const sha = (bytes: string | Buffer) => createHash('sha256').update(bytes).digest('hex');
const here = path.dirname(fileURLToPath(import.meta.url));
const packages = ['image_incremental', 'session_rust', 'wood', 'session_viewer'];

function fixture(lowDisk = true, declaredPackages = packages) {
    const root = fs.mkdtempSync(path.join(os.tmpdir(), 'course-low-disk-')), viewer = path.join(root, 'viewer');
    const workbench = path.join(viewer, 'target/course-ib/compact-browser-v1/workspace');
    const cache = path.join(viewer, 'target/course-cargo'), host = 'x86_64-pc-windows-msvc';
    const native = path.join(cache, host), folder = path.join(viewer, 'target/case');
    const put = (file: string, text: string) => { fs.mkdirSync(path.dirname(file), {recursive: true}); fs.writeFileSync(file, text); };
    put(path.join(workbench, 'examples/sample.rs'), 'fixture generator');
    put(path.join(workbench, 'src/lib.rs'), 'fixture source');
    const lock = 'version = 4\n[[package]]\nname = "regcrate"\nversion = "1.2.3"\nsource = "registry+fixture"\n' + declaredPackages.map(name => '\n[[package]]\nname = "' + name + '"\nversion = "0.1.0"\n').join('');
    put(path.join(workbench, 'Cargo.toml'), 'fresh manifest'); put(path.join(workbench, 'Cargo.lock'), lock);
    fs.mkdirSync(folder, {recursive: true});
    const cargo = path.join(root, 'cargo.exe'); put(cargo, 'pinned cargo');
    for (const name of packages) put(path.join(native, 'debug/deps', name + '.rlib'), 'local ' + name);
    const kept = [path.join(native, 'debug/deps/registry.rlib'), path.join(cache, 'debug/deps/host.rlib'),
        path.join(cache, 'wasm32-unknown-unknown/debug/deps/wasm.rlib')];
    for (const file of kept) put(file, 'retained third-party ' + file);
    const deps = path.join(native, 'debug/deps'), registry = path.join(root, '.cargo/registry/src/test-index/regcrate-1.2.3');
    put(path.join(registry, 'Cargo.toml'), 'registry package'); put(path.join(registry, 'src/lib.rs'), 'registry source');
    const siblings = (variant: string) => [path.join(deps, 'libregcrate-' + variant + '.rlib'), path.join(deps, 'libregcrate-' + variant + '.rmeta'), path.join(deps, 'regcrate-' + variant + '.d')];
    const current = siblings('aaaaaaaaaaaaaaaa'), obsolete = siblings('bbbbbbbbbbbbbbbb');
    for (const files of [current, obsolete]) {
        put(files[0], 'registry rlib ' + files[0]); put(files[1], 'registry metadata ' + files[1]);
        put(files[2], files[2] + ': ' + path.join(registry, 'src/lib.rs') + '\n');
    }
    kept.push(...current);
    put(path.join(path.dirname(workbench), 'workbench.lock'), JSON.stringify({pid:process.pid}));
    const sampleExe = path.join(native, 'debug/examples/sample.exe'); put(sampleExe, 'sample executable');
    kept.push(sampleExe);
    const artifactRecords: any[] = [
        {reason:'compiler-artifact',package_id:'registry+fixture#regcrate@1.2.3',manifest_path:path.join(registry,'Cargo.toml'),fresh:true,target:{name:'regcrate',kind:['lib']},filenames:[current[0],current[1]]},
        {reason:'compiler-artifact',package_id:'path+fixture#image_incremental@0.1.0',manifest_path:path.join(workbench,'Cargo.toml'),fresh:false,target:{name:'sample',kind:['example']},filenames:[sampleExe],executable:sampleExe},
        {reason:'build-finished',success:true},
    ];
    const before = new Map(kept.map(file => [file, fs.readFileSync(file)]));
    const events: string[] = [], calls: any[] = [];
    const controls = {sampleFails: false, copyFails: false, corruptCopy: false, cleanFails: false, output: null as string | null, changeLock: false, redirected: false, emptyLog: false, driftRegistry: false, registrySymlink: false};
    const p = {folder, buildMode: imageBuildMode({COURSE_IMAGE_LOW_DISK: lowDisk ? '1' : '0'}), group: {workbench, target: cache}, tools: {rustc: 'rustc fixture\nhost: ' + host + '\n'},
        overlay: {cargoSha256: sha('fresh manifest'), lockSha256: sha(lock)}, toolFiles: {cargo: {path: cargo, sha256: sha('pinned cargo')}}};
    const io: any = {...fs, copyFileSync(source: string, target: string) {
        events.push('copy'); if (controls.copyFails) throw Error('copy failed');
        fs.copyFileSync(source, target); if (controls.corruptCopy) fs.writeFileSync(target, 'corrupt fixture');
    }, lstatSync(file: string) { const stat = fs.lstatSync(file); if (controls.registrySymlink && file === obsolete[0]) return {...stat, isSymbolicLink:()=>true}; return stat; }, realpathSync(file: string) { return controls.redirected && file.startsWith(native) ? path.join(root, 'redirected') : fs.realpathSync(file); }};
    const run: any = (executable: string, args: string[], options: any) => {
        const name = args[args.indexOf('--package') + 1], dry = args.includes('--dry-run');
        calls.push({executable, args, options, dry}); events.push((dry ? 'dry ' : 'clean ') + name);
        assert.equal(executable, cargo); assert(packages.includes(name)); assert(args.includes('--locked') && args.includes('--offline'));
        assert(declaredPackages.includes(name), 'Fake Cargo rejects absent package: ' + name);
        assert.equal(args[args.indexOf('--manifest-path') + 1], path.join(workbench, 'Cargo.toml'));
        assert.equal(args[args.indexOf('--target-dir') + 1], native); assert.equal(options.shell, false);
        assert.equal(fs.readFileSync(path.join(folder, 'sample.pb'), 'utf8'), 'generated fixture');
        const file = path.join(native, 'debug/deps', name + '.rlib');
        if (dry) return {status: 0, stdout: controls.output ?? (fs.existsSync(file) ? file + '\n' : ''), stderr: ''};
        const intent = JSON.parse(fs.readFileSync(path.join(folder, 'native-sample-cache-retirement.json'), 'utf8'));
        assert.equal(intent.status, 'CLEANING_NATIVE_LOCAL_OUTPUTS_AFTER_VALIDATED_SAMPLE_COPY');
        if (controls.cleanFails) return {status: 1, stdout: '', stderr: 'clean failed'};
        fs.unlinkSync(file); if (controls.driftRegistry) put(obsolete[0], 'drifted registry'); if (controls.changeLock) put(path.join(workbench, 'Cargo.lock'), 'changed lock');
        return {status: 0, stdout: '', stderr: 'retired local package'};
    };
    const execute = (label: string, args: string[]) => {
        events.push(label); assert.equal(label, 'sample'); assert.deepEqual(args, ['cargo', 'run', '--example', 'sample', '--locked', '--target', host, '-j8', ...(lowDisk ? ['--message-format=json-render-diagnostics'] : [])]);
        if (controls.sampleFails) throw Error('sample failed'); put(path.join(workbench, 'sample.pb'), 'generated fixture');
        const log = path.join(folder, 'sample.log'); put(log, controls.emptyLog ? '' : artifactRecords.map(row=>JSON.stringify(row)).join('\n')+'\n');
        return {log,logSha256:sha(fs.readFileSync(log)),resources:{exitCode:0,timedOut:false}};
    };
    const check = () => assert.equal(fs.readFileSync(path.join(workbench, 'src/lib.rs'), 'utf8'), 'fixture source');
    const phase = () => nativeSamplePhase(viewer, p, () => host, check, execute, {fs: io, run});
    const pipeline = () => { const result = phase(); events.push('wasm'); return result; };
    return {root, viewer, folder, workbench, cache, native, p, controls, events, calls, phase, pipeline, current, obsolete, artifactRecords, put,
        cleanup() { assert(root.startsWith(path.join(os.tmpdir(), 'course-low-disk-'))); fs.rmSync(root, {recursive: true, force: true}); },
        unchanged() { for (const [file, bytes] of before) assert.deepEqual(fs.readFileSync(file), bytes); }};
}

test('default and explicit0 preserve WASM incremental1; only exact1 enables low disk', () => {
    for (const value of [undefined, '', '0', 'true']) assert.deepEqual(imageBuildMode({COURSE_IMAGE_LOW_DISK: value}), {lowDisk: false, nativeIncremental: '0', wasmIncremental: '1'});
    assert.deepEqual(imageBuildMode({COURSE_IMAGE_LOW_DISK: '1'}), {lowDisk: true, nativeIncremental: '0', wasmIncremental: '0'});
});

test('validated fixture copy precedes only four bounded Native cleans and the WASM phase', () => {
    const f = fixture();
    try {
        const result = f.pipeline(); assert(result); assert.equal(f.calls.length, 12);
        assert.deepEqual(f.calls.slice(0, 4).map(call => call.args.at(-3)), packages);
        assert(f.calls.slice(0, 4).every(call => call.dry));
        assert(f.events.indexOf('copy') < f.events.indexOf('clean image_incremental')); assert.equal(f.events.at(-1), 'wasm');
        const report = JSON.parse(fs.readFileSync(result.path, 'utf8'));
        assert.equal(report.status, 'NATIVE_LOCAL_OUTPUTS_RETIRED_SAMPLE_AND_SOURCE_UNCHANGED');
        assert.deepEqual(report.skippedPackages, []);
        assert.equal(report.commands.length, 4); assert.equal(report.attempts.length, 12); assert.equal(sha(fs.readFileSync(result.path)), result.sha256);
        assert.equal(report.sample.sha256, sha('generated fixture')); f.unchanged();
    } finally { f.cleanup(); }
});

test('flag0 follows the original sample/copy path without cleanup or a retirement receipt', () => {
    const f = fixture(false);
    try { assert.equal(f.pipeline(), null); assert.deepEqual(f.events, ['sample', 'copy', 'wasm']); assert.equal(f.calls.length, 0); f.unchanged(); }
    finally { f.cleanup(); }
});

for (const failure of ['sampleFails', 'copyFails', 'corruptCopy'] as const) test(failure + ' prevents any native retirement and the WASM phase', () => {
    const f = fixture();
    try { f.controls[failure] = true; assert.throws(f.pipeline); assert.equal(f.calls.length, 0); assert(!f.events.includes('wasm')); f.unchanged(); }
    finally { f.cleanup(); }
});

test('a source-only checkpoint without a sample does not clean caches or resolve a native target', () => {
    const f = fixture();
    try {
        fs.unlinkSync(path.join(f.workbench, 'examples/sample.rs'));
        assert.equal(nativeSamplePhase(f.viewer, f.p, () => { throw Error('unexpected target lookup'); }, () => {}, () => { throw Error('unexpected sample'); }), null);
        assert.equal(f.calls.length, 0); f.unchanged();
    } finally { f.cleanup(); }
});

for (const boundary of ['wasm32-unknown-unknown/debug/deps/escape.rlib', '../outside.rlib']) test('rejects dry-run deletion outside Native debug: ' + boundary, () => {
    const f = fixture();
    try {
        f.controls.output = path.resolve(f.cache, boundary) + '\n'; assert.throws(f.pipeline, /Native clean escapes/);
        assert(f.calls.every(call => call.dry)); assert(!f.events.includes('wasm')); f.unchanged();
    } finally { f.cleanup(); }
});

for (const failure of ['cleanFails', 'changeLock'] as const) test(failure + ' preserves failure evidence and blocks WASM', () => {
    const f = fixture();
    try {
        f.controls[failure] = true; assert.throws(f.pipeline); assert(!f.events.includes('wasm'));
        const report = JSON.parse(fs.readFileSync(path.join(f.folder, 'native-sample-cache-retirement.json'), 'utf8'));
        assert.equal(report.status, 'FAILED_WASM_BUILD_BLOCKED'); assert(report.attempts.length >= 6); f.unchanged();
    } finally { f.cleanup(); }
});

test('a redirected native cache is refused before any Cargo cleanup', () => {
    const f = fixture();
    try { f.controls.redirected = true; assert.throws(f.pipeline, /redirected/); assert.equal(f.calls.length, 0); assert(!f.events.includes('wasm')); f.unchanged(); }
    finally { f.cleanup(); }
});

test('a substituted shared-cache root cannot select foreign cache outputs', () => {
    const f = fixture();
    try { f.p.group.target = path.join(f.root, 'foreign-cache'); assert.throws(f.pipeline, /Only the shared/); assert.equal(f.calls.length, 0); assert(!f.events.includes('wasm')); f.unchanged(); }
    finally { f.cleanup(); }
});

test('native retirement is bound to the compiler host recorded in the preparation', () => {
    const f = fixture();
    try { f.p.tools.rustc = 'rustc fixture\nhost: aarch64-pc-windows-msvc\n'; assert.throws(f.pipeline, /recorded compiler host/); assert.equal(f.calls.length, 0); assert(!f.events.includes('wasm')); f.unchanged(); }
    finally { f.cleanup(); }
});

test('actual case/group and batch environment expressions bind the low-disk mode', () => {
    const source = fs.readFileSync(path.join(here, 'incremental-browser-image-build.ts'), 'utf8');
    const batch = fs.readFileSync(path.join(here, 'compact-capture-batch.ts'), 'utf8');
    const caseExpression = source.match(/identity=(\{source,density,buildMode,[^\r\n]+\});/)![1].replaceAll('import.meta.url', JSON.stringify('file:///fixture/tool.ts'));
    const groupExpression = source.match(/const groupIdentity=([^\r\n]+);/)![1];
    const environmentExpression = batch.match(/const environment = Object\.fromEntries\(([\s\S]*?)\);/)![1];
    const identities = [false, true].map(lowDisk => {
        const environment = lowDisk ? {COURSE_IMAGE_LOW_DISK: '1'} : {};
        const buildMode = imageBuildMode(environment), buildPolicy = {incremental: {nativeSample: '0', wasm: buildMode.wasmIncremental}};
        const context: any = {source: {dependencies: {}, kernel: 'kernel'}, density: 2, buildMode, buildPolicy,
            modeHelper: 'helper.ts', self: 'build.ts', guard: 'guard.ps1', sha: (text: string) => sha(text),
            fileURLToPath: (value: URL) => value.pathname, URL, imageCourseSnapshot: () => null, compilerTools: () => ({}), tools: () => ({}),
            process: {env: environment}, id: 'fixture', hash: sha, expected: () => ({'Cargo.toml': 'manifest'}), lockFor: () => 'lock'};
        context.identity = vm.runInNewContext('(' + caseExpression + ')', context);
        const group = vm.runInNewContext('(' + groupExpression + ')', context);
        const profile = vm.runInNewContext('Object.fromEntries(' + environmentExpression + ')', {process: {env: environment}, nativeDevelopment: {}});
        assert.equal(context.identity.buildMode.lowDisk, lowDisk); assert.equal(group.buildMode.lowDisk, lowDisk);
        assert.equal(profile.COURSE_IMAGE_LOW_DISK, lowDisk ? '1' : null);
        return [context.identity, group, profile].map(value => sha(JSON.stringify(value)));
    });
    assert(identities[0].every((value, index) => value !== identities[1][index]));
    assert(source.indexOf('nativeSamplePhase(viewer,p,nativeTarget,check,execute)') < source.indexOf("const wasmCommand=execute('wasm'"));
});


test('early two-package graph never asks Cargo to clean absent optional packages', () => {
    const f = fixture(true, ['image_incremental', 'session_rust']);
    try {
        const result = f.pipeline(); assert(result);
        const names = f.calls.map(call => call.args[call.args.indexOf('--package') + 1]);
        assert.deepEqual(names, ['image_incremental', 'session_rust', 'image_incremental', 'image_incremental', 'session_rust', 'session_rust']);
        const report = JSON.parse(fs.readFileSync(result.path, 'utf8'));
        assert.deepEqual(report.skippedPackages, ['wood', 'session_viewer']);
        assert.deepEqual(report.commands.map((call: any) => call.arguments.at(-1)), ['image_incremental', 'session_rust']);
        for (const name of ['wood', 'session_viewer']) assert.equal(fs.readFileSync(path.join(f.native, 'debug/deps', name + '.rlib'), 'utf8'), 'local ' + name);
        assert.equal(f.events.at(-1), 'wasm'); f.unchanged();
    } finally { f.cleanup(); }
});

for (const lock of [
    'invalid lock',
    'version = 4\n[[package]]\nversion = "0.1.0"\n',
    'version = 4\n[[package]]\nname = "session_rust"\nversion = "0.1.0"\n',
    'version = 4\n[[package]]\nname = "image_incremental"\nversion = "0.1.0"\nsource = "registry+fixture"\n',
    'version = 4\n[[package]]\nname = "image_incremental"\nversion = "0.1.0"\n[[package]]\nname = "image_incremental"\nversion = "0.2.0"\n',
]) test('invalid or ambiguous pinned package declarations prevent all native cleanup: ' + JSON.stringify(lock), () => {
    const f = fixture();
    try {
        fs.writeFileSync(path.join(f.workbench, 'Cargo.lock'), lock);
        f.p.overlay.lockSha256 = sha(lock);
        assert.throws(f.pipeline, /Cargo lock|local package/);
        assert.equal(f.calls.length, 0); assert(!f.events.includes('wasm')); f.unchanged();
    } finally { f.cleanup(); }
});


test('successful fresh/cached Cargo closure keeps current Native artifacts and retires only obsolete registry siblings', () => {
    const f=fixture();
    try {
        const binary=path.join(f.native,'debug/deps/regcrate-bbbbbbbbbbbbbbbb.dll');f.put(binary,'runtime binary');
        const unrelated=path.join(f.native,'debug/deps/local-cccccccccccccccc.d');f.put(unrelated,unrelated+': '+path.join(f.workbench,'src/lib.rs')+'\n');
        const result=f.pipeline()!;const report=JSON.parse(fs.readFileSync(result.path,'utf8'));
        assert.deepEqual(report.registry.current.map((row:any)=>row.path).sort(),[...f.current].sort());
        assert.deepEqual(report.registry.retired.sort(),[...f.obsolete].sort());
        for(const file of f.obsolete)assert(!fs.existsSync(file));
        assert.equal(fs.readFileSync(binary,'utf8'),'runtime binary');assert(fs.existsSync(unrelated));f.unchanged();
        assert.equal(report.registry.artifactRecords,2);
    }finally{f.cleanup();}
});

for(const kind of ['empty','missing-own','missing-finished','failed-finished','missing-current','relative-artifact'] as const)
test('incomplete successful compiler closure blocks all cleanup: '+kind,()=>{
    const f=fixture();
    try{
        if(kind==='empty')f.controls.emptyLog=true;
        if(kind==='missing-own')f.artifactRecords.splice(1,1);
        if(kind==='missing-finished')f.artifactRecords.splice(2,1);
        if(kind==='failed-finished')f.artifactRecords[2].success=false;
        if(kind==='missing-current')fs.unlinkSync(f.current[0]);
        if(kind==='relative-artifact')f.artifactRecords[0].filenames=['../escape.rlib'];
        assert.throws(f.pipeline);assert.equal(f.calls.length,0);assert(!f.events.includes('wasm'));
        for(const file of f.obsolete)assert(fs.existsSync(file));
    }finally{f.cleanup();}
});

test('registry symlink is rejected before Cargo cleanup or unlink',()=>{
    const f=fixture();
    try{f.controls.registrySymlink=true;assert.throws(f.pipeline,/links/);assert.equal(f.calls.length,0);for(const file of f.obsolete)assert(fs.existsSync(file));f.unchanged();}
    finally{f.cleanup();}
});

test('planned artifact drift blocks registry unlink and records failure before WASM',()=>{
    const f=fixture();
    try{
        f.controls.driftRegistry=true;assert.throws(f.pipeline,/changed after planning/);assert(!f.events.includes('wasm'));
        for(const file of f.obsolete)assert(fs.existsSync(file));f.unchanged();
        const report=JSON.parse(fs.readFileSync(path.join(f.folder,'native-sample-cache-retirement.json'),'utf8'));
        assert.equal(report.status,'FAILED_WASM_BUILD_BLOCKED');assert.deepEqual(report.registry.retired,[]);
    }finally{f.cleanup();}
});

test('registry-like traversal or foreign dep-info never authorizes removal',()=>{
    const f=fixture();
    try{
        const foreign=path.join(f.root,'foreign.rlib');f.put(foreign,'foreign bytes');
        f.put(f.obsolete[2],f.obsolete[2]+': '+path.join(f.root,'.cargo/registry/src/test-index')+'/../foreign-1.0.0/src/lib.rs\n');
        const result=f.pipeline()!;const report=JSON.parse(fs.readFileSync(result.path,'utf8'));
        assert.deepEqual(report.registry.obsolete,[]);for(const file of f.obsolete)assert(fs.existsSync(file));
        assert.equal(fs.readFileSync(foreign,'utf8'),'foreign bytes');f.unchanged();
    }finally{f.cleanup();}
});

test('another process workbench ownership blocks all cleanup',()=>{
    const f=fixture();
    try{
        f.put(path.join(path.dirname(f.workbench),'workbench.lock'),JSON.stringify({pid:process.pid+1}));
        assert.throws(f.pipeline,/own the active/);assert.equal(f.calls.length,0);for(const file of f.obsolete)assert(fs.existsSync(file));f.unchanged();
    }finally{f.cleanup();}
});

test('a registry package absent from the current lock may retire only its old Native libraries',()=>{
    const f=fixture();
    try{
        const registry=path.join(f.root,'.cargo/registry/src/test-index/absent-crate-0.9.1');
        f.put(path.join(registry,'Cargo.toml'),'old registry package');f.put(path.join(registry,'src/lib.rs'),'old source');
        const stem=path.join(f.native,'debug/deps/absent_crate-cccccccccccccccc');
        const dep=stem+'.d',lib=path.join(path.dirname(stem),'lib'+path.basename(stem)+'.rlib');
        f.put(dep,dep+': '+path.join(registry,'src/lib.rs')+'\n');f.put(lib,'obsolete library');
        f.pipeline();assert(!fs.existsSync(dep));assert(!fs.existsSync(lib));f.unchanged();
    }finally{f.cleanup();}
});

test('binary-only dep-info stays untouched even for an obsolete registry version',()=>{
    const f=fixture();
    try{
        fs.unlinkSync(f.obsolete[0]);fs.unlinkSync(f.obsolete[1]);
        const result=f.pipeline()!;const report=JSON.parse(fs.readFileSync(result.path,'utf8'));
        assert.deepEqual(report.registry.obsolete,[]);assert(fs.existsSync(f.obsolete[2]));f.unchanged();
    }finally{f.cleanup();}
});
