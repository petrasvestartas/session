import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {assetsFor, course, docs, expected, filesIn, fingerprint, hash, json, lockFor, materialize, read, reference, viewer, write, resolveJourneyImage, materializedContent} from './model.ts';
import {dependencyInputs} from './dependency-inputs.ts';
import {nativeTarget, runCommand} from './verify.ts';
import {png, ppm} from './png.ts';
import {adapterMetadataPath, fileReference, observeExecution, finishExecution} from './capture-storage.ts';

import {imageProfile, nativeScaleBridge, verifyNativeScale} from './image-render-scale.ts';
import {nativeRefinementBudget, nativeRefinementFiles, nativeRefinementDensity} from './image-native-refinement.ts';

const output = path.join(viewer, 'target/course-presentation');
const relative = (file: string) => path.relative(viewer, file).split(path.sep).join('/');
const digest = (file: string) => hash(fs.readFileSync(file));

export function nativeAdapterProfile() {
    const mode = process.env.COURSE_NATIVE_ADAPTER || 'cpu';
    assert(['cpu', 'nvidia'].includes(mode), 'Unsupported native image adapter profile');
    if (mode === 'cpu') return null;
    assert.equal(process.platform, 'win32', 'NVIDIA image profile requires Windows DX12');
    const metadata = fileReference(adapterMetadataPath()), enumeration = json(metadata.path), selected = enumeration.selected;
    assert.equal(selected.vendor, 0x10de);
    assert(Number.isInteger(selected.device) && selected.device > 0 && selected.device <= 0xffffffff);
    assert(selected.luid && Number.isInteger(selected.flags) && !(selected.flags & 2), 'Selected adapter must be enumerated hardware');
    assert(enumeration.adapters.some((adapter: any) => ['vendor', 'device', 'luid', 'flags'].every(name => adapter[name] === selected[name])));
    const enumerator = fileReference(enumeration.tool); assert.equal(enumerator.sha256, enumeration.toolSha256);
    assert(process.env.VIEWER_DXC_PATH, 'NVIDIA image profile requires an explicit DXC SDK');
    const dxc = fileReference(process.env.VIEWER_DXC_PATH), dxil = fileReference(path.join(path.dirname(dxc.path), 'dxil.dll'));
    return {mode, vendor: selected.vendor, device: selected.device, enumeratedLuid: selected.luid,
        backend: 'Dx12', deviceType: 'DiscreteGpu', metadata, enumerator, dxc, dxil};
}

export function adapterDriverBridge(code: string, profile: NonNullable<ReturnType<typeof nativeAdapterProfile>>) {
    const changes = [{before: 'info.backend == wgpu::Backend::Dx12 && info.device_type == wgpu::DeviceType::Cpu\n' +
        '                && info.vendor == 0x1414 && info.device == 0x8c\n' +
        '        }).ok_or("Windows frame evidence requires the Microsoft WARP DX12 adapter")?',
    after: 'info.backend == wgpu::Backend::Dx12 && info.device_type == wgpu::DeviceType::DiscreteGpu\n' +
        `                && info.vendor == ${profile.vendor} && info.device == ${profile.device}\n` +
        '        }).ok_or("NVIDIA image profile requires the selected hardware DX12 adapter")?'},
    {before: '    eprintln!("Native frame adapter: {:?}", adapter.get_info());',
    after: '    eprintln!("Native frame adapter: {:?}", adapter.get_info());\n' +
        '    let info = adapter.get_info();\n' +
        '    let mut adapter_receipt = std::fs::OpenOptions::new().write(true).create_new(true).open("frame-adapter.json")?;\n' +
        '    writeln!(adapter_receipt, "{{\\"vendor\\":{},\\"device\\":{},\\"backend\\":\\"{:?}\\",\\"deviceType\\":\\"{:?}\\"}}",\n' +
        '        info.vendor, info.device, info.backend, info.device_type)?;'}];
    const beforeSha256 = hash(code);
    let transformed = code;
    for (const change of changes) {
        assert.equal(transformed.split(change.before).length - 1, 1, 'Canonical adapter header changed');
        transformed = transformed.replace(change.before, change.after);
    }
    let reversed = transformed;
    for (const change of [...changes].reverse()) reversed = reversed.replace(change.after, change.before);
    assert.equal(reversed, code, 'Adapter bridge must reverse to the exact generated presentation');
    return {code: transformed, beforeSha256, afterSha256: hash(transformed), reversedSha256: hash(reversed),
        changes: changes.map(change => ({...change, needleCount: 1}))};
}

