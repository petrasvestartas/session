import path from 'node:path';
import assert from 'node:assert/strict';
import {test} from 'node:test';
import {hash} from './model.ts';
import {cleanCompilerTransition} from './compiler-cache.ts';

function fixture() {
    const viewer = path.resolve('cache-policy-fixture/viewer'), cache = path.join(viewer, 'target/course-cargo');
    const group = path.join(viewer, 'target/course-ib/compact-browser-v1'), workbench = path.join(group, 'workspace');
    const compact = path.join(viewer, 'target/course-compact'), folder = path.join(viewer, 'target/course-presentation/case');
    const nodes = new Map<string, Buffer | null>(), links = new Map<string, string>(), descriptors = new Map<number, string>();
    const calls: any[] = [], verifications: string[] = [], synced: number[] = [], closed: number[] = [], persisted: Buffer[] = [];
    const put = (file: string, data: string | Buffer | null = null) => {
        file = path.resolve(file); nodes.set(file, data === null ? null : Buffer.from(data));
        for (let parent = path.dirname(file); !nodes.has(parent); parent = path.dirname(parent)) {
            nodes.set(parent, null); if (path.dirname(parent) === parent) break;
        }
    };
    const cargo = path.join(viewer, 'tools/cargo.exe'); put(cargo, 'pinned cargo');
    put(path.join(workbench, 'Cargo.toml'), 'previous manifest'); put(path.join(workbench, 'Cargo.lock'), 'previous lock'); put(folder);
    const identity = {cargo: 'previous', tools: {rustc: 'rustc fixture\nhost: x86_64-pc-windows-msvc\n'}, policy: {profile: 'debug'}};
    const g = {key: 'compact-browser-v1', folder: group, workbench, target: cache, package: 'image_incremental', identity};
    const builtFile = path.join(viewer, 'target/course-presentation/previous/built.json');
    const before = {group: g, overlay: {cargoSha256: hash('previous manifest'), lockSha256: hash('previous lock'),
        dependencyPaths: [{absolute: path.resolve(viewer, '../session_rust')}]}};
    put(builtFile, JSON.stringify(before));
    const previous = {completedReceipt: builtFile, completedReceiptSha256: hash(nodes.get(builtFile)!)};
    const receiptFile = path.join(compact, 'browser-previous/capture.json');
    const receipt = {build: {path: builtFile, sha256: previous.completedReceiptSha256}}; put(receiptFile, JSON.stringify(receipt));
    const prepared = {id: 'next', preparedFile: path.join(folder, 'prepared.json'), folder,
        group: {...g, identity: {...identity, cargo: 'next'}}, toolFiles: {cargo: {path: cargo, sha256: hash('pinned cargo')}}};
    put(path.join(compact, 'active.lock'), JSON.stringify({pid: 123, lane: 'browser', id: 'next'}));
    put(path.join(compact, 'browser-pending.json'), JSON.stringify({pid: 123, lane: 'browser', id: 'next'}));
    put(path.join(group, 'workbench.lock'), JSON.stringify({pid: 123, id: 'next', preparedFile: prepared.preparedFile}));
    for (const name of ['sharedb7.exe', 'cpp-cache/object.o', 'registry/dependency.rlib']) put(path.join(viewer, name), 'untouched ' + name);
    put(cache);
    const controls: any = {status: 'PUBLISHED_BINARY_RETIRED', dry: null, failActual: false};
    const io: any = {
        existsSync: (file: string) => nodes.has(path.resolve(file)) || links.has(path.resolve(file)),
        readFileSync: (file: string) => { const value = nodes.get(path.resolve(file)); assert(value); return value; },
        realpathSync: (file: string) => {
            file = path.resolve(file);
            for (const [link, target] of links) if (file === link || file.startsWith(link + path.sep)) return path.join(target, path.relative(link, file));
            assert(nodes.has(file)); return file;
        },
        lstatSync: (file: string) => ({isSymbolicLink: () => links.has(path.resolve(file)), isDirectory: () => nodes.get(path.resolve(file)) === null}),
        readdirSync: (file: string) => [...new Set([...nodes.keys(), ...links.keys()].filter(name => path.dirname(name) === file).map(name => path.basename(name)))],
        openSync: (file: string, mode: string) => { assert.equal(mode, 'wx'); assert(!nodes.has(file)); const fd = descriptors.size + 1; descriptors.set(fd, file); put(file, ''); return fd; },
        writeSync: (fd: number, bytes: Buffer, offset: number, length: number, position: number) => {
            const file = descriptors.get(fd)!, old = nodes.get(file)!;
            const written = controls.shortWrites ? Math.min(length, 127) : length;
            const next = Buffer.alloc(Math.max(old.length, position + written)); old.copy(next);
            bytes.copy(next, position, offset, offset + written); put(file, next); return written;
        },
        ftruncateSync: (fd: number, length: number) => put(descriptors.get(fd)!, nodes.get(descriptors.get(fd)!)!.subarray(0, length)),
        fsyncSync: (fd: number) => { synced.push(fd); persisted.push(Buffer.from(nodes.get(descriptors.get(fd)!)!)); },
        closeSync: (fd: number) => closed.push(fd),
    };
    const run: any = (executable: string, args: string[], options: any) => {
        const dry = args.includes('--dry-run'), targetDirectory = args[args.indexOf('--target-dir') + 1], name = args[args.indexOf('--package') + 1];
        const file = path.join(targetDirectory, 'debug/deps', name + '-fixture.rlib');
        calls.push({executable, args, options, dry});
        if (dry) {
            if (!nodes.has(file)) put(file, 'compiled variant');
            return {status: 0, stdout: controls.dry ? controls.dry({file, args, calls}) : file + '\n', stderr: ''};
        }
        if (controls.failActual) return {status: 1, stdout: '', stderr: 'fixture clean failure'};
        assert.equal(JSON.parse(persisted[0].toString()).status, 'CLEANING', 'Durable intent must precede actual clean');
        if (controls.staleTail) {
            const report = path.join(folder, 'compiler-cache-transition.json');
            put(report, Buffer.concat([nodes.get(report)!, Buffer.alloc(128 * 1024, 88)]));
        }
        nodes.delete(file); return {status: 0, stdout: '', stderr: 'Removed fixture package variant'};
    };
    const runtime: any = {fs: io, run, pid: 123, verify: (file: string) => { verifications.push(file); return {status: controls.status, receipt}; }};
    const options = {previous, prepared, captures: [receiptFile], viewer};
    return {options, runtime, viewer, cache, group, compact, workbench, nodes, links, calls, controls, put, before, previous, receipt, receiptFile,
        synced, closed, persisted, verifications, report: path.join(folder, 'compiler-cache-transition.json'), run: () => cleanCompilerTransition(options, runtime)};
}

