import {verifyNativeScale} from './image-render-scale.ts';
import {cargoTarget, observeExecution, finishExecution, compilerTools, fileReference} from './capture-storage.ts';
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {spawnSync} from 'node:child_process';
import {bindings, nativeAdapterProfile, verifyNativeAdapter, prepare as prepareSnapshot} from './presentation.ts';
import {assetsFor, expected, filesIn, hash, json, read, viewer, write} from './model.ts';
import {nativeTarget} from './verify.ts';
import {png, ppm} from './png.ts';

const script = fileURLToPath(import.meta.url);
const workbench = path.join(viewer, 'target/course-incremental-presentation-workbench');
const sha = (file: string) => hash(fs.readFileSync(file));
const ownerName = '.presentation-workbench-owner.json';
const ownedName = '.presentation-workbench-inputs.json';
const ownership = {owner: 'incremental-native-presentation', workbench, viewer};

export function capturePolicy() {
    const processors = Number(process.env.COURSE_CAPTURE_PROCESSORS || 2);
    assert([2, 8].includes(processors), 'Capture processors must be 2 or 8');
    const imageLane = process.env.COURSE_CAPTURE_IMAGE_LANE || '0';
    assert(['0', '1'].includes(imageLane), 'Capture image lane must be 0 or 1');
    const compileMemory = process.env.COURSE_NATIVE_COMPILE_MEMORY_MIB;
    assert(compileMemory === undefined || compileMemory === '4096', 'Native compile memory override must be 4096 MiB');
    if (compileMemory !== undefined) assert(processors === 2 && imageLane === '1', '4 GiB native compilation requires the two-processor image lane');
    const guard = fileURLToPath(new URL('capture-guard.ps1', import.meta.url));
    return {processors, memoryMiB: compileMemory === '4096' ? 4096 : processors === 8 ? 8192 : 6144, jobs: processors === 8 ? 8 : 1,
        guard, guardSha256: sha(guard), ...(imageLane === '1' ? {imageLane: true} : {}),
        ...(compileMemory === '4096' ? {minimumAvailableMemoryMiB: 5120} : {})};
}

export function compiledDrawPolicy(memoryMiB = 2048) {
    assert.equal(memoryMiB, 2048, 'Compiled draw resume is qualified for a 2 GiB execution limit');
    return {...capturePolicy(), processors: 2, memoryMiB, jobs: 1, imageLane: true, minimumAvailableMemoryMiB: 3072};
}

function runCaptureCommand(name: string, args: string[], directory: string, extra: Record<string, string> = {}, policy = capturePolicy()) {
    const evidence = path.join(viewer, 'target/course-checks');
    const development = Object.fromEntries(Object.entries(nativeDevelopmentProfile()).filter((entry): entry is [string, string] => entry[1] !== null));
    assert.equal(process.platform, 'win32', 'This image-only optional guard is Windows-specific');
    fs.mkdirSync(evidence, {recursive: true});
    const log = path.join(evidence, name + '.log'); assert(!fs.existsSync(log));
    const command = path.join(evidence, name + '.command.json');
    write(command, JSON.stringify({executable: args[0], arguments: args.slice(1)}) + '\n');
    const descriptor = fs.openSync(log, 'wx');
    let result;
    try {
        result = spawnSync('powershell.exe', ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', policy.guard,
            '-ProcessorCount', String(policy.processors), '-MemoryLimitMiB', String(policy.memoryMiB),
            ...(policy.imageLane ? ['-ImageLane'] : []),
            '-TimeoutSeconds', '1800', '-ManifestPath', path.join(evidence, name + '.resources.json'),
            '-WorkingDirectory', directory, '-CommandFile', command], {cwd: directory, windowsHide: true,
            env: {...process.env, RUSTC_WRAPPER: '', CARGO_NET_OFFLINE: 'true', NO_COLOR: 'true',
                CARGO_BUILD_JOBS: String(policy.jobs), RUST_TEST_THREADS: '1', REGEN_PROTO: '0',
                CARGO_TARGET_DIR: cargoTarget(), ...development, ...extra},
            stdio: ['ignore', descriptor, descriptor]});
    } finally { fs.closeSync(descriptor); }
    if (result.error || result.status !== 0) throw Error(`${name} failed. Read ${log}\n${read(log).slice(-4000)}`);
    return {name, args, environment: development, returncode: result.status};
}

