import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import Module, {createRequire} from 'node:module';
import {fileURLToPath} from 'node:url';
import {bindings} from './presentation.ts';
import {observeGpu, privateBrowserHost} from './private-browser-host.ts';
import {docs, hash, json, read, viewer, write} from './model.ts';

const self = fileURLToPath(import.meta.url), id = '68ac-connect-current-pages-to-the-browser-renderer';
const sha = (file: string) => hash(fs.readFileSync(file));
const ref = (file: string) => ({path: path.resolve(file), sha256: sha(path.resolve(file))});

function check(prepared: any) {
    assert.deepEqual(bindings(id), prepared.source);
    for (const [file, digest] of Object.entries(prepared.pins)) assert.equal(sha(file), digest, file);
    for (const [name, digest] of Object.entries(prepared.bundle)) assert.equal(sha(path.join(prepared.dist, name)), digest, name);
}

export function prepareStreaming(builtFile: string) {
    builtFile = path.resolve(builtFile); const built = json(builtFile), source = bindings(id);
    assert.equal(built.id, id); assert.equal(built.density, 2); assert.deepEqual(built.source, source);
    assert(['BUILT_NOT_CAPTURED', 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW'].includes(built.status));
    const callback = path.join(docs, 'journey/checks/browser/streaming-visual.cjs');
    assert(read(callback).endsWith('module.exports.feature = feature;\r\n') || read(callback).endsWith('module.exports.feature = feature;\n'));
    const pins = Object.fromEntries([self, builtFile, path.join(docs, 'capture_journey.cjs'), callback,
        path.join(docs, 'journey/checks/browser/visual-originals.json'), path.join(docs, 'journey/checks/browser/canvas-image.cjs'),
        fileURLToPath(new URL('private-browser-host.ts', import.meta.url)), path.join(docs, 'journey/course.json')].map(file => [file, sha(file)]));
    const dist = path.join(built.workspace, 'target/course-checks', id, 'dist');
    const key = hash(JSON.stringify({source, pins, bundle: built.bundle})).slice(0, 20);
    const directory = path.join(viewer, 'target/course-presentation', 'streaming-feature-' + key);
    assert(!fs.existsSync(directory), 'Preserve earlier attempt');
    const record = {status: 'PREPARED_UNCHANGED_FEATURE_ONLY', id, directory, workspace: built.workspace, dist, source, pins, bundle: built.bundle,
        built: ref(builtFile), imageDpr: 2, expectedViewport: {width: 1100, height: 800},
        scope: 'Only the original exported streaming-visual.feature runs. Its inherited full callback failed earlier and is not rerun or accepted.',
        expected: {cads: 1, surfaces: 1, rangedOwners: 2, uniqueStreamSources: 1, sourcePoints: 2000064, pointSizes: [12, 7], relativeX: 2}};
    check(record); write(path.join(directory, 'prepared.json'), JSON.stringify(record, null, 2) + '\n'); return record;
}

async function worker(configFile: string) {
    const prepared = json(configFile); check(prepared);
    const require = createRequire(path.join(docs, 'capture_journey.cjs'));
    const {chromium} = require('playwright');
    const captureFile = path.join(docs, 'capture_journey.cjs'), original = read(captureFile), boundary = original.indexOf('\nasync function capture()');
    assert(boundary > 0);
    const args = process.argv; process.argv = [args[0], captureFile, id];
    let helpers: any;
    try {
        const runner = new Module(captureFile); runner.filename = captureFile; runner.paths = (Module as any)._nodeModulePaths(path.dirname(captureFile));
        (runner as any)._compile(original.slice(0, boundary) + '\nmodule.exports={command,drawing};', captureFile); helpers = runner.exports;
    } finally { process.argv = args; }
    helpers.step = json(path.join(docs, 'journey/course.json')).steps.find((step: any) => step.id === id);
    const browser = await chromium.launch({executablePath: process.env.VIEWER_BROWSER_EXECUTABLE, headless: true, args: JSON.parse(process.env.VIEWER_CHROME_ARGS!)});
    const context = await browser.newContext({viewport: {width: 900, height: 760}, deviceScaleFactor: 2, acceptDownloads: true});
    const page = await context.newPage(); page.setDefaultTimeout(120000); const errors: string[] = [];
    page.on('pageerror', (error: Error) => errors.push(String(error)));
    const AsyncFunction = Object.getPrototypeOf(async function() {}).constructor;
    await new AsyncFunction('page', observeGpu)(page);
    try {
        await page.goto(process.env.VIEWER_URL!);
        await page.waitForFunction(() => {
            const w = window as any;
            return w.wasmBindings?.runtime_running?.() && JSON.parse(w.wasmBindings.diagnostic_snapshot()).outcome === 'ready';
        });
        const feature = require(path.join(docs, 'journey/checks/browser/streaming-visual.cjs')).feature;
        const featureResult = await feature(page, helpers);
        await page.waitForFunction(() => {
            const c = document.querySelector('canvas') as HTMLCanvasElement;
            return devicePixelRatio === 2 && c.width === 2200 && c.height === 1600;
        });
        const actualGpu = await page.evaluate(async () => {
            const w = window as any;
            await Promise.all(w.__presentationDevices.map((device: any) => device.queue.onSubmittedWorkDone()));
            await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
            return {adapters: w.__presentationAdapters, errors: w.__presentationErrors};
        });
        assert(actualGpu.adapters.length); for (const adapter of actualGpu.adapters) { assert.match(adapter.vendor, /nvidia/i); assert.equal(adapter.fallback, false); }
        assert.deepEqual(actualGpu.errors, []); assert.deepEqual(errors, []);
        assert.deepEqual(page.viewportSize(), prepared.expectedViewport);
        const state = await page.evaluate(() => ({status: document.querySelector('#status')?.textContent,
            attributes: Object.fromEntries([...document.querySelector('canvas')!.attributes].filter(a => a.name.startsWith('data-')).map(a => [a.name, a.value]))}));
        assert.equal(state.status, 'URL scene replaced.');
        const image = path.join(prepared.directory, 'presentation.png');
        assert(!fs.existsSync(image)); await page.screenshot({path: image, scale: 'device', fullPage: false});
        const bytes = fs.readFileSync(image); assert.equal(bytes.readUInt32BE(16), 2200); assert.equal(bytes.readUInt32BE(20), 1600);
        check(prepared);
        write(path.join(prepared.directory, 'capture.json'), JSON.stringify({status: 'PRESENTATION_ONLY_AWAITING_ROOT_VISUAL_REVIEW', source: prepared.source,
            originalInheritedCallbackExecuted: false, unchangedExportedFeatureExecuted: true, callback: ref(path.join(docs, 'journey/checks/browser/streaming-visual.cjs')),
            featureResult, state, actualGpu, browser: browser.version(), image: {path: image, sha256: sha(image), width: 2200, height: 1600}}, null, 2) + '\n');
    } catch (error) {
        const state = await page.evaluate(() => ({status: document.querySelector('#status')?.textContent,
            attributes: Object.fromEntries([...(document.querySelector('canvas')?.attributes || [])].map(a => [a.name, a.value]))})).catch(() => null);
        write(path.join(prepared.directory, 'failure.json'), JSON.stringify({error: String((error as Error).stack || error), state, errors}, null, 2));
        await page.screenshot({path: path.join(prepared.directory, 'failure.png')}).catch(() => {}); throw error;
    } finally { await browser.close(); }
}

export async function runStreaming(preparedFile: string) {
    const prepared = json(path.resolve(preparedFile)); check(prepared);
    const host = await privateBrowserHost(prepared.workspace, prepared.directory, self, prepared);
    check(prepared);
    for (const [name, digest] of Object.entries(prepared.bundle)) if (host.served[`${id}/dist/${name}`]) assert.equal(host.served[`${id}/dist/${name}`], digest);
    assert.equal(host.served[`${id}/dist/viewer_journey_bg.wasm`], prepared.bundle['viewer_journey_bg.wasm']);
    const record = {...prepared, ...json(path.join(prepared.directory, 'capture.json')), host};
    write(path.join(prepared.directory, 'result.json'), JSON.stringify(record, null, 2) + '\n'); return record;
}

if (process.argv[1] && path.resolve(process.argv[1]) === self) {
    const [mode, file] = process.argv.slice(2);
    if (mode === '--worker') await worker(file);
    else {
        const result = mode === 'prepare' ? prepareStreaming(file) : mode === 'run' ? await runStreaming(file) : null;
        assert(result, 'Usage: browser-streaming-presentation.ts prepare BUILT_BROWSER_PREPARED | run PREPARED'); console.log(JSON.stringify(result, null, 2));
    }
}