export function verifyNativeAdapter(directory: string, profile: ReturnType<typeof nativeAdapterProfile>) {
    if (!profile) return null;
    const receipt = fileReference(path.join(directory, 'frame-adapter.json')), adapter = json(receipt.path);
    for (const name of ['vendor', 'device', 'backend', 'deviceType'] as const) assert.equal(adapter[name], profile[name], 'Executed native adapter differs from image profile');
    return {...receipt, adapter};
}

export function nativeCaptureDensity(lane: string, density = 2) {
    assert([2, 3, 4].includes(density), 'Native image density must be 2, 3 or 4');
    assert(lane === 'native' || density === 2, 'Explicit image density requires the native lane');
    return density;
}

export function nativeDensityBridge(code: string, density: number, fixture: string, resize: string, renderer: string) {
    nativeCaptureDensity('native', density);
    if (density !== 4) return null;
    assert(fixture && code.endsWith(fixture), 'Exact canonical fixture must remain the driver suffix');
    const dimensions = code.match(/const WIDTH: u32 = (\d+);\nconst HEIGHT: u32 = (\d+);/);
    assert(dimensions, 'Native physical dimensions missing');
    const physical = dimensions.slice(1).map(Number);
    assert(physical.every(n => n > 0 && n <= 4096), 'Native physical dimensions exceed the capture limit');
    assert(typeof renderer === 'string' && /pub fn set_density\(&mut self, density: f32\)/.test(renderer), 'Assembled Renderer must expose set_density(f32)');
    const signature = renderer.match(/pub fn resize\([^)]*\)/)?.[0];
    const supportedResize = signature?.includes('width: u32, height: u32') ? '    renderer.resize(WIDTH, HEIGHT);\n'
        : signature?.includes('size: crate::viewport::Viewport') ? '    renderer.resize(viewer_journey::viewport::Viewport { width: WIDTH, height: HEIGHT });\n' : null;
    assert(supportedResize, 'Assembled Renderer must expose a supported physical resize');
    assert.equal(resize, supportedResize, 'Chosen resize must match the assembled Renderer signature');
    assert.equal(code.split(resize).length, 2, 'Exact outer resize boundary changed');
    const constructors = [...fixture.matchAll(/let mut renderer\s*=\s*Renderer::new\(\s*device\s*,\s*queue\s*,\s*format\s*,\s*&(?:editor\.scene|e\.scene|math_source)\s*\);/g)];
    assert.equal(constructors.length, 1, 'One source-bound mathematical renderer constructor required');
    assert(fixture.includes('fn draw(') && fixture.includes('fn verify_pixels('), 'Canonical draw and pixel assertion tails required');
    const before = constructors[0][0], constructor = fixture.indexOf(before), drain = fixture.indexOf('math_quality.drain_matrix(');
    assert(drain > constructor + before.length, 'Renderer construction must precede mathematical refinement');
    assert.equal(fixture.split('math_quality.drain_matrix(').length, 2, 'One mathematical refinement boundary required');
    assert(fixture.slice(drain).split(';')[0].includes('[WIDTH,HEIGHT]'), 'Mathematical refinement must use the physical target');
    assert(!fixture.slice(constructor + before.length, drain).includes('renderer.resize('), 'Unexpected pre-refinement resize');
    assert.equal(code.split(before).length, 2, 'Exact renderer constructor boundary changed');
    const after = before + `\n    renderer.set_density(${density}.0);\n` + resize.trimEnd();
    const transformed = code.replace(before, after);
    assert.equal(transformed.replace(after, before), code, 'Native density bridge must reverse exactly');
    assert.equal(transformed.slice(transformed.indexOf('fn draw(')), code.slice(code.indexOf('fn draw(')), 'Draw and canonical pixel assertions must remain unchanged');
    return {code: transformed, density, physical, rendererSha256: hash(renderer), beforeSha256: hash(code), afterSha256: hash(transformed), reversedSha256: hash(code),
        changes: [{before, after, needleCount: 1}], constructorAdaptedBeforeMath: true, drawAndPixelAssertionsPreserved: true};
}