function ownedPath(name: string) {
    assert(!path.isAbsolute(name));
    const file = path.resolve(workbench, name), relative = path.relative(workbench, file);
    assert(relative && relative !== '..' && !relative.startsWith('..' + path.sep) && !path.isAbsolute(relative), name);
    let cursor = workbench;
    for (const part of relative.split(path.sep)) {
        cursor = path.join(cursor, part);
        if (fs.existsSync(cursor)) assert(!fs.lstatSync(cursor).isSymbolicLink(), `Owned workbench cannot contain links: ${cursor}`);
    }
    return file;
}

function ensureWorkbench() {
    if (!fs.existsSync(workbench)) {
        fs.mkdirSync(workbench);
        write(path.join(workbench, ownerName), JSON.stringify(ownership, null, 2) + '\n');
    }
    assert.equal(fs.realpathSync(workbench), path.resolve(workbench));
    assert.deepEqual(json(path.join(workbench, ownerName)), ownership, 'Only this private workbench is owned');
}

export function nativeDevelopmentProfile(environment: NodeJS.ProcessEnv = process.env) {
    const names = new Set(['DEBUG', 'OPT_LEVEL', 'INCREMENTAL', 'CODEGEN_UNITS', 'STRIP'].map(name => 'CARGO_PROFILE_DEV_' + name));
    for (const name of Object.keys(environment)) if (name.startsWith('CARGO_PROFILE_DEV_')) names.add(name);
    return Object.fromEntries([...names].sort().map(name => [name, environment[name] || (name === 'CARGO_PROFILE_DEV_DEBUG' ? '0' : null)]));
}

function compilerProfile() {
    const rustc = spawnSync('rustc', ['-vV'], {encoding: 'utf8', windowsHide: true});
    const cargo = spawnSync('cargo', ['--version'], {encoding: 'utf8', windowsHide: true});
    assert.equal(rustc.status, 0); assert.equal(cargo.status, 0);
    const dxc = process.env.VIEWER_DXC_PATH;
    return {toolFiles: compilerTools(), host: nativeTarget(), rustc: rustc.stdout.trim(), cargo: cargo.stdout.trim(),
        capture: capturePolicy(), development: nativeDevelopmentProfile(), nativeAdapter: nativeAdapterProfile(), CARGO_INCREMENTAL: '1', CARGO_BUILD_JOBS: String(capturePolicy().jobs), CARGO_TARGET_DIR: cargoTarget(),
        RUSTFLAGS: process.env.RUSTFLAGS || '', CARGO_ENCODED_RUSTFLAGS: process.env.CARGO_ENCODED_RUSTFLAGS || '',
        dxc: dxc && fs.existsSync(dxc) ? {path: path.resolve(dxc), sha256: sha(dxc), dxilSha256: sha(path.join(path.dirname(dxc), 'dxil.dll'))} : null};
}

function snapshotInputs(snapshot: any) {
    const names = new Set([...Object.keys(expected(snapshot.source.checkpoint)), 'Cargo.lock', ...assetsFor(snapshot.source.checkpoint)]);
    const driver = path.relative(snapshot.directory, path.join(viewer, snapshot.driver.path)).split(path.sep).join('/');
    names.add(driver);
    if (snapshot.adapterBridge) for (const file of Object.values(snapshot.adapterBridge) as any[]) names.add(path.relative(snapshot.directory, file.path));
    if (snapshot.nativeDensityBridge) for (const file of Object.values(snapshot.nativeDensityBridge) as any[]) names.add(path.relative(snapshot.directory, file.path));
    if (snapshot.renderScaleBridge) for (const file of Object.values(snapshot.renderScaleBridge) as any[]) names.add(path.relative(snapshot.directory, file.path));
    return Object.fromEntries([...names].sort().map(name => [name, {snapshot: path.join(snapshot.directory, name), sha256: sha(path.join(snapshot.directory, name))}]));
}

