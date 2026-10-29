import {cargoTarget} from './capture-storage.ts';
import fs from 'node:fs';
import path from 'node:path';
import Module from 'node:module';
import vm from 'node:vm';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {docs, filesIn, fingerprint, hash, json, materialize, read, reference, referenceFiles, viewer, write} from './model.ts';
import {runCommand} from './verify.ts';
import {observeGpu, privateBrowserHost, validateGpu} from './private-browser-host.ts';
import {exactBindgen} from './exact-bindgen.ts';

const script = fileURLToPath(import.meta.url), capture = path.join(docs, 'capture_viewer.cjs');
const sha = (file: string) => hash(fs.readFileSync(file));

function sourceBinding() {
    const files = referenceFiles('37'), external = reference().external;
    const kernel = path.resolve(viewer, '../session_rust');
    return {files: fingerprint(files), external: Object.fromEntries(external.map((name: string) => [name, sha(path.join(docs, 'lessons/37', name))])),
        boxes: sha(path.join(docs, 'lessons/37/assets/course-boxes.pb')),
        kernel: fingerprint(Object.fromEntries(filesIn(path.join(kernel, 'src')).map(file => [path.relative(kernel, file), sha(file)]))),
        kernelInputs: Object.fromEntries(['Cargo.toml', 'build.rs'].map(name => [name, sha(path.join(kernel, name))])),
        capture: sha(capture), helper: sha(script), selector: sha(fileURLToPath(new URL('exact-bindgen.ts', import.meta.url))),
        host: sha(fileURLToPath(new URL('private-browser-host.ts', import.meta.url)))};
}

export function practiceClone() {
    const original = read(capture), launch = original.match(/chromium\.launch\(\{channel:'chrome', headless:true,[\s\S]*?\}\)/)?.[0];
    assert(launch);
    const replacements = [
        [launch!, "chromium.launch({headless:true,executablePath:process.env.VIEWER_BROWSER_EXECUTABLE,args:JSON.parse(process.env.VIEWER_CHROME_ARGS)})"],
        ['deviceScaleFactor:1', 'deviceScaleFactor:2'],
        ["?data=off&inspect=1&dpr=1", '?data=off&inspect=1&dpr=2'],
    ];
    let adapted = original;
    for (const [before, after] of replacements) { assert.equal(adapted.split(before).length, 2); adapted = adapted.replace(before, after); }
    const page = '        const errors = [];', beforeImage = '            records.frames[name] = await state();';
    assert.equal(adapted.split(page).length, 2); assert.equal(adapted.split(beforeImage).length, 2);
    adapted = adapted.replace(page, observeGpu + '\n' + page).replace(beforeImage, validateGpu + '\n' + beforeImage);
    let restored = adapted.replace(observeGpu + '\n' + page, page).replace(validateGpu + '\n' + beforeImage, beforeImage);
    for (const [before, after] of [...replacements].reverse()) restored = restored.replace(after, before);
    assert.equal(restored, original); new vm.Script(adapted);
    return {adapted, sha256: hash(adapted), assertionPreservation: 'Only launch bootstrap, DPR1→2 configuration/query and additive real-GPU observation. Every original scene, selection, camera, undo and UI assertion is retained.'};
}