function checkAdapterBridge(record: any) {
    if (record.nativeDensityBridge) {
        const bridge = record.nativeDensityBridge, before = read(bridge.before.path);
        assert.equal(hash(before), bridge.before.sha256);
        const step = course().steps.find(step => step.id === record.source.checkpoint); assert(step?.frame);
        assert(!record.renderScaleBridge, 'Raw native density must not use a filtered image bridge');
        const resize = record.rendererResize.includes('width: u32, height: u32')
            ? '    renderer.resize(WIDTH, HEIGHT);\n'
            : '    renderer.resize(viewer_journey::viewport::Viewport { width: WIDTH, height: HEIGHT });\n';
        const generated = nativeDensityBridge(before, record.density, read(path.join(docs, step.frame)), resize, expected(record.source.checkpoint)['src/renderer.rs']); assert(generated);
        const {code, ...proof} = generated;
        assert.deepEqual(json(bridge.record.path), proof); assert.equal(digest(bridge.record.path), bridge.record.sha256);
        assert.equal(code, read(record.adapterBridge?.before.path || path.join(viewer, record.driver.path)));
    }

    if (record.renderScaleBridge) {
        const bridge = record.renderScaleBridge, before = read(bridge.before.path);
        assert.equal(hash(before), bridge.before.sha256);
        const step = course().steps.find(step => step.id === record.source.checkpoint); assert(step?.frame);
        const generated = nativeScaleBridge(before, record.source.imageProfile.native, [record.width, record.height], record.density, read(path.join(docs, step.frame)));
        const {code, ...proof} = generated;
        assert.deepEqual(json(bridge.record.path), proof);
        assert.equal(digest(bridge.record.path), bridge.record.sha256);
        assert.equal(code, read(record.adapterBridge?.before.path || path.join(viewer, record.driver.path)));
    }
    if (!record.source.nativeAdapter) return;
    const bridge = record.adapterBridge;
    assert.equal(digest(bridge.before.path), bridge.before.sha256);
    const transformed = adapterDriverBridge(read(bridge.before.path), record.source.nativeAdapter);
    assert.equal(transformed.code, read(path.join(viewer, record.driver.path)));
    const {code, ...proof} = transformed;
    assert.deepEqual(json(bridge.record.path), proof);
    assert.equal(digest(bridge.record.path), bridge.record.sha256);
}

