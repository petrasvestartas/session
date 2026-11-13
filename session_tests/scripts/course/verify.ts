import {cargoTarget} from './capture-storage.ts';
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {course, docs, viewer, read, json, write, hash, fingerprint, expected, materialize, filesIn, reference, lockFor, compare, assetsFor, withExpectedSequence, resolveJourneyImage, type Step} from './model.ts';
import {png, ppm} from './png.ts';
import {dependencyInputs} from './dependency-inputs.ts';

export const evidence = path.join(viewer, 'target/course-checks');
const pictures = path.join(docs, 'screenshots/journey');
let host: string;

export function nativeTarget() {
    if (!host) {
        const result = spawnSync('rustc', ['-vV'], {encoding: 'utf8'});
        host = result.stdout?.match(/^host: (.+)$/m)?.[1]?.trim() || '';
        if (result.error || result.status !== 0 || !host) throw Error('Cannot determine the installed Rust host target');
    }
    return host;
}

export function signature(id: string) {
    const step = course().steps.find(step => step.id === id)!;
    const files = expected(id);
    const kernel = path.join(viewer, '../session_rust');
    const inputs = files['Cargo.toml'].includes('session_rust')
        ? [...filesIn(path.join(kernel, 'src')), path.join(kernel, 'Cargo.toml'), path.join(kernel, 'build.rs')] : [];
    return fingerprint({...files, ...dependencyInputs(files['Cargo.toml']), 'Cargo.lock': read(lockFor(id)),
        ...Object.fromEntries(assetsFor(id).map(name => [name, hash(fs.readFileSync(path.join(viewer, name)))])),
        kernel: fingerprint(Object.fromEntries(inputs.map(file => [path.relative(kernel, file), hash(fs.readFileSync(file))]))),
        constructor: step.frame_constructor || '',
        frame_mode: step.frame_mode || 'triangle',
        frame_size: JSON.stringify(step.frame_size || [640, 480]),
        ...(step.frame ? {frame: read(path.join(docs, step.frame))} : {}),
        harness: read(path.join(docs, 'journey/frame.rs')), checker: read(fileURLToPath(import.meta.url))});
}

export function runCommand(name: string, args: string[], directory: string, extra: Record<string, string> = {}) {
    fs.mkdirSync(evidence, {recursive: true});
    const log = path.join(evidence, `${name}.log`);
    const descriptor = fs.openSync(log, 'w');
    const env: Record<string, string | undefined> = {...process.env, RUSTC_WRAPPER: '', CARGO_NET_OFFLINE: process.env.CARGO_NET_OFFLINE || 'true', NO_COLOR: 'true',
        CARGO_BUILD_JOBS: '1', RUST_TEST_THREADS: '1', REGEN_PROTO: '0',
        CARGO_TARGET_DIR: cargoTarget(), ...extra};
    let result;
    try {
        if (process.platform === 'win32') {
            const guard = fileURLToPath(new URL('../run-guarded.ps1', import.meta.url));
            const command = path.join(evidence, `${name}.command.json`);
            write(command, JSON.stringify({executable: args[0], arguments: args.slice(1)}) + '\n');
            result = spawnSync('powershell.exe', ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', guard,
                '-ManifestPath', path.join(evidence, `${name}.resources.json`), '-WorkingDirectory', directory,
                '-TimeoutSeconds', env.COURSE_TIMEOUT_SECONDS || '600',
                '-MemoryLimitMiB', env.COURSE_MEMORY_MIB || '2048', '-CommandFile', command],
                {cwd: directory, env, stdio: ['ignore', descriptor, descriptor]});
        } else {
            result = spawnSync('buildslot', ['timeout', '10m', 'prlimit', '--data=6442450944', '--', ...args],
                {cwd: directory, env, stdio: ['ignore', descriptor, descriptor]});
        }
    } finally {
        fs.closeSync(descriptor);
    }
    if (result.error || result.status !== 0) throw Error(`${name} failed. Read ${log}\n${read(log).slice(-4000)}`);
    return {name, args, returncode: result.status};
}

export function removeGenerated(directory: string, root: string) {
    const within = (parent: string, child: string) => {
        const relative = path.relative(parent, child);
        return relative !== '' && relative !== '..' && !relative.startsWith('..' + path.sep) && !path.isAbsolute(relative);
    };
    const actualRoot = fs.realpathSync(root);
    assert(within(fs.realpathSync(viewer), actualRoot), 'Generated root must remain inside the viewer workspace');
    assert(within(actualRoot, path.resolve(directory)), 'Cleanup must remain inside its generated root');
    let existing = directory;
    while (!fs.existsSync(existing)) existing = path.dirname(existing);
    const actual = fs.realpathSync(existing);
    assert(actual === actualRoot || within(actualRoot, actual), 'Cleanup cannot follow a link outside its generated root');
    fs.rmSync(directory, {recursive: true, force: true});
}

export function verificationCheckpoints(ids: string[], stored = false, all = false) {
    const known = course().steps.map(step => step.id);
    if (ids.some(id => !known.includes(id))) throw Error('Unknown checkpoint');
    if (ids.length) return [...new Set(ids)];
    if (all || stored) return known;
    throw Error('Choose verification checkpoint IDs or --all');
}

