import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {assetsFor, course, docs, expected, filesIn, fingerprint, hash, json, lockFor, materialize, read, reference, viewer, write} from './model.ts';
import {dependencyInputs} from './dependency-inputs.ts';
import {nativeTarget, runCommand} from './verify.ts';
import {png, ppm} from './png.ts';

const output = path.join(viewer, 'target/course-presentation');
const relative = (file: string) => path.relative(viewer, file).split(path.sep).join('/');
const digest = (file: string) => hash(fs.readFileSync(file));

function checkMaterialized(id: string, directory: string) {
    const kernel = path.relative(directory, path.resolve(viewer, '../session_rust')).split(path.sep).join('/');
    for (const [name, text] of Object.entries(expected(id))) {
        const wanted = name === 'Cargo.toml' ? text.replaceAll('../../../session_rust', kernel) : text;
        assert.equal(read(path.join(directory, name)), wanted, `Materialized checkpoint changed: ${name}`);
    }
    assert.equal(digest(path.join(directory, 'Cargo.lock')), digest(lockFor(id)));
    for (const name of assetsFor(id)) assert.equal(digest(path.join(directory, name)), digest(path.join(viewer, name)), name);
}

export function inventory() {
    const steps = course().steps;
    const chapters = filesIn(path.join(docs, 'journey/consolidation')).filter(file => /chapter-\d+\.json$/.test(file)).map(json);
    const lessons = chapters.flatMap(chapter => chapter.lessons);
    const pages = new Set<string>(lessons.map(lesson => path.join(docs, lesson.page)));
    for (const chapter of reference().chapters) pages.add(path.join(docs, chapter.page));
    for (const directory of ['journey/redraft-10-sections/lessons', 'typing']) {
        const root = path.join(docs, directory);
        if (fs.existsSync(root)) for (const file of filesIn(root)) if (file.endsWith('.md')) pages.add(file);
    }
    const images = new Map<string, Set<string>>();
    const missing: {page: string; image: string}[] = [];
    for (const page of pages) {
        const text = read(page);
        const references = [...text.matchAll(/!\[[^\]]*\]\(([^)]+)\)|<img\b[^>]*\bsrc=["']([^"']+)["']/g)];
        for (const match of references) {
            const url = (match[1] || match[2]).replace(/^<|>$/g, '').split(/[?#]/)[0];
            if (!/\.png$/i.test(url) || /^(?:https?:|data:)/.test(url)) continue;
            const file = path.resolve(path.dirname(page), decodeURIComponent(url));
            if (!fs.existsSync(file)) { missing.push({page: relative(page), image: url}); continue; }
            const sources = images.get(file) || new Set<string>();
            sources.add(relative(page)); images.set(file, sources);
        }
    }
    const readerImages = new Set(images.keys());
    const directRoot = path.join(docs, 'screenshots/journey');
    for (const name of fs.readdirSync(directRoot)) {
        if (name.endsWith('.png')) {
            const file = path.join(directRoot, name);
            if (!images.has(file)) images.set(file, new Set());
        }
    }
    const records = [...images].map(([file, pages]) => {
        const name = path.basename(file, '.png');
        const native = steps.find(step => name === step.id);
        const browser = native ? null : steps.find(step => name === `${step.id}-browser`);
        const exact = native || browser;
        const variant = exact ? null : steps.filter(step => name.startsWith(`${step.id}-`)).sort((a, b) => b.id.length - a.id.length)[0];
        const checkpoint = exact || variant;
        const bytes = fs.readFileSync(file);
        assert(bytes.subarray(0, 8).equals(Buffer.from([137, 80, 78, 71, 13, 10, 26, 10])), file);
        return {path: relative(file), sha256: hash(bytes), width: bytes.readUInt32BE(16), height: bytes.readUInt32BE(20),
            checkpoint: checkpoint?.id || null, frame: checkpoint?.frame || null,
            kind: native ? 'native' : browser ? 'browser' : variant ? 'variant' : 'supporting',
            queue: readerImages.has(file) ? 'reader-published' : 'direct-public',
            blankExpected: Boolean(checkpoint && /^(?:01-canvas|01a-gpu|02-clear)$/.test(checkpoint.id)),
            pages: [...pages].sort()};
    }).sort((a, b) => a.path.localeCompare(b.path));
    const result = {status: 'INVENTORY_ONLY', lessons: lessons.length, pages: pages.size, uniqueImages: records.length,
        native: records.filter(image => image.kind === 'native').length,
        browser: records.filter(image => image.kind === 'browser').length,
        variants: records.filter(image => image.kind === 'variant').length,
        supporting: records.filter(image => image.kind === 'supporting').length,
        readerImages: records.filter(image => image.queue === 'reader-published').length,
        directImages: records.filter(image => image.path.startsWith('docs/screenshots/journey/')).length,
        missing, images: records};
    const text = JSON.stringify(result, null, 2) + '\n';
    const snapshot = path.join(output, `inventory-${hash(text).slice(0, 20)}.json`);
    if (!fs.existsSync(snapshot)) write(snapshot, text);
    if (!fs.existsSync(path.join(output, 'inventory.json'))) write(path.join(output, 'inventory.json'), text);
    write(path.join(output, 'inventory-current.json'), JSON.stringify({snapshot, sha256: digest(snapshot)}, null, 2) + '\n');
    return {...result, snapshot};
}

