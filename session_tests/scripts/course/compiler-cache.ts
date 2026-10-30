import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {isDeepStrictEqual} from 'node:util';
import {gunzipSync} from 'node:zlib';
import {hash} from './model.ts';
import {verifyCaptureLedger} from './capture-ledger.ts';

type Runtime = {fs: typeof fs; run: typeof spawnSync; verify: typeof verifyCaptureLedger; pid: number};
const production: Runtime = {fs, run: spawnSync, verify: verifyCaptureLedger, pid: process.pid};

export function cleanCompilerTransition(options: {previous: any; prepared: any; captures: string[]; viewer: string}, runtime: Runtime = production) {
    const io = runtime.fs, {previous, prepared: p} = options, g = p.group;
    const bytes = (file: string) => io.readFileSync(file);
    const reference = (file: string) => ({path: path.resolve(file), sha256: hash(bytes(file))});
    const read = (file: string) => JSON.parse((file.endsWith('.gz') ? gunzipSync(bytes(file)) : bytes(file)).toString('utf8'));
    assert(previous, 'Cache cleanup requires a previous completed build');
    assert.equal(hash(bytes(previous.completedReceipt)), previous.completedReceiptSha256);
    const before = read(previous.completedReceipt);
    if (isDeepStrictEqual(before.group.identity, g.identity)) return null;
    assert.equal(before.group.key, 'compact-browser-v1'); assert.equal(g.key, before.group.key);
    assert.equal(g.package, 'image_incremental'); assert.equal(before.group.package, g.package);
    const viewer = path.resolve(options.viewer), cache = path.join(viewer, 'target/course-cargo');
    assert.equal(path.resolve(g.target), cache, 'Only the designated shared course Cargo cache may be cleaned');
    assert.equal(path.resolve(before.group.target), cache);
    assert.equal(path.resolve(g.folder), path.join(viewer, 'target/course-ib/compact-browser-v1'));
    assert.equal(path.resolve(g.workbench), path.join(viewer, 'target/course-ib/compact-browser-v1/workspace'));
    assert.equal(path.resolve(before.group.workbench), path.resolve(g.workbench));
    const safe = (file: string) => {
        file = path.resolve(file);
        let current = file;
        while (!io.existsSync(current)) { const parent = path.dirname(current); assert.notEqual(parent, current); current = parent; }
        assert.equal(io.realpathSync(current), current, 'Compiler cache path or ancestor is redirected');
        for (let current = file; io.existsSync(current); current = path.dirname(current)) {
            assert(!io.lstatSync(current).isSymbolicLink(), 'Compiler cache cannot traverse links');
            if (path.dirname(current) === current) break;
        }
        return file;
    };
    safe(cache); safe(g.workbench);
    assert.equal(hash(bytes(path.join(g.workbench, 'Cargo.toml'))), before.overlay.cargoSha256);
    assert.equal(hash(bytes(path.join(g.workbench, 'Cargo.lock'))), before.overlay.lockSha256);
    const matches = options.captures.filter(file => {
        const receipt = read(file);
        return receipt.build?.path === previous.completedReceipt && receipt.build.sha256 === previous.completedReceiptSha256;
    });
    assert(matches.length, 'Previous built.json has no exact sealed capture ledger');
    const predecessor = matches.map(file => ({file, reference: reference(file), verified: runtime.verify(file)}));
    for (const {verified} of predecessor) {
        assert.equal(verified.status, 'PUBLISHED_BINARY_RETIRED', 'Previous capture must be published and fully retired');
        assert.equal(verified.receipt.build.path, previous.completedReceipt);
        assert.equal(verified.receipt.build.sha256, previous.completedReceiptSha256);
    }
    const compact = path.join(viewer, 'target/course-compact');
    const quiescent = () => {
        const lane = read(path.join(compact, 'active.lock')), workbench = read(path.join(g.folder, 'workbench.lock'));
        assert.equal(lane.pid, runtime.pid); assert.equal(lane.lane, 'browser'); assert.equal(lane.id, p.id);
        assert.equal(workbench.pid, runtime.pid); assert.equal(workbench.id, p.id); assert.equal(workbench.preparedFile, p.preparedFile);
        const pending = read(path.join(compact, 'browser-pending.json'));
        assert.equal(pending.pid, runtime.pid); assert.equal(pending.id, p.id); assert.equal(pending.lane, 'browser');
        for (const file of [path.join(compact, 'native-pending.json'), path.join(g.folder, 'pending.json'),
            path.join(viewer, 'target/course-presentation/incremental-native-running.lock')])
            assert(!io.existsSync(file), 'Another active or unsealed compiler consumer exists: ' + file);
        const batch = path.join(viewer, 'target/course-compact-batches/batch.lock');
        if (io.existsSync(batch)) assert.equal(read(batch).pid, runtime.pid);
        for (const {reference: saved} of predecessor) assert.equal(reference(saved.path).sha256, saved.sha256);
        assert.equal(hash(bytes(previous.completedReceipt)), previous.completedReceiptSha256);
        assert.equal(hash(bytes(path.join(g.workbench, 'Cargo.toml'))), before.overlay.cargoSha256);
        assert.equal(hash(bytes(path.join(g.workbench, 'Cargo.lock'))), before.overlay.lockSha256);
    };
    quiescent();
    const host = before.group.identity.tools.rustc.match(/^host: ([A-Za-z0-9_-]+)$/m)?.[1];
    assert(host, 'Previous native helper target is not recorded');
    const packages = ['image_incremental'];
    if (before.overlay.dependencyPaths.some((item: any) => path.resolve(item.absolute) === path.resolve(viewer, '../session_rust'))) packages.push('session_rust');
    const cargo = p.toolFiles.cargo; assert.equal(reference(cargo.path).sha256, cargo.sha256);
    const attempts: any[] = [];
    const command = (args: string[]) => {
        quiescent();
        assert.equal(reference(cargo.path).sha256, cargo.sha256);
        const result = runtime.run(cargo.path, args, {cwd: g.workbench, encoding: 'utf8', windowsHide: true, shell: false,
            maxBuffer: 32 * 1024 * 1024, env: {...process.env, CARGO_TARGET_DIR: cache, CARGO_NET_OFFLINE: 'true', CARGO_TERM_COLOR: 'never'}});
        const record = {executable: cargo.path, arguments: args, stdout: String(result.stdout), stderr: String(result.stderr), exitCode: result.status, error: result.error ? String(result.error) : null};
        attempts.push(record);
        assert(!result.error, String(result.error)); assert.equal(result.status, 0, String(result.stderr));
        return record;
    };
    const validate = (text: string, allowed: string) => {
        safe(allowed);
        const paths = text.split(/\r?\n/).filter(line => line.trim()).map(line => {
            assert.equal(line, line.trim(), 'Unexpected whitespace in Cargo deletion path');
            assert(path.isAbsolute(line), 'Cargo dry run must list absolute deletion paths');
            const file = safe(line), relative = path.relative(allowed, file);
            assert(relative && relative !== '..' && !relative.startsWith('..' + path.sep) && !path.isAbsolute(relative), 'Cargo clean escapes its selected debug cache');
            const inspect = (file: string) => {
                safe(file);
                if (io.existsSync(file) && io.lstatSync(file).isDirectory()) for (const entry of io.readdirSync(file)) inspect(path.join(file, String(entry)));
            };
            inspect(file); return file;
        });
        return [...new Set(paths)].sort();
    };
    const plans = [];
    for (const target of ['', host, 'wasm32-unknown-unknown']) for (const name of packages) {
        const targetDirectory = path.join(cache, target), allowed = path.join(targetDirectory, 'debug');
        const args = ['clean', '--locked', '--offline', '--manifest-path', path.join(g.workbench, 'Cargo.toml'),
            '--profile', 'dev', '--target-dir', targetDirectory, '--package', name];
        const dryRun = command([...args, '--dry-run', '--verbose']), paths = validate(dryRun.stdout, allowed);
        plans.push({target, package: name, allowed, args, dryRun, paths});
    }
    const file = path.join(p.folder, 'compiler-cache-transition.json'); assert(!io.existsSync(file), 'Preserve the prior cleanup attempt');
    const report: any = {status: 'CLEANING', previousBuild: reference(previous.completedReceipt),
        previousIdentity: before.group.identity, nextIdentity: g.identity, predecessors: predecessor.map(item => item.reference), plans, attempts, commands: []};
    const fd = io.openSync(file, 'wx');
    const persist = () => {
        const bytes = Buffer.from(JSON.stringify(report, null, 2) + '\n');
        for (let at = 0; at < bytes.length;) {
            const written = io.writeSync(fd, bytes, at, bytes.length - at, at); assert(written > 0); at += written;
        }
        io.ftruncateSync(fd, bytes.length); io.fsyncSync(fd);
    };
    let error;
    try {
        persist();
        for (const plan of plans) {
            const dryRun = command([...plan.args, '--dry-run', '--verbose']);
            assert.deepEqual(validate(dryRun.stdout, plan.allowed), plan.paths, 'Cargo ownership paths changed after preflight');
            if (plan.paths.length) {
                report.commands.push(command(plan.args));
                for (const file of plan.paths) assert(!io.existsSync(file), 'Cargo left a planned package artifact present: ' + file);
            }
        }
        quiescent(); report.status = 'PACKAGE_VARIANTS_RETIRED';
    } catch (caught) { error = caught; report.status = 'FAILED_COMPILE_BLOCKED'; report.error = String(caught); }
    finally { try { persist(); } finally { io.closeSync(fd); } }
    if (error) throw error;
    return reference(file);
}
