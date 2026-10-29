import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {randomUUID} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureCompact} from './compact-capture.ts';
import {verifyCaptureLedger, retirePublishedCapture} from './capture-ledger.ts';
import {adapterMetadataPath, cargoTarget, compilerTools, fileReference} from './capture-storage.ts';
import {course, docs, filesIn, hash, json, lockFor, viewer} from './model.ts';

type Entry = {lane: 'native' | 'browser'; id: string; route?: string; destination: string};
type Queue = {entries: Entry[]; retained: {destination: string; sha256: string; reason: string}[];
    special: {destination: string; id: string | null; sha256?: string; reason: string}[]};
type Reference = ReturnType<typeof fileReference>;
type Runtime = {
    viewer: string; destinations: string; state: string;
    generation: (queue: Queue) => any;
    capture: (lane: string, id: string, route?: string) => Promise<any>;
    verify: (file: string) => any;
    retire: (file: string, destination: string, publication: string) => any;
};

const script = fileURLToPath(import.meta.url);
const routes = new Set(['generic', 'additional', 'exported', 'historical', 'terminal', 'child-context', 'early-default', 'main-page', 'streaming', 'fallback', 'source-status']);
const aliases: Record<string, string> = {'retained-streaming': 'streaming', 'source-main-page': 'main-page', 'retained-maintained': 'maintained'};

function destination(file: string, runtime: Runtime) {
    const root = path.resolve(runtime.destinations), result = path.resolve(runtime.viewer, file);
    assert.equal(fs.realpathSync(root), root, 'Public image directory is redirected');
    assert.equal(path.dirname(result), root, 'Only explicit journey screenshot PNGs may be published');
    assert.equal(path.extname(result).toLowerCase(), '.png');
    if (fs.existsSync(result)) assert(fs.lstatSync(result).isFile() && !fs.lstatSync(result).isSymbolicLink(), 'Public image is not an ordinary file');
    return result;
}

function listedImage(file: string, runtime: Runtime) {
    const result = path.resolve(runtime.viewer, file), relative = path.relative(path.join(runtime.viewer, 'docs'), result);
    assert(relative && !relative.startsWith('..') && !path.isAbsolute(relative), 'Listed image must stay inside documentation');
    assert.equal(path.extname(result).toLowerCase(), '.png');
    if (fs.existsSync(result)) assert.equal(fs.realpathSync(result), result, 'Listed image is redirected');
    return result;
}

export function readBatchQueue(file: string, runtime: Runtime = production): Queue {
    const input = json(file), rows = Array.isArray(input) ? input : input.entries;
    assert(Array.isArray(rows) && rows.length, 'Supply an explicit nonempty entries array');
    const entries = rows.map(row => {
        assert(['native', 'browser'].includes(row.lane) && typeof row.id === 'string' && row.id.length);
        const route = aliases[row.route] || row.route || 'generic';
        assert(row.lane !== 'browser' || row.id !== '97-finish-with-the-maintained-browser-runtime' || route === 'maintained', 'Final maintained runtime must have its explicit maintained route');
        return {lane: row.lane, id: row.id, route, destination: destination(row.destination, runtime)};
    });
    const retained = (input.retained || []).map((row: any) => {
        assert(/^[a-f0-9]{64}$/.test(row.sha256) && typeof row.reason === 'string' && row.reason.length, 'Retained scope requires its exact hash and reason');
        return {destination: listedImage(row.destination, runtime), sha256: row.sha256, reason: row.reason};
    });
    const special = (input.special || []).map((row: any) => {
        assert(!row.sha256 || /^[a-f0-9]{64}$/.test(row.sha256));
        return {destination: listedImage(row.destination || row.path, runtime), id: row.id || row.checkpoint || null,
            ...(row.sha256 ? {sha256: row.sha256} : {}), reason: row.reason || 'Requires a separate supporting or variant capture helper'};
    });
    const names = [...entries, ...retained, ...special].map(row => process.platform === 'win32' ? row.destination.toLowerCase() : row.destination);
    assert.equal(new Set(names).size, names.length, 'Destinations must be unique and disjoint from retained scope');
    return {entries, retained, special};
}

