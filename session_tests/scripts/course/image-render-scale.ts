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
    for (const pin of row.pins) assert.equal(hash(fs.readFileSync(path.join(docsRoot, pin.path))), pin.sha256, 'Image fixture changed: ' + pin.path);
    const files = ['image-render-scale.ts', 'image-render-scale-scope.json', 'image-filtered-present.rs', 'image-filtered-present.wgsl'];
    return {renderScale: 2, lane, ...(row.scene ? {scene: row.scene} : {}), ...(row.route ? {route: row.route} : {}), recipePins: row.pins, samples: 1, presentation: lane === 'native' ? 'linear-srgb-gpu' : 'canvas-compositor',
        files: [...files.map(name => ({path: path.join(root, name), sha256: hash(fs.readFileSync(path.join(root, name)))})),
            ...(['comparison', 'surface-network'].includes(row.scene) && lane === 'browser' ? [{path: path.join(docsRoot, 'journey/checks/browser/adaptive-gpu-reuse.cjs'),
                sha256: hash(fs.readFileSync(path.join(docsRoot, 'journey/checks/browser/adaptive-gpu-reuse.cjs')))}] : [])]};
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

export function nativeScaleBridge(code: string, profile: any, output: number[], density: number, fixture: string) {
    assert.equal(profile.renderScale, 2); assert.equal(profile.lane, 'native');
    const internal = output.map(n => n * 2);
    assert(internal.every(n => Number.isSafeInteger(n) && n > 0 && n <= 4096));
    const size = `const WIDTH: u32 = ${output[0]};\nconst HEIGHT: u32 = ${output[1]};`;
    const resize = code.match(/    renderer\.resize\([^\n]+\);/g); assert.equal(resize?.length, 1);
    assert(fixture.length > 0 && code.endsWith(fixture), 'Exact canonical fixture must be the generated driver suffix');
    const constructors = ['math_source', 'editor.scene'].map(source => `let mut renderer=Renderer::new(device,queue,format,&${source});`);
    const matched = constructors.filter(needle => fixture.includes(needle)); assert.equal(matched.length, 1);
    const constructor = matched[0]; assert.equal(fixture.split(constructor).length - 1, 1);
    const begin = code.indexOf('    let row_bytes = '), end = code.length - fixture.length;
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
        {before: readback, after: '    let texture = course_filtered_present(&renderer, &texture, format);\n' + readback.replace(/\bWIDTH\b/g, 'OUTPUT_WIDTH').replace(/\bHEIGHT\b/g, 'OUTPUT_HEIGHT') + present},
    ];
    return {...transform(code, changes), profile, output, internal, density: density * 2, externalDpr: density,
        shaderSha256: hash(shader), constructorAdaptedBeforeMathRefinement: true,
        drawAndPixelAssertionsPreserved: true};
}

export function browserScaleBridge(code: string, profile: any) {
    assert.equal(profile.renderScale, 2); assert.equal(profile.lane, 'browser');
    const initial = '    let density = if density.is_finite() && density > 0.0 { density } else { 1.0 };';
    const initialChanges = code.includes(initial) ? [{before: initial, after: initial + '\n    let density = density * course_image_scale(&window);'}] : [];
    const initialization = initialChanges.length ? 'density-before-initial-config' : 'temporary-css-config-then-scaled-resize';
    if (!initialChanges.length) {
        assert.equal(profile.scene, 'surface-network', 'Unknown browser initialization');
        const boot = '    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);\n' +
            '    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);';
        const firstResize = '    if resize(\n        &window,\n        &canvas,\n        &surface,\n        &mut config,\n        &mut renderer,\n        &mut editor,\n    ) {';
        assert.equal(code.split(boot).length, 2); assert.equal(code.split(firstResize).length, 2);
        const at = code.indexOf(firstResize); assert(at > code.indexOf(boot));
        assert(code.indexOf('math_view.refresh(') > at, 'First math refresh must follow scaled resize');
        assert(code.indexOf('\n        present(') > at, 'First presentation must follow scaled resize');
    }
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
        ...initialChanges,
        {before: resize, after: resize + '\n    let density = density * course_image_scale(window);'},
        {before: '    editor.camera.aspect = size.aspect();', after: '    editor.camera.aspect = size.aspect();\n' +
            '    canvas.set_attribute("data-image-render-profile", &format!("{{\\"renderScale\\":{},\\"density\\":{},\\"internal\\":[{},{}]}}", course_image_scale(window), density, config.width, config.height)).expect("Image profile diagnostics");'},
        {before: 'fn resize(', after: helper + 'fn resize('},
    ]), profile, initialization, activation: 'Only after the unchanged image recipe; window DPR and CSS remain unchanged'};
}

