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
    execute('sample', ['cargo', 'run', '--example', 'sample', '--locked', '--target', host, '-j8']);
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
    const plans = selected.map(name => {
        const args = ['clean', '--locked', '--offline', '--manifest-path', manifest, '--profile', 'dev', '--target-dir', native, '--package', name];
        const dryRun = run([...args, '--dry-run', '--verbose']);
        return {package: name, args, paths: [...new Set(validate(dryRun.stdout))].sort(), dryRun};
    });
    const reportFile = path.join(p.folder, 'native-sample-cache-retirement.json');
    const report: any = {status: 'CLEANING_NATIVE_LOCAL_OUTPUTS_AFTER_VALIDATED_SAMPLE_COPY', mode: p.buildMode,
        sample: {path: sample, copied, sha256: sampleHash}, manifest: {path: manifest, sha256: manifestHash},
        lock: {path: lock, sha256: lockHash}, skippedPackages, allowed, plans, attempts, commands: []};
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
        check(); unchanged(); report.status = 'NATIVE_LOCAL_OUTPUTS_RETIRED_SAMPLE_AND_SOURCE_UNCHANGED'; persist();
    } catch (error) {
        report.status = 'FAILED_WASM_BUILD_BLOCKED'; report.error = String(error); persist(); throw error;
    } finally { io.closeSync(fd); }
    return {path: reportFile, sha256: hash(reportFile)};
}
