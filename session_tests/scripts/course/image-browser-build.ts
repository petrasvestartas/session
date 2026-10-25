import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {prepareBrowser} from './browser-presentation.ts';
import {bindings} from './presentation.ts';
import {exactBindgen} from './exact-bindgen.ts';
import {assetsFor, docs, expected, filesIn, hash, json, lockFor, read, viewer, write} from './model.ts';
import {nativeTarget} from './verify.ts';

const self = fileURLToPath(import.meta.url);
const original = fileURLToPath(new URL('./browser-presentation.ts', import.meta.url));
const guard = fileURLToPath(new URL('./capture-guard.ps1', import.meta.url));
const sha = (file: string) => hash(fs.readFileSync(file));
const policy = {purpose: 'Image builds only; original behavior/capture guards unchanged', processors: 8, memoryMiB: 8192, jobs: 8, timeoutSeconds: 1800, profile: 'debug'};
const builtStatuses = ['BUILT_NOT_CAPTURED', 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW'];

function validate(record: any) {
    assert.deepEqual(record.source, bindings(record.id));
    for (const [file, digest] of Object.entries(record.pins)) assert.equal(sha(file), digest, file);
    assert.equal(sha(record.worker), record.workerSha256);
    assert.equal(sha(record.adapted), record.adaptedSha256);
    const kernel = path.relative(record.native, path.resolve(viewer, '../session_rust')).split(path.sep).join('/');
    for (const [name, text] of Object.entries(expected(record.id)))
        assert.equal(read(path.join(record.native, name)), name === 'Cargo.toml' ? text.replaceAll('../../../session_rust', kernel) : text, name);
    assert.equal(sha(path.join(record.native, 'Cargo.lock')), sha(lockFor(record.id)));
    for (const name of assetsFor(record.id)) assert.equal(sha(path.join(record.native, name)), sha(path.join(viewer, name)), name);
    if (record.bundle) {
        const dist = path.join(record.workspace, 'target/course-checks', record.id, 'dist');
        assert.deepEqual(Object.fromEntries(filesIn(dist).map(file => [path.relative(dist, file).split(path.sep).join('/'), sha(file)])), record.bundle);
    }
}

export function prepareImageBrowser(id: string, density = 2) {
    assert([2, 3].includes(density));
    const capture = path.join(docs, 'capture_journey.cjs');
    const key = hash(JSON.stringify({source: bindings(id), density, helper: sha(original), capture: sha(capture)}));
    const folder = path.join(viewer, 'target/course-presentation', `browser-${id}-${key.slice(0, 16)}-dpr${density}`);
    const preparedFile = path.join(folder, 'prepared.json');
    const record = fs.existsSync(preparedFile) ? json(preparedFile) : prepareBrowser(id, density);
    validate(record);
    if (builtStatuses.includes(record.status)) return {record, preparedFile, reused: true};
    assert.equal(record.status, 'PREPARED_NOT_BUILT');
    assert(fs.existsSync(guard), 'The dedicated image-only guard must exist before preparation');
    const identity = {source: record.source, density, wrapperSha256: sha(self), guard: {path: guard, sha256: sha(guard)}, policy,
        originalBuilder: {path: original, sha256: sha(original)}, bindgenSelectorSha256: sha(fileURLToPath(new URL('./exact-bindgen.ts', import.meta.url)))};
    const directory = path.join(viewer, 'target/course-presentation/image-builds', hash(JSON.stringify(identity)).slice(0, 24));
    const file = path.join(directory, 'prepared.json');
    const sidecar = {status: 'PREPARED_IMAGE_BUILD_NOT_EXECUTED', directory, preparedFile, ...identity};
    if (fs.existsSync(file)) assert.deepEqual(json(file), sidecar, 'Preserve an existing image-build attempt');
    else write(file, JSON.stringify(sidecar, null, 2) + '\n');
    return {record, preparedFile, reused: false, sidecar, sidecarFile: file};
}

export function buildImageBrowser(id: string, density = 2) {
    const prepared = prepareImageBrowser(id, density), record = prepared.record;
    if (prepared.reused) return record;
    assert.equal(process.platform, 'win32', 'This accelerated policy uses the Windows image-only Job Object guard');
    const sidecar = prepared.sidecar!, commands: any[] = [];
    const check = () => {
        validate(record);
        assert.equal(sha(self), sidecar.wrapperSha256); assert.equal(sha(guard), sidecar.guard.sha256);
        assert.equal(sha(fileURLToPath(new URL('./exact-bindgen.ts', import.meta.url))), sidecar.bindgenSelectorSha256);
        assert.deepEqual(json(prepared.sidecarFile!), sidecar);
    };
    const execute = (label: string, args: string[]) => {
        check();
        const prefix = path.join(sidecar.directory, label), commandFile = prefix + '.command.json', log = prefix + '.log', resourcesFile = prefix + '.resources.json';
        assert(!fs.existsSync(log), 'Preserve previous actual build output: ' + log);
        write(commandFile, JSON.stringify({executable: args[0], arguments: args.slice(1)}) + '\n');
        const fd = fs.openSync(log, 'wx');
        let result;
        try {
            result = spawnSync('powershell.exe', ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', guard,
                '-ProcessorCount', String(policy.processors), '-MemoryLimitMiB', String(policy.memoryMiB), '-TimeoutSeconds', String(policy.timeoutSeconds),
                '-WorkingDirectory', record.native, '-ManifestPath', resourcesFile, '-CommandFile', commandFile],
            {cwd: record.native, windowsHide: true, stdio: ['ignore', fd, fd], env: {...process.env, RUSTC_WRAPPER: '', CARGO_NET_OFFLINE: process.env.CARGO_NET_OFFLINE || 'true',
                NO_COLOR: 'true', CARGO_BUILD_JOBS: String(policy.jobs), RUST_TEST_THREADS: '1', REGEN_PROTO: '0', CARGO_TARGET_DIR: path.join(viewer, 'target/course-build')}});
        } finally {fs.closeSync(fd);}
        const resources = json(resourcesFile);
        assert.equal(result.status, 0, log); assert.equal(resources.exitCode, 0); assert.equal(resources.timedOut, false);
        assert.equal(resources.processors, policy.processors); assert.equal(resources.jobCommitLimitBytes, policy.memoryMiB * 1048576);
        check();
        const receipt = {label, args, commandFile, log, logSha256: sha(log), resourcesFile, resourcesSha256: sha(resourcesFile), resources};
        commands.push(receipt); return receipt;
    };
    if (fs.existsSync(path.join(record.native, 'examples/sample.rs'))) {
        execute('sample', ['cargo', 'run', '--example', 'sample', '--locked', '--target', nativeTarget(), '-j8']);
        assert(fs.statSync(path.join(record.native, 'sample.pb')).size > 0);
    }
    execute('wasm', ['cargo', 'build', '--lib', '--locked', '--target', 'wasm32-unknown-unknown', '-j8']);
    const crate = read(path.join(record.native, 'Cargo.toml')).match(/^name\s*=\s*"([^"]+)"/m)![1].replaceAll('-', '_');
    const wasm = path.join(viewer, 'target/course-build/wasm32-unknown-unknown/debug', crate + '.wasm');
    const wasmSha256 = sha(wasm), bindgen = exactBindgen(path.join(record.native, 'Cargo.lock'));
    const dist = path.join(record.workspace, 'target/course-checks', id, 'dist');
    execute('bindgen', [bindgen.executable, wasm, '--target', 'web', '--out-dir', dist, '--out-name', 'viewer_journey']);
    assert.equal(sha(wasm), wasmSha256); assert.equal(sha(bindgen.executable), bindgen.sha256);
    const start = `<script type="module">import init,* as bindings from './viewer_journey.js'; const wasm=await init({module_or_path:'./viewer_journey_bg.wasm'}); window.wasmBindings=bindings; dispatchEvent(new CustomEvent('TrunkApplicationStarted',{detail:{wasm}}));</script>`;
    let index = read(path.join(record.native, 'index.html')).replace(/<link\b[^>]*\bdata-trunk\b[^>]*\brel="rust"[^>]*>/, start);
    assert(index.includes(start)); index = index.replace(/<link\b[^>]*\bdata-trunk\b[^>]*>/g, '');
    write(path.join(dist, 'index.html'), index);
    for (const asset of assetsFor(id)) {
        const destination = asset.startsWith('assets/text/') ? asset.replace('assets/text/', 'text/') : asset;
        const file = path.join(dist, destination); fs.mkdirSync(path.dirname(file), {recursive: true}); fs.copyFileSync(path.join(record.native, asset), file);
    }
    fs.cpSync(record.native, path.join(record.workspace, 'target', `course-${id}`), {recursive: true, filter: file => !file.includes(`${path.sep}dist${path.sep}`)});
    check();
    const methodFile = path.join(sidecar.directory, 'built.json'); assert(!fs.existsSync(methodFile));
    const method = {...sidecar, status: 'BUILT_IMAGE_ONLY_SOURCE_BOUND', commands, bindgen, wasm: {path: wasm, sha256: wasmSha256},
        bundle: Object.fromEntries(filesIn(dist).map(file => [path.relative(dist, file).split(path.sep).join('/'), sha(file)])),
        scope: 'Same debug WASM, lock-matched bindgen, sample/index/assets recipe; image build resource policy only. No canonical callback execution or GPU capture claim.'};
    write(methodFile, JSON.stringify(method, null, 2) + '\n');
    const built = {...record, status: 'BUILT_NOT_CAPTURED', commands, wasm: method.wasm, bundle: method.bundle, imageBuild: {path: methodFile, sha256: sha(methodFile)}};
    validate(built); write(prepared.preparedFile, JSON.stringify(built, null, 2) + '\n'); return built;
}

if (process.argv[1] && path.resolve(process.argv[1]) === self) {
    const [mode, id, density = '2'] = process.argv.slice(2);
    assert(['prepare', 'build'].includes(mode), 'image-browser-build.ts prepare|build CHECKPOINT [DENSITY]');
    console.log(JSON.stringify(mode === 'prepare' ? prepareImageBrowser(id, Number(density)) : buildImageBrowser(id, Number(density)), null, 2));
}
