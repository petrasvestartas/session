import fs from 'node:fs';
import path from 'node:path';
import {spawnSync} from 'node:child_process';
import {assetsFor, expected, fingerprint, lockFor, materialize, viewer} from './model.ts';
import {nativeTarget} from './verify.ts';

// Invoke under the workspace storage/build guards; use the shared Cargo target cache.
const suite = process.argv[2] || 'quality';
if (!['quality', 'dock'].includes(suite)) throw Error('Choose quality or dock');
const id = suite === 'quality' ? '96zzzzzzv-reuse-coherent-frame-targets' : '03d-input';
const output = fs.mkdtempSync(path.join(viewer, `target/${suite}-regression-`));
fs.rmdirSync(output); // materialize requires a new directory at the course's standard target depth.
const project = output;
try {
    const files = expected(id);
    materialize(files, project, lockFor(id), assetsFor(id));
    fs.mkdirSync(path.join(project, 'tests'), {recursive: true});
    fs.copyFileSync(new URL(`./${suite}-regression.rs`, import.meta.url), path.join(project, `tests/${suite}_regression.rs`));
    const result = spawnSync('cargo', ['test', '--locked', '--manifest-path', path.join(project, 'Cargo.toml'),
        '--target', nativeTarget(), '--test', `${suite}_regression`, '-j8'], {
        cwd: viewer, stdio: 'inherit', env: {...process.env, REGEN_PROTO: '0', CARGO_TARGET_DIR: path.join(viewer, 'target')},
    });
    if (result.error || result.status !== 0) throw result.error || Error(`Quality regression exited ${result.status}`);
    console.log(JSON.stringify({checkpoint: id, source: fingerprint(files), suite, result: 'passed'}));
} finally {
    fs.rmSync(output, {recursive: true, force: true});
}
