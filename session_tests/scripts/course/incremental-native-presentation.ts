import {cargoTarget, observeExecution, finishExecution, compilerTools} from './capture-storage.ts';
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {spawnSync} from 'node:child_process';
import {bindings, prepare as prepareSnapshot} from './presentation.ts';
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
    const guard = fileURLToPath(new URL('capture-guard.ps1', import.meta.url));
    return {processors, memoryMiB: processors === 8 ? 8192 : 6144, jobs: processors === 8 ? 8 : 1,
        guard, guardSha256: sha(guard)};
}

function runCaptureCommand(name: string, args: string[], directory: string, extra: Record<string, string> = {}) {
    const policy = capturePolicy(), evidence = path.join(viewer, 'target/course-checks');
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
            '-TimeoutSeconds', '1800', '-ManifestPath', path.join(evidence, name + '.resources.json'),
            '-WorkingDirectory', directory, '-CommandFile', command], {cwd: directory, windowsHide: true,
            env: {...process.env, RUSTC_WRAPPER: '', CARGO_NET_OFFLINE: 'true', NO_COLOR: 'true',
                CARGO_BUILD_JOBS: String(policy.jobs), RUST_TEST_THREADS: '1', REGEN_PROTO: '0',
                CARGO_TARGET_DIR: cargoTarget(), ...extra},
            stdio: ['ignore', descriptor, descriptor]});
    } finally { fs.closeSync(descriptor); }
    if (result.error || result.status !== 0) throw Error(`${name} failed. Read ${log}\n${read(log).slice(-4000)}`);
    return {name, args, returncode: result.status};
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

function compilerProfile() {
    const rustc = spawnSync('rustc', ['-vV'], {encoding: 'utf8', windowsHide: true});
    const cargo = spawnSync('cargo', ['--version'], {encoding: 'utf8', windowsHide: true});
    assert.equal(rustc.status, 0); assert.equal(cargo.status, 0);
    const dxc = process.env.VIEWER_DXC_PATH;
    return {toolFiles: compilerTools(), host: nativeTarget(), rustc: rustc.stdout.trim(), cargo: cargo.stdout.trim(),
        capture: capturePolicy(), CARGO_INCREMENTAL: '1', CARGO_BUILD_JOBS: String(capturePolicy().jobs), CARGO_TARGET_DIR: cargoTarget(),
        RUSTFLAGS: process.env.RUSTFLAGS || '', CARGO_ENCODED_RUSTFLAGS: process.env.CARGO_ENCODED_RUSTFLAGS || '',
        dxc: dxc && fs.existsSync(dxc) ? {path: path.resolve(dxc), sha256: sha(dxc), dxilSha256: sha(path.join(path.dirname(dxc), 'dxil.dll'))} : null};
}

function snapshotInputs(snapshot: any) {
    const names = new Set([...Object.keys(expected(snapshot.source.checkpoint)), 'Cargo.lock', ...assetsFor(snapshot.source.checkpoint)]);
    const driver = path.relative(snapshot.directory, path.join(viewer, snapshot.driver.path)).split(path.sep).join('/');
    names.add(driver);
    return Object.fromEntries([...names].sort().map(name => [name, {snapshot: path.join(snapshot.directory, name), sha256: sha(path.join(snapshot.directory, name))}]));
}

function checkSnapshot(record: any) {
    if (sha(script) !== record.helperSha256) {
        const legacy = 'a1b6e47c0aea300df5b23cd7e2738e9bf00b4d2804d9fd4493344ab8d2d0f266';
        assert.equal(record.helperSha256, legacy, 'Unknown historical incremental helper');
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

export function prepareIncremental(id: string, density = 2) {
    const snapshot = prepareSnapshot(id, density), profile = compilerProfile(), helperSha256 = sha(script);
    const key = hash(JSON.stringify({snapshotKey: snapshot.key, density, profile, helperSha256}));
    const folder = path.join(viewer, 'target/course-presentation/incremental-native', id + '-' + key.slice(0, 20));
    const recordFile = path.join(folder, 'result.json');
    if (fs.existsSync(recordFile)) { const existing = json(recordFile); checkSnapshot(existing); return existing; }
    const record = {status: 'PREPARED_NOT_EXECUTED', id, density, key, source: snapshot.source, snapshotKey: snapshot.key,
        snapshotDirectory: snapshot.directory, snapshotDriver: snapshot.driver, profile, helperSha256,
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
    return record;
}

export function runIncremental(id: string, density = 2) {
    const record: any = prepareIncremental(id, density);
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
            {CARGO_INCREMENTAL: '1', COURSE_MEMORY_MIB: '6144', COURSE_TIMEOUT_SECONDS: '1800'});
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
        assert.equal(resources.exitCode, 0); assert.equal(resources.processors, record.profile.capture.processors); assert.equal(resources.jobCommitLimitBytes, record.profile.capture.memoryMiB * 1048576);
        record.compile = {command, seconds: (Date.now() - buildStart) / 1000, log: buildLog, logSha256: sha(buildLog), resources};
        const runtime = path.join(record.folder, 'runtime');
        for (const [name, input] of Object.entries(record.inputs) as [string, any][]) write(path.join(runtime, name), fs.readFileSync(input.snapshot));
        assert(record.profile.dxc, 'Supply the same task-local DXC used by the canonical native capture');
        const dxc = record.profile.dxc.path, dxil = path.join(path.dirname(dxc), 'dxil.dll');
        assert.equal(sha(dxc), record.profile.dxc.sha256); assert.equal(sha(dxil), record.profile.dxc.dxilSha256);
        record.runtimeDlls = {dxc: {path: dxc, sha256: sha(dxc)}, dxil: {path: dxil, sha256: sha(dxil)}};
        record.status = 'EXECUTING'; save(); const executeStart = Date.now();
        const executeLog = path.join(viewer, 'target/course-checks', label + '-execute.log'); assert(!fs.existsSync(executeLog));
        const beforeExecution = observeExecution([binary, dxc, dxil, ...Object.keys(record.inputs).map(name => path.join(runtime, name))]);
        const executed = runCaptureCommand(label + '-execute', [binary], runtime,
            {VIEWER_DXC_PATH: dxc, PATH: path.dirname(dxc) + path.delimiter + process.env.PATH, COURSE_MEMORY_MIB: '6144', COURSE_TIMEOUT_SECONDS: '1800'});
        record.executionIntegrity = finishExecution(beforeExecution);
        checkSnapshot(record); checkWorkbench(record.inputs);
        for (const [name, input] of Object.entries(record.inputs) as [string, any][]) assert.equal(sha(path.join(runtime, name)), input.sha256, name);
        const frame = path.join(runtime, 'frame.ppm'), pixels = ppm(frame);
        assert.equal(pixels.width, record.width); assert.equal(pixels.height, record.height);
        const image = path.join(record.folder, `${id}-dpr${density}.png`); write(image, png(frame));
        const executeResources = json(path.join(viewer, 'target/course-checks', label + '-execute.resources.json'));
        assert.equal(executeResources.exitCode, 0); assert.equal(executeResources.processors, record.profile.capture.processors); assert.equal(executeResources.jobCommitLimitBytes, record.profile.capture.memoryMiB * 1048576);
        record.execute = {command: executed, seconds: (Date.now() - executeStart) / 1000, log: executeLog, logSha256: sha(executeLog), resources: executeResources};
        record.image = {path: image, sha256: sha(image), width: pixels.width, height: pixels.height};
        record.status = 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW'; save(); return verifyIncremental(record.recordFile);
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
