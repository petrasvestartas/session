import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {prepareScene} from './browser-scene-presentation.ts';
import {privateImageBrowserHost} from './parallel-image-browser-host.ts';
import {bindings} from './presentation.ts';
import {hash, json, viewer, write} from './model.ts';

const self = fileURLToPath(import.meta.url), worker = fileURLToPath(new URL('browser-scene-presentation.ts', import.meta.url));
const sha = (file: string) => hash(fs.readFileSync(file));
const reference = (file: string) => ({path: path.resolve(file), sha256: sha(file)});

function check(prepared: any) {
    assert.deepEqual(bindings(prepared.id), prepared.source);
    assert.equal(prepared.helperSha256, sha(worker));
    for (const [file, digest] of Object.entries(prepared.pins)) assert.equal(sha(file), digest, file);
    for (const [name, digest] of Object.entries(prepared.bundle)) assert.equal(sha(path.join(prepared.dist, name)), digest, name);
}

export function prepareImageLaneScene(id: string, builtFile: string, basePreparedFile = '') {
    const built = json(path.resolve(builtFile));
    const base = basePreparedFile ? json(path.resolve(basePreparedFile)) : prepareScene(id, builtFile);
    assert.equal(base.id, id); assert.deepEqual(base.source, built.source);
    const profile = {name: 'parallel-image-only', processors: 8, memoryMiB: 8192, affinityMask: '0xFF00', canonicalBehaviorExecuted: false,
        parent: reference(self), originalWorker: reference(worker),
        guard: reference(fileURLToPath(new URL('parallel-image-guard.ps1', import.meta.url))),
        host: reference(fileURLToPath(new URL('parallel-image-browser-host.ts', import.meta.url)))};
    const key = hash(JSON.stringify({source: base.source, bundle: base.bundle, recipe: base.recipe, profile})).slice(0, 20);
    const directory = path.join(viewer, 'target/course-presentation/image-lane-scenes', id + '-' + key);
    assert(!fs.existsSync(directory), 'Preserve previous image-lane attempt');
    const pins = {...base.pins};
    for (const record of [profile.parent, profile.originalWorker, profile.guard, profile.host]) pins[record.path] = record.sha256;
    const prepared = {...base, directory, workspace: built.workspace, pins, captureProfile: profile, canonicalBehaviorExecuted: false,
        basePreparation: reference(basePreparedFile || path.join(base.directory, 'prepared.json'))};
    check(prepared); write(path.join(directory, 'prepared.json'), JSON.stringify(prepared, null, 2) + '\n'); return prepared;
}

export async function runImageLaneScene(preparedFile: string) {
    const prepared = json(path.resolve(preparedFile)); check(prepared);
    const resultFile = path.join(prepared.directory, 'result.json');
    assert(!fs.existsSync(resultFile), 'Completed image-lane captures must be verified/reused, not rerun');
    const host = await privateImageBrowserHost(prepared.workspace, prepared.directory, worker,
        {...prepared, workerArguments: ['worker', path.resolve(preparedFile), '{VIEWER_URL}']});
    check(prepared);
    const scene = json(path.join(prepared.directory, 'scene.json'));
    assert.equal(scene.status, 'PRESENTATION_ONLY_AWAITING_ROOT_VISUAL_REVIEW'); assert.equal(scene.canonicalBehaviorExecuted, false);
    assert.equal(sha(scene.image.path), scene.image.sha256);
    assert.equal(scene.image.width, 1800); assert.equal(scene.image.height, 1520);
    assert.equal(host.served[`${prepared.id}/dist/index.html`], prepared.bundle['index.html']);
    for (const name of Object.keys(prepared.bundle).filter(name => name.endsWith('.wasm')))
        assert.equal(host.served[`${prepared.id}/dist/${name}`], prepared.bundle[name]);
    const result = {...prepared, ...scene, captureProfile: prepared.captureProfile, resources: host.resources, host,
        served: host.served, elapsedSeconds: host.resources.elapsedSeconds,
        execution: {log: {path: host.log, sha256: host.logSha256}, resources: {path: host.resourcesFile, sha256: host.resourcesSha256}}};
    write(resultFile, JSON.stringify(result, null, 2) + '\n'); return result;
}

if (process.argv[1] && path.resolve(process.argv[1]) === self) {
    const [mode, id, file, base] = process.argv.slice(2);
    const result = mode === 'prepare' ? prepareImageLaneScene(id, file, base) : mode === 'run' ? await runImageLaneScene(id) : null;
    assert(result, 'image-lane-browser-scene.ts prepare ID BUILT_JSON [BASE_SCENE_PREPARED] | run PREPARED_JSON');
    console.log(JSON.stringify(result, null, 2));
}
