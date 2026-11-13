import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import {course, docs, viewer, expected, hash, read, lockFor} from './model.ts';

test('consolidated Windows evidence remains bound to its source, assets and nine reviewed frames', () => {
    const receipt = JSON.parse(read(path.join(docs, 'journey/release/consolidated-windows-20261113.json')));
    const normalized = (file: string) => hash(read(file).replaceAll('\r\n', '\n'));
    assert.equal(normalized(path.join(viewer, '../session_tests/scripts/course/verify.ts')), receipt.checker.native);
    assert.equal(normalized(path.join(docs, 'capture_journey.cjs')), receipt.checker.browserNormalized);
    for (const [id, record] of Object.entries<any>(receipt.steps)) {
        const files = expected(id);
        assert.deepEqual(Object.keys(files).sort(), Object.keys(record.normalizedEndpoint).sort());
        for (const [name, digest] of Object.entries(record.normalizedEndpoint))
            assert.equal(hash(files[name].replaceAll('\r\n', '\n')), digest, id + ': ' + name);
        assert.equal(normalized(lockFor(id)), record.lock);
        const step = course().steps.find(step => step.id === id)!;
        assert.equal(normalized(path.join(docs, step.frame!)), record.nativeFrame);
        for (const [name, digest] of Object.entries(record.assets))
            assert.equal(hash(fs.readFileSync(path.join(viewer, name))), digest, name);
        assert.equal(record.captures.length, 3);
        for (const capture of record.captures)
            assert.equal(hash(fs.readFileSync(path.join(viewer, capture.file))), capture.sha256);
        assert(record.build.pixels.sha256);
        for (const resource of record.resources) {
            assert.equal(resource.exitCode, 0);
            assert.equal(resource.timedOut, false);
        }
        for (const capture of Object.values<any>(record.browser)) {
            assert.equal(capture.source, record.build.source);
            assert.equal(capture.checker, receipt.checker.browser);
        }
    }
});
