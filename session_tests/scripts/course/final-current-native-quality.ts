import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {assetsFor, expected, filesIn, fingerprint, hash, lockFor, materialize, read, viewer, write} from './model.ts';
import {nativeTarget, runCommand, signature} from './verify.ts';

const id = '97-finish-with-the-maintained-browser-runtime';
const runner = fileURLToPath(import.meta.url);
const sha = (file: string) => hash(fs.readFileSync(file));
const json = (file: string) => JSON.parse(read(file));
const record = (file: string, data: unknown) => {
    assert(!fs.existsSync(file), `Preserve existing evidence: ${file}`);
    write(file, JSON.stringify(data, null, 2) + '\n');
};
const absolutePaths = (text: string) => {
    for (const [name, directory] of [['session_rust', '../session_rust'], ['wood', '../../wood/rust'], ['current_viewer', '.']]) {
        text = text.replace(new RegExp(`^(${name}\\s*=.*?path\\s*=\\s*)"[^"]+"`, 'm'),
            (_, prefix) => prefix + JSON.stringify(path.resolve(viewer, directory).replaceAll('\\', '/')));
    }
    return text;
};

function inputs() {
    const files = expected(id);
    return {files, sourceFingerprint: fingerprint(files), signature: signature(id), runnerSha256: sha(runner),
        lockSha256: sha(lockFor(id)), assets: Object.fromEntries(assetsFor(id).map(name => [name, sha(path.join(viewer, name))]))};
}