export function bindings(id: string) {
    const step = course().steps.find(step => step.id === id);
    assert(step, `Unknown checkpoint: ${id}`);
    const files = expected(id);
    const kernel = path.resolve(viewer, '../session_rust');
    const inputs = files['Cargo.toml'].includes('session_rust')
        ? [...filesIn(path.join(kernel, 'src')), path.join(kernel, 'Cargo.toml'), path.join(kernel, 'build.rs')] : [];
    return {checkpoint: id, files: fingerprint(files), lock: digest(lockFor(id)),
        assets: Object.fromEntries(assetsFor(id).map(name => [name, digest(path.join(viewer, name))])),
        dependencies: dependencyInputs(files['Cargo.toml']),
        kernel: fingerprint(Object.fromEntries(inputs.map(file => [path.relative(kernel, file), digest(file)]))),
        fixture: step.frame ? digest(path.join(docs, step.frame)) : null,
        harness: digest(path.join(docs, 'journey/frame.rs')), constructor: step.frame_constructor || null,
        logicalSize: step.frame_size || [640, 480], helper: digest(fileURLToPath(import.meta.url))};
}

export function captureDirectory(id: string, density = 2) {
    const checkpoint = id === '04-input-light' ? '04-input' : id;
    const source = bindings(checkpoint);
    const key = hash(JSON.stringify({source, density, variant: id, attempt: process.env.COURSE_CAPTURE_ATTEMPT || null}));
    return {checkpoint, source, key, directory: path.join(viewer, 'target', `course-presentation-${id}-${key.slice(0, 16)}-dpr${density}`)};
}

function retained(record: any) {
    assert.deepEqual(bindings(record.source.checkpoint), record.source, 'Retained source/dependency binding changed');
    checkMaterialized(record.source.checkpoint, record.directory);
    assert.equal(digest(path.join(viewer, record.driver.path)), record.driver.sha256);
    if (record.status === 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW') {
        assert.equal(digest(path.join(viewer, record.image.path)), record.image.sha256);
    }
    return record;
}

