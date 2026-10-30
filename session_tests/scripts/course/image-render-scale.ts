import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';

const self = fileURLToPath(import.meta.url), root = path.dirname(self);
const read = (name: string) => fs.readFileSync(path.join(root, name), 'utf8');
const hash = (value: string | Buffer) => createHash('sha256').update(value).digest('hex');
const scope = JSON.parse(read('image-render-scale-scope.json'));

export function imageProfile(id: string, lane: string, requested = process.env.COURSE_IMAGE_RENDER_SCALE || '1', docsRoot = fileURLToPath(new URL('../../../session_viewer/docs/', import.meta.url))) {
    assert(['1', '2'].includes(requested), 'Image render scale must be exactly 1 or 2');
    const row = scope.find((row: any) => row.id === id && row.lane === lane);
    if (requested === '1' || !row) return null;
    for (const pin of row.pins) assert.equal(hash(fs.readFileSync(path.join(docsRoot, pin.path))), pin.sha256, 'Cap image fixture changed: ' + pin.path);
    const files = ['image-render-scale.ts', 'image-render-scale-scope.json', 'image-filtered-present.rs', 'image-filtered-present.wgsl'];
    return {renderScale: 2, lane, recipePins: row.pins, samples: 1, presentation: lane === 'native' ? 'linear-srgb-gpu' : 'canvas-compositor',
        files: files.map(name => ({path: path.join(root, name), sha256: hash(fs.readFileSync(path.join(root, name)))}))};
}

function transform(original: string, changes: {before: string; after: string}[]) {
    let code = original;
    for (const change of changes) {
        assert.equal(code.split(change.before).length - 1, 1, 'Image profile source boundary changed: ' + change.before.slice(0, 100));
        code = code.replace(change.before, change.after);
    }
    let reversed = code;
    for (const change of [...changes].reverse()) {
        assert.equal(reversed.split(change.after).length - 1, 1);
        reversed = reversed.replace(change.after, change.before);
    }
    assert.equal(reversed, original);
    return {code, beforeSha256: hash(original), afterSha256: hash(code), reversedSha256: hash(reversed),
        changes: changes.map(change => ({...change, needleCount: 1}))};
}

export function nativeScaleBridge(code: string, profile: any, output: number[], density: number) {
    assert.equal(profile.renderScale, 2); assert.equal(profile.lane, 'native');
    const internal = output.map(n => n * 2);
    assert(internal.every(n => Number.isSafeInteger(n) && n > 0 && n <= 4096));
    const size = `const WIDTH: u32 = ${output[0]};\nconst HEIGHT: u32 = ${output[1]};`;
    const resize = code.match(/    renderer\.resize\([^\n]+\);/g); assert.equal(resize?.length, 1);
    const constructor = '    let mut renderer=Renderer::new(device,queue,format,&math_source);';
    const begin = code.indexOf('    let row_bytes = '), end = code.indexOf('fn original_scene()');
    assert(begin > 0 && end > begin);
    const readback = code.slice(begin, end);
    const shader = read('image-filtered-present.wgsl');
    const present = `const COURSE_PRESENT_SHADER: &str = r#"${shader}"#;\n` + read('image-filtered-present.rs') + '\n';
    const changes = [
        {before: size, after: `const WIDTH: u32 = ${internal[0]};\nconst HEIGHT: u32 = ${internal[1]};\nconst OUTPUT_WIDTH: u32 = ${output[0]};\nconst OUTPUT_HEIGHT: u32 = ${output[1]};\nconst IMAGE_DENSITY: f32 = ${density * 2}.0;`},
        {before: `    renderer.set_density(${density}.0);`, after: '    renderer.set_density(IMAGE_DENSITY);'},
        {before: constructor, after: constructor + '\n    renderer.set_density(IMAGE_DENSITY);\n' + resize![0]},
        {before: 'usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,',
            after: 'usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::TEXTURE_BINDING,'},
        {before: readback, after: '    let texture = course_filtered_present(&renderer, &texture, format);\n' + readback.replace(/\bWIDTH\b/g, 'OUTPUT_WIDTH').replace(/\bHEIGHT\b/g, 'OUTPUT_HEIGHT')},
        {before: 'fn original_scene()', after: present + 'fn original_scene()'},
    ];
    return {...transform(code, changes), profile, output, internal, density: density * 2, externalDpr: density,
        shaderSha256: hash(shader), constructorAdaptedBeforeMathRefinement: true,
        drawAndPixelAssertionsPreserved: true};
}