function checkMaterialized(id: string, directory: string, refinement: any = null) {
    for (const [name, text] of Object.entries(nativeRefinementFiles(expected(id), refinement))) {
        const wanted = materializedContent(name, text, directory);
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
            const actual = file.startsWith(path.join(docs, 'screenshots/journey') + path.sep) ? resolveJourneyImage(file) : file;
            if (!fs.existsSync(actual)) { missing.push({page: relative(page), image: url}); continue; }
            const sources = images.get(file) || new Set<string>();
            sources.add(relative(page)); images.set(file, sources);
        }
    }
    const readerImages = new Set(images.keys());
    const directRoot = path.join(docs, 'screenshots/journey');
    const aliasFile = path.join(directRoot, 'image-aliases.json');
    const aliases = fs.existsSync(aliasFile) ? Object.keys(json(aliasFile).aliases) : [];
    for (const name of new Set([...fs.readdirSync(directRoot), ...aliases.filter(name => !name.includes('/'))])) {
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
        const actual = resolveJourneyImage(file), bytes = fs.readFileSync(actual);
        assert(bytes.subarray(0, 8).equals(Buffer.from([137, 80, 78, 71, 13, 10, 26, 10])), file);
        return {path: relative(file), ...(actual === file ? {} : {physicalPath: relative(actual)}), sha256: hash(bytes), width: bytes.readUInt32BE(16), height: bytes.readUInt32BE(20),
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
    const files = expected(id), refinement = nativeRefinementBudget(id);
    const adapted = nativeRefinementFiles(files, refinement);
    const kernel = path.resolve(viewer, '../session_rust');
    const inputs = files['Cargo.toml'].includes('session_rust')
        ? [...filesIn(path.join(kernel, 'src')), path.join(kernel, 'Cargo.toml'), path.join(kernel, 'build.rs')] : [];
    return {checkpoint: id, files: fingerprint(files), lock: digest(lockFor(id)),
        assets: Object.fromEntries(assetsFor(id).map(name => [name, digest(path.join(viewer, name))])),
        dependencies: dependencyInputs(files['Cargo.toml']),
        kernel: fingerprint(Object.fromEntries(inputs.map(file => [path.relative(kernel, file), digest(file)]))),
        fixture: step.frame ? digest(path.join(docs, step.frame)) : null,
        harness: digest(path.join(docs, 'journey/frame.rs')), constructor: step.frame_constructor || null,
        logicalSize: step.frame_size || [640, 480], imageProfile: {native: imageProfile(id, 'native', undefined, docs), browser: imageProfile(id, 'browser', undefined, docs)}, helper: digest(fileURLToPath(import.meta.url)), nativeAdapter: nativeAdapterProfile(),
        ...(refinement ? {nativeRefinementBudget: {...refinement, originalMathSha256: hash(files['src/math_quality.rs']), adaptedMathSha256: hash(adapted['src/math_quality.rs'])}} : {})};
}

export function captureDirectory(id: string, density = 2) {
    const checkpoint = id === '04-input-light' ? '04-input' : id;
    const source = bindings(checkpoint);
    const key = hash(JSON.stringify({source, density, variant: id, attempt: process.env.COURSE_CAPTURE_ATTEMPT || null}));
    return {checkpoint, source, key, directory: path.join(viewer, 'target', `course-presentation-${id}-${key.slice(0, 16)}-dpr${density}`)};
}

function retained(record: any) {
    assert.deepEqual(bindings(record.source.checkpoint), record.source, 'Retained source/dependency binding changed');
    checkMaterialized(record.source.checkpoint, record.directory, record.source.nativeRefinementBudget || null);
    assert.equal(digest(path.join(viewer, record.driver.path)), record.driver.sha256);
    checkAdapterBridge(record);
    if (record.status === 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW') {
        assert.equal(digest(path.join(viewer, record.image.path)), record.image.sha256);
        assert.deepEqual(verifyNativeAdapter(record.directory, record.source.nativeAdapter), record.adapterReceipt || null);
        assert.deepEqual(verifyNativeScale(record.directory, record.renderScaleBridge), record.renderScaleReceipt || null);
    }
    return record;
}

export function prepare(id: string, density = 2) {
    nativeCaptureDensity('native', density);
    assert(id !== '01-canvas', 'The canvas-only checkpoint needs a browser capture');
    const {checkpoint, source, key, directory} = captureDirectory(id, density);
    const step = course().steps.find(step => step.id === checkpoint);
    assert(step, `Unknown checkpoint: ${id}`);
    nativeRefinementDensity(source.nativeRefinementBudget, density);
    const files = nativeRefinementFiles(expected(checkpoint), source.nativeRefinementBudget || null);
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
    if (source.imageProfile.native) assert(step.frame, 'Native image scaling requires an exact source-bound fixture');
    assert(density !== 4 || !source.imageProfile.native, 'Raw DPR4 requires renderScale1, without a filtered image profile');
    const scale = source.imageProfile.native ? nativeScaleBridge(code, source.imageProfile.native, [width, height], density, read(path.join(docs, step.frame!))) : null;
    const scaleBefore = code;
    if (scale) code = scale.code;
    const densityBefore = code;
    const physical = density === 4 ? nativeDensityBridge(code, density, read(path.join(docs, step.frame!)), resizeCall, renderer) : null;
    if (physical) code = physical.code;
    const bridge = source.nativeAdapter ? adapterDriverBridge(code, source.nativeAdapter) : null;
    materialize(files, directory, lockFor(checkpoint), assetsFor(checkpoint));
    let densityBridge = null;
    if (physical) {
        const before = path.join(directory, 'native-density-before.rs'), proof = path.join(directory, 'native-density-bridge.json');
        const {code: _code, ...record} = physical;
        write(before, densityBefore); write(proof, JSON.stringify(record, null, 2) + '\n');
        densityBridge = {before: fileReference(before), record: fileReference(proof)};
    }
    let renderScaleBridge = null;
    if (scale) {
        const before = path.join(directory, 'render-scale-before.rs'), proof = path.join(directory, 'render-scale-bridge.json');
        write(before, scaleBefore); write(proof, JSON.stringify({...scale, code: undefined}, null, 2) + '\n');
        renderScaleBridge = {before: fileReference(before), record: fileReference(proof)};
    }
    let adapterBridge = null;
    if (bridge) {
        const before = path.join(directory, 'adapter-driver-before.rs'), proof = path.join(directory, 'adapter-bridge.json');
        write(before, code); write(proof, JSON.stringify({...bridge, code: undefined}, null, 2) + '\n');
        adapterBridge = {before: fileReference(before), record: fileReference(proof)};
        code = bridge.code;
    }
    const driver = path.join(directory, 'examples/presentation.rs');
    write(driver, code);
    const record = {status: 'PREPARED_NOT_EXECUTED', key, source, directory, density, width, height,
        rendererResize: resizeSignature, rendererDensity: scaling, driver: {path: relative(driver), sha256: digest(driver)}, adapterBridge, renderScaleBridge,
        ...(densityBridge ? {nativeDensityBridge: densityBridge} : {}),
        fixtureFunctionsPreserved: !scale && !physical && (step.frame ? code.endsWith(read(path.join(docs, step.frame))) : true),
        generatedConstructorAdaptedBeforeMath: Boolean(scale || physical), generatedDrawAndPixelAssertionsPreserved: scale ? scale.drawAndPixelAssertionsPreserved : physical ? physical.drawAndPixelAssertionsPreserved : true,
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
    checkAdapterBridge(record);
    const adapterProfile = record.source.nativeAdapter;
    if (adapterProfile) assert(!fs.existsSync(path.join(directory, 'frame-adapter.json')), 'Preserve previous adapter execution evidence');
    const adapterInputs = adapterProfile ? observeExecution([path.join(viewer, record.driver.path), ...['metadata', 'enumerator', 'dxc', 'dxil'].map(name => adapterProfile[name].path)]) : null;
    const label = `presentation-${id}-${key.slice(0, 16)}-dpr${density}`;
    assert(!fs.existsSync(path.join(viewer, 'target/course-checks', label + '.log')), 'Preserve the previous execution attempt');
    const command = runCommand(label,
        ['cargo', 'run', '--example', 'presentation', '--locked', '--target', nativeTarget(), '-j1'], directory,
        {COURSE_MEMORY_MIB: process.env.COURSE_MEMORY_MIB || '4096', COURSE_TIMEOUT_SECONDS: '1800',
            ...(adapterProfile ? {PATH: path.dirname(adapterProfile.dxc.path) + path.delimiter + process.env.PATH} : {})});
    const adapterInputIntegrity = adapterInputs ? finishExecution(adapterInputs) : null;
    const adapterReceipt = verifyNativeAdapter(directory, adapterProfile);
    const renderScaleReceipt = verifyNativeScale(directory, record.renderScaleBridge);
    assert.deepEqual(bindings(checkpoint), record.source, 'Checkpoint or dependencies changed during capture');
    checkMaterialized(checkpoint, directory);
    const frame = path.join(directory, 'frame.ppm');
    const pixels = ppm(frame);
    assert.equal(pixels.width, record.width); assert.equal(pixels.height, record.height);
    const image = path.join(directory, `${id}-dpr${density}.png`);
    write(image, png(frame));
    const result = {...record, status: 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW', command, adapterInputIntegrity, adapterReceipt, renderScaleReceipt,
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