test('an unchanged dependency/profile identity is a no-op with no Cargo, ledger verification or record write', () => {
    const f = fixture(); f.options.prepared.group.identity = structuredClone(f.before.group.identity);
    assert.equal(f.run(), null); assert.equal(f.calls.length, 0); assert.equal(f.verifications.length, 0); assert(!f.nodes.has(f.report));
});

test('transition validates every plan before Cargo-scoped cleaning and records exact non-shell commands', () => {
    const f = fixture(), preserved = [...f.nodes].filter(([name]) => !name.startsWith(f.cache + path.sep));
    const result = f.run(); assert(result); assert.equal(result.path, f.report);
    assert.equal(f.calls.length, 18); assert(f.calls.slice(0, 6).every(row => row.dry));
    for (const [index, target] of ['', 'x86_64-pc-windows-msvc', 'wasm32-unknown-unknown'].entries()) for (const [offset, name] of ['image_incremental', 'session_rust'].entries()) {
        const row = f.calls[index * 2 + offset];
        assert.deepEqual(row.args, ['clean', '--locked', '--offline', '--manifest-path', path.join(f.workbench, 'Cargo.toml'),
            '--profile', 'dev', '--target-dir', path.join(f.cache, target), '--package', name, '--dry-run', '--verbose']);
        assert.equal(row.executable, f.options.prepared.toolFiles.cargo.path); assert.equal(row.options.shell, false); assert.equal(row.options.cwd, f.workbench);
    }
    assert(f.calls.filter(row => !row.dry).every(row => !row.args.includes('--dry-run') && !row.args.includes('--verbose')));
    for (const [name, value] of preserved) assert.deepEqual(f.nodes.get(name), value, name);
    const report = JSON.parse(f.nodes.get(f.report)!.toString()); assert.equal(report.status, 'PACKAGE_VARIANTS_RETIRED');
    assert.equal(report.commands.length, 6); assert.deepEqual(f.synced, [1, 1]); assert.deepEqual(f.closed, [1]);
    const intent = JSON.parse(f.persisted[0].toString()); assert.equal(intent.status, 'CLEANING'); assert.equal(intent.plans.length, 6); assert.equal(intent.commands.length, 0);
});