export function browserScaleBridge(code: string, profile: any) {
    assert.equal(profile.renderScale, 2); assert.equal(profile.lane, 'browser');
    const initial = '    let density = if density.is_finite() && density > 0.0 { density } else { 1.0 };';
    const candidates = ['    let density = crate::browser_recovery::density(window.device_pixel_ratio());',
        '    let density = crate::browser_recovery::density(crate::browser_defaults::scale(window.device_pixel_ratio()));'];
    const matched = candidates.filter(line => code.includes(line)); assert.equal(matched.length, 1);
    const resize = matched[0];
    const helper = `fn course_image_scale(window: &web_sys::Window) -> f64 {
    let value = js_sys::Reflect::get(window.as_ref(), &JsValue::from_str("__courseImageRenderScale")).expect("Image scale property");
    if value.is_undefined() { return 1.0; }
    let scale = value.as_f64().expect("Numeric image scale");
    assert!(scale == 1.0 || scale == 2.0, "Image scale must be 1 or 2");
    scale
}

`;
    return {...transform(code, [
        {before: initial, after: initial + '\n    let density = density * course_image_scale(&window);'},
        {before: resize, after: resize + '\n    let density = density * course_image_scale(window);'},
        {before: '    editor.camera.aspect = size.aspect();', after: '    editor.camera.aspect = size.aspect();\n' +
            '    canvas.set_attribute("data-image-render-profile", &format!("{{\\"renderScale\\":{},\\"density\\":{},\\"internal\\":[{},{}]}}", course_image_scale(window), density, config.width, config.height)).expect("Image profile diagnostics");'},
        {before: 'fn resize(', after: helper + 'fn resize('},
    ]), profile, activation: 'Only after the unchanged image recipe; window DPR and CSS remain unchanged'};
}

export function validateBrowserScale(before: any, after: any, scale: number) {
    assert.equal(scale, 2);
    assert.deepEqual(after.css, before.css); assert.equal(after.dpr, before.dpr);
    assert.deepEqual(after.camera, before.camera); assert.deepEqual(after.sources, before.sources);
    assert.deepEqual(after.selected, before.selected); assert.deepEqual(after.clipping, before.clipping);
    assert.equal(after.requestedScale, scale);
    assert.deepEqual(after.rendererProfile, {renderScale: scale, density: before.dpr * scale, internal: after.internal});
    assert.deepEqual(after.internal, before.internal.map((n: number) => n * scale), 'Reject clamping, recovery fallback, or stale resize');
    assert.deepEqual(before.internal, before.css.map((n: number) => Math.round(n * before.dpr)));
}

export function capSnapshotOptions(frame: any) {
    const [cads, surfaces, objects] = frame.sources, planes = frame.clipping.rows;
    assert.equal(objects, 2, 'The cap fixture retains exactly its CAD and clipping-plane objects');
    assert.equal(cads.length, 1); assert.deepEqual(surfaces, []); assert.equal(planes.length, 1);
    assert(Number.isSafeInteger(cads[0].id) && Number.isSafeInteger(planes[0].id));
    assert.notEqual(cads[0].id, planes[0].id, 'The CAD and clipping plane retain distinct source owners');
    return {lane: 'mesh', source: 'cad', expectedRows: 2, expectedQualities: 1};
}

export async function applyBrowserScale(page: any, profile: any, settle: (options: ReturnType<typeof capSnapshotOptions>) => Promise<any>) {
    if (!profile) return null;
    const state = () => {
        const canvas = document.querySelector('canvas')!, rect = canvas.getBoundingClientRect();
        const value = (name: string) => JSON.parse(canvas.getAttribute(name)!);
        const clipping = value('data-clipping');
        return {css: [rect.width, rect.height], internal: [canvas.width, canvas.height], dpr: devicePixelRatio,
            requestedScale: (window as any).__courseImageRenderScale || 1, camera: value('data-camera-matrix'),
            sources: ['data-cads', 'data-surfaces', 'data-object-count', 'data-view-flags'].map(value), selected: value('data-selected-ids'),
            clipping: {usage: clipping.usage.slice(0, 3), rows: clipping.rows, caps: clipping.caps[1], error: clipping.error, cap_error: clipping.cap_error,
                appearance: Object.fromEntries(['opacity', 'outlines', 'arctic'].filter(key => Object.hasOwn(clipping, key)).map(key => [key, clipping[key]]))},
            rendererProfile: value('data-image-render-profile')};
    };
    const before = await page.evaluate(state); assert.equal(before.requestedScale, 1);
    const options = capSnapshotOptions(before);
    await page.evaluate(() => { (window as any).__courseImageRenderScale = 2; window.dispatchEvent(new Event('resize')); });
    const settled = await settle(options); assert.equal(settled.adaptive, true, 'A cap image must retain accepted mathematical quality at the internal resolution');
    assert.equal(settled.quality.qualities.length, 1);
    assert.equal(settled.quality.qualities[0][0], before.sources[0][0].id, 'The accepted mathematical quality belongs to the original CAD');
    const after = await page.evaluate(state); validateBrowserScale(before, after, 2);
    assert.equal(after.clipping.caps, 1); assert.deepEqual(after.clipping.usage, [1, 1, 1]);
    assert.equal(after.clipping.rows.length, 1); assert.equal(after.clipping.error, null); assert.equal(after.clipping.cap_error, null);
    return {profile, before, after, settled, canonicalBehaviorRenderScale: 1, presentation: 'canvas-compositor'};
}

export function verifyNativeScale(directory: string, bridge: any) {
    if (!bridge) return null;
    const record = JSON.parse(fs.readFileSync(bridge.record.path, 'utf8'));
    assert.equal(hash(fs.readFileSync(bridge.record.path)), bridge.record.sha256);
    const file = path.join(directory, 'frame-render-scale.json'), bytes = fs.readFileSync(file), value = JSON.parse(bytes.toString('utf8'));
    assert.deepEqual(value, {renderScale: 2, internal: record.internal, output: record.output, density: record.density,
        format: 'Rgba8UnormSrgb', filter: 'linear', samples: 1});
    return {path: file, sha256: hash(bytes), value};
}