function checkSnapshot(record: any) {
    if (sha(script) !== record.helperSha256) {
        const legacy = record.helperSha256;
        assert(['a1b6e47c0aea300df5b23cd7e2738e9bf00b4d2804d9fd4493344ab8d2d0f266',
            '9616534822d36da3e0f9f3341566e56f4dbad0265f00eeaf52a6d8f900044100'].includes(legacy), 'Unknown historical incremental helper');
        assert.equal(sha(path.join(viewer, 'target/course-presentation/helper-snapshots', legacy, 'incremental-native-presentation.ts')), legacy);
    }
    if (record.profile.capture) assert.equal(sha(record.profile.capture.guard), record.profile.capture.guardSha256, 'Capture guard changed');
    assert.deepEqual(bindings(record.source.checkpoint), record.source, 'Displayed checkpoint or dependencies changed');
    for (const [name, input] of Object.entries(record.inputs) as [string, any][]) assert.equal(sha(input.snapshot), input.sha256, name);
}

function checkWorkbench(inputs: any) {
    ensureWorkbench();
    const actual = filesIn(workbench).map(file => path.relative(workbench, file).split(path.sep).join('/'))
        .filter(name => ![ownerName, ownedName].includes(name)).sort();
    assert.deepEqual(actual, Object.keys(inputs).sort(), 'Workbench contains unexpected files');
    for (const [name, input] of Object.entries(inputs) as [string, any][]) {
        const file = ownedPath(name); assert(fs.lstatSync(file).isFile()); assert.equal(sha(file), input.sha256, name);
    }
}

function synchronize(inputs: any) {
    ensureWorkbench();
    const manifest = path.join(workbench, ownedName), previous = fs.existsSync(manifest) ? json(manifest) : {};
    const existing = filesIn(workbench).map(file => path.relative(workbench, file).split(path.sep).join('/'))
        .filter(name => ![ownerName, ownedName].includes(name));
    assert(existing.every(name => Object.hasOwn(previous, name)), 'Refuse to overwrite an unowned workbench file');
    for (const [name, old] of Object.entries(previous) as [string, any][]) {
        const file = ownedPath(name); assert.equal(sha(file), old.sha256, 'Previously owned input changed: ' + name);
        if (!Object.hasOwn(inputs, name)) fs.unlinkSync(file);
    }
    let changed = 0, unchanged = 0, removed = 0;
    for (const name of Object.keys(previous)) if (!Object.hasOwn(inputs, name)) removed++;
    for (const [name, input] of Object.entries(inputs) as [string, any][]) {
        const destination = ownedPath(name);
        if (previous[name]?.sha256 === input.sha256) { unchanged++; continue; }
        assert.equal(sha(input.snapshot), input.sha256);
        write(destination, fs.readFileSync(input.snapshot)); changed++;
    }
    write(manifest, JSON.stringify(inputs, null, 2) + '\n'); checkWorkbench(inputs);
    return {changed, unchanged, removed};
}

