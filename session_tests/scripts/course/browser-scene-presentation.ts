import fs from 'node:fs';
import path from 'node:path';
import http from 'node:http';
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {createRequire} from 'node:module';
import {fileURLToPath} from 'node:url';
import {bindings} from './presentation.ts';
import {course, docs, filesIn, hash, json, read, viewer, write} from './model.ts';

const self = fileURLToPath(import.meta.url);
const sha = (file: string) => hash(fs.readFileSync(file));
const browserExecutable = () => process.env.VIEWER_BROWSER_EXECUTABLE || path.join(viewer, 'target/course-browsers/chromium_headless_shell-1208/chrome-headless-shell-win64/chrome-headless-shell.exe');
const adapterMetadata = path.join(viewer, 'target/course-windows-20261018/nvidia-hardware-browser-preparation/adapter-metadata.json');
const comparisonCheck = 'journey/checks/browser/comparison.cjs';
const cloudCheck = 'journey/checks/browser/cloud-points.cjs';
const networkCheck = 'journey/checks/browser/network.cjs';
const instanceChecks = ['scene-reload', 'record-properties', 'arena-reclamation', 'arena-empty'].map(name => 'journey/checks/browser/' + name + '.cjs');
const contactChecks = ['target-resize', 'quality-target-reuse', 'quality-target-reuse-with-policy'].map(name => 'journey/checks/browser/' + name + '.cjs');

export function sceneRecipe(id: string, step = course().steps.find(step => step.id === id)) {
    assert(step, 'Unknown checkpoint');
    if (step.browser_check === 'journey/checks/browser/stroke-input.cjs'
        && JSON.stringify(step.browser_actions) === JSON.stringify([{file: 'sample.pb'}, {command: 'Close'}, {command: 'Example Line'}, {command: 'View Reset'}]))
        return {kind: 'documented-example-line', callback: step.browser_check,
            meaning: 'The original Example Line after the documented Close and View Reset; recovery/stroke tests use separate contexts.',
            asset: null, commands: ['Close', 'Example Line', 'View Reset']};
    if (step.browser_actions?.length) return {kind: 'full-canonical-fallback', callback: step.browser_check || null,
        reason: 'This checkpoint adds actions outside the reviewed final-scene recipe.'};
    if (step.browser_check === comparisonCheck) return {kind: 'retained-four-brep', callback: step.browser_check,
        meaning: 'The original four comparison BReps in the fitted isometric, opaque outline view.',
        asset: 'assets/pb/course-comparison.pb', commands: ['Layers Off', 'View Grid Off', 'View Isometric', 'Opacity 1', 'View Outline On', 'Fit', 'Opacity 0.4', 'Opacity 1']};
    if (step.browser_check === cloudCheck) return {kind: 'selected-original-cloud', callback: step.browser_check,
        meaning: 'The original 1681-point Example Cloud with its selected original control points.',
        asset: null, commands: ['Close', 'Layers Off', 'View Grid Off', 'Example Cloud', 'Fit', 'View Isometric', 'Select Next', 'Controls']};
    if (step.browser_check === networkCheck) return {kind: 'two-original-nurbs-surfaces', callback: step.browser_check,
        meaning: 'The original six-curve network surface beside the four-point NURBS patch, unselected and fitted isometrically.',
        asset: null, commands: ['Close', 'Line w0,1,0 w2,1,1', 'Line w2,0,0 w2,2,2', 'Line w2,0,0 w0,0,0', 'Line w1,0,0 w1,2,1',
            'Line w0,2,0 w2,2,2', 'Line w0,0,0 w0,2,0', 'Select All', 'Nurbs Surface Network',
            'Nurbs Surface 4 Points w3,0,0 w4,0,0 w4,1,0.3 w3,1,0', 'View Isometric', 'Select None', 'Fit']};
    if (instanceChecks.includes(step.browser_check!)) return {kind: 'ordinary-and-instance-slots', callback: step.browser_check,
        meaning: 'The original page retains its startup originals, the current-command closed triangle, and two original definition instances. Reload, ambient-property and arena exercises run in separate contexts.',
        asset: null, commands: ['Layers Off', 'View Grid Off', 'Snap Off', 'Grid Snap Off', 'CPlane XY',
            'Polyline', 'w0,0,0', 'w1,0,0', 'w1,1,0', 'Close', 'Opacity 1', 'Fit',
            'Example Instances', 'Fit', 'View Isometric', 'Select None', 'Opacity 1', 'Close']};
    if (contactChecks.includes(step.browser_check!)) return {kind: 'original-contact-panel', callback: step.browser_check,
        meaning: 'The retained ordinary contact-panel archive, top orthographic Arctic view. Temporary resize/quality contexts are not the published original page.',
        asset: null, commands: ['Layers Off', 'Arctic On', 'View Outline Off', 'View Grid Off', 'View Lighting Off', 'View Backfaces Off', 'Select None', 'Snap Off', 'View Top', 'View Orthographic', 'Fit']};
    return {kind: 'full-canonical-fallback', callback: step.browser_check || null,
        reason: 'No reviewed deterministic final-scene mapping. Use browser-presentation.ts build/run for this checkpoint.'};
}