export function validateBrowserScale(before: any, after: any, scale: number) {
    assert.equal(scale, 2);
    assert.deepEqual(after.css, before.css); assert.equal(after.dpr, before.dpr);
    assert.deepEqual(after.camera, before.camera); assert.deepEqual(after.sources, before.sources);
    assert.deepEqual(after.ownership, before.ownership);
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

export function comparisonSnapshotOptions(frame: any) {
    const [cads, surfaces, objects] = frame.sources;
    assert.equal(objects, 4, 'The comparison fixture retains exactly four CAD objects');
    assert.equal(cads.length, 4); assert.deepEqual(surfaces, []);
    assert(cads.every((cad: any) => Number.isSafeInteger(cad.id)));
    assert.equal(new Set(cads.map((cad: any) => cad.id)).size, 4, 'Four distinct CAD owners');
    assert.deepEqual(frame.clipping.rows, []); assert.equal(frame.clipping.usage[0], 0);
    assert.equal(frame.clipping.caps, 0); assert.equal(frame.clipping.error, null); assert.equal(frame.clipping.cap_error, null);
    return {lane: 'mesh', source: 'cad', expectedRows: 4, expectedQualities: 4};
}

export function surfaceSnapshotOptions(frame: any) {
    const [cads, surfaces, objects, , drawing] = frame.sources;
    assert.equal(objects, 2); assert.deepEqual(cads, []); assert.equal(surfaces.length, 2);
    assert(surfaces.every((surface: any) => Number.isSafeInteger(surface.id)));
    assert.equal(new Set(surfaces.map((surface: any) => surface.id)).size, 2);
    assert.equal(drawing.lines.length, 6); assert.deepEqual(drawing.points, []); assert.deepEqual(drawing.polylines, []);
    assert.deepEqual(frame.ownership.instances, []); assert.deepEqual(frame.ownership.curves, []);
    assert.deepEqual(frame.clipping.rows, []); assert.equal(frame.clipping.usage[0], 0); assert.equal(frame.clipping.caps, 0);
    assert.equal(frame.clipping.error, null); assert.equal(frame.clipping.cap_error, null); assert.equal(frame.clipping.ambient_error, null);
    assert.equal(frame.clipping.appearance.arctic, true); assert.equal(frame.clipping.appearance.outlines, true);
    return {lane: 'mesh', source: 'surface', expectedRows: 2, expectedQualities: 2};
}

export function ordinaryImageOwners(frame: any) {
    const [cads, surfaces, objects, flags, drawing] = frame.sources, owner = frame.ownership;
    assert.equal(objects, 2); assert.deepEqual(cads, []); assert.deepEqual(surfaces, []);
    assert.deepEqual(drawing, {points: [], lines: [], polylines: []});
    assert.deepEqual(owner.curves, []); assert.deepEqual(owner.instances, []); assert.deepEqual(frame.selected, []);
    assert.equal(owner.residency.length, 2); assert.equal(owner.metadata.length, 2); assert.equal(owner.placements.length, 2);
    const guids = owner.residency.map((row: any) => {
        assert.equal(row.length, 3); assert.equal(typeof row[0], 'string'); assert(row[0]);
        assert.equal(row[1], true); assert.equal(row[2], null); return row[0];
    }).sort();
    assert.equal(new Set(guids).size, 2);
    assert.deepEqual(owner.metadata.map((row: any) => {
        assert.equal(row.length, 5); assert.equal(typeof row[1], 'string'); assert.equal(typeof row[2], 'string');
        assert.equal(row[3], true); assert.equal(row[4], false); return row[0];
    }).sort(), guids);
    assert(Array.isArray(owner.origins));
    const ids = owner.placements.map((row: any) => {
        assert.equal(row.length, 2); const match = /^ObjectId\((\d+)\)$/.exec(row[0]); assert(match);
        assert(Array.isArray(row[1]) && row[1].length === 16 && row[1].every(Number.isFinite));
        const id = Number(match[1]); assert(Number.isSafeInteger(id)); return id;
    }).sort((a: number, b: number) => a - b);
    assert.equal(new Set(ids).size, 2); assert.equal(flags.rows.length, 2);
    assert.deepEqual(flags.rows.map((row: any) => { assert.equal(row.visible, true); assert.equal(row.locked, false); return row.id; }).sort((a: number, b: number) => a - b), ids);
    assert.deepEqual(frame.clipping.rows, []); assert.equal(frame.clipping.usage[0], 0); assert.equal(frame.clipping.caps, 0);
    assert.equal(frame.clipping.error, null); assert.equal(frame.clipping.cap_error, null); assert.equal(frame.clipping.ambient_error, null);
    assert.equal(frame.clipping.appearance.arctic, true); assert.equal(frame.clipping.appearance.outlines, true);
}

export async function settleOrdinaryImage(page: any, readState: () => Promise<any>) {
    await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
    await page.waitForFunction(() => {
        const text = document.querySelector('canvas')!.getAttribute('data-math-display'); if (text === null) return false;
        const q = JSON.parse(text);
        return q && q.pending === false && q.limited === null && Array.isArray(q.qualities) && q.qualities.length === 0;
    }, null, {timeout: 30000});
    const read = async () => {
        const image = await readState(); ordinaryImageOwners(image);
        const diagnostics = await page.evaluate(() => {
            const value = (name: string) => JSON.parse(document.querySelector('canvas')!.getAttribute(name)!);
            return {quality: value('data-math-display'), cpu: value('data-cpu-usage'), stats: value('data-gpu-stats'), usage: value('data-gpu-usage')};
        });
        const {quality, cpu, stats, usage} = diagnostics;
        assert(quality); assert.equal(quality.pending, false); assert.equal(quality.limited, null); assert.deepEqual(quality.qualities, []);
        for (const value of [quality.bytes, quality.generations]) assert(Number.isSafeInteger(value) && value >= 0);
        for (const [tuple, length] of [[cpu, 6], [stats, 4], [usage, 4]] as const)
            assert(Array.isArray(tuple) && tuple.length === length && tuple.every((value: number) => Number.isSafeInteger(value) && value >= 0));
        assert.equal(usage[0], 2); assert.equal(usage[2], 2, 'The two authored triangles retain two live GPU rows');
        return {image, ...diagnostics};
    };
    const flush = () => page.evaluate(async () => {
        window.dispatchEvent(new Event('resize'));
        await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
    });
    await flush(); const first = await read();
    await flush(); const repeated = await read();
    assert.deepEqual(repeated, first, 'Two fresh same-camera redraws preserve complete CPU/GPU tuples and authored owners');
    return {kind: 'authored-mesh-static-reuse', expectedRows: 2, expectedQualities: 0, first, repeated};
}

export async function applyBrowserScale(page: any, profile: any, settle: (options: ReturnType<typeof capSnapshotOptions>) => Promise<any>) {
    if (!profile) return null;
    const state = () => {
        const canvas = document.querySelector('canvas')!, rect = canvas.getBoundingClientRect();
        const value = (name: string) => JSON.parse(canvas.getAttribute(name)!);
        const clipping = value('data-clipping');
        return {css: [rect.width, rect.height], internal: [canvas.width, canvas.height], dpr: devicePixelRatio,
            requestedScale: (window as any).__courseImageRenderScale || 1, camera: value('data-camera-matrix'),
            sources: ['data-cads', 'data-surfaces', 'data-object-count', 'data-view-flags', 'data-drawing-originals', 'data-source-metadata'].map(value), selected: value('data-selected-ids'),
            ownership: {instances: value('data-instances'), curves: value('data-curve-originals'), residency: value('data-source-residency'),
                metadata: value('data-row-metadata'), origins: value('data-source-origins'), placements: value('data-row-placements')},
            clipping: {usage: clipping.usage.slice(0, 3), rows: clipping.rows, caps: clipping.caps[1], error: clipping.error, cap_error: clipping.cap_error, ambient_error: clipping.ambient_error,
                appearance: Object.fromEntries(['opacity', 'outlines', 'arctic'].filter(key => Object.hasOwn(clipping, key)).map(key => [key, clipping[key]]))},
            rendererProfile: value('data-image-render-profile')};
    };
    const before = await page.evaluate(state); assert.equal(before.requestedScale, 1);
    assert(!profile.scene || ['caps', 'comparison', 'surface-network', 'arctic-demo'].includes(profile.scene), 'Unknown image scene contract');
    const ordinary = profile.scene === 'arctic-demo', surface = profile.scene === 'surface-network';
    if (ordinary) ordinaryImageOwners(before);
    const options = ordinary ? null : surface ? surfaceSnapshotOptions(before) : profile.scene === 'comparison' ? comparisonSnapshotOptions(before) : capSnapshotOptions(before);
    await page.evaluate(() => { (window as any).__courseImageRenderScale = 2; window.dispatchEvent(new Event('resize')); });
    let settled: any;
    try {
        settled = ordinary ? await settleOrdinaryImage(page, () => page.evaluate(state)) : await settle(options!);
    }
    catch (error) {
        let finalState;
        try {
            finalState = await page.evaluate(() => {
                const canvas = document.querySelector('canvas');
                if (!canvas) return {canvas: null};
                const rect = canvas.getBoundingClientRect();
                const names = ['data-math-display', 'data-cpu-usage', 'data-gpu-stats', 'data-gpu-usage', 'data-path-gpu',
                    'data-curve-cpu', 'data-cads', 'data-surfaces', 'data-object-count', 'data-view-flags',
                    'data-camera-matrix', 'data-selected-ids', 'data-clipping', 'data-image-render-profile', 'data-status'];
                return {css: [rect.width, rect.height], internal: [canvas.width, canvas.height], dpr: devicePixelRatio,
                    requestedScale: (window as any).__courseImageRenderScale,
                    attributes: Object.fromEntries(names.map(name => [name, canvas.getAttribute(name)]))};
            });
        } catch (diagnosticError) { finalState = {readError: String(diagnosticError)}; }
        const diagnostic = {profile, options, before, finalState};
        try { (error as any).imageRenderScaleFailure = diagnostic; } catch {}
        try { console.error('Image render scale settle failure: ' + JSON.stringify(diagnostic)); } catch {}
        throw error;
    }
    if (!ordinary) {
        assert.equal(settled.adaptive, true, 'An image must retain accepted mathematical quality at the internal resolution');
        assert.equal(settled.quality.qualities.length, options!.expectedQualities);
        assert.deepEqual(settled.quality.qualities.map((row: any) => row[0]).sort((a: number, b: number) => a - b),
            before.sources[surface ? 1 : 0].map((owner: any) => owner.id).sort((a: number, b: number) => a - b), 'Every accepted mathematical quality belongs to the original CAD or surface owners');
    }
    const after = await page.evaluate(state); validateBrowserScale(before, after, 2);
    if (ordinary) ordinaryImageOwners(after);
    else if (surface) surfaceSnapshotOptions(after);
    else if (profile.scene === 'comparison') comparisonSnapshotOptions(after);
    else {
        assert.equal(after.clipping.caps, 1); assert.deepEqual(after.clipping.usage, [1, 1, 1]);
        assert.equal(after.clipping.rows.length, 1); assert.equal(after.clipping.error, null); assert.equal(after.clipping.cap_error, null);
    }
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
