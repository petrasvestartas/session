import fs from 'node:fs';
import path from 'node:path';
import http from 'node:http';
import assert from 'node:assert/strict';
import {spawn, spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {bindings, captureDirectory, prepare} from './presentation.ts';
import {assetsFor, course, docs, expected, filesIn, hash, json, lockFor, materialize, read, viewer, write} from './model.ts';
import {nativeTarget, runCommand} from './verify.ts';

const sha = (file: string) => hash(fs.readFileSync(file));
const rootFor = (id: string, density: number) => {
    const key = hash(JSON.stringify({source: bindings(id), density, helper: sha(fileURLToPath(import.meta.url)), capture: sha(capture)}));
    return path.join(viewer, 'target/course-presentation', `browser-${id}-${key.slice(0, 16)}-dpr${density}`);
};
const capture = path.join(docs, 'capture_journey.cjs');

function bindgenFor(lock: string) {
    const version = read(lock).replace(/\r\n/g, '\n').match(/\[\[package\]\]\nname = "wasm-bindgen"\nversion = "([^"]+)"/)?.[1];
    assert(version, 'The checkpoint lock must pin wasm-bindgen');
    const cached = process.platform === 'win32' && process.env.LOCALAPPDATA
        ? path.join(process.env.LOCALAPPDATA, 'trunkrs/trunk/cache', 'wasm-bindgen-' + version, 'wasm-bindgen.exe') : '';
    const executable = cached && fs.existsSync(cached) ? cached : 'wasm-bindgen';
    const result = spawnSync(executable, ['--version'], {encoding: 'utf8'});
    assert.equal(result.status, 0, 'Cannot execute the checkpoint wasm-bindgen CLI');
    assert.equal(result.stdout.trim(), 'wasm-bindgen ' + version, 'The wasm-bindgen CLI must match the checkpoint lock');
    return executable;
}

const observe = `await page.context().addInitScript(() => {
    window.__presentationDevices = []; window.__presentationAdapters = []; window.__presentationGpuErrors = [];
    if (!globalThis.GPUAdapter) return;
    const original = GPUAdapter.prototype.requestDevice;
    GPUAdapter.prototype.requestDevice = async function(...args) {
        const device = await original.apply(this, args), info = this.info;
        window.__presentationAdapters.push({vendor: info?.vendor, description: info?.description, fallback: info?.isFallbackAdapter});
        window.__presentationDevices.push(device);
        device.addEventListener('uncapturederror', event => window.__presentationGpuErrors.push(event.error.message));
        device.lost.then(info => window.__presentationGpuErrors.push('device lost: ' + info.reason));
        return device;
    };
});`;