function validate(prepared: any) {
    assert.equal(sha(self), prepared.helperSha256);
    assert.deepEqual(bindings(prepared.id), prepared.source, 'Source/dependency inputs changed');
    assert.deepEqual(sceneRecipe(prepared.id), prepared.recipe);
    for (const [file, digest] of Object.entries(prepared.pins)) assert.equal(sha(file), digest, file);
    for (const [name, digest] of Object.entries(prepared.bundle)) assert.equal(sha(path.join(prepared.dist, name)), digest, name);
    if (prepared.baseline) assert.equal(sha(prepared.baseline.path), prepared.baseline.sha256);
}

export function prepareScene(id: string, browserPreparation: string, baselineFile?: string) {
    const recipe = sceneRecipe(id);
    assert.notEqual(recipe.kind, 'full-canonical-fallback', JSON.stringify(recipe));
    browserPreparation = path.resolve(browserPreparation);
    const built = json(browserPreparation), source = bindings(id);
    assert.equal(built.id, id);
    assert(['BUILT_NOT_CAPTURED', 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW'].includes(built.status));
    assert.deepEqual(built.source, source, 'Use a build of this actual checkpoint');
    const dist = path.join(built.workspace, 'target/course-checks', id, 'dist');
    assert(built.bundle?.['index.html']);
    const bundle = Object.fromEntries(filesIn(dist).map(file => [path.relative(dist, file).split(path.sep).join('/'), sha(file)]));
    assert.deepEqual(bundle, built.bundle);
    assert(Object.keys(bundle).some(name => name.endsWith('.wasm')), 'Actual WASM bundle required');
    const step = course().steps.find(step => step.id === id)!;
    const callback = path.join(docs, step.browser_check!);
    const original = read(callback);
    const extraPins: string[] = [];
    let fixture: any = null;
    if (recipe.kind === 'retained-four-brep') {
        assert(original.includes("fetch('assets/pb/course-comparison.pb')"));
        assert(original.includes(JSON.stringify(recipe.commands!.slice(0, 6)).replaceAll('"', "'")));
        assert(original.includes("await helpers.command(page,'Opacity 0.4');await helpers.command(page,'Opacity 1')"));
        assert.equal(sha(path.join(dist, recipe.asset!)), source.assets[recipe.asset!]);
    } else if (recipe.kind === 'ordinary-and-instance-slots') {
        for (const name of ['current-command', 'selection-palette', 'ordinary-slot', 'scene-reload', 'record-properties', 'arctic-background', 'arena-reclamation'])
            extraPins.push(path.join(docs, 'journey/checks/browser', name + '.cjs'));
        const slot = read(path.join(docs, 'journey/checks/browser/ordinary-slot.cjs'));
        assert(slot.includes("['Example Instances','Fit','View Isometric','Select None','Opacity 1']"));
        assert(slot.includes("await helpers.command(page,'Close');assert.deepEqual(await value('data-drawing-originals'),document)"));
    } else if (recipe.kind === 'original-contact-panel') {
        const file = path.join(docs, 'journey/checks/browser/ambient-contacts-original.json');
        fixture = json(file).fixtures.ordinary;
        assert.equal(hash(Buffer.from(fixture.proto, 'base64')), fixture.sourceSha256);
        extraPins.push(file, path.join(docs, 'journey/checks/browser/ambient-contacts.cjs'));
        assert(read(extraPins.at(-1)!).includes(JSON.stringify(recipe.commands).replaceAll('"', "'")));
        assert(original.includes("await contacts.open(page,'ordinary',h)") || original.includes('await reuse(page,h)'));
    } else if (recipe.kind === 'documented-example-line') {
        extraPins.push(path.join(docs, 'journey/checks/browser/recovery-browser.cjs'), path.join(docs, 'journey/checks/browser/document-phases.cjs'));
        assert(original.includes('finally { await context.close(); await original.bringToFront(); }'));
    } else assert(original.includes(JSON.stringify(recipe.commands).replaceAll('"', "'")), 'Exact original callback tail required');
    const baseline = baselineFile ? {path: path.resolve(baselineFile), sha256: sha(path.resolve(baselineFile))} : null;
    if (baseline) assert.deepEqual(json(baseline.path).source, source, 'Baseline must name the same current source');
    const executablePath = browserExecutable();
    const pins = Object.fromEntries([browserPreparation, callback, ...extraPins, path.join(docs, 'journey/course.json'), executablePath, adapterMetadata].map(file => [file, sha(file)]));
    const key = hash(JSON.stringify({source, recipe, bundle, baseline, helper: sha(self)})).slice(0, 20);
    const directory = path.join(viewer, 'target/course-presentation', 'scene-' + id + '-' + key);
    assert(!fs.existsSync(directory), 'Preserve previous scene attempt');
    const prepared = {status: 'PREPARED_IMAGE_ONLY_NOT_EXECUTED', id, directory, source, recipe, pins, dist, bundle,
        helperSha256: sha(self), executablePath, baseline, fixture, viewport: {width: 900, height: 760}, density: 2,
        terminalStatus: recipe.kind === 'retained-four-brep' ? 'File imported. Undo removes the entire import.'
            : recipe.kind === 'ordinary-and-instance-slots' ? 'No point tool is running'
            : recipe.kind === 'original-contact-panel' ? 'Document replaced. Undo restores the previous document.' : 'Document closed.',
        declaredMetadataStatus: step.browser_result_status || step.browser_status,
        scope: 'Presentation only; no canonical callback, historical behavior revalidation, or full verify execution is claimed.',
        sceneContract: recipe.kind === 'retained-four-brep' ? {cads: 4, selected: 0, grid: false, opacity: 1, outlines: true}
            : recipe.kind === 'selected-original-cloud' ? {clouds: 1, points: 1681, selected: 1, pickMode: 'Control', grid: false}
            : recipe.kind === 'two-original-nurbs-surfaces' ? {surfaces: 2, selected: 0, gather: null}
            : recipe.kind === 'ordinary-and-instance-slots' ? {instances: 2, surfaceDraws: 4, selected: 0, closedTriangle: [[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 0, 0]]}
            : recipe.kind === 'original-contact-panel' ? {retainedArchive: fixture.sourceSha256, originalElement: fixture.elementGuid, selected: 0, arctic: true, outlines: false}
            : {strokeDraws: 1, strokeVertices: 44}};
    write(path.join(directory, 'prepared.json'), JSON.stringify(prepared, null, 2) + '\n');
    validate(prepared);
    return prepared;
}

function semanticState(attributes: Record<string, string>) {
    const parsed = (name: string) => attributes[name] === undefined ? null : JSON.parse(attributes[name]);
    const clipping = parsed('data-clipping');
    return {camera: parsed('data-camera-matrix'), frame: parsed('data-camera-frame'), projection: attributes['data-projection'],
        grid: attributes['data-grid-enabled'], pickMode: attributes['data-pick-mode'], selectedCount: parsed('data-selected-ids')?.length,
        cads: parsed('data-cads')?.map(({source, faces, edges, vertices}: any) => ({source, faces, edges, vertices})),
        surfaces: parsed('data-surfaces')?.map(({controls, steps, samples, faces, edges}: any) => ({controls, steps, samples, faces, edges})),
        instances: parsed('data-instances')?.map(({definition_guid, model}: any) => ({definition_guid, model})),
        drawing: parsed('data-drawing-originals'),
        clouds: parsed('data-cloud-sources')?.map(({points, nodes, model, point_size, point_px}: any) => ({points, nodes, model, point_size, point_px})),
        settings: clipping ? {opacity: clipping.opacity, outlines: clipping.outlines, arctic: clipping.arctic} : null};
}

async function worker(preparedFile: string, base: string) {
    const prepared = json(preparedFile); validate(prepared);
    const require = createRequire(path.join(viewer, 'target/course-tools/package.json'));
    const {chromium} = require('playwright');
    const metadata = json(adapterMetadata);
    const browser = await chromium.launch({executablePath: prepared.executablePath, headless: true, args: ['--enable-unsafe-webgpu', '--ignore-gpu-blocklist', '--use-gl=angle', '--use-angle=d3d11',
        '--use-adapter-luid=' + metadata.selected.luid, '--enable-dawn-features=d3d_skip_shader_optimizations']});
    const errors: string[] = [];
    const context = await browser.newContext({viewport: prepared.viewport, deviceScaleFactor: 2});
    await context.addInitScript(() => {
        const w = window as any; w.__sceneDevices = []; w.__sceneAdapters = []; w.__sceneErrors = [];
        if (!(globalThis as any).GPUAdapter) return;
        const original = (globalThis as any).GPUAdapter.prototype.requestDevice;
        (globalThis as any).GPUAdapter.prototype.requestDevice = async function(...args: any[]) {
            const device = await original.apply(this, args), info = this.info;
            w.__sceneAdapters.push({vendor: info?.vendor, description: info?.description, fallback: info?.isFallbackAdapter});
            w.__sceneDevices.push(device);
            device.addEventListener('uncapturederror', (event: any) => w.__sceneErrors.push(event.error.message));
            device.lost.then((info: any) => w.__sceneErrors.push('device lost: ' + info.reason)); return device;
        };
    });
    const page = await context.newPage(); page.setDefaultTimeout(90000); page.on('pageerror', (error: Error) => errors.push(String(error)));
    page.on('console', (message: any) => { if (message.type() === 'error') errors.push(message.text()); });
    const command = async (line: string) => {
        await page.waitForFunction(() => {
            const c = document.querySelector('canvas')!, ui = JSON.parse(c.getAttribute('data-command-ui') || '{}');
            const f = ui.controls?.find((item: any) => item.key === 'command/input');
            return f && f.rect[3] <= c.getBoundingClientRect().height;
        });
        const ui = JSON.parse(await page.locator('canvas').getAttribute('data-command-ui'));
        const field = ui.controls.find((item: any) => item.key === 'command/input'), box = await page.locator('canvas').boundingBox();
        await page.mouse.click(box.x + (field.rect[0] + field.rect[2]) / 2, box.y + field.rect[3] - 2);
        await page.waitForFunction(() => JSON.parse(document.querySelector('canvas')!.getAttribute('data-command-ui')!).focused);
        if (ui.command) await page.keyboard.press('ControlOrMeta+A');
        await page.keyboard.type(line, {delay: 0});
        await page.waitForFunction((line: string) => JSON.parse(document.querySelector('canvas')!.getAttribute('data-command-ui')!).command === line, line);
        await page.keyboard.press('Enter');
        await page.waitForFunction((length: number) => {
            const ui = JSON.parse(document.querySelector('canvas')!.getAttribute('data-command-ui')!);
            return ui.command === '' && ui.history.length === Math.min(200, length + 1);
        }, ui.history.length);
        const current = JSON.parse(await page.locator('canvas').getAttribute('data-command-ui'));
        assert([line, line.replace(/^CPlane(?=\s|$)/i, 'Construction Plane')].some(text => current.history.at(-1).startsWith('> ' + text + '\n')), 'Actual submitted command');
        assert(!current.history.at(-1).includes('Unknown command'));
    };
    try {
        const address = new URL(base);
        if (prepared.recipe.kind === 'original-contact-panel') {
            address.searchParams.set('nogrid', '1'); address.searchParams.set('opacity', '1');
            if (prepared.recipe.callback !== 'journey/checks/browser/target-resize.cjs') address.searchParams.set('msaa', '1');
        }
        await page.goto(address.href);
        await page.waitForFunction(() => {
            const w = window as any;
            return w.wasmBindings?.runtime_running?.() && JSON.parse(w.wasmBindings.diagnostic_snapshot()).outcome === 'ready';
        });
        const step = course().steps.find(step => step.id === prepared.id)!;
        assert.equal(await page.locator('#status').textContent(), step.browser_status);
        if (prepared.recipe.kind === 'retained-four-brep') {
            await command('Close');
            const bytes = fs.readFileSync(path.join(prepared.dist, prepared.recipe.asset));
            const choosing = page.waitForEvent('filechooser'); await command('Open');
            await (await choosing).setFiles({name: 'comparison.pb', mimeType: 'application/octet-stream', buffer: bytes});
            await page.waitForFunction(() => JSON.parse(document.querySelector('canvas')!.getAttribute('data-cads')!).length === 4);
        } else if (prepared.recipe.kind === 'original-contact-panel') {
            const bytes = Buffer.from(prepared.fixture.proto, 'base64'); assert.equal(hash(bytes), prepared.fixture.sourceSha256);
            const choosing = page.waitForEvent('filechooser'); await command('Open Replace');
            await (await choosing).setFiles({name: 'original-contact-ambient.pb', mimeType: 'application/octet-stream', buffer: bytes});
            await page.waitForFunction(() => document.querySelector('#status')?.textContent === 'Document replaced. Undo restores the previous document.');
        }
        for (const line of prepared.recipe.commands) await command(line);
        await page.waitForFunction(({kind, status}: any) => {
            const c = document.querySelector('canvas')!, value = (name: string) => JSON.parse(c.getAttribute(name) || 'null');
            if (document.querySelector('#status')?.textContent !== status) return false;
            if (kind === 'retained-four-brep') return value('data-cads')?.length === 4;
            if (kind === 'selected-original-cloud') return value('data-cloud-sources')?.[0]?.points === 1681 && value('data-cloud-gpu')?.[0] === 1681;
            if (kind === 'two-original-nurbs-surfaces') return value('data-surfaces')?.length === 2 && value('data-gather') === null;
            if (kind === 'ordinary-and-instance-slots') return value('data-instances')?.length === 2 && value('data-gpu-usage')?.[2] === 4;
            if (kind === 'documented-example-line') return value('data-stroke-usage')?.[0] === 1;
            return value('data-clipping')?.arctic && value('data-gpu-usage')?.[0] > 0;
        }, {kind: prepared.recipe.kind, status: prepared.terminalStatus});
        await page.waitForFunction(() => {
            const c = document.querySelector('canvas') as HTMLCanvasElement, rect = c.getBoundingClientRect();
            return devicePixelRatio === 2 && c.width === Math.round(rect.width * 2) && c.height === Math.round(rect.height * 2);
        });
        await page.evaluate(async () => {
            await Promise.all((window as any).__sceneDevices.map((device: any) => device.queue.onSubmittedWorkDone()));
            await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
        });
        const state = await page.evaluate(() => {
            const c = document.querySelector('canvas') as HTMLCanvasElement, w = window as any;
            return {attributes: Object.fromEntries([...c.attributes].filter(a => a.name.startsWith('data-')).map(a => [a.name, a.value])),
                canvas: {width: c.width, height: c.height, rect: {x: c.getBoundingClientRect().x, y: c.getBoundingClientRect().y, width: c.getBoundingClientRect().width, height: c.getBoundingClientRect().height}},
                status: document.querySelector('#status')?.textContent, adapters: w.__sceneAdapters, gpuErrors: w.__sceneErrors};
        });
        write(path.join(prepared.directory, 'actual-scene.json'), JSON.stringify(state, null, 2) + '\n');
        const semantic = semanticState(state.attributes);
        assert.equal(state.status, prepared.terminalStatus);
        assert.deepEqual(state.canvas.rect, {x: 0, y: 0, ...prepared.viewport});
        assert.deepEqual(state.gpuErrors, []); assert.deepEqual(errors, []);
        assert(state.adapters.length > 0);
        for (const adapter of state.adapters) { assert.match(adapter.vendor, /nvidia/i); assert.equal(adapter.fallback, false); }
        if (['retained-four-brep', 'selected-original-cloud', 'ordinary-and-instance-slots', 'original-contact-panel'].includes(prepared.recipe.kind)) assert.equal(semantic.grid, 'false');
        assert.equal(semantic.camera?.length, 16); assert(semantic.camera.every(Number.isFinite));
        if (prepared.recipe.kind === 'retained-four-brep') {
            assert.equal(semantic.cads?.length, 4); assert.equal(semantic.selectedCount, 0);
            assert(semantic.cads!.every((cad: any) => cad.source && cad.faces.length > 0 && cad.edges.length > 0));
            assert.equal(semantic.settings?.opacity, 1); assert.equal(semantic.settings?.outlines, true);
            assert.equal(semantic.settings?.arctic, false);
        } else if (prepared.recipe.kind === 'selected-original-cloud') {
            assert.equal(semantic.clouds?.length, 1); assert.equal(semantic.clouds![0].points, 1681);
            assert(semantic.clouds![0].nodes > 0); assert.equal(semantic.clouds![0].model.length, 16);
            assert.equal(semantic.selectedCount, 1); assert.equal(semantic.pickMode, 'Control');
            assert.equal(state.attributes['data-cloud-error'], '');
            assert.equal(JSON.parse(state.attributes['data-cloud-gpu'])[0], 1681);
        } else if (prepared.recipe.kind === 'two-original-nurbs-surfaces') {
            assert.equal(semantic.surfaces?.length, 2); assert.equal(semantic.selectedCount, 0);
            assert.equal(JSON.parse(state.attributes['data-gather']), null);
        } else if (prepared.recipe.kind === 'ordinary-and-instance-slots') {
            assert.equal(semantic.instances?.length, 2); assert.equal(semantic.selectedCount, 0);
            assert(semantic.instances![0].definition_guid);
            assert.equal(semantic.instances![0].definition_guid, semantic.instances![1].definition_guid);
            assert.equal(JSON.parse(state.attributes['data-gpu-usage'])[2], 4);
            assert(semantic.drawing.polylines.some((points: any) => JSON.stringify(points) === JSON.stringify(prepared.sceneContract.closedTriangle)));
            assert.equal(semantic.settings?.opacity, 1);
        } else if (prepared.recipe.kind === 'original-contact-panel') {
            const nodes = JSON.parse(state.attributes['data-original-tree-placements']);
            assert.equal(nodes.filter((node: any) => node.name === prepared.fixture.elementGuid && node.object !== null).length, 1);
            assert.equal(semantic.selectedCount, 0); assert.equal(semantic.projection, 'Orthographic');
            assert.equal(semantic.settings?.arctic, true); assert.equal(semantic.settings?.outlines, false); assert.equal(semantic.settings?.opacity, 1);
        } else {
            assert.deepEqual(JSON.parse(state.attributes['data-stroke-usage']).slice(0, 2), [1, 44]);
        }
        if (prepared.baseline) {
            const baseline = json(prepared.baseline.path);
            assert.deepEqual(semantic, semanticState(baseline.attributes || baseline.state?.attributes));
            assert.equal(state.status, baseline.status || baseline.state?.status);
        }
        const image = path.join(prepared.directory, 'presentation.png');
        assert(!fs.existsSync(image)); await page.screenshot({path: image, scale: 'device', fullPage: false});
        const bytes = fs.readFileSync(image);
        assert.equal(bytes.readUInt32BE(16), 1800); assert.equal(bytes.readUInt32BE(20), 1520);
        const pixels = require('pngjs').PNG.sync.read(bytes);
        let ink = 0, yellow = 0;
        for (let y = 20; y < pixels.height - 120; y++) for (let x = 20; x < pixels.width - 20; x++) {
            const i = (y * pixels.width + x) * 4, [r, g, b] = pixels.data.subarray(i, i + 3);
            if (Math.min(r, g, b) < 220) ink++;
            if (r > 180 && g > 120 && b < 80) yellow++;
        }
        assert(ink > 100, 'Actual geometry must be visible above the command dock');
        if (prepared.recipe.kind === 'selected-original-cloud') assert(yellow > 10, 'Actual selected cloud control markers must be visible');
        validate(prepared);
        write(path.join(prepared.directory, 'scene.json'), JSON.stringify({status: 'PRESENTATION_ONLY_AWAITING_ROOT_VISUAL_REVIEW', source: prepared.source,
            canonicalBehaviorExecuted: false, recipe: prepared.recipe, semantic, state, url: page.url(), visiblePixels: {ink, yellow}, baselineCompared: !!prepared.baseline,
            declaredMetadataStatus: prepared.declaredMetadataStatus, terminalMatchesMetadata: state.status === prepared.declaredMetadataStatus,
            publication: 'Not published. Root scene/image review is required; old canonical behavior proofs remain separately source-bound.',
            baselineScope: 'Source identity/GUIDs of synthetic clouds are not compared across fresh contexts; point/node/model/size and selection counts are compared. Retained BRep source GUIDs are compared.',
            image: {path: image, sha256: sha(image), width: 1800, height: 1520}, browser: browser.version()}, null, 2) + '\n');
    } catch (error) {
        await page.screenshot({path: path.join(prepared.directory, 'failure.png')}).catch(() => {});
        write(path.join(prepared.directory, 'failure.json'), JSON.stringify({error: String((error as Error).stack || error), errors}, null, 2)); throw error;
    } finally { await browser.close(); }
}

export async function runScene(preparedFile: string) {
    preparedFile = path.resolve(preparedFile); const prepared = json(preparedFile); validate(prepared);
    const served: Record<string, string> = {}, mime: Record<string, string> = {'.html': 'text/html', '.js': 'text/javascript', '.wasm': 'application/wasm', '.json': 'application/json', '.ttf': 'font/ttf'};
    const server = http.createServer((request, response) => {
        try {
            const url = new URL(request.url!, 'http://localhost'), name = decodeURIComponent(url.pathname).replace(/^\//, '') || 'index.html';
            const file = path.resolve(prepared.dist, name);
            assert(file.startsWith(prepared.dist + path.sep));
            const bytes = fs.readFileSync(file); assert.equal(hash(bytes), prepared.bundle[name]); served[name] = hash(bytes);
            response.writeHead(200, {'Content-Type': mime[path.extname(file)] || 'application/octet-stream', 'Cache-Control': 'no-store'}).end(bytes);
        } catch { response.writeHead(404).end('Not found'); }
    });
    await new Promise<void>(resolve => server.listen(0, '127.0.0.1', resolve));
    const descriptor = fs.openSync(path.join(prepared.directory, 'browser.log'), 'wx');
    const started = Date.now();
    try {
        const code = await new Promise((resolve, reject) => {
            const child = spawn(process.execPath, [path.resolve(viewer, '../session_tests/scripts/run-guarded.cjs'), '-MemoryLimitMiB', '6144', '-TimeoutSeconds', '600',
                '-WorkingDirectory', viewer, '-ManifestPath', path.join(prepared.directory, 'resources.json'), process.execPath,
                '--import', new URL('../typescript.mjs', import.meta.url).href, self, 'worker', preparedFile, 'http://127.0.0.1:' + (server.address() as any).port + '/'],
            {cwd: viewer, windowsHide: true, stdio: ['ignore', descriptor, descriptor], env: {...process.env}});
            child.once('error', reject); child.once('exit', resolve);
        });
        assert.equal(code, 0, 'Presentation failed; preserve browser.log/failure evidence'); validate(prepared);
        const resources = json(path.join(prepared.directory, 'resources.json')); assert.equal(resources.exitCode, 0); assert.equal(resources.processors, 2);
        assert.equal(resources.jobCommitLimitBytes, 6442450944); assert.equal(resources.timedOut, false);
        assert.equal(served['index.html'], prepared.bundle['index.html']);
        for (const name of Object.keys(prepared.bundle).filter(name => name.endsWith('.wasm'))) assert.equal(served[name], prepared.bundle[name]);
        const result = {...prepared, ...json(path.join(prepared.directory, 'scene.json')), resources, served, elapsedSeconds: (Date.now() - started) / 1000,
            execution: {log: {path: path.join(prepared.directory, 'browser.log'), sha256: sha(path.join(prepared.directory, 'browser.log'))},
                resources: {path: path.join(prepared.directory, 'resources.json'), sha256: sha(path.join(prepared.directory, 'resources.json'))}}};
        write(path.join(prepared.directory, 'result.json'), JSON.stringify(result, null, 2) + '\n'); return result;
    } finally { fs.closeSync(descriptor); await new Promise<void>(resolve => server.close(() => resolve())); }
}

if (process.argv[1] && path.resolve(process.argv[1]) === self) {
    const [mode, id, built, baseline] = process.argv.slice(2);
    const result = mode === 'recipe' ? sceneRecipe(id) : mode === 'prepare' ? prepareScene(id, built, baseline)
        : mode === 'run' ? await runScene(id) : mode === 'worker' ? await worker(id, built) : undefined;
    if (mode !== 'worker') { assert(result, 'Usage: browser-scene-presentation.ts recipe ID | prepare ID BUILT_BROWSER_PREPARED [SOURCE_MATCHED_SCENE_BASELINE] | run PREPARED'); console.log(JSON.stringify(result, null, 2)); }
}