function prepare(dxc: string, baselinePath: string) {
    assert(path.isAbsolute(dxc));
    const source = inputs();
    const baseline = json(baselinePath);
    assert(baseline.status.startsWith('PASS:1068/1068 unfiltered'));
    const namesPath = path.resolve(viewer, baseline.testList.path);
    assert.equal(sha(namesPath), baseline.testList.sha256);
    const names: string[] = json(namesPath);
    assert.equal(names.length, 1068);
    assert.equal(new Set(names).size, names.length);
    const acceptedPreparation = path.resolve(viewer, baseline.nativePrepared.path);
    assert.equal(sha(acceptedPreparation), baseline.nativePrepared.sha256);
    const acceptedNative = json(acceptedPreparation);
    const acceptedTools = acceptedNative.tools;
    const preservedTestModules = Object.fromEntries(Object.entries(source.files)
        .filter(([name]) => name.startsWith('src/') && name.includes('test') && name.endsWith('.rs'))
        .map(([name, text]) => {
            assert.equal(text, read(path.join(acceptedNative.source, name)), `Preserve original native assertions: ${name}`);
            return [name, hash(text)];
        }));
    const tools = Object.fromEntries(['cargo', 'rustc'].map(name => {
        const result = spawnSync('rustup', ['which', name], {encoding: 'utf8', windowsHide: true});
        assert.equal(result.status, 0);
        const executable = result.stdout.trim();
        assert.equal(sha(executable), acceptedTools[name].sha256, `Retain accepted ${name} compiler identity`);
        return [name, {path: executable, sha256: sha(executable)}];
    }));
    const inventory = JSON.parse(source.files['src/current_inventory.json']);
    const roots: Record<string, string> = {viewer, kernel: path.resolve(viewer, '../session_rust'), wood: path.resolve(viewer, '../../wood/rust')};
    const canonicalPins: Record<string, string> = {};
    for (const row of inventory.files) {
        const file = path.join(roots[row.root], row.path);
        const bytes = fs.readFileSync(file), text = bytes.toString('utf8');
        const normalized = Buffer.from(text, 'utf8').equals(bytes) ? Buffer.from(text.replaceAll('\r\n', '\n')) : bytes;
        assert.equal(hash(normalized), row.sha256, `Original inventory: ${row.root}/${row.path}`);
        canonicalPins[file] = sha(file);
    }
    const productionFiles = [...filesIn(path.join(viewer, 'src')), ...filesIn(path.join(viewer, 'assets')),
        ...['Cargo.toml', 'Cargo.lock', 'build.rs', 'index.html', 'Trunk.toml'].map(name => path.join(viewer, name))];
    for (const file of productionFiles) canonicalPins[file] = sha(file);
    const compiler = {path: dxc, sha256: sha(dxc), dxil: path.join(path.dirname(dxc), 'dxil.dll'), dxilSha256: sha(path.join(path.dirname(dxc), 'dxil.dll'))};
    const key = hash(JSON.stringify({source: source.sourceFingerprint, signature: source.signature,
        runner: source.runnerSha256, canonicalPins, baseline: sha(baselinePath), compiler, tools}));
    const directory = path.join(viewer, 'target', `course-final-current-native-${key.slice(0, 20)}`);
    assert(!fs.existsSync(directory), `Use existing preparation: ${directory}`);
    fs.mkdirSync(directory);
    const stage = path.join(directory, 'source'), production = path.join(directory, 'production'), alias = path.join(directory, 'native-alias');
    materialize(source.files, stage, lockFor(id), assetsFor(id));
    const sourceManifest = absolutePaths(source.files['Cargo.toml']);
    write(path.join(stage, 'Cargo.toml'), sourceManifest);
    for (const [name, text] of Object.entries(source.files)) if (name.startsWith('src/')) assert.equal(read(path.join(stage, name)), text);
    for (const file of productionFiles) write(path.join(production, path.relative(viewer, file)), fs.readFileSync(file));
    const nativeName = `viewer_journey_final_native_${key.slice(0, 24)}`;
    const productionName = `session_viewer_native_reference_${key.slice(0, 24)}`;
    const productionManifest = absolutePaths(read(path.join(production, 'Cargo.toml')))
        .replace('name = "session_viewer"', `name = "${productionName}"`)
        .replace('[lib]', `[lib]\nname = "${productionName}"`);
    write(path.join(production, 'Cargo.toml'), productionManifest + '\n[workspace]\n');
    let manifest = sourceManifest.replace('name = "viewer_journey"', `name = "${nativeName}"`)
        .replace('crate-type = ["cdylib", "rlib"]', `name = "${nativeName}"\ncrate-type = ["rlib"]\npath = ${JSON.stringify(path.join(stage, 'src/lib.rs').replaceAll('\\', '/'))}`)
        .replaceAll('package = "session_viewer"', `package = "${productionName}"`)
        .replace(new RegExp(`^(current_viewer\\s*=.*?path\\s*=\\s*)"[^"]+"`, 'm'),
            (_, prefix) => prefix + JSON.stringify(production.replaceAll('\\', '/')))
        .replaceAll('path = "src/', `path = "${stage.replaceAll('\\', '/')}/src/`);
    assert(manifest.includes(`name = "${nativeName}"`));
    assert(manifest.includes(`package = "${productionName}"`));
    write(path.join(alias, 'Cargo.toml'), manifest);
    const seedLock = read(lockFor(id));
    assert.equal(seedLock.split('name = "viewer_journey"').length, 2);
    assert.equal(seedLock.split('name = "session_viewer"').length, 2);
    write(path.join(alias, 'Cargo.lock'), seedLock.replaceAll('"viewer_journey"', `"${nativeName}"`).replaceAll('"session_viewer"', `"${productionName}"`));
    const pins = Object.fromEntries([...filesIn(stage), ...filesIn(production), ...filesIn(alias)].map(file => [file, sha(file)]));
    const prepared = path.join(directory, 'prepared.json');
    record(prepared, {status: 'PREPARED current714 source with every original native test; no compiler/test execution',
        id, sourceFingerprint: source.sourceFingerprint, sourceFiles: Object.keys(source.files).length,
        sourceKey: key, signature: source.signature, runnerSha256: source.runnerSha256,
        courseSha256: sha(path.join(viewer, 'docs/journey/course.json')), stage, production, alias, nativeName, productionName,
        compiler, tools, pins, canonicalPins, sourceManifestLookup: viewer, inventoryEntries: inventory.files.length,
        baseline: {path: baselinePath, sha256: sha(baselinePath), names: {path: namesPath, sha256: sha(namesPath)}, expectedCount: names.length},
        preservedTestModules,
        overlay: 'Exact source714 src and test bodies. Original source Cargo lookup remains canonical for inventory tests; separate unique native compiler/production aliases only. Registry versions/checksums and original Cargo.lock retained.'});
    console.log(prepared);
}

