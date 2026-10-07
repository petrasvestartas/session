import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {course, docs, viewer, read, json, write, hash, fingerprint, expected, materialize, filesIn, reference, lockFor, compare, assetsFor} from './model.ts';
import {png, ppm} from './png.ts';

export const evidence = path.join(viewer, 'target/course-checks');
const native = 'x86_64-unknown-linux-gnu';
const pictures = path.join(docs, 'screenshots/journey');

export function signature(id: string) {
    const step = course().steps.find(step => step.id === id)!;
    const files = expected(id);
    const kernel = path.join(viewer, '../session_rust');
    const inputs = files['Cargo.toml'].includes('session_rust')
        ? [...filesIn(path.join(kernel, 'src')), path.join(kernel, 'Cargo.toml'), path.join(kernel, 'build.rs')] : [];
    return fingerprint({...files, 'Cargo.lock': read(lockFor(id)),
        ...Object.fromEntries(assetsFor(id).map(name => [name, hash(fs.readFileSync(path.join(viewer, name)))])),
        kernel: fingerprint(Object.fromEntries(inputs.map(file => [path.relative(kernel, file), hash(fs.readFileSync(file))]))),
        constructor: step.frame_constructor || '',
        frame_size: JSON.stringify(step.frame_size || [640, 480]),
        ...(step.frame ? {frame: read(path.join(docs, step.frame))} : {}),
        harness: read(path.join(docs, 'journey/frame.rs')), checker: read(new URL(import.meta.url).pathname)});
}

export function runCommand(name: string, args: string[], directory: string, extra: Record<string, string> = {}) {
    fs.mkdirSync(evidence, {recursive: true});
    const log = path.join(evidence, `${name}.log`);
    const descriptor = fs.openSync(log, 'w');
    const env = {...process.env, RUSTC_WRAPPER: '', CARGO_NET_OFFLINE: 'true', NO_COLOR: 'true',
        CARGO_BUILD_JOBS: '4', RUST_TEST_THREADS: '1', REGEN_PROTO: '0',
        CARGO_TARGET_DIR: path.join(viewer, 'target/course-build'), ...extra};
    let result;
    try {
        result = spawnSync('buildslot', ['timeout', '10m', 'prlimit', '--data=6442450944', '--', ...args],
            {cwd: directory, env, stdio: ['ignore', descriptor, descriptor]});
    } finally {
        fs.closeSync(descriptor);
    }
    if (result.error || result.status !== 0) throw Error(`${name} failed. Read ${log}\n${read(log).slice(-4000)}`);
    return {name, args, returncode: result.status};
}

export function verify(ids: string[], stored = false) {
    const report = path.join(evidence, 'results.json');
    const records = fs.existsSync(report) ? json(report) : {};
    const known = course().steps.map(step => step.id);
    if (ids.some(id => !known.includes(id))) throw Error('Unknown checkpoint');
    for (const id of ids.length ? ids : known) {
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
            if (id !== '01-canvas') assert(record.pixels);
            if (id === '04-input') assert(record.roundtrip);
            console.log(`${id}: evidence current`);
            continue;
        }
        delete records[id];
        write(report, JSON.stringify(records, null, 2) + '\n');
        const target = path.join(viewer, 'target', `course-${id}`);
        fs.rmSync(target, {recursive: true, force: true});
        materialize(expected(id), target, lockFor(id), assetsFor(id));
        assert.deepEqual(compare(expected(id), target), [], `${id}: the build must use the displayed source`);
        const record: any = {source: signature(id), commands: [], browser: 'not verified'};
        record.commands.push(runCommand(`${id}-wasm`, ['cargo', 'build', '--lib', '--locked', '--target', 'wasm32-unknown-unknown', '-j4'], target));
        if (step.tests) record.commands.push(runCommand(`${id}-tests`, ['cargo', 'test', '--lib', '--locked', '--target', native, '-j4'], target));
        record.commands.push(runCommand(`${id}-web`, ['trunk', 'build', '--release', '--public-url', './'], target));
        const bundle = path.join(evidence, id, 'dist');
        fs.rmSync(bundle, {recursive: true, force: true});
        fs.mkdirSync(path.dirname(bundle), {recursive: true});
        fs.cpSync(path.join(target, 'dist'), bundle, {recursive: true});
        if (id !== '01-canvas') {
            const args = id === '02-clear' ? 'device, queue' : 'device, queue, format';
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
            const command = ['cargo', 'run', '--example', 'frame', '--locked', '--target', native, '-j4'];
            record.commands.push(runCommand(`${id}-frame`, command, target, {COURSE_FRAME: id === '02-clear' ? 'clear' : 'triangle'}));
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
        records[id] = record;
        write(report, JSON.stringify(records, null, 2) + '\n');
        console.log(`${id}: Rust, WebAssembly and available rendering checks passed`);
    }
}

export function structure() {
    const images = new Map<string, string>();
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
            const digest = hash(fs.readFileSync(file));
            const previous = images.get(digest);
            // Some changes (GPU initialization or ownership) deliberately keep the picture.
            const firstClear = path.basename(file) === '02-clear-browser.png'
                && previous && path.basename(previous) === '01-canvas-browser.png';
            const sameAs = step.image_same_as;
            const parent = sameAs ? steps.findIndex(candidate => candidate.id === sameAs) : -1;
            if (sameAs) assert(parent >= 0 && parent < steps.indexOf(step), `${step.id}: repeated-image parent must precede this step`);
            const declaredSame = sameAs && previous && [`${sameAs}.png`, `${sameAs}-browser.png`].includes(path.basename(previous));
            assert(!previous || firstClear || declaredSame, `Undeclared repeated lesson image: ${file}`);
            images.set(digest, file);
        }
    }
    const destination = json(path.join(docs, 'journey/destination.json'));
    const names = filesIn(path.join(viewer, 'src')).map(file => path.relative(viewer, file));
    assert.deepEqual(names.sort(), Object.keys(destination.files).sort());
    for (const [name, record] of Object.entries(destination.files) as [string, {sha256: string; course: string}][]) {
        assert.equal(hash(fs.readFileSync(path.join(viewer, name))), record.sha256, `Reference changed: ${name}`);
        assert(destination.courses.some((course: {id: string}) => course.id === record.course));
    }
    for (const [name, digest] of Object.entries(destination.inputs)) assert.equal(hash(fs.readFileSync(path.join(viewer, name))), digest, name);
    const mapped = destination.courses.flatMap((course: {reference_steps: string[]}) => course.reference_steps);
    assert.equal(mapped.length, new Set(mapped).size);
    assert.deepEqual(mapped.sort(), reference().chapters.map((chapter: {id: string}) => chapter.id).sort());
    console.log(`${course().steps.length} lesson checkpoints; ${names.length} viewer source files accounted for.`);
}