export function prepareIncremental(id: string, density = 2, drawMemoryMiB?: number) {
    const snapshot = prepareSnapshot(id, density), profile = compilerProfile(), helperSha256 = sha(script);
    const execution = drawMemoryMiB === undefined ? {} : {executionPolicy: compiledDrawPolicy(drawMemoryMiB)};
    const key = hash(JSON.stringify({snapshotKey: snapshot.key, density, profile, helperSha256, ...execution}));
    const folder = path.join(viewer, 'target/course-presentation/incremental-native', id + '-' + key.slice(0, 20));
    const recordFile = path.join(folder, 'result.json');
    if (fs.existsSync(recordFile)) { const existing = json(recordFile); checkSnapshot(existing); return existing; }
    const record = {status: 'PREPARED_NOT_EXECUTED', id, density, key, source: snapshot.source, snapshotKey: snapshot.key, ...execution,
        ...(snapshot.nativeDensityBridge ? {nativeDensityBridge: snapshot.nativeDensityBridge} : {}),
        snapshotDirectory: snapshot.directory, snapshotDriver: snapshot.driver, adapterBridge: snapshot.adapterBridge, renderScaleBridge: snapshot.renderScaleBridge, profile, helperSha256,
        inputs: snapshotInputs(snapshot), folder, recordFile, workbench, width: snapshot.width, height: snapshot.height,
        scope: 'Exact keyed snapshot preserved; stable private compiler source path; every checkpoint still receives its own Cargo invocation and actual renderer execution.'};
    write(recordFile, JSON.stringify(record, null, 2) + '\n'); checkSnapshot(record); return record;
}