export function prepare(id: string, density = 2) {
    assert([2, 3].includes(density), 'Presentation density must be 2 or 3');
    assert(id !== '01-canvas', 'The canvas-only checkpoint needs a browser capture');
    const {checkpoint, source, key, directory} = captureDirectory(id, density);
    const step = course().steps.find(step => step.id === checkpoint);
    assert(step, `Unknown checkpoint: ${id}`);
    const files = expected(checkpoint);
    if (fs.existsSync(directory)) return retained(json(path.join(directory, 'presentation.json')));
    const [width, height] = source.logicalSize.map(value => value * density);
    assert(width <= 4096 && height <= 4096, 'Presentation dimensions exceed the capture limit');
    const constructor = step.frame_constructor || (id === '02-clear' ? 'Renderer::new(device, queue)' : 'Renderer::new(device, queue, format)');
    const renderer = files['src/renderer.rs'] || '';
    const resizeSignature = renderer.match(/pub fn resize\([^)]*\)/)?.[0] || null;
    const resizing = Boolean(resizeSignature);
    const scaling = /pub fn set_density\(/.test(renderer);
    const resizeCall = !resizing ? '' : resizeSignature!.includes('width: u32, height: u32')
        ? '    renderer.resize(WIDTH, HEIGHT);\n'
        : resizeSignature!.includes('size: crate::viewport::Viewport')
            ? '    renderer.resize(viewer_journey::viewport::Viewport { width: WIDTH, height: HEIGHT });\n' : null;
    assert(resizeCall !== null, `Unrecognized resize signature: ${resizeSignature}`);
    let code = read(path.join(docs, 'journey/frame.rs'))
        .replace('// COURSE_SIZE', `const WIDTH: u32 = ${width};\nconst HEIGHT: u32 = ${height};`)
        .replace('// COURSE_NEW', `let mut renderer = ${constructor};\n` +
            resizeCall +
            (scaling ? `    renderer.set_density(${density}.0);` : ''));
    let draw = 'renderer.draw(&view);';
    if (checkpoint === '04-input') draw = 'let mut background = viewer_journey::background::Background::default(); ' +
        (id === '04-input-light' ? 'background.toggle(); ' : '') + 'renderer.draw(&view, &background);';
    if (step.frame) {
        draw = 'draw(&renderer, &view);';
        assert(code.includes('fn verify_pixels('), 'Canonical frame verification boundary missing');
        code = code.slice(0, code.indexOf('fn verify_pixels(')) + read(path.join(docs, step.frame));
    }
    assert.equal(code.split('    verify_pixels(&pixels);').length, 2, 'Canonical readback validation boundary changed');
    code = code.replace('// COURSE_DRAW', draw).replace('    verify_pixels(&pixels);',
        '    assert_eq!(pixels.len(), WIDTH as usize * HEIGHT as usize * 4);');
    materialize(files, directory, lockFor(checkpoint), assetsFor(checkpoint));
    const driver = path.join(directory, 'examples/presentation.rs');
    write(driver, code);
    const record = {status: 'PREPARED_NOT_EXECUTED', key, source, directory, density, width, height,
        rendererResize: resizeSignature, rendererDensity: scaling, driver: {path: relative(driver), sha256: digest(driver)},
        fixtureFunctionsPreserved: step.frame ? code.endsWith(read(path.join(docs, step.frame))) : true,
        canonicalPixelAssertions: 'Preserved unchanged in original fixture; fixed-resolution verification remains separate from this presentation capture.'};
    write(path.join(directory, 'presentation.json'), JSON.stringify(record, null, 2) + '\n');
    return record;
}

export function run(id: string, density = 2) {
    const {directory, key} = captureDirectory(id, density);
    const record = json(path.join(directory, 'presentation.json'));
    if (record.status === 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW') return retained(record);
    assert.equal(record.status, 'PREPARED_NOT_EXECUTED', 'A capture may be executed only once');
    const checkpoint = record.source.checkpoint;
    assert.deepEqual(bindings(checkpoint), record.source, 'Checkpoint or dependencies changed after preparation');
    assert.equal(digest(path.join(viewer, record.driver.path)), record.driver.sha256);
    checkMaterialized(checkpoint, directory);
    const label = `presentation-${id}-${key.slice(0, 16)}-dpr${density}`;
    assert(!fs.existsSync(path.join(viewer, 'target/course-checks', label + '.log')), 'Preserve the previous execution attempt');
    const command = runCommand(label,
        ['cargo', 'run', '--example', 'presentation', '--locked', '--target', nativeTarget(), '-j1'], directory,
        {COURSE_MEMORY_MIB: process.env.COURSE_MEMORY_MIB || '4096', COURSE_TIMEOUT_SECONDS: '1800'});
    assert.deepEqual(bindings(checkpoint), record.source, 'Checkpoint or dependencies changed during capture');
    checkMaterialized(checkpoint, directory);
    const frame = path.join(directory, 'frame.ppm');
    const pixels = ppm(frame);
    assert.equal(pixels.width, record.width); assert.equal(pixels.height, record.height);
    const image = path.join(directory, `${id}-dpr${density}.png`);
    write(image, png(frame));
    const result = {...record, status: 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW', command,
        image: {path: relative(image), sha256: digest(image), width: pixels.width, height: pixels.height}};
    write(path.join(directory, 'presentation.json'), JSON.stringify(result, null, 2) + '\n');
    return result;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
    const [mode, id, scale] = process.argv.slice(2);
    const result = mode === 'inventory' ? inventory() : mode === 'prepare' ? prepare(id, Number(scale || 2))
        : mode === 'run' ? run(id, Number(scale || 2)) : null;
    assert(result, 'Usage: presentation.ts inventory | prepare ID [2|3] | run ID [2|3]');
    console.log(JSON.stringify(mode === 'inventory' ? {...result, images: undefined} : result, null, 2));
}