test('empty Cargo deletion plans succeed without any actual clean command', () => {
    const f = fixture(); f.controls.dry = () => '';
    assert(f.run()); assert.equal(f.calls.filter(row => !row.dry).length, 0);
    const report = JSON.parse(f.nodes.get(f.report)!.toString());
    assert.equal(report.status, 'PACKAGE_VARIANTS_RETIRED'); assert(report.plans.every((plan: any) => plan.paths.length === 0));
    assert.equal(report.commands.length, 0); assert.deepEqual(f.synced, [1, 1]);
});

test('positioned final persistence tolerates short writes and truncates a longer prior record without holes', () => {
    const f = fixture(); f.controls.shortWrites = true; f.controls.staleTail = true;
    assert(f.run());
    const bytes = f.nodes.get(f.report)!, report = JSON.parse(bytes.toString());
    assert.equal(report.status, 'PACKAGE_VARIANTS_RETIRED'); assert.equal(report.commands.length, 6);
    assert(!bytes.includes(0)); assert.equal(bytes.length, Buffer.byteLength(JSON.stringify(report, null, 2) + '\n'));
    assert.deepEqual(f.synced, [1, 1]); assert.deepEqual(f.closed, [1]);
});

for (const state of ['CAPTURED_BINARY_AVAILABLE', 'RETIREMENT_INCOMPLETE']) test(`a ${state} predecessor blocks all cleanup commands`, () => {
    const f = fixture(); f.controls.status = state;
    assert.throws(f.run, /published and fully retired/); assert.equal(f.calls.length, 0); assert(!f.nodes.has(f.report));
});

test('a built receipt alone or a mismatched capture binding never authorizes cleanup', () => {
    const f = fixture(); f.options.captures = []; assert.throws(f.run, /no exact sealed/); assert.equal(f.calls.length, 0);
    f.options.captures = [f.receiptFile]; f.put(f.receiptFile, JSON.stringify({build: {...f.receipt.build, sha256: '0'.repeat(64)}}));
    assert.throws(f.run, /no exact sealed/); assert.equal(f.calls.length, 0);
});

test('changed prior build bytes, old workbench inputs, or Cargo executable reject before cleanup', () => {
    for (const name of ['build', 'manifest', 'cargo']) {
        const f = fixture();
        const file = name === 'build' ? f.previous.completedReceipt : name === 'manifest' ? path.join(f.workbench, 'Cargo.toml') : f.options.prepared.toolFiles.cargo.path;
        f.put(file, 'changed bytes'); assert.throws(f.run); assert.equal(f.calls.length, 0);
    }
});

test('an external shared target directory is never accepted for automatic cleaning', () => {
    const f = fixture(); f.options.prepared.group.target = path.join(f.viewer, 'unrelated-cache');
    assert.throws(f.run, /designated shared/); assert.equal(f.calls.length, 0);
});

