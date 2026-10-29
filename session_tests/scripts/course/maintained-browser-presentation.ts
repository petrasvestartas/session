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
import {fileReference, observeExecution, finishExecution} from './capture-storage.ts';

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

export async function runMaintainedPresentation(preparedFile?: string) {
    const built = preparedFile ? json(preparedFile) : buildBrowser(id, 2), clone = maintainedClone(), source = bindings(id);
    assert.equal(built.id, id); assert.deepEqual(built.source, source);
    if (preparedFile) {
        assert.equal(built.status, 'BUILT_NOT_CAPTURED');
        assert.equal(sha(built.imageBuild.path), built.imageBuild.sha256);
        const compiler = json(built.imageBuild.path);
        assert.deepEqual(compiler.source, source); assert.deepEqual(compiler.bundle, built.bundle);
    }
    const key = hash(JSON.stringify({helper: sha(script), host: sha(fileURLToPath(new URL('private-browser-host.ts', import.meta.url))), source,
        capture: clone.originalSha256, adapted: clone.adaptedSha256, prepared: preparedFile ? fileReference(preparedFile) : null}));
    const folder = path.join(built.folder, 'maintained-' + key.slice(0, 16)), recordFile = path.join(folder, 'result.json');
    if (fs.existsSync(recordFile)) {
        const record = json(recordFile); assert.deepEqual(bindings(id), record.source);
        assert.equal(sha(record.image.path), record.image.sha256); return record;
    }
    assert(!fs.existsSync(folder), 'Preserve the previous maintained attempt');
    const workspace = preparedFile ? built.workspace : path.join(folder, 'workspace');
    if (!preparedFile) fs.cpSync(built.workspace, workspace, {recursive: true});
    const transportCopies = [];
    for (const link of read(path.join(built.native, 'index.html')).matchAll(/<link\b[^>]*data-trunk[^>]*rel="copy-file"[^>]*href="([^"]+)"[^>]*>/g)) {
        const sourceAsset = path.join(built.native, link[1]), destination = path.join(workspace, 'target/course-checks', id, 'dist', path.basename(link[1]));
        if (preparedFile) assert.equal(sha(destination), sha(sourceAsset), 'Copy-file transport must already belong to the immutable compiler bundle');
        else fs.copyFileSync(sourceAsset, destination);
        transportCopies.push({source: link[1], destination: path.basename(link[1]), sha256: sha(destination), scope: 'Exact original Trunk copy-file transport, no source change.'});
    }
    const commands = built.commands.map((command: any) => {
        if (!preparedFile) return command;
        assert.equal(sha(command.resourcesFile), command.resourcesSha256);
        assert.deepEqual(json(command.resourcesFile), command.resources);
        assert.equal(command.resources.exitCode, 0); assert.equal(command.resources.timedOut, false);
        return {...command, returncode: command.resources.exitCode};
    });
    const results = path.join(workspace, 'target/course-checks/results.json');
    write(results, JSON.stringify({[id]: {source: source.files, commands}}, null, 2));
    const adapted = path.join(folder, 'capture-adapted.cjs'); write(adapted, clone.adapted);
    const before = observeExecution([originalFile, adapted, results, path.join(workspace, 'docs/journey/course.json'),
        path.join(workspace, 'docs/journey/checks/browser/final.cjs'), path.join(workspace, 'docs/journey/checks/fingerprint.cjs')]);
    const host = await privateBrowserHost(workspace, folder, script, {id, originalFile, adapted, adaptedSha256: clone.adaptedSha256});
    const driver = finishExecution(before);
    for (const phase of ['before', 'after'] as const) host.executionIntegrity[phase].files.push(...driver[phase].files);
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
        ...(preparedFile ? {bundle: built.bundle, imageBuild: built.imageBuild, prepared: fileReference(preparedFile)} : {}),
        image: {path: image, sha256: hash(bytes), width: 1800, height: 1520},
        destinations: [`docs/screenshots/journey/${id}-browser.png`, `docs/screenshots/journey/${id}-browser-dpr2.png`],
        scope: 'Both public filenames may use this one actual new-source DPR2 image after root review. Original DPR1 image and both-density behavior evidence remain private and preserved.'};
    write(recordFile, JSON.stringify(record, null, 2) + '\n'); return record;
}

if (process.argv[2] === '--worker') {
    const config = json(process.argv[3]), source = read(config.adapted), runner = new Module(config.originalFile);
    if (config.adaptedSha256) assert.equal(hash(source), config.adaptedSha256);
    runner.filename = config.originalFile; runner.paths = Module._nodeModulePaths(path.dirname(config.originalFile));
    process.argv = [process.argv[0], config.originalFile, config.id]; runner._compile(source, config.originalFile);
} else if (process.argv[1] && path.resolve(process.argv[1]) === script) {
    if (process.argv[2] === 'check') console.log(JSON.stringify({...maintainedClone(), original: undefined, adapted: undefined, status: 'CLONE_SYNTAX_AND_ASSERTION_PRESERVATION_PASS_NO_EXECUTION'}));
    else { assert.equal(process.argv[2], 'run'); console.log(JSON.stringify(await runMaintainedPresentation(), null, 2)); }
}