function highDensity(density: number, early: boolean) {
    return `
            const behaviorFile = path.join(output, id + '-behavior-dpr1.png');
            await page.screenshot({path: behaviorFile, fullPage: true});
            const behaviorViewport = page.viewportSize();
            const densitySession = await page.context().newCDPSession(page);
            await densitySession.send('Emulation.setDeviceMetricsOverride', {width: behaviorViewport.width, height: behaviorViewport.height, deviceScaleFactor: ${density}, mobile: false});
            ${early ? `await page.reload();
            await page.waitForFunction(() => {
                const status = document.querySelector('#status')?.textContent || '';
                return !status.startsWith('Waiting');
            });
            for (const action of step.browser_actions || []) {
                if (action.command) await command(page, action.command);
                else if (action.file) {
                    const chooser = page.waitForEvent('filechooser'); await command(page, 'Open');
                    await (await chooser).setFiles('target/course-' + id + '/' + action.file);
                    await page.waitForFunction(() => document.querySelector('#status')?.textContent === 'File imported. Undo removes the entire import.');
                    if (action.fit) await command(page, 'Fit');
                } else if (action.viewport) await page.setViewportSize({width: action.viewport[0], height: action.viewport[1]});
                else if (action.wheel !== undefined) { await page.locator('canvas').hover(); await page.mouse.wheel(0, action.wheel); }
                else if (action.key) { const b = await page.locator('canvas').boundingBox(); await page.mouse.click(b.x + 8, b.y + 8); await page.keyboard.press(action.key); }
                else if (action.drag) {
                    const b = await page.locator('canvas').boundingBox(), [from, to] = action.drag;
                    await page.mouse.move(b.x + from[0] * b.width, b.y + from[1] * b.height); await page.mouse.down({button: 'right'});
                    await page.mouse.move(b.x + to[0] * b.width, b.y + to[1] * b.height, {steps: 5}); await page.mouse.up({button: 'right'});
                } else if (action.canvas) {
                    const b = await page.locator('canvas').boundingBox();
                    await page.locator('canvas').click({position: {x: action.canvas[0] * b.width, y: action.canvas[1] * b.height}});
                }
                await page.waitForTimeout(200);
            }` : `await page.evaluate(() => window.dispatchEvent(new Event('resize')));`}
            await page.waitForFunction(density => {
                const canvas = document.querySelector('canvas'), rect = canvas.getBoundingClientRect();
                return window.devicePixelRatio === density && canvas.width === Math.round(rect.width * density) && canvas.height === Math.round(rect.height * density);
            }, ${density}, {timeout: 30000});
            await page.evaluate(async () => {
                await Promise.all(window.__presentationDevices.map(device => device.queue.onSubmittedWorkDone()));
                await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
            });
            const densityState = await page.evaluate(() => ({dpr: window.devicePixelRatio,
                canvas: {width: document.querySelector('canvas').width, height: document.querySelector('canvas').height},
                adapters: window.__presentationAdapters, gpuErrors: window.__presentationGpuErrors}));
            assert.deepEqual(densityState.gpuErrors, [], 'No WebGPU errors during density transition');
            if (id !== '01-canvas') {
                assert(densityState.adapters.length > 0, 'An actual browser WebGPU device must be observed');
                for (const adapter of densityState.adapters) { assert.match(adapter.vendor, /nvidia/i); assert.equal(adapter.fallback, false); }
            }
            assert.deepEqual(errors, []);
            ${early ? "assert(!/^Cannot|^Error/.test(await page.locator('#status').textContent()), 'No startup/action error during high-DPI presentation');"
                : "assert.equal(await page.locator('#status').textContent(), step.browser_result_status || step.browser_status);"}
            const densityRecord = {status: 'BEHAVIOR_DPR1_PASSED_IMAGE_DPR${density}_PENDING_ROOT_REVIEW', checkpoint: id,
                behaviorDpr: 1, imageDpr: ${density}, viewport: page.viewportSize(), densityState,
                presentationStatus: await page.locator('#status').textContent(),
                recipe: '${early ? 'Fresh high-DPI startup and documented browser_actions only; original full callback verified at DPR1.' : 'Same post-callback scene; high-DPI resize only.'}',
                behaviorImage: {path: behaviorFile, sha256: hash(fs.readFileSync(behaviorFile))}};
            fs.writeFileSync(path.join(output, id + '-presentation.json'), JSON.stringify(densityRecord, null, 2));
`;
}

