import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import {test} from 'node:test';
import {copyTrunkFiles} from './incremental-browser-image-build.ts';
import {maintainedClone} from './maintained-browser-presentation.ts';
import {docs, hash, read} from './model.ts';

test('maintained adapter preserves the current original capture source and both density assertions', () => {
    const clone = maintainedClone();
    assert.equal(clone.originalSha256, hash(read(path.join(docs, 'capture_current.cjs'))));
    assert.notEqual(clone.adaptedSha256, clone.originalSha256);
    for (const assertion of ['for (const density of [1, 2])', 'assert(blue > 5000 * density * density',
        'assert(black > 200 * density * density', 'assert.deepEqual(errors, []);']) assert(clone.adapted.includes(assertion));
});

test('explicit Trunk copy-file transport preserves bytes, rejects escape and refuses changed destinations', () => {
    const root = fs.mkdtempSync(path.join(os.tmpdir(), 'maintained-transport-'));
    const native = path.join(root, 'source'), dist = path.join(root, 'dist');
    fs.mkdirSync(path.join(native, 'assets'), {recursive: true}); fs.mkdirSync(dist);
    const index = path.join(native, 'index.html');
    try {
        fs.writeFileSync(path.join(native, 'view_local.yaml'), 'source: local');
        fs.writeFileSync(path.join(native, 'assets/diagnostics.js'), 'original diagnostics');
        fs.writeFileSync(path.join(native, 'assets/unlisted.js'), 'not transported');
        fs.writeFileSync(index, '<link data-trunk rel="copy-file" href="view_local.yaml">\n<link data-trunk rel="copy-file" href="assets/diagnostics.js">');
        copyTrunkFiles(native, dist);
        assert.deepEqual(fs.readdirSync(dist).sort(), ['diagnostics.js', 'view_local.yaml']);
        assert.equal(fs.readFileSync(path.join(dist, 'diagnostics.js'), 'utf8'), 'original diagnostics');
        copyTrunkFiles(native, dist);
        fs.writeFileSync(path.join(dist, 'diagnostics.js'), 'other owner');
        assert.throws(() => copyTrunkFiles(native, dist), /destination collision/);
        assert.equal(fs.readFileSync(path.join(dist, 'diagnostics.js'), 'utf8'), 'other owner');
        fs.writeFileSync(path.join(root, 'outside.js'), 'outside');
        fs.writeFileSync(index, '<link data-trunk rel="copy-file" href="../outside.js">');
        assert.throws(() => copyTrunkFiles(native, dist), /inside the materialized source/);
        assert(!fs.existsSync(path.join(dist, 'outside.js')));
    } finally { fs.rmSync(root, {recursive: true}); }
});