export function webEvidence(id: string, record: any, source: string) {
    assert.equal(record?.source, source, `${id}: web evidence is stale or missing`);
    assert(record.commands.every((command: {returncode: number}) => command.returncode === 0));
    assert(record.commands.some((command: {name: string}) => command.name === `${id}-wasm`));
    assert(record.commands.some((command: {name: string; args: string[]}) =>
        command.name === `${id}-web` && command.args.includes('--release')), `${id}: web bundle must be optimized`);
    return {...record, commands: [...record.commands]};
}

export function verify(ids: string[], stored = false, all = process.env.COURSE_VERIFY_ALL === '1', phase: 'all' | 'web' | 'native' = 'all') {
    assert(!stored || phase === 'all', 'Stored acceptance checks require all phases');
    const report = path.join(evidence, 'results.json');
    const records = fs.existsSync(report) ? json(report) : {};
    const pendingFile = path.join(evidence, 'web-pending.json');
    const pending = fs.existsSync(pendingFile) ? json(pendingFile) : {};
    const selected = verificationCheckpoints(ids, stored, all);
    for (const id of selected) {
        const step = course().steps.find(step => step.id === id)!;
        assert(/^[a-z0-9-]+$/.test(id), 'Invalid checkpoint ID');
        if (stored) {
            const record = records[id];
            assert.equal(record?.source, signature(id), `${id}: build evidence is stale or missing`);
            assert(record.commands.every((command: {returncode: number}) => command.returncode === 0));
            assert(record.commands.some((command: {name: string}) => command.name === `${id}-wasm`));
            assert(record.commands.some((command: {name: string; args: string[]}) =>
                command.name === `${id}-web` && command.args.includes('--release')), `${id}: web bundle must be optimized`);
            if (step.tests) assert(record.commands.some((command: {name: string}) => command.name === `${id}-tests`));
            assert(record.pixels, `${id}: native pixel verification is required`);
            if (id === '04-input') assert(record.roundtrip);
            console.log(`${id}: evidence current`);
            continue;
        }
        delete records[id];
        write(report, JSON.stringify(records, null, 2) + '\n');
        const target = path.join(viewer, 'target', `course-${id}`);
        const source = signature(id);
        const record: any = phase === 'native' ? webEvidence(id, pending[id], source)
            : {source, commands: [], browser: 'not verified'};
        removeGenerated(target, path.join(viewer, 'target'));
        try {
            materialize(expected(id), target, lockFor(id), assetsFor(id));
            assert.deepEqual(compare(expected(id), target), [], `${id}: the build must use the displayed source`);
            if (phase !== 'native') {
                record.commands.push(runCommand(`${id}-wasm`, ['cargo', 'build', '--lib', '--locked', '--target', 'wasm32-unknown-unknown', '-j1'], target));
                record.commands.push(runCommand(`${id}-web`, ['trunk', 'build', '--release', '--public-url', './'], target));
                const bundle = path.join(evidence, id, 'dist');
                removeGenerated(bundle, evidence);
                if (process.env.COURSE_KEEP_BUNDLES === '1') {
                    fs.mkdirSync(path.dirname(bundle), {recursive: true});
                    fs.cpSync(path.join(target, 'dist'), bundle, {recursive: true});
                }
                assert.equal(signature(id), source, `${id}: source changed during web verification`);
                pending[id] = webEvidence(id, record, source);
                write(pendingFile, JSON.stringify(pending, null, 2) + '\n');
            }
            if (phase === 'web') {
                console.log(`${id}: WebAssembly and release bundle passed; native acceptance pending`);
                continue;
            }
            if (step.tests) record.commands.push(runCommand(`${id}-tests`, ['cargo', 'test', '--lib', '--locked', '--target', nativeTarget(), '-j1'], target));
            {
                const args = step.frame_mode === 'clear' ? 'device, queue' : 'device, queue, format';
                const constructor = step.frame_constructor || `Renderer::new(${args})`;
                const [width, height] = step.frame_size || [640, 480];
                assert([width, height].every(value => Number.isInteger(value) && value > 0 && value <= 4096));
                let code = read(path.join(docs, 'journey/frame.rs'))
                    .replace('// COURSE_SIZE', `const WIDTH: u32 = ${width};\nconst HEIGHT: u32 = ${height};`)
                    .replace('// COURSE_NEW', `let renderer = ${constructor};`);
                let draw = 'renderer.draw(&view);';
                if (id === '04-input') {
                    draw = 'let mut background = viewer_journey::background::Background::default();\n' +
                        '    let toggles: u32 = std::env::args().nth(1).unwrap_or("0".into()).parse()?;\n' +
                        '    for _ in 0..toggles {\n        background.toggle();\n    }\n' +
                        '    renderer.draw(&view, &background);';
                }
                if (step.frame) {
                    draw = 'draw(&renderer, &view);';
                    code = code.slice(0, code.indexOf('fn verify_pixels(')) + read(path.join(docs, step.frame));
                }
                code = code.replace('// COURSE_DRAW', draw);
                write(path.join(target, 'examples/frame.rs'), code);
                const command = ['cargo', 'run', '--example', 'frame', '--locked', '--target', nativeTarget(), '-j1'];
                record.commands.push(runCommand(`${id}-frame`, command, target, {COURSE_FRAME: step.frame_mode || 'triangle'}));
                const frame = path.join(target, 'frame.ppm');
                const original = ppm(frame).pixels;
                write(path.join(pictures, `${id}.png`), png(frame));
                record.pixels = {sha256: hash(original)};
                if (id === '04-input') {
                    record.commands.push(runCommand(`${id}-toggle`, [...command, '--', '1'], target, {COURSE_FRAME: 'light'}));
                    write(path.join(pictures, '04-input-light.png'), png(frame));
                    record.commands.push(runCommand(`${id}-roundtrip`, [...command, '--', '2'], target, {COURSE_FRAME: 'triangle'}));
                    assert.deepEqual(ppm(frame).pixels, original);
                    record.roundtrip = true;
                }
            }
            assert.equal(signature(id), source, `${id}: source changed during native verification`);
            records[id] = record;
            write(report, JSON.stringify(records, null, 2) + '\n');
            delete pending[id];
            write(pendingFile, JSON.stringify(pending, null, 2) + '\n');
            console.log(`${id}: Rust, WebAssembly and available rendering checks passed`);
        } finally {
            if (process.env.COURSE_KEEP_WORKSPACES !== '1') {
                removeGenerated(target, path.join(viewer, 'target'));
            }
        }
    }
}