for (const name of ['native-pending.json', 'incremental-native-running.lock', 'workbench-pending.json', 'foreign-lane']) test(`${name} prevents a transition from touching compiler variants`, () => {
    const f = fixture();
    if (name === 'foreign-lane') f.put(path.join(f.compact, 'active.lock'), JSON.stringify({pid: 999, lane: 'browser', id: 'next'}));
    else if (name === 'workbench-pending.json') f.put(path.join(f.group, 'pending.json'), '{}');
    else if (name === 'incremental-native-running.lock') f.put(path.join(f.viewer, 'target/course-presentation', name), '');
    else f.put(path.join(f.compact, name), '{}');
    assert.throws(f.run); assert.equal(f.calls.length, 0);
});

for (const location of ['outside', 'debug-root', 'other-target', 'relative']) test(`Cargo dry-run ${location} paths reject before any deletion`, () => {
    const f = fixture();
    f.controls.dry = ({file}: any) => {
        if (location === 'outside') return path.join(f.viewer, 'sharedb7.exe') + '\n';
        if (location === 'debug-root') return path.dirname(path.dirname(file)) + '\n';
        if (location === 'other-target') return path.join(f.cache, 'wasm32-unknown-unknown/debug/deps/other.rlib') + '\n';
        return '../escape.rlib\n';
    };
    assert.throws(f.run); assert.equal(f.calls.filter(row => !row.dry).length, 0);
});

test('redirected cache ancestors and links nested under a proposed directory reject before cleanup', () => {
    const f = fixture(); f.links.set(f.cache, path.join(f.viewer, 'cpp-cache'));
    assert.throws(f.run, /redirected/); assert.equal(f.calls.length, 0);
    f.links.clear();
    const directory = path.join(f.cache, 'debug/incremental/image_incremental-fixture'); f.put(directory);
    f.links.set(path.join(directory, 'outside'), path.join(f.viewer, 'cpp-cache'));
    f.controls.dry = () => directory + '\n';
    assert.throws(f.run, /redirected/); assert.equal(f.calls.filter(row => !row.dry).length, 0);
});

test('changed ownership paths between dry run and actual clean block deletion and preserve a failed record', () => {
    const f = fixture(); f.controls.dry = ({file, calls}: any) => file + (calls.length > 6 ? '.changed' : '') + '\n';
    assert.throws(f.run, /ownership paths changed/); assert.equal(f.calls.filter(row => !row.dry).length, 0);
    assert.equal(JSON.parse(f.nodes.get(f.report)!.toString()).status, 'FAILED_COMPILE_BLOCKED'); assert.deepEqual(f.closed, [1]);
});

test('a failed actual Cargo clean propagates and stops subsequent cleanup or compilation', () => {
    const f = fixture(); f.controls.failActual = true;
    assert.throws(f.run, /fixture clean failure/); assert.equal(f.calls.filter(row => !row.dry).length, 1);
    const report = JSON.parse(f.nodes.get(f.report)!.toString());
    assert.equal(report.status, 'FAILED_COMPILE_BLOCKED'); assert.equal(report.attempts.at(-1).exitCode, 1);
    assert.equal(report.attempts.at(-1).stderr, 'fixture clean failure'); assert.deepEqual(f.synced, [1, 1]);
});

test('a previous manifest without session_rust never selects its package for cleaning', () => {
    const f = fixture(); f.before.overlay.dependencyPaths = [];
    f.put(f.previous.completedReceipt, JSON.stringify(f.before)); f.previous.completedReceiptSha256 = hash(f.nodes.get(f.previous.completedReceipt)!);
    f.receipt.build.sha256 = f.previous.completedReceiptSha256; f.put(f.receiptFile, JSON.stringify(f.receipt));
    assert(f.run()); assert.equal(f.calls.filter(row => !row.dry).length, 3);
    assert(f.calls.every(row => row.args[row.args.indexOf('--package') + 1] === 'image_incremental'));
});
