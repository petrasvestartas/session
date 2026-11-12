import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {spawnSync} from 'node:child_process';

type Runtime = {fs: typeof fs; run: typeof spawnSync};
const production: Runtime = {fs, run: spawnSync};
const packages = ['image_incremental', 'session_rust', 'wood', 'session_viewer'];

export function imageBuildMode(environment: NodeJS.ProcessEnv = process.env) {
    const lowDisk = environment.COURSE_IMAGE_LOW_DISK === '1';
    return {lowDisk, nativeIncremental: '0', wasmIncremental: lowDisk ? '0' : '1'};
}

export function nativeSamplePhase(viewer: string, p: any, target: () => string, check: () => void,
    execute: (label: string, args: string[]) => unknown, runtime: Runtime = production) {
    const io = runtime.fs, g = p.group;
    if (!io.existsSync(path.join(g.workbench, 'examples/sample.rs'))) return null;
    const host = target();
    assert(/^[A-Za-z0-9_-]+$/.test(host), 'Native target must be a single toolchain triple');
    const sample = path.join(g.workbench, 'sample.pb'), copied = path.join(p.folder, 'sample.pb');
    if (io.existsSync(sample)) io.unlinkSync(sample);
    const sampleCommand: any = execute('sample', ['cargo', 'run', '--example', 'sample', '--locked', '--target', host, '-j2',
        ...(p.buildMode.lowDisk ? ['--message-format=json-render-diagnostics'] : [])]);
    assert(io.statSync(sample).size > 0, 'Native sample must exist before cache retirement');
    io.copyFileSync(sample, copied);
    if (!p.buildMode.lowDisk) return null;
    assert.equal(host, p.tools.rustc.match(/^host: ([A-Za-z0-9_-]+)$/m)?.[1], 'Native cache target differs from the recorded compiler host');
    assert.notEqual(host, 'wasm32-unknown-unknown');

    const hash = (file: string) => createHash('sha256').update(io.readFileSync(file)).digest('hex');
    const sampleHash = hash(sample);
    assert.equal(hash(copied), sampleHash, 'Copied native fixture differs');
    const viewerRoot = path.resolve(viewer), cache = path.join(viewerRoot, 'target/course-cargo');
    assert.equal(path.resolve(g.target), cache, 'Only the shared course Cargo cache is eligible');
    assert.equal(path.resolve(g.workbench), path.join(viewerRoot, 'target/course-ib/compact-browser-v1/workspace'));
    const native = path.join(cache, host), allowed = path.join(native, 'debug');
    const manifest = path.join(g.workbench, 'Cargo.toml'), lock = path.join(g.workbench, 'Cargo.lock');
    const manifestHash = hash(manifest), lockHash = hash(lock);
    assert.equal(manifestHash, p.overlay.cargoSha256);
    assert.equal(lockHash, p.overlay.lockSha256);
    // The successful --locked sample run validated this pinned Cargo lockfile.
    const blocks = io.readFileSync(lock, 'utf8').split(/^\[\[package\]\]\r?$/m);
    assert(blocks.length > 1 && /^version = [34]\r?$/m.test(blocks[0]), 'Expected a Cargo lockfile with package declarations');
    const declared = blocks.slice(1).map(block => {
        const names = [...block.matchAll(/^name = "([A-Za-z0-9_-]+)"\r?$/gm)];
        assert.equal(names.length, 1, 'Cargo lock package must have exactly one name');
        assert(/^version = "[^"\r\n]+"\r?$/m.test(block), 'Cargo lock package must have a version');
        return {name: names[0][1], local: !/^source\s*=/m.test(block)};
    });
    const selected = packages.filter(name => declared.some(entry => entry.name === name));
    assert(selected.includes('image_incremental'), 'Cargo lock must declare the image_incremental root');
    for (const name of selected) {
        const entries = declared.filter(entry => entry.name === name);
        assert(entries.length === 1 && entries[0].local, 'Native cleanup requires one local package: ' + name);
    }
    const skippedPackages = packages.filter(name => !selected.includes(name));
    const safe = (file: string) => {
        file = path.resolve(file);
        let existing = file;
        while (!io.existsSync(existing)) {
            const parent = path.dirname(existing); assert.notEqual(parent, existing); existing = parent;
        }
        assert.equal(io.realpathSync(existing), existing, 'Native cache path is redirected');
        for (let item = existing; ; item = path.dirname(item)) {
            assert(!io.lstatSync(item).isSymbolicLink(), 'Native cache cannot traverse links');
            if (path.dirname(item) === item) break;
        }
        return file;
    };
    const unchanged = () => {
        assert.equal(hash(sample), sampleHash); assert.equal(hash(copied), sampleHash);
        assert.equal(hash(manifest), manifestHash); assert.equal(hash(lock), lockHash);
        assert.equal(hash(p.toolFiles.cargo.path), p.toolFiles.cargo.sha256);
        safe(allowed);
    };
    check();
    const validate = (stdout: string) => stdout.split(/\r?\n/).filter(line => line.trim()).map(line => {
        assert.equal(line, line.trim()); assert(path.isAbsolute(line), 'Cargo dry-run must return absolute paths');
        const file = safe(line), relative = path.relative(allowed, file);
        assert(relative && relative !== '..' && !relative.startsWith('..' + path.sep) && !path.isAbsolute(relative), 'Native clean escapes its debug cache');
        const inspect = (entry: string) => {
            safe(entry);
            if (io.existsSync(entry) && io.lstatSync(entry).isDirectory()) for (const child of io.readdirSync(entry)) inspect(path.join(entry, String(child)));
        };
        inspect(file); return file;
    });
    const attempts: any[] = [];
    const run = (args: string[]) => {
        unchanged();
        const result = runtime.run(p.toolFiles.cargo.path, args, {cwd: g.workbench, windowsHide: true, shell: false,
            encoding: 'utf8', maxBuffer: 32 * 1024 * 1024,
            env: {...process.env, CARGO_TARGET_DIR: cache, CARGO_NET_OFFLINE: 'true', CARGO_INCREMENTAL: '0'}});
        const command = {executable: p.toolFiles.cargo.path, arguments: args, stdout: String(result.stdout), stderr: String(result.stderr), exitCode: result.status};
        attempts.push(command);
        assert(!result.error, String(result.error)); assert.equal(result.status, 0, String(result.stderr));
        unchanged(); return command;
    };
    // Successful Cargo JSON is the complete current closure; dep-info identifies old registry-only siblings.
    assert.equal(sampleCommand?.log, path.join(p.folder, 'sample.log'), 'Exact successful sample log required');
    assert.equal(sampleCommand.resources?.exitCode, 0); assert.equal(sampleCommand.resources?.timedOut, false);
    assert.equal(hash(sampleCommand.log), sampleCommand.logSha256, 'Sample compiler log changed');
    const messages = io.readFileSync(sampleCommand.log, 'utf8').split(/\r?\n/).filter(line => line.startsWith('{')).map(line => JSON.parse(line));
    const finished = messages.filter(row => row.reason === 'build-finished');
    assert.equal(finished.length, 1); assert.equal(finished[0].success, true, 'Successful Cargo closure required');
    const artifacts = messages.filter(row => row.reason === 'compiler-artifact');
    assert(artifacts.length > 0, 'Nonempty Cargo artifact closure required');
    const own = artifacts.filter(row => path.resolve(row.manifest_path) === manifest && row.target?.kind?.includes('example') && row.target.name === 'sample');
    assert.equal(own.length, 1, 'Exactly one current sample artifact required');
    assert.equal(path.resolve(own[0].executable), path.join(allowed, 'examples/sample.exe'));
    assert(own[0].filenames.includes(own[0].executable)); safe(own[0].executable);
    assert(io.statSync(own[0].executable).isFile(), 'Actual successful sample executable required');
    const deps = safe(path.join(allowed, 'deps')), currentPaths = new Set<string>(), registryRoots = new Set<string>();
    const variant = /^lib([A-Za-z0-9_]+)-([a-f0-9]{16})\.(?:rlib|rmeta)$/;
    for (const artifact of artifacts) {
        assert(Array.isArray(artifact.filenames) && artifact.filenames.every((file: any) => typeof file === 'string' && path.isAbsolute(file)), 'Absolute compiler filenames required');
        assert(typeof artifact.fresh === 'boolean', 'Fresh and cached compiler records must be explicit');
        if (!String(artifact.package_id).startsWith('registry+')) continue;
        const cargo = safe(path.resolve(artifact.manifest_path));
        assert(cargo.replaceAll('\\', '/').includes('/registry/src/') && path.basename(cargo) === 'Cargo.toml', 'Observed registry manifest required');
        registryRoots.add(path.dirname(path.dirname(cargo)));
        for (const file of artifact.filenames) {
            const full = path.resolve(file);
            if (path.dirname(full) !== deps) continue; // Shared host proc macros and runtime binaries are never candidates.
            const match = variant.exec(path.basename(full));
            if (!match) continue;
            assert.equal(full, path.normalize(file), 'Compiler artifact path must be canonical');
            for (const name of ['lib' + match[1] + '-' + match[2] + '.rlib', 'lib' + match[1] + '-' + match[2] + '.rmeta', match[1] + '-' + match[2] + '.d']) {
                const sibling = path.join(deps, name); if (io.existsSync(sibling)) currentPaths.add(safe(sibling));
            }
            assert(currentPaths.has(full), 'Compiler-reported current artifact is missing');
            assert(currentPaths.has(path.join(deps, match[1] + '-' + match[2] + '.d')), 'Current registry dep-info is required');
        }
    }
    assert(currentPaths.size > 0 && registryRoots.size > 0, 'Current Native registry closure is required');
    const pin = (file: string) => {
        assert.equal(path.dirname(file), deps, 'Registry retirement stays in the exact Native deps directory');
        safe(file); const stat = io.lstatSync(file); assert(stat.isFile(), 'Registry artifact must be a regular file');
        return {path: file, bytes: stat.size, mtimeMs: stat.mtimeMs, sha256: hash(file)};
    };
    const current = [...currentPaths].sort().map(pin), obsolete: ReturnType<typeof pin>[] = [];
    for (const name of io.readdirSync(deps)) {
        const match = /^([A-Za-z0-9_]+)-([a-f0-9]{16})\.d$/.exec(String(name));
        if (!match || packages.includes(match[1])) continue;
        const dep = path.join(deps, String(name));
        if (currentPaths.has(dep)) continue;
        safe(dep); assert(io.lstatSync(dep).isFile());
        const first = io.readFileSync(dep, 'utf8').split(/\r?\n/)[0], prefix = dep + ': ';
        if (!first.startsWith(prefix)) continue;
        const input = first.slice(prefix.length).replaceAll('\\', '/');
        const registryRoot = [...registryRoots].find(folder => input.startsWith(folder.replaceAll('\\', '/') + '/'));
        if (!registryRoot) continue;
        const packageFolder = input.slice(registryRoot.length + 1).split('/')[0];
        const registryPackage = /^([A-Za-z0-9_-]+)-(\d+\.\d+\.\d+[^\s/]*)$/.exec(packageFolder);
        if (!registryPackage || registryPackage[1].replaceAll('-', '_') !== match[1]) continue;
        const firstSource = input.split(' ')[0], packageRoot = path.join(registryRoot, packageFolder);
        const relativeSource = path.relative(packageRoot, path.resolve(firstSource));
        if (!relativeSource || relativeSource === '..' || relativeSource.startsWith('..' + path.sep) || path.isAbsolute(relativeSource)) continue;
        safe(firstSource); assert(io.statSync(firstSource).isFile(), 'Registry source evidence must exist');
        const registryManifest = safe(path.join(registryRoot, packageFolder, 'Cargo.toml'));
        assert(io.statSync(registryManifest).isFile(), 'Obsolete variant requires actual registry source ownership');
        const libraries = [path.join(deps, 'lib' + match[1] + '-' + match[2] + '.rlib'), path.join(deps, 'lib' + match[1] + '-' + match[2] + '.rmeta')];
        if (!libraries.some(file => io.existsSync(file))) continue; // Do not retire binary/proc-macro-only dep-info.
        for (const file of [...libraries, dep])
            if (io.existsSync(file)) { assert(!currentPaths.has(file)); obsolete.push(pin(file)); }
    }
    const ownership = path.join(path.dirname(g.workbench), 'workbench.lock');
    const ownershipHash = hash(ownership);
    assert.equal(JSON.parse(io.readFileSync(ownership, 'utf8')).pid, process.pid, 'This process must own the active compiler workbench');
    const registryUnchanged = () => {
        unchanged(); safe(deps); safe(ownership);
        assert.equal(hash(ownership), ownershipHash, 'Compiler workbench ownership changed');
        assert.equal(hash(sampleCommand.log), sampleCommand.logSha256, 'Successful compiler closure changed');
    };
    registryUnchanged();
    const registry = {scope: 'Only obsolete Native registry rlib/rmeta/dep-info variants; current closure, binaries, fingerprints, host and WASM untouched',
        compilerLog: {path: sampleCommand.log, sha256: sampleCommand.logSha256}, artifactRecords: artifacts.length,
        current, obsolete, retired: [] as string[]};

    const plans = selected.map(name => {
        const args = ['clean', '--locked', '--offline', '--manifest-path', manifest, '--profile', 'dev', '--target-dir', native, '--package', name];
        const dryRun = run([...args, '--dry-run', '--verbose']);
        return {package: name, args, paths: [...new Set(validate(dryRun.stdout))].sort(), dryRun};
    });
    const reportFile = path.join(p.folder, 'native-sample-cache-retirement.json');
    const report: any = {status: 'CLEANING_NATIVE_LOCAL_OUTPUTS_AFTER_VALIDATED_SAMPLE_COPY', mode: p.buildMode,
        sample: {path: sample, copied, sha256: sampleHash}, manifest: {path: manifest, sha256: manifestHash},
        lock: {path: lock, sha256: lockHash}, skippedPackages, allowed, plans, attempts, commands: [], registry};
    const fd = io.openSync(reportFile, 'wx');
    const persist = () => {
        const bytes = Buffer.from(JSON.stringify(report, null, 2) + '\n');
        for (let at = 0; at < bytes.length;) {
            const written = io.writeSync(fd, bytes, at, bytes.length - at, at); assert(written > 0); at += written;
        }
        io.ftruncateSync(fd, bytes.length); io.fsyncSync(fd);
    };
    try {
        persist();
        check(); unchanged();
        for (const plan of plans) {
            const dryRun = run([...plan.args, '--dry-run', '--verbose']);
            assert.deepEqual([...new Set(validate(dryRun.stdout))].sort(), plan.paths, 'Native cache deletion plan changed');
            if (plan.paths.length) {
                report.commands.push(run(plan.args));
                for (const file of plan.paths) assert(!io.existsSync(file), 'Cargo left a planned native artifact');
                persist();
            }
        }
        registryUnchanged();
        for (const file of registry.current) assert.deepEqual(pin(file.path), file, 'Current Native registry closure changed before retirement');
        for (const file of registry.obsolete) {
            registryUnchanged();
            assert.deepEqual(pin(file.path), file, 'Obsolete registry artifact changed after planning');
            assert(!currentPaths.has(file.path), 'Never retire the current compiler closure');
            io.unlinkSync(file.path); registry.retired.push(file.path);
        }
        for (const file of registry.current) assert.deepEqual(pin(file.path), file, 'Current Native registry closure changed');
        check(); unchanged(); registryUnchanged(); report.status = 'NATIVE_LOCAL_OUTPUTS_RETIRED_SAMPLE_AND_SOURCE_UNCHANGED'; persist();
    } catch (error) {
        report.status = 'FAILED_WASM_BUILD_BLOCKED'; report.error = String(error); persist(); throw error;
    } finally { io.closeSync(fd); }
    return {path: reportFile, sha256: hash(reportFile)};
}