export function structure() {
    return withExpectedSequence(structureChecks);
}

function sourceHash(file: string) {
    const bytes = fs.readFileSync(file);
    const text = bytes.toString('utf8');
    return hash(Buffer.from(text, 'utf8').equals(bytes) ? text.replaceAll('\r\n', '\n') : bytes);
}

export function assertRepeatedImage(step: Step, file: string, previous: string | undefined, steps: Step[], byCheckpoint: Map<string, Set<string>>, digest: string) {
    const firstClear = step.id === '02-clear' && path.basename(file).endsWith('-browser.png')
        && previous && path.basename(previous).endsWith('-browser.png') && byCheckpoint.get('01-canvas')?.has(digest);
    const sameAs = step.image_same_as;
    const parent = sameAs ? steps.findIndex(candidate => candidate.id === sameAs) : -1;
    if (sameAs) assert(parent >= 0 && parent < steps.indexOf(step), `${step.id}: repeated-image parent must precede this step`);
    const declaredSame = sameAs && byCheckpoint.get(sameAs)?.has(digest);
    assert(!previous || firstClear || declaredSame, `Undeclared repeated lesson image: ${file}`);
}

function structureChecks() {
    const images = new Map<string, string>(), byCheckpoint = new Map<string, Set<string>>();
    const steps = course().steps;
    for (const step of steps) {
        const files = expected(step.id);
        const page = read(path.join(docs, step.page));
        assert(page.includes(step.question) && page.includes(step.answer), step.id);
        assert.deepEqual([...page.matchAll(/--8<-- "([^"]+)"/g)].map(match => match[1]),
            step.edits.filter(edit => read(path.join(docs, edit.snippet))).map(edit => edit.snippet));
        assert(files['Cargo.toml'] && files['src/lib.rs']);
        for (const match of page.matchAll(/!\[[^\]]*\]\(([^)]+)\)/g)) {
            const file = path.resolve(path.dirname(path.join(docs, step.page)), match[1]);
            const digest = hash(fs.readFileSync(resolveJourneyImage(file)));
            const previous = images.get(digest);
            assertRepeatedImage(step, file, previous, steps, byCheckpoint, digest);
            images.set(digest, file);
            const seen = byCheckpoint.get(step.id) || new Set<string>();
            seen.add(digest); byCheckpoint.set(step.id, seen);
        }
    }
    const destination = json(path.join(docs, course().inventory || 'journey/current-viewer-2026-10-09.json'));
    const names = filesIn(path.join(viewer, 'src')).map(file => path.relative(viewer, file).split(path.sep).join('/'));
    assert.deepEqual(names.sort(), Object.keys(destination.files).sort());
    for (const [name, record] of Object.entries(destination.files) as [string, {sha256: string; course: string}][]) {
        assert.equal(sourceHash(path.join(viewer, name)), record.sha256, `Reference changed: ${name}`);
        assert(destination.courses.some((course: {id: string}) => course.id === record.course));
    }
    for (const [name, digest] of Object.entries(destination.inputs)) assert.equal(sourceHash(path.join(viewer, name)), digest, name);
    const mapped = destination.courses.flatMap((course: {reference_steps: string[]}) => course.reference_steps);
    assert.equal(mapped.length, new Set(mapped).size);
    assert.deepEqual(mapped.sort(), reference().chapters.map((chapter: {id: string}) => chapter.id).sort());
    console.log(`${course().steps.length} lesson checkpoints; ${names.length} viewer source files accounted for.`);
}
