import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {viewer, filesIn, hash, json} from './model.ts';

export const cargoTarget = () => path.resolve(process.env.CARGO_TARGET_DIR || path.join(viewer, 'target/course-cargo'));
export const adapterMetadataPath = () => path.resolve(process.env.COURSE_ADAPTER_METADATA || path.join(viewer, 'target/course-adapter/adapter-metadata.json'));

export function adapterMetadata() {
    const file = adapterMetadataPath(), record = json(file);
    assert.equal(record.selected.vendor, 0x10de, 'Capture requires an actual NVIDIA adapter enumeration receipt');
    assert.equal(record.selected.device, 0x27e0);
    assert(record.selected.luid, 'Adapter receipt must contain the enumerated LUID');
    return record;
}

export function fileReference(file: string) {
    file = path.resolve(file);
    return {path: file, sha256: hash(fs.readFileSync(file)), bytes: fs.statSync(file).size};
}

export function observeExecution(files: string[]) {
    return {at: new Date().toISOString(), files: [...new Set(files.map(file => path.resolve(file)))].sort().map(fileReference)};
}

export function finishExecution(before: ReturnType<typeof observeExecution>) {
    const after = observeExecution(before.files.map(file => file.path));
    assert.deepEqual(after.files, before.files, 'Execution inputs changed during the actual process');
    return {before, after};
}

export function bundleInputs(workspace: string, id: string) {
    return filesIn(path.join(workspace, 'target/course-checks', id, 'dist'));
}

export function compilerTools() {
    return Object.fromEntries(['rustc', 'cargo'].map(name => {
        const result = spawnSync('rustup', ['which', name], {encoding: 'utf8', windowsHide: true});
        assert.equal(result.status, 0, 'Resolve the actual installed compiler executable');
        return [name, fileReference(result.stdout.trim())];
    }));
}

export function workbenchPath(root: string, name: string) {
    root = path.resolve(root);
    assert.equal(fs.realpathSync(root), root, 'Workbench root or ancestor is redirected');
    const file = path.resolve(root, name), relative = path.relative(root, file);
    assert(relative && !relative.startsWith('..') && !path.isAbsolute(relative), 'Workbench path escapes ownership');
    let current = root;
    for (const part of relative.split(path.sep)) {
        current = path.join(current, part);
        if (fs.existsSync(current)) assert(!fs.lstatSync(current).isSymbolicLink(), 'Workbench contains a link');
    }
    return file;
}

export function checkWorkbenchOwnership(root: string, previous: Record<string, string>) {
    fs.mkdirSync(root, {recursive: true});
    assert.equal(fs.realpathSync(root), path.resolve(root), 'Workbench root or ancestor is redirected');
    const names = filesIn(root).map(file => path.relative(root, file).split(path.sep).join('/')).sort();
    assert.deepEqual(names, Object.keys(previous).sort(), 'Workbench contains unowned or missing files');
    for (const name of names) assert.equal(fileReference(workbenchPath(root, name)).sha256, previous[name], 'Previously owned input changed: ' + name);
}