export function prepareBrowser(id: string, density = 2) {
    assert([2, 3].includes(density));
    assert(course().steps.some(step => step.id === id), 'Unknown checkpoint');
    const folder = rootFor(id, density), workspace = path.join(folder, 'workspace');
    assert(!fs.existsSync(folder), 'Preserve the existing browser attempt');
    const native = captureDirectory(id, density).directory;
    if (!fs.existsSync(native)) {
        if (id === '01-canvas') materialize(expected(id), native, lockFor(id), assetsFor(id));
        else prepare(id, density);
    }
    const source = bindings(id), original = read(capture);
    const pageBoundary = '        page = await browser.newPage({viewport: {width: 900, height: 760}, deviceScaleFactor: 1});';
    const imageBoundary = '            await page.screenshot({path: path.join(output, file), fullPage: true});';
    assert.equal(original.split(pageBoundary).length, 2);
    assert.equal(original.split(imageBoundary).length, 2);
    const early = course().steps.findIndex(step => step.id === id) < course().steps.findIndex(step => step.id === '20-resize');
    const insert = highDensity(density, early);
    const presentationImage = `            const physicalCapture = await densitySession.send('Page.captureScreenshot', {format: 'png', fromSurface: true, captureBeyondViewport: false});
            fs.writeFileSync(path.join(output, file), Buffer.from(physicalCapture.data, 'base64'));`;
    const adapted = original.replace(pageBoundary, pageBoundary + '\n        ' + observe)
        .replace(imageBoundary, insert + presentationImage);
    assert.equal(adapted.replace(pageBoundary + '\n        ' + observe, pageBoundary).replace(insert + presentationImage, imageBoundary), original);
    const worker = path.join(folder, 'worker.cjs');
    const bootstrap = `const fs=require('node:fs'),path=require('node:path'),Module=require('node:module');
const source=fs.readFileSync(${JSON.stringify(path.join(folder, 'capture-adapted.cjs'))},'utf8');
const filename=${JSON.stringify(capture)}, runner=new Module(filename,module); runner.filename=filename;
runner.paths=Module._nodeModulePaths(path.dirname(filename)); process.argv=[process.argv[0],filename,${JSON.stringify(id)}];
runner._compile(source,filename);\n`;
    write(path.join(folder, 'capture-adapted.cjs'), adapted); write(worker, bootstrap);
    write(path.join(workspace, 'docs/journey/course.json'), read(path.join(docs, 'journey/course.json')));
    fs.cpSync(path.join(docs, 'journey/checks'), path.join(workspace, 'docs/journey/checks'), {recursive: true});
    fs.mkdirSync(path.join(workspace, 'docs/screenshots/journey'), {recursive: true});
    const prior = ['03-triangle-browser.png', '03a-panel-browser.png', '03b-memory-browser.png'];
    for (const file of prior) if (fs.existsSync(path.join(docs, 'screenshots/journey', file)))
        fs.copyFileSync(path.join(docs, 'screenshots/journey', file), path.join(workspace, 'docs/screenshots/journey', file));
    write(path.join(workspace, 'target/course-checks/results.json'), JSON.stringify({[id]: {source: source.files}}));
    const pins = Object.fromEntries([capture, fileURLToPath(import.meta.url), ...filesIn(path.join(docs, 'journey/checks'))].map(file => [file, sha(file)]));
    const record = {status: 'PREPARED_NOT_BUILT', id, density, folder, workspace, native, source, pins,
        worker, workerSha256: sha(worker), adapted: path.join(folder, 'capture-adapted.cjs'), adaptedSha256: hash(adapted),
        assertionPreservation: 'Two additive insertions only: GPU observer before navigation and high-DPI presentation after every original callback/status assertion.',
        behaviorDpr: 1, imageDpr: density, buildProfile: 'debug; no optimized/release claim'};
    write(path.join(folder, 'prepared.json'), JSON.stringify(record, null, 2) + '\n');
    return record;
}

function check(prepared: any) {
    assert.deepEqual(bindings(prepared.id), prepared.source, 'Source or dependency changed');
    for (const [file, digest] of Object.entries(prepared.pins)) assert.equal(sha(file), digest, file);
    assert.equal(sha(prepared.worker), prepared.workerSha256);
    assert.equal(sha(prepared.adapted), prepared.adaptedSha256);
    const kernel = path.relative(prepared.native, path.resolve(viewer, '../session_rust')).split(path.sep).join('/');
    for (const [name, text] of Object.entries(expected(prepared.id))) {
        assert.equal(read(path.join(prepared.native, name)), name === 'Cargo.toml' ? text.replaceAll('../../../session_rust', kernel) : text, name);
    }
    for (const name of assetsFor(prepared.id)) assert.equal(sha(path.join(prepared.native, name)), sha(path.join(viewer, name)), name);
    if (prepared.bundle) for (const [name, digest] of Object.entries(prepared.bundle))
        assert.equal(sha(path.join(prepared.workspace, 'target/course-checks', prepared.id, 'dist', name)), digest, name);
}