function sourceGeneration(queue: Queue) {
    const steps = course().steps, ids = new Set(steps.map(step => step.id));
    for (const row of queue.entries) assert(ids.has(row.id), 'Unknown checkpoint: ' + row.id);
    const roots = [path.dirname(script), path.join(docs, 'journey/code'), path.join(docs, 'journey/checks'),
        ...[viewer, path.resolve(viewer, '../session_rust'), path.resolve(viewer, '../../wood/rust')].flatMap(root => ['src', 'assets'].map(name => path.join(root, name)))].filter(fs.existsSync);
    const directories = roots.map(root => ({root, names: filesIn(root).sort()}));
    const inputs = directories.flatMap(directory => directory.names);
    inputs.push(path.join(docs, 'journey/course.json'), path.join(docs, 'journey/frame.rs'), path.join(docs, 'capture_journey.cjs'));
    for (const row of queue.entries) {
        inputs.push(lockFor(row.id));
        const step = steps.find(step => step.id === row.id)!;
        for (const file of [step.frame, step.browser_check]) if (file) inputs.push(path.join(docs, file));
        const module = path.basename(step.browser_check || '', '.cjs');
        if (row.lane === 'browser' && ['caps', 'performance'].includes(module)) inputs.push(path.join(docs, 'journey/release',
            module === 'caps' ? 'exported88ad-browser-image-review.json' : 'exported-primary-three-browser-image-review.json'));
    }
    for (const root of [viewer, path.resolve(viewer, '../session_rust'), path.resolve(viewer, '../../wood/rust')])
        for (const name of ['Cargo.toml', 'Cargo.lock', 'build.rs', 'Trunk.toml', 'index.html']) if (fs.existsSync(path.join(root, name))) inputs.push(path.join(root, name));
    for (const name of ['typescript.mjs', 'run-guarded.cjs', 'run-guarded.ps1']) inputs.push(fileURLToPath(new URL('../' + name, import.meta.url)));
    for (const name of ['VIEWER_DXC_PATH', 'VIEWER_BROWSER_EXECUTABLE', 'COURSE_ADAPTER_METADATA']) if (process.env[name]) inputs.push(path.resolve(process.env[name]!));
    if (queue.entries.some(row => row.lane === 'browser')) {
        inputs.push(adapterMetadataPath());
        inputs.push(process.env.VIEWER_BROWSER_EXECUTABLE || path.join(viewer, 'target/course-browsers/chromium_headless_shell-1208/chrome-headless-shell-win64/chrome-headless-shell.exe'));
    }
    if (process.env.VIEWER_DXC_PATH) inputs.push(path.join(path.dirname(process.env.VIEWER_DXC_PATH), 'dxil.dll'));
    for (const match of (process.env.NODE_OPTIONS || '').matchAll(/(?:--require|-r)(?:=|\s+)(?:"([^"]+)"|'([^']+)'|(\S+))/g)) inputs.push(path.resolve(match[1] || match[2] || match[3]));
    const environment = Object.fromEntries(['RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'CARGO_INCREMENTAL', 'COURSE_CAPTURE_PROCESSORS', 'VIEWER_DXC_PATH',
        'VIEWER_BROWSER_EXECUTABLE', 'COURSE_ADAPTER_METADATA', 'NODE_OPTIONS'].map(name => [name, process.env[name] || null]));
    return {files: [...new Set(inputs)].sort().map(fileReference), directories,
        profile: {environment, target: cargoTarget(), compiler: compilerTools(), node: fileReference(process.execPath)}};
}

const production: Runtime = {viewer, destinations: path.join(docs, 'screenshots/journey'), state: path.join(viewer, 'target/course-compact-batches'),
    generation: sourceGeneration, capture: captureCompact, verify: verifyCaptureLedger, retire: retirePublishedCapture};

function writeOnce(file: string, value: any) {
    const bytes = JSON.stringify(value, null, 2) + '\n';
    fs.mkdirSync(path.dirname(file), {recursive: true});
    assert.equal(fs.realpathSync(path.dirname(file)), path.resolve(path.dirname(file)), 'Evidence directory is redirected');
    if (fs.existsSync(file)) { assert.equal(fs.readFileSync(file, 'utf8'), bytes, 'Immutable batch evidence changed'); return fileReference(file); }
    const temporary = file + '.' + randomUUID() + '.tmp', fd = fs.openSync(temporary, 'wx');
    try { fs.writeFileSync(fd, bytes); fs.fsyncSync(fd); }
    finally { fs.closeSync(fd); }
    fs.renameSync(temporary, file);
    return fileReference(file);
}

function assertReference(reference: Reference) {
    assert.equal(fileReference(reference.path).sha256, reference.sha256, 'Recorded evidence changed: ' + reference.path);
}

function assertSource(source: any) {
    for (const directory of source.directories || []) assert.deepEqual(filesIn(directory.root).sort(), directory.names, 'Source file set changed during batch');
    for (const file of source.files) assertReference(file);
    for (const file of Object.values(source.profile?.compiler || {}) as Reference[]) assertReference(file);
    if (source.profile?.node) assertReference(source.profile.node);
    for (const [name, value] of Object.entries(source.profile?.environment || {})) assert.equal(process.env[name] || null, value, 'Capture environment changed during batch');
}

function publish(image: Reference, output: string, previous: Reference | null, runtime: Runtime) {
    output = destination(output, runtime); assertReference(image);
    const bytes = fs.readFileSync(image.path);
    assert(bytes.length > 24 && bytes.subarray(0, 8).equals(Buffer.from([137, 80, 78, 71, 13, 10, 26, 10])), 'Capture must be a PNG');
    assert(bytes.readUInt32BE(16) > 0 && bytes.readUInt32BE(20) > 0);
    if (fs.existsSync(output) && fileReference(output).sha256 === image.sha256) return fileReference(output);
    if (previous) { assert.equal(previous.path, output); assertReference(previous); }
    else assert(!fs.existsSync(output), 'Destination appeared after capture started');
    const temporary = path.join(path.dirname(output), '.' + path.basename(output) + '.' + randomUUID() + '.tmp');
    const fd = fs.openSync(temporary, 'wx');
    try { fs.writeFileSync(fd, bytes); fs.fsyncSync(fd); }
    finally { fs.closeSync(fd); }
    try {
        assert.equal(fileReference(temporary).sha256, image.sha256);
        destination(output, runtime);
        if (previous) assertReference(previous); else assert(!fs.existsSync(output));
        fs.renameSync(temporary, output);
    } finally { if (fs.existsSync(temporary)) fs.unlinkSync(temporary); }
    assert.equal(fileReference(output).sha256, image.sha256);
    return fileReference(output);
}

export async function runCompactBatch(queueFile: string, options: {limit?: number; statusOnly?: boolean} = {}, runtime: Runtime = production) {
    const queue = readBatchQueue(queueFile, runtime), source = runtime.generation(queue);
    const generation = hash(JSON.stringify({queue, source})), directory = path.join(runtime.state, generation);
    const limit = options.limit ?? Infinity; assert(limit === Infinity || Number.isInteger(limit) && limit >= 0);
    fs.mkdirSync(runtime.state, {recursive: true});
    assert.equal(fs.realpathSync(runtime.state), path.resolve(runtime.state));
    const lock = path.join(runtime.state, 'batch.lock'), descriptor = fs.openSync(lock, 'wx');
    fs.writeFileSync(descriptor, JSON.stringify({pid: process.pid, generation})); fs.closeSync(descriptor);
    const previousAttempt = process.env.COURSE_CAPTURE_ATTEMPT;
    try {
        const manifest = writeOnce(path.join(directory, 'manifest.json'), {schema: 1, generation, queue, source});
        const eventsFolder = path.join(directory, 'events'); fs.mkdirSync(eventsFolder, {recursive: true});
        const events: any[] = [], states: Record<number, any> = {};
        let previous: string | null = null;
        for (const name of fs.readdirSync(eventsFolder).filter(name => name.endsWith('.json')).sort()) {
            const file = path.join(eventsFolder, name), event = json(file), sha256 = fileReference(file).sha256;
            assert.equal(event.sequence, events.length + 1); assert.equal(event.previous, previous); assert.equal(event.generation, generation);
            assert.equal(name, `${String(event.sequence).padStart(6, '0')}-${sha256}.json`);
            events.push(event); previous = sha256;
            states[event.index] = {...states[event.index], ...event.data};
        }
        const append = (index: number, data: any) => {
            const event = {sequence: events.length + 1, previous, generation, created: new Date().toISOString(), index, data};
            const sha256 = hash(JSON.stringify(event, null, 2) + '\n');
            writeOnce(path.join(eventsFolder, `${String(event.sequence).padStart(6, '0')}-${sha256}.json`), event);
            events.push(event); previous = sha256; states[index] = {...states[index], ...data};
        };
        const unsupported = queue.entries.flatMap((entry, index) => entry.route === 'maintained' || entry.lane === 'browser' && !routes.has(entry.route!)
            ? [{index, id: entry.id, route: entry.route, destination: entry.destination, reason: 'Requires a separately qualified capture helper; no generic substitution'}] : [])
            .concat(queue.special.map((entry, index) => ({index: queue.entries.length + index, ...entry, route: 'separate-helper'})));
        const current = new Set<number>();
        const summary = () => {
            const rows = Object.values(states), retired = rows.filter(row => row.retired).length;
            const total = queue.entries.length + queue.special.length;
            return {generation, directory, total, standardEntries: queue.entries.length, captured: rows.filter(row => row.capture).length,
                published: rows.filter(row => row.publication).length, retired, verifiedCurrent: current.size, remaining: total - current.size,
                retainedUnchanged: queue.retained.length, unsupported, visualReview: 'PENDING_INDEPENDENT_REVIEW',
                status: current.size === total ? 'COMPLETE_EXECUTION_PUBLICATION_RETIREMENT' : 'CHECKPOINT_PENDING'};
        };
        for (const row of queue.retained) assert.equal(fileReference(row.destination).sha256, row.sha256, 'Explicit unchanged image changed');
        for (const [index, row] of Object.entries(states)) if (row.retired) {
            try {
                assertReference(row.capture); assertReference(row.publication);
                const verified = runtime.verify(row.capture.path);
                assert.equal(verified.status, 'PUBLISHED_BINARY_RETIRED');
                assert.equal(fileReference(queue.entries[Number(index)].destination).sha256, verified.receipt.image.sha256, 'Completed publication is no longer current');
                current.add(Number(index));
            } catch (error) {
                return {...summary(), status: 'STOPPED_ON_FAILURE', failed: {index: Number(index), id: queue.entries[Number(index)].id, error: String(error)}};
            }
        }
        if (options.statusOnly) return summary();
        let completed = 0;
        for (const [index, entry] of queue.entries.entries()) {
            if (states[index]?.retired || unsupported.some(row => row.index === index)) continue;
            if (completed >= limit) break;
            try {
                assertSource(source);
                let state = states[index];
                if (!state?.capture) {
                    assert(!state?.started, 'Interrupted capture has no journaled receipt; preserve pending evidence and diagnose before retry');
                    append(index, {started: true, attempt: generation.slice(0, 20) + '-' + index + '-' + randomUUID(),
                        previousDestination: fs.existsSync(entry.destination) ? fileReference(entry.destination) : null});
                    state = states[index]; process.env.COURSE_CAPTURE_ATTEMPT = state.attempt;
                    const captured = await runtime.capture(entry.lane, entry.id, entry.route);
                    assertReference(captured.receipt);
                    append(index, {capture: captured.receipt}); state = states[index];
                }
                assertReference(state.capture);
                const verified = runtime.verify(state.capture.path), image = verified.receipt.image;
                assertSource(source);
                if (!state.publication) {
                    const output = publish(image, entry.destination, state.previousDestination, runtime);
                    const publication = writeOnce(path.join(directory, 'publications', String(index).padStart(6, '0') + '.json'),
                        {status: 'PUBLISHED_EXECUTION_QUALIFIED_VISUAL_REVIEW_PENDING', generation, manifest, entry, capture: state.capture,
                            image, destination: output, previousDestination: state.previousDestination, visualReview: 'PENDING_INDEPENDENT_REVIEW'});
                    append(index, {publication}); state = states[index];
                } else {
                    assertReference(state.publication);
                    assert.equal(fileReference(entry.destination).sha256, image.sha256, 'Pending retirement publication changed');
                }
                const retired = runtime.retire(state.capture.path, entry.destination, state.publication.path);
                assert.equal(retired.status, 'PUBLISHED_BINARY_RETIRED');
                assert.equal(fileReference(entry.destination).sha256, image.sha256);
                append(index, {retired: true}); current.add(index); completed++;
            } catch (error) {
                append(index, {failure: {at: new Date().toISOString(), message: String((error as Error).stack || error)}});
                return {...summary(), status: 'STOPPED_ON_FAILURE', failed: {index, id: entry.id, error: String(error)}};
            }
        }
        return summary();
    } finally {
        if (previousAttempt === undefined) delete process.env.COURSE_CAPTURE_ATTEMPT; else process.env.COURSE_CAPTURE_ATTEMPT = previousAttempt;
        assert.equal(json(lock).pid, process.pid); fs.unlinkSync(lock);
    }
}

if (process.argv[1] && path.resolve(process.argv[1]) === script) {
    const [mode, queue, ...args] = process.argv.slice(2), at = args.indexOf('--limit');
    assert(['run', 'status'].includes(mode) && queue, 'run|status QUEUE_JSON [--limit N]');
    assert(args.length === 0 || args.length === 2 && args[0] === '--limit', 'Unknown batch argument');
    const result = await runCompactBatch(path.resolve(queue), {statusOnly: mode === 'status', limit: at < 0 ? undefined : Number(args[at + 1])});
    console.log(JSON.stringify(result, null, 2));
    if (result.status === 'STOPPED_ON_FAILURE') process.exitCode = 1;
}