export function verifyIncremental(recordFile: string) {
    const record = json(recordFile); checkSnapshot(record);
    assert.equal(record.status, 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW');
    assert.equal(sha(record.executable.path), record.executable.sha256);
    assert.equal(sha(record.image.path), record.image.sha256);
    assert.equal(sha(record.compilerArtifact.path), record.compilerArtifact.sha256);
    assert.equal(sha(record.compile.log), record.compile.logSha256); assert.equal(sha(record.execute.log), record.execute.logSha256);
    verifyExecutionResources(record.execute.resources, record.executionPolicy || record.profile.capture);
    if (record.compiledOrigin) {
        const origin = record.compiledOrigin;
        assert.equal(sha(origin.path), origin.sha256); assert.equal(sha(origin.helper.path), origin.helper.sha256);
        for (const file of origin.evidence) assert.equal(sha(file.path), file.sha256);
        const compiled = json(origin.path);
        for (const name of ['source', 'inputs', 'profile', 'compilerArtifact', 'executable', 'compile', 'snapshotKey', 'density', 'width', 'height'])
            assert.deepEqual(record[name], compiled[name], 'Compiled origin differs: ' + name);
        assert.equal(compiled.helperSha256, origin.helper.sha256);
    }
    assert.deepEqual(verifyNativeAdapter(path.join(record.folder, 'runtime'), record.profile.nativeAdapter), record.adapterReceipt || null);
    assert.deepEqual(verifyNativeScale(path.join(record.folder, 'runtime'), record.renderScaleBridge), record.renderScaleReceipt || null);
    return record;
}

export function verifyExecutionResources(resources: any, policy: ReturnType<typeof capturePolicy> & {minimumAvailableMemoryMiB?: number}) {
    assert.equal(resources.exitCode, 0); assert.equal(resources.timedOut, false);
    assert.equal(resources.processors, policy.processors); assert.equal(resources.jobCommitLimitBytes, policy.memoryMiB * 1048576);
    if (policy.imageLane) assert.equal(resources.laneMutex, 'Local\\SessionViewerCourseImageJob');
    if (policy.minimumAvailableMemoryMiB) {
        assert.equal(resources.minimumAvailableMemoryMiB, policy.minimumAvailableMemoryMiB);
        assert(resources.availableMemoryMiBBeforeLaunch >= policy.minimumAvailableMemoryMiB);
    }
}

export function verifyCompiledArtifact(record: any) {
    assert.equal(record.status, 'FAILED_PRESERVED');
    for (const name of ['image', 'execute', 'executionIntegrity', 'adapterReceipt', 'renderScaleReceipt', 'compiledOrigin'])
        assert(!record[name], 'Resume requires compilation followed by admission refusal, without a previous draw: ' + name);
    assert.equal(record.compile.command.returncode, 0);
    assert.equal(sha(record.compile.log), record.compile.logSha256);
    assert.equal(sha(record.compilerArtifact.path), record.compilerArtifact.sha256);
    assert.equal(sha(record.executable.path), record.executable.sha256);
    assert.equal(sha(record.executable.compilerReportedPath), record.executable.sha256, 'Shared compiler artifact was replaced');
    const messages = read(record.compile.log).split(/\r?\n/).flatMap(line => { try { return [JSON.parse(line)]; } catch { return []; } });
    assert.deepEqual(messages.filter(message => message.reason === 'build-finished'), [{reason: 'build-finished', success: true}]);
    const artifacts = messages.filter(message => message.reason === 'compiler-artifact' && message.target?.name === 'presentation'
        && message.target?.kind?.includes('example') && message.executable);
    assert.equal(artifacts.length, 1);
    const artifact = json(record.compilerArtifact.path); assert.deepEqual(artifacts[0], artifact);
    assert.equal(path.resolve(artifact.manifest_path), path.join(record.workbench, 'Cargo.toml'));
    assert.equal(path.resolve(artifact.target.src_path), path.join(record.workbench, 'examples/presentation.rs'));
    assert.equal(artifact.executable, record.executable.compilerReportedPath);
    assert(artifact.filenames.includes(artifact.executable));
    const prefix = record.compile.log.replace(/-build\.log$/, ''); assert.notEqual(prefix, record.compile.log);
    const command = record.compile.command.args;
    assert.deepEqual(command, ['cargo', 'build', '--example', 'presentation', '--locked', '--offline', '--target', record.profile.host,
        '-j' + record.profile.capture.jobs, '--message-format=json-render-diagnostics']);
    assert.deepEqual(json(prefix + '-build.command.json'), {executable: command[0], arguments: command.slice(1)});
    assert.deepEqual(json(prefix + '-build.resources.json'), record.compile.resources);
    assert.deepEqual(record.compile.resources.arguments, command.slice(1));
    assert.equal(record.compile.resources.workingDirectory, record.workbench);
    verifyExecutionResources(record.compile.resources, record.profile.capture);
    assert(!fs.existsSync(prefix + '-execute.resources.json'), 'A renderer process already received an execution receipt');
    assert.match(read(prefix + '-execute.log'), /Capture admission requires \d+ MiB available; found \d+ MiB\./);
    assert.deepEqual(json(prefix + '-execute.command.json'), {executable: record.executable.path, arguments: []});
    const runtime = path.join(record.folder, 'runtime');
    assert(!filesIn(record.folder).some(file => /\.(png|ppm)$/i.test(file) || ['frame-adapter.json', 'frame-render-scale.json'].includes(path.basename(file))), 'Previous draw output exists');
    for (const [name, input] of Object.entries(record.inputs) as [string, any][])
        assert.equal(sha(path.join(runtime, name)), input.sha256, 'Failed runtime changed: ' + name);
    return ['-build.log', '-build.command.json', '-build.resources.json', '-execute.log', '-execute.command.json'].map(suffix => fileReference(prefix + suffix));
}

export function verifyCompiledNative(recordFile: string) {
    const record = json(recordFile);
    assert.equal(path.resolve(recordFile), record.recordFile);
    assert.equal(record.workbench, workbench);
    assert.equal(record.key, hash(JSON.stringify({snapshotKey: record.snapshotKey, density: record.density, profile: record.profile, helperSha256: record.helperSha256})));
    assert.equal(record.folder, path.join(viewer, 'target/course-presentation/incremental-native', record.id + '-' + record.key.slice(0, 20)));
    assert.equal(record.recordFile, path.join(record.folder, 'result.json'));
    checkSnapshot(record); checkWorkbench(record.inputs);
    assert.deepEqual(json(path.join(workbench, ownedName)), record.inputs, 'Workbench ownership manifest differs');
    const snapshot = json(path.join(record.snapshotDirectory, 'presentation.json'));
    assert.equal(snapshot.key, record.snapshotKey); assert.deepEqual(snapshot.source, record.source);
    assert.deepEqual(snapshotInputs(snapshot), record.inputs);
    assert.deepEqual(snapshot.driver, record.snapshotDriver);
    for (const name of ['width', 'height', 'density', 'adapterBridge', 'renderScaleBridge']) assert.deepEqual(record[name], snapshot[name]);
    assert.equal(sha(path.join(viewer, record.snapshotDriver.path)), record.snapshotDriver.sha256);
    assert.deepEqual(compilerTools(), record.profile.toolFiles);
    assert.deepEqual(nativeDevelopmentProfile(), record.profile.development);
    assert.deepEqual(capturePolicy(), record.profile.capture);
    assert.deepEqual(nativeAdapterProfile(), record.profile.nativeAdapter);
    assert.equal(cargoTarget(), record.profile.CARGO_TARGET_DIR);
    for (const name of ['RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS']) assert.equal(process.env[name] || '', record.profile[name]);
    assert.equal(path.resolve(process.env.VIEWER_DXC_PATH || ''), record.profile.dxc.path);
    assert.equal(sha(record.profile.dxc.path), record.profile.dxc.sha256);
    assert.equal(sha(path.join(path.dirname(record.profile.dxc.path), 'dxil.dll')), record.profile.dxc.dxilSha256);
    const evidence = verifyCompiledArtifact(record);
    return {record, evidence};
}

export function resumeCompiledNative(recordFile: string, drawMemoryMiB = 2048) {
    const policy = compiledDrawPolicy(drawMemoryMiB), {record: compiled, evidence} = verifyCompiledNative(recordFile);
    const helper = path.join(viewer, 'target/course-presentation/helper-snapshots', compiled.helperSha256, 'incremental-native-presentation.ts');
    assert.equal(sha(helper), compiled.helperSha256, 'Exact compiled-origin helper must be preserved');
    const compiledOrigin = {...fileReference(recordFile), helper: fileReference(helper), evidence};
    const helperSha256 = sha(script), key = hash(JSON.stringify({compiledOrigin, helperSha256, policy, attempt: process.env.COURSE_CAPTURE_ATTEMPT || null}));
    const folder = path.join(viewer, 'target/course-presentation/incremental-native', compiled.id + '-draw-' + key.slice(0, 20));
    assert(!fs.existsSync(folder), 'Preserve the previous compiled-draw attempt; select a new COURSE_CAPTURE_ATTEMPT');
    const record: any = {...compiled, status: 'PREPARED_COMPILED_DRAW', key, folder, recordFile: path.join(folder, 'result.json'),
        helperSha256, compiledOrigin, executionPolicy: policy,
        scope: 'Exact source and workbench verified against preserved successful compilation; fresh guarded renderer execution with explicit draw resources.'};
    delete record.error;
    const lock = path.join(viewer, 'target/course-presentation/incremental-native-running.lock');
    fs.closeSync(fs.openSync(lock, 'wx'));
    const save = () => write(record.recordFile, JSON.stringify(record, null, 2) + '\n');
    try {
        verifyCompiledNative(recordFile); save();
        return executeIncremental(record, `incremental-${record.id}-${key.slice(0, 16)}`, save, policy);
    } catch (error) {
        record.status = 'FAILED_PRESERVED'; record.error = String((error as Error).stack || error); save(); throw error;
    } finally { fs.unlinkSync(lock); }
}

function executeIncremental(record: any, label: string, save: () => void, policy: ReturnType<typeof capturePolicy> & {minimumAvailableMemoryMiB?: number} = record.profile.capture) {
    const binary = record.executable.path;
    const runtime = path.join(record.folder, 'runtime');
    for (const [name, input] of Object.entries(record.inputs) as [string, any][]) write(path.join(runtime, name), fs.readFileSync(input.snapshot));
    assert(record.profile.dxc, 'Supply the same task-local DXC used by the canonical native capture');
    const dxc = record.profile.dxc.path, dxil = path.join(path.dirname(dxc), 'dxil.dll');
    assert.equal(sha(dxc), record.profile.dxc.sha256); assert.equal(sha(dxil), record.profile.dxc.dxilSha256);
    record.runtimeDlls = {dxc: {path: dxc, sha256: sha(dxc)}, dxil: {path: dxil, sha256: sha(dxil)}};
    record.executionPolicy = policy;
    record.status = 'EXECUTING'; save(); const executeStart = Date.now();
    const executeLog = path.join(viewer, 'target/course-checks', label + '-execute.log'); assert(!fs.existsSync(executeLog));
    if (record.profile.nativeAdapter) assert(!fs.existsSync(path.join(runtime, 'frame-adapter.json')), 'Preserve previous adapter execution evidence');
    const adapterInputs = record.profile.nativeAdapter ? [record.profile.nativeAdapter.metadata.path, record.profile.nativeAdapter.enumerator.path] : [];
    const beforeExecution = observeExecution([binary, dxc, dxil,
        ...(record.compiledOrigin ? [record.compiledOrigin.path, record.compiledOrigin.helper.path, record.compilerArtifact.path,
            ...record.compiledOrigin.evidence.map((file: any) => file.path)] : []),
        ...adapterInputs, ...Object.keys(record.inputs).map(name => path.join(runtime, name))]);
    const executed = runCaptureCommand(label + '-execute', [binary], runtime,
        {VIEWER_DXC_PATH: dxc, PATH: path.dirname(dxc) + path.delimiter + process.env.PATH, COURSE_MEMORY_MIB: String(policy.memoryMiB), COURSE_TIMEOUT_SECONDS: '1800',
            ...(policy.minimumAvailableMemoryMiB ? {COURSE_CAPTURE_MIN_FREE_MIB: String(policy.minimumAvailableMemoryMiB)} : {})}, policy);
    record.executionIntegrity = finishExecution(beforeExecution);
    record.adapterReceipt = verifyNativeAdapter(runtime, record.profile.nativeAdapter);
    record.renderScaleReceipt = verifyNativeScale(runtime, record.renderScaleBridge);
    checkSnapshot(record); checkWorkbench(record.inputs);
    for (const [name, input] of Object.entries(record.inputs) as [string, any][]) assert.equal(sha(path.join(runtime, name)), input.sha256, name);
    const frame = path.join(runtime, 'frame.ppm'), pixels = ppm(frame);
    assert.equal(pixels.width, record.width); assert.equal(pixels.height, record.height);
    const image = path.join(record.folder, `${record.id}-dpr${record.density}.png`); write(image, png(frame));
    const executeResources = json(path.join(viewer, 'target/course-checks', label + '-execute.resources.json'));
    verifyExecutionResources(executeResources, policy);
    record.execute = {command: executed, seconds: (Date.now() - executeStart) / 1000, log: executeLog, logSha256: sha(executeLog), resources: executeResources};
    record.image = {path: image, sha256: sha(image), width: pixels.width, height: pixels.height};
    record.status = 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW'; save(); return verifyIncremental(record.recordFile);
}

export function runIncremental(id: string, density = 2, drawMemoryMiB?: number) {
    const record: any = prepareIncremental(id, density, drawMemoryMiB);
    if (record.status === 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW') return verifyIncremental(record.recordFile);
    assert.equal(record.status, 'PREPARED_NOT_EXECUTED'); assert.deepEqual(compilerProfile(), record.profile);
    assert(record.profile.dxc, 'Supply the same task-local DXC used by the canonical native capture before starting the build');
    const lock = path.join(viewer, 'target/course-presentation/incremental-native-running.lock');
    fs.closeSync(fs.openSync(lock, 'wx'));
    const save = () => write(record.recordFile, JSON.stringify(record, null, 2) + '\n');
    try {
        checkSnapshot(record); record.status = 'BUILDING'; record.sync = synchronize(record.inputs); save();
        const label = `incremental-${id}-${record.key.slice(0, 16)}`;
        const buildLog = path.join(viewer, 'target/course-checks', label + '-build.log');
        assert(!fs.existsSync(buildLog), 'Preserve the previous compilation attempt');
        const buildStart = Date.now();
        const command = runCaptureCommand(label + '-build', ['cargo', 'build', '--example', 'presentation', '--locked', '--offline',
            '--target', record.profile.host, '-j' + record.profile.capture.jobs, '--message-format=json-render-diagnostics'], workbench,
            {CARGO_INCREMENTAL: '1', COURSE_MEMORY_MIB: String(record.profile.capture.memoryMiB), COURSE_TIMEOUT_SECONDS: '1800',
                ...(record.profile.capture.minimumAvailableMemoryMiB ? {COURSE_CAPTURE_MIN_FREE_MIB: String(record.profile.capture.minimumAvailableMemoryMiB)} : {})}, record.profile.capture);
        checkSnapshot(record); checkWorkbench(record.inputs);
        const messages = read(buildLog).split(/\r?\n/).flatMap(line => { try { return [JSON.parse(line)]; } catch { return []; } });
        const artifacts = messages.filter(message => message.reason === 'compiler-artifact' && message.target?.name === 'presentation'
            && message.target?.kind?.includes('example') && message.executable);
        assert.equal(artifacts.length, 1, 'Exactly one actual presentation executable artifact must be reported');
        const artifact = artifacts[0]; assert.equal(path.resolve(artifact.manifest_path), path.join(workbench, 'Cargo.toml'));
        assert.equal(path.resolve(artifact.target.src_path), path.join(workbench, 'examples/presentation.rs'));
        const artifactFile = path.join(record.folder, 'compiler-artifact.json'); write(artifactFile, JSON.stringify(artifact, null, 2) + '\n');
        const binary = path.join(record.folder, 'bin/presentation.exe'); fs.mkdirSync(path.dirname(binary), {recursive: true});
        fs.copyFileSync(artifact.executable, binary); assert.equal(sha(binary), sha(artifact.executable));
        record.compilerArtifact = {path: artifactFile, sha256: sha(artifactFile), fresh: artifact.fresh};
        record.executable = {path: binary, sha256: sha(binary), compilerReportedPath: artifact.executable};
        const resources = json(path.join(viewer, 'target/course-checks', label + '-build.resources.json'));
        verifyExecutionResources(resources, record.profile.capture);
        record.compile = {command, seconds: (Date.now() - buildStart) / 1000, log: buildLog, logSha256: sha(buildLog), resources};
        return executeIncremental(record, label, save, record.executionPolicy || record.profile.capture);
    } catch (error) {
        record.status = 'FAILED_PRESERVED'; record.error = String((error as Error).stack || error); save(); throw error;
    } finally { fs.unlinkSync(lock); }
}

if (process.argv[1] && path.resolve(process.argv[1]) === script) {
    const [mode, id, density] = process.argv.slice(2);
    const result = mode === 'prepare' ? prepareIncremental(id, Number(density || 2))
        : mode === 'run' ? runIncremental(id, Number(density || 2)) : mode === 'verify' ? verifyIncremental(id) : null;
    assert(result, 'Usage: incremental-native-presentation.ts prepare|run ID [2|3] | verify RECORD_JSON');
    console.log(JSON.stringify({status: result.status, id: result.id, recordFile: result.recordFile, image: result.image, sync: result.sync,
        compileSeconds: result.compile?.seconds, executeSeconds: result.execute?.seconds}, null, 2));
}