function run(preparedPath: string) {
    const p = json(preparedPath);
    const current = () => {
        assert.equal(sha(runner), p.runnerSha256);
        assert.equal(fingerprint(expected(id)), p.sourceFingerprint);
        assert.equal(signature(id), p.signature);
        for (const [file, wanted] of Object.entries({...p.pins, ...p.canonicalPins})) assert.equal(sha(file), wanted, file);
        for (const ref of [p.baseline, p.baseline.names]) assert.equal(sha(ref.path), ref.sha256);
        assert.equal(sha(p.compiler.path), p.compiler.sha256);
        assert.equal(sha(p.compiler.dxil), p.compiler.dxilSha256);
        for (const tool of Object.values(p.tools) as {path: string; sha256: string}[]) assert.equal(sha(tool.path), tool.sha256);
    };
    current();
    const directory = path.dirname(preparedPath), buildPath = path.join(directory, 'build');
    assert(!fs.existsSync(buildPath), 'One build and one unfiltered integration per preparation');
    fs.mkdirSync(buildPath);
    const prefix = `final-current-native-${p.sourceKey.slice(0, 12)}-${new Date().toISOString().replace(/[^0-9TZ]/g, '')}`;
    const env: Record<string, string> = {COURSE_MEMORY_MIB: '4096', COURSE_TIMEOUT_SECONDS: '1800', CARGO_INCREMENTAL: '0',
        RUST_TEST_THREADS: '1', RUSTC_WRAPPER: '', RUSTC_WORKSPACE_WRAPPER: '', RUSTFLAGS: '', CARGO_ENCODED_RUSTFLAGS: '',
        RUSTC: p.tools.rustc.path,
        WGPU_BACKEND: 'dx12', VIEWER_ADAPTER: 'Microsoft Basic Render Driver', VIEWER_GPU_WAIT_SECONDS: '60', VIEWER_DXC_PATH: p.compiler.path};
    const evidence = path.join(viewer, 'target/course-checks');
    const phase = (label: string, args: string[]) => {
        current();
        assert(!fs.existsSync(path.join(evidence, `${label}.log`)));
        const command = runCommand(label, args, viewer, env);
        current();
        const log = path.join(evidence, `${label}.log`), resourcesPath = path.join(evidence, `${label}.resources.json`);
        const resources = process.platform === 'win32' ? json(resourcesPath) : null;
        if (resources) {
            assert.equal(resources.exitCode, 0); assert.equal(resources.timedOut, false);
            assert.equal(resources.processors, 2); assert.equal(resources.jobCommitLimitBytes, 4294967296);
        }
        return {command, log: {path: log, sha256: sha(log)},
            resources: resources ? {path: resourcesPath, sha256: sha(resourcesPath), measured: resources} : null};
    };
    const build = phase(`${prefix}-build`, [p.tools.cargo.path, 'test', '--manifest-path', path.join(p.alias, 'Cargo.toml'),
        '--lib', '--no-run', '--locked', '--offline', '--target', nativeTarget(), '-j1', '-vv', '--message-format=json-render-diagnostics']);
    const log = read(build.log.path);
    const artifacts = log.split(/\r?\n/).flatMap(line => {try {return [JSON.parse(line)];} catch {return [];}});
    const artifact = artifacts.find(row => row.reason === 'compiler-artifact' && row.target.name === p.nativeName && row.profile.test && row.executable);
    assert(artifact && !artifact.fresh, 'Fresh unique current native test executable required');
    assert.equal(path.resolve(artifact.target.src_path), path.join(p.stage, 'src/lib.rs'));
    assert(log.includes(`--crate-name ${p.nativeName} `));
    const binary = path.join(buildPath, 'native-full-current.exe');
    fs.copyFileSync(artifact.executable, binary, fs.constants.COPYFILE_EXCL);
    for (const [file, name, wanted] of [[p.compiler.path, 'dxcompiler.dll', p.compiler.sha256], [p.compiler.dxil, 'dxil.dll', p.compiler.dxilSha256]]) {
        fs.copyFileSync(file, path.join(buildPath, name), fs.constants.COPYFILE_EXCL);
        assert.equal(sha(path.join(buildPath, name)), wanted);
    }
    const dep = artifact.executable.replace(/\.exe$/, '.d');
    assert(fs.existsSync(dep)); assert(read(dep).includes('ambient_sources.wgsl'));
    fs.copyFileSync(dep, path.join(buildPath, 'native-full-current.d'), fs.constants.COPYFILE_EXCL);
    const bytes = fs.readFileSync(binary);
    for (const marker of ['source_normal_y', 'source_normal_z']) assert(bytes.includes(Buffer.from(marker)));
    const binarySha256 = sha(binary);
    const listing = phase(`${prefix}-list`, [binary, '--list']);
    const listed = read(listing.log.path).split(/\r?\n/).filter(line => line.endsWith(': test')).map(line => line.slice(0, -6));
    const baseline: string[] = json(p.baseline.names.path);
    assert.deepEqual([...listed].sort(), [...baseline].sort(), 'Every original accepted test remains present');
    assert.equal(listed.length, p.baseline.expectedCount); assert.equal(new Set(listed).size, listed.length);
    record(path.join(directory, 'built.json'), {status: 'PASS actual current714 compilation/list only',
        sourceFingerprint: p.sourceFingerprint, binary, binarySha256, artifact, build, listing, actualTestCount: listed.length});
    const execution = phase(`${prefix}-full`, [binary, '--test-threads=1']);
    const stdout = read(execution.log.path);
    const passed = [...stdout.matchAll(/^test (\S+) \.\.\. ok\s*$/gm)].map(match => match[1]);
    assert.deepEqual([...passed].sort(), [...listed].sort()); assert.equal(new Set(passed).size, listed.length);
    assert(stdout.includes(`test result: ok. ${listed.length} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;`));
    assert.equal(sha(binary), binarySha256);
    record(path.join(directory, 'result.json'), {status: 'PASS one actual unfiltered current final97 native integration',
        id, sourceFingerprint: p.sourceFingerprint, sourceFiles: p.sourceFiles, courseSha256: p.courseSha256,
        tests: {passed: listed.length, failed: 0, ignored: 0, measured: 0, filtered: 0, threads: 1},
        prepared: {path: preparedPath, sha256: sha(preparedPath)}, binary: {path: binary, sha256: binarySha256},
        depInfo: {path: path.join(buildPath, 'native-full-current.d'), sha256: sha(path.join(buildPath, 'native-full-current.d'))},
        build, listing, execution, baseline: p.baseline, namedPassed: passed,
        scope: 'All original714 native tests executed once; no new browser/WASM/optimized packaging or release-gate acceptance inferred'});
    console.log(path.join(directory, 'result.json'));
}

const args = process.argv.slice(2);
if (args[0] === '--prepare' && args.includes('--dxc')) {
    const baseline = args.includes('--baseline') ? args[args.indexOf('--baseline') + 1] : path.join(viewer, 'docs/journey/release/final-brep-final-native-integration-review.json');
    prepare(path.resolve(args[args.indexOf('--dxc') + 1]), path.resolve(baseline));
} else if (args[0] === '--run' && args[1]) run(path.resolve(args[1]));
else throw Error('Use --prepare --dxc ABSOLUTE_DLL [--baseline RECORD], then --run PREPARED_JSON');
