import fs from 'node:fs';
import path from 'node:path';
import Module from 'node:module';
import vm from 'node:vm';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {buildBrowser} from './browser-presentation.ts';
import {bindings} from './presentation.ts';
import {docs, hash, json, read, viewer, write} from './model.ts';
import {observeGpu, privateBrowserHost, validateGpu} from './private-browser-host.ts';

const id = '97-finish-with-the-maintained-browser-runtime';
const sha = (file: string) => hash(fs.readFileSync(file));
const script = fileURLToPath(import.meta.url), originalFile = path.join(docs, 'capture_current.cjs');

export function maintainedClone() {
    const original = read(originalFile), start = '            const page = await context.newPage();';
    const image = '                const image = PNG.sync.read(bytes);';
    assert.equal(original.split(start).length, 2); assert.equal(original.split(image).length, 2);
    const extra = validateGpu + `\nfs.writeFileSync(path.join(output,'actual-gpu-dpr'+density+'.json'),JSON.stringify(actualGpu,null,2));`;
    const adapted = original.replace(start, start + '\n' + observeGpu).replace(image, image + '\n' + extra);
    assert.equal(adapted.replace(start + '\n' + observeGpu, start).replace(image + '\n' + extra, image), original);
    new vm.Script(adapted);
    return {original, adapted, originalSha256: hash(original), adaptedSha256: hash(adapted)};
}

export async function runMaintainedPresentation() {
    const built = buildBrowser(id, 2), clone = maintainedClone(), source = bindings(id);
    const key = hash(JSON.stringify({helper: sha(script), host: sha(fileURLToPath(new URL('private-browser-host.ts', import.meta.url))), source,
        capture: clone.originalSha256, adapted: clone.adaptedSha256}));
    const folder = path.join(built.folder, 'maintained-' + key.slice(0, 16)), recordFile = path.join(folder, 'result.json');
    if (fs.existsSync(recordFile)) {
        const record = json(recordFile); assert.deepEqual(bindings(id), record.source);
        assert.equal(sha(record.image.path), record.image.sha256); return record;
    }
    assert(!fs.existsSync(folder), 'Preserve the previous maintained attempt');
    const workspace = path.join(folder, 'workspace'); fs.cpSync(built.workspace, workspace, {recursive: true});
    const transportCopies = [];
    for (const link of read(path.join(built.native, 'index.html')).matchAll(/<link\b[^>]*data-trunk[^>]*rel="copy-file"[^>]*href="([^"]+)"[^>]*>/g)) {
        const sourceAsset = path.join(built.native, link[1]), destination = path.join(workspace, 'target/course-checks', id, 'dist', path.basename(link[1]));
        fs.copyFileSync(sourceAsset, destination);
        transportCopies.push({source: link[1], destination: path.basename(link[1]), sha256: sha(destination), scope: 'Exact original Trunk copy-file transport, no source change.'});
    }
    write(path.join(workspace, 'target/course-checks/results.json'), JSON.stringify({[id]: {source: source.files, commands: built.commands}}, null, 2));
    write(path.join(folder, 'capture-adapted.cjs'), clone.adapted);
    const host = await privateBrowserHost(workspace, folder, script, {id, originalFile, adapted: path.join(folder, 'capture-adapted.cjs')});
    assert.deepEqual(bindings(id), source);
    const outputs = path.join(workspace, 'docs/screenshots/journey');
    const browser = json(path.join(outputs, 'browser.json')).steps[id];
    assert.deepEqual(browser.images.map((image: any) => image.density), [1, 2]);
    const image = path.join(outputs, `${id}-browser-dpr2.png`), bytes = fs.readFileSync(image);
    assert.equal(bytes.readUInt32BE(16), 1800); assert.equal(bytes.readUInt32BE(20), 1520);
    for (const [name, expected] of Object.entries(built.bundle))
        assert.equal(sha(path.join(workspace, 'target/course-checks', id, 'dist', name)), expected);
    assert.equal(host.served[`${id}/dist/viewer_journey_bg.wasm`], built.bundle['viewer_journey_bg.wasm']);
    const record = {status: 'CAPTURED_AWAITING_ROOT_VISUAL_REVIEW', id, folder, source, originalCapture: {path: originalFile, sha256: clone.originalSha256},
        clone: {sha256: clone.adaptedSha256, assertionPreservation: 'Only additive actual-GPU observer/validation; all original DPR1/DPR2 behavior and pixel assertions retained.'},
        browser, host, transportCopies, sourceRecord: recordFile,
        image: {path: image, sha256: hash(bytes), width: 1800, height: 1520},
        destinations: [`docs/screenshots/journey/${id}-browser.png`, `docs/screenshots/journey/${id}-browser-dpr2.png`],
        scope: 'Both public filenames may use this one actual new-source DPR2 image after root review. Original DPR1 image and both-density behavior evidence remain private and preserved.'};
    write(recordFile, JSON.stringify(record, null, 2) + '\n'); return record;
}

if (process.argv[2] === '--worker') {
    const config = json(process.argv[3]), source = read(config.adapted), runner = new Module(config.originalFile);
    runner.filename = config.originalFile; runner.paths = Module._nodeModulePaths(path.dirname(config.originalFile));
    process.argv = [process.argv[0], config.originalFile, config.id]; runner._compile(source, config.originalFile);
} else if (process.argv[1] && path.resolve(process.argv[1]) === script) {
    if (process.argv[2] === 'check') console.log(JSON.stringify({...maintainedClone(), original: undefined, adapted: undefined, status: 'CLONE_SYNTAX_AND_ASSERTION_PRESERVATION_PASS_NO_EXECUTION'}));
    else { assert.equal(process.argv[2], 'run'); console.log(JSON.stringify(await runMaintainedPresentation(), null, 2)); }
}
