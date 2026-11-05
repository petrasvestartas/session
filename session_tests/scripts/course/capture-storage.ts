import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {viewer, docs, filesIn, hash, json, write} from './model.ts';

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

export type ImageCourseManifest = {path: string; sha256: string; bytes: number};
const courseReference = (workspace: string) => path.join(workspace, 'docs/journey/course-manifest-input.json');
export function verifyImageCourseManifest(input: ImageCourseManifest) {
    assert(path.isAbsolute(input.path) && /^[a-f0-9]{64}$/.test(input.sha256));
    assert.equal(path.basename(path.dirname(input.path)), input.sha256);
    assert.equal(path.basename(input.path), 'course.json');
    assert.equal(fs.realpathSync(input.path), input.path);
    const stat = fs.lstatSync(input.path); assert(stat.isFile() && !stat.isSymbolicLink() && stat.nlink === 1);
    const bytes = fs.readFileSync(input.path); assert.equal(bytes.length, input.bytes); assert.equal(hash(bytes), input.sha256);
    return bytes;
}
export function imageCourseSnapshot(shared = process.env.COURSE_IMAGE_SHARED_MANIFEST === '1', source = path.join(docs, 'journey/course.json'), directory = path.join(viewer, 'target/shared-inputs')): ImageCourseManifest | null {
    if (!shared) return null;
    const bytes = fs.readFileSync(source), digest = hash(bytes), root = path.resolve(directory), folder = path.join(root, digest), file = path.join(folder, 'course.json');
    fs.mkdirSync(root, {recursive: true}); assert.equal(fs.realpathSync(root), root);
    fs.mkdirSync(folder, {recursive: true}); assert.equal(fs.realpathSync(folder), folder);
    try { fs.writeFileSync(file, bytes, {flag: 'wx'}); } catch (error) { if ((error as NodeJS.ErrnoException).code !== 'EEXIST') throw error; }
    const input = {path: file, sha256: digest, bytes: bytes.length}; assert(verifyImageCourseManifest(input).equals(bytes)); return input;
}
export function stageImageCourseManifest(workspace: string, shared = process.env.COURSE_IMAGE_SHARED_MANIFEST === '1', source = path.join(docs, 'journey/course.json'), directory = path.join(viewer, 'target/shared-inputs')) {
    const input = imageCourseSnapshot(shared, source, directory), local = path.join(workspace, 'docs/journey/course.json');
    if (!input) { write(local, fs.readFileSync(source).toString('utf8')); return null; }
    assert(!fs.existsSync(local), 'Shared manifest must not shadow a per-case copy');
    const file = courseReference(workspace), bytes = JSON.stringify(input, null, 2) + '\n';
    if (fs.existsSync(file)) assert.equal(fs.readFileSync(file, 'utf8'), bytes); else write(file, bytes);
    return input;
}
export function imageCourseManifest(workspace: string): ImageCourseManifest | null {
    const file = courseReference(workspace); if (!fs.existsSync(file)) return null;
    const input = json(file) as ImageCourseManifest; verifyImageCourseManifest(input); return input;
}
export function imageCourseManifestPath(workspace: string) { return imageCourseManifest(workspace)?.path || path.join(workspace, 'docs/journey/course.json'); }
export function imageCourseManifestInputs(workspace: string) { const input = imageCourseManifest(workspace); return input ? [courseReference(workspace), input.path] : []; }
export function adaptImageCourseRead(source: string, workspace: string) {
    const input = imageCourseManifest(workspace); if (!input) return source;
    const matches = source.match(/fs\.readFileSync\('docs\/journey\/course\.json'(?:, 'utf8')?\)/g); assert.equal(matches?.length, 1, 'One exact canonical course read is required');
    const before = matches![0], text = before.includes("'utf8'");
    const after = '((bytes)=>{assert.equal(crypto.createHash("sha256").update(bytes).digest("hex"),' + JSON.stringify(input.sha256) + ');return bytes' + (text ? '.toString("utf8")' : '') + ';})(fs.readFileSync(' + JSON.stringify(input.path) + '))';
    const adapted = source.replace(before, () => after); assert.equal(adapted.replace(after, () => before), source); return adapted;
}