export async function runPracticePresentation() {
    const source = sourceBinding(), key = hash(JSON.stringify(source)), clone = practiceClone();
    const folder = path.join(viewer, 'target/course-presentation/reference-practice-' + key.slice(0, 16)), resultFile = path.join(folder, 'result.json');
    if (fs.existsSync(resultFile)) {
        const result = json(resultFile); assert.deepEqual(sourceBinding(), result.source);
        for (const image of result.images) assert.equal(sha(image.path), image.sha256); return result;
    }
    assert(!fs.existsSync(folder), 'Preserve the previous practice attempt');
    const native = path.join(viewer, 'target', 'course-practice-presentation-' + key.slice(0, 16));
    materialize(referenceFiles('37'), native, path.join(docs, 'lessons/37/Cargo.lock'));
    for (const name of reference().external) if (!fs.existsSync(path.join(native, name))) write(path.join(native, name), fs.readFileSync(path.join(docs, 'lessons/37', name)));
    write(path.join(native, 'assets/pb/course-boxes.pb'), fs.readFileSync(path.join(docs, 'lessons/37/assets/course-boxes.pb')));
    const workspace = path.join(folder, 'workspace'), dist = path.join(workspace, 'target/course-checks/reference37/dist');
    const commands = [runCommand('practice-' + key.slice(0, 16) + '-wasm', ['cargo', 'build', '--lib', '--locked', '--target', 'wasm32-unknown-unknown', '-j1'], native,
        {COURSE_MEMORY_MIB: '4096', COURSE_TIMEOUT_SECONDS: '1800'})];
    const bindgen = exactBindgen(path.join(native, 'Cargo.lock'));
    commands.push(runCommand('practice-' + key.slice(0, 16) + '-bindgen', [bindgen.executable, path.join(cargoTarget(), 'wasm32-unknown-unknown/debug/session_viewer.wasm'),
        '--target', 'web', '--out-dir', dist, '--out-name', 'session_viewer'], native));
    const start = `<script type="module">import init,* as bindings from './session_viewer.js';const wasm=await init({module_or_path:'./session_viewer_bg.wasm'});window.wasmBindings=bindings;dispatchEvent(new CustomEvent('TrunkApplicationStarted',{detail:{wasm}}));</script>`;
    const originalIndex = read(path.join(native, 'index.html'));
    for (const link of originalIndex.matchAll(/<link\b[^>]*data-trunk[^>]*rel="(copy-file|copy-dir)"[^>]*href="([^"]+)"[^>]*>/g)) {
        if (link[2] === 'target/docs/site') continue;
        const target = link[0].match(/data-target-path="([^"]+)"/)?.[1] || path.basename(link[2]);
        fs.cpSync(path.join(native, link[2]), path.join(dist, target), {recursive: link[1] === 'copy-dir'});
    }
    let index = originalIndex.replace(/<link\b[^>]*data-trunk[^>]*rel="rust"[^>]*>/, start); assert(index.includes(start));
    index = index.replace(/<link\b[^>]*data-trunk[^>]*>/g, ''); write(path.join(dist, 'index.html'), index);
    fs.mkdirSync(path.join(workspace, 'docs/screenshots/practice'), {recursive: true});
    write(path.join(folder, 'capture-adapted.cjs'), clone.adapted);
    const host = await privateBrowserHost(workspace, folder, script, {id: 'reference37', originalFile: capture, adapted: path.join(folder, 'capture-adapted.cjs')});
    assert.deepEqual(sourceBinding(), source);
    const images = ['loaded', 'point', 'layers', 'selected', 'arctic', 'opacity'].map(name => {
        const image = path.join(workspace, 'docs/screenshots/practice', `viewer-${name}.png`), bytes = fs.readFileSync(image);
        assert.equal(bytes.readUInt32BE(16), 2200); assert.equal(bytes.readUInt32BE(20), 1520);
        return {name, path: image, sha256: hash(bytes), width: 2200, height: 1520, destination: `docs/screenshots/practice/viewer-${name}.png`};
    });
    const result = {status: 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW', sourceRecord: resultFile, source, commands, bindgen, clone: {...clone, adapted: undefined}, host, images,
        scope: 'Exact reference37 handwritten source, current pinned dependencies, original supplied box scene and original six-view recipe at genuine DPR2. This is a reference source capture, not a claim that all later production changes occur in reference37.'};
    write(resultFile, JSON.stringify(result, null, 2) + '\n'); return result;
}

if (process.argv[2] === '--worker') {
    const config = json(process.argv[3]), runner = new Module(config.originalFile);
    runner.filename = config.originalFile; runner.paths = Module._nodeModulePaths(path.dirname(config.originalFile));
    runner._compile(read(config.adapted), config.originalFile);
} else if (process.argv[1] && path.resolve(process.argv[1]) === script) {
    if (process.argv[2] === 'check') console.log(JSON.stringify({...practiceClone(), adapted: undefined, status: 'CLONE_SYNTAX_AND_ASSERTION_PRESERVATION_PASS_NO_EXECUTION'}));
    else { assert.equal(process.argv[2], 'run'); console.log(JSON.stringify(await runPracticePresentation(), null, 2)); }
}