export function buildBrowser(id: string, density = 2) {
    const folder = rootFor(id, density);
    if (!fs.existsSync(path.join(folder, 'prepared.json'))) prepareBrowser(id, density);
    const prepared = json(path.join(folder, 'prepared.json'));
    check(prepared);
    if (['BUILT_NOT_CAPTURED', 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW'].includes(prepared.status)) return prepared;
    assert.equal(prepared.status, 'PREPARED_NOT_BUILT');
    const commands = [], label = `presentation-${id}-${path.basename(folder).match(/-([a-f0-9]{16})-dpr/)![1]}`;
    const execute = (name: string, args: string[], environment: Record<string, string> = {}) => {
        assert(!fs.existsSync(path.join(viewer, 'target/course-checks', name + '.log')), 'Preserve the previous build attempt');
        return runCommand(name, args, prepared.native, environment);
    };
    if (fs.existsSync(path.join(prepared.native, 'examples/sample.rs'))) {
        commands.push(execute(`${label}-sample`, ['cargo', 'run', '--example', 'sample', '--locked', '--target', nativeTarget(), '-j1'],
            {COURSE_MEMORY_MIB: '4096', COURSE_TIMEOUT_SECONDS: '1800'}));
        assert(fs.statSync(path.join(prepared.native, 'sample.pb')).size > 0);
    }
    commands.push(execute(`${label}-browser-wasm`, ['cargo', 'build', '--lib', '--locked', '--target', 'wasm32-unknown-unknown', '-j1'],
        {COURSE_MEMORY_MIB: '4096', COURSE_TIMEOUT_SECONDS: '1800'}));
    const manifest = read(path.join(prepared.native, 'Cargo.toml'));
    const crate = manifest.match(/^name\s*=\s*"([^"]+)"/m)![1].replaceAll('-', '_');
    const wasm = path.join(viewer, 'target/course-build/wasm32-unknown-unknown/debug', crate + '.wasm');
    const dist = path.join(prepared.workspace, 'target/course-checks', id, 'dist');
    commands.push(execute(`${label}-browser-bindgen`, [bindgenFor(path.join(prepared.native, 'Cargo.lock')), wasm, '--target', 'web', '--out-dir', dist, '--out-name', 'viewer_journey']));
    const start = `<script type="module">import init,* as bindings from './viewer_journey.js'; const wasm=await init({module_or_path:'./viewer_journey_bg.wasm'}); window.wasmBindings=bindings; dispatchEvent(new CustomEvent('TrunkApplicationStarted',{detail:{wasm}}));</script>`;
    let index = read(path.join(prepared.native, 'index.html')).replace(/<link\b[^>]*\bdata-trunk\b[^>]*\brel="rust"[^>]*>/, start);
    assert(index.includes(start), 'Original Trunk startup link not found');
    index = index.replace(/<link\b[^>]*\bdata-trunk\b[^>]*>/g, '');
    write(path.join(dist, 'index.html'), index);
    for (const asset of assetsFor(id)) {
        const destination = asset.startsWith('assets/text/') ? asset.replace('assets/text/', 'text/') : asset;
        const file = path.join(dist, destination); fs.mkdirSync(path.dirname(file), {recursive: true});
        fs.copyFileSync(path.join(prepared.native, asset), file);
    }
    fs.cpSync(prepared.native, path.join(prepared.workspace, 'target', `course-${id}`), {recursive: true, filter: file => !file.includes(`${path.sep}dist${path.sep}`)});
    check(prepared);
    const result = {...prepared, status: 'BUILT_NOT_CAPTURED', commands, wasm: {path: wasm, sha256: sha(wasm)},
        bundle: Object.fromEntries(filesIn(dist).map(file => [path.relative(dist, file).split(path.sep).join('/'), sha(file)]))};
    write(path.join(folder, 'prepared.json'), JSON.stringify(result, null, 2) + '\n');
    return result;
}

export async function runBrowser(id: string, density = 2) {
    const folder = rootFor(id, density), prepared = json(path.join(folder, 'prepared.json'));
    check(prepared);
    if (prepared.status === 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW') {
        assert.equal(sha(prepared.image.path), prepared.image.sha256); return prepared;
    }
    assert.equal(prepared.status, 'BUILT_NOT_CAPTURED');
    const adapter = json(path.join(viewer, 'target/course-windows-20261018/nvidia-hardware-browser-preparation/adapter-metadata.json'));
    const chrome = process.env.VIEWER_BROWSER_EXECUTABLE || path.join(viewer, 'target/course-browsers/chromium_headless_shell-1208/chrome-headless-shell-win64/chrome-headless-shell.exe');
    assert(fs.existsSync(chrome), 'Pinned browser executable missing');
    const base = path.join(prepared.workspace, 'target/course-checks');
    const mime: Record<string, string> = {'.html': 'text/html', '.js': 'text/javascript', '.wasm': 'application/wasm', '.json': 'application/json', '.ttf': 'font/ttf'};
    const served: Record<string, string> = {};
    const server = http.createServer((request, response) => {
        try {
            const file = path.resolve(base, '.' + decodeURIComponent(new URL(request.url!, 'http://localhost').pathname));
            assert(file.startsWith(base + path.sep));
            const actual = fs.statSync(file).isDirectory() ? path.join(file, 'index.html') : file;
            response.writeHead(200, {'Content-Type': mime[path.extname(actual)] || 'application/octet-stream', 'Cache-Control': 'no-store'});
            const bytes = fs.readFileSync(actual);
            served[path.relative(base, actual).split(path.sep).join('/')] = hash(bytes);
            response.end(bytes);
        } catch { response.writeHead(404).end('Not found'); }
    });
    await new Promise<void>(resolve => server.listen(0, '127.0.0.1', resolve));
    const address = server.address() as {port: number};
    const log = path.join(folder, 'browser.log'), descriptor = fs.openSync(log, 'wx');
    try {
        const args = [path.resolve(viewer, '../session_tests/scripts/run-guarded.cjs'), '-MemoryLimitMiB', '6144', '-TimeoutSeconds', '1800',
            '-WorkingDirectory', prepared.workspace, '-ManifestPath', path.join(folder, 'resources.json'), process.execPath,
            '--require', path.resolve(viewer, '../session_tests/scripts/course/warp-browser-timeout.cjs'), prepared.worker];
        const code = await new Promise((resolve, reject) => {
            const child = spawn(process.execPath, args, {cwd: prepared.workspace, windowsHide: true, stdio: ['ignore', descriptor, descriptor], env: {...process.env,
                NODE_PATH: path.join(viewer, 'target/course-tools/node_modules'), COURSE_SAMPLES_PREPARED: '1', VIEWER_HEADLESS: '1',
                VIEWER_WARP_ACTION_TIMEOUT_MS: '120000', VIEWER_BROWSER_EXECUTABLE: chrome, JOURNEY_URL: `http://127.0.0.1:${address.port}/`,
                VIEWER_CHROME_ARGS: JSON.stringify(['--enable-unsafe-webgpu', '--ignore-gpu-blocklist', '--use-gl=angle', '--use-angle=d3d11',
                    '--use-adapter-luid=' + adapter.selected.luid, '--enable-dawn-features=d3d_skip_shader_optimizations'])}});
            child.once('error', reject); child.once('exit', resolve);
        });
        assert.equal(code, 0, `Browser presentation failed; inspect ${log}`);
        check(prepared);
        const image = path.join(prepared.workspace, 'docs/screenshots/journey', `${id}-browser.png`), bytes = fs.readFileSync(image);
        const presentation = json(path.join(prepared.workspace, 'docs/screenshots/journey', `${id}-presentation.json`));
        assert.equal(bytes.readUInt32BE(16), presentation.viewport.width * density);
        assert.equal(bytes.readUInt32BE(20), presentation.viewport.height * density);
        const resources = json(path.join(folder, 'resources.json'));
        assert.equal(resources.exitCode, 0); assert.equal(resources.processors, 2); assert.equal(resources.jobCommitLimitBytes, 6442450944);
        assert.equal(served[`${id}/dist/index.html`], prepared.bundle['index.html']);
        assert.equal(served[`${id}/dist/viewer_journey_bg.wasm`], prepared.bundle['viewer_journey_bg.wasm']);
        const result = {...prepared, status: 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW', presentation, served, resources,
            image: {path: image, sha256: hash(bytes), width: bytes.readUInt32BE(16), height: bytes.readUInt32BE(20)}};
        write(path.join(folder, 'prepared.json'), JSON.stringify(result, null, 2) + '\n');
        return result;
    } finally { fs.closeSync(descriptor); await new Promise<void>(resolve => server.close(() => resolve())); }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
    const [mode, id, scale] = process.argv.slice(2), density = Number(scale || 2);
    const result = mode === 'prepare' ? prepareBrowser(id, density) : mode === 'build' ? buildBrowser(id, density)
        : mode === 'run' ? await runBrowser(id, density) : null;
    assert(result, 'Usage: browser-presentation.ts prepare|build|run ID [2|3]');
    console.log(JSON.stringify(result, null, 2));
}
