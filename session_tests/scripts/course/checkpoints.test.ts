import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {save, restore, savedPath} from './checkpoints.ts';
import {expected, compare, safeOutput, hash, referenceFiles, reference, dependencies, lockFor, read, materialize, viewer} from './model.ts';

function fixture(run: (folder: string) => void) {
    const folder = fs.mkdtempSync(path.join(os.tmpdir(), 'viewer-course-'));
    try { run(folder); }
    finally { fs.rmSync(folder, {recursive: true, force: true}); }
}

test('restore preserves the unfinished project and does not overwrite a save', () => fixture(folder => {
    const project = path.join(folder, 'journey');
    fs.mkdirSync(project);
    fs.writeFileSync(path.join(project, 'main.rs'), 'working');
    fs.mkdirSync(path.join(project, 'target'));
    fs.writeFileSync(path.join(project, 'target/build'), 'compiled output');
    const checkpoint = save('working', project);
    assert(!fs.existsSync(path.join(checkpoint, 'target')));
    assert.throws(() => save('working', project), /Refusing to overwrite/);
    fs.writeFileSync(path.join(project, 'main.rs'), 'experiment');
    const backup = restore('working', project)!;
    assert.equal(fs.readFileSync(path.join(project, 'main.rs'), 'utf8'), 'working');
    assert.equal(fs.readFileSync(path.join(backup, 'main.rs'), 'utf8'), 'experiment');
    assert(fs.existsSync(path.join(backup, 'target/build')));
}));

test('unsafe names and references inside the project are rejected, including symlinks', () => fixture(folder => {
    const project = path.join(folder, 'journey');
    fs.mkdirSync(project);
    fs.symlinkSync(project, path.join(folder, 'alias'), 'dir');
    for (const name of ['', '../outside', '/tmp/outside']) assert.throws(() => savedPath(name, project));
    for (const target of [project, path.join(project, 'answer'), path.join(folder, 'alias/answer')]) {
        assert.throws(() => safeOutput(target, project), /outside/);
    }
    safeOutput(path.join(folder, 'reference'), project);
}));

test('read-only comparison identifies a typo and leaves the project untouched', () => fixture(folder => {
    const files = expected('01-canvas');
    for (const [name, code] of Object.entries(files)) {
        const file = path.join(folder, name);
        fs.mkdirSync(path.dirname(file), {recursive: true});
        fs.writeFileSync(file, code);
    }
    assert.deepEqual(compare(files, folder), []);
    const source = path.join(folder, 'src/lib.rs');
    fs.appendFileSync(source, 'my experiment\n');
    const before = fs.readFileSync(source);
    assert(compare(files, folder)[0].includes('my experiment'));
    assert.deepEqual(fs.readFileSync(source), before);
}));

test('the full reference is reconstructed without Python and matches every final file hash', () => {
    const files = referenceFiles('37');
    for (const [name, digest] of Object.entries(reference().files)) assert.equal(hash(files[name]), digest, name);
    assert.equal(Object.keys(files).length, Object.keys(reference().files).length);
});

test('a dependency update preserves the previous lock and never writes application code', () => fixture(folder => {
    fs.writeFileSync(path.join(folder, 'Cargo.toml'), expected('15-perspective')['Cargo.toml']);
    fs.writeFileSync(path.join(folder, 'Cargo.lock'), read(lockFor('01-canvas')));
    fs.writeFileSync(path.join(folder, 'my-work.rs'), 'my unfinished work');
    dependencies('15-perspective', folder);
    assert.equal(read(path.join(folder, 'Cargo.lock')), read(lockFor('15-perspective')));
    assert.equal(read(path.join(folder, 'Cargo.lock.before-15-perspective')), read(lockFor('01-canvas')));
    dependencies('15-perspective', folder);
    assert.equal(read(path.join(folder, 'my-work.rs')), 'my unfinished work');
    fs.appendFileSync(path.join(folder, 'Cargo.toml'), '# my own dependency experiment\n');
    assert.throws(() => dependencies('15-perspective', folder), /Cargo.toml/);
}));

test('a reference exported at another depth still finds the geometry kernel', () => fixture(folder => {
    const target = path.join(folder, 'reference');
    materialize(expected('15-perspective'), target, lockFor('15-perspective'));
    const relative = read(path.join(target, 'Cargo.toml')).match(/session_rust = \{ path = "([^"]+)"/)![1];
    assert.equal(path.resolve(target, relative), path.resolve(viewer, '../session_rust'));
}));
